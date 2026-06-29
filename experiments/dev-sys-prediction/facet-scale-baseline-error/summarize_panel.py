#!/usr/bin/env python3
"""Summarize facet-count scale and baseline prediction-cloud errors."""

import argparse
import csv
import json
import math
from collections import Counter, defaultdict
from pathlib import Path


PACKET = Path(__file__).resolve().parent


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("--panel", type=Path, default=PACKET / "polytope-panel.jsonl")
    parser.add_argument("--branch-dir", type=Path, required=True)
    parser.add_argument("--prediction-dir", type=Path, required=True)
    parser.add_argument("--out-dir", type=Path, default=PACKET / "summaries")
    parser.add_argument("--facet-counts", default="6,10,12")
    return parser.parse_args()


def load_jsonl(path):
    with path.open() as handle:
        for line in handle:
            if line.strip():
                yield json.loads(line)


def quantile(values, q):
    values = sorted(v for v in values if v is not None and math.isfinite(v))
    if not values:
        return None
    pos = (len(values) - 1) * q
    lo = math.floor(pos)
    hi = math.ceil(pos)
    if lo == hi:
        return values[lo]
    return values[lo] * (hi - pos) + values[hi] * (pos - lo)


def median(values):
    return quantile(values, 0.5)


def mean(values):
    values = [v for v in values if v is not None and math.isfinite(v)]
    if not values:
        return None
    return sum(values) / len(values)


def sample_sd(values):
    values = [v for v in values if v is not None and math.isfinite(v)]
    if len(values) < 2:
        return None
    avg = sum(values) / len(values)
    return math.sqrt(sum((value - avg) ** 2 for value in values) / (len(values) - 1))


def normal_se_mean(values):
    values = [v for v in values if v is not None and math.isfinite(v)]
    sd = sample_sd(values)
    if sd is None:
        return None
    return sd / math.sqrt(len(values))


def tail_catch_probability(row_count, tail_mass):
    if row_count <= 0:
        return None
    return 1.0 - (1.0 - tail_mass) ** row_count


def fmt(value):
    if value is None:
        return ""
    if isinstance(value, int):
        return str(value)
    if isinstance(value, float):
        return f"{value:.6g}"
    return str(value)


def flat_vertices(row):
    return [coord for vertex in row["dual_vertices_f64"] for coord in vertex]


def first_present(row, *keys):
    for key in keys:
        if row.get(key) is not None:
            return row.get(key)
    return None


def abs_first_present(row, *keys):
    value = first_present(row, *keys)
    return abs(value) if value is not None and math.isfinite(value) else None


def abs_delta_between(row, predicted_key, observed_key, *fallback_predicted_keys):
    predicted = first_present(row, predicted_key, *fallback_predicted_keys)
    observed = row.get(observed_key)
    if predicted is None or observed is None:
        return None
    return abs(predicted - observed)


def scale_row(row):
    flat = flat_vertices(row)
    norm = math.sqrt(sum(x * x for x in flat))
    facet_count = int(row["facet_count"])
    coord_rms = norm / math.sqrt(len(flat))
    return {
        "poly_id": row["poly_id"],
        "facet_count": facet_count,
        "flat_norm": norm,
        "coord_rms": coord_rms,
        "geom_pairwise_mean": row.get("geom_vol1_pairwise_dist_mean"),
        "geom_pairwise_max": row.get("geom_vol1_pairwise_dist_max"),
        "geom_norm_mean": row.get("geom_vol1_norm_mean"),
    }


