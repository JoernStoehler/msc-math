#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy", "scikit-learn"]
# ///

"""Build target-free matched pairs and post-target residual inspection seeds."""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import sys
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any

import numpy as np
from sklearn.decomposition import PCA
from sklearn.ensemble import HistGradientBoostingRegressor
from sklearn.linear_model import Ridge
from sklearn.model_selection import StratifiedKFold
from sklearn.neighbors import NearestNeighbors
from sklearn.pipeline import make_pipeline
from sklearn.preprocessing import StandardScaler

HERE = Path(__file__).resolve().parent
sys.path.append(str(HERE.parent / "_shared"))
from random_only import (  # noqa: E402
    TABLES_DIR,
    active_invariant_numeric_feature_names,
    load_trusted_random_tables,
    matrix_for,
    product_bucket,
    provenance_by_poly_id,
)

EXPECTED_TABLE_SHA256 = (
    "49825d7636246f71f4ebd419cf0ccbc86e39e6b7f43d4b03e889bb85e4887aea"
)
EXPECTED_PROVENANCE_SHA256 = (
    "6ff88a5accce9a7ec7e5a494107350b0974b2ce0268ea44caae36a18a7494ef2"
)
EXPECTED_ROWS = 14_336
SENSITIVITY_QUANTILES = (0.10, 0.25, 0.50)
PRIMARY_QUANTILE = 0.25
OOF_SEEDS = (20260711, 20260712)
LBO_SEED = 20260713


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--tables-dir", type=Path, default=TABLES_DIR)
    parser.add_argument("--out-dir", type=Path, default=HERE / "artifacts")
    return parser.parse_args()


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for block in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def write_tsv(path: Path, rows: list[dict[str, Any]], fields: list[str]) -> None:
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(
            handle,
            fieldnames=fields,
            extrasaction="ignore",
            delimiter="\t",
            lineterminator="\n",
        )
        writer.writeheader()
        writer.writerows(rows)


def write_json(path: Path, value: Any) -> None:
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")


def source_bucket(
    row: dict[str, Any], provenance: dict[str, list[dict[str, Any]]]
) -> str:
    if row["capacity_source"] == "random_product_sample":
        return "product-" + product_bucket(provenance[str(row["poly_id"])])
    return f"generic-F{int(row['facet_count'])}"


def arm_for(row: dict[str, Any]) -> str:
    return (
        "product_exact_summary"
        if row["capacity_source"] == "random_product_sample"
        else "generic_caliper"
    )


def pca95(values: np.ndarray) -> np.ndarray:
    if values.shape[1] == 1:
        return values.copy()
    return PCA(n_components=0.95, svd_solver="full").fit_transform(values)


def nearest_map(
    values: np.ndarray, groups: list[list[int]]
) -> tuple[dict[int, int], dict[int, float]]:
    nearest: dict[int, int] = {}
    distance: dict[int, float] = {}
    for group in groups:
        if len(group) < 2:
            continue
        idx = np.asarray(group, dtype=int)
        dist, pos = (
            NearestNeighbors(n_neighbors=2).fit(values[idx]).kneighbors(values[idx])
        )
        for local, row_index in enumerate(idx):
            nearest[int(row_index)] = int(idx[pos[local, 1]])
            distance[int(row_index)] = float(dist[local, 1])
    return nearest, distance


def scaled_by_bucket(values: np.ndarray, buckets: np.ndarray) -> np.ndarray:
    out = np.empty_like(values, dtype=float)
    for bucket in sorted(set(buckets)):
        idx = np.flatnonzero(buckets == bucket)
        out[idx] = StandardScaler().fit_transform(values[idx])
    return out


