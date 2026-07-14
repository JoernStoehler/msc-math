#!/usr/bin/env python3
"""Cheap existing-row checks for the bounce-class mechanism design gate.

This script performs no capacity or geometry evaluation. It consumes the
reviewed raw/class-minima artifacts and, when supplied, the deterministically
rebuilt prepared table.
"""

import argparse
import hashlib
import json
import math
from collections import Counter, defaultdict
from pathlib import Path
from statistics import fmean, median


EXPECTED_SHA256 = {
    "raw": "66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736",
    "classes": "187089804bd17fdac76bdaf51a8d8202e67fb2b14779fe9a418cc8da47c7b4c4",
    "table": "49825d7636246f71f4ebd419cf0ccbc86e39e6b7f43d4b03e889bb85e4887aea",
    "provenance": "6ff88a5accce9a7ec7e5a494107350b0974b2ce0268ea44caae36a18a7494ef2",
}


def sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def read_jsonl(path):
    with open(path, encoding="utf-8") as f:
        return [json.loads(line) for line in f if line.strip()]


def mean_sd(xs):
    mu = fmean(xs)
    sd = math.sqrt(fmean((x - mu) ** 2 for x in xs))
    return mu, sd


def solve(a, y):
    n = len(a)
    aug = [row[:] + [rhs] for row, rhs in zip(a, y)]
    for col in range(n):
        pivot = max(range(col, n), key=lambda r: abs(aug[r][col]))
        if abs(aug[pivot][col]) < 1e-11:
            raise ValueError(f"singular design at column {col}")
        aug[col], aug[pivot] = aug[pivot], aug[col]
        scale = aug[col][col]
        aug[col] = [v / scale for v in aug[col]]
        for row in range(n):
            if row == col:
                continue
            factor = aug[row][col]
            aug[row] = [v - factor * w for v, w in zip(aug[row], aug[col])]
    return [aug[i][-1] for i in range(n)]


def ols(y, x, names):
    p = len(names)
    xtx = [[0.0] * p for _ in range(p)]
    xty = [0.0] * p
    for yy, row in zip(y, x):
        for j in range(p):
            xty[j] += row[j] * yy
            for k in range(p):
                xtx[j][k] += row[j] * row[k]
    beta = solve(xtx, xty)
    return dict(zip(names, beta))


def ranks(xs):
    order = sorted(range(len(xs)), key=lambda i: xs[i])
    out = [0.0] * len(xs)
    i = 0
    while i < len(xs):
        j = i + 1
        while j < len(xs) and xs[order[j]] == xs[order[i]]:
            j += 1
        rank = (i + j - 1) / 2.0
        for q in range(i, j):
            out[order[q]] = rank
        i = j
    return out


def pearson(xs, ys):
    mx, my = fmean(xs), fmean(ys)
    dx = sum((x - mx) ** 2 for x in xs)
    dy = sum((y - my) ** 2 for y in ys)
    if dx == 0 or dy == 0:
        return None
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / math.sqrt(dx * dy)


def spearman(xs, ys):
    return pearson(ranks(xs), ranks(ys))


def quantile(xs, p):
    ys = sorted(xs)
    q = (len(ys) - 1) * p
    lo, hi = math.floor(q), math.ceil(q)
    return ys[lo] if lo == hi else ys[lo] + (q - lo) * (ys[hi] - ys[lo])


def factor_controls(raw):
    q_duals = raw["dual_vertices"][: raw["k"]]
    p_duals = raw["dual_vertices"][raw["k"] :]

    def summaries(duals, offset):
        hs = []
        angles = []
        for a in duals:
            norm = math.hypot(a[offset], a[offset + 1])
            hs.append(1.0 / norm)
            angles.append(math.atan2(a[offset + 1], a[offset]) % math.tau)
        angles.sort()
        gaps = [((angles[(i + 1) % len(angles)] - angles[i]) % math.tau) for i in range(len(angles))]
        logs = [math.log(h) for h in hs]
        _, sd = mean_sd(logs)
        return fmean(logs), sd, min(gaps)

    qmean, qsd, qgap = summaries(q_duals, 0)
    pmean, psd, pgap = summaries(p_duals, 2)
    return {
        "q_mean_log_support": qmean,
        "p_mean_log_support": pmean,
        "q_sd_log_support": qsd,
        "p_sd_log_support": psd,
        "q_min_angle_gap": qgap,
        "p_min_angle_gap": pgap,
    }


