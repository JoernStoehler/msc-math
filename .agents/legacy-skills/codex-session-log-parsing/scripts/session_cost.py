#!/usr/bin/env python3
"""Account for Codex rollout token counters without reading message contents.

Raw ``token_count.info.total_token_usage`` values are cumulative.  This tool
turns them into counter deltas, including a ``--since`` baseline, and charges
cached input at its separate rate.  It intentionally exposes event kinds and
timestamps only; it never emits message, tool-argument, or tool-output text.
"""

from __future__ import annotations

import argparse
import json
import sys
from collections import defaultdict
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Iterable


DEFAULT_RATES = {
    "gpt-5.6-sol": (10.0, 1.0, 60.0),
    "gpt-5.6-terra": (5.0, 0.5, 30.0),
    "gpt-5.6-luna": (2.0, 0.2, 12.0),
}
RATE_SOURCE = "AGENTS.md priority-tier rates recorded 2026-07-16 (USD / million tokens)"
DEFAULT_LOG_ROOTS = (Path("~/.codex/sessions").expanduser(), Path("~/.codex/archived_sessions").expanduser())


@dataclass(frozen=True)
class Counter:
    timestamp: str
    input_tokens: int
    cached_input_tokens: int
    output_tokens: int

    def minus(self, other: "Counter | None") -> "Counter":
        if other is None:
            return self
        return Counter(
            self.timestamp,
            self.input_tokens - other.input_tokens,
            self.cached_input_tokens - other.cached_input_tokens,
            self.output_tokens - other.output_tokens,
        )


@dataclass
class Checkpoint:
    timestamp: str
    label: str
    input_tokens: int = 0
    cached_input_tokens: int = 0
    output_tokens: int = 0
    counter_reset: bool = False


@dataclass
class Rollout:
    path: Path
    session_id: str
    parent_id: str | None = None
    agent_path: str | None = None
    model: str | None = None
    counters: list[Counter] = field(default_factory=list)
    interactions: list[tuple[str, str]] = field(default_factory=list)
    status: str = "unknown"
    last_timestamp: str | None = None
    warnings: list[str] = field(default_factory=list)


def parse_timestamp(value: str) -> datetime:
    return datetime.fromisoformat(value.replace("Z", "+00:00")).astimezone(timezone.utc)


def timestamp_key(value: str) -> tuple[datetime, str]:
    return (parse_timestamp(value), value)


def safe_int(value: Any) -> int | None:
    return value if isinstance(value, int) and not isinstance(value, bool) and value >= 0 else None


def nested(value: Any, *keys: str) -> Any:
    for key in keys:
        if not isinstance(value, dict):
            return None
        value = value.get(key)
    return value


def interaction_label(payload: dict[str, Any]) -> str | None:
    """Return an honest observable kind; never incorporate private contents."""
    kind = payload.get("type")
    if kind == "user_message":
        return "user_message"
    if kind == "agent_message":
        phase = payload.get("phase")
        return f"agent_message:{phase}" if isinstance(phase, str) else "agent_message"
    if kind == "task_started":
        return "task_started"
    if kind == "task_complete":
        return "task_complete"
    if kind == "sub_agent_activity":
        event_kind = payload.get("kind")
        return f"sub_agent_activity:{event_kind}" if isinstance(event_kind, str) else "sub_agent_activity"
    if kind == "function_call":
        name = payload.get("name")
        return f"function_call:{name}" if isinstance(name, str) else "function_call"
    if kind == "function_call_output":
        return "function_call_output"
    return None


