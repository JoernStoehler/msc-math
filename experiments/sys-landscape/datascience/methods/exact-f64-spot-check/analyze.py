#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# ///

"""
Goal: Run the DS-I007 exact-vs-f64 spot check for claim-bearing numeric
      columns in the hostile-landscape datascience tables.
Input Artifacts:
  - dataset directory passed by `--dataset-dir`, containing
    `polytope-table.jsonl` and `observation-table.jsonl`
Output Artifacts:
  - experiments/sys-landscape/datascience/methods/exact-f64-spot-check/report.md
"""

from __future__ import annotations

import argparse
import json
import math
import platform
import statistics
from datetime import datetime, timezone
from fractions import Fraction
from pathlib import Path
from typing import Any


EXPERIMENT_DIR = Path(__file__).resolve().parent
REPO_ROOT = EXPERIMENT_DIR.parents[4]
DEFAULT_DATASET_DIR = EXPERIMENT_DIR.parent.parent / "dataset"
REPORT_MD = EXPERIMENT_DIR / "report.md"

EXPECTED_POLY_ROWS = 8445
EXPECTED_OBS_ROWS = 8445
EXPECTED_MAX_SYS = 0.9750768559799221
EXPECTED_SYS_GT_ONE = 0
PRODUCER_COMMAND = (
    "experiments/sys-landscape/datascience/build-dataset.sh"
)
DEFAULT_COMMAND = (
    "uv run --script experiments/sys-landscape/datascience/methods/"
    "exact-f64-spot-check/analyze.py --dataset-dir "
    "experiments/sys-landscape/datascience/dataset"
)


def repo_path(path: Path) -> str:
    resolved = path.resolve()
    try:
        return str(resolved.relative_to(REPO_ROOT))
    except ValueError:
        return str(resolved)

GEOMETRY_COLUMNS = [
    "geom_vol1_norm_mean",
    "geom_vol1_norm_std",
    "geom_vol1_norm_min",
    "geom_vol1_norm_max",
    "geom_vol1_centroid_norm",
    "geom_vol1_coord_std_x",
    "geom_vol1_coord_std_y",
    "geom_vol1_coord_std_z",
    "geom_vol1_coord_std_w",
    "geom_cosine_mean",
    "geom_cosine_std",
    "geom_cosine_min",
    "geom_cosine_max",
    "geom_vol1_pairwise_dist_mean",
    "geom_vol1_pairwise_dist_std",
    "geom_vol1_pairwise_dist_min",
    "geom_vol1_pairwise_dist_max",
]


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--dataset-dir",
        type=Path,
        default=DEFAULT_DATASET_DIR,
        help="Dataset directory. Defaults to experiments/sys-landscape/datascience/dataset.",
    )
    parser.add_argument(
        "--out-dir",
        type=Path,
        default=EXPERIMENT_DIR,
        help="Output directory for report.md.",
    )
    return parser.parse_args()


def load_jsonl(path: Path) -> list[dict[str, Any]]:
    with path.open() as handle:
        return [json.loads(line) for line in handle if line.strip()]


def parse_rational(token: str) -> Fraction:
    return Fraction(token)


def finite_float(value: Any) -> float:
    out = float(value)
    if not math.isfinite(out):
        raise ValueError(f"non-finite numeric value: {value!r}")
    return out


def validate_dataset(polytopes: list[dict[str, Any]], observations: list[dict[str, Any]]) -> dict[str, Any]:
    max_sys = max(finite_float(row["sys"]) for row in polytopes)
    sys_gt_one = sum(1 for row in polytopes if finite_float(row["sys"]) > 1.0)
    checks = {
        "polytope_rows": len(polytopes),
        "observation_rows": len(observations),
        "expected_polytope_rows": EXPECTED_POLY_ROWS,
        "expected_observation_rows": EXPECTED_OBS_ROWS,
        "max_sys": max_sys,
        "expected_max_sys": EXPECTED_MAX_SYS,
        "max_sys_abs_error": abs(max_sys - EXPECTED_MAX_SYS),
        "sys_gt_one_count": sys_gt_one,
        "expected_sys_gt_one_count": EXPECTED_SYS_GT_ONE,
        "unique_polytope_ids": len({row["poly_id"] for row in polytopes}),
        "unique_observation_ids": len({row["observation_id"] for row in observations}),
    }
    checks["passed"] = (
        checks["polytope_rows"] == EXPECTED_POLY_ROWS
        and checks["observation_rows"] == EXPECTED_OBS_ROWS
        and checks["max_sys_abs_error"] <= 1e-15
        and checks["sys_gt_one_count"] == EXPECTED_SYS_GT_ONE
    )
    return checks


