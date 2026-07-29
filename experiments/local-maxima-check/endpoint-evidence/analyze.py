#!/usr/bin/env python3
"""Validate and normalize finite endpoint evidence.

This consumer deliberately reports evidence categories rather than a Boolean
local-maximality decision.
"""

from __future__ import annotations

import argparse
import json
import math
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any, Iterable

SCHEMA_VERSION = 1
ROW_TYPES = {"state", "probe", "continuation"}
FINITE_CLASSES = {
    "explicit_improvement_found",
    "invalid_or_indeterminate",
    "finite_suite_survivor",
    "no_finite_suite",
}
CLAIM_BOUNDARY = (
    "One valid recomputed improving move rejects the recorded stopping state. "
    "A finite miss means only survival of the named directions and radii; it "
    "does not establish local maximality or control adding facets."
)


def read_jsonl(path: Path) -> Iterable[dict[str, Any]]:
    with path.open(encoding="utf-8") as handle:
        first = handle.readline()
        if first.startswith("version https://git-lfs.github.com/spec/"):
            raise ValueError(
                f"{path} is a Git LFS pointer, not JSONL; materialize the "
                "producer artifact or use a checked-in compact fixture"
            )
        lines = [first, *handle]
    for line_number, line in enumerate(lines, start=1):
        if not line.strip():
            continue
        try:
            row = json.loads(line)
        except json.JSONDecodeError as error:
            raise ValueError(f"{path}:{line_number}: invalid JSON: {error}") from error
        if not isinstance(row, dict):
            raise ValueError(f"{path}:{line_number}: row must be an object")
        yield row


def require(row: dict[str, Any], keys: set[str], context: str) -> None:
    missing = sorted(keys - row.keys())
    if missing:
        raise ValueError(f"{context}: missing fields {missing}")


def load_bundle(path: Path) -> tuple[
    dict[str, dict[str, Any]],
    dict[str, list[dict[str, Any]]],
    dict[str, list[dict[str, Any]]],
]:
    states: dict[str, dict[str, Any]] = {}
    probes: dict[str, list[dict[str, Any]]] = defaultdict(list)
    continuations: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for index, row in enumerate(read_jsonl(path), start=1):
        if row.get("schema_version") != SCHEMA_VERSION:
            raise ValueError(f"row {index}: unsupported schema_version")
        row_type = row.get("row_type")
        if row_type not in ROW_TYPES:
            raise ValueError(f"row {index}: unsupported row_type {row_type!r}")
        state_id = row.get("state_id")
        if not isinstance(state_id, str) or not state_id:
            raise ValueError(f"row {index}: state_id must be a nonempty string")
        if row_type == "state":
            require(
                row,
                {
                    "source",
                    "control_role",
                    "selection_kind",
                    "outcome_selected",
                    "state_valid",
                    "suite_id",
                    "expected_probe_ids",
                    "material_delta_sys",
                    "positive_fixed_f_evidence",
                    "facet_addition_evidence",
                },
                f"state {state_id}",
            )
            if state_id in states:
                raise ValueError(f"duplicate state {state_id!r}")
            if not isinstance(row["expected_probe_ids"], list):
                raise ValueError(f"state {state_id}: expected_probe_ids must be a list")
            states[state_id] = row
        elif row_type == "probe":
            require(
                row,
                {
                    "probe_id",
                    "suite_id",
                    "direction_family",
                    "direction_index",
                    "sign",
                    "radius",
                    "step_norm",
                    "base_sys",
                    "perturbed_sys",
                    "delta_sys",
                    "state_valid",
                    "same_incidence",
                    "uncertainty",
                    "failure",
                },
                f"probe row {index}",
            )
            probes[state_id].append(row)
        else:
            require(
                row,
                {
                    "continuation_id",
                    "extra_compute_ms",
                    "extra_trusted_calls",
                    "delta_sys",
                    "path_length",
                    "displacement",
                    "stopping_reason",
                    "state_valid",
                    "required_for_classification",
                },
                f"continuation row {index}",
            )
            continuations[state_id].append(row)
    dangling = (set(probes) | set(continuations)) - set(states)
    if dangling:
        raise ValueError(f"evidence references unknown states: {sorted(dangling)}")
    return states, dict(probes), dict(continuations)


