#!/usr/bin/env python3
"""Analyze identifiable ridge/sys tail questions from existing artifacts only."""

from __future__ import annotations

import csv
import hashlib
import json
import math
import re
from collections import defaultdict
from pathlib import Path
from statistics import fmean


PACKET_DIR = Path(__file__).resolve().parent
ROOT = PACKET_DIR.parents[3]
OUT = PACKET_DIR / "artifacts" / "current"

RETAINED_TABLE = ROOT / "experiments/sys-datascience/prepare/polytope-table.jsonl"
RETAINED_PROVENANCE = (
    ROOT / "experiments/sys-datascience/prepare/polytope-provenance-table.jsonl"
)
PROPOSER = (
    ROOT
    / "experiments/sys-datascience/methods/extreme-scalar-rejection-proposer"
)
CASCADE = PROPOSER / "artifacts/100k-ridge-concentration-validation"
PROMISING = PROPOSER / "artifacts/100k-promising-scalars"
ONE_MILLION = PROPOSER / "artifacts/ridge-tail-1m-summary"

RIDGE = "ridge_symp_area_sum_over_volume_sqrt"
CASCADE_STAGE_1 = (
    "per_bucket_low_ridge_symp_area_sum_over_volume_sqrt_fraction_0p010000"
)
PROMISING_PER_BUCKET = (
    "per_bucket_low_ridge_symp_area_sum_over_volume_sqrt_top_10"
)


def jsonl(path: Path) -> list[dict]:
    with path.open() as handle:
        return [json.loads(line) for line in handle if line.strip()]


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def bucket_name(k: int, m: int) -> str:
    return f"{k}x{m}"


def short_bucket(bucket_id: str) -> str:
    match = re.search(r":(\d+x\d+):", bucket_id)
    if not match:
        raise ValueError(f"cannot parse bucket id: {bucket_id}")
    return match.group(1)


def average_ranks(values: list[float]) -> list[float]:
    order = sorted(range(len(values)), key=values.__getitem__)
    ranks = [0.0] * len(values)
    start = 0
    while start < len(order):
        stop = start + 1
        while stop < len(order) and values[order[stop]] == values[order[start]]:
            stop += 1
        rank = (start + 1 + stop) / 2
        for index in order[start:stop]:
            ranks[index] = rank
        start = stop
    return ranks


def pearson(xs: list[float], ys: list[float]) -> float:
    mx, my = fmean(xs), fmean(ys)
    dx = [x - mx for x in xs]
    dy = [y - my for y in ys]
    denominator = math.sqrt(sum(x * x for x in dx) * sum(y * y for y in dy))
    return sum(x * y for x, y in zip(dx, dy, strict=True)) / denominator


def summarize(values: list[float]) -> dict:
    return {
        "n": len(values),
        "mean_sys": fmean(values),
        "min_sys": min(values),
        "max_sys": max(values),
        "sys_gt_1": sum(value > 1 for value in values),
    }


def load_evaluated_selection(directory: Path) -> list[dict]:
    selected = jsonl(directory / "selected-candidates-before-sys.jsonl")
    target_rows = jsonl(directory / "sys-evaluation-cache.jsonl")
    if len(target_rows) != len({row["candidate_id"] for row in target_rows}):
        raise ValueError(f"duplicate target ids in {directory}")
    targets = {row["candidate_id"]: row for row in target_rows}
    joined = []
    for row in selected:
        target = targets.get(row["candidate_id"])
        if target is None:
            raise ValueError(f"missing target for {row['candidate_id']}")
        if row["poly_id"] != target["poly_id"]:
            raise ValueError(f"identity mismatch for {row['candidate_id']}")
        joined.append({**row, "sys": target["sys"]})
    return joined


