# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "matplotlib>=3.8",
#   "numpy>=1.26",
# ]
# ///
"""Diagnose predictor errors and stale candidate sets in schema-v2 replay data."""

from __future__ import annotations

import argparse
import hashlib
import json
from collections import Counter
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

from explain_replay import (
    AFFINE,
    ANCHOR_ALL,
    BIN_LABELS,
    DIRECT,
    SELECTOR_LABELS,
    TARGET_ALL,
    TARGET_WINNER,
    WINDOWS,
    bin_index,
    cost_rows,
    coverage_rows,
    finite,
    fmt,
    miss_impact_rows,
    plot_cost_coverage,
    plot_coverage,
    plot_coverage_strata,
    plot_distance_population,
    plot_miss_impact,
    plot_tail,
    pooled_cost_row,
    read_jsonl,
    write_csv,
)

STEP_BINS = [(1, 1), (2, 3), (4, 7), (8, 15), (16, 31), (32, 10**9)]
STEP_LABELS = ["1", "2–3", "4–7", "8–15", "16–31", "32+"]
AFFINE_CAUSES = [
    "same target winner",
    "different target-admissible branch",
    "target transition blocked",
    "target beta nonpositive",
    "target raw KKT failure",
    "no affine prediction",
]


def percentile(values: list[float], q: float) -> float | None:
    values = [value for value in values if finite(value)]
    return float(np.quantile(values, q)) if values else None


def winner_cause_rows(mechanisms: list[dict]) -> list[dict]:
    result = []
    for index, label in enumerate(BIN_LABELS):
        group = [
            row
            for row in mechanisms
            if bin_index(row["normalized_distance"]) == index
        ]
        counts = Counter(row["omission_class"] for row in group)
        result.append(
            {
                "distance_bin": label,
                "distance_bin_index": index,
                "targets": len(group),
                "covered_by_anchor_universe": counts["covered_by_anchor_universe"],
                "anchor_transition_blocked": counts["anchor_transition_blocked"],
                "anchor_raw_kkt_failed": counts["anchor_raw_kkt_failed"],
                "anchor_enumeration_omission": counts["anchor_enumeration_omission"],
                "future_winner_beta_nonpositive_at_anchor": sum(
                    finite(row.get("anchor_raw_normalized_beta_margin"))
                    and row["anchor_raw_normalized_beta_margin"] <= 0.0
                    for row in group
                ),
            }
        )
    return result


def affine_cause(row: dict) -> str:
    if row.get("predicted_winning_sigma") is None:
        return "no affine prediction"
    if row.get("predicted_winner_matches_target"):
        return "same target winner"
    if row.get("predicted_winner_target_transition_feasible") is False:
        return "target transition blocked"
    if row.get("predicted_winner_target_raw_status") != "ok":
        return "target raw KKT failure"
    beta = row.get("predicted_winner_target_raw_normalized_beta_margin")
    if finite(beta) and beta <= 0.0:
        return "target beta nonpositive"
    return "different target-admissible branch"


def affine_cause_rows(atoms: list[dict]) -> list[dict]:
    result = []
    for selector in (TARGET_WINNER, WINDOWS[1], ANCHOR_ALL):
        selected = [
            row
            for row in atoms
            if row["value_model"] == AFFINE and row["selector"] == selector
        ]
        for cause in AFFINE_CAUSES:
            group = [row for row in selected if affine_cause(row) == cause]
            envelope_errors = [
                abs(row["prediction_error"])
                for row in group
                if finite(row.get("prediction_error"))
            ]
            branch_errors = [
                abs(row["selected_branch_prediction_error"])
                for row in group
                if finite(row.get("selected_branch_prediction_error"))
            ]
            result.append(
                {
                    "selector": selector,
                    "selector_label": SELECTOR_LABELS[selector],
                    "cause": cause,
                    "targets": len(group),
                    "fraction": len(group) / len(selected) if selected else None,
                    "median_abs_envelope_error": percentile(envelope_errors, 0.5),
                    "q90_abs_envelope_error": percentile(envelope_errors, 0.9),
                    "max_abs_envelope_error": max(envelope_errors, default=None),
                    "median_abs_selected_branch_error": percentile(branch_errors, 0.5),
                    "q90_abs_selected_branch_error": percentile(branch_errors, 0.9),
                    "max_abs_selected_branch_error": max(branch_errors, default=None),
                }
            )
    return result