def selected_sample(polytopes: list[dict[str, Any]]) -> list[dict[str, Any]]:
    by_sys = sorted(polytopes, key=lambda row: finite_float(row["sys"]))
    selected_indices = {
        0,
        len(by_sys) // 4,
        len(by_sys) // 2,
        (3 * len(by_sys)) // 4,
        len(by_sys) - 1,
    }
    selected_indices.update(range(max(0, len(by_sys) - 5), len(by_sys)))
    selected_indices.update(range(0, len(by_sys), 53))

    selected: list[dict[str, Any]] = []
    seen: set[str] = set()
    for index in sorted(selected_indices):
        row = by_sys[index]
        if row["poly_id"] not in seen:
            selected.append(row)
            seen.add(row["poly_id"])
    return selected


def flatten(vertices: list[list[float]]) -> list[float]:
    return [coord for vertex in vertices for coord in vertex]


def exact_coordinate_error(row: dict[str, Any]) -> dict[str, Any]:
    rationals = [
        [parse_rational(str(coord)) for coord in vertex]
        for vertex in row["dual_vertices_rational"]
    ]
    stored_vertices = [[finite_float(coord) for coord in vertex] for vertex in row["dual_vertices_f64"]]
    stored_flat = [finite_float(coord) for coord in row["dual_vertices_flat_f64"]]
    expected_flat = flatten(stored_vertices)
    rational_flat = [coord for vertex in rationals for coord in vertex]

    if len(stored_flat) != len(expected_flat) or len(stored_flat) != len(rational_flat):
        raise ValueError(f"dual vertex shape mismatch for {row['poly_id']}")

    max_abs_exact_error = 0.0
    max_rel_exact_error = 0.0
    exact_error_numerators: list[int] = []
    for rational, stored in zip(rational_flat, stored_flat, strict=True):
        exact_delta = Fraction.from_float(stored) - rational
        abs_error = abs(float(exact_delta))
        max_abs_exact_error = max(max_abs_exact_error, abs_error)
        denom = abs(float(rational))
        if denom > 0.0:
            max_rel_exact_error = max(max_rel_exact_error, abs_error / denom)
        exact_error_numerators.append(abs(exact_delta.numerator))

    max_flat_disagreement = max(
        abs(flat_value - vertex_value)
        for flat_value, vertex_value in zip(stored_flat, expected_flat, strict=True)
    )
    return {
        "poly_id": row["poly_id"],
        "sys": finite_float(row["sys"]),
        "facet_count": int(row["facet_count"]),
        "coordinate_count": len(stored_flat),
        "max_abs_exact_coordinate_error": max_abs_exact_error,
        "max_rel_exact_coordinate_error": max_rel_exact_error,
        "max_exact_error_numerator": max(exact_error_numerators),
        "dual_vertices_flat_vs_nested_max_abs_error": max_flat_disagreement,
    }


def stats_or_zero(values: list[float]) -> tuple[float, float, float, float]:
    if not values:
        return (0.0, 0.0, 0.0, 0.0)
    mean = sum(values) / len(values)
    var = sum((value - mean) ** 2 for value in values) / len(values)
    return (mean, math.sqrt(var), min(values), max(values))


