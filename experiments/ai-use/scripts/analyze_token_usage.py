#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib"]
# ///
"""Aggregate Codex token-usage events for diagnosis and optional plotting.

The raw rollout JSONL files are the source evidence.  This producer deliberately
does not infer billing cost, research impact, or authorship.  It aggregates the
recorded ``last_token_usage`` fields by the timestamp of each token-count event,
while removing repeated token-count records whose cumulative usage is unchanged.

Example::

    uv run --script experiments/ai-use/scripts/analyze_token_usage.py \
      --start 2026-07-04 --end 2026-07-12 \
      --cutoff 2026-07-12T19:39:00Z \
      --exclude-thread-id 019f57d7-2e11-7181-aaab-685f65245ca8 \
      --out-dir /tmp/codex-token-usage --plot

The default roots are the native Codex session locations.  Use ``--root`` to
replace them, for example when analyzing an imported log archive.
"""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import os
import sys
from collections import defaultdict
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Iterable, Iterator


USAGE_KEYS = (
    "total_tokens",
    "input_tokens",
    "cached_input_tokens",
    "output_tokens",
    "reasoning_output_tokens",
)
CODEX_HOME = Path(os.environ.get("CODEX_HOME", Path.home() / ".codex"))
DEFAULT_ROOTS = (CODEX_HOME / "sessions", CODEX_HOME / "archived_sessions")
SHADOW_PRICES = {
    "gpt-5.4": (2.50, 0.25, 15.00),
    "gpt-5.5": (5.00, 0.50, 30.00),
    "gpt-5.6-sol": (5.00, 0.50, 30.00),
    "gpt-5.6-terra": (2.50, 0.25, 15.00),
    "gpt-5.6-luna": (1.00, 0.10, 6.00),
    "gpt-5.4-mini": (0.75, 0.075, 4.50),
}
LONG_CONTEXT_THRESHOLD = 272_000


@dataclass(frozen=True)
class UsageEvent:
    date: str
    timestamp: str
    path: str
    rollout_id: str
    model: str
    effort: str
    source: str
    depth: str
    usage: tuple[int, ...]


def require_disjoint_roots(
    parser: argparse.ArgumentParser, roots: list[Path]
) -> None:
    resolved = [(root, root.resolve(strict=True)) for root in roots if root.is_dir()]
    for index, (left, left_resolved) in enumerate(resolved):
        for right, right_resolved in resolved[index + 1 :]:
            if (
                left_resolved == right_resolved
                or left_resolved in right_resolved.parents
                or right_resolved in left_resolved.parents
            ):
                parser.error(
                    f"rollout roots must not repeat or overlap: {left}, {right}"
                )


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--start", required=True, help="First UTC date, YYYY-MM-DD")
    parser.add_argument("--end", required=True, help="Last UTC date, YYYY-MM-DD")
    parser.add_argument(
        "--cutoff",
        help="Optional inclusive UTC timestamp; useful for excluding this analysis session",
    )
    parser.add_argument(
        "--root",
        action="append",
        dest="roots",
        help="Rollout root; repeat to add roots (replaces defaults)",
    )
    parser.add_argument(
        "--exclude-thread-id",
        action="append",
        default=[],
        help="Exclude rollout filenames containing this thread id",
    )
    parser.add_argument("--out-dir", required=True, type=Path)
    parser.add_argument(
        "--plot",
        action="store_true",
        help="Also write token-usage-overview.png (requires matplotlib)",
    )
    parser.add_argument(
        "--plot-bucket",
        choices=("daily", "month"),
        default="daily",
        help="Time bucket for the optional plot (CSV outputs remain daily)",
    )
    args = parser.parse_args()
    if args.start > args.end:
        parser.error("--start must not be later than --end")
    if args.cutoff:
        args.cutoff = args.cutoff.replace("Z", "+00:00")
        try:
            datetime.fromisoformat(args.cutoff)
        except ValueError as exc:
            parser.error(f"invalid --cutoff timestamp: {exc}")
    explicit_roots = args.roots is not None
    args.roots = [Path(root) for root in (args.roots or DEFAULT_ROOTS)]
    missing_roots = [root for root in args.roots if not root.is_dir()]
    if explicit_roots and missing_roots:
        parser.error(
            "explicit rollout roots do not exist: "
            + ", ".join(str(root) for root in missing_roots)
        )
    if len(missing_roots) == len(args.roots):
        parser.error(
            "none of the rollout roots exists; set CODEX_HOME or pass one or more --root values"
        )
    require_disjoint_roots(parser, args.roots)
    args.root_file_counts = {
        str(root): sum(1 for _ in root.rglob("rollout-*.jsonl"))
        for root in args.roots
        if root.is_dir()
    }
    empty_explicit_roots = [
        root
        for root in args.roots
        if explicit_roots and args.root_file_counts[str(root)] == 0
    ]
    if empty_explicit_roots:
        parser.error(
            "explicit rollout roots contain no rollout files: "
            + ", ".join(str(root) for root in empty_explicit_roots)
        )
    args.missing_default_roots = missing_roots if not explicit_roots else []
    return args


