# /// script
# requires-python = ">=3.12"
# dependencies = ["blake3", "matplotlib", "numpy"]
# ///

"""Validate, summarize, plot, and discuss the literal multi-start packet."""

from __future__ import annotations

import argparse
import hashlib
import json
import math
from pathlib import Path
from statistics import median

import matplotlib.pyplot as plt
from matplotlib.colors import SymLogNorm
import numpy as np
import blake3


OWNER = Path(__file__).resolve().parent
DEFAULT_SOURCE = OWNER.parents[1] / "sys-datascience/produce/random.jsonl"
DEFAULT_ARTIFACTS = OWNER / "artifacts/evaluation"
DEFAULT_DIAGNOSTIC = OWNER / "artifacts"
FIGURES = OWNER / "figures"
ETAS = [1e-5, 1e-4, 1e-3, 1e-2, 1e-1, 1.0]
MATERIAL_RELATIVE_GAIN = 0.01
NUMERICAL_TOLERANCE = 2e-11


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--source", type=Path, default=DEFAULT_SOURCE)
    parser.add_argument("--artifacts", type=Path, default=DEFAULT_ARTIFACTS)
    parser.add_argument("--diagnostic-artifacts", type=Path, default=DEFAULT_DIAGNOSTIC)
    parser.add_argument("--figures", type=Path, default=FIGURES)
    parser.add_argument("--facet-count", type=int, default=6)
    parser.add_argument("--start-count", type=int, default=6)
    parser.add_argument("--exclude-start-id", default="random_F6_s0_1")
    return parser.parse_args()


def read_json(path: Path) -> dict:
    with path.open() as stream:
        return json.load(stream)


def read_jsonl(path: Path) -> list[dict]:
    with path.open() as stream:
        return [json.loads(line) for line in stream]


def resolve_recorded_path(recorded: str, artifacts: Path) -> Path:
    path = Path(recorded)
    if path.is_absolute():
        return path
    repo_root = OWNER.parents[2]
    candidate = repo_root / path
    if candidate.exists():
        return candidate
    fallback = artifacts / "trajectories" / path.name
    if fallback.exists():
        return fallback
    raise AssertionError(f"recorded trajectory path does not exist: {recorded}")


def close(left: float, right: float, tol: float = NUMERICAL_TOLERANCE) -> bool:
    return abs(left - right) <= tol * (1.0 + abs(left) + abs(right))


def vectors_close(left: list[list[float]], right: list[list[float]]) -> bool:
    return len(left) == len(right) and all(
        close(x, y)
        for lv, rv in zip(left, right, strict=True)
        for x, y in zip(lv, rv, strict=True)
    )


def best_through(rows: list[dict], iteration: int) -> float:
    return max(
        row["sys"]
        for row in rows
        if row["iteration"] <= iteration and row["sys"] is not None
    )


def class_through(rows: list[dict], iteration: int, initial: float) -> str:
    failure = next((row for row in rows if row["failure"] is not None), None)
    if failure is not None and failure["iteration"] <= iteration:
        return "invalid"
    relative = (best_through(rows, iteration) - initial) / initial
    return "material_gain" if relative >= MATERIAL_RELATIVE_GAIN else "no_material_gain"