def branch_summary(branch_dir, poly_id_to_facet):
    rows = list(load_jsonl(branch_dir / "branch-set-diagnostic.jsonl"))
    grouped = defaultdict(list)
    failures = Counter()
    for row in rows:
        facet_count = poly_id_to_facet.get(row["poly_id"], row.get("input_facet_count"))
        key = (facet_count, row["threshold_relative"])
        grouped[key].append(row)
        if row.get("failure"):
            failures[key] += 1

    output = []
    for (facet_count, threshold), group in sorted(grouped.items()):
        output.append(
            {
                "facet_count": facet_count,
                "threshold_relative": threshold,
                "rows": len(group),
                "failures": failures[(facet_count, threshold)],
                "median_returned_orbit_count": median(
                    [row.get("returned_orbit_count") for row in group]
                ),
                "median_near_active_count": median(
                    [row.get("near_active_count") for row in group]
                ),
                "max_near_active_count": max(
                    [row.get("near_active_count") or 0 for row in group], default=0
                ),
                "labels": " ".join(
                    f"{label}:{count}"
                    for label, count in sorted(
                        Counter(row["degeneracy_label"] for row in group).items()
                    )
                ),
            }
        )
    return output


def prediction_summary(prediction_dir, poly_id_to_facet):
    prediction_path = prediction_dir / "prediction-cloud.jsonl"
    if prediction_path.exists() and prediction_path.stat().st_size > 0:
        rows = list(load_jsonl(prediction_path))
        model_source = "prediction-cloud"
    else:
        rows = list(load_jsonl(prediction_dir / "local-geometry-probe.jsonl"))
        model_source = "local-geometry-probe"
    grouped = defaultdict(list)
    for row in rows:
        grouped[(poly_id_to_facet[row["poly_id"]], row["step"])].append(row)

    output = []
    for (facet_count, step), group in sorted(grouped.items()):
        ok = [row for row in group if row["status"] == "ok"]
        failures = [row for row in group if row["status"] != "ok"]
        total_abs = [
            abs(row["decomposition_total_prediction_error"])
            for row in ok
            if row.get("decomposition_total_prediction_error") is not None
        ]
        linear_abs = [
            abs(row["decomposition_linearization_error"])
            for row in ok
            if row.get("decomposition_linearization_error") is not None
        ]
        fixed_abs = [
            abs_first_present(row, "decomposition_fixed_sigma_linearization_error")
            for row in ok
        ]
        inside_abs = [
            abs_first_present(row, "decomposition_inside_window_selection_error")
            for row in ok
        ]
        window_abs = [
            abs_first_present(
                row, "decomposition_window_miss_error", "decomposition_sigma_set_error"
            )
            for row in ok
        ]
        capacity_abs = [
            abs_first_present(row, "decomposition_capacity_linearization_error")
            for row in ok
        ]
        volume_abs = [
            abs_first_present(row, "decomposition_volume_linearization_error")
            for row in ok
        ]
        interaction_abs = [
            abs_first_present(row, "decomposition_capacity_volume_interaction_error")
            for row in ok
        ]
        sigma_abs = [
            abs(row["decomposition_sigma_set_error"])
            for row in ok
            if row.get("decomposition_sigma_set_error") is not None
        ]
        fixed_abs = [
            abs(row["fixed_winner_sys_error_full"])
            for row in ok
            if row.get("fixed_winner_sys_error_full") is not None
        ]
        active_model_abs = [
            abs_delta_between(
                row,
                "direction_model_predicted_delta_sys",
                "observed_delta_sys",
                "predicted_delta_sys",
            )
            for row in ok
        ]
        target_base_sys_gaps = [
            row.get("target_best_sigma_base_sys_gap")
            for row in ok
            if row.get("target_best_sigma_base_sys_gap") is not None
        ]
        target_base_relative_action_gaps = [
            row.get("target_best_sigma_base_relative_action_gap")
            for row in ok
            if row.get("target_best_sigma_base_relative_action_gap") is not None
        ]
        output.append(
            {
                "model_source": model_source,
                "facet_count": facet_count,
                "step": step,
                "rows": len(group),
                "ok_rows": len(ok),
                "failure_rows": len(failures),
                "failure_statuses": " ".join(
                    f"{label}:{count}"
                    for label, count in sorted(Counter(row["status"] for row in failures).items())
                ),
                "mean_abs_total_error": mean(total_abs),
                "sd_abs_total_error": sample_sd(total_abs),
                "normal_se_mean_abs_total_error": normal_se_mean(total_abs),
                "median_abs_total_error": median(total_abs),
                "p90_abs_total_error": quantile(total_abs, 0.9),
                "max_abs_total_error": max(total_abs, default=None),
                "median_abs_active_model_error": median(active_model_abs),
                "max_abs_active_model_error": max(active_model_abs, default=None),
                "median_abs_fixed_sigma_linearization_error": median(fixed_abs),
                "max_abs_fixed_sigma_linearization_error": max(
                    [v for v in fixed_abs if v is not None], default=None
                ),
                "median_abs_inside_window_selection_error": median(inside_abs),
                "max_abs_inside_window_selection_error": max(
                    [v for v in inside_abs if v is not None], default=None
                ),
                "median_abs_window_miss_error": median(window_abs),
                "max_abs_window_miss_error": max(
                    [v for v in window_abs if v is not None], default=None
                ),
                "median_abs_capacity_linearization_error": median(capacity_abs),
                "max_abs_capacity_linearization_error": max(
                    [v for v in capacity_abs if v is not None], default=None
                ),
                "median_abs_volume_linearization_error": median(volume_abs),
                "max_abs_volume_linearization_error": max(
                    [v for v in volume_abs if v is not None], default=None
                ),
                "median_abs_capacity_volume_interaction_error": median(interaction_abs),
                "max_abs_capacity_volume_interaction_error": max(
                    [v for v in interaction_abs if v is not None], default=None
                ),
                "median_abs_linearization_error": median(linear_abs),
                "max_abs_linearization_error": max(linear_abs, default=None),
                "median_abs_sigma_set_error": median(sigma_abs),
                "max_abs_sigma_set_error": max(sigma_abs, default=None),
                "median_abs_fixed_winner_sys_error": median(fixed_abs),
                "max_abs_fixed_winner_sys_error": max(fixed_abs, default=None),
                "target_best_not_in_base_window": sum(
                    1
                    for row in ok
                    if row.get("target_best_sigma_in_base_candidate_window") is False
                ),
                "target_best_base_gap_available_rows": len(target_base_sys_gaps),
                "median_target_best_base_sys_gap": median(target_base_sys_gaps),
                "p90_target_best_base_sys_gap": quantile(target_base_sys_gaps, 0.9),
                "max_target_best_base_sys_gap": max(target_base_sys_gaps, default=None),
                "median_target_best_base_relative_action_gap": median(
                    target_base_relative_action_gaps
                ),
                "median_base_candidate_window_count": median(
                    [
                        row.get("base_candidate_window_count")
                        or row.get("base_returned_orbit_count")
                        for row in ok
                    ]
                ),
                "catch_probability_for_5pct_tail": tail_catch_probability(len(ok), 0.05),
                "catch_probability_for_1pct_tail": tail_catch_probability(len(ok), 0.01),
            }
        )
    return output


