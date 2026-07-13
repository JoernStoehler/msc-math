#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.12"
# ///

"""Aggregate rollout-token rows by parent session lineage.

This producer joins the token packet's ``rollout-daily.csv`` rows with each
rollout's ``session_meta`` record.  It preserves the resource accounting used
by the project-efficiency case studies without copying transcript text into a
durable artifact.  Raw rollout JSONL remains the source evidence for lineage.
"""

from __future__ import annotations

import argparse
import csv
import json
import statistics
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any


PRICES = {
    "gpt-5.5": (5.0, 0.5, 30.0),
    "gpt-5.6-sol": (5.0, 0.5, 30.0),
    "gpt-5.6-terra": (2.5, 0.25, 15.0),
    "gpt-5.6-luna": (1.0, 0.1, 6.0),
    "gpt-5.4-mini": (0.75, 0.075, 4.5),
}
TOKEN_KEYS = (
    "total_tokens",
    "input_tokens",
    "cached_input_tokens",
    "output_tokens",
    "reasoning_output_tokens",
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--rollout-csv", type=Path, required=True)
    parser.add_argument("--start", required=True, help="Inclusive UTC date")
    parser.add_argument("--end", required=True, help="Inclusive UTC date")
    parser.add_argument("--out-dir", type=Path, required=True)
    parser.add_argument(
        "--min-root-tokens",
        type=int,
        default=1_000_000,
        help="Minimum tokens for model-cohort summary (default: 1M)",
    )
    parser.add_argument(
        "--root-id",
        action="append",
        default=[],
        help="Root to copy into selected-roots.csv; repeat for several roots",
    )
    args = parser.parse_args()
    if args.start > args.end:
        parser.error("--start must not be later than --end")
    return args


def json_objects(path: Path):
    try:
        with path.open(encoding="utf-8") as handle:
            for line in handle:
                try:
                    yield json.loads(line)
                except json.JSONDecodeError:
                    continue
    except OSError:
        return


def session_root(path: Path) -> tuple[str, dict[str, Any]] | None:
    for event in json_objects(path):
        if event.get("type") != "session_meta":
            continue
        payload = event.get("payload")
        if not isinstance(payload, dict):
            return None
        session_id = payload.get("session_id") or payload.get("id")
        if not session_id:
            return None
        return str(payload.get("parent_thread_id") or session_id), payload
    return None


def empty_stats() -> dict[str, Any]:
    return {
        **{key: 0 for key in TOKEN_KEYS},
        "baseline_cost_usd": 0.0,
        "paths": set(),
        "dates": set(),
        "models": Counter(),
        "efforts": Counter(),
    }


def add_row(stats: dict[str, Any], row: dict[str, str]) -> None:
    for key in TOKEN_KEYS:
        stats[key] += int(row.get(key, 0) or 0)
    model = row.get("model", "unknown")
    prices = PRICES.get(model)
    if prices:
        stats["baseline_cost_usd"] += (
            int(row.get("uncached_input_tokens", 0) or 0) * prices[0]
            + int(row.get("cached_input_tokens", 0) or 0) * prices[1]
        ) / 1_000_000
        stats["baseline_cost_usd"] += (
            int(row.get("output_tokens", 0) or 0) * prices[2]
        ) / 1_000_000
    stats["models"][model] += int(row.get("total_tokens", 0) or 0)
    stats["efforts"][row.get("effort", "unknown")] += int(
        row.get("total_tokens", 0) or 0
    )
    stats["paths"].add(row["path"])
    stats["dates"].add(row["date"])


def compact_mix(values: Counter[str]) -> str:
    return json.dumps(dict(values.most_common()), separators=(",", ":"))


def row_for_root(root_id: str, stats: dict[str, Any]) -> dict[str, Any]:
    dominant_model = (
        stats["models"].most_common(1)[0][0]
        if stats["models"]
        else "unknown"
    )
    return {
        "root_id": root_id,
        "paths": len(stats["paths"]),
        "start_date": min(stats["dates"]) if stats["dates"] else "",
        "end_date": max(stats["dates"]) if stats["dates"] else "",
        **{key: stats[key] for key in TOKEN_KEYS},
        "baseline_cost_usd": round(stats["baseline_cost_usd"], 6),
        "dominant_model": dominant_model,
        "model_mix_tokens": compact_mix(stats["models"]),
        "effort_mix_tokens": compact_mix(stats["efforts"]),
        "cache_hit_rate": (
            stats["cached_input_tokens"] / stats["input_tokens"]
            if stats["input_tokens"]
            else None
        ),
    }


def write_csv(path: Path, rows: list[dict[str, Any]]) -> None:
    if not rows:
        path.write_text("\n", encoding="utf-8")
        return
    with path.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.DictWriter(handle, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)


def main() -> None:
    args = parse_args()
    path_roots: dict[str, str] = {}
    paths_seen: set[str] = set()
    paths_missing_metadata: set[str] = set()
    root_stats: dict[str, dict[str, Any]] = defaultdict(empty_stats)
    with args.rollout_csv.open(newline="", encoding="utf-8") as handle:
        for row in csv.DictReader(handle):
            if not (args.start <= row["date"] <= args.end):
                continue
            path = row["path"]
            paths_seen.add(path)
            if path not in path_roots and path not in paths_missing_metadata:
                metadata = session_root(Path(path))
                if metadata is None:
                    paths_missing_metadata.add(path)
                    continue
                path_roots[path] = metadata[0]
            root_id = path_roots[path]
            add_row(root_stats[root_id], row)

    summary_rows = [
        row_for_root(root_id, stats)
        for root_id, stats in root_stats.items()
    ]
    summary_rows.sort(key=lambda row: (-row["total_tokens"], row["root_id"]))
    cohort_groups: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for row in summary_rows:
        if row["total_tokens"] >= args.min_root_tokens:
            cohort_groups[row["dominant_model"]].append(row)
    cohort_rows = []
    for model, rows in sorted(
        cohort_groups.items(), key=lambda item: -sum(r["total_tokens"] for r in item[1])
    ):
        paths = [row["paths"] for row in rows]
        tokens = [row["total_tokens"] for row in rows]
        cohort_rows.append(
            {
                "dominant_model": model,
                "min_root_tokens": args.min_root_tokens,
                "root_count": len(rows),
                "total_tokens": sum(tokens),
                "median_root_tokens": statistics.median(tokens),
                "mean_paths": statistics.mean(paths),
                "median_paths": statistics.median(paths),
                "median_tokens_per_path": statistics.median(
                    token / path for token, path in zip(tokens, paths)
                ),
            }
        )

    selected = [row for row in summary_rows if row["root_id"] in set(args.root_id)]
    args.out_dir.mkdir(parents=True, exist_ok=True)
    write_csv(args.out_dir / "root-summary.csv", summary_rows)
    write_csv(args.out_dir / "model-cohort-summary.csv", cohort_rows)
    write_csv(args.out_dir / "selected-roots.csv", selected)
    (args.out_dir / "summary.json").write_text(
        json.dumps(
            {
                "start": args.start,
                "end": args.end,
                "min_root_tokens": args.min_root_tokens,
                "roots": len(summary_rows),
                "rollout_paths_seen": len(paths_seen),
                "paths_with_session_meta": len(path_roots),
                "paths_missing_session_meta": len(paths_missing_metadata),
                "interpretation_boundary": (
                    "Resource aggregation by session lineage; baseline shadow cost "
                    "does not apply long-context multipliers and does not measure value."
                ),
            },
            indent=2,
        )
        + "\n",
        encoding="utf-8",
    )
    print(f"wrote {len(summary_rows)} root rows to {args.out_dir}")


if __name__ == "__main__":
    main()
