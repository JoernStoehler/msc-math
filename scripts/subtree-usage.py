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
import re
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
    return meta.get("parent_thread_id") or spawn.get("parent_thread_id")


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


def counters(value):
    """Required fields must exist; absent optional cache writes remain unknown."""
    if not isinstance(value, dict):
        raise ValueError("missing usage counters")
    result = []
    for key in KEYS:
        raw = value.get(key) if key == KEYS[3] else value[key]
        if raw is None and key == KEYS[3]:
            result.append(None)
            continue
        if isinstance(raw, bool) or not (isinstance(raw, int) or
                isinstance(raw, str) and re.fullmatch(r"[0-9]+", raw)):
            raise ValueError("invalid token count")
        result.append(int(raw))
    inp, cached, out, writes = result
    if min(inp, cached, out) < 0 or cached > inp or (
            writes is not None and (writes < 0 or cached + writes > inp)):
        raise ValueError("invalid token counts")
    return tuple(result)


def usage_events(path, meta, now, warnings):
    """Read accounting envelopes only. Prefer owned response records over mirrors.

    Legacy copied prefixes end at a settings snapshot owned by this thread, or
    an explicit subagent history ordinal. Unknown fork boundaries fail visibly.
    """
    model, previous = "unknown", None
    events, seen_responses = [], {}
    ident = meta.get("id")
    start = date(meta["timestamp"])
    fork = bool(meta.get("forked_from_id"))
    owned = not fork or bool(meta.get("history_base"))
    boundary = meta.get("subagent_history_start_ordinal")
    native_mode = False
    unresolved_fork = False
    legacy_fork_baseline = False
    try:
        with path.open() as f:
            for number, line in enumerate(f, 1):
                if not any(x in line for x in ('"turn_context"', '"token_count"',
                        '"token_usage_record"', '"thread_settings_applied"')):
                    continue
                try:
                    record = json.loads(line)
                    kind, payload = record.get("type"), record.get("payload", {})
                    if not isinstance(payload, dict):
                        raise ValueError("invalid envelope")
                    if kind == "turn_context":
                        settings = (payload.get("collaboration_mode") or {}).get("settings") or {}
                        model = payload.get("model") or settings.get("model") or model
                        continue
                    if kind == "event_msg" and payload.get("type") == "thread_settings_applied":
                        if ident and payload.get("thread_id") == ident:
                            owned = True
                            model = (payload.get("thread_settings") or {}).get("model") or model
                        continue
                    if boundary is not None and isinstance(record.get("ordinal"), int):
                        owned = record["ordinal"] >= boundary
                    if kind == "token_usage_record":
                        # Copied records preserve their original owner's ID.
                        if not ident or payload.get("thread_id") != ident:
                            continue
                        native_mode = True  # invalid owned records must not charge their mirror
                        response = payload.get("response_id")
                        if not isinstance(response, str) or not response:
                            raise ValueError("missing response ID")
                        usage = counters(payload.get("usage"))
                        if response in seen_responses:
                            if seen_responses[response] != usage:
                                raise ValueError("conflicting duplicate response usage")
                            continue
                        seen_responses[response] = usage
                    elif kind == "event_msg" and payload.get("type") == "token_count":
                        info = payload.get("info")
                        if info is None:  # quota-only notification
                            continue
                        current = counters(info.get("total_token_usage"))
                        usage = counters(info.get("last_token_usage"))
                        if current == previous:
                            continue
                        prior, previous = previous, current
                        if native_mode:
                            continue  # cumulative mirror of native response records
                        if not owned:
                            unresolved_fork = True
                            continue  # maintain inherited baseline, never charge it
                        if fork and prior is None:
                            legacy_fork_baseline = True
                            continue  # first cumulative snapshot can be an inherited seed
                        if prior is not None:
                            delta = tuple(a-b if a is not None and b is not None else None
                                          for a, b in zip(current, prior))
                            if any(a != b for a, b in zip(delta, usage)
                                   if a is not None and b is not None):
                                warnings.add(f"{ident}: cumulative/last counter mismatch; using last usage")
                    else:
                        continue
                    when = date(record["timestamp"])
                    if start <= when <= now:
                        events.append((when, model, usage))
                except (ValueError, KeyError, TypeError, AttributeError):
                    warnings.add(f"{ident}: malformed/missing accounting field at line {number}; record omitted")
    except OSError:
        warnings.add(f"{ident}: usage log unreadable")
    if unresolved_fork and not owned and not native_mode:
        warnings.add(f"{ident}: fork ownership boundary unavailable; legacy usage excluded")
    if legacy_fork_baseline and not native_mode:
        warnings.add(f"{ident}: initial fork counter ownership ambiguous; treated as baseline")
    return events


def collect(path, meta, now, warnings):
    events = []
    for when, model, usage in usage_events(path, meta, now, warnings):
        inp, cached, out, writes = usage
        if writes is None:
            warnings.add(f"{meta.get('id')}: cache-write count absent; split input/cost unknown")
        cost = price(model, usage) if writes is not None else None
        if model not in RATES:
            warnings.add(f"Unpriced model: {model}")
        events.append((when, model, inp-cached-writes if writes is not None else None,
                       cached, out, writes, cost))
    if not events:
        warnings.add(f"{meta.get('id')}: no usage records (new/unused thread or missing telemetry)")
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
                    row[k] = row[k] + value if row[k] is not None and value is not None else None
                row[4] += cost or 0
                row[5] += cost is None
    start = date(sessions[root][1]["timestamp"])
    print(f"Root:    {root} ({len(selected)} locally discovered threads)")
    print(f"Started: {start.isoformat(timespec='seconds')}")
    print(f"Now:     {now.isoformat(timespec='seconds')}   elapsed: {now - start}")
    print(f"Latest usage: {latest.isoformat(timespec='seconds') if latest else 'none'}")
    print("Shadow: Standard API-equivalent rates, fixed 2026-09-16; not subscription quota/billing.")
    if any(not message.startswith("Unpriced model:") for message in warnings):
        print("Accounting errors below: numeric rows are observed subtotals, not complete totals.")
    print(f"{'Scope':22} {'Uncached input':>15} {'Cached input':>15} {'Output':>12} {'Cache write':>12} {'Shadow USD':>13}")
    order = ["TOTAL", "last 30m", "last 5m", "root", "workers"]
    order += sorted(k for k in rows if k not in order)
    for key in order:
        a, b, c, d, dollars, unknown = rows[key]
        amount = f"{dollars:,.2f}" + ("+?" if unknown else "")
        values = [f"{v:,}" if v is not None else "unknown" for v in (a, b, c, d)]
        print(f"{key:22} {values[0]:>15} {values[1]:>15} {values[2]:>12} {values[3]:>12} {amount:>13}")
    print("Scope: spawned descendants only; ordinary forks and independent roots excluded.")
    print("Windows use usage-event timestamps; in-flight calls appear after telemetry is written.")
    if warnings:
        print("ACCOUNTING DIAGNOSTICS:")
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
