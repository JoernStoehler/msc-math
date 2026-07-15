#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy"]
# ///
"""Analyze independent-seed factor panels without producing a generator ranking."""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import math
import subprocess
from collections import defaultdict
from pathlib import Path
from typing import Any

import numpy as np

import atlas
import shape_quality

SCHEMA = "generator-distribution-atlas-confirmation-v1"
BASELINE = "current-baseline[delta=0.2]"
ALPHA1 = "repulsive-gap[alpha=1]"
ALPHA4 = "repulsive-gap[alpha=4]"
ALPHA16 = "repulsive-gap[alpha=16]"
REGULAR = "repulsive-gap[regular]"
MUTATION = "regular-mutation[steps=4,scale=0.03]"
ZONOGON = "zonogon[lengths=uniform(0.5,1.5)]"
SATURATION_LEVELS = (4, 8, 12, 24)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write_tsv(path: Path, rows: list[dict[str, Any]]) -> None:
    fields = sorted({key for row in rows for key in row})
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fields, delimiter="\t", lineterminator="\n")
        writer.writeheader()
        for row in rows:
            writer.writerow({key: "NA" if row.get(key) is None else row[key] for key in fields})


def grouped(shapes: list[shape_quality.Shape]) -> dict[tuple[int, int, str], list[shape_quality.Shape]]:
    result: dict[tuple[int, int, str], list[shape_quality.Shape]] = defaultdict(list)
    for shape in shapes:
        seed = int(shape.row["seed"])
        result[(seed, shape.side_count, shape.law)].append(shape)
    for members in result.values():
        members.sort(key=atlas.bounded_selection_key)
    return dict(sorted(result.items()))


def pairwise(members: list[shape_quality.Shape]) -> tuple[float | None, dict[str, Any]]:
    distances, _ = atlas.distances(members)
    summary = shape_quality.within_metrics(distances, 1e-9)
    return summary["pairwise_mean"], summary


def cross(left: list[shape_quality.Shape], right: list[shape_quality.Shape]) -> tuple[np.ndarray, float | None, float | None]:
    distances, _ = atlas.distances(left + right)
    matrix = distances[: len(left), len(left) :]
    left_inner, _ = atlas.distances(left)
    right_inner, _ = atlas.distances(right)
    left_threshold = float(np.median(left_inner[np.triu_indices(len(left), 1)])) if len(left) > 1 else None
    right_threshold = float(np.median(right_inner[np.triu_indices(len(right), 1)])) if len(right) > 1 else None
    return matrix, left_threshold, right_threshold


