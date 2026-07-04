#!/usr/bin/env python3
from __future__ import annotations

import argparse
import csv
import json
import math
from collections import defaultdict
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[4]
DEFAULT_OUT = Path(__file__).resolve().parent / "artifacts/current"

DEFAULT_PROMISING = (
    REPO_ROOT
    / "experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/artifacts/100k-promising-scalars"
)
DEFAULT_RIDGE_1M = (
    REPO_ROOT
    / "experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/artifacts/ridge-tail-1m-summary"
)
DEFAULT_TAIL = Path("/tmp/sys-ds-tail-invariant-analysis")
DEFAULT_FEATURE_CACHE = Path("/tmp/sys-ds-100k-promising-scalars-large-caches/candidate-feature-table.jsonl")

PROMISING = DEFAULT_PROMISING
RIDGE_1M = DEFAULT_RIDGE_1M
TAIL = DEFAULT_TAIL

RIDGE_MAGNITUDE_FEATURES = [
    "ridge_symp_area_sum_over_volume_sqrt",
    "ridge_symp_area_mean_over_volume_sqrt",
    "ridge_symp_area_max_over_volume_sqrt",
    "ridge_symp_area_std_over_volume_sqrt",
    "ridge_symp_area_q95_over_volume_sqrt",
    "ridge_symp_area_q90_over_volume_sqrt",
]

COUNT_FEATURES = ["ridge_count", "edge_count", "vertex_count", "facet_count"]

CONCENTRATION_FEATURES = [
    ("ridge_symp_area_entropy", "high"),
    ("ridge_symp_area_effective_face_count", "high"),
    ("ridge_symp_area_normalized_entropy", "high"),
    ("ridge_symp_area_max_share", "low"),
    ("ridge_symp_area_top3_share", "low"),
]

SMALL_AREA_FEATURES = [
    ("ridge_symp_area_le_1em3_over_volume_sqrt_fraction", "high"),
    ("ridge_symp_area_le_1em2_over_volume_sqrt_fraction", "high"),
    ("ridge_symp_area_le_1em1_over_volume_sqrt_fraction", "high"),
]


def read_tsv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="") as f:
        return list(csv.DictReader(f, delimiter="\t"))


def write_tsv(path: Path, rows: list[dict[str, object]], fieldnames: list[str]) -> None:
    with path.open("w", newline="") as f:
        writer = csv.DictWriter(
            f,
            delimiter="\t",
            fieldnames=fieldnames,
            extrasaction="ignore",
            lineterminator="\n",
        )
        writer.writeheader()
        for row in rows:
            writer.writerow(row)


def read_jsonl(path: Path) -> list[dict[str, object]]:
    rows = []
    with path.open() as f:
        for line in f:
            if line.strip():
                rows.append(json.loads(line))
    return rows


def f(row: dict[str, object], key: str, default: float = float("nan")) -> float:
    value = row.get(key, default)
    if value in ("", None):
        return default
    return float(value)


def median(values: list[float]) -> float:
    values = sorted(v for v in values if math.isfinite(v))
    if not values:
        return float("nan")
    n = len(values)
    mid = n // 2
    if n % 2:
        return values[mid]
    return 0.5 * (values[mid - 1] + values[mid])


def mean(values: list[float]) -> float:
    values = [v for v in values if math.isfinite(v)]
    if not values:
        return float("nan")
    return sum(values) / len(values)


def classify_enrichment(enrichment: float, recall: float | None = None) -> str:
    if not math.isfinite(enrichment):
        return "ambiguous"
    if enrichment >= 2.0 and (recall is None or recall >= 0.25):
        return "survives"
    if enrichment >= 1.2:
        return "weakens"
    if enrichment <= 1.05:
        return "collapses"
    return "ambiguous"


def classify_delta(delta: float, support: int, total: int) -> str:
    if not math.isfinite(delta) or total == 0:
        return "ambiguous"
    support_fraction = support / total
    if delta >= 0.04 and support_fraction >= 0.6:
        return "survives"
    if delta >= 0.015 and support_fraction >= 0.5:
        return "weakens"
    if abs(delta) < 0.01 or support_fraction <= 0.35:
        return "collapses"
    return "ambiguous"


