#!/usr/bin/env python3
"""Summarize f64-decision-compare JSONL files."""

import argparse
import csv
import json
from collections import defaultdict
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Summarize experiments/performance decision-events.jsonl files."
    )
    parser.add_argument(
        "input",
        type=Path,
        help="Path to decision-events.jsonl or to an output directory containing it.",
    )
    parser.add_argument(
        "--csv",
        dest="csv_path",
        type=Path,
        help="Optional CSV output path. The stdout summary is still printed.",
    )
    args = parser.parse_args()

    path = resolve_decision_events_path(args.input)
    rows = read_jsonl(path)
    summary = summarize(rows)
    print_stdout_summary(summary)
    if args.csv_path is not None:
        write_csv(args.csv_path, summary)
    return 0


def resolve_decision_events_path(path: Path) -> Path:
    if path.is_dir():
        return path / "decision-events.jsonl"
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
        key = (
            row["target"],
            row["mode"],
            row["decision"],
            row["left_method"],
            str(row.get("right_method") or ""),
        )
        groups[key].append(row)

    summary = []
    for (target, mode, decision, left_method, right_method), group_rows in groups.items():
        summary.append(
            {
                "target": target,
                "mode": mode,
                "decision": decision,
                "left_method": left_method,
                "right_method": right_method,
                "rows": len(group_rows),
                "families": joined_counts(group_rows, "family"),
                "left_time_ms_mean": mean(group_rows, "left_time_ms"),
                "right_time_ms_mean": mean_present(group_rows, "right_time_ms"),
                "left_time_ms_total": total(group_rows, "left_time_ms"),
                "right_time_ms_total": total_present(group_rows, "right_time_ms"),
                "left_true_total": sum_int(group_rows, "left_true_count"),
                "left_false_total": sum_int(group_rows, "left_false_count"),
                "left_indeterminate_total": sum_int(
                    group_rows, "left_indeterminate_count"
                ),
                "left_error_total": sum_int(group_rows, "left_error_count"),
                "right_true_total": sum_int_present(group_rows, "right_true_count"),
                "right_false_total": sum_int_present(group_rows, "right_false_count"),
                "right_indeterminate_total": sum_int_present(
                    group_rows, "right_indeterminate_count"
                ),
                "right_error_total": sum_int_present(group_rows, "right_error_count"),
                "agreement_total": sum_int_present(group_rows, "agreement_count"),
                "disagreement_total": sum_int_present(group_rows, "disagreement_count"),
                "left_indeterminate_right_decisive_total": sum_int_present(
                    group_rows, "left_indeterminate_right_decisive_count"
                ),
                "left_decisive_right_indeterminate_total": sum_int_present(
                    group_rows, "left_decisive_right_indeterminate_count"
                ),
            }
        )
    return sorted(
        summary,
        key=lambda row: (
            row["target"],
            row["mode"],
            row["decision"],
            row["left_method"],
            row["right_method"],
        ),
    )


def mean(rows: list[dict], key: str) -> float | None:
    values = [float(row[key]) for row in rows]
    return sum(values) / len(values) if values else None


def mean_present(rows: list[dict], key: str) -> float | None:
    values = [float(row[key]) for row in rows if row.get(key) is not None]
    return sum(values) / len(values) if values else None


def total(rows: list[dict], key: str) -> float:
    return sum(float(row[key]) for row in rows)


def total_present(rows: list[dict], key: str) -> float | None:
    values = [float(row[key]) for row in rows if row.get(key) is not None]
    return sum(values) if values else None


def sum_int(rows: list[dict], key: str) -> int:
    return sum(int(row[key]) for row in rows)


def sum_int_present(rows: list[dict], key: str) -> int | None:
    values = [int(row[key]) for row in rows if row.get(key) is not None]
    return sum(values) if values else None


def joined_counts(rows: list[dict], key: str) -> str:
    counts = defaultdict(int)
    for row in rows:
        counts[str(row.get(key, ""))] += 1
    return "|".join(f"{value}:{counts[value]}" for value in sorted(counts))


def print_stdout_summary(summary: list[dict]) -> None:
    columns = [
        ("decision", "decision"),
        ("left", "left_method"),
        ("right", "right_method"),
        ("rows", "rows"),
        ("left_ms", "left_time_ms_mean"),
        ("right_ms", "right_time_ms_mean"),
        ("left_t/f/i/e", "left_counts"),
        ("right_t/f/i/e", "right_counts"),
        ("disagree", "disagreement_total"),
    ]
    rendered_rows = []
    for row in summary:
        rendered = dict(row)
        rendered["left_counts"] = counts_cell(
            row["left_true_total"],
            row["left_false_total"],
            row["left_indeterminate_total"],
            row["left_error_total"],
        )
        rendered["right_counts"] = counts_cell(
            row["right_true_total"],
            row["right_false_total"],
            row["right_indeterminate_total"],
            row["right_error_total"],
        )
        rendered_rows.append(rendered)
    widths = {
        label: max(len(label), *(len(format_value(row[key])) for row in rendered_rows))
        for label, key in columns
    }
    print(" ".join(label.ljust(widths[label]) for label, _ in columns))
    print(" ".join("-" * widths[label] for label, _ in columns))
    for row in rendered_rows:
        print(
            " ".join(
                format_value(row[key]).ljust(widths[label]) for label, key in columns
            )
        )


def counts_cell(true_count, false_count, indeterminate_count, error_count) -> str:
    if true_count is None:
        return ""
    return f"{true_count}/{false_count}/{indeterminate_count}/{error_count}"


def format_value(value) -> str:
    if value is None:
        return ""
    if isinstance(value, float):
        return f"{value:.6g}"
    return str(value)


def write_csv(path: Path, summary: list[dict]) -> None:
    fieldnames = [
        "target",
        "mode",
        "decision",
        "left_method",
        "right_method",
        "rows",
        "families",
        "left_time_ms_mean",
        "right_time_ms_mean",
        "left_time_ms_total",
        "right_time_ms_total",
        "left_true_total",
        "left_false_total",
        "left_indeterminate_total",
        "left_error_total",
        "right_true_total",
        "right_false_total",
        "right_indeterminate_total",
        "right_error_total",
        "agreement_total",
        "disagreement_total",
        "left_indeterminate_right_decisive_total",
        "left_decisive_right_indeterminate_total",
    ]
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(summary)


if __name__ == "__main__":
    raise SystemExit(main())