def json_objects(path: Path) -> Iterator[dict[str, Any]]:
    try:
        with path.open("r", encoding="utf-8", errors="replace") as handle:
            for line in handle:
                try:
                    value = json.loads(line)
                except json.JSONDecodeError:
                    continue
                if isinstance(value, dict):
                    yield value
    except OSError:
        return


def rollout_paths(roots: Iterable[Path]) -> Iterator[Path]:
    seen: set[Path] = set()
    seen_names: set[str] = set()
    for root in roots:
        if not root.exists():
            continue
        for path in sorted(root.rglob("rollout-*.jsonl")):
            resolved = path.resolve()
            # Imported archives can contain a second copy of a native rollout.
            # Prefer the first root supplied by the caller and de-duplicate by
            # rollout filename as well as resolved path.
            if resolved not in seen and path.name not in seen_names:
                seen.add(resolved)
                seen_names.add(path.name)
                yield resolved


def context_values(payload: dict[str, Any]) -> tuple[str | None, str | None]:
    settings = payload.get("collaboration_mode", {}).get("settings", {})
    if not isinstance(settings, dict):
        settings = {}
    model = payload.get("model") or settings.get("model")
    effort = payload.get("effort") or settings.get("reasoning_effort")
    return (
        str(model) if model else None,
        str(effort) if effort else None,
    )


def lineage_values(payload: dict[str, Any]) -> tuple[str, str]:
    source = payload.get("source")
    subagent = source.get("subagent") if isinstance(source, dict) else None
    spawn = subagent.get("thread_spawn", {}) if isinstance(subagent, dict) else {}
    is_subagent = payload.get("thread_source") == "subagent" or bool(spawn)
    source_name = "subagent" if is_subagent else "user"
    depth = spawn.get("depth") if isinstance(spawn, dict) else None
    return source_name, str(depth) if depth is not None else "root"


def first_metadata(path: Path) -> tuple[str, str, str, str]:
    model = effort = None
    source = "user"
    depth = "root"
    session_meta_seen = False
    for event in json_objects(path):
        payload = event.get("payload")
        if not isinstance(payload, dict):
            continue
        if event.get("type") == "session_meta":
            if not session_meta_seen:
                source, depth = lineage_values(payload)
                session_meta_seen = True
        if event.get("type") == "turn_context":
            candidate_model, candidate_effort = context_values(payload)
            model = model or candidate_model
            effort = effort or candidate_effort
    return model or "unknown", effort or "unknown", source, depth


def timestamp_in_window(timestamp: str, start: str, end: str, cutoff: str | None) -> bool:
    if len(timestamp) < 10 or not (start <= timestamp[:10] <= end):
        return False
    if cutoff is None:
        return True
    timestamp_value = datetime.fromisoformat(timestamp.replace("Z", "+00:00"))
    cutoff_value = datetime.fromisoformat(cutoff.replace("Z", "+00:00"))
    return timestamp_value <= cutoff_value