def build_pair_candidates(
    rows: list[dict[str, Any]],
    buckets: np.ndarray,
    combinatorial_names: list[str],
    ridge_names: list[str],
) -> tuple[list[dict[str, Any]], dict[str, Any]]:
    y = np.asarray([float(row["sys"]) for row in rows])
    comb = np.asarray(matrix_for(rows, combinatorial_names), dtype=float)
    ridge = np.asarray(matrix_for(rows, ridge_names), dtype=float)
    ridge_mean = np.asarray(
        [float(row["ridge_symp_area_mean_over_volume_sqrt"]) for row in rows]
    )
    compact = np.asarray(
        [
            [
                float(row["ridge_symp_area_mean_over_volume_sqrt"]),
                float(row["ridge_symp_area_std_over_volume_sqrt"])
                / max(abs(float(row["ridge_symp_area_mean_over_volume_sqrt"])), 1e-12),
                float(row["ridge_symp_area_q95_over_volume_sqrt"])
                / max(abs(float(row["ridge_symp_area_mean_over_volume_sqrt"])), 1e-12),
                float(row["ridge_symp_area_max_share"]),
                float(row["ridge_symp_area_normalized_entropy"]),
                float(row["ridge_symp_area_le_1em2_over_volume_sqrt_fraction"]),
            ]
            for row in rows
        ],
        dtype=float,
    )

    comb_scaled = scaled_by_bucket(comb, buckets)
    compact_scaled = scaled_by_bucket(compact, buckets)
    ridge_pca_scaled = scaled_by_bucket(
        pca95(StandardScaler().fit_transform(ridge)), buckets
    )
    comb_pca_scaled = scaled_by_bucket(
        pca95(StandardScaler().fit_transform(comb)), buckets
    )

    product_strata: dict[tuple[Any, ...], list[int]] = defaultdict(list)
    generic_groups: dict[str, list[int]] = defaultdict(list)
    for index, row in enumerate(rows):
        if arm_for(row) == "product_exact_summary":
            signature = tuple(float(row[name]) for name in combinatorial_names)
            product_strata[(str(buckets[index]), *signature)].append(index)
        else:
            generic_groups[str(buckets[index])].append(index)

    product_groups = [
        indices for indices in product_strata.values() if len(indices) >= 2
    ]
    generic_group_list = [
        indices for indices in generic_groups.values() if len(indices) >= 2
    ]

    # Product arm: exact current combinatorial summaries, then agreement between
    # compact and full-family ridge nearest-neighbor maps.
    p_a, p_da = nearest_map(compact_scaled, product_groups)
    p_b, p_db = nearest_map(ridge_pca_scaled, product_groups)

    # Generic arm: summaries rarely repeat. Require the same mutual neighbor
    # under two independently reduced combined maps, and retain separate
    # combinatorial/ridge distances for calipers and review.
    generic_a = np.column_stack([comb_scaled, compact_scaled])
    generic_b = np.column_stack([comb_pca_scaled, ridge_pca_scaled])
    g_a, g_da = nearest_map(generic_a, generic_group_list)
    g_b, g_db = nearest_map(generic_b, generic_group_list)

    candidates: list[dict[str, Any]] = []
    seen: set[tuple[int, int]] = set()

    def add_robust_pairs(
        arm: str,
        first: dict[int, int],
        first_distance: dict[int, float],
        second: dict[int, int],
        second_distance: dict[int, float],
    ) -> None:
        for i, j in first.items():
            pair = tuple(sorted((i, j)))
            if (
                pair in seen
                or first.get(j) != i
                or second.get(i) != j
                or second.get(j) != i
            ):
                continue
            seen.add(pair)
            low, high = (i, j) if y[i] <= y[j] else (j, i)
            candidates.append(
                {
                    "arm": arm,
                    "bucket": str(buckets[i]),
                    "poly_id_low": str(rows[low]["poly_id"]),
                    "poly_id_high": str(rows[high]["poly_id"]),
                    "sys_low": float(y[low]),
                    "sys_high": float(y[high]),
                    "sys_gap": float(y[high] - y[low]),
                    "metric_a_distance": float(
                        max(first_distance[i], first_distance[j])
                    ),
                    "metric_b_distance": float(
                        max(second_distance[i], second_distance[j])
                    ),
                    "combinatorial_distance": float(
                        np.linalg.norm(comb_scaled[i] - comb_scaled[j])
                    ),
                    "ridge_compact_distance": float(
                        np.linalg.norm(compact_scaled[i] - compact_scaled[j])
                    ),
                    "ridge_full_pca_distance": float(
                        np.linalg.norm(ridge_pca_scaled[i] - ridge_pca_scaled[j])
                    ),
                    "ridge_mean_low": float(ridge_mean[low]),
                    "ridge_mean_high": float(ridge_mean[high]),
                }
            )

    add_robust_pairs("product_exact_summary", p_a, p_da, p_b, p_db)
    add_robust_pairs("generic_caliper", g_a, g_da, g_b, g_db)

    thresholds: dict[str, dict[str, float]] = {}
    for arm in ("product_exact_summary", "generic_caliper"):
        arm_rows = [row for row in candidates if row["arm"] == arm]
        if not arm_rows:
            continue
        for quantile in SENSITIVITY_QUANTILES:
            label = f"q{int(quantile * 100):02d}"
            a_cut = float(
                np.quantile([row["metric_a_distance"] for row in arm_rows], quantile)
            )
            b_cut = float(
                np.quantile([row["metric_b_distance"] for row in arm_rows], quantile)
            )
            thresholds[f"{arm}:{label}"] = {"metric_a": a_cut, "metric_b": b_cut}
            for row in arm_rows:
                row[f"selected_{label}"] = (
                    row["metric_a_distance"] <= a_cut
                    and row["metric_b_distance"] <= b_cut
                )

    candidates.sort(
        key=lambda row: (
            row["arm"],
            row["bucket"],
            row["poly_id_low"],
            row["poly_id_high"],
        )
    )
    metadata = {
        "product_eligible_exact_summary_strata": len(product_groups),
        "product_eligible_rows": sum(len(group) for group in product_groups),
        "generic_bucket_groups": len(generic_group_list),
        "robust_candidate_counts": Counter(row["arm"] for row in candidates),
        "thresholds": thresholds,
    }
    return candidates, metadata


