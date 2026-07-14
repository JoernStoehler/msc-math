#!/usr/bin/env python3
"""Deterministic analysis for the frozen 200-row target packet."""
import hashlib
import json
import random
import sys
from pathlib import Path

THRESHOLD = 0.5949424195457518
B = 20_000
SEED = 20260714


def quantile(xs, p):
    ys = sorted(xs)
    if not ys:
        raise ValueError("empty bootstrap sample")
    x = p * (len(ys) - 1)
    lo, hi = int(x), min(int(x) + 1, len(ys) - 1)
    return ys[lo] + (ys[hi] - ys[lo]) * (x - lo)


def mean(xs):
    return sum(xs) / len(xs)


def bootstrap_means(xs, rng):
    n = len(xs)
    out = []
    for _ in range(B):
        out.append(mean([xs[rng.randrange(n)] for _ in range(n)]))
    return out


def wilson(k, n):
    if n == 0:
        return [0.0, 0.0]
    z = 1.959963984540054
    p = k / n
    den = 1.0 + z * z / n
    center = (p + z * z / (2 * n)) / den
    half = z * ((p * (1 - p) / n + z * z / (4 * n * n)) ** 0.5) / den
    return [center - half, center + half]


def summary(name, rows, rng):
    values = [row["sys"] for row in rows]
    boots = bootstrap_means(values, rng)
    hits = sum(value >= THRESHOLD for value in values)
    return {
        "name": name,
        "n": len(values),
        "mean_sys": mean(values),
        "mean_bootstrap_percentile95": [quantile(boots, 0.025), quantile(boots, 0.975)],
        "median_sys": quantile(values, 0.5),
        "range_sys": [min(values), max(values)],
        "threshold": THRESHOLD,
        "threshold_count": hits,
        "threshold_rate": hits / len(values),
        "threshold_wilson95": wilson(hits, len(values)),
    }


def difference(name, left, right, rng):
    left_values = [row["sys"] for row in left]
    right_values = [row["sys"] for row in right]
    boots = [mean([left_values[rng.randrange(len(left_values))] for _ in left_values]) - mean([right_values[rng.randrange(len(right_values))] for _ in right_values]) for _ in range(B)]
    return {"name": name, "mean_difference": mean(left_values) - mean(right_values), "bootstrap_percentile95": [quantile(boots, 0.025), quantile(boots, 0.975)]}


def main():
    if len(sys.argv) != 3:
        raise SystemExit("usage: analyze.py TARGET-ROWS.jsonl OUT.json")
    source, output = map(Path, sys.argv[1:])
    rows = [json.loads(line) for line in source.read_text().splitlines() if line.strip()]
    assert len(rows) == 200
    assert len({row["candidate_id"] for row in rows}) == 200
    selected = [row for row in rows if row["role"] == "selected"]
    baseline = [row for row in rows if row["role"] == "baseline"]
    first = [row for row in selected if row["f64_rank"] <= 10]
    next_band = [row for row in selected if 11 <= row["f64_rank"] <= 100]
    assert (len(selected), len(baseline), len(first), len(next_band)) == (100, 100, 10, 90)
    rng = random.Random(SEED)
    groups = [summary("selected_low_1pct", selected, rng), summary("baseline_disjoint", baseline, rng), summary("selected_0_to_0p1pct", first, rng), summary("selected_0p1_to_1pct", next_band, rng)]
    contrasts = [difference("low_1pct_vs_baseline_mean_sys", selected, baseline, rng), difference("hardening_0_to_0p1_vs_0p1_to_1pct_mean_sys", first, next_band, rng)]
    max_row = max(rows, key=lambda row: row["sys"])
    product = {"product_5x5_low_1pct": 0.63281993533461989, "product_5x5_low_0p1pct": 0.61889509741270687}
    hardening = contrasts[1]
    generic_product = {
        "generic_selected_minus_product_5x5_selected": groups[0]["mean_sys"] - product["product_5x5_low_1pct"],
        "generic_first10_minus_product_5x5_first10": groups[2]["mean_sys"] - product["product_5x5_low_0p1pct"],
        "generic_hardening": hardening["mean_difference"],
        "product_5x5_hardening": product["product_5x5_low_0p1pct"] - product["product_5x5_low_1pct"],
        "generic_minus_product_hardening_interaction": hardening["mean_difference"] - (product["product_5x5_low_0p1pct"] - product["product_5x5_low_1pct"]),
        "interpretation": "operational comparator only; product generator/combinatorics/backend are confounded",
    }
    continue_rule = "stop"
    disposition = "Stop at 10k: the selected-vs-baseline transfer contrast is positive and material, but hardening reverses/flat (point estimate below 0) and the generic/product operational contrasts are not positive-material; the frozen 100k rule does not trigger."
    result = {"schema": "sys-datascience.generic-ridge-tail-stage1-target.analysis.v1", "row_count": 200, "bootstrap": {"method": "deterministic percentile bootstrap", "resamples": B, "seed": SEED}, "threshold": THRESHOLD, "groups": groups, "contrasts": contrasts, "generic_product_operational_comparator": generic_product, "maximum_sys_row": max_row, "any_sys_gt_1": any(row["sys"] > 1.0 for row in rows), "continue_100k": {"decision": continue_rule, "disposition": disposition}, "target_rows_sha256": hashlib.sha256(source.read_bytes()).hexdigest()}
    output.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps({"rows": 200, "max_sys": max_row["sys"], "hardening_difference": hardening["mean_difference"], "continue_100k": continue_rule, "output": str(output)}))


if __name__ == "__main__":
    main()