def parse_rollout(path: Path) -> Rollout | None:
    """Parse structural metadata and counters only, tolerating malformed rows."""
    session_id: str | None = None
    parent_id: str | None = None
    agent_path: str | None = None
    model: str | None = None
    counters: list[Counter] = []
    interactions: list[tuple[str, str]] = []
    warnings: list[str] = []
    status = "unknown"
    last_timestamp: str | None = None
    seen_interactions: set[tuple[str, str]] = set()
    try:
        lines = path.open(encoding="utf-8")
    except OSError as exc:
        return Rollout(path, path.stem, warnings=[f"cannot read rollout: {exc}"])
    with lines:
        for line_number, line in enumerate(lines, 1):
            try:
                row = json.loads(line)
            except json.JSONDecodeError:
                warnings.append(f"malformed JSON line {line_number}; skipped")
                continue
            if not isinstance(row, dict):
                continue
            timestamp = row.get("timestamp")
            payload = row.get("payload")
            if not isinstance(timestamp, str) or not isinstance(payload, dict):
                continue
            try:
                parse_timestamp(timestamp)
            except ValueError:
                warnings.append(f"invalid timestamp at line {line_number}; skipped")
                continue
            last_timestamp = max(last_timestamp, timestamp, key=timestamp_key) if last_timestamp else timestamp
            if row.get("type") == "session_meta":
                candidate = payload.get("id") or payload.get("session_id")
                # Forked rollouts can append inherited metadata for an ancestor
                # after their own initial session_meta row.  The first valid
                # identity belongs to this file; later identities are context,
                # not a session-id update.
                if session_id is None and isinstance(candidate, str):
                    session_id = candidate
                candidate_parent = payload.get("forked_from_id")
                if isinstance(candidate_parent, str):
                    parent_id = candidate_parent
                spawn = nested(payload, "source", "subagent", "thread_spawn")
                if isinstance(spawn, dict):
                    candidate_parent = spawn.get("parent_thread_id")
                    if isinstance(candidate_parent, str):
                        parent_id = candidate_parent
                    candidate_path = spawn.get("agent_path")
                    if isinstance(candidate_path, str):
                        agent_path = candidate_path
                candidate_model = payload.get("model")
                if isinstance(candidate_model, str):
                    model = candidate_model
            candidate_model = payload.get("model")
            if isinstance(candidate_model, str):
                model = candidate_model
            label = interaction_label(payload)
            if label is not None and (timestamp, label) not in seen_interactions:
                interactions.append((timestamp, label))
                seen_interactions.add((timestamp, label))
                if label == "task_started":
                    status = "running"
                elif label == "task_complete":
                    status = "complete"
            info = nested(payload, "info", "total_token_usage")
            if payload.get("type") == "token_count" and isinstance(info, dict):
                values = [safe_int(info.get(key)) for key in ("input_tokens", "cached_input_tokens", "output_tokens")]
                if any(value is None for value in values):
                    warnings.append(f"invalid token counter at {timestamp}; skipped")
                    continue
                input_tokens, cached_tokens, output_tokens = values
                if cached_tokens > input_tokens:
                    warnings.append(f"cached input exceeds input at {timestamp}; skipped")
                    continue
                counters.append(Counter(timestamp, input_tokens, cached_tokens, output_tokens))
    if session_id is None:
        # A filename is only a display fallback; it cannot drive descendant discovery.
        session_id = path.stem.rsplit("-", 1)[-1]
        warnings.append("session metadata missing; using filename suffix as display id")
    counters.sort(key=lambda counter: timestamp_key(counter.timestamp))
    interactions.sort(key=lambda item: timestamp_key(item[0]))
    return Rollout(path, session_id, parent_id, agent_path, model, counters, interactions, status, last_timestamp, warnings)


def read_metadata(path: Path) -> Rollout | None:
    """Read the initial metadata record without parsing an entire large rollout.

    Current rollout writers put ``session_meta`` at the beginning.  A bounded
    scan keeps discovery practical across a long session archive; a rollout
    without such a record is deliberately not used to infer lineage.
    """
    try:
        lines = path.open(encoding="utf-8")
    except OSError:
        return None
    with lines:
        for line_number, line in enumerate(lines, 1):
            if line_number > 256:
                return None
            try:
                row = json.loads(line)
            except json.JSONDecodeError:
                continue
            if not isinstance(row, dict) or row.get("type") != "session_meta":
                continue
            payload = row.get("payload")
            if not isinstance(payload, dict):
                return None
            session_id = payload.get("id") or payload.get("session_id")
            if not isinstance(session_id, str):
                return None
            parent_id = payload.get("forked_from_id") if isinstance(payload.get("forked_from_id"), str) else None
            agent_path = None
            spawn = nested(payload, "source", "subagent", "thread_spawn")
            if isinstance(spawn, dict):
                if isinstance(spawn.get("parent_thread_id"), str):
                    parent_id = spawn["parent_thread_id"]
                if isinstance(spawn.get("agent_path"), str):
                    agent_path = spawn["agent_path"]
            model = payload.get("model") if isinstance(payload.get("model"), str) else None
            return Rollout(path, session_id, parent_id, agent_path, model)
    return None


