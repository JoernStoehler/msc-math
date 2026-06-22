#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy", "scipy"]
# ///

"""Random-only univariate association screening."""

from __future__ import annotations

import argparse
from pathlib import Path
import random
import sys

import numpy as np
from scipy import stats

HERE = Path(__file__).resolve().parent
sys.path.append(str(HERE.parent / "_shared"))
from random_only import (  # noqa: E402
    TABLES_DIR,
    dataset_label,
    load_trusted_random_tables,
    numeric_feature_names,
    product_bucket,
    provenance_by_poly_id,
    write_json,
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--tables-dir", type=Path, default=TABLES_DIR)
    parser.add_argument("--out-dir", type=Path, default=HERE / "artifacts")
    parser.add_argument("--max-features", type=int, default=None)
    parser.add_argument("--permutations", type=int, default=200)
    return parser.parse_args()


def covariate_family(name: str) -> str:
    if name in {"facet_count", "dual_vertex_count", "vertex_count", "edge_count", "ridge_count"}:
        return "basic counts"
    if name.startswith(("vertex_", "edge_density", "ridge_size_", "facet_vertex_", "facet_neighbor_")):
        return "combinatorial summaries"
    if name.startswith(("geom_", "edge_length_", "facet_volume_")):
        return "Euclidean geometry summaries"
    if name.startswith(
        ("allpair_", "omega_matrix_", "omega_sign_", "ridge_abs_omega_", "ridge_symp_area_")
    ):
        return "symplectic/omega summaries"
    if name.startswith("transition_"):
        return "transition graph summaries"
    if name.startswith("orbit_"):
        return "orbit/capacity-result summaries"
    if name in {"is_simple", "simple_vertex_fraction"}:
        return "simplicity summaries"
    return "other scalar summaries"


def family_inventory(names: list[str]) -> dict[str, dict[str, object]]:
    inventory: dict[str, dict[str, object]] = {}
    for name in names:
        family = covariate_family(name)
        entry = inventory.setdefault(family, {"count": 0, "features": []})
        entry["count"] = int(entry["count"]) + 1
        entry["features"].append(name)
    return dict(sorted(inventory.items()))


def summary_stats(values: list[float]) -> dict[str, float | int]:
    array = np.array(values, dtype=float)
    return {
        "rows": len(values),
        "mean": float(np.mean(array)),
        "median": float(np.median(array)),
        "std": float(np.std(array)),
        "min": float(np.min(array)),
        "max": float(np.max(array)),
        "q90": float(np.quantile(array, 0.9)),
        "q99": float(np.quantile(array, 0.99)),
    }


def factor_test(groups: dict[str, list[float]]) -> dict[str, object]:
    nonempty = {label: values for label, values in sorted(groups.items()) if values}
    group_summaries = {
        label: summary_stats(values) for label, values in nonempty.items()
    }
    values_by_group = list(nonempty.values())
    all_values = [value for values in values_by_group for value in values]
    result: dict[str, object] = {
        "group_count": len(nonempty),
        "row_count": len(all_values),
        "groups": group_summaries,
        "max_minus_min_group_mean": None,
        "eta_squared": None,
        "anova_f": None,
        "anova_p": None,
        "kruskal_h": None,
        "kruskal_p": None,
        "test_status": "not_tested",
    }
    if len(nonempty) < 2 or len(all_values) == 0:
        result["test_status"] = "not_enough_nonempty_groups"
        return result
    group_means = [float(np.mean(values)) for values in values_by_group]
    result["max_minus_min_group_mean"] = float(max(group_means) - min(group_means))
    total_mean = float(np.mean(all_values))
    total_ss = sum((value - total_mean) ** 2 for value in all_values)
    between_ss = sum(
        len(values) * (float(np.mean(values)) - total_mean) ** 2
        for values in values_by_group
    )
    result["eta_squared"] = float(between_ss / total_ss) if total_ss > 0.0 else 0.0
    if total_ss == 0.0:
        result["test_status"] = "all_values_identical"
        return result
    if any(len(values) < 2 for values in values_by_group):
        result["test_status"] = "summarized_only_group_too_small_for_tests"
        return result
    try:
        anova = stats.f_oneway(*values_by_group)
        result["anova_f"] = float(anova.statistic)
        result["anova_p"] = float(anova.pvalue)
    except Exception as error:
        result["anova_error"] = str(error)
    try:
        kruskal = stats.kruskal(*values_by_group)
        result["kruskal_h"] = float(kruskal.statistic)
        result["kruskal_p"] = float(kruskal.pvalue)
    except Exception as error:
        result["kruskal_error"] = str(error)
    result["test_status"] = "tested"
    return result


def first_numeric_field(
    provenance_rows: list[dict[str, object]], field: str
) -> int | float | None:
    values = sorted(
        {
            row[field]
            for row in provenance_rows
            if isinstance(row.get(field), int | float)
        }
    )
    if len(values) == 1:
        return values[0]
    return None


def first_height_range(provenance_rows: list[dict[str, object]]) -> str | None:
    ranges = sorted(
        {
            (float(row["sample_h_min"]), float(row["sample_h_max"]))
            for row in provenance_rows
            if isinstance(row.get("sample_h_min"), int | float)
            and isinstance(row.get("sample_h_max"), int | float)
        }
    )
    if len(ranges) == 1:
        low, high = ranges[0]
        return f"{low:g}:{high:g}"
    if len(ranges) > 1:
        return "multi:" + ",".join(f"{low:g}:{high:g}" for low, high in ranges)
    return None


def source_factor_tests(
    rows: list[dict[str, object]], provenance_rows: list[dict[str, object]]
) -> dict[str, object]:
    provenance = provenance_by_poly_id(provenance_rows)
    factors: dict[str, dict[str, list[float]]] = {
        "capacity_source": {},
        "dataset_label": {},
        "facet_count": {},
        "dataset_label_by_facet_count": {},
        "product_bucket": {},
        "product_bounces": {},
        "sample_height_range": {},
    }
    for row in rows:
        poly_id = str(row["poly_id"])
        provenance_for_row = provenance.get(poly_id, [])
        sys_value = float(row["sys"])
        labels: dict[str, str | None] = {
            "capacity_source": str(row.get("capacity_source", "")) or None,
            "dataset_label": dataset_label(row, provenance_for_row),
            "facet_count": f"F{row.get('facet_count')}",
            "dataset_label_by_facet_count": (
                f"{dataset_label(row, provenance_for_row)}:F{row.get('facet_count')}"
            ),
            "product_bucket": None,
            "product_bounces": None,
            "sample_height_range": first_height_range(provenance_for_row),
        }
        if row.get("capacity_source") == "random_product_sample":
            labels["product_bucket"] = product_bucket(provenance_for_row)
            bounces = first_numeric_field(provenance_for_row, "product_bounces")
            labels["product_bounces"] = f"{int(bounces)}" if bounces is not None else None
        for factor_name, label in labels.items():
            if label is None:
                continue
            factors[factor_name].setdefault(label, []).append(sys_value)
    return {
        factor_name: factor_test(groups) for factor_name, groups in sorted(factors.items())
    }


def obvious_covariate_audit() -> dict[str, object]:
    return {
        "principle": (
            "The sufficient local input is the facet-dual list a_k; every scalar "
            "covariate in this packet, including sys(a), is a derived feature. "
            "Coverage must therefore be judged by first-layer derived objects "
            "from a_k, their normalizations/transforms, and their scalar "
            "summaries; not by counting retained table columns."
        ),
        "conceptual_flow": [
            "dual vertices a",
            "normalize to the volume-one representative when scale is not the question",
            "first-layer derived objects such as a itself, omega(a_i,a_j), two-faces, graphs, and orbit outputs",
            "prepare-stage engineered scalar features f_i(a) that seem useful",
            "univariate association screening of f_i(a) against sys(a)",
        ],
        "feature_owner": (
            "Current shared feature engineering lives in "
            "experiments/sys-datascience/prepare/features*.rs and is run by the "
            "sys-datascience-prepare binary."
        ),
        "normalization_convention": (
            "Prefer volume-one normalization before forming scale-sensitive "
            "geometric or symplectic summaries. This leaves Sp(4) plus translations "
            "as the main invariance issue instead of also carrying scalings."
        ),
        "first_layer_nodes": [
            {
                "node": "facet-dual list a_k",
                "status": "source object",
                "current_summaries": [
                    "volume-one norms",
                    "centroid norm",
                    "coordinate standard deviations",
                    "pairwise Euclidean distances",
                    "pairwise cosines",
                    "centered singular values",
                ],
                "notes": "Raw coordinates are not treated as explanatory evidence without invariantization or canonicalization.",
            },
            {
                "node": "pairwise symplectic form omega(a_i,a_j)",
                "status": "covered in the current prepare schema, pending retained-table rerun for new columns",
                "current_summaries": [
                    "all-pair absolute omega mean/std/min/max at volume one",
                    "zero fraction",
                    "ridge-restricted absolute omega summaries and small-value fractions",
                    "omega matrix singular-value summaries",
                    "permutation-invariant omega-sign out-degree summaries",
                    "absolute normalized-omega summaries in the chosen Euclidean representative",
                ],
                "notes": "Normalized-omega summaries are not general Sp(4) invariants and should be interpreted as Euclidean-representative diagnostics.",
            },
            {
                "node": "two-faces F_i cap F_j",
                "status": "covered in the current prepare schema, pending retained-table rerun for new columns",
                "current_summaries": [
                    "volume-one symplectic area mean/std/min/max over retained two-faces",
                    "sum",
                    "max share",
                    "median and upper quantiles",
                    "top-k tail share",
                    "zero and small-area fractions",
                ],
                "notes": "The current implementation records incidence-ordering diagnostics; area summaries are over successfully ordered two-faces.",
            },
            {
                "node": "face/vertex/edge/ridge incidence graph",
                "status": "covered",
                "current_summaries": [
                    "counts",
                    "degree/incidence/ridge-size summaries",
                    "facet-neighbor summaries",
                ],
            },
            {
                "node": "omega-sign transition graph",
                "status": "covered",
                "current_summaries": [
                    "density",
                    "bidirectional fraction",
                    "out-degree summaries",
                ],
            },
            {
                "node": "capacity/orbit outputs",
                "status": "covered for post-evaluation explanation only",
                "current_summaries": [
                    "sigma/orbit counts and fractions",
                    "selected-orbit norm and out-degree summaries",
                    "cycle omega summaries",
                    "KKT/best-orbit diagnostics",
                ],
            },
            {
                "node": "source/generator metadata",
                "status": "handled by EDA and categorical factor tests where provenance fields are available",
                "current_summaries": [
                    "source-family and product-bucket EDA",
                    "ANOVA/Kruskal-style categorical factor tests in this packet",
                ],
                "missing_summaries": [
                    "accepted-attempt or rejection-difficulty metadata for old retained artifacts",
                    "metadata-only prediction baselines",
                ],
            },
        ],
        "covered_invariant_feature_families": [
            "basic size counts such as facet, vertex, edge, ridge, and dual-vertex counts",
            "simple combinatorial summaries such as degrees, incidence counts, ridge sizes, and facet adjacency",
            "volume-normalized Euclidean summaries of dual vertices, edge lengths, and facet volumes",
            "omega summaries over all facet pairs, including matrix/sign/alignment summaries in the current prepare schema",
            "ridge-level symplectic-area summaries, including volume-normalized sum/mean/max, quantiles, top-k share, and small-area fractions in the current prepare schema",
            "transition-graph summaries from facet intersections and omega signs",
            "orbit/capacity-result summaries for already evaluated rows",
        ],
        "bad_feature_families_not_counted_as_evidence_of_coverage": [
            "raw individual coordinates of a_k, because coordinate-level effects are not invariant under the relevant symmetries and are expected to be weak or uninterpretable",
            "raw flattened dual-vertex arrays, because they require an invariant featurization or an explicitly equivariant model before they answer what high sys corresponds to",
        ],
        "covered_elsewhere_not_as_univariate_numeric_covariates": [
            "source family random_sample versus random_product_sample is handled in random-tail-eda, categorical factor tests, and the mean-difference bootstrap in this packet",
            "facet-count and product-bucket slice behavior is handled in random-tail-eda and categorical factor tests",
        ],
        "missing_or_partial_obvious_feature_families": [
            "generator metadata such as accepted-attempt count or rejection difficulty in old retained artifacts",
            "additional invariant scalar summaries derived from a_k, such as height extrema, angular concentration beyond pairwise cosine summaries, nearest-neighbor statistics, and condition numbers beyond the current singular values",
            "separate factor/product-aware versions of symplectic area summaries if source stratification shows value",
            "explicit product-structure or symmetry scores for deciding whether a generic row looks product-like",
            "local perturbation or sensitivity scalars, because those require non-gradient perturbation or local-behavior data outside the retained random/product table",
        ],
        "assessment": (
            "The current screen is useful for scalar summaries already present in "
            "the retained table it is run against. The branch-level coverage ledger "
            "now controls whether omega, sign, normalized-alignment, two-face-area, "
            "and source-metadata families are implemented, audited, or deferred. "
            "Retained-table reruns are still required before updated feature-family "
            "claims become thesis evidence."
        ),
    }


def main() -> None:
    args = parse_args()
    rows, provenance_rows = load_trusted_random_tables(args.tables_dir)
    y = np.array([float(row["sys"]) for row in rows], dtype=float)
    eligible_names = numeric_feature_names(rows, geometry_only=False)
    post_capacity_names = [
        name
        for name in numeric_feature_names(
            rows, geometry_only=False, include_post_capacity=True
        )
        if name.startswith("orbit_")
    ]
    names = eligible_names[: args.max_features] if args.max_features else eligible_names

    associations = []
    skipped_constant = []
    for name in names:
        x = np.array([float(row.get(name, 0.0)) for row in rows], dtype=float)
        if np.std(x) == 0.0:
            skipped_constant.append(name)
            continue
        pearson = stats.pearsonr(x, y)
        spearman = stats.spearmanr(x, y)
        associations.append(
            {
                "feature": name,
                "pearson_r": float(pearson.statistic),
                "pearson_p": float(pearson.pvalue),
                "spearman_rho": float(spearman.statistic),
                "spearman_p": float(spearman.pvalue),
            }
        )
    associations.sort(key=lambda row: abs(row["spearman_rho"]), reverse=True)

    observed_max = max((abs(row["spearman_rho"]) for row in associations), default=0.0)
    rng = random.Random(20260621)
    null_maxima = []
    selected_names = [row["feature"] for row in associations]
    x_rank_rows = []
    x_rank_norms = []
    for name in selected_names:
        x = np.array([float(row.get(name, 0.0)) for row in rows], dtype=float)
        ranks = stats.rankdata(x)
        centered = ranks - np.mean(ranks)
        x_rank_rows.append(centered)
        x_rank_norms.append(float(np.linalg.norm(centered)))
    x_rank_matrix = np.vstack(x_rank_rows) if x_rank_rows else np.empty((0, len(rows)))
    x_rank_norms_array = np.array(x_rank_norms, dtype=float)
    y_ranks = stats.rankdata(y)
    y_ranks_centered = y_ranks - np.mean(y_ranks)
    for _ in range(args.permutations):
        permuted = np.array(y_ranks_centered, copy=True)
        rng.shuffle(permuted)
        denominator = x_rank_norms_array * float(np.linalg.norm(permuted))
        with np.errstate(divide="ignore", invalid="ignore"):
            rho_values = (x_rank_matrix @ permuted) / denominator
        finite = rho_values[np.isfinite(rho_values)]
        null_maxima.append(float(np.max(np.abs(finite))) if len(finite) else 0.0)
    permutation_p = (
        (sum(1 for value in null_maxima if value >= observed_max) + 1)
        / (len(null_maxima) + 1)
        if null_maxima
        else None
    )

    random_values = [
        float(row["sys"]) for row in rows if row.get("capacity_source") == "random_sample"
    ]
    product_values = [
        float(row["sys"]) for row in rows if row.get("capacity_source") == "random_product_sample"
    ]
    mean_difference = float(np.mean(product_values) - np.mean(random_values))
    rng = random.Random(20260622)
    bootstrap = []
    for _ in range(1000):
        rs = [rng.choice(random_values) for _ in random_values]
        ps = [rng.choice(product_values) for _ in product_values]
        bootstrap.append(float(np.mean(ps) - np.mean(rs)))
    ci = [float(np.quantile(bootstrap, 0.025)), float(np.quantile(bootstrap, 0.975))]

    summary = {
        "row_count": len(rows),
        "screening_name": "univariate association screening",
        "target": "sys",
        "eligible_scalar_covariate_count": len(eligible_names),
        "selected_scalar_covariate_count": len(names),
        "feature_count": len(associations),
        "tested_scalar_covariates": [row["feature"] for row in associations],
        "skipped_constant_covariates": skipped_constant,
        "skipped_by_max_features": eligible_names[len(names) :],
        "post_capacity_covariates_available_but_not_tested": post_capacity_names,
        "source_factor_tests": source_factor_tests(rows, provenance_rows),
        "eligible_covariate_family_inventory": family_inventory(eligible_names),
        "tested_covariate_family_inventory": family_inventory(
            [row["feature"] for row in associations]
        ),
        "obvious_covariate_audit": obvious_covariate_audit(),
        "excluded_by_design": [
            "sys",
            "capacity",
            "volume",
            "capacity_iterations",
            "sigma_gap_cutoff",
            "orbit_result_iterations_log1p",
            "orbit_* post-capacity interpretation fields",
            "non-scalar JSON columns",
            "columns present as numeric values in less than 98% of retained rows",
            "provenance/source metadata as numeric scalar covariates; these are handled by source_factor_tests",
        ],
        "top_associations": associations[:25],
        "max_abs_spearman": observed_max,
        "permutation_family_max_p": permutation_p,
        "product_minus_random_mean_sys": mean_difference,
        "product_minus_random_mean_sys_bootstrap_ci_95": ci,
    }
    write_json(args.out_dir / "summary.json", summary)

    print("# univariate association screening")
    print()
    print(f"- rows: `{summary['row_count']}`")
    print(f"- eligible scalar covariates: `{summary['eligible_scalar_covariate_count']}`")
    print(f"- nonconstant covariates tested: `{summary['feature_count']}`")
    print(f"- max absolute Spearman rho: `{observed_max}`")
    print(f"- family-max permutation p: `{permutation_p}`")
    print(f"- product minus generic mean sys: `{mean_difference}`")
    print(f"Wrote `{args.out_dir}`")


if __name__ == "__main__":
    main()
