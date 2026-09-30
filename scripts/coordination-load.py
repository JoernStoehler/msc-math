#!/usr/bin/env python3
"""Bounded, read-only coordinator telemetry snapshot; JSON on stdout.

Source ~/.config/openobserve/codex-env.sh before running. No credentials,
prompts, tool content, reasoning text, prices or canonical-state writes.
Run once (e.g. every four minutes during an explicitly bounded watch):
  python3 scripts/coordination-load.py ROOT_THREAD_ID --minutes 15
Exit 2 means incomplete observation; advisory signals do not establish overload.
"""
import argparse
from collections import Counter
from datetime import datetime, timedelta, timezone
import importlib.util
import json
import os
from pathlib import Path
import re
from statistics import median
import sys
import urllib.error
import urllib.parse
import urllib.request

ROOT = Path(__file__).resolve().parent.parent
FIELDS = ("_timestamp", "conversation_id", "event_name", "event_kind", "model",
          "input_token_count", "cached_token_count", "output_token_count",
          "cache_write_token_count", "tool_name", "success", "http_response_status_code")
LIMIT = 10000
UUID = re.compile(r"[0-9a-fA-F-]{36}")
TOKEN_LINE = re.compile(r'"type"\s*:\s*"event_msg"\s*,\s*"payload"\s*:\s*\{\s*"type"\s*:\s*"token_count"')


def date(value):
    result = datetime.fromisoformat(value.replace("Z", "+00:00"))
    if result.tzinfo is None:
        raise ValueError("timestamp needs a timezone")
    return result.astimezone(timezone.utc)


def auth_header(env):
    raw = env.get("OTEL_EXPORTER_OTLP_LOGS_HEADERS", env.get("OTEL_EXPORTER_OTLP_HEADERS", ""))
    for field in raw.split(","):
        name, sep, value = field.partition("=")
        if sep and name.strip().lower() == "authorization":
            return urllib.parse.unquote(value.strip())
    raise ValueError("Authorization missing from standard OTLP environment; source codex-env.sh")


def inventory(home):
    # Existing helper reads only first-line session metadata, not transcripts.
    spec = importlib.util.spec_from_file_location("subtree_usage", ROOT / "scripts/subtree-usage.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    sessions = {ident: (path, {"id": ident, "timestamp": meta.get("timestamp"),
                              "parent_thread_id": module.parent(meta)})
                for ident, (path, meta) in module.inventory(home).items()}
    return sessions, module.descendants


def query(base, auth, ids, start, end):
    # IDs are validated as UUIDs by main. Explicit projection excludes native
    # argument/body/content/output/prompt/account fields at the server.
    selected = ",".join("'" + ident + "'" for ident in sorted(ids))
    sql = (f"SELECT {','.join(FIELDS)} FROM codex WHERE conversation_id IN ({selected}) "
           f"ORDER BY _timestamp ASC LIMIT {LIMIT + 1}")
    body = {"query": {"sql": sql, "start_time": int(start.timestamp() * 1e6),
                      "end_time": int(end.timestamp() * 1e6), "from": 0, "size": LIMIT + 1}}
    request = urllib.request.Request(base.rstrip("/") + "/api/default/_search",
                                     data=json.dumps(body).encode(), method="POST",
                                     headers={"Authorization": auth, "Content-Type": "application/json"})
    with urllib.request.urlopen(request, timeout=15) as response:
        data = json.load(response)
    if not isinstance(data.get("hits"), list):
        raise ValueError("search response missing hits list")
    errors = []
    if data.get("is_partial") or data.get("partial"):
        errors.append("OpenObserve reports partial results")
    if len(data["hits"]) > LIMIT:
        errors.append(f"OpenObserve row limit {LIMIT} reached; counters are incomplete")
    # Defensive projection again: never propagate unexpected server fields.
    return [{k: row[k] for k in FIELDS if k in row} for row in data["hits"][:LIMIT]], errors