def validate_trajectory(
    rows: list[dict], summary: dict, source_duals: list[list[float]]
) -> dict:
    assert rows and rows[0]["iteration"] == 0 and rows[0]["role"] == "initial"
    assert all(row["eta"] == summary["eta"] for row in rows)
    assert all(row["state_valid"] == (row["sys"] is not None) for row in rows)
    assert vectors_close(rows[0]["dual_vertices_before"], source_duals)
    assert vectors_close(rows[0]["dual_vertices_after"], source_duals)
    assert len(rows) == summary["iterations_completed"] + 1 + int(
        summary["failure"] is not None
    )
    assert [row["iteration"] for row in rows] == list(range(len(rows)))

    initial = rows[0]["sys"]
    assert initial is not None and close(initial, summary["initial_sys"])
    best = initial
    best_iteration = 0
    increases = decreases = equal = switches = 0
    previous_valid = rows[0]
    previous_sigma = rows[0]["resulting_sigma"]
    for row in rows[1:]:
        eta = row["eta"]
        for gradient, da in zip(row["gradient"], row["da"], strict=True):
            for component, displacement in zip(gradient, da, strict=True):
                assert close(displacement, eta * component)
        expected_after = [
            [x + dx for x, dx in zip(before, da, strict=True)]
            for before, da in zip(row["dual_vertices_before"], row["da"], strict=True)
        ]
        assert vectors_close(expected_after, row["dual_vertices_after"])
        assert vectors_close(
            previous_valid["dual_vertices_after"], row["dual_vertices_before"]
        )
        assert row["selected_sigma"] == previous_sigma
        if row["sys"] is None:
            assert (
                row["role"] == "failure" and not row["state_valid"] and row["failure"]
            )
            continue
        delta = row["sys"] - previous_valid["sys"]
        assert close(delta, row["full_sys_delta"])
        if delta > 0:
            increases += 1
        elif delta < 0:
            decreases += 1
        else:
            equal += 1
        if row["resulting_sigma"] != previous_sigma:
            switches += 1
        previous_sigma = row["resulting_sigma"]
        if row["sys"] > best:
            best = row["sys"]
            best_iteration = row["iteration"]
        assert close(row["best_sys"], best)
        assert row["best_iteration"] == best_iteration
        previous_valid = row

    assert close(best, summary["best_sys"])
    assert best_iteration == summary["best_iteration"]
    assert increases == summary["full_sys_increases"]
    assert decreases == summary["full_sys_decreases"]
    assert equal == summary["full_sys_equal"]
    assert switches == summary["branch_switches"]
    assert summary["failure"] == rows[-1]["failure"]

    complete = (
        summary["failure"] is None
        and summary["iterations_completed"] == summary["requested_updates"]
    )
    failure_iteration = None if complete else rows[-1]["iteration"]
    last_valid = next(row["sys"] for row in reversed(rows) if row["sys"] is not None)
    assert close(last_valid, summary["final_sys"])
    if summary["iterations_completed"] >= 20:
        through_20 = best_through(rows, 20)
        assert close(summary["gain_through_iteration_20"], through_20 - initial)
        assert close(
            summary["additional_best_gain_iterations_21_100"], best - through_20
        )
    else:
        assert summary["gain_through_iteration_20"] is None
    relative_best_gain = (best - initial) / initial
    final_sys = last_valid if complete else None
    final_regret = best - final_sys if complete else None
    result = {
        "start_id": summary["start_id"],
        "eta": summary["eta"],
        "initial_sys": initial,
        "complete": complete,
        "failure": summary["failure"],
        "failure_iteration": failure_iteration,
        "iterations_completed": summary["iterations_completed"],
        "best_sys": best,
        "best_iteration": best_iteration,
        "best_gain": best - initial,
        "relative_best_gain": relative_best_gain,
        "last_valid_sys": last_valid,
        "final_sys": final_sys,
        "final_regret": final_regret,
        "relative_final_regret": None
        if final_regret is None
        else final_regret / initial,
        "branch_switches": switches,
        "full_sys_decreases": decreases,
    }
    full_class = (
        "invalid"
        if not complete
        else (
            "material_gain"
            if relative_best_gain >= MATERIAL_RELATIVE_GAIN
            else "no_material_gain"
        )
    )
    result["full_class"] = full_class
    for prefix in (8, 20):
        prefix_best = best_through(rows, prefix)
        prefix_class = class_through(rows, prefix, initial)
        result[f"relative_best_gain_at_{prefix}"] = (prefix_best - initial) / initial
        result[f"best_improved_after_{prefix}"] = (
            best > prefix_best + NUMERICAL_TOLERANCE
        )
        result[f"class_at_{prefix}"] = prefix_class
        result[f"class_disagrees_at_{prefix}"] = prefix_class != full_class
    return result


