#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.12"
# ///

"""Aggregate monthly Codex resources and month-end Git snapshots.

This producer intentionally leaves semantic value assessment blank. A separate
durable interpretation ledger records only source-backed value judgments.
"""

from __future__ import annotations

import argparse
import calendar
import csv
import json
import sys
from collections import Counter
from datetime import date, timedelta
from pathlib import Path
from typing import Any

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR))
from analyze_project_efficiency import (  # noqa: E402
    diff_summary,
    json_text,
    read_csv,
    required_surface_states,
    run_git,
    snapshot_commit,
    write_csv,
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--start", required=True)
    parser.add_argument("--end", required=True)
    parser.add_argument("--token-dir", type=Path, required=True)
    parser.add_argument("--git-ref", default="main")
    parser.add_argument("--out-dir", type=Path, required=True)
    args = parser.parse_args()
    args.start_date = date.fromisoformat(args.start)
    args.end_date = date.fromisoformat(args.end)
    if args.start_date > args.end_date:
        parser.error("--start must not be later than --end")
    return args


def month_range(start: date, end: date) -> list[str]:
    months = []
    current = date(start.year, start.month, 1)
    while current <= end:
        months.append(current.strftime("%Y-%m"))
        current = date(current.year + (current.month == 12), 1 if current.month == 12 else current.month + 1, 1)
    return months


def month_bounds(month: str, start: date, end: date) -> tuple[str, str, str]:
    year, number = map(int, month.split("-"))
    first = max(date(year, number, 1), start)
    last = min(date(year, number, calendar.monthrange(year, number)[1]), end)
    return month, first.isoformat(), last.isoformat()


def aggregate(rows: list[dict[str, str]], month: str, key: str) -> int:
    return sum(int(row[key]) for row in rows if row["date"].startswith(month))


def main() -> None:
    args = parse_args()
    daily = read_csv(args.token_dir / "daily.csv")
    shadow = read_csv(args.token_dir / "shadow-cost.csv")
    models = read_csv(args.token_dir / "model-effort.csv")
    lineage = read_csv(args.token_dir / "lineage.csv")
    rows: list[dict[str, Any]] = []
    previous_commit: str | None = None
    for month in month_range(args.start_date, args.end_date):
        _, first, last = month_bounds(month, args.start_date, args.end_date)
        daily_rows = [row for row in daily if first <= row["date"] <= last]
        cost_rows = [row for row in shadow if first <= row["date"] <= last]
        model_counts = Counter()
        for row in models:
            if first <= row["date"] <= last:
                model_counts[row["model"]] += int(row["total_tokens"])
        lineage_counts = Counter()
        for row in lineage:
            if first <= row["date"] <= last:
                lineage_counts[row["source"]] += int(row["total_tokens"])

        snapshot = snapshot_commit(last, args.git_ref)
        row: dict[str, Any] = {
            "month": month,
            "resource_start": first,
            "resource_end": last,
            "active_days": len(daily_rows),
            "total_tokens": sum(int(row["total_tokens"]) for row in daily_rows),
            "input_tokens": sum(int(row["input_tokens"]) for row in daily_rows),
            "cached_input_tokens": sum(int(row["cached_input_tokens"]) for row in daily_rows),
            "uncached_input_tokens": sum(int(row["uncached_input_tokens"]) for row in daily_rows),
            "output_tokens": sum(int(row["output_tokens"]) for row in daily_rows),
            "usage_events": sum(int(row["events"]) for row in daily_rows),
            "rollouts": sum(int(row["rollouts"]) for row in daily_rows),
            "cache_hit_rate": (
                sum(int(row["cached_input_tokens"]) for row in daily_rows)
                / sum(int(row["input_tokens"]) for row in daily_rows)
                if daily_rows else ""
            ),
            "baseline_cost_usd": sum(float(row["baseline_cost_usd"]) for row in cost_rows),
            "long_context_cost_usd": sum(float(row["long_context_cost_usd"]) for row in cost_rows),
            "long_context_requests": sum(int(row["long_context_requests"]) for row in cost_rows),
            "model_mix": json_text(dict(model_counts.most_common())),
            "lineage_mix": json_text(dict(lineage_counts.most_common())),
            "snapshot_date": last,
            "git_commit": snapshot or "",
            "git_committed_at": "",
            "git_subject": "",
            "required_surface_states": json_text(
                required_surface_states(snapshot) if snapshot else {"git_snapshot_unavailable": 1}
            ),
            "value_assessment": "",
        }
        if snapshot:
            commit_info = run_git(
                "show", "-s", "--format=%H%x09%ad%x09%s", "--date=iso", snapshot
            )
            _, row["git_committed_at"], row["git_subject"] = commit_info.split("\t", 2)
            row.update(diff_summary(previous_commit, snapshot))
            previous_commit = snapshot
        else:
            row.update(diff_summary(None, month))
        rows.append(row)

    args.out_dir.mkdir(parents=True, exist_ok=True)
    write_csv(args.out_dir / "monthly.csv", rows)
    metadata = {
        "start": args.start,
        "end": args.end,
        "git_ref": args.git_ref,
        "token_dir": str(args.token_dir),
        "interpretation_boundary": (
            "Monthly resource totals are generated. Value assessments are kept "
            "separately so semantic judgments remain reviewable."
        ),
    }
    (args.out_dir / "summary.json").write_text(
        json.dumps({"metadata": metadata, "months": rows}, indent=2) + "\n",
        encoding="utf-8",
    )
    print(f"wrote {len(rows)} monthly rows to {args.out_dir}")


if __name__ == "__main__":
    main()