def rollout_paths(roots: Iterable[Path]) -> Iterable[Path]:
    for root in roots:
        if root.is_file() and root.suffix == ".jsonl":
            yield root
        elif root.is_dir():
            yield from root.rglob("rollout-*.jsonl")


def discover(target: str, roots: list[Path]) -> list[Rollout]:
    target_path = Path(target).expanduser()
    if target_path.is_file():
        root = parse_rollout(target_path)
        assert root is not None
        root_id = root.session_id
        candidates = [root]
        scan_paths = [path for path in rollout_paths(roots) if path != target_path]
    else:
        scan_paths = list(rollout_paths(roots))
        candidates = []
    # Metadata is source truth.  We parse all candidates structurally to find
    # parent links, but retain one file per session id to avoid double charging.
    by_id: dict[str, Rollout] = {}
    for rollout in candidates + [read_metadata(path) for path in scan_paths]:
        if rollout is None:
            continue
        existing = by_id.get(rollout.session_id)
        if existing is None or len(rollout.counters) > len(existing.counters):
            by_id[rollout.session_id] = rollout
    root_id = root_id if target_path.is_file() else target
    if root_id not in by_id:
        raise ValueError(f"no rollout metadata found for session/thread id {target!r}")
    result: list[Rollout] = []
    pending = [root_id]
    seen: set[str] = set()
    while pending:
        current = pending.pop()
        if current in seen:
            continue
        seen.add(current)
        rollout = by_id.get(current)
        if rollout is not None:
            result.append(rollout)
        pending.extend(child.session_id for child in by_id.values() if child.parent_id == current)
    # The metadata pass intentionally did not inspect token events.  Parse the
    # selected lineage fully only after descendant discovery is complete.
    selected: list[Rollout] = []
    for item in result:
        parsed = root if target_path.is_file() and item.path == target_path else parse_rollout(item.path)
        if parsed is not None:
            selected.append(parsed)
    return selected


def counter_deltas(rollout: Rollout, since: datetime | None) -> tuple[Counter, list[Checkpoint], list[str]]:
    """Turn cumulative rows into deltas; a decrease starts a clearly flagged segment."""
    total = Counter("", 0, 0, 0)
    checkpoints: dict[tuple[str, str], Checkpoint] = {}
    warnings = list(rollout.warnings)
    previous: Counter | None = None
    interactions = rollout.interactions
    for counter in rollout.counters:
        current_time = parse_timestamp(counter.timestamp)
        if since is not None and current_time <= since:
            previous = counter
            continue
        reset = previous is not None and any(
            current < prior
            for current, prior in zip(
                (counter.input_tokens, counter.cached_input_tokens, counter.output_tokens),
                (previous.input_tokens, previous.cached_input_tokens, previous.output_tokens),
            )
        )
        delta = counter if reset else counter.minus(previous)
        if reset:
            warnings.append(f"counter reset/nonmonotone stream at {counter.timestamp}; charged new segment from zero")
        previous = counter
        total = Counter("", total.input_tokens + delta.input_tokens, total.cached_input_tokens + delta.cached_input_tokens, total.output_tokens + delta.output_tokens)
        prior_events = [event for event in interactions if timestamp_key(event[0]) <= timestamp_key(counter.timestamp)]
        if prior_events:
            event_time, label = prior_events[-1]
        else:
            event_time, label = counter.timestamp, "session_start/no observable interaction"
        key = (event_time, label)
        checkpoint = checkpoints.setdefault(key, Checkpoint(event_time, label))
        checkpoint.input_tokens += delta.input_tokens
        checkpoint.cached_input_tokens += delta.cached_input_tokens
        checkpoint.output_tokens += delta.output_tokens
        checkpoint.counter_reset = checkpoint.counter_reset or reset
    return total, sorted(checkpoints.values(), key=lambda item: timestamp_key(item.timestamp)), warnings