def load_feature_cache_for(path: Path, candidate_ids: set[str]) -> dict[str, dict[str, object]]:
    out = {}
    with path.open() as f:
        for line in f:
            if not line.strip():
                continue
            row = json.loads(line)
            candidate_id = row["candidate_id"]
            if candidate_id in candidate_ids:
                out[candidate_id] = row
    return out


def source_to_bucket(source: str) -> str:
    # random-product:3x4:h0p8_1p2 -> 3x4
    parts = source.split(":")
    for part in parts:
        if "x" in part and part[0].isdigit():
            return part
    return source


def summarize_selection_summary() -> tuple[list[dict[str, object]], dict[str, dict[str, str]]]:
    summary = read_tsv(PROMISING / "selection-summary.tsv")
    by_id = {row["selection_id"]: row for row in summary}
    rows = []
    for row in summary:
        sid = row["selection_id"]
        if sid == "union_all_selection_rules":
            continue
        feature = row["selection_kind"].removeprefix("global_low_").removeprefix("per_bucket_low_")
        feature = feature.removeprefix("global_high_").removeprefix("per_bucket_high_")
        feature = feature.removesuffix("_percentile")
        is_low_ridge = any(x in sid for x in RIDGE_MAGNITUDE_FEATURES)
        is_per_bucket = sid.startswith("per_bucket_")
        selected_above_p95 = int(f(row, "selected_rows_above_baseline_p95", 0))
        mean_delta = f(row, "improvement_vs_baseline_mean_sys")
        if is_low_ridge and is_per_bucket and mean_delta >= 0.15 and selected_above_p95 >= 20:
            classification = "survives"
        elif is_low_ridge and mean_delta > 0.05:
            classification = "weakens"
        elif (not is_low_ridge) and mean_delta <= 0.03:
            classification = "collapses"
        else:
            classification = "ambiguous"
        rows.append(
            {
                "evidence_source": "generated_100k_promising_scalars",
                "slice": sid,
                "hypothesis": (
                    "low_ridge_magnitude_signal"
                    if is_low_ridge
                    else "product_combinatorial_proxy"
                ),
                "control_or_bucket": "matched_baseline_per_selection",
                "classification": classification,
                "n_selected": int(f(row, "selected_rows", 0)),
                "selected_mean_sys": f(row, "selected_mean_sys"),
                "selected_max_sys": f(row, "selected_max_sys"),
                "baseline_mean_sys": f(row, "baseline_mean_sys"),
                "baseline_p95_sys": f(row, "baseline_p95_sys"),
                "selected_above_baseline_p95": selected_above_p95,
                "effect_size": mean_delta,
                "support_detail": row["selected_bucket_counts"],
                "interpretation_boundary": "pre_sys_rule_selection; selected-vs-matched-baseline generated evidence; no sys>1 row",
            }
        )
    return rows, by_id


def summarize_1m() -> list[dict[str, object]]:
    rows = read_tsv(RIDGE_1M / "role-summary.tsv")
    paired = {}
    for row in rows:
        key = (row["selection_id"], row["bucket"])
        paired.setdefault(key, {})[row["role"]] = row
    out = []
    for (selection_id, bucket), roles in sorted(paired.items()):
        if "selected" not in roles or "baseline" not in roles:
            continue
        selected = roles["selected"]
        baseline = roles["baseline"]
        delta = f(selected, "mean_sys") - f(baseline, "mean_sys")
        ratio = f(selected, "mean_sys") / max(f(baseline, "mean_sys"), 1e-12)
        out.append(
            {
                "evidence_source": "generated_1m_ridge_sum_tail",
                "slice": selection_id,
                "hypothesis": "low_ridge_magnitude_signal",
                "control_or_bucket": bucket,
                "classification": classify_enrichment(ratio),
                "n_selected": int(f(selected, "n", 0)),
                "selected_mean_sys": f(selected, "mean_sys"),
                "selected_max_sys": f(selected, "max_sys"),
                "baseline_mean_sys": f(baseline, "mean_sys"),
                "baseline_p95_sys": "",
                "selected_above_baseline_p95": "",
                "effect_size": delta,
                "support_detail": f"mean_ratio={ratio:.6g}",
                "interpretation_boundary": "pre_sys low-sum selection in generated product run; exploration artifact; no sys>1 row",
            }
        )
    return out


