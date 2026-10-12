#!/usr/bin/env python3
"""Describe sys distributions by producer-owned product_bounces.

This is deliberately an existing-data analysis: it performs no capacity or
geometry evaluations.  It joins the reviewed normalized table to provenance,
restricts to retained random-product rows, and treats (product_k, product_m)
as the predeclared comparison buckets.
"""
import argparse
import hashlib
import json
import math
from collections import defaultdict
from pathlib import Path
from statistics import fmean, median


BOUNCES = (2, 3)
RIDGE_FEATURES = (
    "ridge_symp_area_normalized_entropy",
    "ridge_symp_area_max_share",
    "ridge_symp_area_le_1em2_over_volume_sqrt_fraction",
)


def sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def load_jsonl(path):
    with open(path, encoding="utf-8") as f:
        return [json.loads(line) for line in f if line.strip()]


def quantile(values, p):
    """Linear interpolation, explicitly fixed before inspecting results."""
    xs = sorted(values)
    pos = (len(xs) - 1) * p
    lo, hi = math.floor(pos), math.ceil(pos)
    return xs[lo] if lo == hi else xs[lo] + (pos - lo) * (xs[hi] - xs[lo])


def describe(values):
    return {
        "n": len(values), "mean": fmean(values), "median": median(values),
        "q10": quantile(values, .10), "q25": quantile(values, .25),
        "q75": quantile(values, .75), "q90": quantile(values, .90),
        "q95": quantile(values, .95),
    }


def mean_std(values):
    mu = fmean(values)
    return mu, math.sqrt(sum((x - mu) ** 2 for x in values) / len(values))


