#!/usr/bin/env python3
"""Reconstruct the frozen ridge-tail source comparison.

This analyzer only reads retained rows and already evaluated product panels. It
does not generate geometry or evaluate capacity/sys. The completed target-free
generic stage-one manifest is checked against the formerly future 10k contract;
generic target rows remain owned by the separate target-evaluation packet.
"""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import math
import random
import statistics
from collections import Counter
from pathlib import Path
from typing import Any, Iterable


PACKET = Path(__file__).resolve().parent
ROOT = PACKET.parents[3]
ARTIFACTS = PACKET / "artifacts"
CURRENT = ARTIFACTS / "current"
RETAINED = ROOT / "experiments/polytope-invariant-table/polytope-table.jsonl"
PROVENANCE = ROOT / "experiments/polytope-invariant-table/polytope-provenance-table.jsonl"
PROPOSER = ROOT / "experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/artifacts"
CASCADE = PROPOSER / "100k-ridge-concentration-validation"
PROMISING = PROPOSER / "100k-promising-scalars"
DEFAULT_GENERIC_MANIFEST = ROOT / "experiments/sys-datascience/methods/generic-ridge-tail-stage1/artifacts/stage1/manifest.json"

MEAN_PROXY = "ridge_symp_area_mean_over_volume_sqrt"
SUM_PROXY = "ridge_symp_area_sum_over_volume_sqrt"
THRESHOLD = "high_sys_threshold"
GENERIC = "generic_f10"
PRODUCT_5X5 = "product_5x5"
PRODUCT_4X6 = "product_4x6"
PRODUCT_BUCKETS = {PRODUCT_5X5: (5, 5), PRODUCT_4X6: (4, 6)}
CASCADE_SELECTION = "per_bucket_low_ridge_symp_area_sum_over_volume_sqrt_fraction_0p010000"
PROMISING_SELECTION = "per_bucket_low_ridge_symp_area_mean_over_volume_sqrt_top_10"


def repo_rel(path: Path) -> str:
    """Stable repository-relative path for generated provenance (no worktree path)."""
    return path.resolve().relative_to(ROOT).as_posix()


def read_jsonl(path: Path) -> list[dict[str, Any]]:
    with path.open() as handle:
        return [json.loads(line) for line in handle if line.strip()]


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def canonical_hash(rows: Iterable[dict[str, Any]]) -> str:
    payload = "\n".join(
        json.dumps(row, sort_keys=True, separators=(",", ":"))
        for row in rows
    ).encode()
    return hashlib.sha256(payload).hexdigest()


def write_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")


def write_tsv(path: Path, rows: list[dict[str, Any]], fields: list[str]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fields, delimiter="\t", lineterminator="\n")
        writer.writeheader()
        writer.writerows({field: row.get(field, "") for field in fields} for row in rows)


def finite(value: Any) -> bool:
    return isinstance(value, (int, float)) and math.isfinite(float(value))


def nearest_rank(values: list[float], fraction: float) -> float:
    if not values:
        raise ValueError("nearest rank requires nonempty values")
    index = max(1, math.ceil(fraction * len(values))) - 1
    return sorted(values)[index]


def mean(values: list[float]) -> float:
    if not values:
        raise ValueError("mean requires nonempty values")
    return statistics.fmean(values)