def collect_events(
    roots: Iterable[Path],
    start: str,
    end: str,
    cutoff: str | None,
    excluded_ids: set[str],
) -> tuple[list[UsageEvent], dict[str, int]]:
    events: list[UsageEvent] = []
    stats = defaultdict(int)
    for path in rollout_paths(roots):
        stats["files_scanned"] += 1
        if any(thread_id in path.name for thread_id in excluded_ids):
            stats["files_excluded"] += 1
            continue
        initial_model, initial_effort, source, depth = first_metadata(path)
        model, effort = initial_model, initial_effort
        previous_total: tuple[int, ...] | None = None
        rollout_id = hashlib.sha256(str(path).encode()).hexdigest()[:16]
        for event in json_objects(path):
            payload = event.get("payload")
            if not isinstance(payload, dict):
                continue
            if event.get("type") == "turn_context":
                candidate_model, candidate_effort = context_values(payload)
                model = candidate_model or model
                effort = candidate_effort or effort
                continue
            if event.get("type") != "event_msg" or payload.get("type") != "token_count":
                continue
            info = payload.get("info")
            if not isinstance(info, dict):
                stats["token_events_without_info"] += 1
                continue
            total = info.get("total_token_usage")
            last = info.get("last_token_usage")
            if not isinstance(total, dict) or not isinstance(last, dict):
                stats["token_events_without_usage"] += 1
                continue
            current_total = tuple(int(total.get(key, 0) or 0) for key in USAGE_KEYS)
            if current_total == previous_total:
                stats["duplicate_token_events_skipped"] += 1
                continue
            previous_total = current_total
            timestamp = str(event.get("timestamp", ""))
            if not timestamp_in_window(timestamp, start, end, cutoff):
                continue
            usage = tuple(int(last.get(key, 0) or 0) for key in USAGE_KEYS)
            events.append(
                UsageEvent(
                    date=timestamp[:10],
                    timestamp=timestamp,
                    path=str(path),
                    rollout_id=rollout_id,
                    model=model,
                    effort=effort,
                    source=source,
                    depth=depth,
                    usage=usage,
                )
            )
    stats["usage_events"] = len(events)
    return events, dict(stats)


def add_usage(target: dict[str, Any], usage: tuple[int, ...]) -> None:
    for key, value in zip(USAGE_KEYS, usage):
        target[key] = int(target.get(key, 0)) + value


def finalize(row: dict[str, Any]) -> dict[str, Any]:
    row = dict(row)
    row["uncached_input_tokens"] = row["input_tokens"] - row["cached_input_tokens"]
    row["cache_hit_rate"] = (
        row["cached_input_tokens"] / row["input_tokens"]
        if row["input_tokens"]
        else None
    )
    return row


def aggregate(events: list[UsageEvent]) -> dict[str, list[dict[str, Any]]]:
    groups: dict[str, dict[tuple[str, ...], dict[str, Any]]] = {
        "daily": {},
        "model_effort": {},
        "lineage": {},
        "rollout_daily": {},
    }
    rollout_sets: dict[tuple[str, tuple[str, ...]], set[str]] = defaultdict(set)
    for event in events:
        dimensions = {
            "daily": (event.date,),
            "model_effort": (event.date, event.model, event.effort),
            "lineage": (event.date, event.source, event.depth),
            "rollout_daily": (event.date, event.rollout_id, event.model, event.effort, event.path),
        }
        for name, key in dimensions.items():
            row = groups[name].setdefault(
                key,
                {"date": event.date, **({"model": event.model, "effort": event.effort} if name == "model_effort" else {})}
                | ({"source": event.source, "depth": event.depth} if name == "lineage" else {})
                | ({"rollout_id": event.rollout_id, "model": event.model, "effort": event.effort, "path": event.path} if name == "rollout_daily" else {})
                | {"events": 0},
            )
            row["events"] += 1
            add_usage(row, event.usage)
            rollout_sets[(name, key)].add(event.rollout_id)
    for name, rows in groups.items():
        for key, row in rows.items():
            row["rollouts"] = len(rollout_sets[(name, key)])
            row.update(finalize(row))
        groups[name] = sorted(rows.values(), key=lambda row: tuple(str(row.get(k, "")) for k in ("date", "model", "effort", "source", "depth", "rollout_id", "path")))
    return groups


def shadow_cost_rows(events: list[UsageEvent]) -> list[dict[str, Any]]:
    """Estimate public API-equivalent cost; this is not subscription billing."""
    groups: dict[tuple[str, str, str], dict[str, Any]] = {}
    for event in events:
        prices = SHADOW_PRICES.get(event.model)
        if prices is None:
            continue
        input_price, cached_price, output_price = prices
        input_tokens = event.usage[1]
        cached_tokens = event.usage[2]
        output_tokens = event.usage[3]
        uncached_tokens = input_tokens - cached_tokens
        long_context = input_tokens > LONG_CONTEXT_THRESHOLD
        input_multiplier = 2.0 if long_context else 1.0
        output_multiplier = 1.5 if long_context else 1.0
        key = (event.date, event.model, event.effort)
        row = groups.setdefault(
            key,
            {
                "date": event.date,
                "model": event.model,
                "effort": event.effort,
                "events": 0,
                "input_tokens": 0,
                "cached_input_tokens": 0,
                "uncached_input_tokens": 0,
                "output_tokens": 0,
                "long_context_requests": 0,
                "baseline_cost_usd": 0.0,
                "long_context_cost_usd": 0.0,
                "input_price_per_million": input_price,
                "cached_input_price_per_million": cached_price,
                "output_price_per_million": output_price,
            },
        )
        row["events"] += 1
        row["input_tokens"] += input_tokens
        row["cached_input_tokens"] += cached_tokens
        row["uncached_input_tokens"] += uncached_tokens
        row["output_tokens"] += output_tokens
        row["long_context_requests"] += int(long_context)
        row["baseline_cost_usd"] += (
            uncached_tokens / 1_000_000 * input_price
            + cached_tokens / 1_000_000 * cached_price
            + output_tokens / 1_000_000 * output_price
        )
        row["long_context_cost_usd"] += (
            (uncached_tokens / 1_000_000 * input_price
             + cached_tokens / 1_000_000 * cached_price) * input_multiplier
            + output_tokens / 1_000_000 * output_price * output_multiplier
        )
    for row in groups.values():
        row["baseline_cost_usd"] = round(row["baseline_cost_usd"], 6)
        row["long_context_cost_usd"] = round(row["long_context_cost_usd"], 6)
    return sorted(groups.values(), key=lambda row: (row["date"], row["model"], row["effort"]))