def load_rows(args):
    inputs = {"raw": args.raw, "classes": args.classes}
    if args.table and args.provenance:
        inputs.update({"table": args.table, "provenance": args.provenance})
    for name, path in inputs.items():
        actual = sha256(path)
        if actual != EXPECTED_SHA256[name]:
            raise ValueError(f"{name} SHA-256 mismatch: {actual}")
    raw_rows = read_jsonl(args.raw)
    classes = read_jsonl(args.classes)
    raw = {r["name"]: r for r in raw_rows}
    if len(raw) != len(raw_rows):
        raise ValueError("raw name is not unique")
    if len({r["name"] for r in classes}) != len(classes):
        raise ValueError("class-minima name is not unique")
    if set(raw) != {r["name"] for r in classes}:
        raise ValueError("raw/class-minima name sets differ")

    prepared = {}
    if args.table and args.provenance:
        table_rows = read_jsonl(args.table)
        provenance_rows = read_jsonl(args.provenance)
        table = {r["poly_id"]: r for r in table_rows}
        if len(table) != len(table_rows):
            raise ValueError("prepared table poly_id is not unique")
        for p in provenance_rows:
            if p.get("role") == "random_product_sample":
                if p["source_name"] in prepared:
                    raise ValueError("prepared product source_name is not unique")
                prepared[p["source_name"]] = table[p["poly_id"]]
        if set(prepared) != set(raw):
            raise ValueError("prepared product/raw name sets differ")

    rows = []
    for c in classes:
        r = raw[c["name"]]
        expected = {
            "k": r["k"], "m": r["m"], "producer_bounces": r["bounces"],
            "stored_volume": r["volume"], "stored_capacity": r["capacity"],
            "stored_sys": r["sys"],
        }
        if any(c[key] != value for key, value in expected.items()):
            raise ValueError(f"raw/class key-field mismatch: {c['name']}")
        item = dict(c)
        item.update(factor_controls(r))
        item["bucket"] = f'{c["k"]}x{c["m"]}'
        item["bounce3"] = float(c["producer_bounces"] == 3)
        item["capacity_term"] = 2.0 * math.log(c["stored_capacity"])
        item["volume_term"] = -math.log(2.0 * c["stored_volume"])
        item["log_sys"] = math.log(c["stored_sys"])
        item["inactive_winner_facets"] = c["k"] + c["m"] - min(
            c["active_vertex_counts"][str(c["producer_bounces"])]
        )
        if c["class_minima"]["3"] is not None:
            a2 = c["class_minima"]["2"]["action"]
            a3 = c["class_minima"]["3"]["action"]
            item["abs_log_action_ratio"] = abs(math.log(a3 / a2))
            item["s2"] = a2 * a2 / (2.0 * c["stored_volume"])
            item["s3"] = a3 * a3 / (2.0 * c["stored_volume"])
        if prepared:
            item.update(prepared[c["name"]])
        rows.append(item)
    return rows


def component_model(rows, controls):
    buckets = sorted({r["bucket"] for r in rows})
    control_values = {}
    for control in controls:
        for bucket in buckets:
            rs = [r for r in rows if r["bucket"] == bucket]
            mu, sd = mean_sd([r[control] for r in rs])
            control_values[(control, bucket)] = (mu, sd)
    names = [f"bucket:{b}" for b in buckets] + ["bounce3"] + list(controls)
    x = []
    for r in rows:
        row = [float(r["bucket"] == b) for b in buckets] + [r["bounce3"]]
        for control in controls:
            mu, sd = control_values[(control, r["bucket"])]
            row.append(0.0 if sd == 0 else (r[control] - mu) / sd)
        x.append(row)
    coeff = {}
    for outcome in ("log_sys", "capacity_term", "volume_term"):
        coeff[outcome] = ols([r[outcome] for r in rows], x, names)["bounce3"]
    residual = coeff["log_sys"] - coeff["capacity_term"] - coeff["volume_term"]
    return {
        "rows": len(rows),
        "controls": list(controls),
        "bounce3_coefficients": coeff,
        "additivity_residual": residual,
        "capacity_fraction_of_log_sys_coefficient": coeff["capacity_term"] / coeff["log_sys"],
        "volume_fraction_of_log_sys_coefficient": coeff["volume_term"] / coeff["log_sys"],
    }


