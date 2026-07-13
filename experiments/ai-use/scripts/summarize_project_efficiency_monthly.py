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
import subprocess
from collections import Counter
from datetime import date
from pathlib import Path
from typing import Any


def run_git(*args: str) -> str:
    return subprocess.run(
        ["git", *args], check=True, text=True, capture_output=True
    ).stdout.strip()


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as handle:
        return list(csv.DictReader(handle))


def write_csv(path: Path, rows: list[dict[str, Any]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    if not rows:
        path.write_text("\n", encoding="utf-8")
        return
    with path.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.DictWriter(handle, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)


def json_text(value: Any) -> str:
    return json.dumps(value, sort_keys=True, ensure_ascii=False)


def snapshot_commit(target: str, git_ref: str) -> str | None:
    return run_git("rev-list", "-1", f"--before={target} 23:59:59 UTC", git_ref) or None


def required_surface_states(commit: str) -> dict[str, int]:
    try:
        text = run_git("show", f"{commit}:PROJECT_COMPLETION.md")
    except subprocess.CalledProcessError:
        return {"completion_ledger_unavailable": 1}
    states: Counter[str] = Counter()
    in_table = False
    for line in text.splitlines():
        if line == "## Required Surface":
            in_table = True
            continue
        if in_table and line.startswith("## "):
            break
        if not in_table or not line.startswith("|") or "---" in line:
            continue
        fields = [field.strip() for field in line.strip("|").split("|")]
        if len(fields) >= 4 and fields[0] != "Surface":
            states[fields[1]] += 1
    return dict(sorted(states.items()))


def diff_summary(previous: str | None, current: str) -> dict[str, Any]:
    if not previous:
        return {
            "commits_since_previous_snapshot": "",
            "changed_files_since_previous_snapshot": "",
            "thesis_files_changed": "",
            "experiment_files_changed": "",
            "harness_files_changed": "",
            "other_files_changed": "",
            "commit_subjects": "",
        }
    commits = run_git("log", "--format=%H%x09%s", f"{previous}..{current}").splitlines()
    files = run_git("diff", "--name-only", f"{previous}..{current}").splitlines()
    counts = Counter(
        "thesis" if path.startswith("thesis/") else
        "experiment" if path.startswith("experiments/") else
        "harness" if path.startswith(".agents/") or path.startswith(".codex/") else
        "other"
        for path in files
    )
    return {
        "commits_since_previous_snapshot": len(commits),
        "changed_files_since_previous_snapshot": len(files),
        "thesis_files_changed": counts["thesis"],
        "experiment_files_changed": counts["experiment"],
        "harness_files_changed": counts["harness"],
        "other_files_changed": counts["other"],
        "commit_subjects": " || ".join(line.split("\t", 1)[1] for line in commits),
    }


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