def write_csv(path: Path, rows: list[dict[str, Any]]) -> None:
    if not rows:
        path.write_text("\n", encoding="utf-8")
        return
    preferred = [
        "date", "model", "effort", "source", "depth", "rollout_id", "path", "events", "rollouts",
        *USAGE_KEYS, "uncached_input_tokens", "cache_hit_rate",
        "long_context_requests", "baseline_cost_usd", "long_context_cost_usd",
        "input_price_per_million", "cached_input_price_per_million", "output_price_per_million",
    ]
    fields = [field for field in preferred if any(field in row for row in rows)]
    with path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fields, extrasaction="ignore")
        writer.writeheader()
        writer.writerows(rows)


def plot_overview(groups: dict[str, list[dict[str, Any]]], out_path: Path, bucket: str) -> None:
    try:
        import matplotlib.pyplot as plt
    except ImportError as exc:
        raise SystemExit("--plot requires matplotlib; run through `uv run --script`") from exc

    repo_root = Path(__file__).resolve().parents[3]
    sys.path.insert(0, str(repo_root / "experiments"))
    from figure_config import FIGSIZE_DUAL, setup  # type: ignore[import-not-found]

    setup()
    def bucket_date(date: str) -> str:
        return date[:7] if bucket == "month" else date

    daily: dict[str, dict[str, Any]] = {}
    for row in groups["daily"]:
        target = daily.setdefault(bucket_date(row["date"]), {"total_tokens": 0, "input_tokens": 0, "cached_input_tokens": 0})
        for key in ("total_tokens", "input_tokens", "cached_input_tokens"):
            target[key] += row[key]
    for target in daily.values():
        target["cache_hit_rate"] = (
            target["cached_input_tokens"] / target["input_tokens"]
            if target["input_tokens"]
            else float("nan")
        )
    model_rows: list[dict[str, Any]] = []
    for row in groups["model_effort"]:
        target = next((candidate for candidate in model_rows if candidate["date"] == bucket_date(row["date"]) and candidate["model"] == row["model"]), None)
        if target is None:
            target = {"date": bucket_date(row["date"]), "model": row["model"], "total_tokens": 0}
            model_rows.append(target)
        target["total_tokens"] += row["total_tokens"]
    lineage_rows: list[dict[str, Any]] = []
    for row in groups["lineage"]:
        target = next((candidate for candidate in lineage_rows if candidate["date"] == bucket_date(row["date"]) and candidate["source"] == row["source"]), None)
        if target is None:
            target = {"date": bucket_date(row["date"]), "source": row["source"], "total_tokens": 0}
            lineage_rows.append(target)
        target["total_tokens"] += row["total_tokens"]
    dates = sorted(daily)
    model_totals: dict[str, int] = defaultdict(int)
    for row in model_rows:
        model_totals[row["model"]] += row["total_tokens"]
    top_models = [model for model, _ in sorted(model_totals.items(), key=lambda item: item[1], reverse=True)[:5]]
    models = top_models + (["other models"] if len(top_models) < len(model_totals) else [])
    colors = {
        "gpt-5.4-mini": "#777777",
        "gpt-5": "#332288",
        "gpt-5-codex": "#88ccee",
        "gpt-5.4": "#ee6677",
        "gpt-5.5": "#4477aa",
        "gpt-5.6-sol": "#228833",
        "gpt-5.6-terra": "#cc6677",
        "gpt-5.6-luna": "#aa3377",
        "unknown": "#999999",
    }
    fig, axes = plt.subplots(2, 1, figsize=(FIGSIZE_DUAL[0], 5.4), sharex=True)
    x = list(range(len(dates)))
    bottoms = [0.0] * len(dates)
    for model in models:
        model_names = set(model_totals) - set(top_models) if model == "other models" else {model}
        values = [
            sum(row["total_tokens"] for row in model_rows if row["date"] == date and row["model"] in model_names) / 1e9
            for date in dates
        ]
        axes[0].bar(x, values, bottom=bottoms, label=model, color=colors.get(model, "#999999"))
        bottoms = [bottom + value for bottom, value in zip(bottoms, values)]
    axes[0].set_ylabel("tokens (billions)")
    axes[0].set_title("Recorded Codex usage by model")
    axes[0].legend(ncol=3, frameon=False)

    cache = [100 * daily[date]["cache_hit_rate"] if daily[date]["cache_hit_rate"] is not None else float("nan") for date in dates]
    total_subagent = {date: 0 for date in dates}
    for row in lineage_rows:
        if row["source"] == "subagent":
            total_subagent[row["date"]] = total_subagent.get(row["date"], 0) + row["total_tokens"]
    subagent_share = [100 * total_subagent[date] / daily[date]["total_tokens"] if daily[date]["total_tokens"] else float("nan") for date in dates]
    axes[1].plot(x, cache, marker="o", label="cache-hit input", color="#4477aa")
    axes[1].plot(x, subagent_share, marker="o", label="subagent share", color="#cc6677")
    axes[1].set_ylim(0, 100)
    axes[1].set_ylabel("percent")
    axes[1].set_title("Cache reuse and subagent contribution")
    axes[1].legend(frameon=False)
    axes[1].set_xticks(x, dates, rotation=35, ha="right")
    fig.tight_layout()
    fig.savefig(out_path)
    plt.close(fig)