def write_csv(path, rows):
    path.parent.mkdir(parents=True, exist_ok=True)
    if not rows:
        path.write_text("")
        return
    fields = list(rows[0].keys())
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fields, lineterminator="\n")
        writer.writeheader()
        for row in rows:
            writer.writerow({key: fmt(row.get(key)) for key in fields})


def main():
    args = parse_args()
    facet_counts = [int(value) for value in args.facet_counts.split(",") if value]
    panel_rows = list(load_jsonl(args.panel))
    poly_id_to_facet = {row["poly_id"]: int(row["facet_count"]) for row in panel_rows}

    panel_scale_rows = [scale_row(row) for row in panel_rows]
    branch_rows = branch_summary(args.branch_dir, poly_id_to_facet)
    prediction_rows = prediction_summary(args.prediction_dir, poly_id_to_facet)

    args.out_dir.mkdir(parents=True, exist_ok=True)
    stale_summary = args.out_dir / "SUMMARY.md"
    if stale_summary.exists():
        stale_summary.unlink()
    write_csv(args.out_dir / "panel-scale.csv", panel_scale_rows)
    write_csv(args.out_dir / "branch-window-by-facet.csv", branch_rows)
    write_csv(args.out_dir / "prediction-error-by-facet-step.csv", prediction_rows)

    print(args.out_dir)


if __name__ == "__main__":
    main()