def token_totals(rows):
    completed = [r for r in rows if r.get("event_name") == "codex.sse_event"
                 and r.get("event_kind") == "response.completed"]
    names = ("input_token_count", "cached_token_count", "output_token_count", "cache_write_token_count")
    totals = {key: None for key in names}
    valid = 0
    invalid = 0
    for row in completed:
        try:
            values = [int(row[key]) for key in names[:3]]
            if min(values) < 0 or values[1] > values[0]:
                raise ValueError("invalid counters")
        except (KeyError, TypeError, ValueError):
            invalid += 1
            continue
        valid += 1
        for key, value in zip(names[:3], values):
            totals[key] = (totals[key] or 0) + value
    writes = [r.get(names[3]) for r in completed]
    if completed and all(isinstance(v, int) and v >= 0 for v in writes):
        totals[names[3]] = sum(writes)
    return {"source": "native_otel_completed_response_logs", "observed_completions": len(completed),
            "valid_token_records": valid, "invalid_or_missing_token_records": invalid, **totals}


def response_sizes(rows):
    sizes = []
    for row in sorted(rows, key=lambda r: r.get("_timestamp", 0)):
        if row.get("event_name") != "codex.sse_event" or row.get("event_kind") != "response.completed":
            continue
        try:
            inp, cached = int(row["input_token_count"]), int(row["cached_token_count"])
            if 0 <= cached <= inp:
                sizes.append((inp, inp-cached, row.get("_timestamp")))
        except (KeyError, TypeError, ValueError):
            pass
    return sizes


def local_tokens(path, meta, start, end):
    """Fallback: parse only token_count envelopes, never conversation records."""
    rows = []
    warnings = []
    previous = None
    keys = ("input_tokens", "cached_input_tokens", "output_tokens", "cache_write_input_tokens")
    try:
        with path.open() as handle:
            for line in handle:
                if not TOKEN_LINE.search(line):
                    continue
                try:
                    record = json.loads(line)
                    info = record["payload"].get("info") or {}
                    total, last = info.get("total_token_usage"), info.get("last_token_usage")
                    if not isinstance(total, dict) or not isinstance(last, dict):
                        continue
                    current = tuple(int(total[k]) for k in keys[:3]) + (
                        int(total[keys[3]]) if total.get(keys[3]) is not None else None,)
                    values = tuple(int(last[k]) for k in keys[:3]) + (
                        int(last[keys[3]]) if last.get(keys[3]) is not None else None,)
                    for counts in (current, values):
                        if min(counts[:3]) < 0 or counts[1] > counts[0] or (
                                counts[3] is not None and (counts[3] < 0 or counts[1] + counts[3] > counts[0])):
                            raise ValueError("invalid counters")
                    if current == previous:
                        continue
                    previous = current
                    when = date(record["timestamp"])
                    if not max(start, date(meta["timestamp"])) <= when <= end:
                        continue
                    rows.append(values)
                except (ValueError, KeyError, TypeError):
                    warnings.append("malformed local token record omitted")
    except OSError:
        warnings.append("local token log unreadable")
    result = {"source": "local_token_count_fallback", "observed_completions": len(rows),
              "valid_token_records": len(rows), "invalid_or_missing_token_records": len(warnings)}
    result.update({name: sum(r[i] for r in rows) if rows and all(r[i] is not None for r in rows) else None
                   for i, name in enumerate(("input_token_count", "cached_token_count", "output_token_count", "cache_write_token_count"))})
    return result, sorted(set(warnings))


def unsuccessful(row):
    try:
        status = int(row.get("http_response_status_code") or 0)
    except (TypeError, ValueError):
        status = 0
    return str(row.get("success", "")).lower() == "false" or status >= 400


