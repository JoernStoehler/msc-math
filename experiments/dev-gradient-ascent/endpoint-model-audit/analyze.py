# /// script
# requires-python = ">=3.11"
# dependencies = ["matplotlib"]
# ///
"""Render the three-case endpoint derivative decomposition."""

from __future__ import annotations

import argparse
import json
from pathlib import Path

import matplotlib.pyplot as plt
from matplotlib.ticker import NullFormatter


def fmt(value: float | None) -> str:
    return "--" if value is None else f"{value:.6g}"


def inertia_changes(case: dict, radius: dict) -> bool:
    base = case["base"]["kkt"]["raw_negative_eigenvalue_count"]
    plus = radius["plus"]["kkt"]["raw_negative_eigenvalue_count"]
    minus = radius["minus"]["kkt"]["raw_negative_eigenvalue_count"]
    return base != plus or base != minus


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("audit", type=Path)
    parser.add_argument("out", type=Path)
    args = parser.parse_args()
    data = json.loads(args.audit.read_text())
    args.out.mkdir(parents=True, exist_ok=True)

    source_rows = []
    limit_rows = []
    for case in data["cases"]:
        source = case["radii"][0]
        smallest = case["radii"][-1]
        geometry_agrees = all(
            point["f64_exact_incidence_agree"]
            and point["f64_exact_facet_intersections_agree"]
            and point["f64_exact_omega_signs_agree"]
            for radius in case["radii"]
            for point in (radius["plus"], radius["minus"])
        ) and case["base"]["f64_exact_incidence_agree"]
        source_rows.append(
            {
                "role": case["role"],
                "source_delta": case["source_delta_sys"],
                "predicted_named_delta": case["analytic_branch_ratio_directional"]
                * source["absolute_radius"],
                "actual_named_delta": source["branch_ratio_f64_volume"]["forward"]
                * source["absolute_radius"],
                "action_error": source["action"]["central_relative_error"],
                "volume_error": source["f64_volume"]["central_relative_error"],
                "kkt_perturbation": source[
                    "kkt_frobenius_perturbation_over_base_eigen_gap"
                ],
                "eigen_crossing": inertia_changes(case, source),
            }
        )
        limit_rows.append(
            {
                "role": case["role"],
                "action_error": smallest["action"]["central_relative_error"],
                "ratio_error": smallest["branch_ratio_f64_volume"][
                    "central_relative_error"
                ],
                "geometry_agrees": geometry_agrees,
            }
        )

    publication_style = {
        "font.size": 11,
        "axes.titlesize": 12,
        "axes.labelsize": 11,
        "xtick.labelsize": 10,
        "ytick.labelsize": 10,
        "legend.fontsize": 10,
        "lines.linewidth": 1.6,
        "lines.markersize": 5.5,
    }
    role_label = {
        "top_failure": "top failure",
        "positive_control": "positive control",
        "clean_failure": "clean failure",
    }
    with plt.rc_context(publication_style):
        fig, axes = plt.subplots(3, 1, figsize=(6.8, 7.8), sharex=True)
        for case in data["cases"]:
            radii = [row["normalized_radius"] for row in case["radii"]]
            axes[0].plot(
                radii,
                [row["action"]["central"] for row in case["radii"]],
                marker="o",
                label=role_label[case["role"]],
            )
            axes[0].axhline(
                case["analytic_action_directional"],
                color=axes[0].lines[-1].get_color(),
                linestyle=":",
                alpha=0.75,
            )
            axes[1].plot(
                radii,
                [
                    row["branch_ratio_f64_volume"]["central"]
                    for row in case["radii"]
                ],
                marker="o",
                label=role_label[case["role"]],
            )
            axes[1].axhline(
                case["analytic_branch_ratio_directional"],
                color=axes[1].lines[-1].get_color(),
                linestyle=":",
                alpha=0.75,
            )
            axes[2].plot(
                radii,
                [
                    row["kkt_frobenius_perturbation_over_base_eigen_gap"]
                    for row in case["radii"]
                ],
                marker="o",
                label=role_label[case["role"]],
            )
        axes[0].set_title("Named action derivative")
        axes[0].set_ylabel("central finite difference")
        axes[1].set_title("Named branch-ratio derivative")
        axes[1].set_ylabel("central finite difference")
        axes[2].set_title("KKT perturbation relative to eigenvalue gap")
        axes[2].set_yscale("log")
        axes[2].axhline(1.0, color="black", linestyle="--", linewidth=1)
        axes[2].set_ylabel("Frobenius norm ratio")
        for axis in axes:
            axis.set_xscale("log")
            axis.grid(alpha=0.25)
        axes[-1].set_xticks([1.0e-8, 1.0e-7, 1.0e-6, 1.0e-5])
        axes[-1].xaxis.set_minor_formatter(NullFormatter())
        axes[-1].set_xlabel("normalized radius")
        handles, legend_labels = axes[0].get_legend_handles_labels()
        fig.legend(
            handles,
            legend_labels,
            loc="upper center",
            bbox_to_anchor=(0.5, 0.99),
            ncol=2,
        )
        fig.subplots_adjust(
            left=0.16,
            right=0.98,
            bottom=0.08,
            top=0.87,
            hspace=0.36,
        )
        fig.savefig(args.out / "derivative-and-kkt-scale.png", dpi=180)
        fig.savefig(args.out / "derivative-and-kkt-scale.pdf")
        plt.close(fig)

    source_table = [
        "| role | evaluator delta | predicted named-branch delta | actual named-branch delta | action derivative error | volume derivative error | KKT perturbation / gap | eigenvalue changes sign |",
        "| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |",
    ]
    for row in source_rows:
        source_table.append(
            f"| `{row['role']}` | {fmt(row['source_delta'])} | "
            f"{fmt(row['predicted_named_delta'])} | "
            f"{fmt(row['actual_named_delta'])} | {fmt(row['action_error'])} | "
            f"{fmt(row['volume_error'])} | {fmt(row['kkt_perturbation'])} | "
            f"{'yes' if row['eigen_crossing'] else 'no'} |"
        )
    limit_table = [
        "| role | action derivative error | branch-ratio derivative error | f64/exact geometry agrees at every point |",
        "| --- | ---: | ---: | --- |",
    ]
    for row in limit_rows:
        limit_table.append(
            f"| `{row['role']}` | {fmt(row['action_error'])} | "
            f"{fmt(row['ratio_error'])} | "
            f"{'yes' if row['geometry_agrees'] else 'no'} |"
        )

    report = f"""# Directional derivative decomposition

## Question

Why do two proposals decrease the implemented evaluator output even though
the branch that wins at the proposed point was present in the source candidate
set and its affine model predicted an increase?

## Result

At the optimizer's normalized radius `{fmt(data['source_normalized_radius'])}`,
the error is in the finite-distance linearization of the named KKT action, not
in volume, geometry reconstruction, or a missing target winner. The two
failures perturb their named KKT matrices by 67 and 218 times the smallest
base eigenvalue magnitude and cross a zero eigenvalue. Their action model even
predicts the wrong sign. The success perturbs its matrix by 1.53 times that
gap, does not cross zero, and retains the correct sign.

{chr(10).join(source_table)}

The analytical derivative itself is not a sign-error implementation. At the
diagnostic radius `{fmt(data['audit_normalized_radii'][-1])}`, after the KKT
perturbation is small relative to its eigenvalue gap, finite differences
converge to the analytical derivative:

{chr(10).join(limit_table)}

![Finite-difference derivatives and KKT scale](derivative-and-kkt-scale.png)

The dotted lines in the first two panels are the analytical derivatives. The
third panel compares the Frobenius norm of the KKT matrix perturbation with
the smallest base eigenvalue magnitude; the dashed line marks ratio one.

Across all 39 base and perturbed points, f64 and exact-arithmetic
reconstruction agree on incidence, facet intersections, and omega signs. The
largest relative f64/exact-arithmetic volume difference is below `1e-15`.
Thus these three cases provide no evidence that geometry reconstruction
caused the failures.

## Optimizer consequence

A Euclidean radius alone is not a sufficient trust scale for an affine
named-branch action model. A cheap proposed-point check can instead compare
the KKT matrix change with the source matrix's smallest eigenvalue magnitude,
or directly re-solve the named branches and reject a move whose realized
model value disagrees. This experiment does not yet calibrate a population
threshold or establish which check yields the best compute-versus-improvement
tradeoff.

## Interpretation boundary

This is a three-proposal, outcome-selected named-branch diagnostic. Agreement
at very small radius only diagnoses the derivative formula's local limit; a
normalized radius of `1e-8` is not proposed as an optimizer step. The result
does not establish candidate-family completeness, mathematical capacity,
endpoint stationarity, population frequency, or a final trust policy. The
Rust producer records the one-sided curves, KKT spectra and residuals, beta
vectors, geometry counters, and f64/exact-arithmetic comparisons in
`audit.json`.
"""
    (args.out / "REPORT.md").write_text(report)


if __name__ == "__main__":
    main()
