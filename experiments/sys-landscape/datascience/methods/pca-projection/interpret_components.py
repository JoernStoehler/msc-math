#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = [
#   "numpy",
# ]
# ///
"""Interpret PCA components for the retained sys-landscape dataset."""

from __future__ import annotations

import argparse
import json
from collections import defaultdict
from pathlib import Path

import numpy as np

import analyze
import interpret_pc2_high


METHOD_DIR = Path(__file__).resolve().parent
DEFAULT_DATASET = METHOD_DIR.parents[1] / "dataset"
DEFAULT_SUMMARY = METHOD_DIR / "pca-summary.json"
DEFAULT_OUTPUT = METHOD_DIR / "component-interpretation.json"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--dataset", type=Path, default=DEFAULT_DATASET)
    parser.add_argument("--summary", type=Path, default=DEFAULT_SUMMARY)
    parser.add_argument("--output", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--components", type=int, default=6)
    return parser.parse_args()


def rankdata(values: np.ndarray) -> np.ndarray:
    order = np.argsort(values, kind="mergesort")
    ranks = np.empty(values.shape[0], dtype=float)
    sorted_values = values[order]
    start = 0
    while start < values.shape[0]:
        end = start + 1
        while end < values.shape[0] and sorted_values[end] == sorted_values[start]:
            end += 1
        average_rank = (start + end - 1) / 2.0
        ranks[order[start:end]] = average_rank
        start = end
    return ranks


def correlation(x: np.ndarray, y: np.ndarray) -> float | None:
    if x.shape[0] < 2:
        return None
    x_centered = x - x.mean()
    y_centered = y - y.mean()
    denominator = float(np.linalg.norm(x_centered) * np.linalg.norm(y_centered))
    if denominator == 0.0:
        return None
    return float(np.dot(x_centered, y_centered) / denominator)


def spearman(x: np.ndarray, y: np.ndarray) -> float | None:
    return correlation(rankdata(x), rankdata(y))


def eta_squared_by_group(values: np.ndarray, groups: np.ndarray) -> float | None:
    total = float(np.sum((values - values.mean()) ** 2))
    if total == 0.0:
        return None
    between = 0.0
    for group in sorted(set(groups.tolist())):
        mask = groups == group
        between += float(mask.sum()) * float((values[mask].mean() - values.mean()) ** 2)
    return between / total


def feature_family(column: str) -> str:
    for prefix in (
        "allpair_",
        "edge_",
        "facet_",
        "geom_",
        "ridge_",
        "transition_",
        "vertex_",
    ):
        if column.startswith(prefix):
            return prefix.rstrip("_")
    if column in {"facet_count", "dual_vertex_count", "ridge_count", "edge_count"}:
        return "combinatorics"
    if column == "volume":
        return "volume"
    return "other"


def loading_family_contributions(features: list[str], weights: np.ndarray) -> list[dict]:
    totals: dict[str, float] = defaultdict(float)
    for column, weight in zip(features, weights, strict=True):
        totals[feature_family(column)] += float(weight * weight)
    total = sum(totals.values())
    return [
        {
            "family": family,
            "squared_loading_sum": value,
            "fraction_of_component_squared_loading": value / total if total > 0 else None,
        }
        for family, value in sorted(totals.items(), key=lambda item: item[1], reverse=True)
    ]


def source_score_summaries(scores: np.ndarray, datasets: np.ndarray) -> dict[str, list[dict]]:
    out: dict[str, list[dict]] = {}
    for component_index in range(scores.shape[1]):
        rows = []
        pc_scores = scores[:, component_index]
        for dataset_name in sorted(set(datasets.tolist())):
            mask = datasets == dataset_name
            values = pc_scores[mask]
            rows.append(
                {
                    "dataset": dataset_name,
                    "row_count": int(mask.sum()),
                    "mean": float(values.mean()),
                    "std": float(values.std()),
                    "p05": float(np.quantile(values, 0.05)),
                    "median": float(np.quantile(values, 0.50)),
                    "p95": float(np.quantile(values, 0.95)),
                }
            )
        out[f"pc{component_index + 1}"] = rows
    return out


def pc_sys_relations(scores: np.ndarray, sys_values: np.ndarray, datasets: np.ndarray) -> dict[str, dict]:
    out: dict[str, dict] = {}
    scopes = [("all_retained_rows", np.ones(scores.shape[0], dtype=bool))]
    scopes.extend((name, datasets == name) for name in sorted(set(datasets.tolist())))
    for component_index in range(scores.shape[1]):
        pc_scores = scores[:, component_index]
        component = {}
        for scope_name, mask in scopes:
            component[scope_name] = {
                "row_count": int(mask.sum()),
                "pearson_pc_sys": correlation(pc_scores[mask], sys_values[mask]),
                "spearman_pc_sys": spearman(pc_scores[mask], sys_values[mask]),
            }
        out[f"pc{component_index + 1}"] = component
    return out


def source_local_region_audit(
    scores: np.ndarray,
    sys_values: np.ndarray,
    datasets: np.ndarray,
) -> list[dict]:
    out = []
    for dataset_name in sorted(set(datasets.tolist())):
        source_mask = datasets == dataset_name
        source_count = int(source_mask.sum())
        if source_count < 50:
            continue
        source_sys = sys_values[source_mask]
        source_top_threshold = analyze.quantile_threshold(source_sys, 0.99, "high")
        source_top = source_mask & (sys_values >= source_top_threshold)
        source_top_count = int(source_top.sum())
        for component_index in range(scores.shape[1]):
            pc_scores = scores[:, component_index]
            source_scores = pc_scores[source_mask]
            for side, quantile, name_suffix in (
                ("high", 0.95, "high_top_5_percent"),
                ("low", 0.05, "low_top_5_percent"),
            ):
                threshold = analyze.quantile_threshold(source_scores, quantile, side)
                if side == "high":
                    selected = source_mask & (pc_scores >= threshold)
                else:
                    selected = source_mask & (pc_scores <= threshold)
                selected_count = int(selected.sum())
                captured = int(np.logical_and(selected, source_top).sum())
                expected = selected_count * source_top_count / source_count
                out.append(
                    {
                        "dataset": dataset_name,
                        "region": f"pc{component_index + 1}_{name_suffix}",
                        "source_row_count": source_count,
                        "selected_row_count": selected_count,
                        "source_top_1_percent_threshold": float(source_top_threshold),
                        "source_top_1_percent_row_count": source_top_count,
                        "source_top_1_percent_captured": captured,
                        "expected_source_top_1_percent_capture_random": float(expected),
                        "source_top_1_percent_capture_enrichment": (
                            captured / expected if expected > 0 else None
                        ),
                        "hypergeometric_p_value_ge_observed_capture": analyze.hypergeometric_tail(
                            source_count,
                            source_top_count,
                            selected_count,
                            captured,
                        ),
                    }
                )
    return out


def check_recomputed_pca_matches_summary(
    summary: dict,
    features: list[str],
    components: np.ndarray,
    explained: np.ndarray,
) -> None:
    expected_component_count = int(summary["pca"]["component_count"])
    if components.shape[0] != expected_component_count:
        raise SystemExit(
            "Recomputed PCA component count does not match pca-summary.json; "
            "rerun analyze.py before interpreting components."
        )

    summary_explained = np.array(summary["pca"]["explained_variance_ratio"], dtype=float)
    if summary_explained.shape != explained.shape or not np.allclose(
        summary_explained,
        explained,
        rtol=1e-12,
        atol=1e-12,
    ):
        raise SystemExit(
            "Recomputed PCA explained variance does not match pca-summary.json; "
            "rerun analyze.py before interpreting components."
        )

    recomputed_top_loadings = analyze.top_loadings(features, components)
    if recomputed_top_loadings != summary["pca"]["top_loadings"]:
        raise SystemExit(
            "Recomputed PCA top loadings do not match pca-summary.json; "
            "rerun analyze.py before interpreting components."
        )


def main() -> None:
    args = parse_args()
    summary = json.loads(args.summary.read_text())
    poly_rows = analyze.read_jsonl(args.dataset / "polytope-table.jsonl")
    observation_rows = analyze.read_jsonl(args.dataset / "observation-table.jsonl")
    observation_by_poly_id = {row["poly_id"]: row for row in observation_rows}
    sys_values = np.array([float(row["sys"]) for row in poly_rows], dtype=float)

    interpret_pc2_high.check_summary_dataset(
        summary,
        interpret_pc2_high.current_dataset_fingerprint(
            args.dataset,
            poly_rows,
            observation_rows,
            sys_values,
        ),
    )

    features, excluded = analyze.choose_features(poly_rows)
    if features != summary["validity_guard"]["included_features"]:
        raise SystemExit("Feature policy no longer matches pca-summary.json; rerun analyze.py first.")
    if excluded != summary["validity_guard"]["excluded_inputs"]:
        raise SystemExit("Excluded-input policy no longer matches pca-summary.json; rerun analyze.py first.")

    z = analyze.standardized_matrix(poly_rows, features)
    scores, components, explained = analyze.fit_pca(z, args.components)
    check_recomputed_pca_matches_summary(summary, features, components, explained)
    datasets = np.array(
        [str(observation_by_poly_id[row["poly_id"]].get("dataset")) for row in poly_rows],
        dtype=object,
    )

    component_rows = []
    for component_index in range(scores.shape[1]):
        pc_name = f"pc{component_index + 1}"
        component_rows.append(
            {
                "component": pc_name,
                "explained_variance_ratio": float(explained[component_index]),
                "source_eta_squared": eta_squared_by_group(scores[:, component_index], datasets),
                "loading_family_contributions": loading_family_contributions(
                    features,
                    components[component_index],
                ),
                "top_loadings": summary["pca"]["top_loadings"][pc_name],
            }
        )

    output = {
        "method": "pca-projection-component-interpretation",
        "dataset": summary["dataset"],
        "inputs": {
            "dataset_path": str(args.dataset),
            "pca_summary": str(args.summary),
            "pca_scores": "Recomputed from the retained dataset with analyze.py helpers.",
            "sys_use": "Used after PCA fitting to interpret score association only.",
        },
        "component_interpretation_metrics": component_rows,
        "source_score_summaries": source_score_summaries(scores, datasets),
        "pc_sys_relations": pc_sys_relations(scores, sys_values, datasets),
        "source_local_region_audit": source_local_region_audit(scores, sys_values, datasets),
        "interpretation_limits": [
            "Source labels are not PCA inputs; source_eta_squared measures post-fit alignment with source families.",
            "High source_eta_squared does not prove source causation; it says source families occupy different regions in the fitted PCA coordinates.",
            "PC-sys correlations are descriptive audits on already evaluated retained rows, not candidate-proposer rules.",
        ],
    }

    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(output, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