def load_retained_products() -> dict[str, list[dict]]:
    provenance = {}
    for row in jsonl(RETAINED_PROVENANCE):
        if row["dataset"] == "random_product_sample":
            provenance[row["poly_id"]] = row["source"]
    buckets: dict[str, list[dict]] = defaultdict(list)
    for row in jsonl(RETAINED_TABLE):
        if row["capacity_source"] != "random_product_sample":
            continue
        source = provenance.get(row["poly_id"])
        if source is None:
            raise ValueError(f"missing product provenance for {row['poly_id']}")
        if not math.isfinite(row["sys"]) or not math.isfinite(row[RIDGE]):
            raise ValueError(f"nonfinite retained value for {row['poly_id']}")
        buckets[bucket_name(source["k"], source["m"])].append(row)
    if sorted(map(len, buckets.values())) != [1024] * 10:
        raise ValueError(f"unexpected retained bucket counts: {dict(map(lambda x: (x[0], len(x[1])), buckets.items()))}")
    return dict(sorted(buckets.items()))


def cumulative_row(
    *,
    source: str,
    seed: int,
    q: float,
    candidates_per_bucket: int,
    bucket_values: dict[str, list[float]],
    definition: str,
    censoring: str,
    dependence: str,
) -> dict:
    all_values = [value for values in bucket_values.values() for value in values]
    means = [fmean(values) for values in bucket_values.values()]
    per_bucket_counts = sorted({len(values) for values in bucket_values.values()})
    effective_quantiles = sorted(
        {len(values) / candidates_per_bucket for values in bucket_values.values()}
    )
    return {
        "source": source,
        "seed": seed,
        "proxy_quantile_upper": q,
        "effective_proxy_quantile": ",".join(map(str, effective_quantiles)),
        "selection_definition": definition,
        "candidates_per_bucket": candidates_per_bucket,
        "target_rows_total": len(all_values),
        "target_rows_per_bucket": ",".join(map(str, per_bucket_counts)),
        "pooled_mean_sys": fmean(all_values),
        "min_bucket_mean_sys": min(means),
        "max_bucket_mean_sys": max(means),
        "maximum_sys": max(all_values),
        "sys_gt_1": sum(value > 1 for value in all_values),
        "censoring": censoring,
        "dependence": dependence,
    }


def band_rows(
    source: str,
    seed: int,
    buckets: dict[str, list[dict]],
    population_per_bucket: int,
    edges: list[float],
) -> list[dict]:
    result = []
    pooled_by_band: dict[tuple[float, float], list[float]] = defaultdict(list)
    for bucket, rows in buckets.items():
        ordered = sorted(rows, key=lambda row: row[RIDGE])
        for q_low, q_high in zip(edges, edges[1:]):
            low = math.ceil(q_low * population_per_bucket)
            high = math.ceil(q_high * population_per_bucket)
            values = [row["sys"] for row in ordered[low:high]]
            if not values:
                raise ValueError(f"empty band {source} {bucket} {q_low} {q_high}")
            pooled_by_band[(q_low, q_high)].extend(values)
            result.append(
                {
                    "source": source,
                    "seed": seed,
                    "bucket": bucket,
                    "q_low": q_low,
                    "q_high": q_high,
                    **summarize(values),
                }
            )
    for (q_low, q_high), values in pooled_by_band.items():
        result.append(
            {
                "source": source,
                "seed": seed,
                "bucket": "all_equal_count_buckets",
                "q_low": q_low,
                "q_high": q_high,
                **summarize(values),
            }
        )
    return result


def parse_one_million() -> dict[str, list[float]]:
    by_bucket: dict[str, list[float]] = {}
    path = ONE_MILLION / "role-summary.tsv"
    with path.open(newline="") as handle:
        for row in csv.DictReader(handle, delimiter="\t"):
            if (
                row["selection_id"] == "per_bucket_low_sum_top_10"
                and row["role"] == "selected"
            ):
                # The tracked compact artifact owns only aggregate rows. Store ten
                # copies of the mean so cumulative pooled/equal-bucket means and n
                # remain exact; maxima are handled separately below.
                by_bucket[row["bucket"]] = [float(row["mean_sys"])] * int(row["n"])
    if sorted(map(len, by_bucket.values())) != [10] * 10:
        raise ValueError(f"unexpected 1M selected counts: {by_bucket.keys()}")
    return dict(sorted(by_bucket.items()))


