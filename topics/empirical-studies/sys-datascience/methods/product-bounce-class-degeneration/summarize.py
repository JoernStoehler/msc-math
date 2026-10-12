#!/usr/bin/env python3
"""Deterministic descriptive summaries for the class-degeneration packet."""

import argparse
import hashlib
import json
import math
import random
import statistics
from collections import Counter, defaultdict
from pathlib import Path


def val(x):
    if isinstance(x, (int, float)):
        return float(x)
    if isinstance(x, str) and "/" in x:
        a, b = x.split("/", 1)
        return float(a) / float(b)
    return float(x)


def med(xs):
    return statistics.median(xs) if xs else None


def rank(xs):
    order = sorted(range(len(xs)), key=lambda i: xs[i])
    out = [0.0] * len(xs)
    i = 0
    while i < len(xs):
        j = i + 1
        while j < len(xs) and xs[order[j]] == xs[order[i]]:
            j += 1
        v = (i + j - 1) / 2 + 1
        for q in range(i, j):
            out[order[q]] = v
        i = j
    return out


def corr(x, y):
    if len(x) < 3:
        return None
    mx, my = statistics.mean(x), statistics.mean(y)
    dx, dy = [a - mx for a in x], [b - my for b in y]
    den = math.sqrt(sum(a * a for a in dx) * sum(b * b for b in dy))
    return sum(a * b for a, b in zip(dx, dy)) / den if den else None


def spearman(x, y):
    return corr(rank(x), rank(y))


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def complete(row):
    return row["gap_abs"] is not None and math.isfinite(row["gap_abs"])


