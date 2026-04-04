#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib", "numpy"]
# ///

"""
Analyze perturbations of the HK-O pentagon counterexample.

Goal: Plot histogram of systolic ratios under small facet perturbations,
      summarize stats, and compute PCA directions of the perturbation space.
Input: crates/exp-hko-local-maximum/perturbation-neighborhood/pentagon-perturb.jsonl
Output: crates/exp-hko-local-maximum/perturbation-neighborhood/pentagon_perturb_sys_hist.png
    crates/exp-hko-local-maximum/perturbation-neighborhood/pentagon_perturb_stats.md
    crates/exp-hko-local-maximum/perturbation-neighborhood/pentagon_perturb_stats.tex
    crates/exp-hko-local-maximum/perturbation-neighborhood/pentagon_perturb_pca.md
    crates/exp-hko-local-maximum/perturbation-neighborhood/pentagon_perturb_pca.tex
"""

import json
import sys
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent.parent.parent))
from figure_config import setup, FIGSIZE_SINGLE
setup()

REPO_ROOT = Path(__file__).resolve().parent.parent.parent.parent
EXPERIMENT_DIR = Path(__file__).resolve().parent
DATA_PATH = EXPERIMENT_DIR / "pentagon-perturb.jsonl"
FIGURES_DIR = EXPERIMENT_DIR
N_PCA_COMPONENTS = 5


def load_jsonl(path: Path) -> list[dict]:
    if not path.exists():
        print(f"ERROR: data file not found: {path}", file=sys.stderr)
        print("Run: cargo run --bin pentagon_perturb --release", file=sys.stderr)
        sys.exit(1)
    with open(path) as f:
        return [json.loads(line) for line in f if line.strip()]


def compute_stats(values: np.ndarray) -> dict:
    if values.size == 0:
        raise ValueError("No values provided for stats")
    return {
        "N": int(values.size),
        "mean": float(np.mean(values)),
        "median": float(np.median(values)),
        "std": float(np.std(values, ddof=1)) if values.size > 1 else 0.0,
        "min": float(np.min(values)),
        "max": float(np.max(values)),
    }


def format_stats_table(stats: dict, base_sys: float) -> str:
    def fmt(x: float) -> str:
        return f"{x:.4f}"

    lines = [
        "| N | mean | median | std | min | max | base sys |",
        "|---|---|---|---|---|---|---|",
        f"| {stats['N']} | {fmt(stats['mean'])} | {fmt(stats['median'])} | {fmt(stats['std'])} | {fmt(stats['min'])} | {fmt(stats['max'])} | {fmt(base_sys)} |",
    ]
    return "\n".join(lines)


def plot_histogram(sys_vals: np.ndarray, base_sys: float, output_path: Path) -> None:
    FIGURES_DIR.mkdir(parents=True, exist_ok=True)

    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
    ax.hist(sys_vals, bins=18, color="#3b6ea8", alpha=0.75, edgecolor="white")
    ax.axvline(1.0, color="#c0392b", linestyle="--", alpha=0.7, label="sys = 1")
    ax.axvline(base_sys, color="#2d6a4f", linestyle="-", alpha=0.9, label="base sys")

    ax.set_xlabel("Systolic ratio sys")
    ax.set_ylabel("Count")
    ax.set_title("HK-O pentagon perturbations: sys histogram (pruned)")
    ax.legend(loc="best")

    fig.tight_layout()
    fig.savefig(output_path)
    plt.close(fig)
    print(f"Saved: {output_path}")


def build_delta_matrix(rows: list[dict]) -> np.ndarray:
    """Build matrix of perturbation vectors from dual vertex deltas.

    Each row becomes a 40D vector (10 facets * 4 components of delta a_i).
    """
    vectors = []
    for r in rows:
        delta_duals = r.get("delta_dual_vertices")
        if delta_duals is None:
            raise ValueError("Missing delta_dual_vertices in dataset")
        if len(delta_duals) != 10:
            raise ValueError("Expected 10 facets for delta vectors")
        flat = []
        for facet_idx in range(10):
            da = delta_duals[facet_idx]
            if len(da) != 4:
                raise ValueError("Expected 4D dual vertices")
            flat.extend(da)
        vectors.append(flat)
    return np.array(vectors, dtype=float)


def pca_components(x: np.ndarray) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
    if x.ndim != 2 or x.shape[0] < 2:
        raise ValueError("Need at least two samples for PCA")
    x_centered = x - np.mean(x, axis=0)
    _, s, vt = np.linalg.svd(x_centered, full_matrices=False)
    variances = (s ** 2) / (x_centered.shape[0] - 1)
    explained_ratio = variances / np.sum(variances)
    return vt, variances, explained_ratio


def format_tuple(values: list[float], decimals: int = 4) -> str:
    parts = [f"{v:.{decimals}f}" for v in values[:4]]
    return f"({', '.join(parts)})"