def summarize_retained_attribution() -> list[dict[str, object]]:
    rows = read_tsv(TAIL / "feature-attribution-redundancy.tsv")
    wanted_controls = {
        "none": "raw_no_controls",
        "source_facet_provenance_controls": "after_source_facet_product_controls",
        "strongest_combinatorial_controls": "after_strongest_combinatorial_controls",
        "source_facet_provenance_plus_strongest_combinatorial": "after_source_facet_product_plus_combinatorial_controls",
        "strongest_other_family_prepared_controls": "after_other_feature_family_controls",
        "strongest_nonself_prepared_controls": "after_nonself_prepared_controls",
    }
    best: dict[tuple[str, str], dict[str, str]] = {}
    for row in rows:
        if row["tail_label"] != "top_10_percent":
            continue
        if row["capacity_source"] != "random_product_sample":
            continue
        if row["feature"] not in RIDGE_MAGNITUDE_FEATURES:
            continue
        if row["control_set"] not in wanted_controls:
            continue
        key = (row["feature"], row["control_set"])
        old = best.get(key)
        if old is None or f(row, "enrichment") > f(old, "enrichment"):
            best[key] = row

    out = []
    for (feature, control), row in sorted(best.items()):
        enrichment = f(row, "enrichment")
        recall = f(row, "recall")
        out.append(
            {
                "evidence_source": "retained_tail_rule_mining",
                "slice": f"{feature}:top_10_percent_random_product",
                "hypothesis": (
                    "product_combinatorial_proxy"
                    if control != "none" and classify_enrichment(enrichment, recall) in {"collapses", "ambiguous"}
                    else "low_ridge_magnitude_signal"
                ),
                "control_or_bucket": wanted_controls[control],
                "classification": classify_enrichment(enrichment, recall),
                "n_selected": int(f(row, "selected", 0)),
                "selected_mean_sys": "",
                "selected_max_sys": "",
                "baseline_mean_sys": "",
                "baseline_p95_sys": "",
                "selected_above_baseline_p95": "",
                "effect_size": enrichment,
                "support_detail": f"precision={f(row, 'precision'):.6g}; recall={recall:.6g}; base_rate={f(row, 'base_rate'):.6g}",
                "interpretation_boundary": "retained-table diagnostic; every row already has sys; not generated-candidate validation",
            }
        )
    return out


def summarize_retained_buckets() -> list[dict[str, object]]:
    rows = read_tsv(TAIL / "bucket-interpretation-diagnostics.tsv")
    out = []
    for row in rows:
        if row["label"] != "top_decile":
            continue
        if row["feature"] not in {
            "ridge_symp_area_sum_over_volume_sqrt",
            "ridge_symp_area_mean_over_volume_sqrt",
            "ridge_symp_area_normalized_entropy",
            "ridge_symp_area_effective_face_count",
            "ridge_symp_area_le_1em2_over_volume_sqrt_fraction",
        }:
            continue
        feature = row["feature"]
        if "le_1em2" in feature:
            hypothesis = "small_area_fraction_story"
        elif "entropy" in feature or "effective" in feature:
            hypothesis = "ridge_area_concentration_distribution_signal"
        else:
            hypothesis = "low_ridge_magnitude_signal"
        out.append(
            {
                "evidence_source": "retained_fixed_source_facet_buckets",
                "slice": f"{row['capacity_source']}:facet_{row['facet_count']}:top_decile",
                "hypothesis": hypothesis,
                "control_or_bucket": f"{row['capacity_source']}:facet_{row['facet_count']}",
                "classification": classify_enrichment(f(row, "enrichment"), f(row, "recall")),
                "n_selected": int(f(row, "selected", 0)),
                "selected_mean_sys": "",
                "selected_max_sys": "",
                "baseline_mean_sys": "",
                "baseline_p95_sys": "",
                "selected_above_baseline_p95": "",
                "effect_size": f(row, "enrichment"),
                "support_detail": f"feature={feature}; rule={row['feature_tail_rule']}; precision={f(row, 'precision'):.6g}; recall={f(row, 'recall'):.6g}; spearman={f(row, 'spearman_with_sys'):.6g}",
                "interpretation_boundary": "retained-table fixed source/facet diagnostic; not generated-candidate validation",
            }
        )
    return out


