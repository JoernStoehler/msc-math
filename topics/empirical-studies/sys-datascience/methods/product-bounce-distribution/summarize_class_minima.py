#!/usr/bin/env python3
"""Aggregate the detailed exact class-minima JSONL artifact.

No geometry is evaluated here.  This consumer records a structural audit and
the exact per-class-minimum availability boundary separately from observations.
"""
import argparse
import json
import math
from collections import Counter, defaultdict
from pathlib import Path
from statistics import fmean


def read_jsonl(path):
    with open(path, encoding="utf-8") as f:
        return [json.loads(line) for line in f if line.strip()]


def pearson(xs, ys):
    if len(xs) < 2:
        return None
    mx, my = fmean(xs), fmean(ys)
    dx = sum((x - mx) ** 2 for x in xs)
    dy = sum((y - my) ** 2 for y in ys)
    if dx == 0 or dy == 0:
        return None
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / math.sqrt(dx * dy)


def availability(rows):
    result = {}
    for bounce in ("2", "3"):
        present = sum(row["class_minima"][bounce] is not None for row in rows)
        result[bounce] = {"present": present, "absent": len(rows) - present}
    complete = [r for r in rows if r["normalized_three_minus_two_gap"] is not None]
    result["both"] = {"present": len(complete), "absent": len(rows) - len(complete)}
    return result, complete


def producer_capacity_identity(rows):
    """Check the retained producer winner against its same-class re-evaluation."""
    matches = 0
    mismatches = []
    missing_class = []
    for row in rows:
        bounce = str(row["producer_bounces"])
        winner = row["class_minima"][bounce]
        if winner is None:
            missing_class.append(row["name"])
            continue
        delta = abs(row["stored_capacity"] - winner["action"])
        tolerance = 1e-10 * (1 + abs(row["stored_capacity"]))
        if delta <= tolerance:
            matches += 1
        else:
            mismatches.append({"name": row["name"], "absolute_difference": delta})
    return {
        "definition": "stored producer capacity compared to the exactly certified minimum in its stored producer_bounces class; match tolerance 1e-10*(1+abs(stored_capacity))",
        "rows": len(rows),
        "matches": matches,
        "mismatches": mismatches,
        "missing_stored_class_minimum": missing_class,
    }


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--input", required=True)
    ap.add_argument("--out", required=True)
    args = ap.parse_args()
    rows = read_jsonl(args.input)
    if not rows:
        raise ValueError("empty class-minima input")
    by_bucket = defaultdict(list)
    for row in rows:
        by_bucket[f'{row["k"]}x{row["m"]}'].append(row)
    available, complete = availability(rows)
    active = {}
    for bounce in ("2", "3"):
        counts = [n for r in rows for n in r["active_vertex_counts"][bounce]]
        active[bounce] = {"minimizer_words": len(counts), "by_active_vertex_count": dict(sorted(Counter(counts).items()))}

    def bucket_summary(rs):
        _, usable = availability(rs)
        producer = {b: [r for r in rs if r["producer_bounces"] == b] for b in (2, 3)}
        if producer[2] and producer[3]:
            mean_log_capacity = {b: fmean(math.log(r["stored_capacity"]) for r in producer[b]) for b in (2, 3)}
            mean_log_2volume = {b: fmean(math.log(2 * r["stored_volume"]) for r in producer[b]) for b in (2, 3)}
            mean_log_sys = {b: fmean(math.log(r["stored_sys"]) for r in producer[b]) for b in (2, 3)}
            decomposition = {
                "definition": "mean(log sys)_3 - mean(log sys)_2 = 2[mean(log capacity)_3 - mean(log capacity)_2] - [mean(log(2 volume))_3 - mean(log(2 volume))_2]",
                "three_minus_two_mean_log_sys": mean_log_sys[3] - mean_log_sys[2],
                "capacity_contribution": 2 * (mean_log_capacity[3] - mean_log_capacity[2]),
                "volume_contribution": -(mean_log_2volume[3] - mean_log_2volume[2]),
            }
        else:
            decomposition = None
        return {
            "rows": len(rs),
            "class_minimum_availability": availability(rs)[0],
            "complete_class_rows": len(usable),
            "mean_normalized_three_minus_two_gap": fmean(r["normalized_three_minus_two_gap"] for r in usable) if usable else None,
            "mean_log_action_ratio": fmean(math.log(r["class_minima"]["3"]["action"] / r["class_minima"]["2"]["action"]) for r in usable) if usable else None,
            "producer_bounce_log_sys_decomposition": decomposition,
        }

    gaps = [r["normalized_three_minus_two_gap"] for r in complete]
    angular = {
        "definition": "Pearson association with (A3-A2)/A2 among rows where both exact class minima exist; min cyclic normal-angle gap in either product factor.",
        "complete_class_rows": len(complete),
        "product_min_gap_vs_normalized_gap_pearson": pearson([r["product_min_angular_gap_rad"] for r in complete], gaps),
        "q_min_gap_vs_normalized_gap_pearson": pearson([r["q_min_angular_gap_rad"] for r in complete], gaps),
        "p_min_gap_vs_normalized_gap_pearson": pearson([r["p_min_angular_gap_rad"] for r in complete], gaps),
    }
    result = {
        "input": str(Path(args.input)),
        "row_count": len(rows),
        "definitions": {
            "A_b": "exact minimum action among admissible candidates returned by the existing f64 solved billiard stream with b bounce blocks, then exactly certified",
            "ties": "all exact minimizer sigmas returned by CertifiedOrbitSetMode::MinimizersOnly",
            "normalized_three_minus_two_gap": "(A3-A2)/A2; null when either class has no admissible solved candidate",
        },
        "observations": {
            "class_minimum_availability": available,
            "producer_capacity_class_winner_identity": producer_capacity_identity(rows),
            "active_vertex_counts": active,
            "by_bucket": {bucket: bucket_summary(rs) for bucket, rs in sorted(by_bucket.items())},
            "angular_gap_association": angular,
        },
        "inference_boundary": "This is a re-evaluation of retained rows, not generated-candidate validation, a causal mechanism result, or an exhaustive global enumeration theorem about generic product polytopes. Null class minima report absence of an admissible candidate in the existing f64 solved stream for that class and must not be treated as a numerical value.",
    }
    availability_audit = Path(args.input).with_name("class-minima-availability-audit.json")
    if availability_audit.exists():
        result["observations"]["availability_audit"] = json.loads(
            availability_audit.read_text(encoding="utf-8")
        )
    Path(args.out).write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
