#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib", "numpy", "scikit-learn"]
# ///

"""Random-only projection, clustering, and anomaly checks."""

from __future__ import annotations

import argparse
from pathlib import Path
import sys

import numpy as np
from sklearn.cluster import KMeans
from sklearn.decomposition import PCA
from sklearn.ensemble import IsolationForest
from sklearn.pipeline import make_pipeline
from sklearn.preprocessing import StandardScaler

HERE = Path(__file__).resolve().parent
sys.path.append(str(HERE.parent / "_shared"))
from random_only import (  # noqa: E402
    TABLES_DIR,
    load_trusted_random_tables,
    matrix_for,
    numeric_feature_names,
    write_json,
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--tables-dir", type=Path, default=TABLES_DIR)
    parser.add_argument("--out-dir", type=Path, default=HERE / "artifacts")
    parser.add_argument("--max-features", type=int, default=80)
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    rows, _ = load_trusted_random_tables(args.tables_dir)
    names = numeric_feature_names(rows, geometry_only=True)[: args.max_features]
    x = np.array(matrix_for(rows, names), dtype=float)
    y = np.array([float(row["sys"]) for row in rows], dtype=float)
    args.out_dir.mkdir(parents=True, exist_ok=True)

    pca_pipe = make_pipeline(StandardScaler(), PCA(n_components=5, random_state=20260621))
    pcs = pca_pipe.fit_transform(x)
    pca = pca_pipe.named_steps["pca"]

    cluster_summaries = []
    for k in [2, 3, 5, 8]:
        labels = KMeans(n_clusters=k, n_init=20, random_state=20260621).fit_predict(pcs[:, :5])
        entries = []
        for label in range(k):
            values = y[labels == label]
            entries.append(
                {
                    "cluster": label,
                    "rows": int(len(values)),
                    "mean_sys": float(np.mean(values)),
                    "max_sys": float(np.max(values)),
                    "top_decile_fraction": float(np.mean(values >= np.quantile(y, 0.9))),
                }
            )
        cluster_summaries.append({"k": k, "clusters": entries})

    anomaly = IsolationForest(
        n_estimators=200,
        contamination=0.02,
        random_state=20260621,
    )
    scores = -anomaly.fit(pcs[:, :5]).score_samples(pcs[:, :5])
    top_anomaly_idx = np.argsort(scores)[-25:][::-1]
    top_sys_idx = set(np.argsort(y)[-max(1, int(0.02 * len(y))) :])
    anomaly_top_sys_overlap = len(set(top_anomaly_idx) & top_sys_idx)

    summary = {
        "row_count": len(rows),
        "feature_count": len(names),
        "explained_variance_ratio": [float(v) for v in pca.explained_variance_ratio_],
        "pc1_sys_correlation": float(np.corrcoef(pcs[:, 0], y)[0, 1]),
        "pc2_sys_correlation": float(np.corrcoef(pcs[:, 1], y)[0, 1]),
        "cluster_summaries": cluster_summaries,
        "top_25_anomaly_overlap_with_top_2pct_sys": anomaly_top_sys_overlap,
        "top_anomaly_rows": [
            {
                "poly_id": rows[int(idx)]["poly_id"],
                "sys": float(y[int(idx)]),
                "anomaly_score": float(scores[int(idx)]),
            }
            for idx in top_anomaly_idx
        ],
    }
    write_json(args.out_dir / "summary.json", summary)

    import matplotlib.pyplot as plt

    fig, ax = plt.subplots(figsize=(6.2, 5.2))
    scatter = ax.scatter(pcs[:, 0], pcs[:, 1], c=y, s=6, cmap="viridis", alpha=0.7)
    ax.set_xlabel("PC1")
    ax.set_ylabel("PC2")
    ax.set_title("Random-only geometry PCA colored by sys")
    fig.colorbar(scatter, ax=ax, label="sys")
    fig.tight_layout()
    fig.savefig(args.out_dir / "pca-sys.png", dpi=160)
    plt.close(fig)

    print("# projection-structure")
    print()
    print(f"- rows: `{len(rows)}`")
    print(f"- geometry features: `{len(names)}`")
    print(f"- PC1/sys correlation: `{summary['pc1_sys_correlation']}`")
    print(f"- top anomaly overlap with top 2% sys rows: `{anomaly_top_sys_overlap}`")
    print(f"Wrote `{args.out_dir}`")


if __name__ == "__main__":
    main()