def quantile(values: list[float], probability: float) -> float | None:
    if not values:
        return None
    ordered = sorted(values)
    location = probability * (len(ordered) - 1)
    lower = math.floor(location)
    upper = math.ceil(location)
    if lower == upper:
        return ordered[lower]
    weight = location - lower
    return ordered[lower] * (1 - weight) + ordered[upper] * weight


def summarize(cells: list[dict], start_ids: list[str]) -> dict:
    per_eta = []
    for eta in ETAS:
        group = [cell for cell in cells if cell["eta"] == eta]
        complete = [cell for cell in group if cell["complete"]]
        gains = [cell["relative_best_gain"] for cell in group]
        regrets = [cell["relative_final_regret"] for cell in complete]
        per_eta.append(
            {
                "eta": eta,
                "n_starts": len(group),
                "complete_count": len(complete),
                "invalid_count": len(group) - len(complete),
                "material_gain_count": sum(
                    cell["relative_best_gain"] >= MATERIAL_RELATIVE_GAIN
                    for cell in group
                ),
                "best_gain_relative_q25": quantile(gains, 0.25),
                "best_gain_relative_median": median(gains),
                "best_gain_relative_q75": quantile(gains, 0.75),
                "best_improved_after_8_count": sum(
                    cell["best_improved_after_8"] for cell in group
                ),
                "best_improved_after_20_count": sum(
                    cell["best_improved_after_20"] for cell in group
                ),
                "prefix_8_class_disagreement_count": sum(
                    cell["class_disagrees_at_8"] for cell in group
                ),
                "prefix_20_class_disagreement_count": sum(
                    cell["class_disagrees_at_20"] for cell in group
                ),
                "completed_with_any_final_regret_count": sum(
                    cell["final_regret"] > NUMERICAL_TOLERANCE for cell in complete
                ),
                "completed_with_material_final_regret_count": sum(
                    cell["relative_final_regret"] >= MATERIAL_RELATIVE_GAIN
                    for cell in complete
                ),
                "final_regret_relative_median_completed": median(regrets)
                if regrets
                else None,
                "branch_switches_median": median(
                    cell["branch_switches"] for cell in group
                ),
            }
        )
    per_start = []
    for start_id in start_ids:
        group = [cell for cell in cells if cell["start_id"] == start_id]
        best = max(group, key=lambda cell: cell["relative_best_gain"])
        per_start.append(
            {
                "start_id": start_id,
                "initial_sys": group[0]["initial_sys"],
                "rates_with_material_gain": sum(
                    cell["relative_best_gain"] >= MATERIAL_RELATIVE_GAIN
                    for cell in group
                ),
                "rates_invalid": sum(not cell["complete"] for cell in group),
                "best_eta": best["eta"],
                "best_relative_gain": best["relative_best_gain"],
            }
        )
    complete = [cell for cell in cells if cell["complete"]]
    return {
        "material_relative_gain_threshold": MATERIAL_RELATIVE_GAIN,
        "cell_count": len(cells),
        "complete_cell_count": len(complete),
        "invalid_cell_count": len(cells) - len(complete),
        "start_count": len(start_ids),
        "starts_with_any_material_gain": sum(
            row["rates_with_material_gain"] > 0 for row in per_start
        ),
        "starts_with_any_invalid_rate": sum(
            row["rates_invalid"] > 0 for row in per_start
        ),
        "complete_cells_with_any_final_regret": sum(
            cell["final_regret"] > NUMERICAL_TOLERANCE for cell in complete
        ),
        "complete_cells_with_material_final_regret": sum(
            cell["relative_final_regret"] >= MATERIAL_RELATIVE_GAIN for cell in complete
        ),
        "prefix_8_class_disagreements": sum(
            cell["class_disagrees_at_8"] for cell in cells
        ),
        "prefix_20_class_disagreements": sum(
            cell["class_disagrees_at_20"] for cell in cells
        ),
        "best_improved_after_8": sum(cell["best_improved_after_8"] for cell in cells),
        "best_improved_after_20": sum(cell["best_improved_after_20"] for cell in cells),
        "per_eta": per_eta,
        "per_start": per_start,
    }