def selected_overlap_matrices(out_dir: Path, selected_rows: list[dict[str, object]]) -> None:
    selected_only = [r for r in selected_rows if "selected" in r.get("evaluation_roles", [])]

    id_sets: dict[str, set[str]] = defaultdict(set)
    feature_sets: dict[str, set[str]] = defaultdict(set)
    for row in selected_only:
        cid = str(row["candidate_id"])
        for sid in row.get("selection_ids", []):
            id_sets[str(sid)].add(cid)
        for value in row.get("selection_rule_values", []):
            feature = value["selection_feature"]
            # Include the feature only when this row actually belongs to a selection id for it.
            if any(feature in sid for sid in row.get("selection_ids", [])):
                feature_sets[feature].add(cid)

    def write_matrix(path: Path, sets: dict[str, set[str]]) -> None:
        names = sorted(sets)
        rows = []
        for left in names:
            row = {"rule": left, "selected_rows": len(sets[left])}
            for right in names:
                inter = len(sets[left] & sets[right])
                union = len(sets[left] | sets[right])
                row[right] = f"{inter}|{inter / union:.6f}" if union else "0|0"
            rows.append(row)
        write_tsv(path, rows, ["rule", "selected_rows", *names])

    write_matrix(out_dir / "selection_id_overlap_matrix.tsv", id_sets)
    write_matrix(out_dir / "scalar_feature_overlap_matrix.tsv", feature_sets)

    summary = [
        {
            "matrix": "selection_id_overlap_matrix.tsv",
            "entries": len(id_sets),
            "cell_format": "intersection_count|jaccard",
            "row_universe": len({r["candidate_id"] for r in selected_only}),
        },
        {
            "matrix": "scalar_feature_overlap_matrix.tsv",
            "entries": len(feature_sets),
            "cell_format": "intersection_count|jaccard",
            "row_universe": len({r["candidate_id"] for r in selected_only}),
        },
    ]
    write_tsv(out_dir / "overlap_matrix_summary.tsv", summary, list(summary[0].keys()))