def qshare(rows, threshold):
    rows = [r for r in rows if complete(r)]
    selected = [r for r in rows if r["gap_abs"] <= threshold]
    return (
        sum(r["exact_support_equal"] for r in selected) / len(selected)
        if selected
        else None
    )


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--rows", required=True)
    ap.add_argument("--pairs", required=True)
    ap.add_argument("--out", required=True)
    args = ap.parse_args()
    rows = [json.loads(line) for line in open(args.rows)]
    pairs = [json.loads(line) for line in open(args.pairs)]
    by = defaultdict(list)
    for row in rows:
        by[(row["k"], row["m"])].append(row)
    pby = defaultdict(list)
    for pair in pairs:
        pby[pair["name"]].append(pair)
    complete_rows = [row for row in rows if complete(row)]
    complete_names = {row["name"] for row in complete_rows}
    complete_pairs = [pair for pair in pairs if pair["name"] in complete_names]

    buckets = {}
    for key, bucket_rows in sorted(by.items()):
        bucket_complete = [row for row in bucket_rows if complete(row)]
        buckets[f"{key[0]}x{key[1]}"] = {
            "rows": len(bucket_rows),
            "complete": len(bucket_complete),
            "same_support_rows": sum(row["exact_support_equal"] for row in bucket_complete),
            "support_equality_rate": (
                sum(row["exact_support_equal"] for row in bucket_complete)
                / len(bucket_complete)
                if bucket_complete
                else None
            ),
            "gap_abs_median": med([row["gap_abs"] for row in bucket_complete]),
            "gap_signed_median": med([row["gap_signed"] for row in bucket_complete]),
            "threshold_support_rates": {
                str(t): qshare(bucket_complete, t) for t in [0.001, 0.01, 0.05]
            },
            "pair_count": sum(len(pby[row["name"]]) for row in bucket_complete),
            "primary_pair_count": sum(
                row["same_support_pair_count"] for row in bucket_complete
            ),
        }

    # The shuffle population is explicitly the 9,455 complete rows.
    rng = random.Random(20260713)
    shuffle = []
    for _ in range(100):
        values = []
        for bucket_rows in by.values():
            bucket_rows = [row for row in bucket_rows if complete(row)]
            permuted = list(bucket_rows)
            rng.shuffle(permuted)
            for row, source in zip(bucket_rows, permuted):
                values.append(
                    any(
                        set(a) == set(b)
                        for a in row["a2_supports"]
                        for b in source["a3_supports"]
                    )
                )
        shuffle.append(sum(values) / len(values) if values else None)
    observed = (
        sum(row["exact_support_equal"] for row in complete_rows) / len(complete_rows)
        if complete_rows
        else None
    )

    row_tie = []
    for row in complete_rows:
        row_pairs = pby[row["name"]]
        if row_pairs:
            gaps = [abs(pair["normalized_gap"]) for pair in row_pairs]
            row_tie.append({"min": min(gaps), "max": max(gaps), "range": max(gaps) - min(gaps)})
    primary = {
        "pairs": len(complete_pairs),
        "flip_count": dict(Counter(pair["flip_count"] for pair in complete_pairs)),
        "recovery_pass": sum(pair["recovery_valid"] for pair in complete_pairs),
        "recovery_total": len(complete_pairs),
        "closure_error_max": max(
            (pair["closure_error"] for pair in complete_pairs if pair["closure_error"] is not None),
            default=None,
        ),
        "beta_product_median": med(
            [val(pair["beta_product"]) for pair in complete_pairs if pair["beta_product"]]
        ),
        "pairing_factor_median": med(
            [pair["normalized_pairing_factor"] for pair in complete_pairs if pair["normalized_pairing_factor"] is not None]
        ),
        "cancellation_ratio_median_two_term": med(
            [pair["cancellation_ratio"] for pair in complete_pairs if pair["flip_count"] == 2 and pair["cancellation_ratio"] is not None]
        ),
        "normalized_gap_median": med([abs(pair["normalized_gap"]) for pair in complete_pairs]),
        "symmetric_alignment_pair_count": sum(
            pair["alignment_convention"] == "symmetric_both_words" for pair in complete_pairs
        ),
        "row_tie_summary": {
            "rows_with_pairs": len(row_tie),
            "rows_with_nonzero_gap_range": sum(item["range"] > 1e-15 for item in row_tie),
            "median_gap_range": med([item["range"] for item in row_tie]),
            "max_gap_range": max((item["range"] for item in row_tie), default=None),
        },
    }

    associations = {}
    for key, bucket_rows in by.items():
        bucket_pairs = [pair for row in bucket_rows if complete(row) for pair in pby[row["name"]]]
        for label, fn in [
            ("beta_product", lambda pair: val(pair["beta_product"]) if pair["beta_product"] else None),
            ("pairing_factor", lambda pair: pair["normalized_pairing_factor"]),
            ("cancellation_ratio", lambda pair: pair["cancellation_ratio"]),
        ]:
            values = [(abs(pair["normalized_gap"]), fn(pair)) for pair in bucket_pairs if fn(pair) is not None]
            associations.setdefault(label, []).append(
                {
                    "bucket": f"{key[0]}x{key[1]}",
                    "n": len(values),
                    "spearman": spearman([a for a, _ in values], [b for _, b in values]),
                }
            )

    summary = {
        "schema": "product-bounce-class-degeneration/v2",
        "alignment_convention": "symmetric_both_words",
        "rows": len(rows),
        "pairs": len(pairs),
        "complete_rows": len(complete_rows),
        "same_support_rows": sum(row["exact_support_equal"] for row in complete_rows),
        "support_equality_rate": observed,
        "threshold_support_rates": {str(t): qshare(complete_rows, t) for t in [0.001, 0.01, 0.05]},
        "buckets": buckets,
        "shuffle_control": {
            "population": "complete_rows_only",
            "population_rows": len(complete_rows),
            "seed": 20260713,
            "permutations": 100,
            "observed_rate": observed,
            "shuffle_mean": statistics.mean([x for x in shuffle if x is not None]) if shuffle else None,
            "shuffle_q95": sorted(x for x in shuffle if x is not None)[int(0.95 * len(shuffle)) - 1] if shuffle else None,
        },
        "primary": primary,
        "associations": associations,
        "controls": {
            "cyclic_invariance": "asserted by producer for every rotation of both words",
            "within_word_rotation_beta_q_invariance": "asserted by producer for every cyclic rotation of each word",
            "symmetric_alignment_fixture": {
                "pair_records": len(complete_pairs),
                "a2_rotations_per_pair": sorted({pair["a2_rotation_count"] for pair in complete_pairs}),
                "a3_rotations_per_pair": sorted({pair["a3_rotation_count"] for pair in complete_pairs}),
                "joint_rotation_candidates_per_pair": sorted({pair["a2_rotation_count"] * pair["a3_rotation_count"] for pair in complete_pairs}),
                "purpose": "distinguishes independent two-word cyclic alignment from a one-sided fixed-cut recount",
            },
            "factor_swap": "omitted: no cheap standalone factor relabeling fixture was available without duplicating geometry construction",
        },
        "input_artifacts": {"rows_sha256": sha(args.rows), "pairs_sha256": sha(args.pairs)},
    }
    Path(args.out).write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