def eta_label(eta: float) -> str:
    exponent = round(math.log10(eta))
    return f"$10^{{{exponent}}}$"


def save(fig: plt.Figure, figures: Path, stem: str) -> None:
    figures.mkdir(parents=True, exist_ok=True)
    for extension in ("png", "pdf"):
        fig.savefig(figures / f"{stem}.{extension}", bbox_inches="tight")
    plt.close(fig)


def plot_paired(cells: list[dict], start_ids: list[str], figures: Path) -> None:
    gain = np.full((len(start_ids), len(ETAS)), np.nan)
    regret = np.full_like(gain, np.nan)
    lookup = {(cell["start_id"], cell["eta"]): cell for cell in cells}
    for row, start_id in enumerate(start_ids):
        for column, eta in enumerate(ETAS):
            cell = lookup[start_id, eta]
            gain[row, column] = 100 * cell["relative_best_gain"]
            if cell["complete"]:
                regret[row, column] = 100 * cell["relative_final_regret"]
    fig, axes = plt.subplots(1, 2, figsize=(10.8, 4.4), constrained_layout=True)
    panels = [
        (
            gain,
            "Best-so-far gain (% of initial sys)",
            "viridis",
            SymLogNorm(linthresh=1.0, vmin=0, vmax=np.nanmax(gain)),
        ),
        (regret, "Final regret (% of initial sys); gray = invalid", "magma", None),
    ]
    for ax, (matrix, title, cmap_name, norm) in zip(axes, panels, strict=True):
        cmap = plt.get_cmap(cmap_name).copy()
        cmap.set_bad("#dddddd")
        image = ax.imshow(matrix, aspect="auto", cmap=cmap, norm=norm)
        ax.set_xticks(range(len(ETAS)), [eta_label(eta) for eta in ETAS])
        ax.set_yticks(range(len(start_ids)), start_ids)
        ax.set_xlabel(r"Learning rate $\eta$")
        ax.set_title(title)
        fig.colorbar(image, ax=ax, shrink=0.78)
        for row, start_id in enumerate(start_ids):
            for column, eta in enumerate(ETAS):
                cell = lookup[start_id, eta]
                if np.isfinite(matrix[row, column]):
                    value = matrix[row, column]
                    text = f"{value:.1f}"
                    red, green, blue, _ = image.cmap(image.norm(value))
                    color = (
                        "black"
                        if 0.2126 * red + 0.7152 * green + 0.0722 * blue > 0.55
                        else "white"
                    )
                else:
                    text = f"fail@{cell['failure_iteration']}"
                    color = "black"
                ax.text(
                    column, row, text, ha="center", va="center", fontsize=7, color=color
                )
    axes[0].set_ylabel("Evaluation start (source order)")
    fig.suptitle(
        "Literal ascent paired across six pre-target random F=6 source-prefix starts"
    )
    save(fig, figures, "evaluation-paired-outcomes")