def named_effects(shapes: list[shape_quality.Shape]) -> tuple[list[dict[str, Any]], list[dict[str, Any]], list[dict[str, Any]]]:
    by_group = grouped(shapes)
    effects: list[dict[str, Any]] = []
    within_rows: list[dict[str, Any]] = []
    saturation_rows: list[dict[str, Any]] = []
    seeds = sorted({key[0] for key in by_group})
    sides = sorted({key[1] for key in by_group})

    def get(seed: int, side: int, population: str) -> list[shape_quality.Shape]:
        return by_group.get((seed, side, population), [])

    for seed in seeds:
        for side in sides:
            populations = sorted(population for (s, n, population) in by_group if s == seed and n == side)
            for population in populations:
                members = get(seed, side, population)
                value, summary = pairwise(members)
                gram = summary["positive_gram_spectrum"]
                within_rows.append({
                    "master_seed": seed,
                    "side_count": side,
                    "population": population,
                    "n": len(members),
                    "pairwise_l2_mean": value,
                    "nearest_l2_mean": summary["nearest_neighbor_mean"],
                    "positive_gram_spectrum_participation_ratio": gram["positive_gram_spectrum_participation_ratio"],
                    "negative_eigenmass_fraction": gram["negative_eigenmass_fraction"],
                    "boundary": "positive Gram participation is not intrinsic/metric dimension; negative eigenmass diagnoses non-Euclidean embedding",
                    "distance_contract": "rotation-quotient L2 on the declared support grid via circular correlation; arbitrary rotations are approximate",
                })

            baseline, alpha1 = get(seed, side, BASELINE), get(seed, side, ALPHA1)
            if baseline and alpha1:
                matrix, left_t, right_t = cross(baseline, alpha1)
                left_nearest, right_nearest = np.min(matrix, axis=1), np.min(matrix, axis=0)
                left_overlap = float(np.mean(left_nearest <= left_t)) if left_t is not None else None
                right_overlap = float(np.mean(right_nearest <= right_t)) if right_t is not None else None
                effects.append({"master_seed": seed, "contrast": "baseline_vs_alpha1_nearest_cross_overlap", "side_count": side, "effect": "baseline_to_alpha1_overlap_fraction", "value": left_overlap, "substantial_threshold": 0.5, "pass": left_overlap is not None and left_overlap >= 0.5, "denominator": len(baseline), "definition": "directed nearest cross distance <= source within-population pairwise median"})
                effects.append({"master_seed": seed, "contrast": "baseline_vs_alpha1_nearest_cross_overlap", "side_count": side, "effect": "alpha1_to_baseline_overlap_fraction", "value": right_overlap, "substantial_threshold": 0.5, "pass": right_overlap is not None and right_overlap >= 0.5, "denominator": len(alpha1), "definition": "directed nearest cross distance <= source within-population pairwise median"})
                effects.append({"master_seed": seed, "contrast": "baseline_vs_alpha1_nearest_cross_overlap", "side_count": side, "effect": "bidirectional_substantial", "value": int(left_overlap >= 0.5 and right_overlap >= 0.5), "substantial_threshold": 1, "pass": left_overlap >= 0.5 and right_overlap >= 0.5, "denominator": min(len(baseline), len(alpha1)), "definition": "both directed overlap fractions >= 0.5"})

            regular, alpha4, alpha16 = get(seed, side, REGULAR), get(seed, side, ALPHA4), get(seed, side, ALPHA16)
            alpha1 = get(seed, side, ALPHA1)
            if regular and alpha4 and alpha16 and alpha1:
                d16 = float(np.mean(cross(regular, alpha16)[0]))
                d4 = float(np.mean(cross(regular, alpha4)[0]))
                d1 = float(np.mean(cross(regular, alpha1)[0]))
                order = d16 < d4 < d1
                effects.append({"master_seed": seed, "contrast": "regular_local_order", "side_count": side, "effect": "regular_to_alpha16_cross_l2_mean", "value": d16, "pass": order, "definition": "predeclared local order d(regular,alpha16) < d(regular,alpha4) < d(regular,alpha1)"})
                effects.append({"master_seed": seed, "contrast": "regular_local_order", "side_count": side, "effect": "regular_to_alpha4_cross_l2_mean", "value": d4, "pass": order, "definition": "predeclared local order d(regular,alpha16) < d(regular,alpha4) < d(regular,alpha1)"})
                effects.append({"master_seed": seed, "contrast": "regular_local_order", "side_count": side, "effect": "regular_to_alpha1_cross_l2_mean", "value": d1, "pass": order, "definition": "predeclared local order d(regular,alpha16) < d(regular,alpha4) < d(regular,alpha1)"})
                for population in (REGULAR, MUTATION, ALPHA16, ALPHA4, ALPHA1):
                    members = get(seed, side, population)
                    value, _ = pairwise(members)
                    effects.append({"master_seed": seed, "contrast": "narrow_negative_controls", "side_count": side, "effect": f"within_pairwise_l2_mean:{population}", "value": value, "pass": None, "definition": "descriptive narrow/local control; no global score"})
                values = {population: pairwise(get(seed, side, population))[0] for population in (MUTATION, ALPHA16, ALPHA4, ALPHA1)}
                effects.extend([
                    {"master_seed": seed, "contrast": "narrow_negative_controls", "side_count": side, "effect": "mutation_within_lt_alpha1", "value": int(values[MUTATION] < values[ALPHA1]), "pass": values[MUTATION] < values[ALPHA1], "definition": "four-step mutation remains narrower than equal-support alpha=1 in this stratum"},
                    {"master_seed": seed, "contrast": "narrow_negative_controls", "side_count": side, "effect": "alpha16_within_lt_alpha1", "value": int(values[ALPHA16] < values[ALPHA1]), "pass": values[ALPHA16] < values[ALPHA1], "definition": "alpha=16 remains narrower than equal-support alpha=1 in this stratum"},
                    {"master_seed": seed, "contrast": "narrow_negative_controls", "side_count": side, "effect": "alpha4_between_alpha16_alpha1", "value": int(values[ALPHA16] <= values[ALPHA4] <= values[ALPHA1]), "pass": values[ALPHA16] <= values[ALPHA4] <= values[ALPHA1], "definition": "within-law diversity order alpha16 <= alpha4 <= alpha1"},
                ])

            if side in (4, 6):
                zonogon, baseline = get(seed, side, ZONOGON), get(seed, side, BASELINE)
                zv, _ = pairwise(zonogon); bv, _ = pairwise(baseline)
                ratio = None if zv is None or bv in (None, 0.0) else zv / bv
                effects.append({"master_seed": seed, "contrast": "zonogon_vs_baseline_diversity", "side_count": side, "effect": "within_l2_ratio_zonogon_over_baseline", "value": ratio, "strong_threshold": 2.0, "pass": ratio is not None and ratio >= 2.0, "definition": "strongly exceeds means ratio >= 2; all values are finite-panel raw-grid summaries"})

            for population in (BASELINE, ALPHA1):
                members = get(seed, side, population)
                if not members:
                    continue
                anisotropy = np.array([atlas.invariant_features(shape)[1] for shape in members])
                q95 = float(np.quantile(anisotropy, 0.95))
                effects.append({"master_seed": seed, "contrast": "thin_shape_anisotropy_tail", "side_count": side, "effect": f"q95_covariance_anisotropy:{population}", "value": q95, "tail_threshold": 10.0, "pass": q95 >= 10.0, "definition": "q95 of rotation/translation/positive-scale invariant covariance eigenvalue ratio; raw anisotropy tail"})

        for population in (BASELINE, ALPHA1):
            q95_by_side = {}
            for side in sides:
                members = get(seed, side, population)
                if members:
                    q95_by_side[side] = float(np.quantile([atlas.invariant_features(shape)[1] for shape in members], 0.95))
            if 3 in q95_by_side:
                other = [q95_by_side[side] for side in (4, 6) if side in q95_by_side]
                if other:
                    effects.append({"master_seed": seed, "contrast": "thin_shape_anisotropy_tail", "side_count": 3, "effect": f"triangle_q95_exceeds_other_sides:{population}", "value": int(q95_by_side[3] > max(other)), "pass": q95_by_side[3] > max(other), "definition": "triangle q95 exceeds both available non-triangle q95 values; side-stratified tail emphasis"})

            for population in (REGULAR, MUTATION, ALPHA16, ALPHA4, ALPHA1, BASELINE):
                members = get(seed, side, population)
                if len(members) < 2:
                    continue
                distances, _ = atlas.distances(members)
                previous = None
                first_stable = None
                for requested in SATURATION_LEVELS:
                    n = min(requested, len(members))
                    if n < 2:
                        continue
                    value = float(np.mean(distances[:n, :n][np.triu_indices(n, 1)]))
                    change = None if previous is None or previous == 0.0 else abs(value - previous) / abs(previous)
                    if first_stable is None and (value == 0.0 or (change is not None and change <= 0.15)):
                        first_stable = requested
                    saturation_rows.append({"master_seed": seed, "side_count": side, "population": population, "requested_n": requested, "used_n": n, "pairwise_l2_mean": value, "relative_change": change, "first_stable_n": first_stable, "stability_threshold": 0.15, "definition": "first deterministic prefix n with relative change <= 0.15; exact-zero controls stable at n=4"})
                    previous = value

    return effects, within_rows, saturation_rows


