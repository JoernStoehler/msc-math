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
    "hk2017_enumeration_summary",
]
FIELD_RE = re.compile(r"([A-Za-z_][A-Za-z0-9_]*)=([^\s]+)")

NUMERIC_FIELDS = [
    "iterations",
    "raw_orbits",
    "search_ms",
    "traversal_ms",
    "emit_outside_callback_ms",
    "candidate_callback_ms",
    "callback_overhead_ms",
    "kkt_ms",
    "payload_ms",
    "sigma_len_mean",
    "sigma_len_min",
    "sigma_len_max",
    "kkt_feasible",
    "kkt_infeasible",
    "kkt_singular_matrix",
    "kkt_type_c_violation",
    "kkt_constraint_violation",
    "admissible_f64",
    "indeterminate_f64",
    "payload_inadmissible",
    "numerical_failures",
    "subset_count",
    "cyclic_permutation_count",
    "dfs_prefix_count",
    "edge_rejections",
    "emitted_sigmas",
]

EVENT_COLUMNS = {
    "hk2017_candidate_solve_summary": [
        ("iterations_mean", "iter_mean"),
        ("raw_orbits_mean", "raw_orbits"),
        ("search_ms_mean", "search_ms"),
        ("emit_outside_callback_ms_mean", "emit_outside_cb_ms"),
        ("candidate_callback_ms_mean", "candidate_cb_ms"),
        ("callback_overhead_ms_mean", "cb_overhead_ms"),
        ("kkt_ms_mean", "kkt_ms"),
        ("payload_ms_mean", "payload_ms"),
        ("emit_outside_callback_pct_search", "emit_outside_cb_pct"),
        ("candidate_callback_pct_search", "candidate_cb_pct"),
        ("callback_overhead_pct_search", "cb_overhead_pct"),
        ("kkt_pct_search", "kkt_pct"),
        ("payload_pct_search", "payload_pct"),
        ("sigma_len_mean_mean", "sigma_len_mean"),
        ("sigma_len_min_mean", "sigma_len_min"),
        ("sigma_len_max_mean", "sigma_len_max"),
        ("kkt_feasible_mean", "kkt_feasible"),
        ("kkt_infeasible_mean", "kkt_infeasible"),
        ("admissible_f64_mean", "admissible_f64"),
        ("indeterminate_f64_mean", "indeterminate_f64"),
        ("payload_inadmissible_mean", "payload_inadm"),
        ("numerical_failures_mean", "num_fail"),
    ],
    "hk2017_enumeration_summary": [
        ("subset_count_mean", "subsets"),
        ("cyclic_permutation_count_mean", "cyclic_perms"),
        ("dfs_prefix_count_mean", "dfs_prefixes"),
        ("edge_rejections_mean", "edge_rej"),
        ("emitted_sigmas_mean", "emitted"),
    ],
}


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
            for field in NUMERIC_FIELDS:
                if field in fields:
                    row[field] = float(fields[field])
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
        for field in NUMERIC_FIELDS:
            item[f"{field}_mean"] = mean_present(group, field)
        kkt_ms = item.get("kkt_ms_mean")
        payload_ms = item.get("payload_ms_mean")
        search_ms = item.get("search_ms_mean")
        emit_outside_callback_ms = item.get("emit_outside_callback_ms_mean")
        if emit_outside_callback_ms is None:
            emit_outside_callback_ms = item.get("traversal_ms_mean")
        candidate_callback_ms = item.get("candidate_callback_ms_mean")
        callback_overhead_ms = item.get("callback_overhead_ms_mean")
        if search_ms is not None and search_ms > 0.0:
            item["emit_outside_callback_pct_search"] = (
                100.0 * (emit_outside_callback_ms or 0.0) / search_ms
            )
            item["candidate_callback_pct_search"] = (
                100.0 * (candidate_callback_ms or 0.0) / search_ms
                if candidate_callback_ms is not None
                else None
            )
            item["callback_overhead_pct_search"] = (
                100.0 * (callback_overhead_ms or 0.0) / search_ms
                if callback_overhead_ms is not None
                else None
            )
            item["kkt_pct_search"] = 100.0 * (kkt_ms or 0.0) / search_ms
            item["payload_pct_search"] = 100.0 * (payload_ms or 0.0) / search_ms
        else:
            item["emit_outside_callback_pct_search"] = None
            item["candidate_callback_pct_search"] = None
            item["callback_overhead_pct_search"] = None
            item["kkt_pct_search"] = None
            item["payload_pct_search"] = None
        result.append(item)
    return result


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
    rows_by_event: dict[str, list[dict]] = defaultdict(list)
    for row in summary:
        rows_by_event[row["event"]].append(row)

    first_section = True
    for event in SUMMARY_MARKERS:
        rows = rows_by_event.get(event, [])
        if not rows:
            continue
        if not first_section:
            print()
        first_section = False
        columns = common_columns + EVENT_COLUMNS[event]
        print("\t".join(label for _, label in columns))
        for row in rows:
            print("\t".join(format_value(row.get(key)) for key, _ in columns))


def format_value(value) -> str:
    if value is None:
        return ""
    if isinstance(value, float):
        return f"{value:.6g}"
    return str(value)


def write_csv(path: Path, summary: list[dict]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    fieldnames = ["event", "facet_count", "samples"]
    for field in NUMERIC_FIELDS:
        fieldnames.append(f"{field}_mean")
    fieldnames.extend(
        [
            "emit_outside_callback_pct_search",
            "candidate_callback_pct_search",
            "callback_overhead_pct_search",
            "kkt_pct_search",
            "payload_pct_search",
        ]
    )
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(summary)


if __name__ == "__main__":
    sys.exit(main())