def lifetime_rows(rows: list[dict]) -> list[dict]:
    result = []
    for selector in (*WINDOWS, ANCHOR_ALL):
        selected = [row for row in rows if row["selector"] == selector]
        for index, (lower, upper) in enumerate(STEP_BINS):
            group = [
                row
                for row in selected
                if lower <= row["accepted_steps_after_anchor"] <= upper
            ]
            errors = [
                abs(row["prediction_error"])
                for row in group
                if finite(row.get("prediction_error"))
            ]
            result.append(
                {
                    "selector": selector,
                    "selector_label": SELECTOR_LABELS[selector],
                    "step_bin": STEP_LABELS[index],
                    "step_bin_index": index,
                    "targets": len(group),
                    "starts": len({row["start_id"] for row in group}),
                    "winner_coverage": (
                        float(np.mean([row["target_winner_covered"] for row in group]))
                        if group
                        else None
                    ),
                    "prediction_rate": len(errors) / len(group) if group else None,
                    "material_error_fraction": (
                        float(
                            np.mean(
                                [
                                    not finite(row.get("prediction_error"))
                                    or abs(row["prediction_error"]) > 1.0e-3
                                    for row in group
                                ]
                            )
                        )
                        if group
                        else None
                    ),
                    "median_normalized_distance": percentile(
                        [row["normalized_distance"] for row in group], 0.5
                    ),
                    "q90_abs_error": percentile(errors, 0.9),
                }
            )
    return result


def rollback_rows(rows: list[dict]) -> list[dict]:
    result = []
    for algorithm in ["pooled", *sorted({row["algorithm_id"] for row in rows})]:
        group = (
            rows
            if algorithm == "pooled"
            else [row for row in rows if row["algorithm_id"] == algorithm]
        )
        result.append(
            {
                "algorithm_id": algorithm,
                "future_targets": len(group),
                "previous_step_admissible": float(
                    np.mean([row["previous_step_admissible"] for row in group])
                ),
                "previous_step_within_1e_2": float(
                    np.mean(
                        [
                            finite(row.get("previous_step_gap"))
                            and abs(row["previous_step_gap"]) <= 1.0e-2
                            for row in group
                        ]
                    )
                ),
                "previous_step_within_1e_3": float(
                    np.mean(
                        [
                            finite(row.get("previous_step_gap"))
                            and abs(row["previous_step_gap"]) <= 1.0e-3
                            for row in group
                        ]
                    )
                ),
                "median_admissible_lead_steps": percentile(
                    [row["admissible_lead_steps"] for row in group], 0.5
                ),
                "median_within_1e_2_lead_steps": percentile(
                    [row["within_1e_2_lead_steps"] for row in group], 0.5
                ),
                "median_within_1e_3_lead_steps": percentile(
                    [row["within_1e_3_lead_steps"] for row in group], 0.5
                ),
                "median_winner_identity_lead_steps": percentile(
                    [row["winner_identity_lead_steps"] for row in group], 0.5
                ),
            }
        )
    return result


def gain_rows(atoms: list[dict]) -> list[dict]:
    result = []
    for value_model in (DIRECT, AFFINE):
        group = [
            row
            for row in atoms
            if row["selector"] == WINDOWS[1]
            and row["value_model"] == value_model
            and finite(row.get("prediction_error"))
        ]
        relative = [
            abs(row["prediction_error"]) / max(abs(row["actual_delta"]), 1.0e-3)
            for row in group
        ]
        result.append(
            {
                "value_model": value_model,
                "targets": len(group),
                "median_error_over_gain_floor_1e_3": percentile(relative, 0.5),
                "q90_error_over_gain_floor_1e_3": percentile(relative, 0.9),
                "fraction_error_larger_than_gain_floor_1e_3": float(
                    np.mean(np.asarray(relative) > 1.0)
                ),
                "false_improvement_fraction": float(
                    np.mean(
                        [
                            row["predicted_delta"] > 0.0
                            and row["actual_delta"] <= 0.0
                            for row in group
                        ]
                    )
                ),
                "false_rejection_fraction": float(
                    np.mean(
                        [
                            row["predicted_delta"] <= 0.0
                            and row["actual_delta"] > 0.0
                            for row in group
                        ]
                    )
                ),
            }
        )
    return result


def rank(values: np.ndarray) -> np.ndarray:
    order = np.argsort(values, kind="stable")
    result = np.empty(len(values), dtype=float)
    sorted_values = values[order]
    begin = 0
    while begin < len(values):
        end = begin + 1
        while end < len(values) and sorted_values[end] == sorted_values[begin]:
            end += 1
        result[order[begin:end]] = (begin + end - 1) / 2.0
        begin = end
    return result


def auc(scores: np.ndarray, labels: np.ndarray) -> float | None:
    positive = scores[labels]
    negative = scores[~labels]
    if not len(positive) or not len(negative):
        return None
    wins = sum(
        np.sum(value > negative) + 0.5 * np.sum(value == negative)
        for value in positive
    )
    return float(wins / (len(positive) * len(negative)))