def model(name: str, seed: int):
    if name == "ridge":
        return make_pipeline(StandardScaler(), Ridge(alpha=10.0))
    if name == "hist_gradient_boosting":
        return HistGradientBoostingRegressor(
            max_iter=140,
            max_leaf_nodes=31,
            min_samples_leaf=20,
            l2_regularization=1.0,
            random_state=seed,
        )
    raise ValueError(name)


def build_residual_candidates(
    rows: list[dict[str, Any]], features: list[str], buckets: np.ndarray
) -> tuple[list[dict[str, Any]], dict[str, Any]]:
    names = ("ridge", "hist_gradient_boosting")
    x = np.asarray(matrix_for(rows, features), dtype=float)
    y = np.asarray([float(row["sys"]) for row in rows])
    predictions: dict[tuple[int, str, int], float] = {}

    for seed in OOF_SEEDS:
        splitter = StratifiedKFold(n_splits=5, shuffle=True, random_state=seed)
        for train, test in splitter.split(x, buckets):
            for name in names:
                fitted = model(name, seed).fit(x[train], y[train])
                for index, prediction in zip(
                    test, fitted.predict(x[test]), strict=True
                ):
                    predictions[(int(index), name, seed)] = float(prediction)

    lbo_predictions: dict[tuple[int, str], float] = {}
    for bucket in sorted(set(buckets)):
        test = np.flatnonzero(buckets == bucket)
        train = np.flatnonzero(buckets != bucket)
        for name in names:
            fitted = model(name, LBO_SEED).fit(x[train], y[train])
            for index, prediction in zip(test, fitted.predict(x[test]), strict=True):
                lbo_predictions[(int(index), name)] = float(prediction)

    output: list[dict[str, Any]] = []
    consistent_sign_count = 0
    lbo_agreement_count = 0
    for index, row in enumerate(rows):
        residuals = {
            (name, seed): float(y[index] - predictions[(index, name, seed)])
            for name in names
            for seed in OOF_SEEDS
        }
        signs = [int(np.sign(value)) for value in residuals.values()]
        sign_consistent = all(sign > 0 for sign in signs) or all(
            sign < 0 for sign in signs
        )
        if sign_consistent:
            consistent_sign_count += 1
        median_residual = float(np.median(list(residuals.values())))
        lbo_residuals = {
            name: float(y[index] - lbo_predictions[(index, name)]) for name in names
        }
        lbo_agrees = sign_consistent and all(
            int(np.sign(value)) == int(np.sign(median_residual))
            for value in lbo_residuals.values()
        )
        if lbo_agrees:
            lbo_agreement_count += 1
        if not sign_consistent or abs(median_residual) < 0.1:
            continue
        output.append(
            {
                "poly_id": str(row["poly_id"]),
                "arm": arm_for(row),
                "bucket": str(buckets[index]),
                "sys": float(y[index]),
                "residual_sign": "positive" if median_residual > 0 else "negative",
                "median_oof_residual": median_residual,
                "oof_residual_spread": float(
                    max(residuals.values()) - min(residuals.values())
                ),
                "ridge_seed_20260711_residual": residuals[("ridge", OOF_SEEDS[0])],
                "ridge_seed_20260712_residual": residuals[("ridge", OOF_SEEDS[1])],
                "hist_seed_20260711_residual": residuals[
                    ("hist_gradient_boosting", OOF_SEEDS[0])
                ],
                "hist_seed_20260712_residual": residuals[
                    ("hist_gradient_boosting", OOF_SEEDS[1])
                ],
                "ridge_leave_bucket_out_residual": lbo_residuals["ridge"],
                "hist_leave_bucket_out_residual": lbo_residuals[
                    "hist_gradient_boosting"
                ],
                "leave_bucket_out_sign_agrees": lbo_agrees,
            }
        )
    output.sort(key=lambda row: (row["arm"], row["bucket"], row["poly_id"]))
    return output, {
        "oof_sign_consistent_rows": consistent_sign_count,
        "oof_sign_consistent_and_abs_median_ge_0_1_rows": len(output),
        "oof_sign_consistent_and_leave_bucket_out_agreement_rows": lbo_agreement_count,
        "candidate_counts": Counter(row["arm"] for row in output),
        "candidate_lbo_agreement_counts": Counter(
            row["arm"] for row in output if row["leave_bucket_out_sign_agrees"]
        ),
    }