def bootstrap_mean_ci(values: list[float], seed: int, draws: int = 20_000) -> tuple[float, float]:
    """Deterministic percentile bootstrap; descriptive for n=10, not asymptotic."""
    if len(values) < 2:
        return (values[0], values[0])
    rng = random.Random(seed)
    samples = sorted(mean([values[rng.randrange(len(values))] for _ in values]) for _ in range(draws))
    return (samples[draws // 40], samples[(39 * draws) // 40 - 1])


def bootstrap_difference_ci(left: list[float], right: list[float], seed: int, draws: int = 20_000) -> tuple[float, float]:
    if len(left) < 2 or len(right) < 2:
        return (mean(left) - mean(right), mean(left) - mean(right))
    rng = random.Random(seed)
    samples = sorted(
        mean([left[rng.randrange(len(left))] for _ in left])
        - mean([right[rng.randrange(len(right))] for _ in right])
        for _ in range(draws)
    )
    return (samples[draws // 40], samples[(39 * draws) // 40 - 1])


def wilson(count: int, n: int, z: float = 1.959963984540054) -> tuple[float, float]:
    if n == 0:
        raise ValueError("Wilson interval requires n > 0")
    p = count / n
    denominator = 1 + z * z / n
    centre = (p + z * z / (2 * n)) / denominator
    radius = z * math.sqrt(p * (1 - p) / n + z * z / (4 * n * n)) / denominator
    return (max(0.0, centre - radius), min(1.0, centre + radius))


def bucket_from_provenance(row: dict[str, Any], provenance: dict[str, dict[str, Any]]) -> str | None:
    if row["capacity_source"] != "random_product_sample":
        return None
    source = provenance[row["poly_id"]]["source"]
    return f"{source['k']}x{source['m']}"


def load_retained() -> tuple[dict[str, list[dict[str, Any]]], dict[str, Any]]:
    rows = read_jsonl(RETAINED)
    provenance_rows = read_jsonl(PROVENANCE)
    provenance = {row["poly_id"]: row for row in provenance_rows}
    if len(provenance) != len(provenance_rows):
        raise ValueError("duplicate provenance poly_id")
    populations: dict[str, list[dict[str, Any]]] = {GENERIC: [], PRODUCT_5X5: [], PRODUCT_4X6: []}
    for row in rows:
        if not finite(row.get("sys")) or not finite(row.get(MEAN_PROXY)) or not finite(row.get(SUM_PROXY)):
            raise ValueError(f"nonfinite retained target/proxy for {row.get('poly_id')}")
        if row["capacity_source"] == "random_sample" and row["facet_count"] == 10:
            populations[GENERIC].append(row)
        bucket = bucket_from_provenance(row, provenance)
        for population, pair in PRODUCT_BUCKETS.items():
            if bucket == f"{pair[0]}x{pair[1]}":
                populations[population].append(row)
    expected = {GENERIC: 512, PRODUCT_5X5: 1024, PRODUCT_4X6: 1024}
    for population, count in expected.items():
        if len(populations[population]) != count:
            raise ValueError(f"{population}: expected {count} retained rows, got {len(populations[population])}")
        if len({row["poly_id"] for row in populations[population]}) != count:
            raise ValueError(f"{population}: duplicate retained poly_id")
    for row in populations[GENERIC]:
        if row["facet_count"] != 10:
            raise ValueError("generic population is not F=10")
    for population, pair in PRODUCT_BUCKETS.items():
        counts = Counter(row["ridge_count"] for row in populations[population])
        if len(counts) != 1:
            raise ValueError(f"{population}: ridge count is not fixed: {counts}")
    files = {
        "retained_table": {"path": repo_rel(RETAINED), "sha256": sha256(RETAINED), "rows": len(rows)},
        "retained_provenance": {"path": repo_rel(PROVENANCE), "sha256": sha256(PROVENANCE), "rows": len(provenance_rows)},
    }
    inventory = {
        "source_files": files,
        "populations": {
            population: {
                "rows": len(values),
                "candidate_denominator": len(values),
                "target_visible_rows": len(values),
                "row_hash": canonical_hash(sorted(values, key=lambda row: row["poly_id"])),
                "selection_cutoff": "retained random table; within-population proxy rank only",
                "seed": "retained producer seed is not encoded as a per-row seed; provenance source is random/random_product",
                "censoring": "none within retained rows; this is a finite retained table, not a full future candidate population",
            }
            for population, values in populations.items()
        },
    }
    return populations, inventory


def load_generated_panel(directory: Path, selection_id: str, expected_seed: int, expected_n: int) -> tuple[list[dict[str, Any]], dict[str, Any]]:
    selected_path = directory / "selected-candidates-before-sys.jsonl"
    target_path = directory / "sys-evaluation-cache.jsonl"
    selected = read_jsonl(selected_path)
    targets = read_jsonl(target_path)
    target_by_id = {row["candidate_id"]: row for row in targets}
    if len(target_by_id) != len(targets):
        raise ValueError(f"{directory}: duplicate target candidate_id")
    panel = []
    for row in selected:
        if selection_id not in row.get("selection_ids", []):
            continue
        target = target_by_id.get(row["candidate_id"])
        if target is None:
            raise ValueError(f"{directory}: selected row missing target {row['candidate_id']}")
        if row["poly_id"] != target.get("poly_id") or row["bucket_id"] != target.get("bucket_id"):
            raise ValueError(f"{directory}: selected/target identity mismatch")
        if not finite(target.get("sys")) or not finite(row.get("selection_feature_value")):
            raise ValueError(f"{directory}: nonfinite selected target/proxy")
        if target.get("product_k") is None or target.get("product_m") is None:
            raise ValueError(f"{directory}: target lacks product dimensions")
        panel.append({
            "candidate_id": row["candidate_id"],
            "poly_id": row["poly_id"],
            "bucket_id": row["bucket_id"],
            "selection_feature": row.get("selection_feature"),
            "selection_feature_value": row["selection_feature_value"],
            "sys": target["sys"],
            "seed": target.get("source", {}).get("seed", row.get("source", {}).get("seed")),
            "sample_index": target.get("sample_index", row.get("source", {}).get("sample_index")),
        })
    if not panel:
        raise ValueError(f"{directory}: no rows for selector {selection_id}")
    seeds = {row["seed"] for row in panel}
    if seeds != {expected_seed}:
        raise ValueError(f"{directory}: expected seed {expected_seed}, got {seeds}")
    expected_buckets = {f"random-product:{k}x{m}:h0p8_1p2" for k, m in [(3, 3), (3, 4), (3, 5), (3, 6), (4, 4), (4, 5), (4, 6), (5, 5), (5, 6), (6, 6)]}
    counts = Counter(row["bucket_id"] for row in panel)
    if set(counts) != expected_buckets:
        raise ValueError(f"{directory}: expected all product buckets, got {sorted(counts)}")
    if set(counts.values()) != {expected_n}:
        raise ValueError(f"{directory}: expected {expected_n} rows/bucket, got {dict(counts)}")
    if len({row["candidate_id"] for row in panel}) != len(panel):
        raise ValueError(f"{directory}: duplicate panel candidate id")
    info = {
        "directory": repo_rel(directory),
        "selected_source_sha256": sha256(selected_path),
        "target_source_sha256": sha256(target_path),
        "selection_id": selection_id,
        "seed": expected_seed,
        "candidate_denominator_per_bucket": 10_000,
        "target_visible_rows_total": len(panel),
        "target_visible_rows_per_bucket": expected_n,
        "selection_cutoff": "lowest 1% by sum proxy; mean-rank equivalent because product ridge count is fixed" if expected_n == 100 else "lowest 0.1% by mean proxy (top 10 of 10,000)",
        "censoring": "only selected rows have targets; non-selected candidate targets are unavailable",
        "dependence": "independent candidate seed from the other generated panel; not independent of alternate selectors in the same cache",
        "row_hash": canonical_hash(sorted(panel, key=lambda row: row["candidate_id"])),
        "population_rows": {
            population: {
                "target_visible_rows": sum(
                    row["bucket_id"] == f"random-product:{k}x{m}:h0p8_1p2" for row in panel
                ),
                "row_hash": canonical_hash(
                    sorted(
                        [row for row in panel if row["bucket_id"] == f"random-product:{k}x{m}:h0p8_1p2"],
                        key=lambda row: row["candidate_id"],
                    )
                ),
            }
            for population, (k, m) in PRODUCT_BUCKETS.items()
        },
    }
    return panel, info


def retained_summaries(populations: dict[str, list[dict[str, Any]]], threshold: float) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    rows: list[dict[str, Any]] = []
    audit: list[dict[str, Any]] = []
    for population, values in populations.items():
        ordered = sorted(values, key=lambda row: (row[MEAN_PROXY], row["poly_id"]))
        baseline = [row["sys"] for row in values]
        baseline_mean = mean(baseline)
        fractions = [0.01, 0.05, 0.10, 0.20] if len(values) >= 1000 else [0.05, 0.10, 0.20]
        previous: list[float] | None = None
        for index, fraction in enumerate(fractions):
            n = math.ceil(fraction * len(ordered))
            selected = ordered[:n]
            sys_values = [row["sys"] for row in selected]
            count = sum(value >= threshold for value in sys_values)
            ci_low, ci_high = bootstrap_mean_ci(sys_values, 1100 + index + len(population))
            wlow, whigh = wilson(count, n)
            contrast = mean(sys_values) - baseline_mean
            rows.append({
                "population": population,
                "source": "retained_full_target",
                "proxy": MEAN_PROXY,
                "band": f"0-{fraction:g}",
                "fraction": fraction,
                "n": n,
                "effective_n": len({row["poly_id"] for row in selected}),
                "baseline_n": len(values),
                "mean_proxy": f"{mean([row[MEAN_PROXY] for row in selected]):.17g}",
                "mean_sys": f"{mean(sys_values):.17g}",
                "mean_sys_ci95": f"[{ci_low:.17g},{ci_high:.17g}]",
                "baseline_mean_sys": f"{baseline_mean:.17g}",
                "contrast_vs_baseline": f"{contrast:.17g}",
                "high_sys_threshold": f"{threshold:.17g}",
                "high_sys_exceedances": count,
                "high_sys_exceedance_rate": f"{count / n:.17g}",
                "high_sys_wilson95": f"[{wlow:.17g},{whigh:.17g}]",
                "censoring": "none within retained rows",
            })
            if previous is not None:
                previous_mean = mean(previous)
                delta = mean(sys_values) - previous_mean
                low, high = bootstrap_difference_ci(sys_values, previous, 2300 + index + len(population))
                rows[-1]["adjacent_hardening_contrast"] = f"{delta:.17g}"
                rows[-1]["adjacent_hardening_ci95"] = f"[{low:.17g},{high:.17g}]"
            else:
                rows[-1]["adjacent_hardening_contrast"] = ""
                rows[-1]["adjacent_hardening_ci95"] = ""
            previous = sys_values
        for fraction in [0.01, 0.05, 0.10, 0.20]:
            n = math.ceil(fraction * len(values))
            mean_top = {row["poly_id"] for row in sorted(values, key=lambda row: (row[MEAN_PROXY], row["poly_id"]))[:n]}
            sum_top = {row["poly_id"] for row in sorted(values, key=lambda row: (row[SUM_PROXY], row["poly_id"]))[:n]}
            audit.append({
                "population": population,
                "fraction": fraction,
                "n": n,
                "mean_sum_overlap": len(mean_top & sum_top),
                "mean_sum_jaccard": f"{len(mean_top & sum_top) / len(mean_top | sum_top):.17g}",
                "mean_sum_disagreement_rows": len(mean_top ^ sum_top) // 2,
                "interpretation": "rank equivalence audit only; not tail evidence",
            })
    return rows, audit


def panel_summary(panel: list[dict[str, Any]], population: str, panel_label: str, threshold: float, retained_baseline: float) -> dict[str, Any]:
    vals = [row["sys"] for row in panel]
    count = sum(value >= threshold for value in vals)
    low, high = bootstrap_mean_ci(vals, 501 + len(panel) + len(population))
    wlow, whigh = wilson(count, len(vals))
    return {
        "population": population,
        "panel": panel_label,
        "n": len(vals),
        "effective_n": len({row["candidate_id"] for row in panel}),
        "mean_sys": f"{mean(vals):.17g}",
        "mean_sys_ci95": f"[{low:.17g},{high:.17g}]",
        "baseline_reference": "retained full-target population",
        "baseline_mean_sys": f"{retained_baseline:.17g}",
        "contrast_vs_retained_baseline": f"{mean(vals) - retained_baseline:.17g}",
        "high_sys_threshold": f"{threshold:.17g}",
        "high_sys_exceedances": count,
        "high_sys_exceedance_rate": f"{count / len(vals):.17g}",
        "high_sys_wilson95": f"[{wlow:.17g},{whigh:.17g}]",
        "censoring": "target rows are pre-target proxy-selected; no target rows outside this panel are visible",
    }


def validate_future_manifest(path: Path | None) -> dict[str, Any]:
    if path is None or not path.exists():
        return {
            "status": "pending_manifest",
            "path": repo_rel(path or DEFAULT_GENERIC_MANIFEST),
            "target_visible_rows": 0,
            "message": "10k generic stage-one manifest is not present; future target evaluation is intentionally not consumed",
        }
    manifest = json.loads(path.read_text())
    required = {"schema", "status", "seed", "facet_count", "proxy", "counts", "target_exposure"}
    missing = sorted(required - manifest.keys())
    if missing:
        raise ValueError(f"generic manifest missing fields: {missing}")
    if manifest["facet_count"] != 10 or manifest["proxy"] != MEAN_PROXY:
        raise ValueError("generic manifest facet/proxy mismatch")
    counts = manifest["counts"]
    expected = {"accepted_candidates": 10_000, "selected": 100, "baseline": 100, "panel_union": 200, "future_band_zero_to_point_one_percent": 10, "future_band_point_one_to_one_percent": 90}
    for key, value in expected.items():
        if counts.get(key) != value:
            raise ValueError(f"generic manifest count mismatch {key}: {counts.get(key)} != {value}")
    exposure = manifest["target_exposure"]
    if exposure.get("capacity_computed_for_new_population") or exposure.get("sys_computed_for_new_population") or exposure.get("target_fields_present_in_stage_one_artifacts"):
        raise ValueError("generic stage-one manifest exposes target fields")
    return {
        "status": "validated_target_free_manifest",
        "path": repo_rel(path),
        "sha256": sha256(path),
        "target_visible_rows": 0,
        "counts": counts,
        "seed": manifest["seed"],
        "proxy": manifest["proxy"],
    }


def contract(threshold: float) -> dict[str, Any]:
    return {
        "schema": "sys-datascience.ridge-tail-source-comparison.future-contract.v1",
        "status": "frozen_contract_target_evaluation_pending",
        "population": "generic/non-product F=10",
        "candidate_count": 10_000,
        "proxy": MEAN_PROXY,
        "volume_definition": "f64 volume for future production; retained rational-derived values are historical only",
        "selected_rows": {"count": 100, "fraction": 0.01, "bands": {"0-.1%": 10, ".1-1%": 90}},
        "baseline": {"count": 100, "rule": "deterministic disjoint baseline from the same F=10 candidate population, frozen before target evaluation"},
        "singleton_0p01_percent": {"count": 1, "inferential_role": "none"},
        "primary_contrasts": [
            {"name": "low_1pct_vs_baseline_mean_sys", "definition": "mean(sys in selected lowest 1%) - mean(sys in deterministic baseline)", "decision_effect": 0.04},
            {"name": "hardening_0_to_0p1_vs_0p1_to_1pct_mean_sys", "definition": "mean(sys in 0-.1%) - mean(sys in .1-1%)", "decision_effect": 0.04},
            {"name": "generic_vs_product_5x5", "definition": "same nominal cutoff contrasts against the reused product 5x5 panels; operational, not causal"},
        ],
        "high_sys_exceedance": {"threshold": threshold, "definition": "retained generic F=10 nearest-rank empirical 90th percentile; sys >= threshold", "source": "retained generic rows only", "uncertainty": "Wilson 95% interval"},
        "mean_uncertainty": {"method": "deterministic percentile bootstrap 20,000 resamples; report as descriptive at n=10 and n=90, not asymptotic proof"},
        "comparison_guardrails": ["product 5x5 has fixed ridge count, so mean/sum ranks are equivalent", "product 4x6 is a sensitivity check only", "generated product panels are censored to selected target rows and are not symmetric with future generic design", "differences are operational and confounded by generator, combinatorics, and capacity backend"],
        "continue_100k": {"rule": "continue only if generic hardening or generic-minus-product interaction is material (point estimate >= 0.04 with positive uncertainty evidence), or a 95% interval still spans both plateau (0) and decision-changing +0.04 while retained product evidence is not flat/reversed; budget remaining alone is insufficient", "stop": "stop if contrasts are reversed/flat and uncertainty upper bounds are below +0.04, or if both generic and product comparisons are practically flat/reversed", "next_scale": "100,000 candidates; 1,000 at 1%, 100 at .1%, 10 at .01%; one million is outside this contract"},
        "target_boundary": "This packet never generates geometry or computes capacity/sys for generic candidates; future target evaluation is a separate stage consuming at most the frozen 200 rows.",
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--generic-manifest", type=Path, default=DEFAULT_GENERIC_MANIFEST)
    args = parser.parse_args()
    if args.generic_manifest is not None and not args.generic_manifest.exists():
        raise SystemExit(f"generic manifest does not exist: {args.generic_manifest}")
    populations, inventory = load_retained()
    threshold = nearest_rank([row["sys"] for row in populations[GENERIC]], 0.90)
    inventory["high_sys_threshold"] = {"value": threshold, "definition": "retained generic F=10 nearest-rank 90th percentile", "source_row_hash": inventory["populations"][GENERIC]["row_hash"]}
    cascade, cascade_info = load_generated_panel(CASCADE, CASCADE_SELECTION, 1_618_033, 100)
    promising, promising_info = load_generated_panel(PROMISING, PROMISING_SELECTION, 271_828, 10)
    inventory["generated_panels"] = {"cascade_1pct": cascade_info, "promising_0p1pct": promising_info}
    retained_rows, audit = retained_summaries(populations, threshold)
    panel_rows = []
    for population, pair in PRODUCT_BUCKETS.items():
        k, m = pair
        bucket = f"random-product:{k}x{m}:h0p8_1p2"
        retained_baseline = mean([row["sys"] for row in populations[population]])
        panel_rows.append(panel_summary([row for row in cascade if row["bucket_id"] == bucket], population, "generated_seed1618033_low_1pct", threshold, retained_baseline))
        panel_rows.append(panel_summary([row for row in promising if row["bucket_id"] == bucket], population, "generated_seed271828_low_0p1pct", threshold, retained_baseline))
    # Add the product sensitivity contrast at each generated cutoff.
    sensitivity = []
    for panel_label, source in [("generated_seed1618033_low_1pct", cascade), ("generated_seed271828_low_0p1pct", promising)]:
        for population, (k, m) in PRODUCT_BUCKETS.items():
            bucket = f"random-product:{k}x{m}:h0p8_1p2"
            vals = [row["sys"] for row in source if row["bucket_id"] == bucket]
            other_population = PRODUCT_4X6 if population == PRODUCT_5X5 else PRODUCT_5X5
            other_k, other_m = PRODUCT_BUCKETS[other_population]
            other_bucket = f"random-product:{other_k}x{other_m}:h0p8_1p2"
            other_vals = [row["sys"] for row in source if row["bucket_id"] == other_bucket]
            low, high = bootstrap_difference_ci(vals, other_vals, 7700 + len(vals) + k + m)
            sensitivity.append({"panel": panel_label, "comparison": f"{population}_minus_{other_population}", "n_left": len(vals), "n_right": len(other_vals), "mean_difference": f"{mean(vals) - mean(other_vals):.17g}", "difference_ci95": f"[{low:.17g},{high:.17g}]", "decision_effect": 0.04, "qualitative": "not material at 0.04 unless interval/evidence changes the operational decision"})
    future_manifest = validate_future_manifest(args.generic_manifest)
    contract_value = contract(threshold)
    write_json(CURRENT / "source-inventory.json", inventory)
    fields = ["population", "source", "proxy", "band", "fraction", "n", "effective_n", "baseline_n", "mean_proxy", "mean_sys", "mean_sys_ci95", "baseline_mean_sys", "contrast_vs_baseline", "adjacent_hardening_contrast", "adjacent_hardening_ci95", "high_sys_threshold", "high_sys_exceedances", "high_sys_exceedance_rate", "high_sys_wilson95", "censoring"]
    write_tsv(CURRENT / "retained-band-summary.tsv", retained_rows, fields)
    write_tsv(CURRENT / "generated-product-summary.tsv", panel_rows, ["population", "panel", "n", "effective_n", "mean_sys", "mean_sys_ci95", "baseline_reference", "baseline_mean_sys", "contrast_vs_retained_baseline", "high_sys_threshold", "high_sys_exceedances", "high_sys_exceedance_rate", "high_sys_wilson95", "censoring"])
    write_tsv(CURRENT / "mean-sum-proxy-audit.tsv", audit, ["population", "fraction", "n", "mean_sum_overlap", "mean_sum_jaccard", "mean_sum_disagreement_rows", "interpretation"])
    write_tsv(CURRENT / "product-sensitivity.tsv", sensitivity, ["panel", "comparison", "n_left", "n_right", "mean_difference", "difference_ci95", "decision_effect", "qualitative"])
    write_json(CURRENT / "future-analysis-contract.json", contract_value)
    summary = {
        "schema": "sys-datascience.ridge-tail-source-comparison.summary.v1",
        "source_inventory": inventory,
        "future_manifest": future_manifest,
        "retained_summary_rows": len(retained_rows),
        "generated_summary_rows": len(panel_rows),
        "product_sensitivity_rows": len(sensitivity),
        "product_sensitivity_observation": "5x5 versus 4x6 means differ by less than the provisional 0.04 contrast at both generated cutoffs; 4x6 does not materially change the qualitative operational conclusion",
        "target_producing_code_path": False,
        "ready_to_consume_future_200": future_manifest["status"] == "validated_target_free_manifest",
    }
    write_json(CURRENT / "summary.json", summary)


if __name__ == "__main__":
    main()