def distance_rows(atoms: list[dict]) -> list[dict]:
    group = [
        row
        for row in atoms
        if row["selector"] == WINDOWS[1] and row["value_model"] == DIRECT
    ]
    labels = np.asarray(
        [
            not finite(row.get("prediction_error"))
            or abs(row["prediction_error"]) > 1.0e-2
            for row in group
        ],
        dtype=bool,
    )
    result = []
    for field in (
        "normalized_distance",
        "symmetry_transverse_normalized_distance",
        "candidate_count",
        "anchor_sys",
    ):
        scores = np.asarray([row[field] for row in group], dtype=float)
        result.append(
            {
                "field": field,
                "targets": len(group),
                "material_errors": int(np.sum(labels)),
                "rank_correlation_with_material_error": float(
                    np.corrcoef(rank(scores), rank(labels.astype(float)))[0, 1]
                ),
                "auc_for_material_error": auc(scores, labels),
            }
        )
    return result


def plot_winner_causes(rows: list[dict], path: Path) -> None:
    categories = [
        ("covered_by_anchor_universe", "covered"),
        ("anchor_transition_blocked", "transition blocked at anchor"),
        ("anchor_raw_kkt_failed", "raw KKT failure at anchor"),
        ("anchor_enumeration_omission", "other enumeration omission"),
    ]
    figure, axis = plt.subplots(figsize=(10.0, 5.2))
    bottom = np.zeros(len(rows))
    totals = np.asarray([row["targets"] for row in rows], dtype=float)
    for field, label in categories:
        values = np.asarray(
            [row[field] / total if total else 0.0 for row, total in zip(rows, totals)]
        )
        axis.bar(np.arange(len(rows)), values, bottom=bottom, label=label)
        bottom += values
    axis.set_xticks(
        np.arange(len(rows)),
        [f"{row['distance_bin']}\nn={row['targets']}" for row in rows],
    )
    axis.set_ylim(0, 1)
    axis.set_ylabel("fraction of target full-sys winners")
    axis.set_xlabel("ambient normalized distance bin")
    axis.legend(fontsize=8)
    axis.grid(axis="y", alpha=0.2)
    figure.tight_layout()
    figure.savefig(path, dpi=180)
    figure.savefig(path.with_suffix(".pdf"))
    plt.close(figure)


def plot_affine_causes(rows: list[dict], path: Path) -> None:
    selectors = [TARGET_WINNER, WINDOWS[1], ANCHOR_ALL]
    figure, axes = plt.subplots(1, 2, figsize=(13.0, 5.2))
    bottom = np.zeros(len(selectors))
    for cause in AFFINE_CAUSES:
        values = [
            next(
                row["fraction"]
                for row in rows
                if row["selector"] == selector and row["cause"] == cause
            )
            for selector in selectors
        ]
        axes[0].bar(np.arange(len(selectors)), values, bottom=bottom, label=cause)
        bottom += np.asarray(values)
    axes[0].set_xticks(
        np.arange(len(selectors)),
        [SELECTOR_LABELS[selector] for selector in selectors],
        rotation=12,
    )
    axes[0].set_ylim(0, 1)
    axes[0].set_ylabel("fraction of targets")
    axes[0].legend(fontsize=7)
    x = np.arange(len(AFFINE_CAUSES))
    width = 0.35
    for offset, selector in [(-width / 2, WINDOWS[1]), (width / 2, ANCHOR_ALL)]:
        values = [
            next(
                (
                    row["q90_abs_envelope_error"]
                    for row in rows
                    if row["selector"] == selector and row["cause"] == cause
                ),
                None,
            )
            for cause in AFFINE_CAUSES
        ]
        axes[1].bar(
            x + offset,
            [value if finite(value) else np.nan for value in values],
            width,
            label=SELECTOR_LABELS[selector],
        )
    axes[1].set_xticks(x, AFFINE_CAUSES, rotation=25, ha="right")
    axes[1].set_yscale("symlog", linthresh=1e-5)
    axes[1].set_ylabel("90th percentile absolute sys error")
    axes[1].legend(fontsize=8)
    for axis in axes:
        axis.grid(axis="y", alpha=0.2)
    figure.tight_layout()
    figure.savefig(path, dpi=180)
    figure.savefig(path.with_suffix(".pdf"))
    plt.close(figure)


