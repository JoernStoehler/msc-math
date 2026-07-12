#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.12"
# ///

"""Join Codex resource observations with integrated Git snapshots.

This is deliberately a pilot producer. It measures resource proxies and the
state of the existing completion ledger; it does not invent a 0--100 value
score. Semantic value assessment belongs in the accompanying report and must
cite the snapshot and downstream gate it refers to.
"""

from __future__ import annotations

import argparse
import csv
import json
import re
import subprocess
from collections import Counter, defaultdict
from datetime import date, datetime, timedelta, timezone
from pathlib import Path
from typing import Any, Iterable


DEFAULT_ROOTS = (
    "/home/vscode/.codex/sessions",
    "/home/vscode/.codex/archived_sessions",
    "/home/vscode/.codex/imported_session_logs",
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--date", dest="dates", action="append", required=True)
    parser.add_argument("--token-dir", type=Path, required=True)
    parser.add_argument("--git-ref", default="main")
    parser.add_argument("--root", type=Path, action="append", default=None)
    parser.add_argument("--out-dir", type=Path, required=True)
    args = parser.parse_args()
    args.dates = sorted(set(args.dates))
    for value in args.dates:
        try:
            date.fromisoformat(value)
        except ValueError as exc:
            parser.error(f"invalid --date {value!r}: {exc}")
    return args


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


def aggregate_tokens(token_dir: Path, target_dates: set[str]) -> dict[str, dict[str, Any]]:
    daily = {row["date"]: row for row in read_csv(token_dir / "daily.csv")}
    model_rows = read_csv(token_dir / "model-effort.csv")
    cost_rows = read_csv(token_dir / "shadow-cost.csv")
    result: dict[str, dict[str, Any]] = {}
    for target in sorted(target_dates):
        daily_row = daily.get(target, {})
        models = Counter(
            {row["model"]: 0 for row in model_rows if row["date"] == target}
        )
        for row in model_rows:
            if row["date"] == target:
                models[row["model"]] += int(row["total_tokens"])
        costs = [row for row in cost_rows if row["date"] == target]
        result[target] = {
            "total_tokens": int(daily_row.get("total_tokens", 0)),
            "input_tokens": int(daily_row.get("input_tokens", 0)),
            "cached_input_tokens": int(daily_row.get("cached_input_tokens", 0)),
            "uncached_input_tokens": int(daily_row.get("uncached_input_tokens", 0)),
            "output_tokens": int(daily_row.get("output_tokens", 0)),
            "cache_hit_rate": float(daily_row.get("cache_hit_rate", "nan")),
            "rollouts": int(daily_row.get("rollouts", 0)),
            "usage_events": int(daily_row.get("events", 0)),
            "models": dict(models.most_common()),
            "baseline_cost_usd": sum(
                float(row["baseline_cost_usd"]) for row in costs
            ),
            "long_context_cost_usd": sum(
                float(row["long_context_cost_usd"]) for row in costs
            ),
            "long_context_requests": sum(
                int(row["long_context_requests"]) for row in costs
            ),
        }
    return result


def rollout_paths(token_dir: Path, target_dates: set[str]) -> dict[str, set[Path]]:
    paths: dict[str, set[Path]] = defaultdict(set)
    for row in read_csv(token_dir / "rollout-daily.csv"):
        if row["date"] in target_dates:
            paths[row["date"]].add(Path(row["path"]))
    return paths


def session_observations(
    token_dir: Path, target_dates: set[str]
) -> dict[str, dict[str, Any]]:
    observations: dict[str, dict[str, Any]] = {}
    for target, paths in rollout_paths(token_dir, target_dates).items():
        stats: Counter[str] = Counter()
        first: str | None = None
        last: str | None = None
        existing_rollouts = 0
        for path in sorted(paths):
            if not path.exists():
                continue
            existing_rollouts += 1
            with path.open(encoding="utf-8") as handle:
                for line in handle:
                    try:
                        event = json.loads(line)
                    except json.JSONDecodeError:
                        continue
                    timestamp = event.get("timestamp", "")
                    if not timestamp.startswith(target):
                        continue
                    first = min(first, timestamp) if first else timestamp
                    last = max(last, timestamp) if last else timestamp
                    payload = event.get("payload") or {}
                    event_type = payload.get("type", "unknown")
                    stats[event_type] += 1
                    raw = f"{payload.get('name', '')} {payload.get('arguments', '')}"
                    if re.search(r"licca|sbatch|squeue|scancel|slurm|ssh ", raw, re.I):
                        stats["licca_like_events"] += 1
        span_hours = None
        if first and last:
            start = datetime.fromisoformat(first.replace("Z", "+00:00"))
            end = datetime.fromisoformat(last.replace("Z", "+00:00"))
            span_hours = (end - start).total_seconds() / 3600
        observations[target] = {
            "rollout_files_in_csv": len(paths),
            "rollout_files_read": existing_rollouts,
            "first_event_utc": first or "",
            "last_event_utc": last or "",
            "observed_log_span_hours": span_hours,
            "user_message_events": stats["user_message"],
            "agent_message_events": stats["agent_message"],
            "tool_call_events": stats["function_call"] + stats["custom_tool_call"],
            "context_compactions": stats["context_compacted"],
            "licca_like_events": stats["licca_like_events"],
        }
    return observations


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


def analyze(args: argparse.Namespace) -> list[dict[str, Any]]:
    dates = args.dates
    target_dates = set(dates)
    token_data = aggregate_tokens(args.token_dir, target_dates)
    session_data = session_observations(args.token_dir, target_dates)
    rows = []
    previous_commit: str | None = None
    for target in dates:
        commit = snapshot_commit(target, args.git_ref)
        row: dict[str, Any] = {
            "snapshot_date": target,
            "git_commit": commit or "",
            "git_committed_at": "",
            "git_subject": "",
            "required_surface_states": json_text(
                required_surface_states(commit) if commit else {"git_snapshot_unavailable": 1}
            ),
        }
        if commit:
            commit_info = run_git(
                "show", "-s", "--format=%H%x09%ad%x09%s", "--date=iso", commit
            )
            _, row["git_committed_at"], row["git_subject"] = commit_info.split("\t", 2)
        row.update(diff_summary(previous_commit, commit) if commit else diff_summary(None, target))
        row.update(token_data.get(target, {}))
        row.update(session_data.get(target, {}))
        row["models"] = json_text(row.get("models", {}))
        rows.append(row)
        if commit:
            previous_commit = commit
    return rows


def main() -> None:
    args = parse_args()
    args.out_dir.mkdir(parents=True, exist_ok=True)
    rows = analyze(args)
    write_csv(args.out_dir / "snapshots.csv", rows)
    metadata = {
        "dates": args.dates,
        "git_ref": args.git_ref,
        "token_dir": str(args.token_dir),
        "roots": [str(root) for root in (args.root or DEFAULT_ROOTS)],
        "interpretation_boundary": (
            "Resource metrics are observed proxies. The producer does not assign "
            "a scalar value or infer Jörn hours from message timestamps."
        ),
    }
    (args.out_dir / "summary.json").write_text(
        json.dumps({"metadata": metadata, "snapshots": rows}, indent=2) + "\n",
        encoding="utf-8",
    )
    print(f"wrote {len(rows)} snapshot rows to {args.out_dir}")


if __name__ == "__main__":
    main()