def conditional_concentration_split(
    selected_rows: list[dict[str, object]],
    eval_by_id: dict[str, dict[str, object]],
    feature_by_id: dict[str, dict[str, object]],
) -> list[dict[str, object]]:
    low_ridge_ids = set()
    for row in selected_rows:
        if "selected" not in row.get("evaluation_roles", []):
            continue
        if any(any(feature in sid for feature in RIDGE_MAGNITUDE_FEATURES) for sid in row.get("selection_ids", [])):
            low_ridge_ids.add(str(row["candidate_id"]))

    joined = []
    for cid in low_ridge_ids:
        if cid not in eval_by_id or cid not in feature_by_id:
            continue
        item = dict(feature_by_id[cid])
        item["sys"] = f(eval_by_id[cid], "sys")
        item["candidate_id"] = cid
        joined.append(item)

    split_rows = []
    features = CONCENTRATION_FEATURES + SMALL_AREA_FEATURES
    for feature, predicted_direction in features:
        for bucket in ["all", *sorted({str(r["bucket_id"]) for r in joined})]:
            bucket_rows = joined if bucket == "all" else [r for r in joined if r["bucket_id"] == bucket]
            if len(bucket_rows) < 8:
                continue
            threshold = median([f(r, feature) for r in bucket_rows])
            if not math.isfinite(threshold):
                continue
            if predicted_direction == "high":
                predicted = [r for r in bucket_rows if f(r, feature) >= threshold]
                opposite = [r for r in bucket_rows if f(r, feature) < threshold]
            else:
                predicted = [r for r in bucket_rows if f(r, feature) <= threshold]
                opposite = [r for r in bucket_rows if f(r, feature) > threshold]
            predicted_sys = [f(r, "sys") for r in predicted]
            opposite_sys = [f(r, "sys") for r in opposite]
            delta = mean(predicted_sys) - mean(opposite_sys)
            split_rows.append(
                {
                    "diagnostic_only": "yes",
                    "base_slice": "100k generated selected rows with at least one low ridge magnitude selection id",
                    "bucket": bucket,
                    "feature": feature,
                    "predicted_direction": predicted_direction,
                    "threshold": threshold,
                    "predicted_n": len(predicted),
                    "opposite_n": len(opposite),
                    "predicted_mean_sys": mean(predicted_sys),
                    "opposite_mean_sys": mean(opposite_sys),
                    "delta_mean_sys": delta,
                    "predicted_max_sys": max(predicted_sys) if predicted_sys else "",
                    "opposite_max_sys": max(opposite_sys) if opposite_sys else "",
                    "classification": "diagnostic_pending_rollup",
                    "boundary": "post-sys split mined from evaluated selected rows; not a proposer claim",
                }
            )

    # Roll classification into the all-bucket rows using per-bucket support.
    by_feature = defaultdict(list)
    for row in split_rows:
        if row["bucket"] != "all":
            by_feature[row["feature"]].append(row)
    for row in split_rows:
        if row["bucket"] == "all":
            support_rows = by_feature[row["feature"]]
            support = sum(1 for r in support_rows if f(r, "delta_mean_sys") > 0)
            row["classification"] = classify_delta(f(row, "delta_mean_sys"), support, len(support_rows))
        else:
            row["classification"] = classify_delta(f(row, "delta_mean_sys"), 1 if f(row, "delta_mean_sys") > 0 else 0, 1)
    return split_rows


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Build a compact ridge-mechanism discriminator report from existing sys-datascience artifacts."
    )
    parser.add_argument("--out-dir", type=Path, default=DEFAULT_OUT)
    parser.add_argument("--promising-artifacts", type=Path, default=DEFAULT_PROMISING)
    parser.add_argument("--ridge-1m-summary", type=Path, default=DEFAULT_RIDGE_1M)
    parser.add_argument("--tail-artifacts", type=Path, default=TAIL)
    parser.add_argument("--feature-cache", type=Path, default=DEFAULT_FEATURE_CACHE)
    return parser.parse_args()


def display_path(path: Path) -> str:
    path = path.resolve()
    try:
        return path.relative_to(REPO_ROOT).as_posix()
    except ValueError:
        return str(path)