def thread_snapshot(ident, rows, end):
    events = Counter(r.get("event_name", "unknown") for r in rows)
    times = sorted({int(r["_timestamp"]) / 1e6 for r in rows if "_timestamp" in r})
    failures = [r for r in rows if unsuccessful(r)]
    sizes = response_sizes(rows)
    last_completion = sizes[-1][2] / 1e6 if sizes and sizes[-1][2] is not None else None
    return {"thread_id": ident, "models": sorted({r["model"] for r in rows if r.get("model")}),
            "event_counts": dict(sorted(events.items())),
            "request_events": {key: events[key] for key in ("codex.api_request", "codex.websocket_request")},
            "tools": dict(sorted(Counter(r.get("tool_name", "unknown") for r in rows
                                          if r.get("event_name") == "codex.tool_result").items())),
            "observed_unsuccessful_events": len(failures),
            "latest_completed_input_tokens": sizes[-1][0] if sizes else None,
            "latest_completed_non_cached_input_tokens": sizes[-1][1] if sizes else None,
            "latest_completed_at": datetime.fromtimestamp(last_completion, timezone.utc).isoformat()
                if last_completion is not None else None,
            "completed_response_silence_seconds": round(end.timestamp()-last_completion, 1)
                if last_completion is not None else None,
            "unsuccessful_event_kinds": dict(sorted(Counter(
                r.get("event_name", "unknown") + ":" + r.get("tool_name", "")
                for r in failures).items())), "tokens": token_totals(rows),
            "last_native_event_at": datetime.fromtimestamp(times[-1], timezone.utc).isoformat() if times else None,
            "native_event_silence_seconds": round(end.timestamp() - times[-1], 1) if times else None,
            "largest_observed_event_gap_seconds": round(max((b-a for a, b in zip(times, times[1:])), default=0), 1) if times else None}


def coordination(state):
    active = [{"id": t["id"], "owner": t.get("owner"), "status": t["status"]}
              for t in state.get("tasks", []) if t.get("status") == "running"]
    gates = [{"id": d["id"], "status": d.get("status")}
             for d in state.get("decisions", []) if d.get("status") not in ("resolved", "parked", "done")]
    return {"source": "docs/coordination/current.json", "updated_at": state.get("updated_at"),
            "running_assignments": sorted(active, key=lambda t: t["id"]),
            "unresolved_decisions": sorted(gates, key=lambda d: d["id"]),
            "semantic_queue_or_pending_review_count": None}


def portfolio_ids(state):
    ids = set()
    for task in state.get("tasks", []):
        values = [task.get("owner")] + task.get("related_threads", [])
        ids.update(value for value in values if isinstance(value, str) and UUID.fullmatch(value))
    return ids


def review_signals(root, threads, rows, state, end):
    signals = []
    for thread in threads:
        repeated = {kind: count for kind, count in thread["unsuccessful_event_kinds"].items() if count >= 2}
        if repeated:
            signals.append({"signal": "repeated_unsuccessful_events", "thread_id": thread["thread_id"],
                            "evidence": repeated,
                            "limitation": "Nested tools may count the same failure; failures do not establish overload."})
    recent = end.timestamp()*1e6-4*60*1e6
    root_sizes = response_sizes([r for r in rows if r.get("conversation_id") == root])
    root_completion_recent = bool(root_sizes and root_sizes[-1][2] is not None
                                  and recent <= root_sizes[-1][2] <= end.timestamp()*1e6)
    worker_sizes = [t["latest_completed_input_tokens"] for t in threads
                    if t["thread_id"] != root and t["latest_completed_input_tokens"] is not None
                    and t["latest_completed_at"] is not None
                    and recent <= date(t["latest_completed_at"]).timestamp()*1e6 <= end.timestamp()*1e6]
    if root_completion_recent and len(worker_sizes) >= 2 and root_sizes[-1][0] > 2*median(worker_sizes):
        signals.append({"signal": "coordinator_context_larger_than_recent_workers",
                        "evidence": {"latest_root_input": root_sizes[-1][0],
                                     "median_recent_worker_input": median(worker_sizes), "relative_factor": 2},
                        "limitation": "Input size is a context proxy; different assignments may justify the difference."})
    baseline = [uncached for _, uncached, _ in root_sizes[:-1]]
    if root_completion_recent and len(baseline) >= 3 and median(baseline) > 0 and root_sizes[-1][1] > 3*median(baseline):
        signals.append({"signal": "coordinator_non_cached_input_spike",
                        "evidence": {"latest_non_cached_input": root_sizes[-1][1],
                                     "prior_window_median_non_cached_input": median(baseline), "relative_factor": 3},
                        "limitation": "Cache misses and context changes may explain this; billing and spending remain unknown."})
    signals += [{"signal": "running_assignment_without_owner", "task_id": t["id"],
                 "evidence": "running status has no owner", "limitation": "Canonical metadata may lag."}
                for t in state.get("running_assignments", []) if not t["owner"]]
    return signals