def plot_lifetime(rows: list[dict], path: Path) -> None:
    figure, axes = plt.subplots(1, 2, figsize=(12.5, 5.0), sharex=True)
    for selector in (*WINDOWS, ANCHOR_ALL):
        group = sorted(
            [row for row in rows if row["selector"] == selector],
            key=lambda row: row["step_bin_index"],
        )
        axes[0].plot(
            np.arange(len(group)),
            [row["winner_coverage"] for row in group],
            marker="o",
            label=SELECTOR_LABELS[selector],
        )
        axes[1].plot(
            np.arange(len(group)),
            [row["material_error_fraction"] for row in group],
            marker="o",
            label=SELECTOR_LABELS[selector],
        )
    axes[0].set_ylabel("target winner coverage")
    axes[1].set_ylabel("unusable or absolute sys error > 1e-3")
    for axis in axes:
        axis.set_xticks(np.arange(len(STEP_LABELS)), STEP_LABELS, rotation=20)
        axis.set_xlabel("accepted steps after anchor")
        axis.set_ylim(-0.02, 1.02)
        axis.grid(alpha=0.2)
    axes[0].legend(fontsize=7)
    figure.tight_layout()
    figure.savefig(path, dpi=180)
    figure.savefig(path.with_suffix(".pdf"))
    plt.close(figure)


def plot_rollback(rows: list[dict], path: Path) -> None:
    figure, axis = plt.subplots(figsize=(8.8, 5.0))
    for field, label in [
        ("admissible_lead_steps", "physically admissible"),
        ("within_1e_2_lead_steps", "within 1e-2 of full sys"),
        ("within_1e_3_lead_steps", "within 1e-3 of full sys"),
        ("winner_identity_lead_steps", "already the full-sys winner"),
    ]:
        values = np.sort([row[field] for row in rows if row[field] is not None])
        axis.step(
            values,
            np.arange(1, len(values) + 1) / len(values),
            where="post",
            label=label,
        )
    axis.set_xlabel("accepted steps before the recorded future win")
    axis.set_ylabel("fraction detectable by this lead time")
    axis.set_ylim(0, 1.02)
    axis.grid(alpha=0.2)
    axis.legend(fontsize=8)
    figure.tight_layout()
    figure.savefig(path, dpi=180)
    figure.savefig(path.with_suffix(".pdf"))
    plt.close(figure)


def plot_distances(atoms: list[dict], path: Path) -> None:
    rows = [
        row
        for row in atoms
        if row["selector"] == WINDOWS[1] and row["value_model"] == DIRECT
    ]
    ambient = np.asarray([row["normalized_distance"] for row in rows])
    transverse = np.asarray(
        [row["symmetry_transverse_normalized_distance"] for row in rows]
    )
    errors = np.asarray(
        [
            abs(row["prediction_error"])
            if finite(row.get("prediction_error"))
            else np.nan
            for row in rows
        ]
    )
    figure, axes = plt.subplots(1, 2, figsize=(11.5, 4.8))
    axes[0].scatter(ambient, transverse, s=10, alpha=0.5)
    limit = max(np.max(ambient), np.max(transverse))
    axes[0].plot([0, limit], [0, limit], color="black", linewidth=0.8)
    axes[0].set_xlabel("ambient normalized distance")
    axes[0].set_ylabel("symmetry-transverse normalized distance")
    axes[1].scatter(
        np.maximum(ambient, 1e-8),
        np.maximum(errors, 1e-12),
        s=10,
        alpha=0.45,
    )
    axes[1].set_xscale("log")
    axes[1].set_yscale("log")
    axes[1].set_xlabel("ambient normalized distance")
    axes[1].set_ylabel("absolute 10%-set direct prediction error")
    for axis in axes:
        axis.grid(alpha=0.2)
    figure.tight_layout()
    figure.savefig(path, dpi=180)
    figure.savefig(path.with_suffix(".pdf"))
    plt.close(figure)


