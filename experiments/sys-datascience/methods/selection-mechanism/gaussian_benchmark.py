#!/usr/bin/env python3
"""Gaussian rank-dependence reference, matching the existing selection statistic.

No geometry or capacity evaluations. The fitted parameters reuse the observed
data, so simulated envelopes are descriptive model references, not confidence
intervals for observed correlations or a pre-specified hypothesis test.
"""
import argparse
import csv
import hashlib
import json
import math
import time
from pathlib import Path

import numpy as np

import analyze


HERE = Path(__file__).resolve().parent


def read_tsv(path):
    with path.open() as stream:
        return list(csv.DictReader(stream, delimiter="\t"))


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def row_ranks(values):
    """One-based ranks of continuous simulated values, separately in each row."""
    order = np.argsort(values, axis=1)
    result = np.empty_like(order)
    np.put_along_axis(result, order, np.arange(1, values.shape[1] + 1)[None, :], axis=1)
    return result


def write_tsv(path, rows):
    with path.open("w") as stream:
        writer = csv.DictWriter(stream, fieldnames=list(rows[0]), delimiter="\t", lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def reference_summary(values, observed):
    lower, median, upper = np.quantile(values, [.025, .5, .975])
    return dict(observed=float(observed), simulated_mean=float(np.mean(values)),
                simulated_median=float(median), pointwise_lower_025=float(lower),
                pointwise_upper_975=float(upper),
                observed_above_envelope=bool(observed > upper),
                observed_below_envelope=bool(observed < lower))


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--replicates", type=int, default=1000)
    parser.add_argument("--seed", type=int, default=2026091801)
    parser.add_argument("--out", type=Path, default=HERE / "gaussian-artifacts")
    args = parser.parse_args()
    assert args.replicates >= 1000
    start = time.monotonic()
    detail_path = HERE / "artifacts/conditional-ranks-by-group.tsv"
    pooled_path = HERE / "artifacts/conditional-ranks.tsv"
    detail = read_tsv(detail_path)
    pooled = read_tsv(pooled_path)
    base = sorted([r for r in detail if r["selection"] == "high_sys" and float(r["fraction"]) == 1], key=lambda r: r["group"])
    assert len(base) == 18
    assert [int(r["n"]) for r in base] == [512] * 8 + [1024] * 10
    fractions = analyze.FRACTIONS
    selections = ["high_sys", "low_R"]
    observed_pooled = {(r["selection"], float(r["fraction"])): float(r["pooled_spearman"]) for r in pooled}
    observed_detail = {(r["selection"], float(r["fraction"]), r["group"]): float(r["spearman"]) for r in detail}
    sizes = [int(r["n"]) for r in base]
    # Each selected group has the same rank marginals in every replication.
    # Precompute average ranks of the pooled percentiles, including ties
    # between equally sized groups and exact ties between unlike sizes.
    transforms, denominators, totals = {}, {}, {}
    for fraction in fractions:
        counts = [math.ceil(n * fraction) for n in sizes]
        percentiles = np.concatenate([np.arange(1, n + 1) / n for n in counts])
        centered = analyze.rank(percentiles) - (len(percentiles) + 1) / 2
        denominators[fraction] = float(np.sum(centered ** 2))
        totals[fraction] = len(percentiles)
        transforms[fraction] = {}
        offset = 0
        for count in counts:
            candidate = centered[offset:offset + count]
            if count in transforms[fraction]:
                assert np.array_equal(transforms[fraction][count], candidate)
            transforms[fraction][count] = candidate
            offset += count
    numerators = {(s, f): np.zeros(args.replicates) for s in selections for f in fractions}
    simulations = {}
    group_rows = []
    fit_rows = []
    # Independent streams by group, with deterministic alphabetical ordering.
    streams = np.random.SeedSequence(args.seed).spawn(len(base))
    check_pairs = {(s, f): [] for s in selections for f in fractions}
    for group_index, row in enumerate(base):
        group, n = row["group"], int(row["n"])
        observed_rho = float(row["spearman"])
        gaussian_r = 2 * math.sin(math.pi * observed_rho / 6)
        assert -1 < gaussian_r < 1
        fit_rows.append(dict(group=group, n=n, observed_full_spearman=observed_rho, gaussian_pearson=gaussian_r))
        rng = np.random.default_rng(streams[group_index])
        x = rng.standard_normal((args.replicates, n))
        y = gaussian_r * x + math.sqrt(1 - gaussian_r ** 2) * rng.standard_normal((args.replicates, n))
        orders = {"high_sys": np.argsort(-y, axis=1), "low_R": np.argsort(x, axis=1)}
        for selection in selections:
            for fraction in fractions:
                count = math.ceil(n * fraction)
                indices = orders[selection][:, :count]
                rx = row_ranks(np.take_along_axis(x, indices, axis=1))
                ry = row_ranks(np.take_along_axis(y, indices, axis=1))
                transform = transforms[fraction][count]
                numerators[selection, fraction] += np.sum(transform[rx - 1] * transform[ry - 1], axis=1)
                group_spearman = np.sum((rx - (count + 1) / 2) * (ry - (count + 1) / 2), axis=1) / (count * (count ** 2 - 1) / 12)
                group_rows.append(dict(selection=selection, fraction=fraction, group=group, n=count,
                    **reference_summary(group_spearman, observed_detail[selection, fraction, group])))
                # Save first replication's ranks to compare against the
                # exact non-vectorized implementation in analyze.py.
                check_pairs[selection, fraction].extend(zip(rx[0] / count, ry[0] / count))
    summary = []
    max_check_error = 0.0
    for selection in selections:
        for fraction in fractions:
            values = numerators[selection, fraction] / denominators[fraction]
            simulations[f"{selection}_{fraction:g}"] = values
            pairs = np.asarray(check_pairs[selection, fraction])
            check = analyze.spearman(pairs[:, 0], pairs[:, 1])
            error = abs(check - values[0])
            assert error < 1e-12, (selection, fraction, error)
            max_check_error = max(max_check_error, error)
            summary.append(dict(selection=selection, fraction=fraction, n=totals[fraction],
                **reference_summary(values, observed_pooled[selection, fraction])))
    assert np.max(np.abs(simulations["high_sys_1"] - simulations["low_R_1"])) < 1e-12
    args.out.mkdir(exist_ok=True)
    write_tsv(args.out / "conditional-rank-reference.tsv", summary)
    write_tsv(args.out / "conditional-rank-reference-by-group.tsv", group_rows)
    write_tsv(args.out / "fitted-groups.tsv", fit_rows)
    np.savez_compressed(args.out / "pooled-replicates.npz", **simulations)
    metadata = dict(seed=args.seed, replicates=args.replicates, numpy_version=np.__version__,
        sources={str(p.relative_to(HERE)): sha256(p) for p in (detail_path, pooled_path, HERE / "analyze.py")},
        script_sha256=sha256(Path(__file__)), fractions=fractions,
        model="Within each of 18 original groups: iid bivariate normal ranks with Pearson r = 2 sin(pi * observed full-group Spearman / 6); groups independent",
        calibration="Separate fitted rank correlation per original group, preserving observed heterogeneity and 512/1024 sample sizes; parameters held fixed, no refitting within replicates",
        statistic="Exactly analyze.py: select ceil(fraction*n) within each group; re-rank each retained coordinate; divide by group count; pool all rows; compute Spearman with average ranks for ties",
        maximum_direct_statistic_check_error=max_check_error,
        envelope=f"Pointwise empirical 2.5% and 97.5% quantiles of {args.replicates} simulated replications; not simultaneous; does not include fitted-parameter uncertainty",
        interpretation="Descriptive fitted-model benchmark using observed data for calibration; not an independent validation, formal model test, or identification of a geometric or causal mechanism",
        historical_packet_check="tail-dependence-feasibility/analyze.py and README inspect tail overlaps/selected panels but do not contain this Gaussian benchmark",
        runtime_seconds=time.monotonic() - start)
    (args.out / "metadata.json").write_text(json.dumps(metadata, indent=2) + "\n")
    print(json.dumps({"metadata": metadata, "summary": summary}, indent=2))


if __name__ == "__main__":
    main()
