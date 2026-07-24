# /// script
# dependencies = ["pandas", "numpy", "matplotlib", "seaborn", "tabulate"]
# ///
"""Build an exploratory atlas of scalar-feature behavior in the retained sys tail."""

import argparse
import json
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import seaborn as sns


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--table",
        type=Path,
        default=ROOT / "experiments/polytope-invariant-table/polytope-table.jsonl",
    )
    parser.add_argument(
        "--provenance",
        type=Path,
        default=ROOT
        / "experiments/polytope-invariant-table/polytope-provenance-table.jsonl",
    )
    parser.add_argument(
        "--hko-summary",
        type=Path,
        default=ROOT
        / "experiments/sys-datascience/methods/hko-reference-coverage/artifacts/summary.json",
    )
    parser.add_argument("--out-dir", type=Path, default=HERE / "artifacts")
    return parser.parse_args()


def main():
    args = parse_args()
    args.out_dir.mkdir(parents=True, exist_ok=True)

    table = pd.read_json(args.table, lines=True)
    provenance = pd.read_json(args.provenance, lines=True)
    provenance = provenance[["poly_id", "dataset", "product_k", "product_m"]]
    frame = table.merge(provenance, on="poly_id", how="left", validate="one_to_one")
    frame["bucket"] = np.where(
        frame["dataset"].eq("random_product_sample"),
        "P"
        + frame["product_k"].astype("Int64").astype(str)
        + "x"
        + frame["product_m"].astype("Int64").astype(str),
        "G-F" + frame["facet_count"].astype(str),
    )

    features = {
        "ridge sum": "ridge_symp_area_sum_over_volume_sqrt",
        "ridge mean": "ridge_symp_area_mean_over_volume_sqrt",
        "ridge max": "ridge_symp_area_max_over_volume_sqrt",
        "ridge q95": "ridge_symp_area_q95_over_volume_sqrt",
        "ridge std": "ridge_symp_area_std_over_volume_sqrt",
        "max share": "ridge_symp_area_max_share",
        "top-3 share": "ridge_symp_area_top3_share",
        "ridge median": "ridge_symp_area_median_over_volume_sqrt",
        "ridge q25": "ridge_symp_area_q25_over_volume_sqrt",
    }

    frame["sys_rank"] = frame.groupby("bucket")["sys"].rank(
        pct=True, method="average"
    )
    rows = []
    bucket_rows = []
    for label, column in features.items():
        rank_column = f"rank::{label}"
        frame[rank_column] = frame.groupby("bucket")[column].rank(
            pct=True, method="average"
        )
        pooled_rho = frame[[rank_column, "sys_rank"]].corr(method="spearman").iloc[0, 1]
        bucket_rhos = frame.groupby("bucket").apply(
            lambda group: group[[column, "sys"]].corr(method="spearman").iloc[0, 1],
            include_groups=False,
        )
        favorable_low = pooled_rho < 0
        selected = (
            frame[rank_column] <= 0.1
            if favorable_low
            else frame[rank_column] >= 0.9
        )
        tail = frame["sys_rank"] >= 0.95
        rows.append(
            {
                "feature": label,
                "pooled_within_bucket_rank_rho": pooled_rho,
                "bucket_rho_median": bucket_rhos.median(),
                "bucket_rho_min": bucket_rhos.min(),
                "bucket_rho_max": bucket_rhos.max(),
                "buckets_same_sign": int(
                    ((bucket_rhos < 0) if favorable_low else (bucket_rhos > 0)).sum()
                ),
                "buckets": int(bucket_rhos.size),
                "favorable_decile_mean_sys": frame.loc[selected, "sys"].mean(),
                "other_mean_sys": frame.loc[~selected, "sys"].mean(),
                "favorable_decile_max_sys": frame.loc[selected, "sys"].max(),
                "tail_rate_selected": tail[selected].mean(),
                "tail_rate_other": tail[~selected].mean(),
            }
        )
        for bucket, value in bucket_rhos.items():
            bucket_rows.append({"feature": label, "bucket": bucket, "rho": value})

    summary = pd.DataFrame(rows).sort_values(
        "pooled_within_bucket_rank_rho", key=np.abs, ascending=False
    )
    summary.to_csv(args.out_dir / "feature-summary.tsv", sep="\t", index=False)
    pd.DataFrame(bucket_rows).to_csv(
        args.out_dir / "bucket-rhos.tsv", sep="\t", index=False
    )

    ridge_rank = frame["rank::ridge sum"]
    edge_bins = pd.cut(
        ridge_rank,
        [0, 0.01, 0.02, 0.05, 0.10, 0.20, 0.50, 1],
        include_lowest=True,
    )
    edge_summary = (
        frame.assign(ridge_percentile_band=edge_bins)
        .groupby("ridge_percentile_band", observed=True)
        .agg(
            n=("sys", "size"),
            mean_sys=("sys", "mean"),
            q90_sys=("sys", lambda values: values.quantile(0.9)),
            q99_sys=("sys", lambda values: values.quantile(0.99)),
            max_sys=("sys", "max"),
            bucket_top5_rate=("sys_rank", lambda values: (values >= 0.95).mean()),
        )
    )
    edge_summary.to_csv(args.out_dir / "ridge-extreme-tail.tsv", sep="\t")

    rank_columns = [f"rank::{label}" for label in features]
    rank_correlation = frame[rank_columns].corr(method="spearman")
    rank_correlation.index = list(features)
    rank_correlation.columns = list(features)
    rank_correlation.to_csv(
        args.out_dir / "feature-rank-correlation.tsv", sep="\t"
    )

    sns.set_theme(style="whitegrid", context="notebook")
    figure, axes = plt.subplots(3, 3, figsize=(15, 12), sharex=True)
    for axis, (label, _column) in zip(axes.flat, features.items()):
        ranks = frame[f"rank::{label}"]
        bins = pd.cut(
            ranks, np.linspace(0, 1, 11), include_lowest=True, labels=False
        )
        view = (
            frame.assign(_bin=bins)
            .groupby("_bin", observed=True)
            .agg(
                q50=("sys", "median"),
                q90=("sys", lambda values: values.quantile(0.9)),
                q99=("sys", lambda values: values.quantile(0.99)),
                maximum=("sys", "max"),
                top5=("sys_rank", lambda values: (values >= 0.95).mean()),
            )
        )
        x_values = (view.index.to_numpy() + 0.5) / 10
        axis.plot(x_values, view.q50, label="median", lw=1.2)
        axis.plot(x_values, view.q90, label="q90", lw=1.2)
        axis.plot(x_values, view.q99, label="q99", lw=1.5)
        axis.plot(x_values, view.maximum, label="max", lw=1, alpha=0.65)
        rate_axis = axis.twinx()
        rate_axis.plot(
            x_values, view.top5, color="black", ls="--", lw=1, alpha=0.55
        )
        rate_axis.set_ylim(0, max(0.25, view.top5.max() * 1.15))
        axis.set_title(label)
        axis.set_ylim(0, 1.02)
        axis.set_xlabel("within-bucket feature percentile")
        axis.set_ylabel("sys")
    axes.flat[0].legend(loc="lower right", fontsize=8)
    figure.suptitle(
        "Conditional sys tail by invariant percentile\n"
        "(dashed: fraction in bucket-specific top 5% sys)",
        y=1.01,
    )
    figure.tight_layout()
    figure.savefig(
        args.out_dir / "conditional-tail-atlas.png", dpi=170, bbox_inches="tight"
    )
    plt.close(figure)

    heat = pd.DataFrame(bucket_rows).pivot(
        index="bucket", columns="feature", values="rho"
    )
    heat = heat.reindex(sorted(heat.index, key=lambda value: (value.startswith("P"), value)))
    figure, axis = plt.subplots(figsize=(13, 8))
    sns.heatmap(
        heat,
        cmap="vlag",
        center=0,
        vmin=-1,
        vmax=1,
        annot=True,
        fmt=".2f",
        ax=axis,
    )
    axis.set_title("Spearman correlation of invariant with sys inside each bucket")
    figure.tight_layout()
    figure.savefig(args.out_dir / "bucket-correlation-heatmap.png", dpi=170)
    plt.close(figure)

    figure, axes = plt.subplots(1, 3, figsize=(15, 4.5))
    for axis, (label, column) in zip(axes, list(features.items())[1:4]):
        for dataset, color_map in [
            ("random_sample", "Blues"),
            ("random_product_sample", "Oranges"),
        ]:
            group = frame[frame.dataset.eq(dataset)]
            axis.hexbin(
                group[column],
                group.sys,
                gridsize=45,
                mincnt=1,
                bins="log",
                cmap=color_map,
                alpha=0.55,
            )
        axis.set_title(label)
        axis.set_xlabel(column)
        axis.set_ylabel("sys")
    figure.suptitle("Raw scalar/sys clouds (generic blue, product orange)")
    figure.tight_layout()
    figure.savefig(args.out_dir / "raw-clouds.png", dpi=170)
    plt.close(figure)

    hko = json.loads(args.hko_summary.read_text())
    report = [
        "# Conditional-tail atlas results",
        "",
        f"Rows: {len(frame)}; buckets: {frame.bucket.nunique()}; "
        f"max sys: {frame.sys.max():.6f}.",
        "",
        "HKO reference: sys={:.6f}, ridge sum={:.6f}; its ridge sum is below "
        "every retained row, yet it lies inside the current 39-feature "
        "nearest-neighbor cloud.".format(
            hko["hko"]["sys"], hko["hko"]["ridge_area_score"]
        ),
        "",
        "## Feature summary",
        "",
        summary.to_markdown(index=False, floatfmt=".3f"),
        "",
        "## Extreme low-ridge bands",
        "",
        edge_summary.to_markdown(floatfmt=".3f"),
        "",
        "This is exploratory reuse of evaluated rows. It creates no proposer or "
        "generalization claim.",
    ]
    (args.out_dir / "analysis.md").write_text("\n".join(report) + "\n")
    print(args.out_dir)


if __name__ == "__main__":
    main()
