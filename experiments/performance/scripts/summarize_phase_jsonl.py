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
    groups: dict[tuple[str, str, str, int, str], list[dict]] = defaultdict(list)
    sample_totals: dict[tuple[str, str, str, int, int], float] = defaultdict(float)
    sample_has_error: dict[tuple[str, str, str, int, int], bool] = defaultdict(bool)

    for row in rows:
        family = str(row.get("family", ""))
        method = str(row.get("method", ""))
        key = (row["target"], family, method, int(row["facet_count"]), row["phase"])
        groups[key].append(row)
        sample_key = (row["target"], family, method, int(row["facet_count"]), int(row["sample"]))
        if row.get("status") == "ok":
            sample_totals[sample_key] += float(row["elapsed_ms"])
        else:
            sample_has_error[sample_key] = True

    result = []
    for (target, family, method, facet_count, phase), group_rows in groups.items():
        ok_group_rows = [row for row in group_rows if row.get("status") == "ok"]
        error_group_rows = [row for row in group_rows if row.get("status") != "ok"]
        capacity_rows = [row for row in ok_group_rows if capacity_ran(row)]
        capacity_not_run_rows = [row for row in ok_group_rows if capacity_not_run(row)]
        total_ms = sum(float(row["elapsed_ms"]) for row in ok_group_rows)
        error_total_ms = sum(float(row["elapsed_ms"]) for row in error_group_rows)
        sample_count = len({int(row["sample"]) for row in group_rows})
        denominator = sum(
            sample_totals[(target, family, method, facet_count, sample)]
            for sample in {int(row["sample"]) for row in ok_group_rows}
        )
        completed_total_ms = sum(
            float(row["elapsed_ms"])
            for row in ok_group_rows
            if not sample_has_error[(target, family, method, facet_count, int(row["sample"]))]
        )
        completed_denominator = sum(
            sample_totals[(target, family, method, facet_count, sample)]
            for sample in {int(row["sample"]) for row in ok_group_rows}
            if not sample_has_error[(target, family, method, facet_count, sample)]
        )
        ok_rows = len(ok_group_rows)
        result.append(
            {
                "target": target,
                "family": family,
                "method": method,
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
                "validation_status_counts": value_counts(ok_group_rows, "validation_status"),
                "outcome_counts": value_counts(ok_group_rows, "outcome"),
                "trust_class_counts": value_counts(ok_group_rows, "trust_class"),
                "agreement_status_counts": value_counts(ok_group_rows, "agreement_status"),
                "capacity_run_events": len(capacity_rows),
                "capacity_not_run_events": len(capacity_not_run_rows),
                "sigma_count_mean": mean_present_any(
                    ok_group_rows, ["sigma_count", "iterations"]
                ),
                "capacity_sigma_count_mean": mean_present_any(
                    capacity_rows, ["sigma_count", "iterations"]
                ),
                "raw_orbits_mean": mean_present(ok_group_rows, "raw_orbits"),
                "returned_orbits_mean": mean_present(ok_group_rows, "returned_orbits"),
                "allowed_transitions_mean": mean_present(
                    ok_group_rows, "allowed_transitions"
                ),
                "allowed_edges_mean": mean_present(ok_group_rows, "allowed_edges"),
                "cycles_mean": mean_present(ok_group_rows, "cycles"),
                "validation_bundle_time_ms_mean": mean_present_any(
                    ok_group_rows, ["validation_bundle_time_ms", "validation_time_ms"]
                ),
                "capacity_bundle_time_ms_mean": mean_present_any(
                    capacity_rows, ["capacity_bundle_time_ms", "f64_time_ms"]
                ),
                "validation_sanity_ms_mean": mean_present(ok_group_rows, "validation_sanity_ms"),
                "validation_origin_lp_diagnostic_ms_mean": mean_present(
                    ok_group_rows, "validation_origin_lp_diagnostic_ms"
                ),
                "validation_origin_policy_predicate_ms_mean": mean_present(
                    ok_group_rows, "validation_origin_policy_predicate_ms"
                ),
                "validation_combinatorics_ms_mean": mean_present(
                    ok_group_rows, "validation_combinatorics_ms"
                ),
                "validation_classification_ms_mean": mean_present(
                    ok_group_rows, "validation_classification_ms"
                ),
                "validation_geometry_vertex_scan_ms_mean": mean_present(
                    ok_group_rows, "validation_geometry_vertex_scan_ms"
                ),
                "validation_geometry_facet_intersections_ms_mean": mean_present(
                    ok_group_rows, "validation_geometry_facet_intersections_ms"
                ),
                "validation_geometry_omega_signs_ms_mean": mean_present(
                    ok_group_rows, "validation_geometry_omega_signs_ms"
                ),
                "validation_lp_facet_statuses_ms_mean": mean_present(
                    ok_group_rows, "validation_lp_facet_statuses_ms"
                ),
                "validation_lp_facet_intersections_ms_mean": mean_present(
                    ok_group_rows, "validation_lp_facet_intersections_ms"
                ),
                "validation_lp_omega_recompute_ms_mean": mean_present(
                    ok_group_rows, "validation_lp_omega_recompute_ms"
                ),
                "capacity_combinatorics_ms_mean": mean_present(
                    capacity_rows, "capacity_combinatorics_ms"
                ),
                "capacity_transition_matrix_ms_mean": mean_present(
                    capacity_rows, "capacity_transition_matrix_ms"
                ),
                "capacity_candidate_solve_ms_mean": mean_present(
                    capacity_rows, "capacity_candidate_solve_ms"
                ),
                "capacity_candidate_kkt_solve_ms_mean": mean_present(
                    capacity_rows, "capacity_candidate_kkt_solve_ms"
                ),
                "capacity_candidate_non_kkt_ms_mean": mean_present(
                    capacity_rows, "capacity_candidate_non_kkt_ms"
                ),
                "capacity_report_ms_mean": mean_present(capacity_rows, "capacity_report_ms"),
                "capacity_geometry_vertex_scan_ms_mean": mean_present(
                    capacity_rows, "capacity_geometry_vertex_scan_ms"
                ),
                "capacity_geometry_facet_intersections_ms_mean": mean_present(
                    capacity_rows, "capacity_geometry_facet_intersections_ms"
                ),
                "capacity_geometry_omega_signs_ms_mean": mean_present(
                    capacity_rows, "capacity_geometry_omega_signs_ms"
                ),
                "capacity_lp_facet_statuses_ms_mean": mean_present(
                    capacity_rows, "capacity_lp_facet_statuses_ms"
                ),
                "capacity_lp_facet_intersections_ms_mean": mean_present(
                    capacity_rows, "capacity_lp_facet_intersections_ms"
                ),
                "capacity_lp_omega_recompute_ms_mean": mean_present(
                    capacity_rows, "capacity_lp_omega_recompute_ms"
                ),
                "admissible_f64_mean": mean_present(capacity_rows, "admissible_f64_count"),
                "indeterminate_f64_mean": mean_present(capacity_rows, "indeterminate_f64_count"),
                "inadmissible_mean": mean_present(capacity_rows, "inadmissible_count"),
                "numerical_failure_mean": mean_present(capacity_rows, "numerical_failure_count"),
                "facet_intersection_true_mean": mean_present(capacity_rows, "facet_intersection_true_count"),
                "facet_intersection_false_mean": mean_present(capacity_rows, "facet_intersection_false_count"),
                "facet_intersection_indeterminate_mean": mean_present(capacity_rows, "facet_intersection_indeterminate_count"),
                "omega_indeterminate_mean": mean_present(capacity_rows, "omega_indeterminate_count"),
                "vertex_indeterminate_mean": mean_present(capacity_rows, "vertex_indeterminate_count"),
            }
        )
    return result