def cost(counter: Counter | Checkpoint, rates: tuple[float, float, float]) -> float:
    input_rate, cached_rate, output_rate = rates
    return ((counter.input_tokens - counter.cached_input_tokens) * input_rate + counter.cached_input_tokens * cached_rate + counter.output_tokens * output_rate) / 1_000_000


def render(data: dict[str, Any]) -> str:
    rates = data["rates"]
    if rates["override"] is not None:
        cards = f"override for all sessions: {tuple(rates['override'].values())}"
    else:
        cards = "; ".join(f"{name}={tuple(values.values())}" for name, values in rates["cards"].items())
    lines = [
        f"Rates USD/Mtok (input, cached input, output): {cards} [{rates['source']}]",
        f"Scope: {data['target']}" + (f" since {data['since']}" if data["since"] else ""),
        "",
        "Sessions:",
        "  agent/session                         model          status      uncached       cached       output       cost",
    ]
    for session in data["sessions"]:
        agent = session["agent_path"] or ("/root" if session["parent_id"] is None else "(subagent path unavailable)")
        display = f"{agent} [{session['session_id'][:8]}]"
        lines.append(
            f"  {display[:36]:36} {session['model'][:14]:14} {session['status'][:10]:10} "
            f"{session['uncached_input_tokens']:12,d} {session['cached_input_tokens']:12,d} {session['output_tokens']:12,d} ${session['cost_usd']:9.4f}"
        )
    total = data["total"]
    lines.extend([
        f"  TOTAL{'':32} {'':14} {'':10} {total['uncached_input_tokens']:12,d} {total['cached_input_tokens']:12,d} {total['output_tokens']:12,d} ${total['cost_usd']:9.4f}",
        "",
        "Checkpoints (token deltas attributed after the last observable event; no bodies shown):",
        "  timestamp                    session    event                                  uncached       cached       output       cost",
    ])
    for checkpoint in data["checkpoints"]:
        reset = " [reset]" if checkpoint["counter_reset"] else ""
        lines.append(
            f"  {checkpoint['timestamp'][:27]:27} {checkpoint['session_id'][:8]:8} {checkpoint['label'][:38]:38}{reset:8} "
            f"{checkpoint['uncached_input_tokens']:12,d} {checkpoint['cached_input_tokens']:12,d} {checkpoint['output_tokens']:12,d} ${checkpoint['cost_usd']:9.4f}"
        )
    if data["warnings"]:
        lines.extend(["", "Warnings:"] + [f"  - {warning}" for warning in data["warnings"]])
    return "\n".join(lines)


def rate_card(model: str | None, model_override: str | None, override_rates: tuple[float, float, float] | None) -> tuple[str, tuple[float, float, float]]:
    if override_rates is not None:
        return (model_override or model or "custom", override_rates)
    selected = model_override or model
    if selected not in DEFAULT_RATES:
        observed = repr(selected) if selected is not None else "missing"
        raise ValueError(
            f"no rate card for model {observed}; pass --model or all three explicit rates"
        )
    return selected, DEFAULT_RATES[selected]


def rate_dict(rates: tuple[float, float, float]) -> dict[str, float]:
    return {"input": rates[0], "cached_input": rates[1], "output": rates[2]}