def solve(a, y):
    """Solve normal equations by Gaussian elimination with partial pivoting."""
    n = len(a)
    aug = [row[:] + [rhs] for row, rhs in zip(a, y)]
    for col in range(n):
        pivot = max(range(col, n), key=lambda r: abs(aug[r][col]))
        if abs(aug[pivot][col]) < 1e-12:
            raise ValueError("singular regression design")
        aug[col], aug[pivot] = aug[pivot], aug[col]
        scale = aug[col][col]
        aug[col] = [v / scale for v in aug[col]]
        for row in range(n):
            if row != col:
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
    pred = [sum(b * v for b, v in zip(beta, row)) for row in x]
    ss_res = sum((yy - pp) ** 2 for yy, pp in zip(y, pred))
    mu = fmean(y)
    ss_tot = sum((yy - mu) ** 2 for yy in y)
    return {"coefficients": dict(zip(names, beta)), "r_squared": 1 - ss_res / ss_tot}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--table", required=True)
    ap.add_argument("--provenance", required=True)
    ap.add_argument("--out", required=True)
    args = ap.parse_args()
    out = Path(args.out)
    table_rows, prov_rows = load_jsonl(args.table), load_jsonl(args.provenance)
    table = {r["poly_id"]: r for r in table_rows}
    prov = {r["poly_id"]: r for r in prov_rows}
    if len(table) != len(table_rows) or len(prov) != len(prov_rows):
        raise ValueError("poly_id is not unique within an input")
    if set(table) != set(prov):
        raise ValueError("poly_id sets differ between inputs")
    rows = []
    for poly_id, provenance in prov.items():
        if provenance.get("role") == "random_product_sample":
            table_row = table[poly_id]
            bounce = provenance.get("product_bounces")
            k, m = provenance.get("product_k"), provenance.get("product_m")
            if bounce not in BOUNCES or not all(isinstance(v, int) for v in (k, m)):
                raise ValueError(f"bad retained random-product provenance: {poly_id}")
            if not math.isfinite(table_row["sys"]):
                raise ValueError(f"non-finite sys: {poly_id}")
            rows.append({"poly_id": poly_id, "k": k, "m": m, "bounce": bounce,
                         "sys": table_row["sys"],
                         **{key: table_row[key] for key in RIDGE_FEATURES}})
    buckets = defaultdict(list)
    for row in rows:
        buckets[(row["k"], row["m"])].append(row)
    if any(set(r["bounce"] for r in rs) != set(BOUNCES) for rs in buckets.values()):
        raise ValueError("a (k,m) bucket lacks one bounce class")

    bucket_results, standardized = {}, []
    for bucket, rs in sorted(buckets.items()):
        sys_all = [r["sys"] for r in rs]
        thresholds = {f"q{q}": quantile(sys_all, q / 100) for q in (10, 90, 95)}
        mu, sd = mean_std(sys_all)
        classes = {}
        for bounce in BOUNCES:
            vals = [r["sys"] for r in rs if r["bounce"] == bounce]
            tail = {}
            for label, threshold in thresholds.items():
                lower = label == "q10"
                count = sum(v <= threshold if lower else v >= threshold for v in vals)
                tail[label] = {"count": count, "rate": count / len(vals)}
            classes[str(bounce)] = {**describe(vals), "tail_membership": tail,
                                    "mean_bucket_z": (fmean(vals) - mu) / sd}
        c2, c3 = classes["2"], classes["3"]
        bucket_results[f"{bucket[0]}x{bucket[1]}"] = {
            "bucket": {"k": bucket[0], "m": bucket[1]}, "all": describe(sys_all),
            "thresholds": thresholds, "by_bounces": classes,
            "three_minus_two_mean": c3["mean"] - c2["mean"],
            "three_minus_two_median": c3["median"] - c2["median"],
            "three_minus_two_bucket_z": c3["mean_bucket_z"] - c2["mean_bucket_z"],
        }
        for row in rs:
            standardized.append({**row, "sys_z": (row["sys"] - mu) / sd})

    pooled = {str(bounce): describe([r["sys"] for r in rows if r["bounce"] == bounce])
              for bounce in BOUNCES}
    for bounce in BOUNCES:
        zs = [r["sys_z"] for r in standardized if r["bounce"] == bounce]
        pooled[str(bounce)]["mean_bucket_z"] = fmean(zs)
        pooled[str(bounce)]["median_bucket_z"] = median(zs)
    pooled_tails = {}
    for label in ("q10", "q90", "q95"):
        item = {}
        for bounce in BOUNCES:
            rates = [x["by_bounces"][str(bounce)]["tail_membership"][label]["rate"]
                     for x in bucket_results.values()]
            count = sum(x["by_bounces"][str(bounce)]["tail_membership"][label]["count"]
                        for x in bucket_results.values())
            denom = sum(x["by_bounces"][str(bounce)]["n"]
                        for x in bucket_results.values())
            item[str(bounce)] = {"count": count, "n": denom, "rate": count / denom,
                                 "equal_bucket_mean_rate": fmean(rates)}
        item["three_minus_two_rate"] = item["3"]["rate"] - item["2"]["rate"]
        pooled_tails[label] = item

    deltas = [x["three_minus_two_mean"] for x in bucket_results.values()]
    z_deltas = [x["three_minus_two_bucket_z"] for x in bucket_results.values()]
    direction = {
        "mean_positive_buckets": sum(x > 0 for x in deltas),
        "mean_negative_buckets": sum(x < 0 for x in deltas),
        "bucket_z_positive_buckets": sum(x > 0 for x in z_deltas),
        "bucket_z_negative_buckets": sum(x < 0 for x in z_deltas),
        "total_buckets": len(bucket_results),
    }

    # One predeclared adjustment, not a model search: within-bucket standardized
    # sys on bounce indicator, first alone then with three ridge summaries also
    # standardized within (k,m).
    y = [r["sys_z"] for r in standardized]
    base_x = [[1.0, float(r["bounce"] == 3)] for r in standardized]
    ridge_stats = {}
    for feature in RIDGE_FEATURES:
        for bucket, rs in buckets.items():
            mu, sd = mean_std([r[feature] for r in rs])
            for row in standardized:
                if (row["k"], row["m"]) == bucket:
                    row[feature + "_z"] = 0.0 if sd == 0 else (row[feature] - mu) / sd
        ridge_stats[feature] = {"zero_variance_buckets": sum(
            mean_std([r[feature] for r in rs])[1] == 0 for rs in buckets.values())}
    adjusted_x = [row + [r[f + "_z"] for f in RIDGE_FEATURES]
                  for row, r in zip(base_x, standardized)]
    reduced = ols(y, base_x, ["intercept", "bounce_3"])
    adjusted = ols(y, adjusted_x, ["intercept", "bounce_3", *RIDGE_FEATURES])

    result = {
        "question": "Descriptive association of producer-owned product_bounces (2 vs 3) with sys among retained random-product rows; no new target evaluations.",
        "input": {"table": str(Path(args.table).resolve()), "table_sha256": sha256(args.table),
                  "provenance": str(Path(args.provenance).resolve()),
                  "provenance_sha256": sha256(args.provenance)},
        "join_validation": {"table_rows": len(table_rows), "provenance_rows": len(prov_rows),
                            "table_unique_poly_ids": len(table),
                            "provenance_unique_poly_ids": len(prov),
                            "identical_poly_id_sets": True,
                            "retained_random_product_rows": len(rows)},
        "definitions": {
            "buckets": "exact (product_k, product_m)",
            "quantiles": "linear interpolation on all retained rows within each bucket",
            "tail_rule": "q10: sys <= threshold; q90/q95: sys >= threshold",
            "bucket_standardization": "(sys - within-bucket mean) / within-bucket population SD",
        },
        "by_bucket": bucket_results,
        "pooled": {
            "by_bounces": pooled, "tails": pooled_tails,
            "three_minus_two_mean_bucket_z":
                pooled["3"]["mean_bucket_z"] - pooled["2"]["mean_bucket_z"],
            "direction_consistency": direction,
        },
        "ridge_adjustment": {
            "method": "OLS on within-(k,m)-standardized sys; bounce_3 only versus bounce_3 plus three predeclared within-bucket-standardized ridge summaries",
            "ridge_features": RIDGE_FEATURES, "feature_checks": ridge_stats,
            "reduced": reduced, "adjusted": adjusted,
            "bounce_coefficient_retention":
                adjusted["coefficients"]["bounce_3"] / reduced["coefficients"]["bounce_3"],
        },
        "interpretation_boundary": "This is an observational, within-retained-sample association. product_bounces is producer metadata, not an independently validated proposer or a causal mechanism. Ridge adjustment is descriptive and cannot establish mediation or explanation.",
    }
    out.mkdir(parents=True, exist_ok=True)
    (out / "summary.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps(result, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
