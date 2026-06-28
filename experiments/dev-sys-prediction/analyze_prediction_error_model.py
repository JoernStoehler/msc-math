#!/usr/bin/env python3
"""Summarize local-feature prediction error from prediction-cloud JSONL rows."""

from __future__ import annotations

import argparse
import csv
import json
import math
from collections import Counter, defaultdict
from pathlib import Path


HERE = Path(__file__).resolve().parent
DEFAULT_OUTPUT_DIR = HERE / "error-model"


def finite(value):
    return isinstance(value, (int, float)) and math.isfinite(value)


def abs_or_none(value):
    return abs(value) if finite(value) else None


def direction_class(label):
    if label.startswith("random_"):
        return "random"
    if label.startswith("angled_"):
        return "angled_policy"
    if label.startswith("negative_"):
        return "negative_policy"
    if label == "single_near_active_gradient":
        return "single_policy_gradient"
    if label in {
        "near_active_box_lp_normalized_direction",
        "near_active_maximin_direction",
    }:
        return "near_active_box_lp"
    return label


def error_source(row):
    status = row.get("status")
    if status != "ok":
        if status and "construction_failed" in status:
            return "construction_or_domain_failure"
        return "non_ok_failure"
    lin = row.get("decomposition_linearization_error")
    sigma = row.get("decomposition_sigma_set_error")
    if not finite(lin) or not finite(sigma):
        return "unknown_decomposition"
    if abs(sigma) > abs(lin):
        return "sigma_window"
    return "smooth_linearization"


def safe_ratio(numerator, denominator, eps=1e-12):
    if not finite(numerator) or not finite(denominator) or abs(denominator) <= eps:
        return None
    return numerator / denominator


def bool_number(value):
    if isinstance(value, bool):
        return 1 if value else 0
    return None


def feature_row(row):
    total_error = row.get("decomposition_total_prediction_error")
    observed_delta = row.get("observed_delta_sys")
    predicted_delta = row.get("candidate_window_predicted_delta_sys")
    predicted_gap_to_second = row.get("candidate_window_predicted_gap_to_second")
    return {
        "poly_id": row.get("poly_id"),
        "degeneracy_label": row.get("degeneracy_label"),
        "direction_label": row.get("direction_label"),
        "direction_class": direction_class(row.get("direction_label", "")),
        "step": row.get("step"),
        "status": row.get("status"),
        "source_class": error_source(row),
        "base_sys": row.get("base_sys"),
        "target_sys": row.get("target_sys"),
        "observed_delta_sys": observed_delta,
        "candidate_window_predicted_delta_sys": predicted_delta,
        "prediction_error": total_error,
        "abs_prediction_error": abs_or_none(total_error),
        "relative_error_to_observed_delta": safe_ratio(total_error, observed_delta),
        "abs_relative_error_to_observed_delta": abs_or_none(safe_ratio(total_error, observed_delta)),
        "relative_error_to_base_sys": safe_ratio(total_error, row.get("base_sys")),
        "abs_relative_error_to_base_sys": abs_or_none(safe_ratio(total_error, row.get("base_sys"))),
        "linearization_error": row.get("decomposition_linearization_error"),
        "abs_linearization_error": abs_or_none(row.get("decomposition_linearization_error")),
        "sigma_set_error": row.get("decomposition_sigma_set_error"),
        "abs_sigma_set_error": abs_or_none(row.get("decomposition_sigma_set_error")),
        "sum_residual": row.get("decomposition_sum_residual"),
        "base_near_active_count": row.get("base_near_active_count"),
        "base_candidate_window_count": row.get("base_candidate_window_count"),
        "target_orbit_iterations": row.get("target_orbit_iterations"),
        "candidate_window_witness_base_gap": row.get("candidate_window_witness_base_gap"),
        "candidate_window_witness_relative_action_gap": row.get(
            "candidate_window_witness_relative_action_gap"
        ),
        "candidate_window_witness_action": row.get("candidate_window_witness_action"),
        "candidate_window_witness_derivative": row.get("candidate_window_witness_derivative"),
        "candidate_window_predicted_gap_to_second": predicted_gap_to_second,
        "abs_candidate_window_predicted_gap_to_second": abs_or_none(predicted_gap_to_second),
        "candidate_window_second_orbit_index": row.get("candidate_window_second_orbit_index"),
        "candidate_window_second_sigma": " ".join(
            str(x) for x in row.get("candidate_window_second_sigma") or []
        ),
        "candidate_window_second_predicted_delta_sys": row.get(
            "candidate_window_second_predicted_delta_sys"
        ),
        "candidate_window_witness_sigma": " ".join(
            str(x) for x in row.get("candidate_window_witness_sigma") or []
        ),
        "target_near_active_count": row.get("target_near_active_count"),
        "target_best_sigma": " ".join(str(x) for x in row.get("target_best_sigma") or []),
        "target_best_sigma_in_base_near_active_set": row.get(
            "target_best_sigma_in_base_near_active_set"
        ),
        "target_best_sigma_in_base_candidate_window": row.get(
            "target_best_sigma_in_base_candidate_window"
        ),
        "target_best_sigma_matches_candidate_window_witness": row.get(
            "target_best_sigma_matches_candidate_window_witness"
        ),
        "target_best_sigma_in_base_near_active_set_number": bool_number(
            row.get("target_best_sigma_in_base_near_active_set")
        ),
        "target_best_sigma_in_base_candidate_window_number": bool_number(
            row.get("target_best_sigma_in_base_candidate_window")
        ),
        "target_best_sigma_matches_candidate_window_witness_number": bool_number(
            row.get("target_best_sigma_matches_candidate_window_witness")
        ),
        "observed_rank_desc": row.get("observed_rank_desc"),
        "candidate_window_rank_desc": row.get("candidate_window_rank_desc"),
        "near_active_rank_desc": row.get("near_active_rank_desc"),
        "fixed_winner_action_error": row.get("fixed_winner_action_error"),
        "abs_fixed_winner_action_error": abs_or_none(row.get("fixed_winner_action_error")),
        "fixed_winner_volume_error": row.get("fixed_winner_volume_error"),
        "abs_fixed_winner_volume_error": abs_or_none(row.get("fixed_winner_volume_error")),
        "fixed_winner_sys_error_action_part": row.get("fixed_winner_sys_error_action_part"),
        "abs_fixed_winner_sys_error_action_part": abs_or_none(
            row.get("fixed_winner_sys_error_action_part")
        ),
        "fixed_winner_sys_error_volume_part": row.get("fixed_winner_sys_error_volume_part"),
        "abs_fixed_winner_sys_error_volume_part": abs_or_none(
            row.get("fixed_winner_sys_error_volume_part")
        ),
    }


