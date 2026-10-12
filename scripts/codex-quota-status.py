#!/usr/bin/env python3
"""Read current account quota through Codex app-server; no model or reset requests.

Requires Python 3.11+ and an installed, signed-in Codex CLI. Contract checked
against CLI 0.160.0 (rust-v0.160.0): account/rateLimits/read and v2/account.rs.
"""

import argparse
import asyncio
import datetime as dt
import json
import math
import sys


def utc_timestamp(seconds):
    if not isinstance(seconds, (int, float)) or isinstance(seconds, bool):
        return None
    try:
        return dt.datetime.fromtimestamp(seconds, dt.timezone.utc).isoformat()
    except (ValueError, OverflowError, OSError):
        return None


def number(value):
    return value if type(value) in (int, float) and math.isfinite(value) else None


def summarize(response):
    """Allowlist metadata; never emit account IDs, credits balances or raw replies."""
    buckets = response.get("rateLimitsByLimitId")
    if not isinstance(buckets, dict) or not buckets:
        snapshot = response.get("rateLimits")
        buckets = {snapshot.get("limitId") or "unspecified": snapshot} if isinstance(snapshot, dict) else {}
    windows = []
    absent = []
    for bucket, snapshot in sorted(buckets.items()):
        for slot in ("primary", "secondary"):
            window = snapshot.get(slot) if isinstance(snapshot, dict) else None
            if not isinstance(window, dict):
                absent.append(f"{bucket}/{slot}")
                continue
            used = number(window.get("usedPercent"))
            duration = number(window.get("windowDurationMins"))
            label = "weekly" if duration == 10080 else "5-hour" if duration == 300 else f"{duration:g}-minute" if duration is not None else "unknown duration"
            windows.append({"bucket": bucket, "slot": slot, "window": label,
                            "duration_minutes": duration, "used_percent": used,
                            "remaining_percent": max(0, min(100, 100 - used)) if used is not None else None,
                            "resets_at": utc_timestamp(window.get("resetsAt"))})
    summary = response.get("rateLimitResetCredits")
    summary = summary if isinstance(summary, dict) else {}
    credits = summary.get("credits")
    expirations = None if not isinstance(credits, list) else [
        {"status": credit.get("status"), "expires_at": utc_timestamp(credit.get("expiresAt")),
         "no_expiry": "expiresAt" in credit and credit["expiresAt"] is None}
        for credit in credits if isinstance(credit, dict)]
    allowed = response.get("ordinaryUsageAllowed")
    return {"observed_at": dt.datetime.now(dt.timezone.utc).isoformat(),
            "source": "codex app-server account/rateLimits/read", "windows": windows,
            "windows_not_returned": absent,
            "ordinary_usage_allowed": allowed if type(allowed) is bool else None,
            "available_resets": number(summary.get("availableCount")),
            "reset_credit_expirations": expirations, "reset_eligibility": "unknown"}


async def read_quota(executable):
    process = await asyncio.create_subprocess_exec(
        executable, "app-server", "--stdio", stdin=asyncio.subprocess.PIPE,
        stdout=asyncio.subprocess.PIPE, stderr=asyncio.subprocess.DEVNULL)

    async def send(message):
        process.stdin.write((json.dumps(message) + "\n").encode())
        await process.stdin.drain()

    async def request(request_id, method, params):
        await send({"id": request_id, "method": method, "params": params})
        while True:
            line = await process.stdout.readline()
            if not line:
                raise RuntimeError("Codex app-server closed before responding")
            reply = json.loads(line)
            if reply.get("id") != request_id:
                continue
            if "error" in reply:
                # Server errors may contain private account details; do not echo them.
                raise RuntimeError(f"Codex app-server rejected {method}; quota unknown")
            if not isinstance(reply.get("result"), dict):
                raise RuntimeError("Unexpected app-server response; quota unknown")
            return reply["result"]

    try:
        async with asyncio.timeout(40):
            await request(1, "initialize", {"clientInfo": {"name": "quota_status", "version": "1.0"},
                                           "capabilities": {"experimentalApi": True}})
            await send({"method": "initialized"})
            return summarize(await request(2, "account/rateLimits/read", {}))
    finally:
        if process.returncode is None:
            try:
                process.terminate()
            except ProcessLookupError:
                pass
            try:
                await asyncio.wait_for(process.wait(), timeout=5)
            except asyncio.TimeoutError:
                process.kill()
                await process.wait()


def render(report):
    def shown(value):
        return "unknown" if value is None else str(value)

    lines = [f"Observed {report['observed_at']} (account-wide, server-reported quota)"]
    for window in report["windows"]:
        lines.append(f"{window['bucket']} {window['window']} ({window['slot']}): "
                     f"{shown(window['used_percent'])}% used, {shown(window['remaining_percent'])}% remaining; "
                     f"resets {shown(window['resets_at'])}")
    if not report["windows"]:
        lines.append("Quota windows: unknown (none returned)")
    if report["windows_not_returned"]:
        lines.append("Windows not returned: " + ", ".join(report["windows_not_returned"]))
    lines.append(f"Ordinary usage allowed: {shown(report['ordinary_usage_allowed'])}")
    lines.append(f"Available resets: {shown(report['available_resets'])}; reset eligibility: unknown")
    expirations = report["reset_credit_expirations"]
    lines.append("Reset credit expirations (detail list may be capped): " + (
        "unknown" if expirations is None else "; ".join(
            f"{shown(credit['status'])}: {'no expiry' if credit['no_expiry'] else shown(credit['expires_at'])}"
            for credit in expirations) or "none returned"))
    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--codex", default="codex", help="Codex executable (default: PATH)")
    parser.add_argument("--json", action="store_true", help="Print only allowlisted quota metadata")
    args = parser.parse_args()
    try:
        report = asyncio.run(read_quota(args.codex))
    except FileNotFoundError:
        print("Quota unknown: Codex executable not found; use --codex /path/to/codex", file=sys.stderr)
        return 1
    except RuntimeError as exc:
        print(str(exc), file=sys.stderr)
        return 1
    except Exception:
        print("Quota unknown: app-server check failed or timed out; check Codex sign-in/support", file=sys.stderr)
        return 1
    print(json.dumps(report, indent=2) if args.json else render(report))
    return 0


if __name__ == "__main__":
    sys.exit(main())
