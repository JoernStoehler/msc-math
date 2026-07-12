#!/usr/bin/env python3
"""Convert f64 capacity scan rows into numerics observation events."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


EXPERIMENT_DIR = Path(__file__).resolve().parent


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Emit numerics events from f64 capacity scan JSONL rows."
    )
    parser.add_argument(
        "--manifest",
        type=Path,
        default=EXPERIMENT_DIR.parent / "verification" / "manifest.json",
        help="Verification manifest used to order and check source ids.",
    )
    parser.add_argument(
        "--scan",
        action="append",
        default=[],
        type=Path,
        help="Scan JSONL path. Repeatable.",
    )
    parser.add_argument(
        "--out-dir",
        type=Path,
        default=Path("/tmp/f64-capacity-numerics"),
        help="Directory for events.jsonl.",
    )
    args = parser.parse_args()

    manifest = json.loads(args.manifest.read_text(encoding="utf-8"))
    scan_rows = load_scan_rows(args.scan)
    events = build_events(manifest["cases"], scan_rows)
    write_jsonl(args.out_dir / "events.jsonl", events)
    return 1 if has_missing_scan_rows(events) else 0


def load_scan_rows(paths: list[Path]) -> dict[str, dict[str, Any]]:
    rows: dict[str, dict[str, Any]] = {}
    for path in paths:
        with path.open(encoding="utf-8") as handle:
            for line_number, line in enumerate(handle, start=1):
                stripped = line.strip()
                if not stripped:
                    continue
                row = json.loads(stripped)
                source_id = row["source_id"]
                if source_id in rows:
                    raise ValueError(f"duplicate source_id {source_id} in {path}:{line_number}")
                rows[source_id] = row
    return rows


def build_events(cases: list[dict[str, Any]], scan_rows: dict[str, dict[str, Any]]) -> list[dict[str, Any]]:
    missing_count = 0
    events: list[dict[str, Any]] = [
        {
            "event": "run_started",
            "target": "f64-capacity-scan-to-numerics",
            "mode": "smoke",
            "schema_version": 1,
        }
    ]
    for case in cases:
        row = scan_rows.get(case["source_id"])
        if row is None:
            missing_count += 1
            events.append(
                {
                    "event": "context_finished",
                    "mode": "smoke",
                    "context_id": case["case_id"],
                    "status": "missing_scan_row",
                }
            )
            continue
        events.extend(events_for_case(case, row))
    events.append(
        {
            "event": "run_finished",
            "mode": "smoke",
            "contexts": len(cases),
            "missing_contexts": missing_count,
            "status": "incomplete" if missing_count else "ok",
        }
    )
    return events


def has_missing_scan_rows(events: list[dict[str, Any]]) -> bool:
    return any(row.get("status") == "missing_scan_row" for row in events)


def events_for_case(case: dict[str, Any], row: dict[str, Any]) -> list[dict[str, Any]]:
    context = context_fields(case, row)
    events: list[dict[str, Any]] = [
        {
            "event": "context_started",
            **context,
        }
    ]
    events.append(capacity_observation(context, row))
    for variable in DIAGNOSTIC_VARIABLES:
        events.append(diagnostic_observation(context, row, variable))
    events.append(
        {
            "event": "context_finished",
            "mode": "smoke",
            "context_id": case["case_id"],
            "status": "ok",
        }
    )
    return events


def context_fields(case: dict[str, Any], row: dict[str, Any]) -> dict[str, Any]:
    return {
        "mode": "smoke",
        "context_id": case["case_id"],
        "context_kind": "f64_capacity_scan_row",
        "object_id": row["source_id"],
        "object_family": row["family"],
        "input_pair_kind": input_pair_kind(row),
        "exact_geometry_validation_status": exact_geometry_validation_status(row),
        "sigma": row.get("f64_sigma") or [],
        "sample_policy": "verification_manifest",
    }


def input_pair_kind(row: dict[str, Any]) -> str:
    if (
        row.get("exact_audit_status") == "reference_route_capacity_success"
        and row.get("audit_capacity_label") is not None
    ):
        return "f64_input_with_fresh_reference_route_label"
    if row.get("audit_capacity_label") is not None:
        return "f64_input_with_stored_capacity_label"
    return "f64_input_without_capacity_label"


def exact_geometry_validation_status(row: dict[str, Any]) -> str:
    status = row.get("exact_audit_status")
    if status == "not_requested":
        return "not_requested"
    if status in {
        "reference_route_capacity_success",
        "reference_route_capacity_failure",
    }:
        return "accepted"
    if status == "exact_validation_rejected":
        return "rejected"
    return "unknown"


def capacity_observation(context: dict[str, Any], row: dict[str, Any]) -> dict[str, Any]:
    f64_value = row.get("f64_capacity")
    comparison_label = row.get("audit_capacity_label")
    label_kind = capacity_comparison_label_kind(row)
    event = observation_base(context, "capacity", f64_value)
    event.update(
        {
            "algorithm": "f64_capacity",
            "stage": "scan_output",
            "status": "ok" if f64_value is not None else "missing_f64_capacity",
        }
    )
    if comparison_label is not None and label_kind is not None:
        event["comparison_label_kind"] = label_kind
        event["comparison_label_f64"] = comparison_label
        event.update(label_difference_fields(f64_value, comparison_label))
    return event


def capacity_comparison_label_kind(row: dict[str, Any]) -> str | None:
    if (
        row.get("exact_audit_status") == "reference_route_capacity_success"
        and row.get("audit_capacity_label") is not None
    ):
        return "fresh_reference_route_capacity_label"
    if row.get("audit_capacity_label") is not None:
        return "stored_artifact_label"
    return None


def diagnostic_observation(context: dict[str, Any], row: dict[str, Any], variable: str) -> dict[str, Any]:
    event = observation_base(context, variable, row.get(variable))
    event.update(
        {
            "algorithm": "f64_capacity",
            "stage": "scan_diagnostic",
            "status": "ok" if row.get(variable) is not None else "missing",
        }
    )
    return event


def observation_base(context: dict[str, Any], variable: str, value: Any) -> dict[str, Any]:
    event = {
        "event": "observation",
        **context,
        "algorithm": "f64_capacity",
        "stage": "scan_output",
        "variable": variable,
        "sample_policy": context["sample_policy"],
        "status": "ok",
    }
    if value is not None:
        event["f64"] = float(value)
    return event


def label_difference_fields(f64_value: Any, label_value: Any) -> dict[str, float]:
    if f64_value is None:
        return {}
    f64_number = float(f64_value)
    label_number = float(label_value)
    abs_difference = abs(f64_number - label_number)
    fields = {"comparison_label_abs_difference": abs_difference}
    if label_number != 0.0:
        fields["comparison_label_rel_difference"] = abs_difference / abs(label_number)
    return fields


def write_jsonl(path: Path, rows: list[dict[str, Any]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8") as handle:
        for row in rows:
            handle.write(json.dumps(row, sort_keys=True) + "\n")


DIAGNOSTIC_VARIABLES = (
    "near_minimizing_sigma_count",
    "min_action_gap",
    "sigma_count",
    "admissible_f64_count",
    "indeterminate_f64_count",
    "inadmissible_count",
    "numerical_failure_count",
    "vertex_indeterminate_count",
    "near_singular_vertex_count",
    "bounded_near_singular_vertex_count",
    "ambiguous_vertex_incidence_count",
    "facet_intersection_indeterminate_count",
    "omega_indeterminate_count",
    "origin_lp_max_min_lambda",
    "origin_lp_max_abs_residual",
    "product_rounding_max_minor_over_major",
    "product_rounding_max_abs_change",
    "removed_facet_count",
    "near_redundant_facet_removal_delta_bound",
    "capacity_ratio_upper_bound",
    "volume_ratio_upper_bound",
    "sys_ratio_lower_bound",
    "sys_ratio_upper_bound",
)


if __name__ == "__main__":
    raise SystemExit(main())