def capacity_ran(row: dict) -> bool:
    if row.get("capacity_ran") is not None:
        return bool(row["capacity_ran"])
    outcome = row.get("outcome")
    return outcome is not None and outcome != "not_run"


def capacity_not_run(row: dict) -> bool:
    if row.get("capacity_ran") is not None:
        return not bool(row["capacity_ran"])
    return row.get("outcome") == "not_run"


def mean_present(rows: list[dict], key: str) -> float | None:
    values = [float(row[key]) for row in rows if row.get(key) is not None]
    if not values:
        return None
    return sum(values) / len(values)


def mean_present_any(rows: list[dict], keys: list[str]) -> float | None:
    values = []
    for row in rows:
        for key in keys:
            if row.get(key) is not None:
                values.append(float(row[key]))
                break
    if not values:
        return None
    return sum(values) / len(values)


def value_counts(rows: list[dict], key: str) -> str | None:
    counts: dict[str, int] = defaultdict(int)
    for row in rows:
        value = row.get(key)
        if value is not None:
            counts[str(value)] += 1
    if not counts:
        return None
    return "|".join(f"{value}:{counts[value]}" for value in sorted(counts))


def first_present(rows: list[dict], key: str) -> str | None:
    for row in rows:
        value = row.get(key)
        if value is not None:
            return str(value)
    return None


