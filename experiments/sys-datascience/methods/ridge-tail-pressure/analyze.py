#!/usr/bin/env python3
"""Stratified ridge/sys tail diagnostics for retained and frozen 1M inputs.

The normal path uses tracked retained/100k sources and this packet's small
1M input snapshot.  ``--one-m-feature-table`` is an optional identity audit of
the 2.5 GB full feature scan; it is not needed to reproduce the results because
the frozen selected and evaluated rows record the selection-feature values.
This analysis intentionally never estimates a probability of sys > 1.
"""
from __future__ import annotations

import argparse
import csv
import hashlib
import json
import math
import random
import sys
from collections import defaultdict
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
REPO: Path
RETAINED: Path
PRODUCT: Path
FEATURES: Path | None
EVALUATED: Path
CONCENTRATION: Path
ONE_M_PLAN: Path
ONE_M_SELECTED: Path
CONCENTRATION_PLAN: Path
RIDGE = "ridge_symp_area_sum_over_volume_sqrt"
QS = (0.10, 0.05, 0.02, 0.01)
FULL_1M_FEATURE_TABLE_SHA256 = "9af141c74510ea5309b77ebc4abd0190b4c6a6f6efd9c5d2d67c3059c25af36d"


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for block in iter(lambda: f.read(1024 * 1024), b""):
            h.update(block)
    return h.hexdigest()


def repo_root(start: Path) -> Path:
    """Find the checkout from this packet path or an explicitly supplied path."""
    for candidate in (start, *start.parents):
        if (candidate / "experiments/polytope-invariant-table/polytope-table.jsonl").is_file():
            return candidate
    raise FileNotFoundError("could not find repo root; pass --repo-root")


def read_jsonl(path: Path):
    with path.open() as f:
        for line in f:
            if line.strip():
                yield json.loads(line)


def quantile(xs: list[float], q: float) -> float:
    """R-7 sample quantile, used only for descriptive conditional sys columns."""
    xs = sorted(xs)
    if len(xs) == 1:
        return xs[0]
    position = (len(xs) - 1) * q
    lo, hi = math.floor(position), math.ceil(position)
    return xs[lo] + (position - lo) * (xs[hi] - xs[lo])


def wilson(x: int, n: int, z: float = 1.959963984540054) -> tuple[float, float]:
    if not n:
        return (math.nan, math.nan)
    p = x / n
    d = 1 + z * z / n
    c = (p + z * z / (2 * n)) / d
    r = z * math.sqrt((p * (1 - p) + z * z / (4 * n)) / n) / d
    return c - r, c + r


def bootstrap_stratified(rows_by_bucket, q: float, reps: int, seed: int):
    """Percentile CI for the conditional high-sys rate after re-ranking in buckets."""
    rng = random.Random(seed)
    buckets = list(rows_by_bucket.values())
    vals = []
    for _ in range(reps):
        hit = total = 0
        for rows in buckets:
            sample = [rows[rng.randrange(len(rows))] for _ in range(len(rows))]
            n = len(sample)
            take = math.ceil(n * q)
            low = set(sorted(range(n), key=lambda i: sample[i][0])[:take])
            high = set(sorted(range(n), key=lambda i: sample[i][1], reverse=True)[:take])
            hit += len(low & high)
            total += take
        vals.append(hit / total)
    return quantile(vals, .025), quantile(vals, .975)


def write_tsv(path: Path, rows: list[dict]):
    with path.open("w", newline="") as f:
        fields = list(rows[0]) if rows else []
        out = csv.DictWriter(f, fieldnames=fields, delimiter="\t", lineterminator="\n")
        out.writeheader()
        out.writerows(rows)