def percentile(values, p):
    values = sorted(v for v in values if finite(v))
    if not values:
        return None
    if len(values) == 1:
        return values[0]
    pos = (len(values) - 1) * p
    lo = math.floor(pos)
    hi = math.ceil(pos)
    if lo == hi:
        return values[lo]
    return values[lo] * (hi - pos) + values[hi] * (pos - lo)


def summarize_group(rows, group_fields):
    groups = defaultdict(list)
    for row in rows:
        groups[tuple(row.get(field) for field in group_fields)].append(row)
    out = []
    for key, group in sorted(groups.items(), key=lambda item: tuple(str(x) for x in item[0])):
        source_counts = Counter(row["source_class"] for row in group)
        ok_rows = [row for row in group if row["status"] == "ok"]
        decomp_rows = [row for row in ok_rows if finite(row.get("abs_prediction_error"))]
        record = {field: value for field, value in zip(group_fields, key)}
        record.update(
            {
                "rows": len(group),
                "ok_rows": len(ok_rows),
                "decomposed_ok_rows": len(decomp_rows),
                "construction_or_domain_failures": source_counts[
                    "construction_or_domain_failure"
                ],
                "smooth_linearization_rows": source_counts["smooth_linearization"],
                "sigma_window_rows": source_counts["sigma_window"],
                "unknown_decomposition_rows": source_counts["unknown_decomposition"],
                "median_abs_prediction_error": percentile(
                    [row.get("abs_prediction_error") for row in decomp_rows], 0.5
                ),
                "p90_abs_prediction_error": percentile(
                    [row.get("abs_prediction_error") for row in decomp_rows], 0.9
                ),
                "max_abs_prediction_error": percentile(
                    [row.get("abs_prediction_error") for row in decomp_rows], 1.0
                ),
                "max_abs_linearization_error": percentile(
                    [row.get("abs_linearization_error") for row in decomp_rows], 1.0
                ),
                "max_abs_sigma_set_error": percentile(
                    [row.get("abs_sigma_set_error") for row in decomp_rows], 1.0
                ),
                "median_abs_relative_error_to_observed_delta": percentile(
                    [
                        row.get("abs_relative_error_to_observed_delta")
                        for row in decomp_rows
                    ],
                    0.5,
                ),
                "max_abs_relative_error_to_base_sys": percentile(
                    [row.get("abs_relative_error_to_base_sys") for row in decomp_rows],
                    1.0,
                ),
            }
        )
        out.append(record)
    return out


