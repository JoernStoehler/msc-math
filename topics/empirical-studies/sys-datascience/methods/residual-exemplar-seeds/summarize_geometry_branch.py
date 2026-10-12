#!/usr/bin/env python3
"""Join the frozen residual panel to incidence and branch diagnostics."""

import argparse
import csv
import hashlib
import json
from pathlib import Path

WINDOWS = (1e-12, 1e-6, 1e-3, 1e-2)


def load_jsonl(path):
    with path.open() as handle:
        return [json.loads(line) for line in handle if line.strip()]


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        while chunk := handle.read(65536):
            digest.update(chunk)
    return digest.hexdigest()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--panel", type=Path, required=True)
    parser.add_argument("--geometry", type=Path, required=True)
    parser.add_argument("--branches", type=Path, required=True)
    parser.add_argument("--out-dir", type=Path, required=True)
    args = parser.parse_args()
    args.out_dir.mkdir(parents=True, exist_ok=True)

    geometry = {row["poly_id"]: row for row in load_jsonl(args.geometry)}
    branches = {}
    for row in load_jsonl(args.branches):
        branches[(row["poly_id"], row["threshold_relative"])] = row
    with args.panel.open(newline="") as handle:
        panel = list(csv.DictReader(handle, delimiter="\t"))

    columns = ["panel_role", "arm", "bucket", "low_poly_id", "high_poly_id", "low_sys", "high_sys"]
    geometry_fields = (
        "adjacent_abs_difference_mean",
        "adjacent_endpoint_pearson",
        "top_quartile_induced_edge_count",
        "top_quartile_component_count",
        "top_quartile_internal_edge_fraction",
    )
    for field in geometry_fields:
        columns += [f"low_{field}", f"high_{field}"]
    columns += ["low_returned_orbit_count", "high_returned_orbit_count"]
    for window in WINDOWS:
        label = f"{window:g}"
        columns += [
            f"low_near_active_raw_orbits_{label}",
            f"high_near_active_raw_orbits_{label}",
            f"low_near_active_cyclic_classes_{label}",
            f"high_near_active_cyclic_classes_{label}",
        ]
    columns += [
        "high_exact_raw_sigmas",
        "high_exact_canonical_cyclic_sigmas",
        "low_exact_raw_sigmas",
        "low_exact_canonical_cyclic_sigmas",
    ]

    output = []
    for record in panel:
        ids = [record["poly_id_a"]]
        if record["poly_id_b"]:
            ids.append(record["poly_id_b"])
        ids.sort(key=lambda poly_id: geometry[poly_id]["input_sys"])
        low, high = ids[0], ids[-1]
        row = {
            "panel_role": record["panel_role"], "arm": record["arm"], "bucket": record["bucket"],
            "low_poly_id": low, "high_poly_id": high if len(ids) == 2 else "",
            "low_sys": geometry[low]["input_sys"], "high_sys": geometry[high]["input_sys"] if len(ids) == 2 else "",
        }
        for field in geometry_fields:
            row[f"low_{field}"] = geometry[low][field]
            row[f"high_{field}"] = geometry[high][field] if len(ids) == 2 else ""
        row["low_returned_orbit_count"] = branches[(low, 1e-12)]["returned_orbit_count"]
        row["high_returned_orbit_count"] = branches[(high, 1e-12)]["returned_orbit_count"] if len(ids) == 2 else ""
        for window in WINDOWS:
            label = f"{window:g}"
            row[f"low_near_active_raw_orbits_{label}"] = branches[(low, window)]["near_active_raw_orbit_count"]
            row[f"high_near_active_raw_orbits_{label}"] = branches[(high, window)]["near_active_raw_orbit_count"] if len(ids) == 2 else ""
            row[f"low_near_active_cyclic_classes_{label}"] = branches[(low, window)]["near_active_distinct_cyclic_class_count"]
            row[f"high_near_active_cyclic_classes_{label}"] = branches[(high, window)]["near_active_distinct_cyclic_class_count"] if len(ids) == 2 else ""
        row["low_exact_raw_sigmas"] = json.dumps(branches[(low, 1e-12)]["near_active_raw_sigmas"], separators=(",", ":"))
        row["high_exact_raw_sigmas"] = json.dumps(branches[(high, 1e-12)]["near_active_raw_sigmas"], separators=(",", ":")) if len(ids) == 2 else ""
        row["low_exact_canonical_cyclic_sigmas"] = json.dumps(branches[(low, 1e-12)]["near_active_canonical_cyclic_sigmas"], separators=(",", ":"))
        row["high_exact_canonical_cyclic_sigmas"] = json.dumps(branches[(high, 1e-12)]["near_active_canonical_cyclic_sigmas"], separators=(",", ":")) if len(ids) == 2 else ""
        output.append(row)

    comparison_path = args.out_dir / "panel-comparison.tsv"
    with comparison_path.open("w", newline="") as handle:
        writer = csv.DictWriter(
            handle, delimiter="\t", fieldnames=columns, lineterminator="\n"
        )
        writer.writeheader()
        writer.writerows(output)

    pair_rows = [row for row in output if row["panel_role"] == "discordant_pair"]
    summary = {
        "method": "residual-exemplar-geometry-branch-summary-v1",
        "evidence_class": "post-target_G_hypothesis_seed",
        "input_sha256": {"panel": sha256(args.panel), "geometry": sha256(args.geometry), "branches": sha256(args.branches)},
        "contract": {
            "geometry_metrics": list(geometry_fields),
            "branch_windows_relative": WINDOWS,
            "pair_order": "low_sys_then_high_sys; sys was already used to select and order the panel",
            "sigma_word_contract": {
                "returned_orbit_count": "raw number returned by the orbit solver",
                "near_active_raw_orbits": "raw admissible returned words within the action window",
                "near_active_cyclic_classes": "distinct cyclic classes represented by the lexicographically smallest rotation of each nonempty raw word",
            },
        },
        "observations": {
            "pair_count": len(pair_rows),
            "pairs_with_more_near_active_cyclic_classes_at_low_sys_1e_3": sum(int(row["low_near_active_cyclic_classes_0.001"]) > int(row["high_near_active_cyclic_classes_0.001"]) for row in pair_rows),
            "pairs_tied_near_active_cyclic_classes_1e_3": sum(int(row["low_near_active_cyclic_classes_0.001"]) == int(row["high_near_active_cyclic_classes_0.001"]) for row in pair_rows),
            "product_pairs_with_four_exact_cyclic_classes_low_and_one_exact_cyclic_class_high": sum(row["arm"] == "product_exact_summary" and int(row["low_near_active_cyclic_classes_1e-12"]) == 4 and int(row["high_near_active_cyclic_classes_1e-12"]) == 1 for row in pair_rows),
            "geometry_metrics_showing_same_strict_direction_all_pairs": [],
        },
        "interpretation_boundary": "descriptive only: the panel and pair ordering are post-target, action spectra are target-cost diagnostics, and neither family is an independently validated proposer or mechanism",
    }
    with (args.out_dir / "comparison-summary.json").open("w") as handle:
        json.dump(summary, handle, indent=2)
        handle.write("\n")


if __name__ == "__main__":
    main()