def aggregate_effects(effects: list[dict[str, Any]]) -> list[dict[str, Any]]:
    grouped_effects: dict[tuple[str, int, str], list[dict[str, Any]]] = defaultdict(list)
    for row in effects:
        grouped_effects[(row["contrast"], row["side_count"], row["effect"])].append(row)
    out = []
    for (contrast, side, effect), rows in sorted(grouped_effects.items()):
        values = [float(row["value"]) for row in rows if isinstance(row.get("value"), (int, float)) and math.isfinite(float(row["value"]))]
        passes = [row["pass"] for row in rows if isinstance(row.get("pass"), bool)]
        out.append({"contrast": contrast, "side_count": side, "effect": effect, "seeds": len(rows), "mean": None if not values else float(np.mean(values)), "median": None if not values else float(np.median(values)), "min": None if not values else float(np.min(values)), "max": None if not values else float(np.max(values)), "between_seed_std": None if len(values) < 2 else float(np.std(values, ddof=1)), "seeds_passing": sum(passes), "pass_rate": None if not passes else sum(passes) / len(passes), "definition": rows[0].get("definition")})
    return out


def rank_stability(effects: list[dict[str, Any]]) -> list[dict[str, Any]]:
    grouped_effects: dict[tuple[int, str], dict[int, dict[str, float]]] = defaultdict(lambda: defaultdict(dict))
    for row in effects:
        if row["contrast"] == "regular_local_order" and row["effect"].startswith("regular_to_"):
            grouped_effects[(row["side_count"], row["contrast"])][row["master_seed"]][row["effect"]] = float(row["value"])
    out = []
    for (side, contrast), per_seed in sorted(grouped_effects.items()):
        statuses = []
        for seed, values in sorted(per_seed.items()):
            order = sorted(values, key=values.get)
            status = "alpha16<alpha4<alpha1" if order == ["regular_to_alpha16_cross_l2_mean", "regular_to_alpha4_cross_l2_mean", "regular_to_alpha1_cross_l2_mean"] else "reversal_or_tie"
            statuses.append(status)
            out.append({"side_count": side, "master_seed": seed, "named_order": status, "order_values": json.dumps(values, sort_keys=True), "order_reversal": status != "alpha16<alpha4<alpha1", "contrast": contrast})
        out.append({"side_count": side, "master_seed": "joint", "named_order": "stable" if all(s == "alpha16<alpha4<alpha1" for s in statuses) else "seed-dependent/reversed", "order_values": "NA", "order_reversal": any(s != "alpha16<alpha4<alpha1" for s in statuses), "contrast": contrast})
    return out


