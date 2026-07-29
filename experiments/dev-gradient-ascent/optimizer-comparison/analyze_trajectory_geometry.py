# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "matplotlib>=3.8",
#   "numpy>=1.26",
# ]
# ///
"""Compare matched optimizer paths in ambient and linearized quotient coordinates."""

from __future__ import annotations

import argparse
import csv
import json
from collections import defaultdict
from itertools import combinations
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


def read_jsonl(path: Path) -> list[dict]:
    with path.open() as stream:
        return [json.loads(line) for line in stream if line.strip()]


def state_at_budget(run, run_rounds, evaluations, budget):
    evaluation_id = run["initial_evaluation_id"]
    for row in sorted(run_rounds, key=lambda item: item["round_index"]):
        if float(row.get("charged_compute_ms_after", 0.0)) > budget:
            break
        evaluation_id = row["best_evaluation_id_after"]
    return evaluations[evaluation_id]


def normalized_distance(left, right, initial):
    left = np.asarray(left, dtype=float)
    right = np.asarray(right, dtype=float)
    initial = np.asarray(initial, dtype=float)
    return float(np.linalg.norm(left - right) / np.linalg.norm(initial))


def sp4_basis():
    result = []
    for row in range(2):
        for col in range(2):
            matrix = np.zeros((4, 4))
            matrix[row, col] = 1.0
            matrix[2 + col, 2 + row] = -1.0
            result.append(matrix)
    for row, col in ((0, 0), (0, 1), (1, 1)):
        matrix = np.zeros((4, 4))
        matrix[row, 2 + col] = 1.0
        matrix[col, 2 + row] = 1.0
        result.append(matrix)
    for row, col in ((0, 0), (0, 1), (1, 1)):
        matrix = np.zeros((4, 4))
        matrix[2 + row, col] = 1.0
        matrix[2 + col, row] = 1.0
        result.append(matrix)
    return result


def symmetry_tangent_basis(dual_flat):
    duals = np.asarray(dual_flat, dtype=float).reshape((-1, 4))
    generators = []
    for coordinate in range(4):
        generators.append((-duals[:, coordinate, None] * duals).reshape(-1))
    generators.append((-duals).reshape(-1))
    for matrix in sp4_basis():
        generators.append((-duals @ matrix).reshape(-1))
    _, singular_values, right = np.linalg.svd(np.asarray(generators), full_matrices=False)
    tolerance = 1e-11 * max(singular_values[0], 1.0)
    rank = int(np.sum(singular_values > tolerance))
    if rank != 15:
        raise ValueError(f"expected symmetry-tangent rank 15, got {rank}")
    return right[:rank]


def dimension_for_fraction(cumulative, fraction):
    return int(np.searchsorted(cumulative, fraction, side="left") + 1)


def trajectory_dimensions(states, evaluations, by_key, starts, algorithms, budgets):
    rows = []
    spectra = []
    nonzero_budgets = [budget for budget in budgets if budget > 0]
    for start in starts:
        initial = evaluations[by_key[(start, algorithms[0])]["initial_evaluation_id"]]
        initial_flat = np.asarray(initial["dual_flat"], dtype=float)
        orbit_basis = symmetry_tangent_basis(initial_flat)
        displacements = []
        labels = []
        for algorithm in algorithms:
            for budget in nonzero_budgets:
                point = np.asarray(states[(start, algorithm, budget)]["dual_flat"], dtype=float)
                displacement = point - initial_flat
                displacement -= orbit_basis.T @ (orbit_basis @ displacement)
                displacements.append(displacement)
                labels.append((algorithm, budget))
        # Identical later checkpoints from a stopped run should not receive extra
        # statistical weight merely because they were recorded repeatedly.
        unique = {}
        for label, displacement in zip(labels, displacements):
            key = tuple(displacement)
            unique.setdefault(key, (label, displacement))
        matrix = np.asarray([item[1] for item in unique.values()])
        singular_values = np.linalg.svd(matrix, compute_uv=False)
        variances = singular_values**2
        total = float(np.sum(variances))
        if total == 0.0:
            cumulative = np.ones(1)
            rank = 0
        else:
            cumulative = np.cumsum(variances) / total
            rank = int(np.sum(singular_values > 1e-11 * singular_values[0]))
        row = {
            "start_id": start,
            "recorded_nonzero_checkpoints": len(displacements),
            "unique_quotient_displacements": len(unique),
            "linearized_quotient_dimension": initial_flat.size - 15,
            "observed_rank": rank,
            "components_for_90_percent": dimension_for_fraction(cumulative, 0.90),
            "components_for_95_percent": dimension_for_fraction(cumulative, 0.95),
            "components_for_99_percent": dimension_for_fraction(cumulative, 0.99),
            "first_component_fraction": float(cumulative[0]),
        }
        rows.append(row)
        for index, cumulative_fraction in enumerate(cumulative, start=1):
            spectra.append(
                {
                    "start_id": start,
                    "component_count": index,
                    "cumulative_variance_fraction": float(cumulative_fraction),
                }
            )
    return rows, spectra


