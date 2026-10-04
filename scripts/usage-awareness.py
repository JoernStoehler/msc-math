#!/usr/bin/env python3
"""On-demand count-up local usage report; retained withdrawn hook prototype.

The PostToolUse prototype was withdrawn after its complexity was rejected. It
is unconfigured and unselected; no activation approval is pending. The ordinary
on-demand report remains usable. Reads accounting envelopes; prototype state
stores only report/checkpoint metadata, never hook tool bodies.
"""
import argparse
from datetime import datetime, timezone
import fcntl
import importlib.util
import json
import os
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parent.parent
UUID = re.compile(r"[0-9a-fA-F]{8}(?:-[0-9a-fA-F]{4}){3}-[0-9a-fA-F]{12}")
spec = importlib.util.spec_from_file_location("usage", ROOT / "scripts/subtree-usage.py")
USAGE = importlib.util.module_from_spec(spec)
spec.loader.exec_module(USAGE)


class ReportError(ValueError):
    """Safe report diagnostics constructed here, rather than log/body contents."""


def snapshot(home, ident, now):
    if not isinstance(ident, str) or not UUID.fullmatch(ident):
        raise ReportError("a valid local thread UUID is required")
    sessions = USAGE.inventory(home)
    if ident not in sessions:
        raise ReportError(f"{ident}: thread metadata absent from local logs")
    root, seen = ident, set()
    while USAGE.parent(sessions[root][1]):
        if root in seen:
            raise ReportError("cycle in spawn ancestry")
        seen.add(root)
        ancestor = USAGE.parent(sessions[root][1])
        if ancestor not in sessions:
            raise ReportError(f"{ancestor}: spawn ancestor absent from local logs")
        root = ancestor
    rows, errors = [], set()
    latest = None
    for thread in sorted(USAGE.descendants(sessions, root)):
        events = USAGE.usage_events(*sessions[thread], now, errors)
        totals = [sum(e[2][i] for e in events) for i in range(3)]
        inp, cached, output = totals
        rows.append({"thread_id": thread, "role": "root" if thread == root else "worker",
                     "input_tokens": inp, "cached_input_tokens": cached,
                     "output_tokens": output, "usage_units": inp-cached+output,
                     "recorded_responses": len(events)})
        if events:
            newest = max(e[0] for e in events)
            latest = max(latest, newest) if latest else newest
    own = next(row for row in rows if row["thread_id"] == ident)
    started = USAGE.date(sessions[root][1]["timestamp"])
    return {"schema_version": 1, "observed_at": now.isoformat(timespec="seconds"),
            "latest_recorded_at": latest.isoformat(timespec="seconds") if latest else None,
            "root": root, "thread_id": ident, "status": "partial" if errors else "recorded",
            "scope": "this local root and its spawned descendants; ordinary forks are separate",
            "unit": "input minus cached input plus output; no model weighting or quota/billing conversion",
            "root_elapsed_seconds": max(0, int((now-started).total_seconds())),
            "tree_usage_units": sum(row["usage_units"] for row in rows),
            "own_usage_units": own["usage_units"], "threads": rows,
            "errors": sorted(errors)}


def message(report, previous=None):
    tree, own = report["tree_usage_units"], report["own_usage_units"]
    delta = ""
    if previous and previous.get("root") == report["root"]:
        prior = previous.get("tree_usage_units")
        if report["errors"] or previous.get("errors"):
            delta = "; change unavailable across accounting errors"
        elif isinstance(prior, int) and tree >= prior:
            delta = f"; +{tree-prior:,} since the previous reminder"
        else:
            delta = "; prior checkpoint no longer comparable"
    label = "observed subtotal" if report["errors"] else "recorded usage"
    text = (f"Local {label} at {report['observed_at']}: shared tree {tree:,} units"
            f"{delta}; your thread {own:,}. "
            f"Units = noncached input + output, without model weighting; quota/spend conversion unknown. "
            f"Root {report['root']}; {len(report['threads'])} local threads. "
            f"Latest record: {report['latest_recorded_at'] or 'none yet'}; in-flight usage follows completion.")
    if report["errors"]:
        text += " Accounting errors: " + "; ".join(report["errors"][:2])
        if len(report["errors"]) > 2:
            text += f"; {len(report['errors'])-2} further errors in the on-demand report"
    text += f" On demand: python3 {ROOT}/scripts/usage-awareness.py --json."
    text += " Use this observation when evaluating further work within the authorized scope."
    return text


def hook_identity(payload):
    # agent_id identifies the child; session_id can identify its shared root.
    ident = payload.get("agent_id") or payload.get("session_id")
    if not isinstance(ident, str) or not UUID.fullmatch(ident):
        raise ReportError("hook lacks a valid session/agent UUID")
    return ident


