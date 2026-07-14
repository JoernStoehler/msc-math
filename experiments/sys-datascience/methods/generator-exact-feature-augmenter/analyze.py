#!/usr/bin/env python3
# /// script
# dependencies = ["numpy"]
# ///
"""Validation/reporting for the target-free exact feature packet."""

from __future__ import annotations

import argparse
import json
import math
from collections import defaultdict
from pathlib import Path
from typing import Any

from augment import AnalysisError, load_rows


def validate(rows: list[dict[str, Any]], require_complete: bool = True) -> None:
    if not rows:
        raise AnalysisError("empty feature packet")
    ids = set()
    for row in rows:
        if row.get("schema") != "generator-exact-feature-augmenter-row-v1":
            raise AnalysisError("unexpected feature schema")
        sid = row.get("source_id")
        if not isinstance(sid, str) or sid in ids:
            raise AnalysisError("duplicate or missing source_id")
        ids.add(sid)
        if row.get("geometry_validation_status") != "validated":
            raise AnalysisError("unvalidated geometry row")
        for key in ("capacity", "sys", "iterations", "bounce_label", "target"):
            if row.get(key) is not None:
                raise AnalysisError(f"target field {key} present in feature input")
        if row.get("coordinate_order") != "q1,q2,p1,p2":
            raise AnalysisError("wrong coordinate order")
        if row.get("ordering_failure_count", 0) != 0:
            raise AnalysisError("face ordering failure present")
        if not row.get("decomposition_identity_ok") or row.get("decomposition_max_abs_error", math.inf) > 1e-10:
            raise AnalysisError("ridge-area decomposition audit failed")
        if row.get("strict_cycle") is not None:
            allowed = row.get("source_kind") == "orientation" and row.get("bucket") == "3x3" and row.get("map_variant") == "identity"
            allowed |= row.get("source_kind") == "tangential" and row.get("bucket") == "3x3" and row.get("law") == "factorial-baseline"
            if not allowed:
                raise AnalysisError("forbidden row has non-null strict-cycle metadata")
            cycle = row["strict_cycle"]
            if not isinstance(cycle.get("strict_sign_cell"), bool) or not isinstance(cycle.get("strict_cycle_feasible"), bool):
                raise AnalysisError("strict-cycle metadata is incomplete")
        cov = row.get("vertex_covariance", {})
        if cov.get("status") == "eligible" and any(not math.isfinite(float(cov[k])) for k in ("nu1", "nu2", "rho", "condition")):
            raise AnalysisError("nonfinite covariance diagnostic")
    if not require_complete:
        return
    orientations = [r for r in rows if r.get("source_kind") == "orientation"]
    for base in {(r.get("source_id", "").split("/map=")[0], r.get("bucket")) for r in orientations}:
        cells = [r for r in orientations if (r.get("source_id", "").split("/map=")[0], r.get("bucket")) == base]
        if len(cells) != 5 or {r.get("source_id", "").split("/map=")[-1] for r in cells} != {"identity", "u2-deterministic", "u2-haar", "so4-deterministic", "so4-haar"}:
            raise AnalysisError(f"incomplete orientation five-variant grid for {base}")
    tang = [r for r in rows if r.get("source_kind") == "tangential"]
    for key in {(r.get("source_pairing_id"), r.get("bucket")) for r in tang}:
        cells = [r for r in tang if (r.get("source_pairing_id"), r.get("bucket")) == key]
        if len(cells) != 4 or {next((arm for arm in ("factorial-baseline", "factorial-q", "factorial-p", "factorial-both") if arm in r.get("source_id", "")), "") for r in cells} != {"factorial-baseline", "factorial-q", "factorial-p", "factorial-both"}:
            raise AnalysisError(f"incomplete tangential four-arm grid for {key}")