def dimension_summary(rows):
    result = {}
    for key in (
        "unique_quotient_displacements",
        "observed_rank",
        "components_for_90_percent",
        "components_for_95_percent",
        "components_for_99_percent",
        "first_component_fraction",
    ):
        values = np.asarray([row[key] for row in rows], dtype=float)
        result[key] = {
            "median": float(np.median(values)),
            "q10": float(np.quantile(values, 0.1)),
            "q90": float(np.quantile(values, 0.9)),
        }
    return result


def plot_dimension_spectrum(spectra, path):
    by_component = defaultdict(list)
    for row in spectra:
        by_component[row["component_count"]].append(
            row["cumulative_variance_fraction"]
        )
    components = sorted(by_component)
    median = [np.median(by_component[index]) for index in components]
    q10 = [np.quantile(by_component[index], 0.1) for index in components]
    q90 = [np.quantile(by_component[index], 0.9) for index in components]
    figure, axis = plt.subplots(figsize=(7.2, 4.5))
    axis.plot(components, median, marker="o", markersize=3)
    axis.fill_between(components, q10, q90, alpha=0.2, label="10–90% across starts")
    for fraction in (0.9, 0.95, 0.99):
        axis.axhline(fraction, color="black", linewidth=0.7, alpha=0.35)
    axis.set_xlabel("principal components")
    axis.set_ylabel("cumulative squared displacement")
    axis.set_ylim(0, 1.01)
    axis.grid(alpha=0.2)
    axis.legend()
    figure.tight_layout()
    figure.savefig(path, dpi=180)
    figure.savefig(path.with_suffix(".pdf"))
    plt.close(figure)


def analyze(dataset: Path, budgets: list[float]):
    runs = read_jsonl(dataset / "runs.jsonl")
    evaluations = {
        row["evaluation_id"]: row for row in read_jsonl(dataset / "evaluations.jsonl")
    }
    rounds = defaultdict(list)
    for row in read_jsonl(dataset / "rounds.jsonl"):
        rounds[row["run_id"]].append(row)
    by_key = {(row["start_id"], row["algorithm_id"]): row for row in runs}
    starts = sorted({row["start_id"] for row in runs})
    algorithms = sorted({row["algorithm_id"] for row in runs})
    if len(by_key) != len(starts) * len(algorithms):
        raise ValueError("trajectory geometry requires a complete matched start/algorithm grid")
    states = {}
    for start in starts:
        for algorithm in algorithms:
            run = by_key[(start, algorithm)]
            for budget in budgets:
                states[(start, algorithm, budget)] = state_at_budget(
                    run, rounds[run["run_id"]], evaluations, budget
                )
    pair_rows = []
    for start in starts:
        initial = evaluations[by_key[(start, algorithms[0])]["initial_evaluation_id"]]
        for left, right in combinations(algorithms, 2):
            for budget in budgets:
                left_state = states[(start, left, budget)]
                right_state = states[(start, right, budget)]
                pair_rows.append(
                    {
                        "start_id": start,
                        "left_algorithm": left,
                        "right_algorithm": right,
                        "compute_budget_ms": budget,
                        "ambient_distance_over_initial_norm": normalized_distance(
                            left_state["dual_flat"],
                            right_state["dual_flat"],
                            initial["dual_flat"],
                        ),
                        "left_sys": left_state["sys"],
                        "right_sys": right_state["sys"],
                        "absolute_sys_difference": abs(
                            float(left_state["sys"]) - float(right_state["sys"])
                        ),
                        "same_winning_sigma": (
                            left_state.get("winning_sigma")
                            == right_state.get("winning_sigma")
                        ),
                    }
                )
    movement = []
    for start in starts:
        initial = evaluations[by_key[(start, algorithms[0])]["initial_evaluation_id"]]
        for algorithm in algorithms:
            final = states[(start, algorithm, budgets[-1])]
            for budget in budgets:
                state = states[(start, algorithm, budget)]
                movement.append(
                    {
                        "start_id": start,
                        "algorithm_id": algorithm,
                        "compute_budget_ms": budget,
                        "distance_from_start_over_initial_norm": normalized_distance(
                            state["dual_flat"], initial["dual_flat"], initial["dual_flat"]
                        ),
                        "distance_to_recorded_final_over_initial_norm": normalized_distance(
                            state["dual_flat"], final["dual_flat"], initial["dual_flat"]
                        ),
                        "sys": state["sys"],
                        "remaining_sys_gain_to_recorded_final": max(
                            0.0, float(final["sys"]) - float(state["sys"])
                        ),
                    }
                )
    dimensions, spectra = trajectory_dimensions(
        states, evaluations, by_key, starts, algorithms, budgets
    )
    return pair_rows, movement, dimensions, spectra, starts, algorithms