def retained_analysis():
    # This source has no poly_id. sys is unique in both current inputs, yielding
    # an exact one-to-one join, checked below rather than assumed from order.
    product_by_sys = {}
    for r in read_jsonl(PRODUCT):
        key = repr(r["sys"])
        assert key not in product_by_sys
        product_by_sys[key] = (r["k"], r["m"])
    rows_by_bucket = defaultdict(list)
    joined = 0
    for r in read_jsonl(RETAINED):
        if r["capacity_source"] != "random_product_sample":
            continue
        key = repr(r["sys"])
        assert key in product_by_sys, key
        bucket = product_by_sys[key]
        rows_by_bucket[bucket].append((float(r[RIDGE]), float(r["sys"])))
        joined += 1
    assert joined == len(product_by_sys) == 10240
    assert len(rows_by_bucket) == 10 and {len(x) for x in rows_by_bucket.values()} == {1024}

    detail, pooled = [], []
    for q in QS:
        total_low = total_overlap = expected = 0
        all_low_sys, all_rest_sys = [], []
        for bucket, rows in sorted(rows_by_bucket.items()):
            n, take = len(rows), math.ceil(len(rows) * q)
            low = set(sorted(range(n), key=lambda i: rows[i][0])[:take])
            high = set(sorted(range(n), key=lambda i: rows[i][1], reverse=True)[:take])
            overlap = len(low & high)
            total_low += take
            total_overlap += overlap
            expected += take * take / n
            low_sys = [rows[i][1] for i in low]
            rest_sys = [rows[i][1] for i in range(n) if i not in low]
            all_low_sys.extend(low_sys)
            all_rest_sys.extend(rest_sys)
            lo, hi = wilson(overlap, take)
            detail.append({
                "k": bucket[0], "m": bucket[1], "q": q, "n": n,
                "low_ridge_n": take, "high_sys_n": take, "same_q_overlap_n": overlap,
                "conditional_high_sys_rate": overlap / take,
                "conditional_rate_wilson95_low": lo, "conditional_rate_wilson95_high": hi,
                "independence_expected_overlap": take*take/n,
                "same_q_enrichment": overlap / (take*take/n),
                "low_ridge_sys_mean": sum(low_sys)/take,
                "other_sys_mean": sum(rest_sys)/len(rest_sys),
                "mean_difference_low_minus_other": sum(low_sys)/take - sum(rest_sys)/len(rest_sys),
                "low_ridge_sys_median": quantile(low_sys, .5),
                "low_ridge_sys_p90": quantile(low_sys, .9),
                "other_sys_median": quantile(rest_sys, .5),
                "other_sys_p90": quantile(rest_sys, .9),
            })
        lo, hi = wilson(total_overlap, total_low)
        b_lo, b_hi = bootstrap_stratified(rows_by_bucket, q, reps=200, seed=271828 + round(q*10000))
        pooled.append({
            "q": q, "strata": len(rows_by_bucket), "n": sum(map(len, rows_by_bucket.values())),
            "low_ridge_n": total_low, "high_sys_n_within_stratum": total_low,
            "same_q_overlap_n": total_overlap,
            "conditional_high_sys_rate": total_overlap/total_low,
            "conditional_rate_wilson95_low": lo, "conditional_rate_wilson95_high": hi,
            "stratified_bootstrap95_low": b_lo, "stratified_bootstrap95_high": b_hi,
            "stratified_independence_expected_overlap": expected,
            "same_q_enrichment": total_overlap/expected,
            "low_ridge_sys_mean": sum(all_low_sys)/len(all_low_sys),
            "other_sys_mean": sum(all_rest_sys)/len(all_rest_sys),
            "mean_difference_low_minus_other": sum(all_low_sys)/len(all_low_sys)-sum(all_rest_sys)/len(all_rest_sys),
            "low_ridge_sys_median": quantile(all_low_sys, .5),
            "low_ridge_sys_p90": quantile(all_low_sys, .9),
            "other_sys_median": quantile(all_rest_sys, .5),
            "other_sys_p90": quantile(all_rest_sys, .9),
        })
    return rows_by_bucket, detail, pooled