def hook(home, payload, state_dir, interval, now):
    if payload.get("hook_event_name") != "PostToolUse":
        raise ReportError("expected PostToolUse")
    cwd = Path(payload.get("cwd", "")).resolve()
    if cwd != ROOT and ROOT not in cwd.parents:
        return None  # this candidate is scoped to its owning checkout
    ident = hook_identity(payload)
    state_dir.mkdir(parents=True, exist_ok=True)
    # One lock per receiving agent prevents concurrent tools from duplicating a
    # reminder. Other agents still receive their own shared-tree observation.
    with (state_dir / f"{ident}.lock").open("a") as lock:
        try:
            fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        except BlockingIOError:
            return None  # an overlapping tool is already handling this agent
        path = state_dir / f"{ident}.json"
        previous = None
        if path.exists():
            try:
                previous = json.loads(path.read_text())
            except (ValueError, OSError):
                pass
        stamp = now.timestamp()
        if not isinstance(previous, dict):
            previous = None
        checked = previous.get("checked_at", 0) if previous else 0
        if not isinstance(checked, (int, float)):
            previous, checked = None, 0
        if previous and 0 <= stamp-checked < interval:
            return None  # cheap path: no inventory/log scan
        try:
            report = snapshot(home, ident, now)
        except (ValueError, OSError, KeyError, TypeError, AttributeError) as exc:
            reason = str(exc) if isinstance(exc, ReportError) else "local accounting logs unreadable or invalid"
            checkpoint = {"root": None, "tree_usage_units": None, "own_usage_units": None,
                          "errors": [reason], "checked_at": stamp}
            path.write_text(json.dumps(checkpoint) + "\n")
            if previous and previous.get("errors") == [reason]:
                return None
            return {"hookSpecificOutput": {"hookEventName": "PostToolUse",
                "additionalContext": f"Local usage report unavailable: {reason}. On demand: python3 {ROOT}/scripts/usage-awareness.py --json."}}
        changed = previous is None or any(report[key] != previous.get(key)
            for key in ("root", "tree_usage_units", "own_usage_units", "errors"))
        # Persist accounting/checkpoint fields only, never stdin/tool bodies.
        checkpoint = {key: report[key] for key in
                      ("root", "tree_usage_units", "own_usage_units", "errors")}
        checkpoint["checked_at"] = stamp
        path.write_text(json.dumps(checkpoint) + "\n")
        if not changed:
            return None
        return {"hookSpecificOutput": {"hookEventName": "PostToolUse",
                                       "additionalContext": message(report, previous)}}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("thread", nargs="?", help="current thread UUID; default CODEX_THREAD_ID")
    parser.add_argument("--json", action="store_true", help="full per-thread accounting report")
    parser.add_argument("--hook", action="store_true",
                        help="retained withdrawn prototype: read PostToolUse JSON on stdin; unconfigured/unselected")
    parser.add_argument("--interval-seconds", type=int, default=300,
                        help="withdrawn prototype cadence between eligible tool results; unconfigured/unselected")
    parser.add_argument("--codex-home", type=Path, default=Path(os.environ.get("CODEX_HOME", Path.home()/".codex")))
    parser.add_argument("--state-dir", type=Path, default=Path(os.environ.get("XDG_CACHE_HOME", Path.home()/".cache"))/"msc-math/usage-reminders",
                        help="checkpoint directory for withdrawn prototype; unconfigured/unselected")
    parser.add_argument("--now", type=USAGE.date, help="fixed timestamp for verification")
    args = parser.parse_args()
    if args.interval_seconds < 1:
        parser.error("interval must be positive")
    now = args.now or datetime.now(timezone.utc)
    try:
        if args.hook:
            # The runtime supplies tool fields too. Discard them immediately;
            # only event/scope/identity fields participate in reporting.
            raw = json.load(sys.stdin)
            payload = {key: raw.get(key) for key in
                       ("hook_event_name", "cwd", "agent_id", "session_id")}
            del raw
            output = hook(args.codex_home, payload, args.state_dir, args.interval_seconds, now)
            if output:
                print(json.dumps(output))
            return 0
        report = snapshot(args.codex_home, args.thread or os.environ.get("CODEX_THREAD_ID"), now)
        print(json.dumps(report, indent=2) if args.json else message(report))
        return 2 if report["errors"] else 0
    except (ValueError, OSError, KeyError, TypeError, AttributeError) as exc:
        # Hook failures inform the model without blocking the completed tool.
        reason = str(exc) if isinstance(exc, ReportError) else "invalid event or unreadable accounting logs/state"
        error = f"Local usage report unavailable: {reason}."
        if args.hook:
            print(json.dumps({"hookSpecificOutput": {"hookEventName": "PostToolUse",
                                                     "additionalContext": error}}))
            return 0
        print(error, file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main())
