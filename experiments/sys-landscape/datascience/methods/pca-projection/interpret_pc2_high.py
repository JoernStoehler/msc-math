#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = [
#   "numpy",
# ]
# ///
"""Interpret the PC2-high enrichment found by the PCA projection audit."""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path

import numpy as np

import analyze


METHOD_DIR = Path(__file__).resolve().parent
DEFAULT_DATASET = METHOD_DIR.parents[1] / "dataset"
DEFAULT_SUMMARY = METHOD_DIR / "pca-summary.json"
DEFAULT_OUTPUT = METHOD_DIR / "pc2-high-audit.json"

AUDIT_FIELDS = (
    "dataset",
    "family",
    "role",
    "search_space",
    "optimizer",
    "backend",
)
THRESHOLD_FRACTIONS = (0.005, 0.01, 0.02, 0.05, 0.10, 0.20)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--dataset", type=Path, default=DEFAULT_DATASET)
    parser.add_argument("--summary", type=Path, default=DEFAULT_SUMMARY)
    parser.add_argument("--output", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument("--components", type=int, default=6)
    return parser.parse_args()


def quantile_high(values: np.ndarray, fraction: float) -> float:
    return analyze.quantile_threshold(values, 1.0 - fraction, "high")


def counter_for_field(
    observation_by_poly_id: dict[str, dict],
    poly_rows: list[dict],
    mask: np.ndarray,
    field: str,
) -> dict[str, int]:
    counts: Counter[str] = Counter()
    for index, row in enumerate(poly_rows):
        if bool(mask[index]):
            counts[str(observation_by_poly_id[row["poly_id"]].get(field))] += 1
    return dict(sorted(counts.items()))


def composition(
    observation_by_poly_id: dict[str, dict],
    poly_rows: list[dict],
    masks: dict[str, np.ndarray],
) -> dict[str, dict[str, dict[str, int]]]:
    out: dict[str, dict[str, dict[str, int]]] = {}
    for field in AUDIT_FIELDS:
        out[field] = {}
        for name, mask in masks.items():
            out[field][name] = counter_for_field(observation_by_poly_id, poly_rows, mask, field)
    return out


def source_strata(
    observation_by_poly_id: dict[str, dict],
    poly_rows: list[dict],
    sys_values: np.ndarray,
    pc2_high: np.ndarray,
    global_top_1_percent: np.ndarray,
) -> list[dict]:
    datasets = sorted({str(row.get("dataset")) for row in observation_by_poly_id.values()})
    out = []
    for dataset_name in datasets:
        source_mask = np.array(
            [
                str(observation_by_poly_id[row["poly_id"]].get("dataset")) == dataset_name
                for row in poly_rows
            ],
            dtype=bool,
        )
        source_sys = sys_values[source_mask]
        source_top_threshold = quantile_high(source_sys, 0.01)
        source_top_1_percent = source_mask & (sys_values >= source_top_threshold)
        source_count = int(source_mask.sum())
        pc2_source_count = int(np.logical_and(source_mask, pc2_high).sum())
        global_top_source_count = int(np.logical_and(source_mask, global_top_1_percent).sum())
        global_top_captured = int(np.logical_and.reduce((source_mask, pc2_high, global_top_1_percent)).sum())
        source_top_count = int(source_top_1_percent.sum())
        source_top_captured = int(np.logical_and(pc2_high, source_top_1_percent).sum())
        expected_global = pc2_source_count * global_top_source_count / source_count
        expected_source = pc2_source_count * source_top_count / source_count
        out.append(
            {
                "dataset": dataset_name,
                "row_count": source_count,
                "pc2_high_row_count": pc2_source_count,
                "pc2_high_fraction": pc2_source_count / source_count,
                "global_top_1_percent_row_count": global_top_source_count,
                "global_top_1_percent_captured_by_pc2_high": global_top_captured,
                "expected_global_top_1_percent_capture_within_source_random": expected_global,
                "global_top_1_percent_capture_enrichment_within_source": (
                    global_top_captured / expected_global if expected_global > 0 else None
                ),
                "global_top_1_percent_capture_hypergeometric_p_value_within_source": (
                    analyze.hypergeometric_tail(
                        source_count,
                        global_top_source_count,
                        pc2_source_count,
                        global_top_captured,
                    )
                    if pc2_source_count > 0
                    else None
                ),
                "source_top_1_percent_threshold": source_top_threshold,
                "source_top_1_percent_row_count": source_top_count,
                "source_top_1_percent_captured_by_pc2_high": source_top_captured,
                "expected_source_top_1_percent_capture_within_source_random": expected_source,
                "source_top_1_percent_capture_enrichment_within_source": (
                    source_top_captured / expected_source if expected_source > 0 else None
                ),
                "source_top_1_percent_capture_hypergeometric_p_value_within_source": (
                    analyze.hypergeometric_tail(
                        source_count,
                        source_top_count,
                        pc2_source_count,
                        source_top_captured,
                    )
                    if pc2_source_count > 0
                    else None
                ),
            }
        )
    return out


def threshold_stability(
    sys_values: np.ndarray,
    pc2_scores: np.ndarray,
    scope_mask: np.ndarray,
    top_mask: np.ndarray,
    population_label: str,
) -> list[dict]:
    out = []
    scope_count = int(scope_mask.sum())
    top_count = int(top_mask.sum())
    for fraction in THRESHOLD_FRACTIONS:
        threshold = quantile_high(pc2_scores[scope_mask], fraction)
        selected = scope_mask & (pc2_scores >= threshold)
        selected_count = int(selected.sum())
        captured = int(np.logical_and(selected, top_mask).sum())
        expected = selected_count * top_count / scope_count
        out.append(
            {
                "population": population_label,
                "pc2_high_fraction": fraction,
                "pc2_threshold": threshold,
                "selected_row_count": selected_count,
                "top_1_percent_row_count": top_count,
                "top_1_percent_captured": captured,
                "expected_top_1_percent_capture_random": expected,
                "top_1_percent_capture_enrichment": captured / expected if expected > 0 else None,
                "hypergeometric_p_value_ge_observed_capture": analyze.hypergeometric_tail(
                    scope_count,
                    top_count,
                    selected_count,
                    captured,
                ),
            }
        )
    return out


def pc2_deciles(
    sys_values: np.ndarray,
    pc2_scores: np.ndarray,
    scope_mask: np.ndarray,
    top_mask: np.ndarray,
    population_label: str,
) -> list[dict]:
    indices = np.where(scope_mask)[0]
    ordered = indices[np.argsort(pc2_scores[indices])]
    out = []
    for decile_index in range(10):
        low = int(decile_index * len(ordered) / 10)
        high = int((decile_index + 1) * len(ordered) / 10)
        part = ordered[low:high]
        out.append(
            {
                "population": population_label,
                "pc2_decile_low_to_high": decile_index + 1,
                "row_count": int(part.shape[0]),
                "pc2_min": float(pc2_scores[part].min()),
                "pc2_max": float(pc2_scores[part].max()),
                "mean_sys": float(sys_values[part].mean()),
                "max_sys": float(sys_values[part].max()),
                "global_top_1_percent_row_count": int(top_mask[part].sum()),
            }
        )
    return out


def feature_means(
    summary: dict,
    poly_rows: list[dict],
    pc2_high: np.ndarray,
    pc2_high_top_1_percent: np.ndarray,
    gradient_ascent_products: np.ndarray,
) -> list[dict]:
    columns = [item["column"] for item in summary["pca"]["top_loadings"]["pc2"]]
    out = []
    for column in columns:
        values = np.array([float(row[column]) for row in poly_rows], dtype=float)
        gap_pc2_high = gradient_ascent_products & pc2_high
        gap_not_pc2_high = gradient_ascent_products & ~pc2_high
        out.append(
            {
                "column": column,
                "global_mean": float(values.mean()),
                "pc2_high_mean": float(values[pc2_high].mean()),
                "not_pc2_high_mean": float(values[~pc2_high].mean()),
                "pc2_high_global_top_1_percent_mean": float(values[pc2_high_top_1_percent].mean()),
                "gradient_ascent_products_pc2_high_mean": float(values[gap_pc2_high].mean()),
                "gradient_ascent_products_not_pc2_high_mean": float(values[gap_not_pc2_high].mean()),
            }
        )
    return out


def main() -> None:
    args = parse_args()
    summary = json.loads(args.summary.read_text())
    poly_rows = analyze.read_jsonl(args.dataset / "polytope-table.jsonl")
    observation_rows = analyze.read_jsonl(args.dataset / "observation-table.jsonl")
    observation_by_poly_id = {row["poly_id"]: row for row in observation_rows}

    features, excluded = analyze.choose_features(poly_rows)
    summary_features = summary["validity_guard"]["included_features"]
    if features != summary_features:
        raise SystemExit("Feature policy no longer matches pca-summary.json; rerun analyze.py first.")
    if excluded != summary["validity_guard"]["excluded_inputs"]:
        raise SystemExit("Excluded-input policy no longer matches pca-summary.json; rerun analyze.py first.")

    z = analyze.standardized_matrix(poly_rows, features)
    scores, _, _ = analyze.fit_pca(z, args.components)
    sys_values = np.array([float(row["sys"]) for row in poly_rows], dtype=float)
    pc2_scores = scores[:, 1]
    pc2_threshold = quantile_high(pc2_scores, 0.05)
    pc2_high = pc2_scores >= pc2_threshold
    global_top_threshold = analyze.quantile_threshold(sys_values, 0.99, "high")
    global_top_1_percent = sys_values >= global_top_threshold
    pc2_high_top_1_percent = pc2_high & global_top_1_percent

    datasets = np.array(
        [str(observation_by_poly_id[row["poly_id"]].get("dataset")) for row in poly_rows],
        dtype=object,
    )
    gradient_ascent_products = datasets == "gradient_ascent_products"
    gradient_ascent_product_top_threshold = quantile_high(
        sys_values[gradient_ascent_products],
        0.01,
    )
    gradient_ascent_product_top_1_percent = gradient_ascent_products & (
        sys_values >= gradient_ascent_product_top_threshold
    )

    candidate_regions = summary["audit"]["candidate_regions"]
    pc2_high_region = next(
        region for region in candidate_regions if region["name"] == "pc2_high_top_5_percent"
    )
    raw_p_value = pc2_high_region["hypergeometric_p_value_ge_observed_top_1_percent_capture"]

    masks = {
        "global": np.ones(len(poly_rows), dtype=bool),
        "global_top_1_percent": global_top_1_percent,
        "pc2_high_top_5_percent": pc2_high,
        "pc2_high_top_5_percent_and_global_top_1_percent": pc2_high_top_1_percent,
    }
    global_mask = np.ones(len(poly_rows), dtype=bool)

    audit = {
        "method": "pca-projection-pc2-high-interpretation",
        "dataset": summary["dataset"],
        "inputs": {
            "dataset_path": str(args.dataset),
            "pca_summary": str(args.summary),
            "pca_summary_consumed_for": [
                "included feature policy consistency check",
                "excluded input policy consistency check",
                "PC2 top-loading columns",
                "candidate-region raw p-value",
            ],
            "pca_scores": "Recomputed from the retained dataset with analyze.py helpers.",
        },
        "pc2_high_rule": {
            "score": "PC2 from the existing standardized PCA fit.",
            "region": "top 5% of retained rows by PC2 score",
            "pc2_threshold": pc2_threshold,
            "row_count": int(pc2_high.sum()),
            "row_fraction": float(pc2_high.mean()),
            "sys_use": "audit and interpretation only, after PCA fitting and region definition",
        },
        "global_top_1_percent_rule": {
            "sys_threshold": global_top_threshold,
            "row_count": int(global_top_1_percent.sum()),
        },
        "candidate_region_multiple_comparison_note": {
            "candidate_regions_audited_in_summary": len(candidate_regions),
            "pc2_high_raw_hypergeometric_p_value": raw_p_value,
            "bonferroni_over_summary_candidate_regions": min(1.0, raw_p_value * len(candidate_regions)),
            "limit": (
                "This adjusts only over the fixed regions in pca-summary.json, "
                "not over all choices made during exploratory interpretation."
            ),
        },
        "composition": composition(observation_by_poly_id, poly_rows, masks),
        "source_strata": source_strata(
            observation_by_poly_id,
            poly_rows,
            sys_values,
            pc2_high,
            global_top_1_percent,
        ),
        "threshold_stability": (
            threshold_stability(
                sys_values,
                pc2_scores,
                global_mask,
                global_top_1_percent,
                "all retained rows; global top-1% sys",
            )
            + threshold_stability(
                sys_values,
                pc2_scores,
                gradient_ascent_products,
                gradient_ascent_product_top_1_percent,
                "gradient_ascent_products only; source-local top-1% sys",
            )
        ),
        "pc2_deciles": (
            pc2_deciles(
                sys_values,
                pc2_scores,
                global_mask,
                global_top_1_percent,
                "all retained rows",
            )
            + pc2_deciles(
                sys_values,
                pc2_scores,
                gradient_ascent_products,
                global_top_1_percent,
                "gradient_ascent_products",
            )
            + pc2_deciles(
                sys_values,
                pc2_scores,
                datasets == "random_product_sample",
                global_top_1_percent,
                "random_product_sample",
            )
        ),
        "pc2_top_loading_feature_means": feature_means(
            summary,
            poly_rows,
            pc2_high,
            pc2_high_top_1_percent,
            gradient_ascent_products,
        ),
        "interpretation": {
            "evidence_for": [
                (
                    "PC2-high is mostly evidence for a product or near-degenerate "
                    "symplectic-geometry direction in the retained rows."
                ),
                (
                    "The global top-1% capture is entangled with source/provenance: "
                    "all captured top-1% rows are gradient_ascent_products rows."
                ),
                (
                    "Within gradient_ascent_products, high PC2 still enriches "
                    "source-local top-1% sys rows, so the signal is not explained "
                    "by selecting that source alone."
                ),
            ],
            "not_evidence_for": [
                "a current candidate-proposer for unevaluated rows",
                "a monotone rule that larger PC2 is always better",
                "a source-independent high-sys pattern across the retained dataset",
                "a validated sys > 1 row",
            ],
            "action_changed": (
                "Keep the PCA row as mixed supporting evidence with a future reopen trigger. "
                "Do not escalate it as a candidate-proposer. Split a product-family "
                "PCA-band proposer only if the thesis-success loop still needs a targeted "
                "product-family follow-up after higher-priority method rows are closed."
            ),
        },
    }

    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(audit, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
