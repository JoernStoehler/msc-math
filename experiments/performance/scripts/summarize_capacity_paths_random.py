#!/usr/bin/env python3
"""Summarize capacity-path random benchmark JSONL output."""

import argparse
import csv
import json
import statistics
from collections import defaultdict
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Summarize capacity-path random benchmark path-events.jsonl."
    )
    parser.add_argument(
        "input",
        type=Path,
        help="Path to path-events.jsonl or to an output directory containing it.",
    )
    parser.add_argument("--csv", type=Path, help="Optional CSV output path.")
    args = parser.parse_args()

    rows = read_jsonl(resolve_input(args.input))
    summary = summarize(rows)
    print_summary(summary)
    if args.csv is not None:
        write_csv(args.csv, summary)
    return 0


def resolve_input(path: Path) -> Path:
    if path.is_dir():
        return path / "path-events.jsonl"
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
    groups = defaultdict(list)
    for row in rows:
        groups[(row["facet_count"], row["path"])].append(row)

    summary = []
    for (facet_count, path), group_rows in sorted(groups.items()):
        ok_rows = [row for row in group_rows if row.get("status") == "ok"]
        errors = [row for row in group_rows if row.get("status") != "ok"]
        times = [float(row["elapsed_ms"]) for row in ok_rows]
        sigma_counts = [
            row.get("sigma_count", row.get("iterations"))
            for row in ok_rows
            if row.get("sigma_count", row.get("iterations")) is not None
        ]
        admissible_counts = [
            row["admissible_f64_count"]
            for row in ok_rows
            if row.get("admissible_f64_count") is not None
        ]
        raw_orbits = [
            row["raw_orbits"] for row in ok_rows if row.get("raw_orbits") is not None
        ]
        capacity_diffs = [
            row["capacity_abs_diff_from_fallback"]
            for row in ok_rows
            if row.get("capacity_abs_diff_from_fallback") is not None
        ]
        summary.append(
            {
                "facet_count": facet_count,
                "path": path,
                "events": len(group_rows),
                "ok_events": len(ok_rows),
                "error_events": len(errors),
                "median_ms": median(times),
                "mean_ms": mean(times),
                "min_ms": min(times) if times else None,
                "max_ms": max(times) if times else None,
                "median_sigma_or_iteration_count": median(sigma_counts),
                "median_admissible_f64_count": median(admissible_counts),
                "median_raw_orbits": median(raw_orbits),
                "max_capacity_abs_diff_from_fallback": max(capacity_diffs)
                if capacity_diffs
                else None,
                "first_error": errors[0].get("error") if errors else None,
            }
        )
    return summary


def median(values: list[float]) -> float | None:
    return statistics.median(values) if values else None


def mean(values: list[float]) -> float | None:
    return statistics.mean(values) if values else None


def print_summary(summary: list[dict]) -> None:
    for row in summary:
        print(
            "F={facet_count} path={path} ok={ok_events}/{events} "
            "median_ms={median_ms} mean_ms={mean_ms} "
            "median_count={median_sigma_or_iteration_count} "
            "median_admissible_f64={median_admissible_f64_count} "
            "median_raw_orbits={median_raw_orbits} "
            "max_abs_diff={max_capacity_abs_diff_from_fallback} "
            "first_error={first_error}".format(**format_row(row))
        )


def format_row(row: dict) -> dict:
    formatted = row.copy()
    for key, value in formatted.items():
        if isinstance(value, float):
            formatted[key] = f"{value:.6g}"
        elif value is None:
            formatted[key] = ""
    return formatted


def write_csv(path: Path, summary: list[dict]) -> None:
    if not summary:
        path.write_text("")
        return
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=list(summary[0].keys()))
        writer.writeheader()
        writer.writerows(summary)


if __name__ == "__main__":
    raise SystemExit(main())