def report(rows: list[dict[str, Any]]) -> dict[str, Any]:
    by_group: dict[tuple[str, str], list[dict[str, Any]]] = defaultdict(list)
    for row in rows:
        by_group[(row["source_kind"], row["bucket"])].append(row)
    groups = []
    for (kind, bucket), values in sorted(by_group.items()):
        item = {
            "source_kind": kind, "bucket": bucket, "rows": len(values),
            "euclidean_mean_range": [min(r["euclidean_ridge_area_x_mean"] for r in values), max(r["euclidean_ridge_area_x_mean"] for r in values)],
            "symplectic_mean_range": [min(r["symplectic_ridge_area_x_mean"] for r in values), max(r["symplectic_ridge_area_x_mean"] for r in values)],
            "kappa_weighted_range": [min(r["kappa_euclidean_weighted_mean"] for r in values), max(r["kappa_euclidean_weighted_mean"] for r in values)],
            "covariance_eligible": sum(r["vertex_covariance"].get("status") == "eligible" for r in values),
        }
        if kind == "tangential":
            arm_values: dict[str, list[float]] = defaultdict(list)
            for value in values:
                arm_values[value["law"]].append(value["euclidean_ridge_area_x_mean"])
            ranges = {arm: [min(v), max(v)] for arm, v in sorted(arm_values.items())}
            item["euclidean_mean_range_by_arm"] = ranges
            item["euclidean_mean_all_arm_overlap"] = [max(v[0] for v in ranges.values()), min(v[1] for v in ranges.values())] if ranges else [None, None]
        groups.append(item)
    def orientation_variant(row: dict[str, Any]) -> str:
        return row["map_variant"]

    orientation_pairs = []
    orient_groups: dict[tuple[str, str], list[dict[str, Any]]] = defaultdict(list)
    for row in rows:
        if row["source_kind"] == "orientation":
            orient_groups[(row["source_id"].split("/map=", 1)[0], row["bucket"])].append(row)
    for (base, bucket), values in sorted(orient_groups.items()):
        by_variant = {orientation_variant(r): r for r in values}
        identity = by_variant.get("identity")
        if identity is None:
            continue
        for variant, row in sorted(by_variant.items()):
            if variant == "identity":
                continue
            d_e = row["euclidean_ridge_area_x_mean"] - identity["euclidean_ridge_area_x_mean"]
            d_s = row["symplectic_ridge_area_x_mean"] - identity["symplectic_ridge_area_x_mean"]
            d_k = row["kappa_euclidean_weighted_mean"] - identity["kappa_euclidean_weighted_mean"]
            cov_a, cov_b = row["vertex_covariance"], identity["vertex_covariance"]
            d_cov = None if cov_a.get("ordinary_eigenvalue_max") is None or cov_b.get("ordinary_eigenvalue_max") is None else cov_a["ordinary_eigenvalue_max"] - cov_b["ordinary_eigenvalue_max"]
            d_cov_min = None if cov_a.get("ordinary_eigenvalue_min") is None or cov_b.get("ordinary_eigenvalue_min") is None else cov_a["ordinary_eigenvalue_min"] - cov_b["ordinary_eigenvalue_min"]
            def cov_delta(name: str) -> float | None:
                a, b = cov_a.get(name), cov_b.get(name)
                return None if a is None or b is None else float(a) - float(b)
            d_volume = row["volume"] - identity["volume"]
            d_condition = cov_delta("condition")
            d_nu1, d_nu2, d_rho = (cov_delta(name) for name in ("nu1", "nu2", "rho"))
            orientation_pairs.append({"base": base, "bucket": bucket, "variant": variant, "delta_volume": d_volume, "delta_euclidean_mean": d_e, "delta_symplectic_mean": d_s, "delta_kappa_weighted": d_k, "delta_covariance_ordinary_max": d_cov, "delta_covariance_ordinary_min": d_cov_min, "delta_covariance_condition": d_condition, "delta_williamson_nu1": d_nu1, "delta_williamson_nu2": d_nu2, "delta_williamson_rho": d_rho, "orthogonal_volume_within_tolerance": abs(d_volume) <= 1e-10 * max(1.0, abs(identity["volume"])), "orthogonal_euclidean_within_tolerance": abs(d_e) <= 1e-9, "orthogonal_covariance_within_tolerance": (d_cov is None or abs(d_cov) <= 1e-8) and (d_cov_min is None or abs(d_cov_min) <= 1e-8), "u2_symplectic_observed": variant.startswith("u2-"), "u2_kappa_observed": variant.startswith("u2-")})

    def tangential_arm(row: dict[str, Any]) -> str:
        return row["law"]

    tangential_pairs = []
    tang_groups: dict[tuple[str, str], list[dict[str, Any]]] = defaultdict(list)
    for row in rows:
        if row["source_kind"] == "tangential":
            tang_groups[(row["source_pairing_id"], row["bucket"])].append(row)
    for (pairing, bucket), values in sorted(tang_groups.items()):
        by_arm = {tangential_arm(r): r for r in values}
        baseline = by_arm.get("factorial-baseline")
        if baseline is None:
            continue
        for arm, row in sorted(by_arm.items()):
            if arm == "factorial-baseline":
                continue
            tangential_pairs.append({"pairing_id": pairing, "bucket": bucket, "arm": arm, "delta_euclidean_mean": row["euclidean_ridge_area_x_mean"] - baseline["euclidean_ridge_area_x_mean"], "delta_symplectic_mean": row["symplectic_ridge_area_x_mean"] - baseline["symplectic_ridge_area_x_mean"], "delta_kappa_weighted": row["kappa_euclidean_weighted_mean"] - baseline["kappa_euclidean_weighted_mean"]})

    return {"schema": "generator-exact-feature-augmenter-report-v1", "target_free": True, "row_count": len(rows), "groups": groups, "orientation_tolerances": {"volume_relative": 1e-10, "euclidean_mean_abs": 1e-9, "covariance_ordinary_max_abs": 1e-8}, "orientation_paired_deltas": orientation_pairs, "tangential_paired_deltas": tangential_pairs, "strict_cycle_metadata_rows": sum(r.get("strict_cycle") is not None for r in rows), "strict_cycle_used_for_grouping_or_selection": False}


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", type=Path, required=True)
    parser.add_argument("--out-dir", type=Path, required=True)
    args = parser.parse_args()
    rows = load_rows(args.input)
    validate(rows)
    args.out_dir.mkdir(parents=True, exist_ok=True)
    (args.out_dir / "report.json").write_text(json.dumps(report(rows), indent=2) + "\n")


if __name__ == "__main__":
    main()