def plot_rates(analysis: dict, figures: Path) -> None:
    per_eta = analysis["per_eta"]
    x = np.arange(len(ETAS))
    fig, axes = plt.subplots(
        1, 3, figsize=(11.4, 3.6), sharex=True, constrained_layout=True
    )
    width = 0.36
    prefix_8 = [
        100 * row["prefix_8_class_disagreement_count"] / row["n_starts"]
        for row in per_eta
    ]
    prefix_20 = [
        100 * row["prefix_20_class_disagreement_count"] / row["n_starts"]
        for row in per_eta
    ]
    bars_8 = axes[0].bar(x - width / 2, prefix_8, width, label="8 iterations")
    bars_20 = axes[0].bar(x + width / 2, prefix_20, width, label="20 iterations")
    axes[0].set_title("Prefix gives different practical class")
    axes[0].legend(fontsize=8)
    invalid_rates = [100 * row["invalid_count"] / row["n_starts"] for row in per_eta]
    invalid_bars = axes[1].bar(x, invalid_rates, color="#c44e52")
    axes[1].set_title("Trajectory becomes invalid")
    retention_rates = [
        100 * row["completed_with_material_final_regret_count"] / row["complete_count"]
        if row["complete_count"]
        else 0
        for row in per_eta
    ]
    retention_bars = axes[2].bar(x, retention_rates, color="#4c72b0")
    axes[2].set_title("Final regret at least 1% of initial sys")
    for ax in axes:
        ax.set_xticks(x, [eta_label(eta) for eta in ETAS])
        ax.set_xlabel(r"$\eta$")
        ax.set_ylim(0, 112)
        ax.grid(axis="y", alpha=0.25)
    axes[0].set_ylabel("Empirical rate (%)")
    axes[0].bar_label(
        bars_8,
        [
            f"{row['prefix_8_class_disagreement_count']}/{row['n_starts']}"
            for row in per_eta
        ],
        padding=2,
        fontsize=7,
    )
    axes[0].bar_label(
        bars_20,
        [
            f"{row['prefix_20_class_disagreement_count']}/{row['n_starts']}"
            for row in per_eta
        ],
        padding=2,
        fontsize=7,
    )
    axes[1].bar_label(
        invalid_bars,
        [f"{row['invalid_count']}/{row['n_starts']}" for row in per_eta],
        padding=2,
        fontsize=7,
    )
    axes[2].bar_label(
        retention_bars,
        [
            f"{row['completed_with_material_final_regret_count']}/{row['complete_count']}"
            for row in per_eta
        ],
        padding=2,
        fontsize=7,
    )
    axes[2].text(
        0.98,
        0.98,
        "denominator: complete trajectories",
        transform=axes[2].transAxes,
        ha="right",
        va="top",
        fontsize=7,
    )
    fig.suptitle("Six-start descriptive rates; each bar uses the same paired starts")
    save(fig, figures, "evaluation-prefix-retention")


def load_rows_for_cell(cell: dict, summary_lookup: dict, artifacts: Path) -> list[dict]:
    summary = summary_lookup[cell["start_id"], cell["eta"]]
    return read_jsonl(resolve_recorded_path(summary["trajectory_path"], artifacts))


def plot_trajectories(
    cells: list[dict],
    summaries: list[dict],
    artifacts: Path,
    diagnostic_artifacts: Path,
    figures: Path,
) -> list[dict]:
    summary_lookup = {(row["start_id"], row["eta"]): row for row in summaries}
    chosen: list[tuple[str, dict, list[dict]]] = []
    diagnostic_rows = read_jsonl(diagnostic_artifacts / "trajectory-eta-1e-1.jsonl")
    chosen.append(
        (
            "Motivating diagnostic (not evaluation)",
            {"start_id": "random_F6_s0_1", "eta": 0.1},
            diagnostic_rows,
        )
    )
    late = max(
        cells,
        key=lambda cell: cell["relative_best_gain"] - cell["relative_best_gain_at_20"],
    )
    chosen.append(
        (
            "Evaluation: largest gain added after iteration 20",
            late,
            load_rows_for_cell(late, summary_lookup, artifacts),
        )
    )
    complete = [cell for cell in cells if cell["complete"]]
    regret = max(complete, key=lambda cell: cell["relative_final_regret"])
    chosen.append(
        (
            "Evaluation: largest final regret",
            regret,
            load_rows_for_cell(regret, summary_lookup, artifacts),
        )
    )
    invalid = [cell for cell in cells if not cell["complete"]]
    failure = max(invalid, key=lambda cell: cell["failure_iteration"])
    chosen.append(
        (
            "Evaluation: latest invalidity",
            failure,
            load_rows_for_cell(failure, summary_lookup, artifacts),
        )
    )

    fig, axes = plt.subplots(2, 2, figsize=(10.8, 7.0), constrained_layout=True)
    selected = []
    for ax, (role, cell, rows) in zip(axes.flat, chosen, strict=True):
        valid = [row for row in rows if row["sys"] is not None]
        iterations = [row["iteration"] for row in valid]
        raw = [row["sys"] for row in valid]
        best = [row["best_sys"] for row in valid]
        ax.plot(iterations, raw, color="#4c72b0", linewidth=1.2, label="raw full sys")
        ax.plot(iterations, best, color="#dd8452", linewidth=1.5, label="best so far")
        ax.axvline(8, color="0.45", linestyle=":", linewidth=1)
        ax.axvline(20, color="0.35", linestyle="--", linewidth=1)
        failure_row = next((row for row in rows if row["failure"]), None)
        if failure_row:
            ax.scatter(
                failure_row["iteration"],
                raw[-1],
                marker="x",
                color="#c44e52",
                s=50,
                linewidth=2,
                zorder=5,
            )
        ax.set_title(f"{role}\n{cell['start_id']}, $\\eta={cell['eta']:g}$", fontsize=9)
        ax.set_xlabel("Iteration")
        ax.set_ylabel("sys")
        ax.grid(alpha=0.25)
        selected.append(
            {"role": role, "start_id": cell["start_id"], "eta": cell["eta"]}
        )
    axes[0, 0].legend(fontsize=8)
    fig.suptitle(
        "Raw paths expose collapse, recovery, retention gaps, and invalid endpoints"
    )
    save(fig, figures, "evaluation-selected-trajectories")
    return selected