def finite_number(value: Any) -> float | None:
    if isinstance(value, (int, float)) and math.isfinite(value):
        return float(value)
    return None


def classify_state(
    state: dict[str, Any],
    probes: list[dict[str, Any]],
    continuations: list[dict[str, Any]],
) -> dict[str, Any]:
    threshold = float(state["material_delta_sys"])
    expected_ids = list(state["expected_probe_ids"])
    expected_set = set(expected_ids)
    if len(expected_set) != len(expected_ids):
        raise ValueError(f"state {state['state_id']}: duplicate expected_probe_ids")
    by_id: dict[str, dict[str, Any]] = {}
    for probe in probes:
        probe_id = probe["probe_id"]
        if probe_id in by_id:
            raise ValueError(f"state {state['state_id']}: duplicate probe {probe_id!r}")
        if probe["suite_id"] != state["suite_id"]:
            raise ValueError(
                f"state {state['state_id']}: probe {probe_id!r} has another suite"
            )
        by_id[probe_id] = probe

    valid_probe_deltas = [
        finite_number(probe["delta_sys"])
        for probe in probes
        if probe["state_valid"]
    ]
    valid_probe_deltas = [value for value in valid_probe_deltas if value is not None]
    valid_continuation_deltas = [
        finite_number(row["delta_sys"])
        for row in continuations
        if row["state_valid"]
    ]
    valid_continuation_deltas = [
        value for value in valid_continuation_deltas if value is not None
    ]
    all_valid_deltas = valid_probe_deltas + valid_continuation_deltas
    improvement_found = any(value > threshold for value in all_valid_deltas)

    missing_probe_ids = sorted(expected_set - by_id.keys())
    unexpected_probe_ids = sorted(by_id.keys() - expected_set)
    invalid_probe_ids = sorted(
        probe_id
        for probe_id in expected_set & by_id.keys()
        if not by_id[probe_id]["state_valid"]
        or finite_number(by_id[probe_id]["delta_sys"]) is None
        or by_id[probe_id]["uncertainty"] == "indeterminate"
    )
    invalid_required_continuations = sorted(
        row["continuation_id"]
        for row in continuations
        if row["required_for_classification"]
        and (not row["state_valid"] or finite_number(row["delta_sys"]) is None)
    )

    if improvement_found:
        finite_classification = "explicit_improvement_found"
    elif not state["state_valid"]:
        finite_classification = "invalid_or_indeterminate"
    elif state["suite_id"] is None and not expected_ids:
        finite_classification = "no_finite_suite"
    elif (
        not expected_ids
        or missing_probe_ids
        or invalid_probe_ids
        or invalid_required_continuations
    ):
        finite_classification = "invalid_or_indeterminate"
    else:
        finite_classification = "finite_suite_survivor"

    control_expected = state.get("expected_finite_classification")
    control_passed = (
        None if control_expected is None else control_expected == finite_classification
    )
    best_delta = max(all_valid_deltas) if all_valid_deltas else None
    slopes = [
        finite_number(probe["delta_sys"]) / float(probe["step_norm"])
        for probe in probes
        if probe["state_valid"]
        and finite_number(probe["delta_sys"]) is not None
        and finite_number(probe["step_norm"]) not in (None, 0.0)
    ]
    valid_continuations = [row for row in continuations if row["state_valid"]]

    return {
        "schema_version": SCHEMA_VERSION,
        "state_id": state["state_id"],
        "source": state["source"],
        "control_role": state["control_role"],
        "selection_kind": state["selection_kind"],
        "outcome_selected": state["outcome_selected"],
        "optimizer_provenance": state.get("optimizer_provenance"),
        "stopping_reason": state.get("stopping_reason"),
        "measured_compute_ms": state.get("measured_compute_ms"),
        "trusted_call_count": state.get("trusted_call_count"),
        "finite_classification": finite_classification,
        "suite_id": state["suite_id"],
        "expected_probe_count": len(expected_ids),
        "observed_probe_count": len(probes),
        "valid_expected_probe_count": len(expected_set)
        - len(missing_probe_ids)
        - len(invalid_probe_ids),
        "missing_probe_ids": missing_probe_ids,
        "unexpected_probe_ids": unexpected_probe_ids,
        "invalid_probe_ids": invalid_probe_ids,
        "invalid_required_continuations": invalid_required_continuations,
        "direction_families": sorted({row["direction_family"] for row in probes}),
        "radii": sorted({float(row["radius"]) for row in probes}),
        "best_delta_sys": best_delta,
        "best_delta_sys_per_step": max(slopes) if slopes else None,
        "continuation_gain": (
            max(valid_continuation_deltas) if valid_continuation_deltas else None
        ),
        "continuation_extra_compute_ms": sum(
            float(row["extra_compute_ms"]) for row in valid_continuations
        ),
        "continuation_extra_trusted_calls": sum(
            int(row["extra_trusted_calls"]) for row in valid_continuations
        ),
        "continuation_path_length": sum(
            float(row["path_length"]) for row in valid_continuations
        ),
        "continuation_displacement": (
            max(float(row["displacement"]) for row in valid_continuations)
            if valid_continuations
            else None
        ),
        "positive_fixed_f_evidence": state["positive_fixed_f_evidence"],
        "facet_addition_evidence": state["facet_addition_evidence"],
        "control_expected_classification": control_expected,
        "control_passed": control_passed,
        "raw_probe_rows": sorted(probes, key=lambda row: row["probe_id"]),
        "raw_continuation_rows": sorted(
            continuations, key=lambda row: row["continuation_id"]
        ),
        "claim_boundary": CLAIM_BOUNDARY,
    }


