#!/usr/bin/env python3
"""Summarize performance phase-event JSONL files."""

import argparse
import csv
import json
import sys
from collections import defaultdict
from pathlib import Path

def main() -> int:
    parser = argparse.ArgumentParser(
        description="Summarize experiments/performance phase-events.jsonl files."
    )
    parser.add_argument(
        "input",
        type=Path,
        help="Path to phase-events.jsonl or to an output directory containing it.",
    )
    parser.add_argument(
        "--csv",
        dest="csv_path",
        type=Path,
        help="Optional CSV output path. The stdout summary is still printed.",
    )
    args = parser.parse_args()

    phase_events_path = resolve_phase_events_path(args.input)
    rows = read_jsonl(phase_events_path)
    summary = summarize(rows)
    print_stdout_summary(summary)
    if args.csv_path is not None:
        write_csv(args.csv_path, summary)
    return 0


def resolve_phase_events_path(path: Path) -> Path:
    if path.is_dir():
        return path / "phase-events.jsonl"
    return path


def read_jsonl(path: Path) -> list[dict]:
    rows = []
    with path.open() as handle:
        for line_number, line in enumerate(handle, start=1):
            line = line.strip()
            if not line:
                continue
            try:
                rows.append(json.loads(line))
            except json.JSONDecodeError as error:
                raise SystemExit(f"{path}:{line_number}: invalid JSON: {error}") from error
    return rows


def summarize(rows: list[dict]) -> list[dict]:
    groups: dict[tuple[str, int, str], list[dict]] = defaultdict(list)
    sample_totals: dict[tuple[str, int, int], float] = defaultdict(float)
    sample_has_error: dict[tuple[str, int, int], bool] = defaultdict(bool)

    for row in rows:
        key = (row["target"], int(row["facet_count"]), row["phase"])
        groups[key].append(row)
        sample_key = (row["target"], int(row["facet_count"]), int(row["sample"]))
        if row.get("status") == "ok":
            sample_totals[sample_key] += float(row["elapsed_ms"])
        else:
            sample_has_error[sample_key] = True

    result = []
    for (target, facet_count, phase), group_rows in groups.items():
        ok_group_rows = [row for row in group_rows if row.get("status") == "ok"]
        error_group_rows = [row for row in group_rows if row.get("status") != "ok"]
        total_ms = sum(float(row["elapsed_ms"]) for row in ok_group_rows)
        error_total_ms = sum(float(row["elapsed_ms"]) for row in error_group_rows)
        sample_count = len({int(row["sample"]) for row in group_rows})
        denominator = sum(
            sample_totals[(target, facet_count, sample)]
            for sample in {int(row["sample"]) for row in ok_group_rows}
        )
        completed_total_ms = sum(
            float(row["elapsed_ms"])
            for row in ok_group_rows
            if not sample_has_error[(target, facet_count, int(row["sample"]))]
        )
        completed_denominator = sum(
            sample_totals[(target, facet_count, sample)]
            for sample in {int(row["sample"]) for row in ok_group_rows}
            if not sample_has_error[(target, facet_count, sample)]
        )
        ok_rows = len(ok_group_rows)
        result.append(
            {
                "target": target,
                "facet_count": facet_count,
                "phase": phase,
                "events": len(group_rows),
                "ok_events": ok_rows,
                "error_events": len(group_rows) - ok_rows,
                "samples": sample_count,
                "total_ms": total_ms,
                "ok_mean_ms": total_ms / ok_rows if ok_rows > 0 else None,
                "error_mean_ms": error_total_ms / len(error_group_rows)
                if error_group_rows
                else None,
                "first_error": first_present(error_group_rows, "error"),
                "pct_of_sample_total": (100.0 * total_ms / denominator)
                if denominator > 0.0
                else 0.0,
                "pct_of_completed_sample_total": (
                    100.0 * completed_total_ms / completed_denominator
                )
                if completed_denominator > 0.0
                else None,
                "iterations_mean": mean_present(ok_group_rows, "iterations"),
                "raw_orbits_mean": mean_present(ok_group_rows, "raw_orbits"),
                "returned_orbits_mean": mean_present(ok_group_rows, "returned_orbits"),
                "allowed_transitions_mean": mean_present(
                    ok_group_rows, "allowed_transitions"
                ),
                "allowed_edges_mean": mean_present(ok_group_rows, "allowed_edges"),
                "cycles_mean": mean_present(ok_group_rows, "cycles"),
            }
        )
    return result


def mean_present(rows: list[dict], key: str) -> float | None:
    values = [float(row[key]) for row in rows if row.get(key) is not None]
    if not values:
        return None
    return sum(values) / len(values)


def first_present(rows: list[dict], key: str) -> str | None:
    for row in rows:
        value = row.get(key)
        if value is not None:
            return str(value)
    return None


def print_stdout_summary(summary: list[dict]) -> None:
    columns = [
        ("target", "target"),
        ("facet_count", "F"),
        ("phase", "phase"),
        ("events", "events"),
        ("ok_events", "ok"),
        ("error_events", "errors"),
        ("samples", "samples"),
        ("ok_mean_ms", "ok_mean_ms"),
        ("error_mean_ms", "error_mean_ms"),
        ("pct_of_sample_total", "pct_total"),
        ("pct_of_completed_sample_total", "pct_complete"),
        ("iterations_mean", "iter_mean"),
        ("raw_orbits_mean", "raw_orbits"),
        ("returned_orbits_mean", "ret_orbits"),
        ("allowed_transitions_mean", "allowed_transitions"),
        ("allowed_edges_mean", "allowed_edges"),
        ("cycles_mean", "cycles"),
        ("first_error", "first_error"),
    ]
    print("\t".join(label for _, label in columns))
    for row in summary:
        print("\t".join(format_value(row.get(key)) for key, _ in columns))


def format_value(value) -> str:
    if value is None:
        return ""
    if isinstance(value, float):
        return f"{value:.6g}"
    return str(value)


def write_csv(path: Path, summary: list[dict]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    fieldnames = [
        "target",
        "facet_count",
        "phase",
        "events",
        "ok_events",
        "error_events",
        "samples",
        "total_ms",
        "ok_mean_ms",
        "error_mean_ms",
        "pct_of_sample_total",
        "pct_of_completed_sample_total",
        "iterations_mean",
        "raw_orbits_mean",
        "returned_orbits_mean",
        "allowed_transitions_mean",
        "allowed_edges_mean",
        "cycles_mean",
        "first_error",
    ]
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(summary)


if __name__ == "__main__":
    sys.exit(main())