def format_eta(eta: float) -> str:
    return f"{eta:g}"


def write_discussion(
    path: Path, run: dict, analysis: dict, selected: list[dict]
) -> None:
    n = analysis["start_count"]
    cells = analysis["cell_count"]
    complete = analysis["complete_cell_count"]
    invalid = analysis["invalid_cell_count"]
    retained = analysis["complete_cells_with_material_final_regret"]
    lines = [
        "# Literal Branch-Gradient Multi-Start Discussion Packet",
        "",
        "## Question and algorithm",
        "",
        "This packet asks whether the favorable motivating trajectory survives a small, pre-target multi-start check. Each step chooses the deterministic currently minimizing admissible branch and unconditionally applies `a <- a + eta * grad_a sys_sigma(a)`. There is no normalization, projection, near-active set, maximin direction, line search, acceptance test, or early stopping. Invalid geometry and decreases are retained observations.",
        "",
        "## Evaluation population",
        "",
        f"The evaluation uses the first {n} `F=6` rows in canonical generic-random source order after excluding the already-observed `random_F6_s0_1`. Every start receives all six rates and up to 100 updates. The source generator uses seed 42 and height interval `[0.8,1.2]`. Selection used neither initial `sys` nor optimizer outcomes. The motivating start appears only as a labeled diagnostic in the selected-trajectory figure.",
        "",
        f"This is a descriptive sample of {n} starts ({cells} paired trajectories), not a precise estimate for all random `F=6` polytopes and not deterministic rerun replication. The fixed operational threshold used for prefix classification is a best-so-far increase of at least {100 * MATERIAL_RELATIVE_GAIN:.0f}% of initial `sys`. It was chosen before full producer execution but was not independently preregistered; treat the counts as descriptive.",
        "",
        "## Direct observations",
        "",
        f"- At least one rate achieved the material-gain threshold on **{analysis['starts_with_any_material_gain']}/{n} starts**.",
        f"- **{complete}/{cells} trajectories** completed 100 updates; **{invalid}/{cells}** became mathematically invalid before then. **{analysis['starts_with_any_invalid_rate']}/{n} starts** had at least one invalid rate.",
        f"- The 8-iteration practical class disagreed with the complete/terminal class on **{analysis['prefix_8_class_disagreements']}/{cells} trajectories**; at 20 iterations it disagreed on **{analysis['prefix_20_class_disagreements']}/{cells}**.",
        f"- The best value improved after iteration 8 on **{analysis['best_improved_after_8']}/{cells} trajectories** and after iteration 20 on **{analysis['best_improved_after_20']}/{cells}**. This includes arbitrarily small improvements; use the class-disagreement count for the 1% practical threshold.",
        f"- Among complete trajectories, the final state was at least 1% of initial `sys` below an earlier best on **{retained}/{complete}**. Invalid trajectories are excluded from that denominator; for them, no valid 100-update endpoint exists. The producer's legacy `summary.json` field `final_sys` stores the last valid pre-failure state, while `analysis.json` sets evaluative `final_sys` to null and preserves that value separately as `last_valid_sys`.",
        "",
        "Exact per-rate denominators, medians, quartiles, switch counts, and censoring are in `analysis.json`; the paired heatmap makes start/rate heterogeneity visible without pooling failures away.",
        "",
        "## Interpretation and competing explanations",
        "",
        "The motivating success was not unique: useful retained gains occur across the evaluation starts. But the literal rule is not a stable endpoint optimizer. Rate and start jointly control invalidity, late recovery, and whether the final state preserves the best value. Frequent branch switches and raw decreases are compatible with repeatedly following a branch that ceases to minimize after the update; this packet observes that pattern but does not establish a causal mechanism.",
        "",
        "A favorable reading is that the rule supplies cheap search directions and that best-so-far retention converts unstable paths into useful candidates. A less favorable reading is that the apparent gains come from a narrow generator slice and a six-rate sweep, while invalidity and endpoint regret mean the literal rule itself is too brittle for deployment. The current sample separates those readings only for this source prefix; another generator or facet count could behave differently.",
        "",
        "## Research decision",
        "",
        "**Retain literal ascent as a deliberately weak paired search baseline, and next compare it against one minimal safeguarded variant on these same frozen starts and rates.** The safeguard should preserve the same gradient proposal while adding explicit best-state retention plus rejection/backtracking of invalid or decreasing full-`sys` updates. Freeze this packet as the baseline; do not tune the baseline after seeing the comparison.",
        "",
        "Testing another population first has lower immediate information value: this packet already shows both cross-start utility and severe trajectory pathology. A same-start safeguard comparison would directly test whether minimal optimizer machinery removes the observed failure modes without attributing generator variation to the method.",
        "",
        "## Allowed and prohibited conclusions",
        "",
        "Allowed: on this six-start generic-random `F=6` prefix, report exact empirical rates for retained gain, invalidity, prefix disagreement, branch switching, and final regret; use the motivating start only as a diagnostic example; treat best-so-far retention as operationally important for this packet.",
        "",
        "Prohibited: population-wide success probabilities; a generally optimal learning rate; claims about other facet counts or generators; independence of trajectories sharing a start; treating deterministic reruns as replication; convergence, monotonicity, local maximality, or a mechanism theorem.",
        "",
        "## Reproduction and validation",
        "",
        f"The retained producer reports `{run['wall_seconds']:.1f}` seconds of trajectory wall time with parallelism `{run['parallelism']}`. `analysis.json` records source SHA-256, row-count and paired-coverage checks, exact update-identity checks, source-row identity, and generated figure paths. Figure examples are selected post hoc and labeled by role:",
        "",
    ]
    lines.extend(
        f"- {row['role']}: `{row['start_id']}`, `eta={format_eta(row['eta'])}`"
        for row in selected
    )
    lines.append("")
    path.write_text("\n".join(lines))