def pearson(xs, ys):
    pairs = [(x, y) for x, y in zip(xs, ys) if finite(x) and finite(y)]
    if len(pairs) < 3:
        return None
    xs = [x for x, _ in pairs]
    ys = [y for _, y in pairs]
    mean_x = sum(xs) / len(xs)
    mean_y = sum(ys) / len(ys)
    cov = sum((x - mean_x) * (y - mean_y) for x, y in pairs)
    var_x = sum((x - mean_x) ** 2 for x in xs)
    var_y = sum((y - mean_y) ** 2 for y in ys)
    if var_x == 0 or var_y == 0:
        return None
    return cov / math.sqrt(var_x * var_y)


def ranks(values):
    indexed = sorted((value, index) for index, value in enumerate(values))
    result = [None] * len(values)
    i = 0
    while i < len(indexed):
        j = i + 1
        while j < len(indexed) and indexed[j][0] == indexed[i][0]:
            j += 1
        rank = (i + j - 1) / 2 + 1
        for _, index in indexed[i:j]:
            result[index] = rank
        i = j
    return result


def correlation_rows(rows):
    decomp = [row for row in rows if finite(row.get("abs_prediction_error"))]
    features = {
        "step": [row.get("step") for row in decomp],
        "log10_step": [math.log10(row["step"]) if finite(row.get("step")) and row["step"] > 0 else None for row in decomp],
        "base_near_active_count": [row.get("base_near_active_count") for row in decomp],
        "base_candidate_window_count": [row.get("base_candidate_window_count") for row in decomp],
        "candidate_window_witness_base_gap": [
            row.get("candidate_window_witness_base_gap") for row in decomp
        ],
        "candidate_window_witness_relative_action_gap": [
            row.get("candidate_window_witness_relative_action_gap") for row in decomp
        ],
        "candidate_window_predicted_gap_to_second": [
            row.get("candidate_window_predicted_gap_to_second") for row in decomp
        ],
        "abs_candidate_window_witness_derivative": [
            abs_or_none(row.get("candidate_window_witness_derivative")) for row in decomp
        ],
        "abs_predicted_delta": [
            abs_or_none(row.get("candidate_window_predicted_delta_sys")) for row in decomp
        ],
        "abs_observed_delta": [abs_or_none(row.get("observed_delta_sys")) for row in decomp],
        "base_sys": [row.get("base_sys") for row in decomp],
        "target_near_active_count": [row.get("target_near_active_count") for row in decomp],
        "target_best_sigma_in_base_candidate_window": [
            row.get("target_best_sigma_in_base_candidate_window_number") for row in decomp
        ],
        "target_best_sigma_matches_candidate_window_witness": [
            row.get("target_best_sigma_matches_candidate_window_witness_number")
            for row in decomp
        ],
        "target_orbit_iterations": [row.get("target_orbit_iterations") for row in decomp],
    }
    targets = {
        "abs_prediction_error": [row.get("abs_prediction_error") for row in decomp],
        "abs_linearization_error": [row.get("abs_linearization_error") for row in decomp],
        "abs_sigma_set_error": [row.get("abs_sigma_set_error") for row in decomp],
    }
    out = []
    for target_name, target_values in targets.items():
        for feature_name, feature_values in features.items():
            pairs = [
                (feature, target)
                for feature, target in zip(feature_values, target_values)
                if finite(feature) and finite(target)
            ]
            feature_rank = ranks([feature for feature, _ in pairs])
            target_rank_paired = ranks([target for _, target in pairs])
            out.append(
                {
                    "target": target_name,
                    "feature": feature_name,
                    "n": len(pairs),
                    "pearson": pearson(
                        [feature for feature, _ in pairs],
                        [target for _, target in pairs],
                    ),
                    "spearman": pearson(feature_rank, target_rank_paired)
                    if len(pairs) >= 3
                    else None,
                }
            )
    return out


def write_csv(path, rows, fieldnames=None):
    path.parent.mkdir(parents=True, exist_ok=True)
    if fieldnames is None:
        fieldnames = sorted(set().union(*(row.keys() for row in rows))) if rows else []
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(
            handle,
            fieldnames=fieldnames,
            extrasaction="ignore",
            lineterminator="\n",
        )
        writer.writeheader()
        for row in rows:
            writer.writerow(row)


def fmt(value):
    if value is None:
        return ""
    if isinstance(value, float):
        if value == 0:
            return "0"
        return f"{value:.3g}"
    return str(value)


