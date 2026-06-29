#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib", "numpy", "scikit-learn"]
# ///

"""Trusted random/product projection, clustering, and anomaly checks."""

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
    dataset_label,
    load_trusted_random_tables,
    matrix_for,
    active_invariant_numeric_feature_names,
    product_bucket,
    provenance_by_poly_id,
    write_json,
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--tables-dir", type=Path, default=TABLES_DIR)
    parser.add_argument("--out-dir", type=Path, default=HERE / "artifacts")
    parser.add_argument("--max-features", type=int, default=None)
    return parser.parse_args()


def safe_corr(left: np.ndarray, right: np.ndarray) -> float | None:
    if len(left) < 2 or np.std(left) == 0.0 or np.std(right) == 0.0:
        return None
    return float(np.corrcoef(left, right)[0, 1])


def vector_summary(values: np.ndarray) -> dict[str, float | int]:
    return {
        "rows": int(len(values)),
        "mean": float(np.mean(values)),
        "std": float(np.std(values)),
        "min": float(np.min(values)),
        "max": float(np.max(values)),
        "q10": float(np.quantile(values, 0.1)),
        "median": float(np.median(values)),
        "q90": float(np.quantile(values, 0.9)),
    }


def top_loadings(
    names: list[str], component: np.ndarray, *, count: int = 12
) -> list[dict[str, float | str]]:
    order = np.argsort(np.abs(component))[::-1][:count]
    return [
        {
            "feature": names[int(idx)],
            "loading": float(component[int(idx)]),
            "abs_loading": float(abs(component[int(idx)])),
        }
        for idx in order
    ]


def summarize_by_label(
    labels: list[str], pcs: np.ndarray, y: np.ndarray, max_labels: int = 20
) -> dict[str, object]:
    grouped: dict[str, list[int]] = {}
    for index, label in enumerate(labels):
        grouped.setdefault(label, []).append(index)
    entries = []
    for label, indices in sorted(grouped.items(), key=lambda item: (-len(item[1]), item[0])):
        if len(entries) >= max_labels:
            break
        idx = np.array(indices, dtype=int)
        entries.append(
            {
                "label": label,
                "rows": int(len(idx)),
                "mean_sys": float(np.mean(y[idx])),
                "max_sys": float(np.max(y[idx])),
                "pc_means": [float(value) for value in np.mean(pcs[idx, :], axis=0)],
                "pc_stds": [float(value) for value in np.std(pcs[idx, :], axis=0)],
            }
        )
    return {
        "group_count": len(grouped),
        "shown_group_count": len(entries),
        "groups": entries,
    }


def first_height_range(provenance_rows: list[dict[str, object]]) -> str:
    ranges = sorted(
        {
            (float(row["sample_h_min"]), float(row["sample_h_max"]))
            for row in provenance_rows
            if isinstance(row.get("sample_h_min"), int | float)
            and isinstance(row.get("sample_h_max"), int | float)
        }
    )
    if len(ranges) == 1:
        low, high = ranges[0]
        return f"{low:g}:{high:g}"
    if len(ranges) > 1:
        return "multi:" + ",".join(f"{low:g}:{high:g}" for low, high in ranges)
    return "missing"


def metadata_labels(
    rows: list[dict[str, object]], provenance_rows: list[dict[str, object]]
) -> dict[str, list[str]]:
    provenance = provenance_by_poly_id(provenance_rows)
    labels = {
        "capacity_source": [],
        "dataset_label": [],
        "facet_count": [],
        "dataset_label_by_facet_count": [],
        "product_bucket": [],
        "sample_height_range": [],
    }
    for row in rows:
        provenance_for_row = provenance.get(str(row["poly_id"]), [])
        source = str(row.get("capacity_source", "missing"))
        dataset = dataset_label(row, provenance_for_row)
        facet = f"F{row.get('facet_count')}"
        labels["capacity_source"].append(source)
        labels["dataset_label"].append(dataset)
        labels["facet_count"].append(facet)
        labels["dataset_label_by_facet_count"].append(f"{dataset}:{facet}")
        labels["product_bucket"].append(
            product_bucket(provenance_for_row)
            if source == "random_product_sample"
            else "not_product"
        )
        labels["sample_height_range"].append(first_height_range(provenance_for_row))
    return labels


def label_counts(labels: list[str], max_labels: int = 12) -> dict[str, object]:
    counts: dict[str, int] = {}
    for label in labels:
        counts[label] = counts.get(label, 0) + 1
    entries = [
        {"label": label, "rows": count}
        for label, count in sorted(counts.items(), key=lambda item: (-item[1], item[0]))[
            :max_labels
        ]
    ]
    return {"group_count": len(counts), "groups": entries}