def write_report(
    path: Path,
    dataset: Path,
    pairs: list[dict],
    atoms: list[dict],
    mechanisms: list[dict],
    affine_causes: list[dict],
    lifetimes: list[dict],
    rollback_summary: list[dict],
    gains: list[dict],
    distances: list[dict],
) -> None:
    usable_pairs = [row for row in pairs if finite(row.get("target_sys"))]
    invalid = [row for row in pairs if not finite(row.get("target_sys"))]
    covered_direct = [
        row
        for row in atoms
        if row["value_model"] == DIRECT
        and row.get("target_winner_covered") is True
        and finite(row.get("prediction_error"))
    ]
    target_controls = [
        row
        for row in atoms
        if row["value_model"] == DIRECT
        and row["selector"] == TARGET_ALL
        and finite(row.get("prediction_error"))
    ]
    mechanism_counts = Counter(row["omission_class"] for row in mechanisms)
    negative_beta = sum(
        finite(row.get("anchor_raw_normalized_beta_margin"))
        and row["anchor_raw_normalized_beta_margin"] <= 0.0
        for row in mechanisms
    )
    represented_winner = [
        row
        for row in affine_causes
        if row["selector"] == TARGET_WINNER and row["cause"] == "same target winner"
    ][0]["targets"]
    distance_lookup = {row["field"]: row for row in distances}
    transverse_ratio = [
        row["symmetry_transverse_normalized_distance"] / row["normalized_distance"]
        for row in atoms
        if row["selector"] == WINDOWS[1]
        and row["value_model"] == DIRECT
        and row["normalized_distance"] > 0
    ]
    status_counts = Counter(row["target_status"] for row in pairs)
    lines = [
        "# Predictor and candidate-set diagnostics",
        "",
        "## Result in one page",
        "",
        (
            f"This is a replay of 144 selected on-trajectory moves from 48 F=10 "
            f"development runs (16 starts and three optimizers), at 0.5, 1, and 2 "
            f"times each recorded displacement. It produced {len(pairs)} target "
            f"evaluations, of which {len(usable_pairs)} were usable."
        ),
        "",
        "The main conclusions are:",
        "",
        (
            f"- **Reevaluating retained branches works when the right branch is "
            f"present.** Across {len(covered_direct)} rows where a named set "
            f"contained the target full-sys winner, the largest recorded error was "
            f"{max(abs(row['prediction_error']) for row in covered_direct):.3g}. "
            f"All {len(target_controls)} target-universe controls also reproduced "
            f"full sys."
        ),
        (
            f"- **Transition change, not the action window, is the dominant hard "
            f"candidate-set failure.** Of {len(mechanisms)} usable targets, "
            f"{mechanism_counts['anchor_transition_blocked']} future winners were "
            f"transition-blocked at the anchor, "
            f"{mechanism_counts['anchor_raw_kkt_failed']} had an anchor raw-KKT "
            f"failure, and {mechanism_counts['anchor_enumeration_omission']} were "
            f"otherwise omitted. Increasing an action window cannot recover a branch "
            f"that was never in the anchor transition-feasible universe."
        ),
        (
            f"- **A hard anchor beta cutoff would discard useful branches.** "
            f"{negative_beta}/{len(mechanisms)} future winners had nonpositive raw "
            f"normalized beta at the anchor."
        ),
        (
            f"- **Frozen-domain affine prediction has two independent failures.** "
            f"The target-winning branch was representable at the anchor in only "
            f"{represented_winner}/{len(mechanisms)} rows. Even then, finite-distance "
            f"same-branch affine error had a long tail; allowing many affine branches "
            f"also selected branches that were transition-blocked or beta-nonpositive "
            f"at the target."
        ),
        (
            "- **Candidate history is useful, but does not make a stale set reliable.** "
            "Future winners were often detectable several accepted steps before "
            "becoming the winner, while one-step-back detection was incomplete. "
            "This supports remembering newly observed winners and retroactive "
            "diagnostics, but not replacing refreshes with an indefinitely growing "
            "anchor set."
        ),
        "",
        "These are predictor diagnostics, not a full optimizer comparison. They say "
        "which approximations fail and which state a practical optimizer should "
        "refresh; they do not establish long-run improvement per compute.",
        "",
        "## Data and definitions",
        "",
        f"Source dataset: `{dataset}`.",
        "",
        (
            "Status counts: "
            + ", ".join(
                f"`{key}` {value}" for key, value in sorted(status_counts.items())
            )
            + "."
        ),
        "",
        "The ambient normalized distance is",
        "",
        r"\[d(a_0,a_1)=\frac{\lVert a_1-a_0\rVert_2}{\lVert a_0\rVert_2}.\]",
        "",
        "The symmetry-transverse distance projects the displacement away from the "
        "15 infinitesimal symmetry directions computed at the anchor before taking "
        "the same ratio. Candidate sets use unrestricted f64 raw KKT germs from "
        "transition-feasible anchor sigma; the action window applies no beta cutoff. "
        "Named branches are reevaluated at the target with target transition and "
        "beta admissibility.",
        "",
        "![Population of replay distances](distance-population.png)",
        "",
        "## Why future winning branches are absent",
        "",
        "![Future-winner status at the anchor](winner-omission-causes.png)",
        "",
        "| anchor status of target winner | count | fraction |",
        "|---|---:|---:|",
    ]
    for key, label in [
        ("covered_by_anchor_universe", "present in anchor universe"),
        ("anchor_transition_blocked", "transition-blocked"),
        ("anchor_raw_kkt_failed", "raw-KKT failure"),
        ("anchor_enumeration_omission", "other enumeration omission"),
    ]:
        lines.append(
            f"| {label} | {mechanism_counts[key]} | "
            f"{fmt(mechanism_counts[key] / len(mechanisms))} |"
        )
    lines.extend(
        [
            "",
            "The absence of `other enumeration omission` is a useful diagnostic: "
            "the current enumeration plumbing usually finds a raw germ when the "
            "target winner is already transition-feasible at the anchor. The main "
            "hard failure for this anchor-candidate rule in this sample is a "
            "transition becoming feasible later.",
            "",
            "Action-window coverage of the target winner:",
            "",
            "| candidate set | pooled winner coverage | median candidates | median measured cost / full sys |",
            "|---|---:|---:|---:|",
        ]
    )
    for selector in [*WINDOWS, ANCHOR_ALL]:
        pooled = pooled_cost_row(atoms, pairs, selector)
        lines.append(
            f"| {SELECTOR_LABELS[selector]} | {fmt(pooled['winner_coverage'])} | "
            f"{fmt(pooled['median_candidates'])} | "
            f"{fmt(pooled['median_fraction_of_full_evaluation'])} |"
        )
    lines.extend(
        [
            "",
            "![Coverage versus distance](coverage-vs-distance.png)",
            "",
            "![Coverage stratified by optimizer and phase](coverage-vs-distance-strata.png)",
            "",
            "The stratified plot is important: optimizer and trajectory phase are "
            "confounded with distance, and some cells are sparse. Pooled curves are "
            "descriptive, not an iid estimate over arbitrary points.",
            "",
            "## What makes affine predictions fail",
            "",
            "For each affine envelope prediction, the trace now records the sigma "
            "selected by the affine minimum and reevaluates that sigma at the target. "
            "This separates finite-distance error of that selected branch from "
            "selecting a branch outside its target physical domain.",
            "",
            "![Affine failure causes](affine-failure-causes.png)",
            "",
            "| 10% affine-set outcome | fraction | median absolute envelope error | 90% error | maximum error |",
            "|---|---:|---:|---:|---:|",
        ]
    )
    for row in affine_causes:
        if row["selector"] == WINDOWS[1] and row["targets"]:
            lines.append(
                f"| {row['cause']} | {fmt(row['fraction'])} | "
                f"{fmt(row['median_abs_envelope_error'])} | "
                f"{fmt(row['q90_abs_envelope_error'])} | "
                f"{fmt(row['max_abs_envelope_error'])} |"
            )
    lines.extend(
        [
            "",
            "The `target winner only` control removes wrong-branch selection. Its "
            "remaining errors are same-branch finite-distance affine errors. A missing "
            "control prediction is explained almost entirely by the target winner "
            "being transition-blocked or beta-nonpositive at the anchor; this is a "
            "domain change, not merely a poor linear fit.",
            "",
            "The all-anchor-germs affine envelope is worse than the 10% envelope: more "
            "branches create more opportunities for an extreme extrapolation from a "
            "branch that is no longer physical. Thus including more branches is "
            "monotone-safe for direct target reevaluation, but not for a frozen-domain "
            "affine minimum.",
            "",
            "![Affine and direct error tails](error-tail-survival.png)",
            "",
            "## Distance and candidate lifetime",
            "",
            "![Ambient and symmetry-transverse distance](distance-diagnostics.png)",
            "",
            (
                "Here the symmetry projection barely changes the moves: the ratio of "
                "symmetry-transverse to ambient distance has median "
                f"{fmt(percentile(transverse_ratio, 0.5))} and 10th percentile "
                f"{fmt(percentile(transverse_ratio, 0.1))}. For classifying a 10%-set "
                "direct error above 1e-2, ambient-distance AUC is "
                f"{fmt(distance_lookup['normalized_distance']['auc_for_material_error'])} "
                "and symmetry-transverse-distance AUC is "
                f"{fmt(distance_lookup['symmetry_transverse_normalized_distance']['auc_for_material_error'])}. "
                "The quotient projection adds essentially no information on this "
                "trajectory sample."
            ),
            "",
            "Candidate lifetime is measured on later accepted trajectory states, not "
            "on scaled off-shell replay points. Each anchor-selected set is directly "
            "reevaluated at those states.",
            "",
            "![Candidate-set lifetime](candidate-lifetime.png)",
            "",
            "| accepted steps after anchor | 10% coverage | 10% material error | all-germs coverage | all-germs material error | median ambient distance |",
            "|---|---:|---:|---:|---:|---:|",
        ]
    )
    for index, label in enumerate(STEP_LABELS):
        by_selector = {
            row["selector"]: row
            for row in lifetimes
            if row["step_bin_index"] == index
        }
        ten = by_selector[WINDOWS[1]]
        all_germs = by_selector[ANCHOR_ALL]
        lines.append(
            f"| {label} | {fmt(ten['winner_coverage'])} | "
            f"{fmt(ten['material_error_fraction'])} | "
            f"{fmt(all_germs['winner_coverage'])} | "
            f"{fmt(all_germs['material_error_fraction'])} | "
            f"{fmt(ten['median_normalized_distance'])} |"
        )
    lines.extend(
        [
            "",
            "Coverage generally decays with accepted-step age, but value error is not "
            "monotone because late trajectories, optimizer family, anchor phase, and "
            "distance are mixed. Therefore accepted-step count alone is not a "
            "sufficient refresh trigger.",
            "",
            "## What retroactive branch checks can reveal",
            "",
            "For every later accepted state, the branch that wins there was reevaluated "
            "at every preceding accepted state back to the selected anchor. The lead "
            "time is how many accepted steps before its recorded win the branch first "
            "became admissible, close in value, or already winning.",
            "",
            "![Retroactive detection lead time](rollback-lead-time.png)",
            "",
            "| population | future targets | previous step admissible | previous step within 1e-2 | previous step within 1e-3 | median admissible lead | median 1e-2 lead | median 1e-3 lead | median winner lead |",
            "|---|---:|---:|---:|---:|---:|---:|---:|---:|",
        ]
    )
    for row in rollback_summary:
        lines.append(
            f"| {row['algorithm_id']} | {row['future_targets']} | "
            f"{fmt(row['previous_step_admissible'])} | "
            f"{fmt(row['previous_step_within_1e_2'])} | "
            f"{fmt(row['previous_step_within_1e_3'])} | "
            f"{fmt(row['median_admissible_lead_steps'])} | "
            f"{fmt(row['median_within_1e_2_lead_steps'])} | "
            f"{fmt(row['median_within_1e_3_lead_steps'])} | "
            f"{fmt(row['median_winner_identity_lead_steps'])} |"
        )
    lines.extend(
        [
            "",
            "This supports a cheap diagnostic after discovering a new winner: check "
            "that sigma at recent saved states to locate when the previous candidate "
            "set became value-wrong. It does not itself repair the already-taken "
            "trajectory, and the result is conditional on trajectories produced by "
            "the existing optimizers.",
            "",
            "## Prediction error relative to the proposed gain",
            "",
            "The denominator below is `max(abs(actual target sys - anchor sys), 1e-3)`. "
            "The floor prevents numerically tiny late moves from dominating the ratio.",
            "",
            "| 10% value model | usable targets | median error/gain | 90% error/gain | error larger than gain | predicts improvement when target worsens | predicts rejection when target improves |",
            "|---|---:|---:|---:|---:|---:|---:|",
        ]
    )
    for row in gains:
        lines.append(
            f"| {row['value_model']} | {row['targets']} | "
            f"{fmt(row['median_error_over_gain_floor_1e_3'])} | "
            f"{fmt(row['q90_error_over_gain_floor_1e_3'])} | "
            f"{fmt(row['fraction_error_larger_than_gain_floor_1e_3'])} | "
            f"{fmt(row['false_improvement_fraction'])} | "
            f"{fmt(row['false_rejection_fraction'])} |"
        )
    lines.extend(
        [
            "",
            "Both models are optimistic on this selected proposal population: the "
            "observed sign error is false improvement, not false rejection. A full-sys "
            "validation before accepting a proposed move is therefore useful even "
            "when the cheap predictor is used to generate it.",
            "",
            "## Invalid replay targets",
            "",
            f"{len(invalid)}/{len(pairs)} replay targets were unusable. Their retained "
            "status and diagnostic fields are:",
            "",
            "| status | geometry route | fallback reason | error | count |",
            "|---|---|---|---|---:|",
        ]
    )
    invalid_counts = Counter(
        (
            row["target_status"],
            row["target_geometry_route"],
            row.get("target_fallback_reason") or "—",
            row.get("target_error") or "—",
        )
        for row in invalid
    )
    for fields, count in invalid_counts.most_common():
        lines.append("| " + " | ".join(map(str, fields)) + f" | {count} |")
    lines.extend(
        [
            "",
            "## Statistical and claim boundary",
            "",
            "- The 16 starts are the independent population units available here. "
            "Rows from the same start, neighboring rounds, replay scales, and "
            "retroactive scans are correlated.",
            "- The optimizer and trajectory-phase mix changes across distance and "
            "lifetime bins. The plots diagnose mechanisms; they are not a fitted "
            "universal error law.",
            "- AUC screens only whether one scalar orders the observed material-error "
            "event. They are not calibrated refresh policies.",
            "- Timings include target geometry and volume reconstruction for every "
            "named-set call. An implementation that reuses them will change the cost "
            "ratios.",
            "- Long-run sys improvement per compute, endpoint local maximality, "
            "start-to-start variance, and trajectory convergence across optimizers "
            "belong to the companion full-optimizer comparison, not this predictor "
            "replay.",
            "",
            "## Reproduction and retained tables",
            "",
            "```bash",
            "cargo run -p optimizer-atoms --release -- \\",
            "  --config experiments/dev-gradient-ascent/optimizer-atoms/manifests/development-f10-16.json \\",
            "  --out /tmp/development-f10-16-replay",
            "",
            "uv run --script experiments/dev-gradient-ascent/optimizer-atoms/diagnose_replay.py \\",
            "  --dataset /tmp/development-f10-16-replay \\",
            "  --out /tmp/development-f10-16-replay-evidence",
            "```",
            "",
            "Machine-readable tables: `winner-causes-by-distance.csv`, "
            "`affine-causes.csv`, `candidate-lifetime.csv`, `rollback-summary.csv`, "
            "`gain-relative-error.csv`, `distance-screen.csv`, "
            "`coverage-by-distance.csv`, `candidate-miss-impact-by-distance.csv`, "
            "and `cost-coverage-by-scale.csv`.",
        ]
    )
    path.write_text("\n".join(lines) + "\n")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--dataset", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()
    if args.out.exists():
        raise ValueError(f"output already exists: {args.out}")
    args.out.mkdir(parents=True)
    paths = {
        name: args.dataset / name
        for name in (
            "pairs.jsonl",
            "atoms.jsonl",
            "winner-mechanisms.jsonl",
            "candidate-lifetimes.jsonl",
            "rollbacks.jsonl",
        )
    }
    data = {name: read_jsonl(path) for name, path in paths.items()}
    if any(not rows for rows in data.values()):
        raise ValueError("schema-v2 replay data is empty or incomplete")
    pairs = data["pairs.jsonl"]
    atoms = data["atoms.jsonl"]
    mechanisms = data["winner-mechanisms.jsonl"]
    lifetimes_raw = data["candidate-lifetimes.jsonl"]
    rollbacks_raw = data["rollbacks.jsonl"]

    coverage = coverage_rows(atoms)
    misses = miss_impact_rows(atoms)
    costs = cost_rows(atoms, pairs)
    winner_causes = winner_cause_rows(mechanisms)
    affine_causes = affine_cause_rows(atoms)
    lifetimes = lifetime_rows(lifetimes_raw)
    rollback_summary = rollback_rows(rollbacks_raw)
    gains = gain_rows(atoms)
    distances = distance_rows(atoms)

    tables = {
        "coverage-by-distance.csv": coverage,
        "candidate-miss-impact-by-distance.csv": misses,
        "cost-coverage-by-scale.csv": costs,
        "winner-causes-by-distance.csv": winner_causes,
        "affine-causes.csv": affine_causes,
        "candidate-lifetime.csv": lifetimes,
        "rollback-summary.csv": rollback_summary,
        "gain-relative-error.csv": gains,
        "distance-screen.csv": distances,
    }
    for name, rows in tables.items():
        write_csv(args.out / name, rows)

    plot_distance_population(pairs, args.out / "distance-population.png")
    plot_coverage(coverage, args.out / "coverage-vs-distance.png")
    plot_coverage_strata(atoms, args.out / "coverage-vs-distance-strata.png")
    plot_miss_impact(misses, args.out / "candidate-miss-impact-vs-distance.png")
    plot_tail(atoms, args.out / "error-tail-survival.png")
    plot_cost_coverage(costs, args.out / "cost-coverage-by-scale.png")
    plot_winner_causes(winner_causes, args.out / "winner-omission-causes.png")
    plot_affine_causes(affine_causes, args.out / "affine-failure-causes.png")
    plot_lifetime(lifetimes, args.out / "candidate-lifetime.png")
    plot_rollback(rollbacks_raw, args.out / "rollback-lead-time.png")
    plot_distances(atoms, args.out / "distance-diagnostics.png")
    write_report(
        args.out / "REPORT.md",
        args.dataset,
        pairs,
        atoms,
        mechanisms,
        affine_causes,
        lifetimes,
        rollback_summary,
        gains,
        distances,
    )
    (args.out / "analysis.json").write_text(
        json.dumps(
            {
                "schema_version": 2,
                "dataset": str(args.dataset),
                "inputs": {
                    name: {
                        "rows": len(data[name]),
                        "sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
                    }
                    for name, path in paths.items()
                },
                "material_error_threshold": 1.0e-2,
                "lifetime_error_threshold": 1.0e-3,
                "gain_ratio_floor": 1.0e-3,
                "claim_boundary": (
                    "Selected development trajectories from 16 starts and three "
                    "optimizers; predictor mechanisms, not full optimizer performance."
                ),
            },
            indent=2,
        )
        + "\n"
    )
    print(f"wrote schema-v2 diagnostic report from {len(atoms)} atom rows")


if __name__ == "__main__":
    main()