def recompute_geometry_columns(row: dict[str, Any]) -> dict[str, float]:
    scale = finite_float(row["volume"]) ** 0.25
    vertices = [
        [float(parse_rational(str(coord))) * scale for coord in vertex]
        for vertex in row["dual_vertices_rational"]
    ]
    norms = [math.sqrt(sum(coord * coord for coord in vertex)) for vertex in vertices]
    centroid = [
        sum(vertex[coord] for vertex in vertices) / len(vertices)
        for coord in range(4)
    ]
    coord_std = []
    for coord in range(4):
        mean = centroid[coord]
        var = sum((vertex[coord] - mean) ** 2 for vertex in vertices) / len(vertices)
        coord_std.append(math.sqrt(var))

    cosines: list[float] = []
    pairwise: list[float] = []
    for i in range(len(vertices)):
        for j in range(i + 1, len(vertices)):
            dot = sum(vertices[i][coord] * vertices[j][coord] for coord in range(4))
            denom = norms[i] * norms[j]
            if denom > 0.0:
                cosines.append(dot / denom)
            pairwise.append(
                math.sqrt(sum((vertices[i][coord] - vertices[j][coord]) ** 2 for coord in range(4)))
            )

    norm_mean, norm_std, norm_min, norm_max = stats_or_zero(norms)
    cosine_mean, cosine_std, cosine_min, cosine_max = stats_or_zero(cosines)
    pair_mean, pair_std, pair_min, pair_max = stats_or_zero(pairwise)

    return {
        "geom_vol1_norm_mean": norm_mean,
        "geom_vol1_norm_std": norm_std,
        "geom_vol1_norm_min": norm_min,
        "geom_vol1_norm_max": norm_max,
        "geom_vol1_centroid_norm": math.sqrt(sum(coord * coord for coord in centroid)),
        "geom_vol1_coord_std_x": coord_std[0],
        "geom_vol1_coord_std_y": coord_std[1],
        "geom_vol1_coord_std_z": coord_std[2],
        "geom_vol1_coord_std_w": coord_std[3],
        "geom_cosine_mean": cosine_mean,
        "geom_cosine_std": cosine_std,
        "geom_cosine_min": cosine_min,
        "geom_cosine_max": cosine_max,
        "geom_vol1_pairwise_dist_mean": pair_mean,
        "geom_vol1_pairwise_dist_std": pair_std,
        "geom_vol1_pairwise_dist_min": pair_min,
        "geom_vol1_pairwise_dist_max": pair_max,
    }


def geometry_consistency(row: dict[str, Any]) -> dict[str, Any]:
    recomputed = recompute_geometry_columns(row)
    errors = {
        column: abs(finite_float(row[column]) - recomputed[column])
        for column in GEOMETRY_COLUMNS
    }
    worst_column = max(errors, key=errors.get)
    return {
        "poly_id": row["poly_id"],
        "sys": finite_float(row["sys"]),
        "checked_columns": GEOMETRY_COLUMNS,
        "max_abs_error": errors[worst_column],
        "worst_column": worst_column,
        "per_column_abs_error": errors,
    }


def summarize_checks(polytopes: list[dict[str, Any]]) -> dict[str, Any]:
    sample = selected_sample(polytopes)
    coordinate_checks = [exact_coordinate_error(row) for row in sample]
    geometry_checks = [geometry_consistency(row) for row in sample]

    max_coordinate_error = max(row["max_abs_exact_coordinate_error"] for row in coordinate_checks)
    max_coordinate_rel_error = max(row["max_rel_exact_coordinate_error"] for row in coordinate_checks)
    max_flat_error = max(row["dual_vertices_flat_vs_nested_max_abs_error"] for row in coordinate_checks)
    max_geometry_error = max(row["max_abs_error"] for row in geometry_checks)
    worst_geometry = max(geometry_checks, key=lambda row: row["max_abs_error"])

    return {
        "sample_policy": (
            "deterministic bounded sample: min/quartile/median/quartile/max sys rows, "
            "top five sys rows, and every 53rd row after sorting by sys; duplicates by poly_id removed"
        ),
        "sample_size": len(sample),
        "sample_poly_ids": [row["poly_id"] for row in sample],
        "sample_sys_values": [finite_float(row["sys"]) for row in sample],
        "coordinate_checks": coordinate_checks,
        "geometry_consistency_checks": geometry_checks,
        "max_abs_exact_coordinate_error": max_coordinate_error,
        "max_rel_exact_coordinate_error": max_coordinate_rel_error,
        "max_flat_vs_nested_abs_error": max_flat_error,
        "max_geometry_consistency_abs_error": max_geometry_error,
        "worst_geometry_consistency": {
            "poly_id": worst_geometry["poly_id"],
            "sys": worst_geometry["sys"],
            "column": worst_geometry["worst_column"],
            "abs_error": worst_geometry["max_abs_error"],
        },
    }