def print_stdout_summary(summary: list[dict]) -> None:
    columns = [
        ("target", "target"),
        ("family", "family"),
        ("method", "method"),
        ("facet_count", "F"),
        ("phase", "phase"),
        ("events", "events"),
        ("ok_events", "ok"),
        ("error_events", "errors"),
        ("samples", "samples"),
        ("ok_mean_ms", "e2e_mean_ms"),
        ("error_mean_ms", "error_mean_ms"),
        ("pct_of_sample_total", "pct_total"),
        ("pct_of_completed_sample_total", "pct_complete"),
        ("validation_status_counts", "validation_statuses"),
        ("outcome_counts", "outcomes"),
        ("trust_class_counts", "trust_classes"),
        ("agreement_status_counts", "agreement_statuses"),
        ("capacity_run_events", "capacity_run"),
        ("capacity_not_run_events", "capacity_not_run"),
        ("sigma_count_mean", "sigma_count_mean"),
        ("capacity_sigma_count_mean", "sigma_count_if_capacity"),
        ("raw_orbits_mean", "raw_orbits"),
        ("returned_orbits_mean", "ret_orbits"),
        ("allowed_transitions_mean", "allowed_transitions"),
        ("allowed_edges_mean", "allowed_edges"),
        ("cycles_mean", "cycles"),
        ("validation_bundle_time_ms_mean", "validation_bundle_ms"),
        ("capacity_bundle_time_ms_mean", "capacity_bundle_ms"),
        ("validation_sanity_ms_mean", "val_sanity_ms"),
        ("validation_origin_lp_diagnostic_ms_mean", "val_origin_lp_diag_ms"),
        ("validation_origin_policy_predicate_ms_mean", "val_origin_policy_ms"),
        ("validation_combinatorics_ms_mean", "val_combinatorics_ms"),
        ("validation_geometry_vertex_scan_ms_mean", "val_vertex_scan_ms"),
        ("validation_lp_facet_statuses_ms_mean", "val_lp_facet_statuses_ms"),
        ("validation_lp_facet_intersections_ms_mean", "val_lp_facet_pairs_ms"),
        ("capacity_combinatorics_ms_mean", "cap_combinatorics_ms"),
        ("capacity_transition_matrix_ms_mean", "cap_transition_matrix_ms"),
        ("capacity_candidate_solve_ms_mean", "cap_kkt_candidates_ms"),
        ("capacity_candidate_kkt_solve_ms_mean", "cap_kkt_solve_ms"),
        ("capacity_candidate_non_kkt_ms_mean", "cap_enum_filter_ms"),
        ("capacity_geometry_vertex_scan_ms_mean", "cap_vertex_scan_ms"),
        ("capacity_lp_facet_statuses_ms_mean", "cap_lp_facet_statuses_ms"),
        ("capacity_lp_facet_intersections_ms_mean", "cap_lp_facet_pairs_ms"),
        ("admissible_f64_mean", "admissible_if_capacity"),
        ("indeterminate_f64_mean", "indet_kkt_if_capacity"),
        ("inadmissible_mean", "inadmissible_if_capacity"),
        ("numerical_failure_mean", "num_fail_if_capacity"),
        ("facet_intersection_true_mean", "facet_pair_true_if_capacity"),
        ("facet_intersection_false_mean", "facet_pair_false_if_capacity"),
        ("facet_intersection_indeterminate_mean", "facet_pair_indet_if_capacity"),
        ("omega_indeterminate_mean", "omega_indet"),
        ("vertex_indeterminate_mean", "vertex_indet"),
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
        "family",
        "method",
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
        "validation_status_counts",
        "outcome_counts",
        "trust_class_counts",
        "agreement_status_counts",
        "capacity_run_events",
        "capacity_not_run_events",
        "sigma_count_mean",
        "capacity_sigma_count_mean",
        "raw_orbits_mean",
        "returned_orbits_mean",
        "allowed_transitions_mean",
        "allowed_edges_mean",
        "cycles_mean",
        "validation_bundle_time_ms_mean",
        "capacity_bundle_time_ms_mean",
        "validation_sanity_ms_mean",
        "validation_origin_lp_diagnostic_ms_mean",
        "validation_origin_policy_predicate_ms_mean",
        "validation_combinatorics_ms_mean",
        "validation_classification_ms_mean",
        "validation_geometry_vertex_scan_ms_mean",
        "validation_geometry_facet_intersections_ms_mean",
        "validation_geometry_omega_signs_ms_mean",
        "validation_lp_facet_statuses_ms_mean",
        "validation_lp_facet_intersections_ms_mean",
        "validation_lp_omega_recompute_ms_mean",
        "capacity_combinatorics_ms_mean",
        "capacity_transition_matrix_ms_mean",
        "capacity_candidate_solve_ms_mean",
        "capacity_candidate_kkt_solve_ms_mean",
        "capacity_candidate_non_kkt_ms_mean",
        "capacity_report_ms_mean",
        "capacity_geometry_vertex_scan_ms_mean",
        "capacity_geometry_facet_intersections_ms_mean",
        "capacity_geometry_omega_signs_ms_mean",
        "capacity_lp_facet_statuses_ms_mean",
        "capacity_lp_facet_intersections_ms_mean",
        "capacity_lp_omega_recompute_ms_mean",
        "admissible_f64_mean",
        "indeterminate_f64_mean",
        "inadmissible_mean",
        "numerical_failure_mean",
        "facet_intersection_true_mean",
        "facet_intersection_false_mean",
        "facet_intersection_indeterminate_mean",
        "omega_indeterminate_mean",
        "vertex_indeterminate_mean",
        "first_error",
    ]
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(summary)


if __name__ == "__main__":
    sys.exit(main())