def summaries(rows):
    groups = defaultdict(list)
    for row in rows:
        groups[
            (row["left_algorithm"], row["right_algorithm"], row["compute_budget_ms"])
        ].append(row)
    result = []
    for (left, right, budget), group in sorted(groups.items()):
        distances = np.asarray(
            [row["ambient_distance_over_initial_norm"] for row in group], dtype=float
        )
        sys_differences = np.asarray(
            [row["absolute_sys_difference"] for row in group], dtype=float
        )
        result.append(
            {
                "left_algorithm": left,
                "right_algorithm": right,
                "compute_budget_ms": budget,
                "starts": len(group),
                "median_ambient_distance": float(np.median(distances)),
                "q10_ambient_distance": float(np.quantile(distances, 0.1)),
                "q90_ambient_distance": float(np.quantile(distances, 0.9)),
                "median_absolute_sys_difference": float(np.median(sys_differences)),
                "same_winning_sigma_fraction": float(
                    np.mean([row["same_winning_sigma"] for row in group])
                ),
            }
        )
    return result


def movement_summaries(rows):
    groups = defaultdict(list)
    for row in rows:
        groups[(row["algorithm_id"], row["compute_budget_ms"])].append(row)
    result = []
    for (algorithm, budget), group in sorted(groups.items()):
        distances = np.asarray(
            [row["distance_from_start_over_initial_norm"] for row in group],
            dtype=float,
        )
        distances_to_final = np.asarray(
            [row["distance_to_recorded_final_over_initial_norm"] for row in group],
            dtype=float,
        )
        remaining_gain = np.asarray(
            [row["remaining_sys_gain_to_recorded_final"] for row in group],
            dtype=float,
        )
        result.append(
            {
                "algorithm_id": algorithm,
                "compute_budget_ms": budget,
                "starts": len(group),
                "median_distance_from_start": float(np.median(distances)),
                "q10_distance_from_start": float(np.quantile(distances, 0.1)),
                "q90_distance_from_start": float(np.quantile(distances, 0.9)),
                "median_distance_to_recorded_final": float(
                    np.median(distances_to_final)
                ),
                "q10_distance_to_recorded_final": float(
                    np.quantile(distances_to_final, 0.1)
                ),
                "q90_distance_to_recorded_final": float(
                    np.quantile(distances_to_final, 0.9)
                ),
                "median_remaining_sys_gain": float(np.median(remaining_gain)),
                "q10_remaining_sys_gain": float(np.quantile(remaining_gain, 0.1)),
                "q90_remaining_sys_gain": float(np.quantile(remaining_gain, 0.9)),
            }
        )
    return result