def raw_bucket_decomposition(rows):
    out = {}
    for bucket in sorted({r["bucket"] for r in rows}):
        rs = [r for r in rows if r["bucket"] == bucket]
        by = {b: [r for r in rs if r["producer_bounces"] == b] for b in (2, 3)}
        if not by[2] or not by[3]:
            continue
        out[bucket] = {
            "rows": len(rs),
            "two_rows": len(by[2]),
            "three_rows": len(by[3]),
            **{
                key: fmean(r[key] for r in by[3]) - fmean(r[key] for r in by[2])
                for key in ("log_sys", "capacity_term", "volume_term")
            },
        }
    return out


def gap_checks(rows):
    complete = [r for r in rows if "abs_log_action_ratio" in r]
    by_bucket = {}
    high, rest = [], []
    for bucket in sorted({r["bucket"] for r in complete}):
        rs = [r for r in complete if r["bucket"] == bucket]
        cutoff = quantile([r["stored_sys"] for r in rs], 0.9)
        hi = [r for r in rs if r["stored_sys"] >= cutoff]
        lo = [r for r in rs if r["stored_sys"] < cutoff]
        high.extend(hi)
        rest.extend(lo)
        by_bucket[bucket] = {
            "rows": len(rs),
            "spearman_abs_log_action_ratio_vs_sys": spearman(
                [r["abs_log_action_ratio"] for r in rs], [r["stored_sys"] for r in rs]
            ),
            "top_decile_median_abs_log_action_ratio": median(
                r["abs_log_action_ratio"] for r in hi
            ),
            "remainder_median_abs_log_action_ratio": median(
                r["abs_log_action_ratio"] for r in lo
            ),
        }
    return {
        "complete_rows": len(complete),
        "by_bucket": by_bucket,
        "pooled_within_bucket_top_decile": {
            "rows": len(high),
            "median_abs_log_action_ratio": median(r["abs_log_action_ratio"] for r in high),
            "near_tie_rate_abs_log_ratio_le_0_01": fmean(
                r["abs_log_action_ratio"] <= 0.01 for r in high
            ),
        },
        "pooled_remainder": {
            "rows": len(rest),
            "median_abs_log_action_ratio": median(r["abs_log_action_ratio"] for r in rest),
            "near_tie_rate_abs_log_ratio_le_0_01": fmean(
                r["abs_log_action_ratio"] <= 0.01 for r in rest
            ),
        },
    }