def build_panel(
    pairs: list[dict[str, Any]], residuals: list[dict[str, Any]]
) -> list[dict[str, Any]]:
    panel: list[dict[str, Any]] = []
    used_buckets: set[str] = set()
    primary = f"selected_q{int(PRIMARY_QUANTILE * 100):02d}"
    for arm in ("product_exact_summary", "generic_caliper"):
        eligible = sorted(
            (row for row in pairs if row.get(primary) and row["arm"] == arm),
            key=lambda row: (-float(row["sys_gap"]), str(row["bucket"])),
        )
        selected = 0
        for row in eligible:
            if row["bucket"] in used_buckets:
                continue
            panel.append(
                {
                    "panel_role": "discordant_pair",
                    "arm": arm,
                    "bucket": row["bucket"],
                    "poly_id_a": row["poly_id_low"],
                    "poly_id_b": row["poly_id_high"],
                    "sys_a": row["sys_low"],
                    "sys_b": row["sys_high"],
                    "score": row["sys_gap"],
                    "selection_note": "top post-target sys gap among target-free analysis-chosen q25 matches; at most one pair per bucket",
                }
            )
            used_buckets.add(str(row["bucket"]))
            selected += 1
            if selected == 2:
                break

    used_buckets.clear()
    selected_ids: set[str] = set()
    consistent = [row for row in residuals if row["leave_bucket_out_sign_agrees"]]
    # Guarantee coverage of both sources and both residual signs when available.
    for arm in ("product_exact_summary", "generic_caliper"):
        for sign in ("positive", "negative"):
            eligible = sorted(
                (
                    row
                    for row in consistent
                    if row["arm"] == arm and row["residual_sign"] == sign
                ),
                key=lambda row: -abs(float(row["median_oof_residual"])),
            )
            if not eligible:
                continue
            row = eligible[0]
            panel.append(
                {
                    "panel_role": "residual_exemplar",
                    "arm": arm,
                    "bucket": row["bucket"],
                    "poly_id_a": row["poly_id"],
                    "poly_id_b": "",
                    "sys_a": row["sys"],
                    "sys_b": "",
                    "score": row["median_oof_residual"],
                    "selection_note": "largest model-specific OOF-sign-consistent residual in arm/sign cell with leave-bucket-out sign agreement",
                }
            )
            selected_ids.add(str(row["poly_id"]))
            used_buckets.add(str(row["bucket"]))

    for row in sorted(
        consistent, key=lambda item: -abs(float(item["median_oof_residual"]))
    ):
        if (
            len([item for item in panel if item["panel_role"] == "residual_exemplar"])
            >= 6
        ):
            break
        if row["poly_id"] in selected_ids or row["bucket"] in used_buckets:
            continue
        panel.append(
            {
                "panel_role": "residual_exemplar",
                "arm": row["arm"],
                "bucket": row["bucket"],
                "poly_id_a": row["poly_id"],
                "poly_id_b": "",
                "sys_a": row["sys"],
                "sys_b": "",
                "score": row["median_oof_residual"],
                "selection_note": "largest remaining model-specific OOF-sign-consistent residual with a new bucket and leave-bucket-out sign agreement",
            }
        )
        selected_ids.add(str(row["poly_id"]))
        used_buckets.add(str(row["bucket"]))
    return panel