def main() -> None:
    args = parse_args()
    rows, provenance_rows = load_trusted_random_tables(args.tables_dir)
    eligible_names = active_invariant_numeric_feature_names(rows)
    names = eligible_names[: args.max_features] if args.max_features else eligible_names
    x = np.array(matrix_for(rows, names), dtype=float)
    y = np.array([float(row["sys"]) for row in rows], dtype=float)
    args.out_dir.mkdir(parents=True, exist_ok=True)

    component_count = min(5, x.shape[0], x.shape[1])
    if component_count < 1:
        raise SystemExit("Projection analysis needs at least one row and one feature")
    pca_pipe = make_pipeline(
        StandardScaler(), PCA(n_components=component_count, random_state=20260621)
    )
    pcs = pca_pipe.fit_transform(x)
    pca = pca_pipe.named_steps["pca"]
    metadata_by_name = metadata_labels(rows, provenance_rows)
    metadata_overlays = {
        name: summarize_by_label(values, pcs, y) for name, values in metadata_by_name.items()
    }

    cluster_summaries = []
    for k in [value for value in [2, 3, 5, 8] if value <= len(rows)]:
        cluster_labels = KMeans(n_clusters=k, n_init=20, random_state=20260621).fit_predict(
            pcs
        )
        entries = []
        for label in range(k):
            cluster_mask = cluster_labels == label
            values = y[cluster_mask]
            cluster_indices = [index for index, selected in enumerate(cluster_mask) if selected]
            entries.append(
                {
                    "cluster": label,
                    "rows": int(len(values)),
                    "mean_sys": float(np.mean(values)),
                    "max_sys": float(np.max(values)),
                    "top_decile_fraction": float(np.mean(values >= np.quantile(y, 0.9))),
                    "metadata_composition": {
                        name: label_counts([labels[index] for index in cluster_indices])
                        for name, labels in metadata_by_name.items()
                    },
                }
            )
        cluster_summaries.append({"k": k, "clusters": entries})

    anomaly = IsolationForest(
        n_estimators=200,
        contamination=0.02,
        random_state=20260621,
    )
    scores = -anomaly.fit(pcs).score_samples(pcs)
    top_anomaly_idx = np.argsort(scores)[-25:][::-1]
    top_sys_idx = set(np.argsort(y)[-max(1, int(0.02 * len(y))) :])
    anomaly_top_sys_overlap = len(set(top_anomaly_idx) & top_sys_idx)

    summary = {
        "row_count": len(rows),
        "eligible_invariant_feature_count": len(eligible_names),
        "feature_count": len(names),
        "feature_names": names,
        "skipped_by_max_features": eligible_names[len(names) :],
        "pca_component_count": component_count,
        "explained_variance_ratio": [float(v) for v in pca.explained_variance_ratio_],
        "pc_sys_correlations": [
            safe_corr(pcs[:, component], y) for component in range(component_count)
        ],
        "pc1_sys_correlation": safe_corr(pcs[:, 0], y),
        "pc2_sys_correlation": safe_corr(pcs[:, 1], y) if component_count >= 2 else None,
        "pc_summaries": {
            f"pc{component + 1}": vector_summary(pcs[:, component])
            for component in range(component_count)
        },
        "top_pc1_loadings": top_loadings(names, pca.components_[0]),
        "top_pc2_loadings": top_loadings(names, pca.components_[1])
        if component_count >= 2
        else [],
        "metadata_overlays": metadata_overlays,
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
    pc2_values = pcs[:, 1] if component_count >= 2 else np.zeros(len(rows))
    scatter = ax.scatter(pcs[:, 0], pc2_values, c=y, s=6, cmap="viridis", alpha=0.7)
    ax.set_xlabel("PC1")
    ax.set_ylabel("PC2")
    ax.set_title("Random/product invariant-feature PCA colored by sys")
    fig.colorbar(scatter, ax=ax, label="sys")
    fig.tight_layout()
    fig.savefig(args.out_dir / "pca-sys.png", dpi=160)
    plt.close(fig)

    print("# projection-structure")
    print()
    print(f"- rows: `{len(rows)}`")
    print(f"- invariant features: `{len(names)}`")
    print(f"- PC1/sys correlation: `{summary['pc1_sys_correlation']}`")
    print(f"- top anomaly overlap with top 2% sys rows: `{anomaly_top_sys_overlap}`")
    print(f"Wrote `{args.out_dir}`")


if __name__ == "__main__":
    main()