def main() -> None:
    args = parse_args()
    events, stats = collect_events(args.roots, args.start, args.end, args.cutoff, set(args.exclude_thread_id))
    if stats.get("files_scanned", 0) == 0:
        raise SystemExit("error: no rollout files found below the available roots")
    groups = aggregate(events)
    cost_rows = shadow_cost_rows(events)
    args.out_dir.mkdir(parents=True, exist_ok=True)
    write_csv(args.out_dir / "daily.csv", groups["daily"])
    write_csv(args.out_dir / "model-effort.csv", groups["model_effort"])
    write_csv(args.out_dir / "lineage.csv", groups["lineage"])
    write_csv(args.out_dir / "rollout-daily.csv", groups["rollout_daily"])
    write_csv(args.out_dir / "shadow-cost.csv", cost_rows)
    metadata = {
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "roots": [str(root) for root in args.roots],
        "roots_scanned": [str(root) for root in args.roots if root.is_dir()],
        "root_rollout_file_counts": args.root_file_counts,
        "missing_default_roots": [str(root) for root in args.missing_default_roots],
        "start": args.start,
        "end": args.end,
        "cutoff": args.cutoff,
        "excluded_thread_ids": args.exclude_thread_id,
        "stats": stats,
        "shadow_pricing": {
            "as_of": "2026-07-12",
            "long_context_threshold_tokens": LONG_CONTEXT_THRESHOLD,
            "long_context_input_multiplier": 2.0,
            "long_context_output_multiplier": 1.5,
            "rates_usd_per_million": {
                model: {"input": prices[0], "cached_input": prices[1], "output": prices[2]}
                for model, prices in SHADOW_PRICES.items()
            },
            "sources": [
                "https://developers.openai.com/api/docs/models/gpt-5.4",
                "https://developers.openai.com/api/docs/models/gpt-5.5",
                "https://developers.openai.com/api/docs/models/gpt-5.6-sol",
                "https://developers.openai.com/api/docs/models/gpt-5.6-terra",
                "https://developers.openai.com/api/docs/models/gpt-5.6-luna",
                "https://developers.openai.com/api/docs/models/gpt-5.4-mini",
            ],
        },
        "notes": [
            "Totals use last_token_usage from token-count events.",
            "Repeated records with identical cumulative total_token_usage are skipped.",
            "Dates and cutoff are interpreted in UTC from event timestamps.",
            "This artifact does not estimate billing cost or research impact.",
        ],
    }
    (args.out_dir / "summary.json").write_text(json.dumps(metadata, indent=2) + "\n", encoding="utf-8")
    if args.plot:
        plot_overview(groups, args.out_dir / "token-usage-overview.png", args.plot_bucket)
    print(f"wrote {len(events)} usage events to {args.out_dir}")


if __name__ == "__main__":
    main()
