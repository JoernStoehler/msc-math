#!/usr/bin/env python3
"""Print local Codex subtree usage. No dependencies, network, or log mutations.

Usage: python3 scripts/subtree-usage.py [ROOT_THREAD_ID]
Defaults to CODEX_THREAD_ID; resolves its top-level parent when available.
See subtree-usage.md for accounting conventions and limitations.
"""
import argparse
from collections import defaultdict
from datetime import datetime, timedelta, timezone
import json
import os
from pathlib import Path
import sys

# Standard API-equivalent shadow rates, USD / million tokens, checked 2026-09-16.
# https://developers.openai.com/api/docs/models/{model}
# Uncached input, cached input, output. Not ChatGPT subscription accounting.
RATES = {
    "gpt-6-astra": (10, 1, 50),
    "gpt-5.6-sol": (4, .4, 20),
    "gpt-5.6-luna": (.2, .02, 1.2),
    "gpt-5.6-terra": (2, .2, 12),
    "gpt-5.5": (5, .5, 30),
    "gpt-5.4": (2.5, .25, 15),
}
KEYS = ("input_tokens", "cached_input_tokens", "output_tokens", "cache_write_input_tokens")


def date(s):
    value = datetime.fromisoformat(s.replace("Z", "+00:00"))
    if value.tzinfo is None:
        raise ValueError("timestamps need a timezone")
    return value.astimezone(timezone.utc)


def parent(meta):
    source = meta.get("source")
    spawn = source.get("subagent", {}) if isinstance(source, dict) else {}
    spawn = spawn.get("thread_spawn", {}) if isinstance(spawn, dict) else {}
    return meta.get("parent_thread_id") or spawn.get("parent_thread_id") or meta.get("forked_from_id")


def inventory(home):
    sessions = {}
    for folder in ("sessions", "archived_sessions"):
        for path in sorted((home / folder).rglob("rollout-*.jsonl")):
            with path.open() as f:
                try:
                    record = json.loads(next(f))
                    meta = record["payload"]
                    if record["type"] != "session_meta":
                        continue
                    ident = meta["id"]
                except (ValueError, KeyError, StopIteration):
                    continue
            # Native/archive copies of the same thread are counted once.
            if ident not in sessions or path.stat().st_size > sessions[ident][0].stat().st_size:
                sessions[ident] = (path, meta)
    return sessions


def descendants(sessions, root):
    chosen = {root}
    while True:
        more = {i for i, (_, m) in sessions.items() if parent(m) in chosen} - chosen
        if not more:
            return chosen
        chosen.update(more)


def collect(path, meta, now, warnings):
    model = "unknown"
    previous = None
    events = []
    start = date(meta["timestamp"])
    with path.open() as f:
        for number, line in enumerate(f, 1):
            # Only parse metadata/context/counters; never load conversation bodies.
            if not any(x in line for x in ('"turn_context"', '"token_count"')):
                continue
            try:
                record = json.loads(line)
            except ValueError:
                warnings.add(f"{meta['id']}: malformed/incomplete log line {number}")
                continue
            payload = record.get("payload", {})
            if record.get("type") == "turn_context":
                settings = (payload.get("collaboration_mode") or {}).get("settings") or {}
                model = payload.get("model") or settings.get("model") or model
                continue
            if record.get("type") != "event_msg" or payload.get("type") != "token_count":
                continue
            info = payload.get("info")
            if not info:  # rate-limit-only notifications carry no usage
                continue
            total, last = info.get("total_token_usage"), info.get("last_token_usage")
            if not isinstance(total, dict) or not isinstance(last, dict):
                warnings.add(f"{meta['id']}: missing usage counters")
                continue
            current = tuple(int(total.get(k, 0)) for k in KEYS)
            if current == previous:
                continue
            usage = tuple(int(last.get(k, 0)) for k in KEYS)
            when = date(record["timestamp"])
            if previous is not None:
                delta = tuple(a - b for a, b in zip(current, previous))
                if delta != usage:
                    warnings.add(f"{meta['id']}: cumulative/last counter mismatch; using last usage")
            previous = current
            if when < start or when > now:
                continue
            inp, cached, out, writes = usage
            if min(usage) < 0 or cached + writes > inp:
                warnings.add(f"{meta['id']}: invalid token counts; record omitted")
                continue
            # Reasoning tokens are already included in output_tokens.
            cost = price(model, usage)
            if cost is None:
                warnings.add(f"Unpriced model: {model}")
            events.append((when, model, inp - cached - writes, cached, out, writes, cost))
    if not events:
        warnings.add(f"{meta['id']}: no usage records (new/unused thread or missing telemetry)")
    return events


