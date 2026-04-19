#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib", "numpy"]
# ///

"""
Goal: Plot histogram of systolic ratios under HK-O pentagon dual-vertex perturbations
      across multiple eps buckets, summarize stats per bucket, compute PCA on the
      canonical eps=0.01 bucket.
Input Artifacts: experiments/hko-local-maximum/perturbation-neighborhood/pentagon-perturb.jsonl
       or experiments/hko-local-maximum/perturbation-neighborhood/data/licca-eps-*.jsonl
Output Artifacts: experiments/hko-local-maximum/perturbation-neighborhood/pentagon_perturb_sys_hist.png
        experiments/hko-local-maximum/perturbation-neighborhood/pentagon_perturb_stats.md
        experiments/hko-local-maximum/perturbation-neighborhood/pentagon_perturb_stats.tex
        experiments/hko-local-maximum/perturbation-neighborhood/pentagon_perturb_pca.md
        experiments/hko-local-maximum/perturbation-neighborhood/pentagon_perturb_pca.tex

Row identity across buckets is (eps, name). Buckets are grouped by the eps field
on each row, not by filename — filenames are informational only.
"""

import json
import sys
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent.parent.parent))
from figure_config import setup, FIGSIZE_SINGLE
setup()

EXPERIMENT_DIR = Path(__file__).resolve().parent
DATA_DIR = EXPERIMENT_DIR / "data"
CANONICAL_DATA = EXPERIMENT_DIR / "pentagon-perturb.jsonl"
FIGURES_DIR = EXPERIMENT_DIR
N_PCA_COMPONENTS = 5
PCA_BUCKET_EPS = 0.01


def pick_jsonl_files(data_dir: Path) -> list[Path]:
    """Prefer the canonical single-file dataset, else fall back to LICCA shards."""
    if CANONICAL_DATA.exists():
        return [CANONICAL_DATA]

    if not data_dir.exists():
        print(f"ERROR: no canonical dataset at {CANONICAL_DATA}", file=sys.stderr)
        print(f"ERROR: data directory not found: {data_dir}", file=sys.stderr)
        print(
            "Run the binary first (from the repository root, see "
            "experiments/hko-local-maximum/perturbation-neighborhood/main.rs).",
            file=sys.stderr,
        )
        sys.exit(1)
    licca_files = sorted(data_dir.glob("licca-eps-*.jsonl"))
    if licca_files:
        return licca_files
    print(f"ERROR: no data files in {data_dir}", file=sys.stderr)
    sys.exit(1)


def load_grouped_by_eps(files: list[Path]) -> dict[float, list[dict]]:
    """Load rows grouped by eps. Malformed lines are skipped: a partial write
    from a concurrent writer (measurement run, crashed shard) must not derail
    the analyzer.
    """
    groups: dict[float, list[dict]] = {}
    for path in files:
        with open(path) as f:
            for line in f:
                if not line.strip():
                    continue
                try:
                    row = json.loads(line)
                except json.JSONDecodeError:
                    continue
                eps = round(float(row["eps"]), 10)
                groups.setdefault(eps, []).append(row)
    if not groups:
        print("ERROR: no rows loaded", file=sys.stderr)
        sys.exit(1)
    return groups


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


def format_stats_table_md(bucket_stats: list[tuple[float, dict]], base_sys: float) -> str:
    def fmt(x: float) -> str:
        return f"{x:.4f}"
    lines = [
        "| eps | N | mean | median | std | min | max | base sys |",
        "|---|---|---|---|---|---|---|---|",
    ]
    for eps, s in bucket_stats:
        lines.append(
            f"| {eps:.4g} | {s['N']} | {fmt(s['mean'])} | {fmt(s['median'])} | "
            f"{fmt(s['std'])} | {fmt(s['min'])} | {fmt(s['max'])} | {fmt(base_sys)} |"
        )
    return "\n".join(lines)


def format_stats_table_tex(bucket_stats: list[tuple[float, dict]], base_sys: float) -> str:
    lines = [
        "\\begin{table}[h]",
        "\\centering",
        "\\begin{tabular}{rrrrrrrr}",
        "\\toprule",
        "$\\varepsilon$ & $N$ & Mean & Median & Std & Min & Max & Base sys \\\\",
        "\\midrule",
    ]
    for eps, s in bucket_stats:
        lines.append(
            f"{eps:.4g} & {s['N']} & {s['mean']:.4f} & {s['median']:.4f} & "
            f"{s['std']:.4f} & {s['min']:.4f} & {s['max']:.4f} & {base_sys:.4f} \\\\"
        )
    lines.extend([
        "\\bottomrule",
        "\\end{tabular}",
        "\\caption{HK-O pentagon perturbations: summary statistics of the perturbed"
        " systolic ratios, grouped by perturbation magnitude $\\varepsilon$.}",
        "\\label{tab:pentagon-perturb-stats}",
        "\\end{table}",
    ])
    return "\n".join(lines)


