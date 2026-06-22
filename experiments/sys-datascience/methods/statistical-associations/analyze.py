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
from random_only import TABLES_DIR, load_trusted_random_tables, numeric_feature_names, write_json  # noqa: E402


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
    if name.startswith(("allpair_", "ridge_abs_omega_", "ridge_symp_area_")):
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
                "status": "covered",
                "current_summaries": [
                    "all-pair absolute omega mean/std/min/max at volume one",
                    "zero fraction",
                    "ridge-restricted absolute omega summaries and small-value fractions",
                ],
            },
            {
                "node": "two-faces F_i cap F_j",
                "status": "partially covered",
                "current_summaries": [
                    "volume-one symplectic area mean/std/min/max over retained two-faces",
                    "sum",
                    "max share",
                    "zero and small-area fractions",
                ],
                "missing_summaries": [
                    "median and upper quantiles",
                    "top-k sums or tail shares",
                    "separate factor/product-aware versions if useful",
                ],
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
                "status": "mostly handled outside scalar screening or missing",
                "current_summaries": [
                    "source-family and product-bucket EDA elsewhere",
                ],
                "missing_summaries": [
                    "factor tests in this packet",
                    "accepted-attempt or rejection-difficulty metadata if retained later",
                ],
            },
        ],
        "covered_invariant_feature_families": [
            "basic size counts such as facet, vertex, edge, ridge, and dual-vertex counts",
            "simple combinatorial summaries such as degrees, incidence counts, ridge sizes, and facet adjacency",
            "volume-normalized Euclidean summaries of dual vertices, edge lengths, and facet volumes",
            "omega summaries over all facet pairs",
            "ridge-level symplectic-area summaries, including volume-normalized sum/mean/max and small-area fractions",
            "transition-graph summaries from facet intersections and omega signs",
            "orbit/capacity-result summaries for already evaluated rows",
        ],
        "bad_feature_families_not_counted_as_evidence_of_coverage": [
            "raw individual coordinates of a_k, because coordinate-level effects are not invariant under the relevant symmetries and are expected to be weak or uninterpretable",
            "raw flattened dual-vertex arrays, because they require an invariant featurization or an explicitly equivariant model before they answer what high sys corresponds to",
        ],
        "covered_elsewhere_not_as_univariate_numeric_covariates": [
            "source family random_sample versus random_product_sample is handled in random-tail-eda and the mean-difference bootstrap in this packet",
            "facet-count and product-bucket slice behavior is handled in random-tail-eda",
        ],
        "missing_or_partial_obvious_feature_families": [
            "categorical product bucket k x m as a univariate factor rather than only as EDA slices",
            "generator metadata such as accepted-attempt count or rejection difficulty, if retained in future tables",
            "additional invariant scalar summaries derived from a_k, such as height extrema, angular concentration beyond pairwise cosine summaries, nearest-neighbor statistics, and condition numbers beyond the current singular values",
            "symplectic area divided by vol^0.5 for all 2-faces, not only retained ridge-level summaries",
            "explicit product-structure or symmetry scores for deciding whether a generic row looks product-like",
            "local perturbation or sensitivity scalars, because those require non-gradient perturbation or local-behavior data outside the retained random/product table",
        ],
        "assessment": (
            "The current screen is useful for retained invariant scalar summaries, "
            "and it already includes several obvious omega and ridge symplectic-area "
            "families. It is not a complete feature-engineering closure from a_k. "
            "A stronger packet would enumerate candidate invariant feature families "
            "from the dual vertices, implement the missing high-value ones in the "
            "shared prepare stage, and only then rerun association screening and "
            "the other black-box methods."
        ),
    }


def main() -> None:
    args = parse_args()
    rows, _ = load_trusted_random_tables(args.tables_dir)
    y = np.array([float(row["sys"]) for row in rows], dtype=float)
    eligible_names = numeric_feature_names(rows, geometry_only=False)
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
            "non-scalar JSON columns",
            "columns present as numeric values in less than 98% of retained rows",
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