def price(model, usage):
    if model not in RATES:
        return None
    inp, cached, out, writes = usage
    rate_in, rate_cache, rate_out = RATES[model]
    long = inp > 272_000
    return ((inp - cached - writes) * rate_in * (2 if long else 1)
            + cached * rate_cache * (2 if long else 1)
            + writes * rate_in * 1.25 * (2 if long else 1)
            + out * rate_out * (1.5 if long else 1)) / 1_000_000


def report(sessions, root, now):
    selected = descendants(sessions, root)
    warnings = set()
    rows = defaultdict(lambda: [0, 0, 0, 0, 0.0, 0])
    latest = None
    for ident in sorted(selected):
        path, meta = sessions[ident]
        for when, model, uncached, cached, out, writes, cost in collect(path, meta, now, warnings):
            latest = max(latest, when) if latest else when
            groups = ["TOTAL", "root" if ident == root else "workers", model]
            for minutes in (30, 5):
                if when >= now - timedelta(minutes=minutes):
                    groups.append(f"last {minutes}m")
            for group in groups:
                row = rows[group]
                for k, value in enumerate((uncached, cached, out, writes)):
                    row[k] += value
                row[4] += cost or 0
                row[5] += cost is None
    start = date(sessions[root][1]["timestamp"])
    print(f"Root:    {root} ({len(selected)} locally discovered threads)")
    print(f"Started: {start.isoformat(timespec='seconds')}")
    print(f"Now:     {now.isoformat(timespec='seconds')}   elapsed: {now - start}")
    print(f"Latest usage: {latest.isoformat(timespec='seconds') if latest else 'none'}")
    print("Shadow: Standard API-equivalent rates, fixed 2026-09-16; not subscription quota/billing.")
    print(f"{'Scope':22} {'Uncached input':>15} {'Cached input':>15} {'Output':>12} {'Cache write':>12} {'Shadow USD':>13}")
    order = ["TOTAL", "last 30m", "last 5m", "root", "workers"]
    order += sorted(k for k in rows if k not in order)
    for key in order:
        a, b, c, d, dollars, unknown = rows[key]
        amount = f"{dollars:,.2f}" + ("+?" if unknown else "")
        print(f"{key:22} {a:15,d} {b:15,d} {c:12,d} {d:12,d} {amount:>13}")
    print("Scope: local metadata-linked descendants only; separate roots/host-only logs excluded.")
    print("Windows use usage-event timestamps; in-flight calls appear after telemetry is written.")
    if warnings:
        print("ACCOUNTING WARNINGS — totals may be incomplete:")
        for message in sorted(warnings):
            print(f"  {message}")
    return 2 if warnings else 0


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", nargs="?", help="subtree root thread ID; default: current session's top-level root")
    parser.add_argument("--codex-home", type=Path, default=Path(os.environ.get("CODEX_HOME", Path.home() / ".codex")))
    parser.add_argument("--now", type=date, help="fixed ISO timestamp for reproducible reports")
    args = parser.parse_args()
    sessions = inventory(args.codex_home)
    root = args.root or os.environ.get("CODEX_THREAD_ID")
    if not root or root not in sessions:
        parser.error("root not found in local logs; supply ROOT_THREAD_ID and/or --codex-home")
    if not args.root:
        seen = set()
        while parent(sessions[root][1]):
            if root in seen:
                parser.error("cycle in thread ancestry")
            seen.add(root)
            ancestor = parent(sessions[root][1])
            if ancestor not in sessions:
                parser.error(f"ancestor {ancestor} missing locally; import logs or explicitly select a subtree")
            root = ancestor
    return report(sessions, root, args.now or datetime.now(timezone.utc))


if __name__ == "__main__":
    sys.exit(main())
