#!/usr/bin/env python3
"""Summarize HK2017 tracing summary events from stderr logs."""

import argparse
import csv
import re
import sys
from collections import defaultdict
from pathlib import Path

SUMMARY_MARKERS = [
    "hk2017_candidate_solve_summary",
    "hk2017_directed_cycle_summary",
    "hk2017_unpruned_enumeration_summary",
]
FIELD_RE = re.compile(r"([A-Za-z_][A-Za-z0-9_]*)=([^\s]+)")

NON_NUMERIC_TRACE_FIELDS = {"target", "sample", "level", "message", "fields.message"}


def main() -> int:
    parser = argparse.ArgumentParser(description="Summarize HK2017 trace summary events.")
    parser.add_argument("input", type=Path, help="Path to a trace stderr log.")
    parser.add_argument(
        "--csv",
        dest="csv_path",
        type=Path,
        help="Optional CSV output path. The stdout summary is still printed.",
    )
    args = parser.parse_args()

    rows = read_summary_events(args.input)
    summary = summarize(rows)
    warn_on_mismatched_event_counts(summary)
    print_stdout_summary(summary)
    if args.csv_path is not None:
        write_csv(args.csv_path, summary)
    return 0


def read_summary_events(path: Path) -> list[dict]:
    rows = []
    with path.open() as handle:
        for line in handle:
            event = next((marker for marker in SUMMARY_MARKERS if marker in line), None)
            if event is None:
                continue
            fields = dict(FIELD_RE.findall(line))
            if "facet_count" not in fields:
                raise SystemExit(f"{path}: summary event without facet_count")
            row = {
                "target": clean_trace_value(fields.get("target", "")),
                "event": event,
                "facet_count": int(fields["facet_count"]),
            }
            for field, value in fields.items():
                if field in NON_NUMERIC_TRACE_FIELDS or field == "facet_count":
                    continue
                try:
                    row[field] = float(clean_trace_value(value))
                except ValueError:
                    pass
            rows.append(row)
    if not rows:
        raise SystemExit(f"{path}: no HK2017 summary events found")
    return rows


def clean_trace_value(value: str) -> str:
    return value.strip('"')


def summarize(rows: list[dict]) -> list[dict]:
    groups: dict[tuple[str, str, int], list[dict]] = defaultdict(list)
    for row in rows:
        groups[(row["target"], row["event"], row["facet_count"])].append(row)

    result = []
    for (target, event, facet_count), group in sorted(groups.items()):
        item = {
            "target": target,
            "event": event,
            "facet_count": facet_count,
            "samples": len(group),
        }
        for field in numeric_fields(group):
            item[f"{field}_mean"] = mean_present(group, field)
        kkt_ms = item.get("kkt_ms_mean")
        payload_ms = item.get("payload_ms_mean")
        search_ms = item.get("search_ms_mean")
        unattributed_ms = item.get("unattributed_search_ms_mean")
        if search_ms is not None and search_ms > 0.0:
            item["unattributed_pct_search"] = 100.0 * (unattributed_ms or 0.0) / search_ms
            item["kkt_pct_search"] = 100.0 * (kkt_ms or 0.0) / search_ms
            item["payload_pct_search"] = 100.0 * (payload_ms or 0.0) / search_ms
        else:
            item["unattributed_pct_search"] = None
            item["kkt_pct_search"] = None
            item["payload_pct_search"] = None
        result.append(item)
    return result


def numeric_fields(rows: list[dict]) -> list[str]:
    excluded = {"target", "event", "facet_count"}
    fields = set()
    for row in rows:
        fields.update(key for key in row if key not in excluded)
    return sorted(fields)


def warn_on_mismatched_event_counts(summary: list[dict]) -> None:
    samples_by_target_facet: dict[tuple[str, int], dict[str, int]] = defaultdict(dict)
    for row in summary:
        samples_by_target_facet[(row["target"], row["facet_count"])][row["event"]] = row["samples"]

    for (target, facet_count), counts in sorted(samples_by_target_facet.items()):
        if len(counts) <= 1:
            continue
        sample_counts = set(counts.values())
        if len(sample_counts) > 1:
            details = ", ".join(f"{event}={count}" for event, count in sorted(counts.items()))
            print(
                f"warning: target={target} F={facet_count} has mismatched summary-event counts: {details}",
                file=sys.stderr,
            )


def mean_present(rows: list[dict], key: str) -> float | None:
    values = [float(row[key]) for row in rows if key in row]
    if not values:
        return None
    return sum(values) / len(values)


def print_stdout_summary(summary: list[dict]) -> None:
    common_columns = [
        ("target", "target"),
        ("facet_count", "F"),
        ("event", "event"),
        ("samples", "samples"),
    ]
    metric_columns = [(key, key) for key in summary_metric_keys(summary)]
    columns = common_columns + metric_columns
    print("\t".join(label for _, label in columns))
    for row in summary:
        print("\t".join(format_value(row.get(key)) for key, _ in columns))


def summary_metric_keys(summary: list[dict]) -> list[str]:
    excluded = {"target", "facet_count", "event", "samples"}
    keys = set()
    for row in summary:
        keys.update(key for key in row if key not in excluded)
    return sorted(keys)


def format_value(value) -> str:
    if value is None:
        return ""
    if isinstance(value, float):
        return f"{value:.6g}"
    return str(value)


def write_csv(path: Path, summary: list[dict]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    fieldnames = ["target", "event", "facet_count", "samples"] + summary_metric_keys(summary)
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(summary)


if __name__ == "__main__":
    sys.exit(main())