def selection_analysis():
    evaluated = {r["candidate_id"]: r for r in read_jsonl(EVALUATED)}
    # The compact pre-target snapshot is sufficient for the required identity
    # audit: every evaluated target row is one frozen selection/baseline row
    # and preserves its selection feature value.  The complete 1M feature scan
    # is optional and only checks this same equality against its original source.
    selected_before_target = {r["candidate_id"]: r for r in read_jsonl(ONE_M_SELECTED)}
    assert set(selected_before_target) == set(evaluated)
    for cid, er in evaluated.items():
        sr = selected_before_target[cid]
        assert (sr["source"]["k"], sr["source"]["m"], sr["selection_feature_value"]) == (er["product_k"], er["product_m"], er["selection_feature_value"])
    # Retain the historical output column/value: the compact snapshot checks
    # all evaluated rows even when the optional full feature scan is absent.
    feature_join_rows = len(evaluated)
    if FEATURES is not None:
        # The reviewed digest is an advisory staleness cue. Row identities and
        # semantic joins below determine whether the optional table is usable.
        if sha256(FEATURES) != FULL_1M_FEATURE_TABLE_SHA256:
            print(
                "warning: optional 1M feature table differs from the retained "
                "bytes; continuing with semantic checks. Reassess retained "
                "interpretations before treating this run as equivalent.",
                file=sys.stderr,
            )
        seen = {}
        for r in read_jsonl(FEATURES):
            if r["candidate_id"] in evaluated:
                seen[r["candidate_id"]] = r
        assert set(seen) == set(evaluated)
        for cid, er in evaluated.items():
            fr = seen[cid]
            assert (fr["product_k"], fr["product_m"], fr[RIDGE]) == (er["product_k"], er["product_m"], er["selection_feature_value"])
        feature_join_rows = len(seen)
    selected, baseline = defaultdict(list), defaultdict(list)
    for r in evaluated.values():
        b = (r["product_k"], r["product_m"])
        if "per_bucket_low_sum_top_10" in r["selection_ids"]:
            selected[b].append(r["sys"])
        if "per_bucket_low_sum_top_10__baseline_rep_0" in r["baseline_ids"]:
            baseline[b].append(r["sys"])
    assert len(selected) == len(baseline) == 10
    assert {len(x) for x in selected.values()} == {10} == {len(x) for x in baseline.values()}
    out, diffs = [], []
    for b in sorted(selected):
        s, z = selected[b], baseline[b]
        d = sum(s)/len(s) - sum(z)/len(z)
        diffs.append(d)
        out.append({"k":b[0], "m":b[1], "selected_n":len(s), "baseline_n":len(z),
                    "selected_mean_sys":sum(s)/len(s), "baseline_mean_sys":sum(z)/len(z),
                    "mean_difference":d, "selected_median_sys":quantile(s,.5),
                    "baseline_median_sys":quantile(z,.5), "selected_max_sys":max(s), "baseline_max_sys":max(z)})
    rng = random.Random(271828)
    boot = [sum(diffs[rng.randrange(10)] for _ in range(10))/10 for _ in range(10000)]
    summary = {"selection":"per_bucket_low_sum_top_10", "strata":10, "selected_n":100, "baseline_n":100,
               "mean_selected_sys":sum(sum(x) for x in selected.values())/100,
               "mean_baseline_sys":sum(sum(x) for x in baseline.values())/100,
               "bucket_equal_weight_mean_difference":sum(diffs)/10,
               "bucket_bootstrap95_low":quantile(boot,.025), "bucket_bootstrap95_high":quantile(boot,.975),
               "buckets_positive_mean_difference":sum(d > 0 for d in diffs),
               "selected_max_sys":max(max(x) for x in selected.values()),
               "baseline_max_sys":max(max(x) for x in baseline.values()),
               "evaluated_cache_rows":len(evaluated), "feature_join_rows":feature_join_rows}
    return out, [summary]


def independent_run_comparison():
    """Compare the frozen stage-1 100k run to the separate 1M ridge-tail run.

    The q values differ (0.01 versus 0.0001), so these are not repeated
    measurements at a common selection pressure.  This table is deliberately
    per bucket: a pooled mean can conceal the observed sign interaction.
    """
    one_m = defaultdict(list)
    for r in read_jsonl(EVALUATED):
        if "per_bucket_low_sum_top_10" in r["selection_ids"]:
            one_m[(r["product_k"], r["product_m"])].append(r["sys"])
    stage_one_id = "per_bucket_low_ridge_symp_area_sum_over_volume_sqrt_fraction_0p010000"
    hundred_k = defaultdict(list)
    for r in read_jsonl(CONCENTRATION):
        if stage_one_id in r["selection_ids"]:
            hundred_k[(r["product_k"], r["product_m"])].append(r["sys"])
    assert set(one_m) == set(hundred_k) and {len(v) for v in one_m.values()} == {10} and {len(v) for v in hundred_k.values()} == {100}
    rows, ds = [], []
    for b in sorted(one_m):
        a, z = one_m[b], hundred_k[b]
        d = sum(a)/len(a) - sum(z)/len(z)
        # Within-bucket nonparametric bootstrap for the contrast of independent
        # sample means.  It quantifies evaluation sampling variability only.
        rng = random.Random(8100 + 100*b[0] + b[1])
        boot = [sum(a[rng.randrange(10)] for _ in range(10))/10 - sum(z[rng.randrange(100)] for _ in range(100))/100 for _ in range(500)]
        ds.append(d)
        rows.append({"k":b[0], "m":b[1], "stage1_100k_q":.01, "stage1_n":100,
                     "stage1_mean_sys":sum(z)/100, "one_m_q":.0001, "one_m_n":10,
                     "one_m_mean_sys":sum(a)/10, "one_m_minus_stage1_mean":d,
                     "within_bucket_bootstrap95_low":quantile(boot,.025), "within_bucket_bootstrap95_high":quantile(boot,.975),
                     "direction":"higher_at_1m_more_extreme" if d > 0 else "lower_at_1m_more_extreme"})
    rng = random.Random(271828)
    pooled = [sum(ds[rng.randrange(10)] for _ in range(10))/10 for _ in range(1000)]
    summary = {"comparison":"independent pre-sys selected runs; 100k stage-1 q=.01 versus 1M q=.0001", "strata":10,
               "equal_bucket_mean_difference":sum(ds)/10, "bucket_bootstrap95_low":quantile(pooled,.025), "bucket_bootstrap95_high":quantile(pooled,.975),
               "buckets_higher_at_1m":sum(d>0 for d in ds), "buckets_lower_at_1m":sum(d<0 for d in ds),
               "interpretation":"heterogeneity diagnostic only: different q and independent generated runs preclude a same-pressure persistence curve."}
    return rows, [summary]