def main() -> None:
    args = parse_args()
    repo_root = OWNER.parents[2]
    run = read_json(args.artifacts / "summary.json")
    provenance = read_json(args.artifacts / "run-provenance.json")
    assert (
        blake3.blake3((OWNER / "main.rs").read_bytes()).hexdigest()
        == provenance["implementation_blake3"]
    )
    assert (
        blake3.blake3(args.source.read_bytes()).hexdigest()
        == provenance["input_blake3"]
    )
    source_rows = read_jsonl(args.source)
    expected = [
        row
        for row in source_rows
        if row.get("facet_count") == args.facet_count
        and row.get("name", row.get("poly_id")) != args.exclude_start_id
    ][: args.start_count]
    expected_ids = [row.get("name", row.get("poly_id")) for row in expected]
    assert run["selected_start_ids"] == expected_ids
    assert provenance["selected_start_ids"] == expected_ids
    assert run["updates"] == 100 and run["etas"] == ETAS
    assert len(run["trajectories"]) == args.start_count * len(ETAS)
    assert len({(row["start_id"], row["eta"]) for row in run["trajectories"]}) == len(
        run["trajectories"]
    )
    source_lookup = {
        row.get("name", row.get("poly_id")): row.get(
            "dual_vertices", row.get("dual_vertices_f64")
        )
        for row in expected
    }
    diagnostic_source = next(
        row
        for row in source_rows
        if row.get("name", row.get("poly_id")) == args.exclude_start_id
    )
    diagnostic_duals = diagnostic_source.get(
        "dual_vertices", diagnostic_source.get("dual_vertices_f64")
    )
    for path in sorted(args.diagnostic_artifacts.glob("trajectory-eta-*.jsonl")):
        assert vectors_close(
            read_jsonl(path)[0]["dual_vertices_before"], diagnostic_duals
        )

    cells = []
    raw_rows = 0
    for summary in run["trajectories"]:
        path = resolve_recorded_path(summary["trajectory_path"], args.artifacts)
        rows = read_jsonl(path)
        raw_rows += len(rows)
        cells.append(
            validate_trajectory(rows, summary, source_lookup[summary["start_id"]])
        )
    analysis = summarize(cells, expected_ids)
    analysis["selection_rule"] = run["selection_rule"]
    analysis["source_path"] = str(args.source.resolve().relative_to(repo_root))
    analysis["source_sha256"] = hashlib.sha256(args.source.read_bytes()).hexdigest()
    analysis["source_generator"] = {
        "family": "generic random",
        "seed": 42,
        "h_min": 0.8,
        "h_max": 1.2,
        "facet_count": 6,
    }
    analysis["selected_start_ids"] = expected_ids
    analysis["raw_row_count"] = raw_rows
    analysis["validation"] = {
        "paired_coverage": f"{args.start_count} starts x {len(ETAS)} rates = {len(cells)} unique trajectories",
        "artifact_identity": "current producer and canonical input BLAKE3 hashes equal run-provenance.json",
        "source_identity": "every evaluation trajectory and every motivating diagnostic trajectory starts at its named canonical source row",
        "update_identity": "every update/failure row satisfies da=eta*gradient and after=before+da within 2e-11 scaled tolerance",
        "summary_agreement": "best values/iterations, delta signs, branch switches, failures, and row counts recomputed from raw JSONL",
        "figure_agreement": "all figures generated directly from the validated raw rows and analysis object",
    }

    plt.rcParams.update({"font.size": 9, "figure.dpi": 150, "savefig.dpi": 200})
    plot_paired(cells, expected_ids, args.figures)
    plot_rates(analysis, args.figures)
    selected = plot_trajectories(
        cells,
        run["trajectories"],
        args.artifacts,
        args.diagnostic_artifacts,
        args.figures,
    )
    analysis["selected_trajectory_examples"] = selected
    analysis["figure_paths"] = [
        str((args.figures / f"{stem}.{extension}").resolve().relative_to(repo_root))
        for stem in (
            "evaluation-paired-outcomes",
            "evaluation-prefix-retention",
            "evaluation-selected-trajectories",
        )
        for extension in ("png", "pdf")
    ]
    with (args.artifacts / "analysis.json").open("w") as stream:
        json.dump({"analysis": analysis, "cells": cells}, stream, indent=2)
        stream.write("\n")
    write_discussion(args.artifacts / "DISCUSSION.md", run, analysis, selected)
    print(
        json.dumps(
            {"analysis": analysis, "discussion": str(args.artifacts / "DISCUSSION.md")},
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