def write_tsv(path: Path, rows: list[dict]) -> None:
    if not rows:
        raise ValueError(f"refusing to write empty table {path}")
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(
            handle,
            fieldnames=list(rows[0]),
            delimiter="\t",
            lineterminator="\n",
        )
        writer.writeheader()
        writer.writerows(rows)


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)

    retained = load_retained_products()
    cascade_rows = load_evaluated_selection(CASCADE)
    cascade_stage = [row for row in cascade_rows if CASCADE_STAGE_1 in row["selection_ids"]]
    cascade_buckets: dict[str, list[dict]] = defaultdict(list)
    for row in cascade_stage:
        cascade_buckets[short_bucket(row["bucket_id"])].append(
            {RIDGE: row["selection_feature_value"], "sys": row["sys"]}
        )
    if sorted(map(len, cascade_buckets.values())) != [100] * 10:
        raise ValueError("frozen 100k stage-one counts are not 100 per bucket")

    promising_rows = load_evaluated_selection(PROMISING)
    promising_selected = [
        row for row in promising_rows if PROMISING_PER_BUCKET in row["selection_ids"]
    ]
    promising_buckets: dict[str, list[float]] = defaultdict(list)
    for row in promising_selected:
        promising_buckets[short_bucket(row["bucket_id"])].append(row["sys"])
    if sorted(map(len, promising_buckets.values())) != [10] * 10:
        raise ValueError("promising-scalar q=0.1% counts are not 10 per bucket")

    one_million_buckets = parse_one_million()
    one_million_report = json.loads(
        (ONE_MILLION / "source-evaluation-report.json").read_text()
    )
    one_million_max = one_million_report["max_sys_selected"]

    overlap_rows = []
    for q in (0.01, 0.05, 0.10, 0.20):
        aggregate = {"n": 0, "k": 0, "intersection": 0}
        for bucket, rows in retained.items():
            n = len(rows)
            k = math.ceil(q * n)
            low_ridge = {row["poly_id"] for row in sorted(rows, key=lambda x: x[RIDGE])[:k]}
            high_sys = {row["poly_id"] for row in sorted(rows, key=lambda x: x["sys"], reverse=True)[:k]}
            intersection = len(low_ridge & high_sys)
            effective_q = k / n
            overlap_rows.append(
                {
                    "bucket": bucket,
                    "requested_q": q,
                    "n": n,
                    "tail_count": k,
                    "effective_q": effective_q,
                    "intersection": intersection,
                    "p_high_sys_given_low_ridge": intersection / k,
                    "enrichment_over_effective_q": intersection / k / effective_q,
                }
            )
            aggregate["n"] += n
            aggregate["k"] += k
            aggregate["intersection"] += intersection
        effective_q = aggregate["k"] / aggregate["n"]
        overlap_rows.append(
            {
                "bucket": "all_equal_count_buckets",
                "requested_q": q,
                "n": aggregate["n"],
                "tail_count": aggregate["k"],
                "effective_q": effective_q,
                "intersection": aggregate["intersection"],
                "p_high_sys_given_low_ridge": aggregate["intersection"] / aggregate["k"],
                "enrichment_over_effective_q": (
                    aggregate["intersection"] / aggregate["k"] / effective_q
                ),
            }
        )

    nested_rows = band_rows(
        "retained-full-target",
        42,
        retained,
        1024,
        [0.0, 0.01, 0.05, 0.20, 1.0],
    )
    nested_rows.extend(
        band_rows(
            "frozen-100k-stage1",
            1618033,
            dict(sorted(cascade_buckets.items())),
            10000,
            [0.0, 0.001, 0.002, 0.005, 0.01],
        )
    )

    cross_scale = []
    for q in (0.01, 0.05, 0.20, 1.0):
        values = {}
        for bucket, rows in retained.items():
            k = math.ceil(q * len(rows))
            values[bucket] = [row["sys"] for row in sorted(rows, key=lambda x: x[RIDGE])[:k]]
        cross_scale.append(
            cumulative_row(
                source="retained-full-target",
                seed=42,
                q=q,
                candidates_per_bucket=1024,
                bucket_values=values,
                definition="within-bucket lowest ridge cumulative tail",
                censoring="none within retained table; retained-table design only",
                dependence="independent retained seed",
            )
        )
    for q in (0.001, 0.002, 0.005, 0.01):
        k = math.ceil(q * 10000)
        values = {
            bucket: [row["sys"] for row in sorted(rows, key=lambda x: x[RIDGE])[:k]]
            for bucket, rows in cascade_buckets.items()
        }
        cross_scale.append(
            cumulative_row(
                source="frozen-100k-stage1",
                seed=1618033,
                q=q,
                candidates_per_bucket=10000,
                bucket_values=values,
                definition="post-hoc subdivision of pre-target-frozen lowest 1%",
                censoring="targets observed only in lowest 1% plus matched baselines",
                dependence="independent seed; cumulative rows nested within this source",
            )
        )
    cross_scale.append(
        cumulative_row(
            source="promising-scalars-100k",
            seed=271828,
            q=0.001,
            candidates_per_bucket=10000,
            bucket_values=dict(sorted(promising_buckets.items())),
            definition="pre-target-frozen per-bucket lowest 10",
            censoring="targets observed only for selected sets and matched baselines",
            dependence="same seed and prefix population as 1M source; not independent of it",
        )
    )
    one_million_row = cumulative_row(
        source="ridge-tail-1m",
        seed=271828,
        q=0.0001,
        candidates_per_bucket=100000,
        bucket_values=one_million_buckets,
        definition="pre-target-frozen per-bucket lowest 10",
        censoring="tracked packet retains aggregate target summaries only",
        dependence="same seed and nested prefix population as promising-scalars-100k",
    )
    one_million_row["maximum_sys"] = one_million_max
    cross_scale.append(one_million_row)
    cross_scale.sort(key=lambda row: (row["proxy_quantile_upper"], row["source"]))

    cross_seed_rows = []
    for bucket in sorted(promising_buckets):
        cascade_q_0p1 = [
            row["sys"]
            for row in sorted(cascade_buckets[bucket], key=lambda row: row[RIDGE])[:10]
        ]
        promising_q_0p1 = promising_buckets[bucket]
        one_million_q_0p01 = one_million_buckets[bucket]
        cross_seed_rows.append(
            {
                "bucket": bucket,
                "mean_sys_q_0p1pct_seed_1618033": fmean(cascade_q_0p1),
                "mean_sys_q_0p1pct_seed_271828": fmean(promising_q_0p1),
                "q_0p1pct_replication_delta_seed271828_minus_seed1618033": (
                    fmean(promising_q_0p1) - fmean(cascade_q_0p1)
                ),
                "mean_sys_q_0p01pct_seed_271828": fmean(one_million_q_0p01),
                "same_seed_hardening_delta_q0p01_minus_q0p1": (
                    fmean(one_million_q_0p01) - fmean(promising_q_0p1)
                ),
                "dependence_note": (
                    "The seed-271828 100k population is a prefix of the 1M population; "
                    "this is a dependent cross-scale comparison, not replication."
                ),
            }
        )

    bucket_trends = []
    pooled_ridge_ranks, pooled_sys_ranks = [], []
    for bucket, rows in sorted(cascade_buckets.items()):
        ordered = sorted(rows, key=lambda row: row[RIDGE])
        ridge_ranks = average_ranks([row[RIDGE] for row in ordered])
        sys_ranks = average_ranks([row["sys"] for row in ordered])
        rho = pearson(ridge_ranks, sys_ranks)
        first_half = fmean(row["sys"] for row in ordered[:50])
        second_half = fmean(row["sys"] for row in ordered[50:])
        most_extreme = fmean(row["sys"] for row in ordered[:10])
        remainder = fmean(row["sys"] for row in ordered[10:])
        bucket_trends.append(
            {
                "bucket": bucket,
                "spearman_ridge_sys_within_lowest_1pct": rho,
                "mean_sys_q_le_0p5pct": first_half,
                "mean_sys_q_0p5_to_1pct": second_half,
                "delta_more_extreme_half": first_half - second_half,
                "mean_sys_q_le_0p1pct": most_extreme,
                "mean_sys_q_0p1_to_1pct": remainder,
                "delta_most_extreme_tenth": most_extreme - remainder,
            }
        )
        pooled_ridge_ranks.extend(rank / len(rows) for rank in ridge_ranks)
        pooled_sys_ranks.extend(rank / len(rows) for rank in sys_ranks)

    zero_hit_bounds = []
    for q, n, source in (
        (0.01, 1000, "frozen-100k-stage1"),
        (0.001, 200, "two q=0.1% seed panels"),
        (0.0001, 100, "ridge-tail-1m"),
    ):
        zero_hit_bounds.append(
            {
                "proxy_quantile_upper": q,
                "zero_hit_target_rows": n,
                "one_sided_95pct_upper_conditional_hit_probability": 1 - 0.05 ** (1 / n),
                "source": source,
            }
        )

    sensitivity = []
    for q in (0.01, 0.0001, 0.000001):
        for conditional_hit_probability in (0.1, 0.01, 0.001, 0.0001, 0.00001, 0.000001):
            selected_for_median_hit = math.ceil(
                math.log(0.5) / math.log1p(-conditional_hit_probability)
            )
            sensitivity.append(
                {
                    "proxy_quantile_upper": q,
                    "assumed_conditional_p_sys_gt_1": conditional_hit_probability,
                    "selected_target_evaluations_for_50pct_hit": selected_for_median_hit,
                    "raw_candidates_for_50pct_hit": math.ceil(selected_for_median_hit / q),
                }
            )

    command_output = (CASCADE / "command-output.txt").read_text()
    wall_seconds = {
        stage: float(value)
        for stage, value in re.findall(
            r"stage=(geometry_cache|feature_table|pre_sys_selection|sys_evaluation).*?wall_seconds=([0-9.]+)",
            command_output,
        )
    }
    runtime_calibration = {
        "source": str((CASCADE / "command-output.txt").relative_to(ROOT)),
        "workers": 12,
        "raw_candidates": 100000,
        "target_evaluations": 2490,
        "stage_wall_seconds": wall_seconds,
        "pre_target_wall_seconds_per_raw_candidate": (
            wall_seconds["geometry_cache"]
            + wall_seconds["feature_table"]
            + wall_seconds["pre_sys_selection"]
        )
        / 100000,
        "target_wall_seconds_per_evaluation": wall_seconds["sys_evaluation"] / 2490,
        "boundary": (
            "Observed local 12-worker wall throughput only. Hardware identity, CPU utilization, "
            "storage scaling, and core-hours were not recorded, so this is not a core-hour model."
        ),
    }

    source_paths = [
        RETAINED_TABLE,
        RETAINED_PROVENANCE,
        CASCADE / "selection-plan.json",
        CASCADE / "selected-candidates-before-sys.jsonl",
        CASCADE / "sys-evaluation-cache.jsonl",
        PROMISING / "selection-plan.json",
        PROMISING / "selected-candidates-before-sys.jsonl",
        PROMISING / "sys-evaluation-cache.jsonl",
        ONE_MILLION / "role-summary.tsv",
        ONE_MILLION / "source-evaluation-report.json",
    ]
    summary = {
        "schema": "sys-datascience.tail-dependence-feasibility.v1",
        "question": (
            "Which ridge/sys tail questions are identifiable from existing target-evaluated designs, "
            "and what new nested-band design would discriminate the live asymptotic conjectures?"
        ),
        "proxy": RIDGE,
        "source_sha256": {str(path.relative_to(ROOT)): sha256(path) for path in source_paths},
        "observed": {
            "retained_product_rows": sum(map(len, retained.values())),
            "retained_rows_per_bucket": 1024,
            "frozen_100k_lowest_1pct_target_rows": len(cascade_stage),
            "frozen_100k_rows_per_bucket_within_lowest_1pct": 100,
            "promising_100k_q_0p1pct_target_rows": len(promising_selected),
            "one_million_q_0p01pct_target_rows": 100,
            "new_target_evaluations": 0,
            "trusted_new_sys_gt_1": 0,
        },
        "retained_tail_overlap": {
            str(row["requested_q"]): row
            for row in overlap_rows
            if row["bucket"] == "all_equal_count_buckets"
        },
        "frozen_100k_within_tail_trend": {
            "pooled_within_bucket_rank_spearman": pearson(
                pooled_ridge_ranks, pooled_sys_ranks
            ),
            "positive_more_extreme_half_deltas": sum(
                row["delta_more_extreme_half"] > 0 for row in bucket_trends
            ),
            "positive_most_extreme_tenth_deltas": sum(
                row["delta_most_extreme_tenth"] > 0 for row in bucket_trends
            ),
            "bucket_count": len(bucket_trends),
        },
        "cross_scale_bucket_pattern": {
            "same_seed_buckets_improving_from_q_0p1pct_to_q_0p01pct": sum(
                row["same_seed_hardening_delta_q0p01_minus_q0p1"] > 0
                for row in cross_seed_rows
            ),
            "buckets_with_nonzero_retained_1pct_same_q_overlap": sum(
                row["intersection"] > 0
                for row in overlap_rows
                if row["requested_q"] == 0.01
                and row["bucket"] != "all_equal_count_buckets"
            ),
            "bucket_count": 10,
        },
        "zero_hit_bounds": zero_hit_bounds,
        "runtime_calibration": runtime_calibration,
        "identifiability": {
            "identified_now": [
                "Within the retained 1024-row-per-bucket table, empirical same-q ridge/sys overlap down to about 1%.",
                "Within a frozen independent population, the conditional sys distribution across disjoint ridge bands from 0.1% through 1%.",
                "Aggregate conditional sys summaries for 10 rows per bucket at the 0.01% ridge tail.",
                "Bucket heterogeneity and coarse saturation/reversal diagnostics for these observed panels.",
            ],
            "not_identified_now": [
                "Same-q ridge/sys tail dependence below about 1%, because the generated populations are target-censored by ridge selection.",
                "A calibrated probability of sys > 1 in any ridge band, because all observed selected panels have zero hits.",
                "An asymptotic limit as q tends to zero from finitely many sparse, partly nested panels.",
                "Core-hours or optimizer-versus-filter compute equivalence, because current artifacts lack hardware/utilization and direct proxy-optimizer measurements.",
            ],
        },
        "interpretation_boundary": (
            "All generated panels use the random Lagrangian-product generator on h in [0.8,1.2]. "
            "Subdivisions of the frozen lowest 1% are exploratory target analyses of a pre-target selection, "
            "not independently frozen validation. Same-seed 100k and 1M panels are dependent nested populations."
        ),
    }

    write_tsv(OUT / "retained-tail-overlap.tsv", overlap_rows)
    write_tsv(OUT / "nested-band-summary.tsv", nested_rows)
    write_tsv(OUT / "cross-scale-summary.tsv", cross_scale)
    write_tsv(OUT / "cross-scale-by-bucket.tsv", cross_seed_rows)
    write_tsv(OUT / "frozen-100k-bucket-trends.tsv", bucket_trends)
    write_tsv(OUT / "hit-rate-sensitivity.tsv", sensitivity)
    (OUT / "summary.json").write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
    print(json.dumps(summary["observed"], indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