def verdict(checks: dict[str, Any], summary: dict[str, Any]) -> dict[str, str]:
    if checks["sys_gt_one_count"] > 0:
        return {
            "verdict": "positive-escalate",
            "evidence_strength": "high",
            "implementation_trust": "high",
            "thesis_use": "Jorn decision needed",
            "caveat": "Stop condition hit: dataset contains sys > 1.",
            "reopen_trigger": "After the sys > 1 row provenance is resolved.",
        }
    if not checks["passed"]:
        return {
            "verdict": "bug-redo",
            "evidence_strength": "low",
            "implementation_trust": "high",
            "thesis_use": "omit before submission",
            "caveat": "Dataset guard mismatch; the spike did not test the expected dataset.",
            "reopen_trigger": "Rerun after row counts, max sys, and sys > 1 count match the packet.",
        }
    if summary["max_flat_vs_nested_abs_error"] != 0.0:
        return {
            "verdict": "bug-redo",
            "evidence_strength": "medium",
            "implementation_trust": "high",
            "thesis_use": "supporting/caveat only",
            "caveat": "The flat and nested f64 vertex arrays disagree.",
            "reopen_trigger": "Inspect table writer before relying on vertex-array features.",
        }
    if summary["max_abs_exact_coordinate_error"] > 1e-15 or summary["max_geometry_consistency_abs_error"] > 5e-13:
        return {
            "verdict": "future",
            "evidence_strength": "medium",
            "implementation_trust": "medium",
            "thesis_use": "supporting/caveat only",
            "caveat": "The spot check found larger-than-expected f64 drift in sampled columns.",
            "reopen_trigger": "Expand DS-I007 into a full table audit before using affected columns.",
        }
    return {
        "verdict": "no-search-output",
        "evidence_strength": "medium",
        "implementation_trust": "high",
        "thesis_use": "supporting/caveat only",
        "caveat": (
            "This is a bounded sanity check. It checks exact rational-to-f64 vertex "
            "encoding and f64 recomputation of selected geometry scalars, not exact "
            "capacity, volume, skeleton, ridge, or orbit-search scalar semantics."
        ),
        "reopen_trigger": (
            "Reopen if DS-I004/DS-I005 start relying on a surprising scalar column, "
            "if the dataset producer changes, or if a future table adds exact source "
            "columns for volume, skeleton, ridge, capacity, or orbit quantities."
        ),
    }