def active_support_checks(rows):
    winner_lengths = defaultdict(Counter)
    inactive = defaultdict(Counter)
    active_factor_counts = defaultdict(Counter)
    for r in rows:
        b = str(r["producer_bounces"])
        winner_lengths[r["producer_bounces"]].update(r["active_vertex_counts"][b])
        inactive[r["producer_bounces"]][r["inactive_winner_facets"]] += 1
        supports = {
            tuple(sorted(set(sigma)))
            for sigma in r["class_minima"][b]["minimizer_sigmas"]
        }
        if len(supports) != 1:
            raise ValueError(f"winner minimizer supports differ: {r['name']}")
        support = set(next(iter(supports)))
        factor_counts = {
            (sum(i < r["k"] for i in set(sigma)), sum(i >= r["k"] for i in set(sigma)))
            for sigma in r["class_minima"][b]["minimizer_sigmas"]
        }
        if len(factor_counts) != 1:
            raise ValueError(f"winner minimizer factor counts differ: {r['name']}")
        q_active, p_active = next(iter(factor_counts))
        active_factor_counts[r["producer_bounces"]][f"{q_active}q+{p_active}p"] += 1
    return {
        "winner_minimizer_word_lengths": {str(k): dict(sorted(v.items())) for k, v in winner_lengths.items()},
        "rows_by_inactive_winner_facet_count": {str(k): dict(sorted(v.items())) for k, v in inactive.items()},
        "rows_by_winner_active_factor_counts": {
            str(k): dict(sorted(v.items())) for k, v in active_factor_counts.items()
        },
        "a3_eight_facet_rows_by_producer_label": {
            str(b): sum(
                r["producer_bounces"] == b and 8 in r["active_vertex_counts"]["3"] for r in rows
            )
            for b in (2, 3)
        },
    }


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--raw", required=True)
    ap.add_argument("--classes", required=True)
    ap.add_argument("--table")
    ap.add_argument("--provenance")
    ap.add_argument("--out", required=True)
    args = ap.parse_args()
    rows = load_rows(args)

    adjustments = {
        "bucket_only": [],
        "generator_coordinates": [
            "q_mean_log_support", "p_mean_log_support",
            "q_sd_log_support", "p_sd_log_support",
            "q_min_angle_gap", "p_min_angle_gap",
        ],
    }
    if args.table and args.provenance:
        adjustments.update({
            "ridge_distribution": [
                "ridge_symp_area_normalized_entropy", "ridge_symp_area_max_share"
            ],
            "generator_plus_ridge_distribution": [
                "q_mean_log_support", "p_mean_log_support",
                "q_sd_log_support", "p_sd_log_support",
                "q_min_angle_gap", "p_min_angle_gap",
                "ridge_symp_area_normalized_entropy", "ridge_symp_area_max_share",
            ],
            "prior_ridge_triplet_sensitivity": [
                "ridge_symp_area_normalized_entropy",
                "ridge_symp_area_max_share",
                "ridge_symp_area_le_1em2_over_volume_sqrt_fraction",
            ],
            "ridge_magnitude_sensitivity": [
                "ridge_symp_area_normalized_entropy", "ridge_symp_area_max_share",
                "ridge_symp_area_sum_over_volume_sqrt",
            ],
        })

    complete = [r for r in rows if r["class_minima"]["3"] is not None]
    labels_by_bucket = defaultdict(set)
    for row in complete:
        labels_by_bucket[row["bucket"]].add(row["producer_bounces"])
    complete_overlap = [
        row for row in complete if labels_by_bucket[row["bucket"]] == {2, 3}
    ]
    result = {
        "schema": "product-bounce-mechanism/v1",
        "question": (
            "How much of the retained within-(k,m) association between the producer's "
            "two-/three-bounce winner label and sys is carried by unscaled capacity "
            "versus volume, how sensitive is it to existing generator/shape controls, "
            "and do active-support counts or class balance support the proposed mechanisms?"
        ),
        "inputs": {
            name: {"path": str(Path(path)), "sha256": sha256(path)}
            for name, path in {
                "raw": args.raw,
                "classes": args.classes,
                **({"table": args.table, "provenance": args.provenance}
                   if args.table and args.provenance else {}),
            }.items()
        },
        "row_counts": {
            "all": len(rows),
            "complete_class_support": len(complete),
            "complete_class_support_within_overlap_buckets": len(complete_overlap),
        },
        "active_support": active_support_checks(rows),
        "raw_bucket_decomposition_all_rows": raw_bucket_decomposition(rows),
        "raw_bucket_decomposition_complete_rows": raw_bucket_decomposition(complete_overlap),
        "component_models_all_rows": {
            name: component_model(rows, controls) for name, controls in adjustments.items()
        },
        "component_models_complete_rows": {
            name: component_model(complete_overlap, controls) for name, controls in adjustments.items()
        },
        "class_balance": gap_checks(rows),
        "threshold_check": {
            "max_stored_sys": max(r["stored_sys"] for r in rows),
            "stored_sys_gt_1_rows": sum(r["stored_sys"] > 1.0 for r in rows),
        },
        "analysis_contract": {
            "component_identity": "log(sys) = 2 log(capacity) - log(2 volume)",
            "model": (
                "OLS with exact (k,m) fixed effects; every listed control is "
                "standardized within (k,m); reported values are bounce3 coefficients"
            ),
            "complete_overlap": (
                "rows with both class minima, restricted to buckets containing both "
                "producer labels; this removes 3x3, whose complete rows are all label 3"
            ),
            "class_balance": (
                "absolute log(A3/A2) is non-tautological with sys=min(s2,s3); A3-null "
                "rows are excluded rather than assigned a numeric gap"
            ),
        },
        "interpretation_boundary": (
            "Existing retained rows and target-derived bounce/class fields only. "
            "Component regressions are exact algebraic decompositions of a descriptive "
            "bounce-label coefficient, not causal mediation. Complete-support checks exclude "
            "A3-null availability states. Ridge-magnitude controls contain volume normalization."
        ),
    }
    Path(args.out).write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