def main() -> None:
    args = parse_args()
    out_dir = args.out_dir
    out_dir.mkdir(parents=True, exist_ok=True)

    global PROMISING, RIDGE_1M, TAIL
    PROMISING = args.promising_artifacts
    RIDGE_1M = args.ridge_1m_summary
    TAIL = args.tail_artifacts

    missing = [p for p in [PROMISING, RIDGE_1M, TAIL, args.feature_cache] if not p.exists()]
    if missing:
        raise SystemExit(f"missing required input(s): {missing}")

    selected_rows = read_jsonl(PROMISING / "selected-candidates-before-sys.jsonl")
    eval_rows = read_jsonl(PROMISING / "sys-evaluation-cache.jsonl")
    eval_by_id = {str(row["candidate_id"]): row for row in eval_rows}

    feature_by_id = load_feature_cache_for(args.feature_cache, set(eval_by_id))

    selection_rows, _ = summarize_selection_summary()
    discriminator_rows = []
    discriminator_rows.extend(summarize_retained_attribution())
    discriminator_rows.extend(summarize_retained_buckets())
    discriminator_rows.extend(summarize_1m())
    discriminator_rows.extend(selection_rows)

    concentration_rows = conditional_concentration_split(selected_rows, eval_by_id, feature_by_id)
    discriminator_rows.extend(
        {
            "evidence_source": "generated_100k_low_ridge_conditional_split",
            "slice": row["bucket"],
            "hypothesis": (
                "small_area_fraction_story"
                if "le_1em" in row["feature"]
                else "ridge_area_concentration_distribution_signal"
            ),
            "control_or_bucket": f"conditional_on_low_ridge_selected; feature={row['feature']}",
            "classification": row["classification"],
            "n_selected": row["predicted_n"],
            "selected_mean_sys": row["predicted_mean_sys"],
            "selected_max_sys": row["predicted_max_sys"],
            "baseline_mean_sys": row["opposite_mean_sys"],
            "baseline_p95_sys": "",
            "selected_above_baseline_p95": "",
            "effect_size": row["delta_mean_sys"],
            "support_detail": f"predicted_direction={row['predicted_direction']}; threshold={row['threshold']}; opposite_n={row['opposite_n']}",
            "interpretation_boundary": row["boundary"],
        }
        for row in concentration_rows
        if row["bucket"] == "all"
    )

    # Explicit Goodhart rows: enrichment exists, but selected maxima stay below 1 and do not improve
    # monotonically into the extreme generated tail.
    discriminator_rows.append(
        {
            "evidence_source": "generated_1m_and_100k_extreme_tail_boundary",
            "slice": "1m_per_bucket_low_sum_top10_and_100k_promising_scalars_union",
            "hypothesis": "extreme_tail_goodharting",
            "control_or_bucket": "extreme_low_ridge_selected_tail",
            "classification": "survives",
            "n_selected": 485,
            "selected_mean_sys": 0.579718264206536,
            "selected_max_sys": 0.867546058507634,
            "baseline_mean_sys": 0.359809716708119,
            "baseline_p95_sys": 0.692432690377396,
            "selected_above_baseline_p95": 124,
            "effect_size": "max_selected_below_1_after_extreme_filtering",
            "support_detail": "1M per-bucket low-sum max selected 0.866920080910149; 100k union max selected 0.867546058507634; no evaluated sys>1",
            "interpretation_boundary": "supports Goodhart/plateau caution, not proof that all low-ridge searches fail",
        }
    )

    fieldnames = [
        "evidence_source",
        "slice",
        "hypothesis",
        "control_or_bucket",
        "classification",
        "n_selected",
        "selected_mean_sys",
        "selected_max_sys",
        "baseline_mean_sys",
        "baseline_p95_sys",
        "selected_above_baseline_p95",
        "effect_size",
        "support_detail",
        "interpretation_boundary",
    ]
    write_tsv(out_dir / "ridge_effect_discriminator.tsv", discriminator_rows, fieldnames)

    selected_overlap_matrices(out_dir, selected_rows)

    concentration_fields = [
        "diagnostic_only",
        "base_slice",
        "bucket",
        "feature",
        "predicted_direction",
        "threshold",
        "predicted_n",
        "opposite_n",
        "predicted_mean_sys",
        "opposite_mean_sys",
        "delta_mean_sys",
        "predicted_max_sys",
        "opposite_max_sys",
        "classification",
        "boundary",
    ]
    write_tsv(out_dir / "conditional_concentration_split.tsv", concentration_rows, concentration_fields)

    rollup_rows = [
        {
            "hypothesis": "low_ridge_magnitude_signal",
            "classification": "survives_with_controls_caveat",
            "main_evidence": "retained fixed source/facet buckets show strong low-sum/mean enrichment; generated 1M and 100k per-bucket low ridge rules improve selected mean sys over matched baselines",
            "main_counterevidence_or_caveat": "pooled residualized retained attribution mostly collapses after source/facet/product and combinatorial controls; generated selected maxima remain below 1",
            "next_packet_recommendation": "Use ridge magnitude as real bucket-local explanatory signal, but do not run another single-scalar scale-up without a new held-out rule.",
        },
        {
            "hypothesis": "product_combinatorial_proxy",
            "classification": "weakens_pure_proxy_but_survives_as_major_confounder",
            "main_evidence": "retained ridge magnitude residuals mostly collapse under source/facet/product controls; count-only 100k rules collapse or stay ambiguous; scalar-feature overlap shows count rules select the same rows",
            "main_counterevidence_or_caveat": "1M and 100k per-bucket low-ridge generated rules still enrich within exact product buckets, so a pure bucket/count proxy is too strong",
            "next_packet_recommendation": "Future claims should say bucket-local low-ridge enrichment, not pooled ridge causality.",
        },
        {
            "hypothesis": "ridge_area_concentration_distribution_signal",
            "classification": "diagnostic_only_survives_for_normalized_entropy_and_low_top_share",
            "main_evidence": "Among 387 low-ridge selected 100k rows with feature cache, high normalized entropy and low max/top3 share split mean sys upward by about 0.047 to 0.051 overall; several buckets agree",
            "main_counterevidence_or_caveat": "This split was mined after sys evaluation and is not a generated-candidate proposer claim; raw entropy/effective count are weaker than normalized entropy/top-share",
            "next_packet_recommendation": "If reopened, freeze one concentration add-on rule before an independent generated run.",
        },
        {
            "hypothesis": "small_area_fraction_story",
            "classification": "collapses_or_ambiguous",
            "main_evidence": "Conditional low-ridge 100k splits on small-area fractions have near-zero or negative mean-sys deltas overall, and per-bucket medians often do not split rows because the fractions are discrete",
            "main_counterevidence_or_caveat": "Retained fixed-bucket diagnostics contain some small-area-fraction rows, but they are not enough to overcome the generated selected-tail split",
            "next_packet_recommendation": "Do not prioritize small-area-fraction features unless a new non-median rule or new feature definition is proposed.",
        },
        {
            "hypothesis": "extreme_tail_goodharting",
            "classification": "survives_as_caution",
            "main_evidence": "1M per-bucket low-sum and 100k promising-scalars enrich selected rows, but maximum selected sys stays around 0.867 and no evaluated row has sys > 1",
            "main_counterevidence_or_caveat": "This does not prove all low-ridge search fails; it only argues against more single-scalar extreme filtering without a new discriminator",
            "next_packet_recommendation": "Spawn an independent frozen concentration-rule validation only if the diagnostic split is worth converting into a proposer.",
        },
    ]
    write_tsv(
        out_dir / "hypothesis_rollup.tsv",
        rollup_rows,
        [
            "hypothesis",
            "classification",
            "main_evidence",
            "main_counterevidence_or_caveat",
            "next_packet_recommendation",
        ],
    )

    metadata = {
        "inputs": {
            "promising_scalars_artifact": display_path(PROMISING),
            "ridge_1m_summary": display_path(RIDGE_1M),
            "tail_rule_mining_artifacts": display_path(TAIL),
            "feature_cache": display_path(args.feature_cache),
            "feature_cache_required": True,
        },
        "row_counts": {
            "selected_candidates_before_sys": len(selected_rows),
            "sys_evaluation_cache": len(eval_rows),
            "feature_cache_rows_joined": len(feature_by_id),
            "conditional_low_ridge_rows_joined": len(
                {
                    r["candidate_id"]
                    for r in selected_rows
                    if "selected" in r.get("evaluation_roles", [])
                    and any(any(feature in sid for feature in RIDGE_MAGNITUDE_FEATURES) for sid in r.get("selection_ids", []))
                    and r["candidate_id"] in feature_by_id
                }
            ),
            "discriminator_rows": len(discriminator_rows),
            "conditional_concentration_rows": len(concentration_rows),
            "hypothesis_rollup_rows": len(rollup_rows),
        },
        "boundaries": [
            "Generated-candidate proposer evidence is limited to rules frozen before sys evaluation.",
            "Conditional concentration rows are diagnostic-only because the split is mined after sys values are known.",
            "No HKO-distance or flank claims are used.",
        ],
    }
    (out_dir / "run-metadata.json").write_text(json.dumps(metadata, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