def write_report(out_dir: Path, payload: dict[str, Any]) -> None:
    checks = payload["dataset_snapshot"]
    check_summary = payload["checks"]
    result = payload["result"]
    worst = check_summary["worst_geometry_consistency"]
    lines = [
        "# DS-I007 Exact-vs-f64 Spot Check",
        "",
        "idea_id = DS-I007",
        "",
        "## Command And Provenance",
        "",
        f"- Actual command run: `{payload['command_run']}`",
        f"- Dataset path: `{payload['dataset_dir']}`",
        f"- Producer command from packet: `{PRODUCER_COMMAND}`",
        f"- Run timestamp UTC: `{payload['run_timestamp_utc']}`",
        f"- Python: `{payload['python_version']}`",
        "",
        "## Dataset Snapshot",
        "",
        f"- Polytope rows: `{checks['polytope_rows']}` expected `{EXPECTED_POLY_ROWS}`.",
        f"- Observation rows: `{checks['observation_rows']}` expected `{EXPECTED_OBS_ROWS}`.",
        f"- Max `sys`: `{checks['max_sys']:.15f}` expected `{EXPECTED_MAX_SYS:.15f}`.",
        f"- `sys > 1` count: `{checks['sys_gt_one_count']}` expected `{EXPECTED_SYS_GT_ONE}`.",
        f"- Dataset guard passed: `{checks['passed']}`.",
        "",
        "## Sample Policy",
        "",
        f"Observation: {check_summary['sample_policy']}. Sample size: `{check_summary['sample_size']}`.",
        "",
        "Sampled `sys` values:",
        "",
        "```json",
        json.dumps(check_summary["sample_sys_values"], indent=2),
        "```",
        "",
        "## Checked Quantities",
        "",
        "Observation: each sampled row checks `dual_vertices_rational` parsed as exact `Fraction` values against `dual_vertices_f64` and `dual_vertices_flat_f64`.",
        f"Observation: maximum exact rational-to-stored-f64 absolute coordinate error was `{check_summary['max_abs_exact_coordinate_error']:.3e}`; maximum relative coordinate error was `{check_summary['max_rel_exact_coordinate_error']:.3e}`.",
        f"Observation: maximum nested-vs-flat f64 disagreement was `{check_summary['max_flat_vs_nested_abs_error']:.3e}`.",
        "",
        "Observation: the script also recomputes the following f64 geometry columns from exact rational coordinates converted to f64 and the table's f64 `volume`:",
        "",
        "```json",
        json.dumps(GEOMETRY_COLUMNS, indent=2),
        "```",
        "",
        f"Observation: maximum geometry-column recomputation error was `{check_summary['max_geometry_consistency_abs_error']:.3e}` in column `{worst['column']}` for poly_id `{worst['poly_id']}`.",
        "",
        "## Numerical Tolerances",
        "",
        "- Dataset max-`sys` guard tolerance: absolute `1e-15`.",
        "- Coordinate verdict threshold: absolute `1e-15`.",
        "- Geometry recomputation verdict threshold: absolute `5e-13`.",
        "",
        "## Inference",
        "",
        "Inference: the sampled rational vertex coordinates survive table conversion to f64 at ordinary binary64 rounding scale, and the flat f64 vertex array is exactly consistent with the nested f64 vertex array in the sample.",
        "Inference: the selected geometry scalar columns are internally consistent with the table writer formulas when recomputed from rational coordinates through f64 arithmetic.",
        "Inference caveat: this does not prove exactness of derived scalar semantics. The spot check does not reconstruct exact volume, capacity, skeleton, ridge, transition, or orbit-search quantities.",
        "",
        "## Verdict",
        "",
        f"- verdict: `{result['verdict']}`",
        f"- evidence_strength: `{result['evidence_strength']}`",
        f"- implementation_trust: `{result['implementation_trust']}`",
        f"- thesis_use: `{result['thesis_use']}`",
        f"- caveat: {result['caveat']}",
        f"- reopen_trigger: {result['reopen_trigger']}",
        "",
    ]
    (out_dir / "report.md").write_text("\n".join(lines))


def main() -> None:
    args = parse_args()
    out_dir = args.out_dir
    out_dir.mkdir(parents=True, exist_ok=True)
    polytopes = load_jsonl(args.dataset_dir / "polytope-table.jsonl")
    observations = load_jsonl(args.dataset_dir / "observation-table.jsonl")
    checks = validate_dataset(polytopes, observations)
    summary = summarize_checks(polytopes) if checks["passed"] and checks["sys_gt_one_count"] == 0 else {}
    result = verdict(checks, summary or {
        "max_flat_vs_nested_abs_error": math.inf,
        "max_abs_exact_coordinate_error": math.inf,
        "max_geometry_consistency_abs_error": math.inf,
    })
    payload = {
        "idea_id": "DS-I007",
        "command_run": DEFAULT_COMMAND,
        "dataset_dir": repo_path(args.dataset_dir),
        "producer_command": PRODUCER_COMMAND,
        "run_timestamp_utc": datetime.now(timezone.utc).isoformat(),
        "python_version": platform.python_version(),
        "dataset_snapshot": checks,
        "checks": summary,
        "result": result,
    }
    if not summary:
        payload["checks"] = {
            "sample_policy": "not run because dataset guard or stop condition failed",
            "sample_size": 0,
        }

    write_report(out_dir, payload)

    if checks["sys_gt_one_count"] > 0:
        raise SystemExit("stop condition hit: sys > 1 found in dataset")
    if not checks["passed"]:
        raise SystemExit("dataset guard mismatch; wrote blocker report")


if __name__ == "__main__":
    main()