def report(rows: list[dict[str, Any]]) -> str:
    lines = [
        "# Endpoint Evidence Report",
        "",
        CLAIM_BOUNDARY,
        "",
        "| State | Role | Finite evidence | Best change | Continuation gain | Control |",
        "| --- | --- | --- | ---: | ---: | --- |",
    ]
    for row in rows:
        best = "-" if row["best_delta_sys"] is None else f"{row['best_delta_sys']:.6g}"
        continuation = (
            "-"
            if row["continuation_gain"] is None
            else f"{row['continuation_gain']:.6g}"
        )
        control = (
            "-"
            if row["control_passed"] is None
            else ("pass" if row["control_passed"] else "FAIL")
        )
        lines.append(
            f"| `{row['state_id']}` | {row['control_role']} | "
            f"`{row['finite_classification']}` | {best} | {continuation} | "
            f"{control} |"
        )
    lines.extend(
        [
            "",
            "Positive fixed-facet evidence and named facet-addition evidence are "
            "retained in `normalized-evidence.jsonl`; neither is inferred from "
            "the finite classification.",
            "",
        ]
    )
    return "\n".join(lines)


def write_outputs(input_path: Path, out_dir: Path) -> None:
    if out_dir.exists():
        raise ValueError(f"output path already exists: {out_dir}")
    states, probes, continuations = load_bundle(input_path)
    rows = [
        classify_state(
            state,
            probes.get(state_id, []),
            continuations.get(state_id, []),
        )
        for state_id, state in sorted(states.items())
    ]
    failed_controls = [
        row["state_id"] for row in rows if row["control_passed"] is False
    ]
    summary = {
        "schema_version": SCHEMA_VERSION,
        "state_count": len(rows),
        "classification_counts": dict(
            sorted(Counter(row["finite_classification"] for row in rows).items())
        ),
        "declared_control_count": sum(
            row["control_passed"] is not None for row in rows
        ),
        "failed_controls": failed_controls,
        "all_declared_controls_passed": not failed_controls,
        "claim_boundary": CLAIM_BOUNDARY,
    }
    out_dir.mkdir(parents=True)
    with (out_dir / "normalized-evidence.jsonl").open(
        "w", encoding="utf-8"
    ) as handle:
        for row in rows:
            handle.write(json.dumps(row, sort_keys=True) + "\n")
    (out_dir / "summary.json").write_text(
        json.dumps(summary, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    (out_dir / "REPORT.md").write_text(report(rows), encoding="utf-8")
    if failed_controls:
        raise ValueError(f"declared controls failed: {failed_controls}")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    write_outputs(args.input, args.out)


if __name__ == "__main__":
    main()