def main() -> None:
    args = parse_args()
    table_path = args.tables_dir / "polytope-table.jsonl"
    provenance_path = args.tables_dir / "polytope-provenance-table.jsonl"
    table_hash = sha256(table_path)
    provenance_hash = sha256(provenance_path)
    # Exact bytes are a staleness cue, not a compatibility gate. The loader and
    # checks below still enforce the table/provenance schema and population.
    if table_hash != EXPECTED_TABLE_SHA256 or provenance_hash != EXPECTED_PROVENANCE_SHA256:
        print(
            "warning: input bytes differ from the retained P2 table; continuing "
            "with semantic checks. Reassess the packet interpretation before "
            "treating this run as equivalent.",
            file=sys.stderr,
        )

    rows, provenance_rows = load_trusted_random_tables(args.tables_dir)
    if len(rows) != EXPECTED_ROWS or len(provenance_rows) != EXPECTED_ROWS:
        raise SystemExit(f"expected {EXPECTED_ROWS} trusted rows and provenance rows")
    poly_ids = [str(row["poly_id"]) for row in rows]
    if len(set(poly_ids)) != len(poly_ids):
        raise SystemExit("duplicate poly_id in trusted table")

    provenance = provenance_by_poly_id(provenance_rows)
    features = active_invariant_numeric_feature_names(rows)
    ridge_names = [name for name in features if name.startswith("ridge_symp_area_")]
    combinatorial_names = [name for name in features if name not in ridge_names]
    buckets = np.asarray([source_bucket(row, provenance) for row in rows])

    pair_rows, pair_metadata = build_pair_candidates(
        rows, buckets, combinatorial_names, ridge_names
    )
    residual_rows, residual_metadata = build_residual_candidates(
        rows, features, buckets
    )
    panel = build_panel(pair_rows, residual_rows)
    primary_pairs = [row for row in pair_rows if row.get("selected_q25")]
    pair_sensitivity_results: dict[str, dict[str, dict[str, float | int | None]]] = {}
    for arm in ("product_exact_summary", "generic_caliper"):
        pair_sensitivity_results[arm] = {}
        for quantile in SENSITIVITY_QUANTILES:
            label = f"q{int(quantile * 100):02d}"
            selected = [
                row
                for row in pair_rows
                if row["arm"] == arm and row.get(f"selected_{label}")
            ]
            pair_sensitivity_results[arm][label] = {
                "selected_pairs": len(selected),
                "sys_gap_ge_0_1_pairs": sum(
                    float(row["sys_gap"]) >= 0.1 for row in selected
                ),
                "max_sys_gap": max(
                    (float(row["sys_gap"]) for row in selected), default=None
                ),
            }

    args.out_dir.mkdir(parents=True, exist_ok=True)
    pair_fields = [
        "arm",
        "bucket",
        "poly_id_low",
        "poly_id_high",
        "sys_low",
        "sys_high",
        "sys_gap",
        "metric_a_distance",
        "metric_b_distance",
        "combinatorial_distance",
        "ridge_compact_distance",
        "ridge_full_pca_distance",
        "ridge_mean_low",
        "ridge_mean_high",
        "selected_q10",
        "selected_q25",
        "selected_q50",
    ]
    residual_fields = [
        "poly_id",
        "arm",
        "bucket",
        "sys",
        "residual_sign",
        "median_oof_residual",
        "oof_residual_spread",
        "ridge_seed_20260711_residual",
        "ridge_seed_20260712_residual",
        "hist_seed_20260711_residual",
        "hist_seed_20260712_residual",
        "ridge_leave_bucket_out_residual",
        "hist_leave_bucket_out_residual",
        "leave_bucket_out_sign_agrees",
    ]
    panel_fields = [
        "panel_role",
        "arm",
        "bucket",
        "poly_id_a",
        "poly_id_b",
        "sys_a",
        "sys_b",
        "score",
        "selection_note",
    ]
    write_tsv(args.out_dir / "discordant-pairs.tsv", pair_rows, pair_fields)
    write_tsv(args.out_dir / "residual-candidates.tsv", residual_rows, residual_fields)
    write_tsv(args.out_dir / "inspection-panel.tsv", panel, panel_fields)

    summary = {
        "question": "Which retained rows are useful post-target inspection candidates where the current invariant summaries or two ordinary models leave target variation unexplained?",
        "epistemic_status": "post-target exploratory inspection candidates (G only); analysis-chosen rules; no demonstrated feature-map failure, proposer, mechanism, causal, transfer, or theorem claim",
        "inputs": {
            "table_sha256": table_hash,
            "provenance_sha256": provenance_hash,
            "trusted_rows": len(rows),
            "capacity_source_counts": Counter(
                str(row["capacity_source"]) for row in rows
            ),
            "bucket_counts": Counter(buckets),
            "active_features": len(features),
            "combinatorial_features": len(combinatorial_names),
            "ridge_features": len(ridge_names),
        },
        "pair_contract": {
            "target_use": "pair construction and calipers are target-free; sys is used only for post-formation ranking",
            "product_arm": "exact bucket and exact current combinatorial summaries; same mutual neighbor under compact-ridge and full-ridge-PCA maps",
            "generic_arm": "exact facet bucket; same mutual neighbor under full-summary+compact-ridge and PCA-summary+PCA-ridge maps",
            "sensitivity_quantiles": SENSITIVITY_QUANTILES,
            "primary_quantile": PRIMARY_QUANTILE,
            "caliper_status": "analysis-chosen after an initial scout and fixed only for this bounded inspection; not predeclared confirmatory evidence",
            **pair_metadata,
            "sensitivity_results": pair_sensitivity_results,
            "primary_selected_counts": Counter(row["arm"] for row in primary_pairs),
            "primary_sys_gap_ge_0_1_counts": Counter(
                row["arm"] for row in primary_pairs if float(row["sys_gap"]) >= 0.1
            ),
            "primary_max_sys_gap_by_arm": {
                arm: max(
                    (
                        float(row["sys_gap"])
                        for row in primary_pairs
                        if row["arm"] == arm
                    ),
                    default=None,
                )
                for arm in ("product_exact_summary", "generic_caliper")
            },
        },
        "residual_contract": {
            "models": ["standardized ridge alpha=10", "histogram gradient boosting"],
            "oof": "model-specific consistency check: two fixed seeds; five-fold cross-fitting stratified by exact source bucket; all four residual signs must agree",
            "leave_bucket_out": "weak extrapolation stress only: both model residual signs must agree with OOF sign; not independent validation, omitted-feature evidence, or generator transfer",
            "candidate_threshold": "absolute median OOF residual >= 0.1",
            **residual_metadata,
            "candidate_abs_median_ge_0_2_counts": Counter(
                row["arm"]
                for row in residual_rows
                if abs(float(row["median_oof_residual"])) >= 0.2
            ),
            "candidate_abs_median_ge_0_2_and_lbo_agreement_counts": Counter(
                row["arm"]
                for row in residual_rows
                if abs(float(row["median_oof_residual"])) >= 0.2
                and row["leave_bucket_out_sign_agrees"]
            ),
        },
        "inspection_panel_rows": len(panel),
        "max_sys": max(float(row["sys"]) for row in rows),
        "sys_gt_1_rows": sum(float(row["sys"]) > 1 for row in rows),
    }
    write_json(args.out_dir / "summary.json", summary)
    (args.out_dir / "command.txt").write_text(
        "OPENBLAS_NUM_THREADS=1 OMP_NUM_THREADS=1 MKL_NUM_THREADS=1 \\\n"
        "  uv run --script experiments/sys-datascience/methods/residual-exemplar-seeds/analyze.py \\\n"
        f"  --tables-dir {args.tables_dir}\n"
    )
    print(json.dumps(summary, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