def write_csv(path, rows):
    with path.open("w", newline="") as stream:
        writer = csv.DictWriter(
            stream, fieldnames=list(rows[0]), lineterminator="\n"
        )
        writer.writeheader()
        writer.writerows(rows)


def plot_pairwise(summary, path):
    pairs = sorted({(row["left_algorithm"], row["right_algorithm"]) for row in summary})
    figure, axis = plt.subplots(figsize=(8.2, 5.0))
    for left, right in pairs:
        group = [
            row
            for row in summary
            if row["left_algorithm"] == left and row["right_algorithm"] == right
        ]
        axis.plot(
            [row["compute_budget_ms"] for row in group],
            [row["median_ambient_distance"] for row in group],
            marker="o",
            label=f"{left} / {right}",
        )
    axis.set_xlabel("measured compute budget (ms)")
    axis.set_ylabel("median ambient distance / initial norm")
    axis.set_xscale("symlog", linthresh=1.0)
    axis.grid(alpha=0.2)
    axis.legend(fontsize=6, ncol=2)
    figure.tight_layout()
    figure.savefig(path, dpi=180)
    figure.savefig(path.with_suffix(".pdf"))
    plt.close(figure)


def plot_endpoint_matrix(summary, algorithms, budget, path):
    index = {algorithm: position for position, algorithm in enumerate(algorithms)}
    matrix = np.zeros((len(algorithms), len(algorithms)))
    for row in summary:
        if row["compute_budget_ms"] != budget:
            continue
        i = index[row["left_algorithm"]]
        j = index[row["right_algorithm"]]
        matrix[i, j] = matrix[j, i] = row["median_ambient_distance"]
    figure, axis = plt.subplots(figsize=(8.0, 6.8))
    image = axis.imshow(matrix, cmap="viridis")
    axis.set_xticks(range(len(algorithms)), algorithms, rotation=40, ha="right", fontsize=7)
    axis.set_yticks(range(len(algorithms)), algorithms, fontsize=7)
    for i in range(len(algorithms)):
        for j in range(len(algorithms)):
            axis.text(j, i, f"{matrix[i, j]:.2f}", ha="center", va="center", fontsize=7)
    figure.colorbar(image, ax=axis, label="median ambient distance / initial norm")
    axis.set_title(f"Matched endpoints at {budget:g} ms")
    figure.tight_layout()
    figure.savefig(path, dpi=180)
    figure.savefig(path.with_suffix(".pdf"))
    plt.close(figure)


def plot_movement(summary, path):
    algorithms = sorted({row["algorithm_id"] for row in summary})
    figure, axes = plt.subplots(1, 2, figsize=(11.0, 4.6))
    for algorithm in algorithms:
        rows = sorted(
            (row for row in summary if row["algorithm_id"] == algorithm),
            key=lambda row: row["compute_budget_ms"],
        )
        x = [row["compute_budget_ms"] for row in rows]
        for axis, key, low, high in (
            (
                axes[0],
                "median_distance_from_start",
                "q10_distance_from_start",
                "q90_distance_from_start",
            ),
            (
                axes[1],
                "median_distance_to_recorded_final",
                "q10_distance_to_recorded_final",
                "q90_distance_to_recorded_final",
            ),
        ):
            middle = np.asarray([row[key] for row in rows])
            lower = np.asarray([row[low] for row in rows])
            upper = np.asarray([row[high] for row in rows])
            axis.plot(x, middle, marker="o", markersize=3, label=algorithm)
            axis.fill_between(x, lower, upper, alpha=0.08)
    axes[0].set_ylabel("distance from start / initial norm")
    axes[1].set_ylabel("distance to recorded final / initial norm")
    for axis in axes:
        axis.set_xlabel("measured compute budget (ms)")
        axis.set_xscale("symlog", linthresh=1.0)
        axis.grid(alpha=0.2)
    axes[1].legend(fontsize=6)
    figure.tight_layout()
    figure.savefig(path, dpi=180)
    figure.savefig(path.with_suffix(".pdf"))
    plt.close(figure)