def cost_rows(paths: list[Path]) -> list[dict[str, Any]]:
    rows = []
    for path in paths:
        report = json.loads(path.read_text())
        seed = report["seed"]
        for item in report["per_population"]:
            rows.append({"master_seed": seed, "report": str(path), "population": f"{item['law']}[{item['parameter']}]", "side_count": item["side_count"], "requested": item["requested"], "accepted": item["accepted"], "exhausted": item["exhausted"], "acceptance_fraction": item["accepted"] / item["requested"] if item["requested"] else None, "total_generation_ms": item["total_generation_ms"], "generation_ms_per_accepted": item["total_generation_ms"] / item["accepted"] if item["accepted"] else None, "max_attempts_observed": item["max_attempts_observed"], "source_revision": report["source_revision"], "source_dirty": report["source_dirty"]})
    return rows


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", type=Path, action="append", required=True)
    parser.add_argument("--producer-report", type=Path, action="append", required=True)
    parser.add_argument("--producer-executable", type=Path, required=True)
    parser.add_argument("--producer-revision", default="fd9c3e7df08d8c9d04491b8ebbb7b2628d2df32e")
    parser.add_argument("--out-dir", type=Path, required=True)
    args = parser.parse_args()
    args.out_dir.mkdir(parents=True, exist_ok=True)
    shapes: list[shape_quality.Shape] = []
    seen: set[str] = set()
    input_hashes = []
    for path in args.input:
        input_hashes.append({"path": str(path), "sha256": sha256(path)})
        for shape in shape_quality.load_shapes(path, 64, 1024):
            if shape.sample_id in seen:
                raise SystemExit(f"duplicate sample_id across input shards: {shape.sample_id}")
            seen.add(shape.sample_id)
            shapes.append(shape)
    effects, within_rows, saturation_rows = named_effects(shapes)
    costs = cost_rows(args.producer_report)
    for name, rows in (("per-seed-effects.tsv", effects), ("joint-effects.tsv", aggregate_effects(effects)), ("rank-stability.tsv", rank_stability(effects)), ("within-population.tsv", within_rows), ("saturation.tsv", saturation_rows), ("acceptance-cost.tsv", costs)):
        write_tsv(args.out_dir / name, rows)
    producer_identity = atlas.producer_provenance(args.producer_executable, args.producer_revision)
    report = {
        "schema": SCHEMA,
        "input_schema": shape_quality.SCHEMA,
        "master_seeds": sorted({int(shape.row["seed"]) for shape in shapes}),
        "rows_validated": len(shapes),
        "input_shards": input_hashes,
        "configuration": {
            "support_grid": 64,
            "steiner_grid": 1024,
            "independence_unit": "master seed; factor-only producer derives law/side/row/attempt sub-seeds by BLAKE3",
            "declared_producer_source_revision": "fd9c3e7df08d8c9d04491b8ebbb7b2628d2df32e",
            "producer_build_checkout_revisions_in_reports": sorted({json.loads(path.read_text())["source_revision"] for path in args.producer_report}),
            "nominal_allocation": "24 requested accepted rows per population/side-count stratum; primal-hull triangles may exhaust under 128 attempts; zonogon only at sides 4,6",
            "named_thresholds": {"substantial_bidirectional_overlap": 0.5, "strong_zonogon_diversity_ratio": 2.0, "anisotropy_q95_tail": 10.0, "early_saturation_relative_change": 0.15},
            "distance_contract": "accepted grid-distance naming: rotation-quotient L2 by declared-grid circular correlation; arbitrary rotations are approximate, grid-aligned rotations exact",
            "positive_gram_boundary": "positive Gram participation ratio is not intrinsic or metric dimension; negative eigenmass only diagnoses failure of Euclidean embedding",
            "raw_feature_boundary": "raw unstandardized feature aggregates are scale-sensitive diagnostics; covariance_anisotropy can dominate and no aggregate is a score",
        },
        "producer_reports": [{"path": str(path), "sha256": sha256(path)} for path in args.producer_report],
        "producer_provenance": producer_identity,
        "repository": {
            "revision": subprocess.check_output(["git", "rev-parse", "HEAD"], text=True).strip(),
            "tree": subprocess.check_output(["git", "rev-parse", "HEAD^{tree}"], text=True).strip(),
            "tracked_clean_predicate": "git status --porcelain --untracked-files=no",
            "tracked_clean": not bool(subprocess.check_output(["git", "status", "--porcelain", "--untracked-files=no"], text=True)),
        },
        "implementation_hashes": {"analyze_py_sha256": sha256(Path(__file__)), "atlas_py_sha256": sha256(Path(atlas.__file__).resolve()), "shape_quality_py_sha256": sha256(Path(shape_quality.__file__).resolve())},
        "observations": {
            "baseline_alpha1_overlap": "named bidirectional overlap event is pass only when both directions are >=0.5 in every side stratum; no global ranking",
            "regular_controls": "regular/mutation are local negative controls; alpha16/alpha4/alpha1 order is reported per side and seed, including reversals",
            "zonogon_diversity": "strong excess is predeclared as ratio >=2 at side 4 or 6; side 6 need not pass",
            "anisotropy": "q95 >=10 is a finite-panel thin-tail flag; triangle emphasis remains a side-stratified contrast",
            "saturation": "first deterministic prefix n with <=0.15 relative change is a descriptive saturation estimate only",
        },
        "interpretation": {
            "allowed": ["per-seed and joint named contrasts, finite-panel effect sizes, rank/order stability, between-seed variability, acceptance/exhaustion/cost, and sign/order reversals"],
            "prohibited": ["best-generator selection or global ranking", "population support or natural-law probability", "mechanism, target/sys prediction, or transfer", "reusing this confirmation set to invent post-hoc criteria without labeling new exploration"],
            "deferred": ["4D exact reconstruction and structural product classification", "target evaluation", "inferential uncertainty beyond between-seed descriptive variability"],
        },
    }
    (args.out_dir / "report.json").write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"rows": len(shapes), "seeds": report["master_seeds"], "out_dir": str(args.out_dir)}))


if __name__ == "__main__":
    main()