def markdown_table(rows, fieldnames):
    lines = []
    lines.append("| " + " | ".join(fieldnames) + " |")
    lines.append("| " + " | ".join("---" for _ in fieldnames) + " |")
    for row in rows:
        lines.append("| " + " | ".join(fmt(row.get(field)) for field in fieldnames) + " |")
    return "\n".join(lines)


def write_report(path, input_path, rows, summaries, sigma_cases, correlations):
    source_counts = Counter(row["source_class"] for row in rows)
    ok_rows = [row for row in rows if row["status"] == "ok"]
    decomp_rows = [
        row
        for row in ok_rows
        if finite(row.get("abs_prediction_error"))
        and finite(row.get("abs_linearization_error"))
        and finite(row.get("abs_sigma_set_error"))
    ]
    top_corr = sorted(
        [row for row in correlations if row["target"] == "abs_prediction_error" and row["n"] >= 20],
        key=lambda row: abs(row["spearman"]) if row["spearman"] is not None else -1,
        reverse=True,
    )[:8]
    lines = [
        "# Linear Lower-Envelope Prediction Error Model",
        "",
        "Generated by `experiments/dev-sys-prediction/analyze_prediction_error_model.py`.",
        "",
        "## Input",
        "",
        f"- prediction cloud: `{input_path}`",
        f"- rows: `{len(rows)}` total, `{len(ok_rows)}` `ok`, `{len(decomp_rows)}` fully decomposed `ok`",
        f"- source classes: `{dict(source_counts)}`",
        "- producer context: current 160-row F=10-ish panel from `dev-sys-prediction-cloud`; rerun the producer command in `CURRENT-RESULTS.md` before treating the numerical summaries as refreshed evidence.",
        "",
        "## Error Magnitude By Step",
        "",
        markdown_table(
            summaries["by_step"],
            [
                "step",
                "rows",
                "ok_rows",
                "construction_or_domain_failures",
                "smooth_linearization_rows",
                "sigma_window_rows",
                "median_abs_prediction_error",
                "p90_abs_prediction_error",
                "max_abs_prediction_error",
                "max_abs_sigma_set_error",
            ],
        ),
        "",
        "## Error Source By Direction Class",
        "",
        markdown_table(
            summaries["by_direction_class"],
            [
                "direction_class",
                "rows",
                "ok_rows",
                "construction_or_domain_failures",
                "smooth_linearization_rows",
                "sigma_window_rows",
                "median_abs_prediction_error",
                "max_abs_prediction_error",
                "max_abs_sigma_set_error",
            ],
        ),
        "",
        "## Error Source By Candidate-Window Count",
        "",
        markdown_table(
            summaries["by_window_count"],
            [
                "base_candidate_window_count",
                "rows",
                "ok_rows",
                "smooth_linearization_rows",
                "sigma_window_rows",
                "median_abs_prediction_error",
                "max_abs_prediction_error",
                "max_abs_sigma_set_error",
            ],
        ),
        "",
        "## Error Source By Target Sigma Visibility",
        "",
        markdown_table(
            summaries["by_target_window_membership"],
            [
                "target_best_sigma_in_base_candidate_window",
                "rows",
                "ok_rows",
                "construction_or_domain_failures",
                "smooth_linearization_rows",
                "sigma_window_rows",
                "unknown_decomposition_rows",
                "median_abs_prediction_error",
                "max_abs_prediction_error",
                "max_abs_sigma_set_error",
            ],
        ),
        "",
        "## Error Source By Target-Winner Match",
        "",
        markdown_table(
            summaries["by_target_winner_match"],
            [
                "target_best_sigma_matches_candidate_window_witness",
                "rows",
                "ok_rows",
                "construction_or_domain_failures",
                "smooth_linearization_rows",
                "sigma_window_rows",
                "unknown_decomposition_rows",
                "median_abs_prediction_error",
                "max_abs_prediction_error",
                "max_abs_sigma_set_error",
            ],
        ),
        "",
        "## Strongest Simple Correlations",
        "",
        "These are descriptive correlations on the tiny current panel, not a fitted predictive model.",
        "",
        markdown_table(top_corr, ["feature", "n", "pearson", "spearman"]),
        "",
        "## Ranked Sigma-Window Cases",
        "",
        markdown_table(
            sigma_cases[:12],
            [
                "rank",
                "poly_id",
                "degeneracy_label",
                "direction_label",
                "step",
                "source_class",
                "abs_sigma_set_error",
                "abs_prediction_error",
                "abs_linearization_error",
                "candidate_window_witness_base_gap",
                "base_candidate_window_count",
            ],
        ),
        "",
        "## Interpretation",
        "",
        "- Current evidence is enough for stratification, not for a stable statistical predictor.",
        "- Small radii through `3e-3` are smooth fixed-window rows in this panel.",
        "- Sigma-window or incoming-branch error first appears at `1e-2` and becomes more common at stress radii.",
        "- Relative error to observed delta is recorded in the feature table, but it is unstable when the observed delta is small or changes sign. Relative error to base `sys` is more stable but less optimizer-local.",
        "- Construction/domain failures are a separate source class; treating them as large numeric errors would hide the boundary condition.",
        "- Target-best-sigma outside the base candidate window is a useful warning signal for sigma-window error, but not a complete source classifier at large radii because several smooth-linearization-dominated rows also have target-best outside the base window.",
        "- Predicted best-versus-second lower-envelope gap has a visible association with error in this panel, but it is not monotone enough to use as an acceptance rule.",
        "- Base facet count, beta-margin, and geometry scale fields were not present in this prediction-cloud schema; add them to future traces if Session A shows radius calibration depends on them.",
        "",
        "## Future Trace Fields",
        "",
        "- keep target best sigma and whether it is in the base candidate window in every prediction row;",
        "- keep predicted best-vs-second branch gap in the lower-envelope model;",
        "- minimum beta margin for the predicted winner and nearby branches;",
        "- volume derivative and predicted volume contribution for the selected branch;",
        "- base geometry scale fields, including facet count and any radius-calibration fields from Session A;",
        "- construction/domain failure reason with enough target coordinates or direction identifiers to replay the case.",
        "",
        "## Outputs",
        "",
        "- `feature-error-table.csv`: one row per prediction-cloud row with local features and source class.",
        "- `summary-by-step.csv`, `summary-by-direction-class.csv`, `summary-by-window-count.csv`, target-sigma visibility summaries, and `correlations.csv`: aggregate views.",
        "- `sigma-window-dominated-cases.csv`: ranked cases for incoming-branch/source tracing work.",
    ]
    path.write_text("\n".join(lines) + "\n")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--prediction-cloud", required=True, type=Path)
    parser.add_argument("--out-dir", default=DEFAULT_OUTPUT_DIR, type=Path)
    args = parser.parse_args()

    raw_rows = [
        json.loads(line)
        for line in args.prediction_cloud.read_text().splitlines()
        if line.strip()
    ]
    rows = [feature_row(row) for row in raw_rows]

    summaries = {
        "by_step": summarize_group(rows, ["step"]),
        "by_direction_class": summarize_group(rows, ["direction_class"]),
        "by_direction_label": summarize_group(rows, ["direction_label"]),
        "by_degeneracy_label": summarize_group(rows, ["degeneracy_label"]),
        "by_window_count": summarize_group(rows, ["base_candidate_window_count"]),
        "by_target_window_membership": summarize_group(
            rows, ["target_best_sigma_in_base_candidate_window"]
        ),
        "by_target_winner_match": summarize_group(
            rows, ["target_best_sigma_matches_candidate_window_witness"]
        ),
        "by_step_direction_class": summarize_group(rows, ["step", "direction_class"]),
        "by_step_target_window_membership": summarize_group(
            rows, ["step", "target_best_sigma_in_base_candidate_window"]
        ),
    }
    correlations = correlation_rows(rows)
    sigma_cases = sorted(
        [
            dict(row, rank=0)
            for row in rows
            if row["source_class"] == "sigma_window"
            and finite(row.get("abs_sigma_set_error"))
            and row["abs_sigma_set_error"] > 1e-12
        ],
        key=lambda row: -(row.get("abs_sigma_set_error") or 0.0),
    )
    for index, row in enumerate(sigma_cases, start=1):
        row["rank"] = index

    args.out_dir.mkdir(parents=True, exist_ok=True)
    write_csv(args.out_dir / "feature-error-table.csv", rows)
    for name, summary_rows in summaries.items():
        write_csv(args.out_dir / f"summary-{name.replace('_', '-')}.csv", summary_rows)
    write_csv(args.out_dir / "correlations.csv", correlations)
    write_csv(args.out_dir / "sigma-window-dominated-cases.csv", sigma_cases)
    write_report(
        args.out_dir / "REPORT.md",
        args.prediction_cloud,
        rows,
        summaries,
        sigma_cases,
        correlations,
    )


if __name__ == "__main__":
    main()