def plot_sample_trajectories(states, starts, algorithms, budgets, path):
    history_final = sorted(
        (
            (float(states[(start, "history-baseline", budgets[-1])]["sys"]), start)
            for start in starts
        )
    )
    positions = np.linspace(0, len(history_final) - 1, 8).round().astype(int)
    selected = [history_final[position][1] for position in positions]
    figure, axes = plt.subplots(4, 2, figsize=(10.5, 10.0), sharex=True)
    for axis, start in zip(axes.flat, selected):
        for algorithm in algorithms:
            axis.plot(
                budgets,
                [states[(start, algorithm, budget)]["sys"] for budget in budgets],
                marker="o",
                markersize=2.5,
                label=algorithm,
            )
        axis.set_title(start, fontsize=8)
        axis.grid(alpha=0.2)
    for axis in axes[-1]:
        axis.set_xlabel("measured compute budget (ms)")
        axis.set_xscale("symlog", linthresh=1.0)
    for axis in axes[:, 0]:
        axis.set_ylabel("best sys")
    axes[0, 1].legend(fontsize=5.5, ncol=2)
    figure.suptitle(
        "Eight history-endpoint quantiles; selected before viewing other methods",
        fontsize=10,
    )
    figure.tight_layout()
    figure.savefig(path, dpi=180)
    figure.savefig(path.with_suffix(".pdf"))
    plt.close(figure)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--dataset", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    parser.add_argument("--budgets", default="0,100,250,500,750,1000")
    parser.add_argument("--overwrite", action="store_true")
    args = parser.parse_args()
    budgets = sorted({float(value) for value in args.budgets.split(",")})
    if args.out.exists() and not args.overwrite:
        raise ValueError(f"output already exists: {args.out}")
    pair_rows, movement, dimensions, spectra, starts, algorithms = analyze(
        args.dataset, budgets
    )
    summary = summaries(pair_rows)
    movement_summary = movement_summaries(movement)
    dimensions_summary = dimension_summary(dimensions)
    args.out.mkdir(parents=True, exist_ok=args.overwrite)
    write_csv(args.out / "pairwise-states.csv", pair_rows)
    write_csv(args.out / "pairwise-summary.csv", summary)
    write_csv(args.out / "movement-from-start.csv", movement)
    write_csv(args.out / "movement-summary.csv", movement_summary)
    write_csv(args.out / "trajectory-dimension-by-start.csv", dimensions)
    write_csv(args.out / "trajectory-dimension-spectrum.csv", spectra)
    plot_pairwise(summary, args.out / "pairwise-distance-vs-compute.png")
    plot_endpoint_matrix(summary, algorithms, budgets[-1], args.out / "endpoint-distance.png")
    plot_movement(movement_summary, args.out / "movement-vs-compute.png")
    plot_sample_trajectories(
        {
            (row["start_id"], row["algorithm_id"], row["compute_budget_ms"]): row
            for row in movement
        },
        starts,
        algorithms,
        budgets,
        args.out / "sample-best-sys-trajectories.png",
    )
    plot_dimension_spectrum(spectra, args.out / "trajectory-dimension-spectrum.png")
    endpoint = [row for row in summary if row["compute_budget_ms"] == budgets[-1]]
    closest = min(endpoint, key=lambda row: row["median_ambient_distance"])
    farthest = max(endpoint, key=lambda row: row["median_ambient_distance"])
    endpoint_movement = [
        row
        for row in movement_summary
        if row["compute_budget_ms"] == budgets[-1]
    ]
    midpoint_budget = budgets[len(budgets) // 2]
    midpoint_movement = [
        row
        for row in movement_summary
        if row["compute_budget_ms"] == midpoint_budget
    ]
    history_pairs = [
        row
        for row in endpoint
        if "history-baseline"
        in (row["left_algorithm"], row["right_algorithm"])
    ]
    text = [
        "# Matched trajectory geometry",
        "",
        f"{len(algorithms)} algorithms on {len(starts)} matched starts were compared "
        "at fixed measured-compute checkpoints.",
        "",
        f"At {budgets[-1]:g} ms the closest median pair is "
        f"`{closest['left_algorithm']}` / `{closest['right_algorithm']}` "
        f"({closest['median_ambient_distance']:.4g} initial norms); the farthest is "
        f"`{farthest['left_algorithm']}` / `{farthest['right_algorithm']}` "
        f"({farthest['median_ambient_distance']:.4g}).",
        "",
        "## Endpoint comparisons involving branch history",
        "",
        "| other algorithm | median ambient distance | 10–90% | median absolute sys difference | same winning word |",
        "|---|---:|---:|---:|---:|",
    ]
    for row in sorted(history_pairs, key=lambda item: item["median_ambient_distance"]):
        other = (
            row["right_algorithm"]
            if row["left_algorithm"] == "history-baseline"
            else row["left_algorithm"]
        )
        text.append(
            f"| {other} | {row['median_ambient_distance']:.4g} | "
            f"{row['q10_ambient_distance']:.4g}–{row['q90_ambient_distance']:.4g} | "
            f"{row['median_absolute_sys_difference']:.4g} | "
            f"{row['same_winning_sigma_fraction']:.1%} |"
        )
    text += [
        "",
        "## Movement from the matched start",
        "",
        "| algorithm | median movement / initial norm | 10–90% |",
        "|---|---:|---:|",
    ]
    for row in sorted(
        endpoint_movement,
        key=lambda item: item["median_distance_from_start"],
        reverse=True,
    ):
        text.append(
            f"| {row['algorithm_id']} | {row['median_distance_from_start']:.4g} | "
            f"{row['q10_distance_from_start']:.4g}–{row['q90_distance_from_start']:.4g} |"
        )
    text += [
        "",
        f"## Distance to the recorded endpoint at {midpoint_budget:g} ms",
        "",
        "| algorithm | median coordinate distance / initial norm | median later sys gain |",
        "|---|---:|---:|",
    ]
    for row in sorted(
        midpoint_movement,
        key=lambda item: item["median_distance_to_recorded_final"],
    ):
        text.append(
            f"| {row['algorithm_id']} | "
            f"{row['median_distance_to_recorded_final']:.4g} | "
            f"{row['median_remaining_sys_gain']:.4g} |"
        )
    text += [
        "",
        "The endpoint here is only the best state recorded by the end of this run, "
        "not a certified local maximum. A small value therefore diagnoses an early "
        "plateau relative to the method's own one-second result; it does not show "
        "successful optimization. `movement-vs-compute.png` gives the full curves.",
        "",
        "## Dimension of the matched trajectory cloud",
        "",
        "For each start, all nonzero-compute checkpoints were projected away from "
        "the 15-dimensional symmetry tangent at that start. Repeated identical "
        "checkpoints after an optimizer stopped were counted once. Principal "
        "components then summarized the resulting point cloud in the 25-dimensional "
        "linearized quotient slice.",
        "",
        "| quantity | median | 10–90% across starts |",
        "|---|---:|---:|",
    ]
    for key, label in (
        ("unique_quotient_displacements", "unique recorded points"),
        ("observed_rank", "observed linear rank"),
        ("components_for_90_percent", "components for 90%"),
        ("components_for_95_percent", "components for 95%"),
        ("components_for_99_percent", "components for 99%"),
        ("first_component_fraction", "fraction in first component"),
    ):
        row = dimensions_summary[key]
        text.append(
            f"| {label} | {row['median']:.4g} | "
            f"{row['q10']:.4g}–{row['q90']:.4g} |"
        )
    text += [
        "",
        "The branch-aware methods reach similarly high objective values without "
        "following one common coordinate path: the four-anchor branch-history method "
        "is separated from the directional and gap variants by about one quarter of "
        "an initial-state norm at the median. The two single-branch gradient variants "
        "remain much closer to each other. This is separation, not a claim that every "
        "individual pair immediately diverges or that the paths reach distinct basins.",
        "",
        "These are ambient coordinate distances with matched facet labels. They show "
        "whether recorded paths separate, but they do not quotient the continuous "
        "symmetry group and therefore cannot establish distinct local maxima. The PCA "
        "uses only the tangent space at the start, so it is a local linear removal of "
        "symmetry directions rather than a global alignment of paths.",
    ]
    (args.out / "SUMMARY.md").write_text("\n".join(text) + "\n")
    print(f"compared {len(algorithms)} algorithms on {len(starts)} matched starts")


if __name__ == "__main__":
    main()