def build_report(target: str, roots: list[Path], model_override: str | None, override_rates: tuple[float, float, float] | None, since: datetime | None) -> dict[str, Any]:
    sessions = discover(target, roots)
    rows: list[dict[str, Any]] = []
    checkpoints: list[dict[str, Any]] = []
    warnings: list[str] = []
    total = Counter("", 0, 0, 0)
    total_cost = 0.0
    for rollout in sessions:
        token_delta, session_checkpoints, session_warnings = counter_deltas(rollout, since)
        total = Counter("", total.input_tokens + token_delta.input_tokens, total.cached_input_tokens + token_delta.cached_input_tokens, total.output_tokens + token_delta.output_tokens)
        model, session_rates = rate_card(rollout.model, model_override, override_rates)
        session_cost = cost(token_delta, session_rates)
        total_cost += session_cost
        row = {
            "session_id": rollout.session_id,
            "path": str(rollout.path),
            "parent_id": rollout.parent_id,
            "agent_path": rollout.agent_path,
            "model": model,
            "status": rollout.status,
            "last_timestamp": rollout.last_timestamp,
            "input_tokens": token_delta.input_tokens,
            "cached_input_tokens": token_delta.cached_input_tokens,
            "uncached_input_tokens": token_delta.input_tokens - token_delta.cached_input_tokens,
            "output_tokens": token_delta.output_tokens,
            "rates": rate_dict(session_rates),
            "cost_usd": round(session_cost, 8),
            "warnings": session_warnings,
        }
        rows.append(row)
        for checkpoint in session_checkpoints:
            checkpoints.append({
                "session_id": rollout.session_id,
                "timestamp": checkpoint.timestamp,
                "label": checkpoint.label,
                "input_tokens": checkpoint.input_tokens,
                "cached_input_tokens": checkpoint.cached_input_tokens,
                "uncached_input_tokens": checkpoint.input_tokens - checkpoint.cached_input_tokens,
                "output_tokens": checkpoint.output_tokens,
                "cost_usd": round(cost(checkpoint, session_rates), 8),
                "counter_reset": checkpoint.counter_reset,
            })
        warnings.extend(f"{rollout.session_id}: {warning}" for warning in session_warnings)
    checkpoints.sort(key=lambda item: timestamp_key(item["timestamp"]))
    return {
        "target": target,
        "since": since.isoformat().replace("+00:00", "Z") if since else None,
        "rates": {
            "cards": {name: rate_dict(card) for name, card in DEFAULT_RATES.items()},
            "override": rate_dict(override_rates) if override_rates is not None else None,
            "source": RATE_SOURCE,
        },
        "sessions": rows,
        "checkpoints": checkpoints,
        "total": {
            "input_tokens": total.input_tokens,
            "cached_input_tokens": total.cached_input_tokens,
            "uncached_input_tokens": total.input_tokens - total.cached_input_tokens,
            "output_tokens": total.output_tokens,
            "cost_usd": round(total_cost, 8),
        },
        "warnings": warnings,
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("target", help="root rollout JSONL path or root session/thread id")
    parser.add_argument("--log-root", action="append", type=Path, help="session directory to search (repeatable); defaults to ~/.codex session roots")
    parser.add_argument("--since", help="UTC/RFC3339 boundary; subtracts the last counter at or before it")
    parser.add_argument("--model", choices=sorted(DEFAULT_RATES), help="declare the rate card model for all sessions")
    parser.add_argument("--input-rate", type=float, help="USD per million uncached-input tokens")
    parser.add_argument("--cached-input-rate", type=float, help="USD per million cached-input tokens")
    parser.add_argument("--output-rate", type=float, help="USD per million output tokens")
    parser.add_argument("--json", action="store_true", help="write the machine-readable report as JSON")
    args = parser.parse_args(argv)
    if any(rate is not None and rate < 0 for rate in (args.input_rate, args.cached_input_rate, args.output_rate)):
        parser.error("rates must be non-negative")
    if any(rate is not None for rate in (args.input_rate, args.cached_input_rate, args.output_rate)) and not all(rate is not None for rate in (args.input_rate, args.cached_input_rate, args.output_rate)):
        parser.error("override all three rates together")
    rates = (args.input_rate, args.cached_input_rate, args.output_rate) if args.input_rate is not None else None
    try:
        since = parse_timestamp(args.since) if args.since else None
    except ValueError:
        parser.error("--since must be an ISO-8601 timestamp, for example 2026-07-17T17:18:45.308Z")
    try:
        report = build_report(args.target, args.log_root or list(DEFAULT_LOG_ROOTS), args.model, rates, since)
    except ValueError as exc:
        parser.error(str(exc))
    if args.json:
        json.dump(report, sys.stdout, indent=2, sort_keys=True)
        print()
    else:
        print(render(report))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