def brief(result):
    def counters(threads):
        fields = ("input_token_count", "cached_token_count", "output_token_count")
        value = {key: sum(t["tokens"][key] or 0 for t in threads)
                 if any(t["tokens"][key] is not None for t in threads) else None for key in fields}
        value.update({"threads_with_native_events": sum(bool(t["event_counts"]) for t in threads),
                      "completed_responses": sum(t["tokens"]["observed_completions"] for t in threads),
                      "valid_token_records": sum(t["tokens"]["valid_token_records"] for t in threads),
                      "websocket_requests": sum(t["request_events"]["codex.websocket_request"] for t in threads),
                      "api_requests": sum(t["request_events"]["codex.api_request"] for t in threads),
                      "tool_result_events": sum(sum(t["tools"].values()) for t in threads),
                      "unsuccessful_events": sum(t["observed_unsuccessful_events"] for t in threads)})
        return value
    def fallback(threads):
        observed = [t["fallback_tokens"] for t in threads if "fallback_tokens" in t]
        fields = ("input_token_count", "cached_token_count", "output_token_count", "cache_write_token_count")
        return {"source": "local_token_count_fallback", "threads_checked": len(observed),
                "threads_with_valid_token_records": sum(bool(t["valid_token_records"]) for t in observed),
                "valid_token_records": sum(t["valid_token_records"] for t in observed),
                "invalid_or_missing_token_records": sum(t["invalid_or_missing_token_records"] for t in observed),
                **{key: sum(t[key] for t in observed if t[key] is not None)
                   if any(t[key] is not None for t in observed) and
                   all(t[key] is not None for t in observed if t["valid_token_records"]) else None
                   for key in fields}}
    root = [t for t in result["threads"] if t["thread_id"] == result["root"]]
    workers = [t for t in result["threads"] if t["thread_id"] != result["root"]]
    return {key: result[key] for key in ("observed_at", "window_start", "root", "scope", "native_status",
                                       "errors", "advisory_signals", "actual_billing_usd", "overload")} | {
        "root_native": counters(root), "workers_native": counters(workers),
        "root_fallback": fallback(root), "workers_fallback": fallback(workers),
        "root_latest_completed_input_tokens": root[0]["latest_completed_input_tokens"],
        "root_native_event_silence_seconds": root[0]["native_event_silence_seconds"],
        "discovered_threads": len(result["threads"]),
        "running_assignments": len(result["coordination"]["running_assignments"])
            if "running_assignments" in result["coordination"] else None,
        "unresolved_decisions": len(result["coordination"]["unresolved_decisions"])
            if "unresolved_decisions" in result["coordination"] else None,
        "scope_sources": result["scope_sources"],
        "warnings": result["limitations"]}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", help="root thread UUID")
    parser.add_argument("--minutes", type=int, default=15, help="bounded window, 1–240 minutes (default 15)")
    parser.add_argument("--now", type=date, help="fixed UTC/ISO observation boundary")
    parser.add_argument("--base-url", default="http://127.0.0.1:5080")
    parser.add_argument("--codex-home", type=Path, default=Path(os.environ.get("CODEX_HOME", Path.home()/".codex")))
    parser.add_argument("--state", type=Path, default=ROOT/"docs/coordination/current.json")
    parser.add_argument("--brief", action="store_true", help="compact root/worker counters and review signals")
    parser.add_argument("--thread", action="append", default=[], help="include extra project thread UUID (repeatable)")
    args = parser.parse_args()
    if not UUID.fullmatch(args.root) or not all(UUID.fullmatch(t) for t in args.thread) or not 1 <= args.minutes <= 240:
        parser.error("root must be a UUID and minutes must be 1–240")
    end = args.now or datetime.now(timezone.utc)
    start = end - timedelta(minutes=args.minutes)
    errors = []
    try:
        raw_state = json.loads(args.state.read_text())
        state = coordination(raw_state)
        project_ids = portfolio_ids(raw_state)
    except (OSError, ValueError, KeyError, TypeError):
        state = {"source": str(args.state), "status": "unknown"}
        project_ids = set()
        errors.append("coordination state unreadable/invalid; recorded portfolio coverage unknown")
    try:
        sessions, descendants = inventory(args.codex_home)
        ids = descendants(sessions, args.root)
        root_descendants = len(ids)
        for ident in project_ids | set(args.thread):
            ids.update(descendants(sessions, ident))
    except OSError:
        sessions, ids = {}, {args.root}
        root_descendants = 1
        ids.update(project_ids | set(args.thread))
        errors.append("local lineage inventory unreadable; only named threads queried")
    if args.root not in sessions:
        errors.append("root absent from local lineage inventory; child coverage unknown")
    for ident in sorted((project_ids | set(args.thread)) - sessions.keys()):
        errors.append(f"{ident}: named thread absent from local inventory; descendants/fallback unknown")
    if any(not UUID.fullmatch(ident) for ident in ids):
        parser.error("local lineage contains a non-UUID thread ID")
    try:
        rows, query_errors = query(args.base_url, auth_header(os.environ), ids, start, end)
        errors.extend(query_errors)
        native_status = "partial" if query_errors else "queried"
    except urllib.error.HTTPError as exc:
        rows, native_status = [], "unavailable"
        errors.append(f"OpenObserve HTTP {exc.code}; response body intentionally omitted")
    except (ValueError, OSError):
        rows, native_status = [], "unavailable"
        errors.append("OpenObserve unavailable or invalid response/auth environment; no secret details emitted")
    threads = []
    for ident in sorted(ids):
        selected = [r for r in rows if r.get("conversation_id") == ident]
        thread = thread_snapshot(ident, selected, end)
        if not thread["tokens"]["valid_token_records"] and ident in sessions:
            thread["fallback_tokens"], warnings = local_tokens(*sessions[ident], start, end)
            errors.extend(f"{ident}: {w}" for w in warnings)
        if thread["tokens"]["invalid_or_missing_token_records"]:
            errors.append(f"{ident}: native completed response has invalid/missing token counters")
        threads.append(thread)
    signals = review_signals(args.root, threads, rows, state, end)
    result = {"observed_at": end.isoformat(), "window_start": start.isoformat(), "root": args.root,
              "scope": "root descendants plus recorded/explicit project threads and their local descendants; not the whole workstation",
              "scope_sources": {"root_locally_discovered_threads": root_descendants,
                                "canonical_project_uuid_threads": sorted(project_ids),
                                "explicit_threads": sorted(set(args.thread))},
              "native_status": native_status, "native_rows": len(rows), "threads": threads,
              "coordination": state, "advisory_signals": signals, "errors": sorted(set(errors)),
              "actual_billing_usd": None, "subscription_quota_remaining": None, "overload": "unknown",
              "limitations": ["Request events are transport attempts, not unique billed model calls.",
                              "Completed-response counters are observed usage, not verified billing.",
                              "Export deduplication/completeness are unverified; no response ID is exported in this projection.",
                              "No native events/completions can mean idle, in-flight work or incomplete export.",
                              "Event gaps are telemetry silence, not proven idle intervals.",
                              "Native OTel has seven-day retention; remote/cloud/Pro sessions are excluded.",
                              "Task burden, missed messages and pending review load require coordinator judgment.",
                              "Nested tool results count exporter events; they are not unique tool operations/failures.",
                              "Local fallback counters are separate; never added to native totals."]}
    print(json.dumps(brief(result) if args.brief else result, indent=2, sort_keys=True))
    return 2 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
