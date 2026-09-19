# /// script
# dependencies = ["numpy", "matplotlib"]
# ///
"""Describe ridge/sys ranks after within-source selection; no new target calls."""
import argparse
import csv
import hashlib
import json
from collections import defaultdict
from pathlib import Path

import numpy as np

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
SNAPSHOT = "c3042846723dbb89db17f2f39d88004d937e1e790949ff6880188fa89fc1900a"
TABLE_HASH = "607c8731fa03d190d497edc3e8f1b4cca88f7d238260cce527680f568bc33d59"
PROVENANCE_HASH = "6ff88a5accce9a7ec7e5a494107350b0974b2ce0268ea44caae36a18a7494ef2"
FRACTIONS = [1.0, .5, .2, .1, .05, .02, .01]


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def rank(values):
    """One-based average ranks, including ties."""
    values = np.asarray(values)
    order = np.argsort(values, kind="stable")
    result = np.empty(len(values), dtype=float)
    first = 0
    while first < len(values):
        last = first + 1
        while last < len(values) and values[order[last]] == values[order[first]]:
            last += 1
        result[order[first:last]] = (first + last + 1) / 2
        first = last
    return result


def spearman(x, y):
    return float(np.corrcoef(rank(x), rank(y))[0, 1])


def write_tsv(path, rows):
    with path.open("w") as output:
        writer = csv.DictWriter(output, fieldnames=list(rows[0]), delimiter="\t", lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--table", type=Path, default=Path.home() / ".cache/msc-math/artifacts/polytope-invariant-table" / SNAPSHOT / "files/polytope-table.jsonl")
    parser.add_argument("--provenance", type=Path, default=ROOT / "experiments/polytope-invariant-table/polytope-provenance-table.jsonl")
    parser.add_argument("--out", type=Path, default=HERE / "artifacts")
    parser.add_argument("--no-figure", action="store_true")
    args = parser.parse_args()
    assert sha256(args.table) == TABLE_HASH, "unexpected retained-table bytes"
    assert sha256(args.provenance) == PROVENANCE_HASH, "unexpected provenance bytes"
    provenance_rows = [json.loads(line) for line in args.provenance.read_text().splitlines()]
    provenance = {row["poly_id"]: row for row in provenance_rows}
    assert len(provenance) == len(provenance_rows) == 14336
    groups = defaultdict(list)
    table_rows = [json.loads(line) for line in args.table.read_text().splitlines()]
    assert len({row["poly_id"] for row in table_rows}) == len(table_rows) == 14336
    assert set(provenance) == {row["poly_id"] for row in table_rows}
    for row in table_rows:
        source = provenance[row["poly_id"]]
        assert row["capacity_source"] == source["dataset"]
        assert row["ridge_symp_area_ordering_failure_count"] == 0
        assert row["ridge_symp_area_ordered_face_count"] == row["ridge_count"]
        if source["dataset"] == "random_product_sample":
            group = f"P{source['product_k']}x{source['product_m']}"
        else:
            assert source["dataset"] == "random_sample"
            group = f"G{row['facet_count']:02}"
        groups[group].append((row["ridge_symp_area_sum_over_volume_sqrt"], row["sys"], row["poly_id"]))
    assert len(groups) == 18
    for group, rows in groups.items():
        assert len(rows) == (1024 if group.startswith("P") else 512)
    args.out.mkdir(parents=True, exist_ok=True)
    detailed, summary, selections = [], [], []
    for selection in ["high_sys", "low_R"]:
        for fraction in FRACTIONS:
            rank_pairs = []
            group_statistics = []
            for group, rows in sorted(groups.items()):
                count = int(np.ceil(len(rows) * fraction))
                column = 1 if selection == "high_sys" else 0
                ordered = sorted(rows, key=lambda row: ((-1 if selection == "high_sys" else 1) * row[column], row[2]))
                # No selection-boundary ties in these data; fail rather than hide one.
                assert count == len(ordered) or ordered[count - 1][column] != ordered[count][column]
                selected = ordered[:count]
                values = np.asarray([(row[0], row[1]) for row in selected])
                assert np.all(np.isfinite(values))
                statistic = spearman(values[:, 0], values[:, 1])
                group_statistics.append(statistic)
                rank_pairs.extend(zip(rank(values[:, 0]) / count, rank(values[:, 1]) / count))
                detailed.append(dict(selection=selection, fraction=fraction, group=group, n=count,
                    spearman=statistic, R_min=float(values[:, 0].min()), R_max=float(values[:, 0].max()),
                    sys_min=float(values[:, 1].min()), sys_max=float(values[:, 1].max())))
                selections.append(dict(selection=selection, fraction=fraction, group=group,
                    selected_ids_sha256=hashlib.sha256(("\n".join(sorted(row[2] for row in selected)) + "\n").encode()).hexdigest()))
            rank_pairs = np.asarray(rank_pairs)
            summary.append(dict(selection=selection, fraction=fraction, n=len(rank_pairs),
                pooled_spearman=spearman(rank_pairs[:, 0], rank_pairs[:, 1]),
                mean_group_spearman=float(np.mean(group_statistics)),
                median_group_spearman=float(np.median(group_statistics)),
                negative_groups=sum(value < 0 for value in group_statistics), groups=len(groups),
                min_group_spearman=min(group_statistics), max_group_spearman=max(group_statistics)))
    write_tsv(args.out / "conditional-ranks.tsv", summary)
    write_tsv(args.out / "conditional-ranks-by-group.tsv", detailed)
    write_tsv(args.out / "selection-checksums.tsv", selections)
    metadata = dict(table_sha256=TABLE_HASH, provenance_sha256=PROVENANCE_HASH, script_sha256=sha256(Path(__file__)),
        rows=14336, groups=18, fractions=FRACTIONS,
        selection="ceil(fraction * group size) rows, selected separately within every original source/facet/product group",
        statistic="Spearman correlation after re-ranking R and sys inside each selected group, dividing each rank by selected group size, concatenating rows, and computing Spearman on these conditional percentiles",
        weighting="Each retained row receives equal weight; product groups contain twice as many rows as generic groups, up to ceiling effects. Equal-group mean and median correlations also retained.",
        numerical_lineage="Historical stored sys values; this analysis does not re-evaluate capacities or volumes.",
        inference="Descriptive post-target conditioning, not an independent prospective experiment or identification of a causal mechanism. Narrow-range correlations may attenuate through range restriction alone.")
    (args.out / "metadata.json").write_text(json.dumps(metadata, indent=2) + "\n")
    if not args.no_figure:
        import matplotlib
        matplotlib.use("Agg")
        import matplotlib.pyplot as plt
        plt.rcParams.update({"font.size": 10, "axes.spines.top": False, "axes.spines.right": False})
        figure, ax = plt.subplots(figsize=(6.4, 3.5), constrained_layout=True)
        for selection, label, style in [("high_sys", "Largest systolic ratios", "o-"), ("low_R", "Smallest ridge sums", "s-")]:
            selected = [row for row in summary if row["selection"] == selection]
            ax.plot([100 * row["fraction"] for row in selected], [row["pooled_spearman"] for row in selected], style, label=label, markersize=4)
        ax.set_xscale("log")
        ax.set_xlim(100, 1)
        ax.set_xticks([100, 50, 20, 10, 5, 2, 1], labels=["100", "50", "20", "10", "5", "2", "1"])
        ax.set_xlabel("Retained fraction within each group (%)")
        ax.set_ylabel("Conditional rank correlation")
        ax.axhline(0, color=".6", linewidth=.7)
        ax.set_ylim(-1, .1)
        ax.legend(frameon=False, loc="lower right")
        ax.text(.02, .98, "14,336 bodies at 100%; 728 at 5%; 158 at 1%", transform=ax.transAxes, va="top", fontsize=8)
        figure.savefig(args.out / "conditional-ranks.pdf")
        figure.savefig(args.out / "conditional-ranks.png", dpi=180)
        plt.close(figure)
    print(json.dumps(summary, indent=2))


if __name__ == "__main__":
    main()