def build_pca_rows(
    base_row: dict,
    components: np.ndarray,
    strengths: np.ndarray,
    n_components: int,
) -> list[list[str]]:
    rows = []
    base_cells = ["base"]
    base_duals = base_row.get("dual_vertices")
    if base_duals is None:
        raise ValueError("Base row missing dual_vertices")
    for facet_idx in range(10):
        a = base_duals[facet_idx]
        base_cells.append(format_tuple([a[0], a[1], a[2], a[3]]))
    base_cells.append(f"{float(base_row['sys']):.4f}")
    rows.append(base_cells)

    for i in range(n_components):
        comp = components[i]
        cells = [f"PC{i + 1}"]
        for facet_idx in range(10):
            offset = facet_idx * 4
            vals = comp[offset : offset + 4].tolist()
            cells.append(format_tuple(vals))
        cells.append(f"{strengths[i]:.4f}")
        rows.append(cells)

    return rows


def format_pca_table_md(rows: list[list[str]]) -> str:
    headers = ["component"] + [f"facet {i}" for i in range(1, 11)] + ["strength"]
    sep = ["---"] * len(headers)
    lines = ["| " + " | ".join(headers) + " |", "| " + " | ".join(sep) + " |"]
    for row in rows:
        lines.append("| " + " | ".join(row) + " |")
    return "\n".join(lines)


def format_pca_table_tex(rows: list[list[str]]) -> str:
    headers = ["Component"] + [f"Facet {i}" for i in range(1, 11)] + ["Strength"]
    col_spec = "l" + "c" * 11
    lines = [
        "\\begin{table}[p]",
        "\\centering",
        "\\scriptsize",
        "\\setlength{\\tabcolsep}{2pt}",
        "\\renewcommand{\\arraystretch}{1.1}",
        f"\\begin{{tabular}}{{{col_spec}}}",
        "\\toprule",
        " " + " & ".join(headers) + " \\\\",
        "\\midrule",
    ]
    for row in rows:
        lines.append(" " + " & ".join(row) + " \\\\")
    lines.extend(
        [
            "\\bottomrule",
            "\\end{tabular}",
            "\\caption{HK-O pentagon perturbations: PCA components of the 40D"
            " perturbation space, listed per facet as $\\Delta a_i \\in"
            " \\mathbb{R}^4$. The last column shows explained variance ratio for"
            " each component, and the base row reports the unperturbed dual"
            " vertices and systolic ratio.}",
            "\\label{tab:pentagon-perturb-pca}",
            "\\end{table}",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    rows = load_jsonl(DATA_PATH)
    base_rows = [r for r in rows if r.get("is_base")]
    if len(base_rows) != 1:
        print("ERROR: expected exactly one base row in dataset", file=sys.stderr)
        sys.exit(1)

    base_sys = float(base_rows[0]["sys"])
    sys_vals = np.array([r["sys"] for r in rows if not r.get("is_base")], dtype=float)
    if sys_vals.size == 0:
        print("ERROR: no perturbed samples found in dataset", file=sys.stderr)
        sys.exit(1)

    stats = compute_stats(sys_vals)
    table_md = format_stats_table(stats, base_sys)

    FIGURES_DIR.mkdir(parents=True, exist_ok=True)
    table_path = FIGURES_DIR / "pentagon_perturb_stats.md"
    table_path.write_text(table_md + "\n")
    print(f"Saved: {table_path}")

    stats_tex_path = FIGURES_DIR / "pentagon_perturb_stats.tex"
    stats_tex = "\n".join(
        [
            "\\begin{table}[h]",
            "\\centering",
            "\\begin{tabular}{rrrrrrr}",
            "\\toprule",
            "$N$ & Mean & Median & Std & Min & Max & Base sys \\\\",
            "\\midrule",
            f"{stats['N']} & {stats['mean']:.4f} & {stats['median']:.4f} & {stats['std']:.4f} "
            f"& {stats['min']:.4f} & {stats['max']:.4f} & {base_sys:.4f} \\\\",
            "\\bottomrule",
            "\\end{tabular}",
            "\\caption{HK-O pentagon perturbations: summary statistics for the"
            " perturbed systolic ratios.}",
            "\\label{tab:pentagon-perturb-stats}",
            "\\end{table}",
        ]
    )
    stats_tex_path.write_text(stats_tex + "\n")
    print(f"Saved: {stats_tex_path}")

    plot_histogram(sys_vals, base_sys, FIGURES_DIR / "pentagon_perturb_sys_hist.png")

    perturbed_rows = [r for r in rows if not r.get("is_base")]
    delta_matrix = build_delta_matrix(perturbed_rows)
    components, variances, explained_ratio = pca_components(delta_matrix)

    strengths = explained_ratio[:N_PCA_COMPONENTS]
    pca_rows = build_pca_rows(base_rows[0], components, strengths, N_PCA_COMPONENTS)

    pca_md = format_pca_table_md(pca_rows)
    pca_md_path = FIGURES_DIR / "pentagon_perturb_pca.md"
    pca_md_path.write_text(pca_md + "\n")
    print(f"Saved: {pca_md_path}")

    pca_tex = format_pca_table_tex(pca_rows)
    pca_tex_path = FIGURES_DIR / "pentagon_perturb_pca.tex"
    pca_tex_path.write_text(pca_tex + "\n")
    print(f"Saved: {pca_tex_path}")

    print("\nStats table:\n")
    print(table_md)
    print("\nPCA table:\n")
    print(pca_md)


if __name__ == "__main__":
    main()
