#!/usr/bin/env python3
"""Adapt retained finite-probe producer schemas to the G1 event contract."""

from __future__ import annotations

import argparse
import importlib.util
import json
from collections import defaultdict
from pathlib import Path
from typing import Any

HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("g1_analyze", HERE / "analyze.py")
assert SPEC and SPEC.loader
ANALYZE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(ANALYZE)


def probe_id(family: str, direction: int, sign: int, radius: float) -> str:
    return f"{family}:d{direction}:s{sign}:r{radius:.17g}"


def write_rows(path: Path, rows: list[dict[str, Any]]) -> None:
    if path.exists():
        raise ValueError(f"output already exists: {path}")
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8") as handle:
        for row in rows:
            handle.write(json.dumps(row, sort_keys=True) + "\n")


def adapt_control_calibration(path: Path) -> list[dict[str, Any]]:
    source_rows = list(ANALYZE.read_jsonl(path))
    grouped: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for row in source_rows:
        grouped[row["case_id"]].append(row)
    output: list[dict[str, Any]] = []
    for state_id, rows in sorted(grouped.items()):
        role = rows[0]["case_role"]
        expected = (
            "finite_suite_survivor"
            if role == "proved_fixed_f_positive_control"
            else "explicit_improvement_found"
        )
        ids = [
            probe_id(
                row["family"],
                int(row["direction_index"]),
                int(row["sign"]),
                float(row["nominal_radius"]),
            )
            for row in rows
        ]
        output.append(
            {
                "schema_version": 1,
                "row_type": "state",
                "state_id": state_id,
                "source": rows[0]["source"],
                "control_role": role,
                "selection_kind": "predeclared_control",
                "outcome_selected": False,
                "state_valid": True,
                "suite_id": "control-calibration-v1",
                "expected_probe_ids": ids,
                "material_delta_sys": 0.0,
                "positive_fixed_f_evidence": (
                    {
                        "status": "proved",
                        "source": "experiments/hko-local-maximum/theorem/",
                    }
                    if state_id == "hko_reference"
                    else None
                ),
                "facet_addition_evidence": None,
                "expected_finite_classification": expected,
            }
        )
        for source, identifier in zip(rows, ids, strict=True):
            output.append(
                {
                    "schema_version": 1,
                    "row_type": "probe",
                    "state_id": state_id,
                    "probe_id": identifier,
                    "suite_id": "control-calibration-v1",
                    "direction_family": source["family"],
                    "direction_index": source["direction_index"],
                    "sign": source["sign"],
                    "radius": source["nominal_radius"],
                    "step_norm": source["step_norm"],
                    "base_sys": source["base_sys"],
                    "perturbed_sys": source["perturbed_sys"],
                    "delta_sys": source["delta_sys"],
                    "state_valid": source["state_valid"],
                    "same_incidence": source["same_incidence_signature"],
                    "uncertainty": (
                        "interval_separated"
                        if source["interval_separated_improvement"]
                        else "nominal_f64"
                    ),
                    "failure": source["failure"],
                }
            )
    return output


def adapt_quotient(states_path: Path, probes_path: Path) -> list[dict[str, Any]]:
    source_states = list(ANALYZE.read_jsonl(states_path))
    source_probes = list(ANALYZE.read_jsonl(probes_path))
    grouped: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for row in source_probes:
        grouped[row["state_id"]].append(row)
    output: list[dict[str, Any]] = []
    for state in source_states:
        state_id = state["state_id"]
        rows = grouped.get(state_id, [])
        ids = [
            probe_id(
                "signed_quotient_basis",
                int(row["basis_index"]),
                int(row["sign"]),
                float(row["relative_radius"]),
            )
            for row in rows
        ]
        expected = None
        if state["control_role"] == "positive_control_exact_theorem_local_maximum":
            expected = "finite_suite_survivor"
        elif state["control_role"] == "negative_control_known_later_literal_improvement":
            expected = "explicit_improvement_found"
        output.append(
            {
                "schema_version": 1,
                "row_type": "state",
                "state_id": state_id,
                "source": state["source_path"],
                "control_role": state["control_role"],
                "selection_kind": state["selection_rule"],
                "outcome_selected": "best" in state["selection_rule"],
                "state_valid": True,
                "suite_id": "signed-quotient-poll-v1",
                "expected_probe_ids": ids,
                "material_delta_sys": 0.0,
                "positive_fixed_f_evidence": (
                    {
                        "status": "proved",
                        "source": "experiments/hko-local-maximum/theorem/",
                    }
                    if state_id == "positive_control_hko2024"
                    else None
                ),
                "facet_addition_evidence": None,
                "expected_finite_classification": expected,
                "stopping_reason": None,
                "measured_compute_ms": None,
                "trusted_call_count": None,
                "optimizer_provenance": {
                    "source_iteration": state["source_iteration"],
                    "source_eta": state["source_eta"],
                    "recorded_sys": state["recorded_sys"],
                    "recomputed_sys": state["recomputed_sys"],
                },
            }
        )
        for source, identifier in zip(rows, ids, strict=True):
            output.append(
                {
                    "schema_version": 1,
                    "row_type": "probe",
                    "state_id": state_id,
                    "probe_id": identifier,
                    "suite_id": "signed-quotient-poll-v1",
                    "direction_family": "signed_quotient_basis",
                    "direction_index": source["basis_index"],
                    "sign": source["sign"],
                    "radius": source["relative_radius"],
                    "step_norm": source["step_norm"],
                    "base_sys": source["base_sys"],
                    "perturbed_sys": source["perturbed_sys"],
                    "delta_sys": source["delta_sys"],
                    "state_valid": source["state_valid"],
                    "same_incidence": source["same_incidence_signature"],
                    "uncertainty": "nominal_f64",
                    "failure": source["failure"],
                }
            )
    return output


def main() -> None:
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="kind", required=True)
    control = subparsers.add_parser("control-calibration")
    control.add_argument("--rows", type=Path, required=True)
    control.add_argument("--out", type=Path, required=True)
    quotient = subparsers.add_parser("quotient-endpoint")
    quotient.add_argument("--states", type=Path, required=True)
    quotient.add_argument("--probes", type=Path, required=True)
    quotient.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    if args.kind == "control-calibration":
        rows = adapt_control_calibration(args.rows)
    else:
        rows = adapt_quotient(args.states, args.probes)
    write_rows(args.out, rows)


if __name__ == "__main__":
    main()