def main(out: Path):
    out.mkdir(parents=True, exist_ok=True)
    _, detail, pooled = retained_analysis()
    select_detail, select_summary = selection_analysis()
    comparison_detail, comparison_summary = independent_run_comparison()
    write_tsv(out/"retained_by_km_q.tsv", detail)
    write_tsv(out/"retained_stratified_pooled.tsv", pooled)
    write_tsv(out/"one_m_selection_by_km.tsv", select_detail)
    write_tsv(out/"one_m_selection_summary.tsv", select_summary)
    write_tsv(out/"independent_100k_vs_1m_by_km.tsv", comparison_detail)
    write_tsv(out/"independent_100k_vs_1m_summary.tsv", comparison_summary)
    input_sha256 = {
        "tracked_retained_table": sha256(RETAINED),
        "tracked_random_product_producer": sha256(PRODUCT),
        "one_m_snapshot/selection-plan.json": sha256(ONE_M_PLAN),
        "one_m_snapshot/selected-candidates-before-sys.jsonl": sha256(ONE_M_SELECTED),
        "one_m_snapshot/sys-evaluation-cache.jsonl": sha256(EVALUATED),
        "tracked_100k_validation/sys-evaluation-cache.jsonl": sha256(CONCENTRATION),
        "tracked_100k_validation/selection-plan.json": sha256(CONCENTRATION_PLAN),
        "optional_full_1m_feature_table_identity_audit_expected": FULL_1M_FEATURE_TABLE_SHA256,
    }
    metadata = {
        "command": "python3 analyze.py --out-dir .",
        "ridge_feature": RIDGE,
        "q_levels": QS,
        "input_sha256": input_sha256,
        "selection_plan_provenance": {
            "one_m": {
                "path": "generated-input-snapshot/one-m-ridge-sum/selection-plan.json", "sha256": sha256(ONE_M_PLAN),
                "semantics": "seed=271828; 100000 candidates/bucket; frozen pre-target per-bucket low ridge-sum top-10 (q=1e-4); one matched baseline replicate",
            },
            "concentration_100k_stage1": {
                "path": "../../extreme-scalar-rejection-proposer/artifacts/100k-ridge-concentration-validation/selection-plan.json", "sha256": sha256(CONCENTRATION_PLAN),
                "semantics": "seed=1618033; 10000 candidates/bucket; frozen pre-target stage-1 low ridge-sum fraction .01, ceil_min_one; one matched baseline replicate",
            },
        },
        "boundary": "Retained same-q diagnostics are post-sys observational associations. The 1M data contain 1,000,000 feature rows but only 466 evaluated selected/baseline rows; its selection comparison is pre-sys selection with matched baselines, not a population tail-overlap estimate. No sys>1 probability is estimated.",
    }
    (out/"metadata.json").write_text(json.dumps(metadata, indent=2)+"\n")


if __name__ == "__main__":
    p = argparse.ArgumentParser()
    p.add_argument("--out-dir", type=Path, default=Path(__file__).parent)
    p.add_argument("--repo-root", type=Path, help="checkout root; inferred from this packet when omitted")
    p.add_argument("--retained-table", type=Path)
    p.add_argument("--product-table", type=Path)
    p.add_argument("--concentration-cache", type=Path)
    p.add_argument("--concentration-plan", type=Path)
    p.add_argument("--one-m-snapshot-dir", type=Path, default=SCRIPT_DIR / "generated-input-snapshot/one-m-ridge-sum")
    p.add_argument("--one-m-feature-table", type=Path, help="optional full 1M feature-table identity audit")
    args = p.parse_args()
    REPO = (args.repo_root or repo_root(SCRIPT_DIR)).resolve()
    RETAINED = args.retained_table or REPO / "experiments/polytope-invariant-table/polytope-table.jsonl"
    PRODUCT = args.product_table or REPO / "experiments/polytope-datasets/random-product.jsonl"
    CONCENTRATION = args.concentration_cache or REPO / "experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/artifacts/100k-ridge-concentration-validation/sys-evaluation-cache.jsonl"
    CONCENTRATION_PLAN = args.concentration_plan or REPO / "experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/artifacts/100k-ridge-concentration-validation/selection-plan.json"
    ONE_M_PLAN = args.one_m_snapshot_dir / "selection-plan.json"
    ONE_M_SELECTED = args.one_m_snapshot_dir / "selected-candidates-before-sys.jsonl"
    EVALUATED = args.one_m_snapshot_dir / "sys-evaluation-cache.jsonl"
    FEATURES = args.one_m_feature_table
    main(args.out_dir)