def plot_histogram_grid(
    bucket_sys: list[tuple[float, np.ndarray]],
    base_sys: float,
    output_path: Path,
) -> None:
    n_buckets = len(bucket_sys)
    # Stack n_buckets single-panel plots vertically. Height is the full
    # FIGSIZE_SINGLE[1] per panel (no magic multiplier); a 3-panel layout
    # fits comfortably in \textwidth * 3 inches of vertical space in the
    # thesis float it lands in.
    fig, axes = plt.subplots(
        n_buckets, 1,
        figsize=(FIGSIZE_SINGLE[0], FIGSIZE_SINGLE[1] * n_buckets),
        sharex=False,
    )
    if n_buckets == 1:
        axes = [axes]

    for ax, (eps, sys_vals) in zip(axes, bucket_sys):
        ax.hist(sys_vals, bins=40, color="#3b6ea8", alpha=0.75, edgecolor="white")
        ax.axvline(1.0, color="#c0392b", linestyle="--", alpha=0.7, label=r"sys $= 1$")
        ax.axvline(base_sys, color="#2d6a4f", linestyle="-", alpha=0.9, label="base sys")
        ax.set_ylabel("Count")
        ax.set_title(rf"$\varepsilon = {eps:.4g}$, $N = {sys_vals.size}$")
        ax.legend(loc="best", fontsize="small")

    axes[-1].set_xlabel(r"Systolic ratio sys")
    fig.tight_layout()
    fig.savefig(output_path)
    plt.close(fig)
    print(f"Saved: {output_path}")


def build_delta_matrix(rows: list[dict]) -> np.ndarray:
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
    lines.extend([
        "\\bottomrule",
        "\\end{tabular}",
        "\\caption{HK-O pentagon perturbations: PCA components of the 40D"
        " perturbation space at $\\varepsilon = 0.01$, listed per facet as"
        " $\\Delta a_i \\in \\mathbb{R}^4$. The last column shows explained variance"
        " ratio for each component, and the base row reports the unperturbed dual"
        " vertices and systolic ratio.}",
        "\\label{tab:pentagon-perturb-pca}",
        "\\end{table}",
    ])
    return "\n".join(lines)


def main() -> None:
    files = pick_jsonl_files(DATA_DIR)
    print(f"Using {len(files)} data file(s):")
    for p in files:
        print(f"  {p.name}")
    groups = load_grouped_by_eps(files)

    base_row: dict | None = None
    for rows in groups.values():
        for r in rows:
            if r.get("is_base"):
                base_row = r
                break
        if base_row is not None:
            break
    if base_row is None:
        print("ERROR: no base row in any bucket", file=sys.stderr)
        sys.exit(1)
    base_sys = float(base_row["sys"])

    bucket_stats: list[tuple[float, dict]] = []
    bucket_sys: list[tuple[float, np.ndarray]] = []
    for eps in sorted(groups.keys()):
        perturbed = [r for r in groups[eps] if not r.get("is_base")]
        sys_vals = np.array([r["sys"] for r in perturbed], dtype=float)
        if sys_vals.size == 0:
            print(f"WARN: eps={eps} bucket has no perturbed rows, skipping", file=sys.stderr)
            continue
        bucket_stats.append((eps, compute_stats(sys_vals)))
        bucket_sys.append((eps, sys_vals))

    FIGURES_DIR.mkdir(parents=True, exist_ok=True)

    table_md = format_stats_table_md(bucket_stats, base_sys)
    table_path = FIGURES_DIR / "pentagon_perturb_stats.md"
    table_path.write_text(table_md + "\n")
    print(f"Saved: {table_path}")

    stats_tex = format_stats_table_tex(bucket_stats, base_sys)
    stats_tex_path = FIGURES_DIR / "pentagon_perturb_stats.tex"
    stats_tex_path.write_text(stats_tex + "\n")
    print(f"Saved: {stats_tex_path}")

    plot_histogram_grid(
        bucket_sys, base_sys, FIGURES_DIR / "pentagon_perturb_sys_hist.png"
    )

    pca_eps = min(groups.keys(), key=lambda e: abs(e - PCA_BUCKET_EPS))
    pca_rows_raw = [r for r in groups[pca_eps] if not r.get("is_base")]
    if len(pca_rows_raw) < 2:
        print(
            f"WARN: PCA bucket eps={pca_eps} has <2 perturbed rows, skipping PCA",
            file=sys.stderr,
        )
    else:
        delta_matrix = build_delta_matrix(pca_rows_raw)
        components, _variances, explained_ratio = pca_components(delta_matrix)
        strengths = explained_ratio[:N_PCA_COMPONENTS]
        pca_rows = build_pca_rows(base_row, components, strengths, N_PCA_COMPONENTS)

        pca_md = format_pca_table_md(pca_rows)
        pca_md_path = FIGURES_DIR / "pentagon_perturb_pca.md"
        pca_md_path.write_text(pca_md + "\n")
        print(f"Saved: {pca_md_path}")

        pca_tex = format_pca_table_tex(pca_rows)
        pca_tex_path = FIGURES_DIR / "pentagon_perturb_pca.tex"
        pca_tex_path.write_text(pca_tex + "\n")
        print(f"Saved: {pca_tex_path}")

        print(f"\nPCA computed on eps={pca_eps} bucket, N={len(pca_rows_raw)}")

    print("\nStats table:\n")
    print(table_md)


if __name__ == "__main__":
    main()
