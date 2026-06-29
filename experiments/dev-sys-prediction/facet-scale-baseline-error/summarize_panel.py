#!/usr/bin/env python3
"""Summarize facet-count scale and baseline prediction-cloud errors."""

import argparse
import csv
import hashlib
import html
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


def compact(values):
    return [value for value in values if value is not None and math.isfinite(value)]


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


def jsonl_row_count(path):
    with path.open() as handle:
        return sum(1 for line in handle if line.strip())


def sha256_file(path):
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


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
        fixed_sigma_abs = [
            abs_first_present(
                row,
                "decomposition_fixed_sigma_linearization_error",
                "fixed_winner_sys_error_full",
            )
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
        fixed_winner_abs = [
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
                "max_abs_active_model_error": max(compact(active_model_abs), default=None),
                "median_abs_fixed_sigma_linearization_error": median(fixed_sigma_abs),
                "max_abs_fixed_sigma_linearization_error": max(
                    compact(fixed_sigma_abs), default=None
                ),
                "median_abs_inside_window_selection_error": median(inside_abs),
                "max_abs_inside_window_selection_error": max(
                    compact(inside_abs), default=None
                ),
                "median_abs_window_miss_error": median(window_abs),
                "max_abs_window_miss_error": max(compact(window_abs), default=None),
                "median_abs_capacity_linearization_error": median(capacity_abs),
                "max_abs_capacity_linearization_error": max(
                    compact(capacity_abs), default=None
                ),
                "median_abs_volume_linearization_error": median(volume_abs),
                "max_abs_volume_linearization_error": max(compact(volume_abs), default=None),
                "median_abs_capacity_volume_interaction_error": median(interaction_abs),
                "max_abs_capacity_volume_interaction_error": max(
                    compact(interaction_abs), default=None
                ),
                "median_abs_linearization_error": median(linear_abs),
                "max_abs_linearization_error": max(linear_abs, default=None),
                "median_abs_sigma_set_error": median(sigma_abs),
                "max_abs_sigma_set_error": max(sigma_abs, default=None),
                "median_abs_fixed_winner_sys_error": median(fixed_winner_abs),
                "max_abs_fixed_winner_sys_error": max(fixed_winner_abs, default=None),
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


def markdown_table(rows, fields):
    output = [
        "| " + " | ".join(fields) + " |",
        "| " + " | ".join("---" for _ in fields) + " |",
    ]
    for row in rows:
        output.append("| " + " | ".join(fmt(row.get(field)) for field in fields) + " |")
    return "\n".join(output)


def write_manifest(path, source_paths, summary_paths):
    rows = []
    for source_path in source_paths:
        source_path = source_path.resolve()
        rows.append(
            {
                "path": str(source_path.relative_to(PACKET)),
                "bytes": source_path.stat().st_size,
                "sha256": sha256_file(source_path),
                "jsonl_rows": jsonl_row_count(source_path)
                if source_path.suffix == ".jsonl"
                else None,
                "expected_empty": source_path.name
                in {
                    "endpoint-diagnostic.jsonl",
                    "endpoint-direction-scan.jsonl",
                    "prediction-cloud.jsonl",
                    "run-trace.jsonl",
                },
            }
        )
    for summary_path in summary_paths:
        summary_path = summary_path.resolve()
        rows.append(
            {
                "path": str(summary_path.relative_to(PACKET)),
                "bytes": summary_path.stat().st_size,
                "sha256": sha256_file(summary_path),
                "jsonl_rows": None,
                "expected_empty": False,
            }
        )
    path.write_text(json.dumps({"artifacts": rows}, indent=2, sort_keys=True) + "\n")


def write_summary(path, panel_scale_rows, branch_rows, prediction_rows):
    scale_by_facet = defaultdict(list)
    for row in panel_scale_rows:
        scale_by_facet[row["facet_count"]].append(row)
    scale_rows = []
    for facet_count, rows in sorted(scale_by_facet.items()):
        scale_rows.append(
            {
                "F": facet_count,
                "basepoints": len(rows),
                "median_flat_norm": median([row["flat_norm"] for row in rows]),
                "median_coord_rms": median([row["coord_rms"] for row in rows]),
                "median_pairwise_mean": median([row["geom_pairwise_mean"] for row in rows]),
            }
        )

    branch_display = [
        {
            "F": row["facet_count"],
            "threshold": row["threshold_relative"],
            "rows": row["rows"],
            "median_returned_orbits": row["median_returned_orbit_count"],
            "median_near_active": row["median_near_active_count"],
            "max_near_active": row["max_near_active_count"],
            "labels": row["labels"],
        }
        for row in branch_rows
        if row["threshold_relative"] in {1e-6, 0.01}
    ]

    prediction_display = [
        {
            "F": row["facet_count"],
            "t": row["step"],
            "ok/fail": f"{row['ok_rows']}/{row['failure_rows']}",
            "median_abs_error": row["median_abs_total_error"],
            "p90_abs_error": row["p90_abs_total_error"],
            "max_abs_error": row["max_abs_total_error"],
            "median_combined_linearization": row["median_abs_linearization_error"],
            "median_window_miss": row["median_abs_window_miss_error"],
            "target_best_missed": row["target_best_not_in_base_window"],
        }
        for row in prediction_rows
    ]

    text = f"""# Facet-Scale Baseline Error Summary

Generated by `summarize_panel.py` from the checked compact panel,
branch diagnostic rows, and local finite-radius prediction rows.

## Scale By Facet Count

{markdown_table(scale_rows, ["F", "basepoints", "median_flat_norm", "median_coord_rms", "median_pairwise_mean"])}

## Branch Window Snapshot

{markdown_table(branch_display, ["F", "threshold", "rows", "median_returned_orbits", "median_near_active", "max_near_active", "labels"])}

## Prediction Error By Radius

{markdown_table(prediction_display, ["F", "t", "ok/fail", "median_abs_error", "p90_abs_error", "max_abs_error", "median_combined_linearization", "median_window_miss", "target_best_missed"])}

This retained packet predates the full split into fixed-sigma linearization and
inside-window branch-selection error. Its `combined_linearization` column is
the older base-window first-order term. Regenerated rows expose the full
fixed-sigma / inside-window / window-miss split in the CSV output.
"""
    path.write_text(text)


def write_prediction_svg(path, prediction_rows):
    series = defaultdict(list)
    values = []
    for row in prediction_rows:
        median_error = row.get("median_abs_total_error")
        max_error = row.get("max_abs_total_error")
        if median_error is None or max_error is None or median_error <= 0 or max_error <= 0:
            continue
        series[row["facet_count"]].append((row["step"], median_error, max_error))
        values.extend([row["step"], median_error, max_error])
    if not values:
        path.write_text("")
        return

    width = 760
    height = 420
    left = 72
    right = 28
    top = 32
    bottom = 62
    plot_w = width - left - right
    plot_h = height - top - bottom

    x_values = [row["step"] for row in prediction_rows if row.get("step", 0) > 0]
    y_values = [
        value
        for row in prediction_rows
        for value in [row.get("median_abs_total_error"), row.get("max_abs_total_error")]
        if value is not None and value > 0
    ]
    min_x, max_x = min(x_values), max(x_values)
    min_y, max_y = min(y_values), max(y_values)
    min_log_x, max_log_x = math.log10(min_x), math.log10(max_x)
    min_log_y = math.floor(math.log10(min_y))
    max_log_y = math.ceil(math.log10(max_y))

    def sx(value):
        return left + (math.log10(value) - min_log_x) / (max_log_x - min_log_x) * plot_w

    def sy(value):
        return top + (max_log_y - math.log10(value)) / (max_log_y - min_log_y) * plot_h

    colors = {6: "#1f77b4", 10: "#d62728", 12: "#2ca02c"}
    parts = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">',
        "<style>text{font-family:Arial,sans-serif;font-size:12px} .label{font-size:13px} .title{font-size:16px;font-weight:700}</style>",
        '<rect width="100%" height="100%" fill="white"/>',
        f'<text class="title" x="{left}" y="22">Prediction error by radius</text>',
        f'<line x1="{left}" y1="{top + plot_h}" x2="{left + plot_w}" y2="{top + plot_h}" stroke="#333"/>',
        f'<line x1="{left}" y1="{top}" x2="{left}" y2="{top + plot_h}" stroke="#333"/>',
    ]

    for tick in sorted(set(x_values)):
        x = sx(tick)
        parts.extend(
            [
                f'<line x1="{x:.1f}" y1="{top}" x2="{x:.1f}" y2="{top + plot_h}" stroke="#eee"/>',
                f'<text x="{x:.1f}" y="{top + plot_h + 20}" text-anchor="middle">{fmt(tick)}</text>',
            ]
        )
    for exponent in range(int(min_log_y), int(max_log_y) + 1):
        value = 10**exponent
        y = sy(value)
        parts.extend(
            [
                f'<line x1="{left}" y1="{y:.1f}" x2="{left + plot_w}" y2="{y:.1f}" stroke="#eee"/>',
                f'<text x="{left - 8}" y="{y + 4:.1f}" text-anchor="end">1e{exponent}</text>',
            ]
        )

    for facet_count, rows in sorted(series.items()):
        rows = sorted(rows)
        color = colors.get(facet_count, "#444")
        median_points = " ".join(f"{sx(step):.1f},{sy(median):.1f}" for step, median, _ in rows)
        max_points = " ".join(f"{sx(step):.1f},{sy(max_error):.1f}" for step, _, max_error in rows)
        parts.append(
            f'<polyline points="{median_points}" fill="none" stroke="{color}" stroke-width="2.5"/>'
        )
        parts.append(
            f'<polyline points="{max_points}" fill="none" stroke="{color}" stroke-width="1.5" stroke-dasharray="5 4"/>'
        )
        for step, median, max_error in rows:
            parts.append(f'<circle cx="{sx(step):.1f}" cy="{sy(median):.1f}" r="3" fill="{color}"/>')
            parts.append(
                f'<circle cx="{sx(step):.1f}" cy="{sy(max_error):.1f}" r="2.5" fill="white" stroke="{color}" stroke-width="1.5"/>'
            )

    legend_x = left + plot_w - 130
    legend_y = top + 8
    parts.append(f'<rect x="{legend_x - 8}" y="{legend_y - 16}" width="138" height="92" fill="white" stroke="#ddd"/>')
    for index, facet_count in enumerate(sorted(series)):
        y = legend_y + index * 20
        color = colors.get(facet_count, "#444")
        parts.append(f'<line x1="{legend_x}" y1="{y}" x2="{legend_x + 24}" y2="{y}" stroke="{color}" stroke-width="2.5"/>')
        parts.append(f'<text x="{legend_x + 32}" y="{y + 4}">F={facet_count} median</text>')
    parts.append(f'<line x1="{legend_x}" y1="{legend_y + 64}" x2="{legend_x + 24}" y2="{legend_y + 64}" stroke="#555" stroke-width="1.5" stroke-dasharray="5 4"/>')
    parts.append(f'<text x="{legend_x + 32}" y="{legend_y + 68}">max</text>')

    parts.append(f'<text class="label" x="{left + plot_w / 2}" y="{height - 18}" text-anchor="middle">radius t</text>')
    parts.append(
        f'<text class="label" transform="translate(18 {top + plot_h / 2}) rotate(-90)" text-anchor="middle">absolute prediction error</text>'
    )
    parts.append(f'<text x="{left}" y="{height - 4}">{html.escape("Log-log; solid = median, dashed = max per (F,t).")}</text>')
    parts.append("</svg>\n")
    path.write_text("\n".join(parts))


def main():
    args = parse_args()
    facet_counts = [int(value) for value in args.facet_counts.split(",") if value]
    panel_rows = list(load_jsonl(args.panel))
    poly_id_to_facet = {row["poly_id"]: int(row["facet_count"]) for row in panel_rows}

    panel_scale_rows = [scale_row(row) for row in panel_rows]
    branch_rows = branch_summary(args.branch_dir, poly_id_to_facet)
    prediction_rows = prediction_summary(args.prediction_dir, poly_id_to_facet)

    args.out_dir.mkdir(parents=True, exist_ok=True)
    panel_scale_path = args.out_dir / "panel-scale.csv"
    branch_path = args.out_dir / "branch-window-by-facet.csv"
    prediction_path = args.out_dir / "prediction-error-by-facet-step.csv"
    summary_path = args.out_dir / "SUMMARY.md"
    manifest_path = args.out_dir / "MANIFEST.json"
    figure_path = args.out_dir / "prediction-error-by-radius.svg"
    write_csv(panel_scale_path, panel_scale_rows)
    write_csv(branch_path, branch_rows)
    write_csv(prediction_path, prediction_rows)
    write_summary(summary_path, panel_scale_rows, branch_rows, prediction_rows)
    write_prediction_svg(figure_path, prediction_rows)
    write_manifest(
        manifest_path,
        [
            args.panel,
            args.branch_dir / "branch-set-diagnostic.jsonl",
            args.branch_dir / "fixture-selection.jsonl",
            args.prediction_dir / "fixture-selection.jsonl",
            args.prediction_dir / "local-geometry-probe.jsonl",
            args.prediction_dir / "prediction-cloud.jsonl",
            args.prediction_dir / "run-trace.jsonl",
            args.prediction_dir / "endpoint-diagnostic.jsonl",
            args.prediction_dir / "endpoint-direction-scan.jsonl",
        ],
        [panel_scale_path, branch_path, prediction_path, summary_path, figure_path],
    )

    print(args.out_dir)


if __name__ == "__main__":
    main()
