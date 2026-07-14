#!/usr/bin/env python3
"""Exact difference-body audit of retained two-bounce class minima."""

import argparse
import hashlib
import json
import math
from collections import Counter, defaultdict
from fractions import Fraction
from pathlib import Path
from statistics import fmean


EXPECTED_SHA256 = {
    "raw": "66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736",
    "classes": "187089804bd17fdac76bdaf51a8d8202e67fb2b14779fe9a418cc8da47c7b4c4",
    "table": "49825d7636246f71f4ebd419cf0ccbc86e39e6b7f43d4b03e889bb85e4887aea",
    "provenance": "6ff88a5accce9a7ec7e5a494107350b0974b2ce0268ea44caae36a18a7494ef2",
}


def sha256(path):
    digest = hashlib.sha256()
    with open(path, "rb") as handle:
        for chunk in iter(lambda: handle.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def read_jsonl(path):
    with open(path, encoding="utf-8") as handle:
        return [json.loads(line) for line in handle if line.strip()]


def frac(value):
    return Fraction(value)


def add(a, b):
    return (a[0] + b[0], a[1] + b[1])


def sub(a, b):
    return (a[0] - b[0], a[1] - b[1])


def scale(c, a):
    return (c * a[0], c * a[1])


def dot(a, b):
    return a[0] * b[0] + a[1] * b[1]


def cross(o, a, b):
    return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])


def convex_hull(points):
    points = sorted(set(points))
    if len(points) < 3:
        raise ValueError("full-dimensional polygon needs at least three vertices")
    lower = []
    for point in points:
        while len(lower) >= 2 and cross(lower[-2], lower[-1], point) <= 0:
            lower.pop()
        lower.append(point)
    upper = []
    for point in reversed(points):
        while len(upper) >= 2 and cross(upper[-2], upper[-1], point) <= 0:
            upper.pop()
        upper.append(point)
    hull = lower[:-1] + upper[:-1]
    if len(hull) < 3:
        raise ValueError("polygon vertices are collinear")
    return hull


def difference_hull(points):
    return convex_hull([sub(a, b) for a in points for b in points])


def width(points, direction):
    values = [dot(direction, point) for point in points]
    return max(values) - min(values)


def two_bounce_width(table_polygon, norm_polygon):
    """Return exact min_{d in boundary(P-P)} h_{Q-Q}(d) and witnesses."""
    table_polygon = convex_hull(table_polygon)
    norm_polygon = convex_hull(norm_polygon)
    dp = difference_hull(table_polygon)
    dq = difference_hull(norm_polygon)
    best = None
    witnesses = []
    for edge_index, a in enumerate(dp):
        b = dp[(edge_index + 1) % len(dp)]
        delta = sub(b, a)
        lines = [(dot(a, z), dot(delta, z)) for z in dq]
        parameters = {Fraction(0), Fraction(1)}
        for i, (intercept_i, slope_i) in enumerate(lines):
            for intercept_j, slope_j in lines[i + 1 :]:
                if slope_i == slope_j:
                    continue
                t = (intercept_j - intercept_i) / (slope_i - slope_j)
                if 0 <= t <= 1:
                    parameters.add(t)
        for t in parameters:
            d = add(a, scale(t, delta))
            value = max(intercept + t * slope for intercept, slope in lines)
            if best is None or value < best:
                best = value
                witnesses = [(edge_index, t, d)]
            elif value == best:
                witnesses.append((edge_index, t, d))

    unique = {}
    for edge_index, t, d in witnesses:
        record = unique.setdefault(
            d,
            {
                "d": d,
                "edge_indices": set(),
                "edge_interior": False,
            },
        )
        record["edge_indices"].add(edge_index)
        record["edge_interior"] |= 0 < t < 1
    records = []
    for record in unique.values():
        values = [dot(record["d"], point) for point in norm_polygon]
        maximum, minimum = max(values), min(values)
        records.append(
            {
                "d": record["d"],
                "edge_indices": sorted(record["edge_indices"]),
                "edge_interior": record["edge_interior"],
                "max_support_count": sum(value == maximum for value in values),
                "min_support_count": sum(value == minimum for value in values),
            }
        )
        if maximum - minimum != best:
            raise AssertionError("difference-body support and width disagree")
    return best, records


def fixture_results():
    fixtures = [
        {
            "name": "unit_halfwidth_square_pair",
            "p": [(-1, -1), (1, -1), (1, 1), (-1, 1)],
            "q": [(-1, -1), (1, -1), (1, 1), (-1, 1)],
            "expected": Fraction(4),
        },
        {
            "name": "rectangle_pair_min_axis_product",
            "p": [(-2, -1), (2, -1), (2, 1), (-2, 1)],
            "q": [(-3, -5), (3, -5), (3, 5), (-3, 5)],
            "expected": Fraction(20),
        },
        {
            "name": "triangle_rotated_difference_body",
            "p": [(0, 0), (1, 0), (0, 1)],
            "q": [(0, 1), (1, 1), (1, 0), (0, -1), (-1, -1), (-1, 0)],
            "expected": Fraction(2),
        },
    ]
    output = []
    for fixture in fixtures:
        p = [tuple(map(Fraction, point)) for point in fixture["p"]]
        q = [tuple(map(Fraction, point)) for point in fixture["q"]]
        actual, witnesses = two_bounce_width(p, q)
        if actual != fixture["expected"]:
            raise AssertionError(f"fixture {fixture['name']}: {actual} != {fixture['expected']}")
        output.append(
            {
                "name": fixture["name"],
                "expected": str(fixture["expected"]),
                "actual": str(actual),
                "witness_count": len(witnesses),
            }
        )
    return output


def factor_vertices(raw):
    vertices = [tuple(frac(x) for x in row) for row in raw["vertices_rational"]]
    q = convex_hull([(row[0], row[1]) for row in vertices])
    p = convex_hull([(row[2], row[3]) for row in vertices])
    if len(q) != raw["k"] or len(p) != raw["m"]:
        raise ValueError(f"{raw['name']}: factor vertex counts disagree with (k,m)")
    return q, p


def mean_sd(values):
    mean = fmean(values)
    sd = math.sqrt(fmean((value - mean) ** 2 for value in values))
    return mean, sd


def factor_controls(raw):
    q_duals = raw["dual_vertices"][: raw["k"]]
    p_duals = raw["dual_vertices"][raw["k"] :]

    def summaries(duals, offset):
        heights = []
        angles = []
        for dual in duals:
            norm = math.hypot(dual[offset], dual[offset + 1])
            heights.append(1.0 / norm)
            angles.append(math.atan2(dual[offset + 1], dual[offset]) % math.tau)
        angles.sort()
        gaps = [
            (angles[(i + 1) % len(angles)] - angles[i]) % math.tau
            for i in range(len(angles))
        ]
        logs = [math.log(height) for height in heights]
        _, sd = mean_sd(logs)
        return fmean(logs), sd, min(gaps)

    q_mean, q_sd, q_gap = summaries(q_duals, 0)
    p_mean, p_sd, p_gap = summaries(p_duals, 2)
    return {
        "q_mean_log_support": q_mean,
        "p_mean_log_support": p_mean,
        "q_sd_log_support": q_sd,
        "p_sd_log_support": p_sd,
        "q_min_angle_gap": q_gap,
        "p_min_angle_gap": p_gap,
    }


def solve(matrix, vector):
    n = len(matrix)
    augmented = [row[:] + [rhs] for row, rhs in zip(matrix, vector)]
    for column in range(n):
        pivot = max(range(column, n), key=lambda row: abs(augmented[row][column]))
        if abs(augmented[pivot][column]) < 1e-11:
            raise ValueError(f"singular design at column {column}")
        augmented[column], augmented[pivot] = augmented[pivot], augmented[column]
        divisor = augmented[column][column]
        augmented[column] = [value / divisor for value in augmented[column]]
        for row in range(n):
            if row == column:
                continue
            multiplier = augmented[row][column]
            augmented[row] = [
                value - multiplier * pivot_value
                for value, pivot_value in zip(augmented[row], augmented[column])
            ]
    return [augmented[i][-1] for i in range(n)]


def ols(y, x, names):
    size = len(names)
    xtx = [[0.0] * size for _ in range(size)]
    xty = [0.0] * size
    for outcome, row in zip(y, x):
        for j in range(size):
            xty[j] += row[j] * outcome
            for k in range(size):
                xtx[j][k] += row[j] * row[k]
    return dict(zip(names, solve(xtx, xty)))


def component_model(rows, controls):
    buckets = sorted({row["bucket"] for row in rows})
    standardizers = {}
    for control in controls:
        for bucket in buckets:
            values = [row[control] for row in rows if row["bucket"] == bucket]
            standardizers[(control, bucket)] = mean_sd(values)
    names = [f"bucket:{bucket}" for bucket in buckets] + ["bounce3"] + list(controls)
    design = []
    for item in rows:
        row = [float(item["bucket"] == bucket) for bucket in buckets] + [item["bounce3"]]
        for control in controls:
            mean, sd = standardizers[(control, item["bucket"])]
            row.append(0.0 if sd == 0 else (item[control] - mean) / sd)
        design.append(row)
    coefficients = {}
    for outcome in ("capacity_term", "width_term", "selection_term"):
        coefficients[outcome] = ols(
            [row[outcome] for row in rows], design, names
        )["bounce3"]
    return {
        "rows": len(rows),
        "controls": list(controls),
        "bounce3_coefficients": coefficients,
        "additivity_residual": coefficients["capacity_term"]
        - coefficients["width_term"]
        - coefficients["selection_term"],
        "width_fraction_of_capacity_coefficient": coefficients["width_term"]
        / coefficients["capacity_term"],
        "selection_fraction_of_capacity_coefficient": coefficients["selection_term"]
        / coefficients["capacity_term"],
    }


def ranks(values):
    order = sorted(range(len(values)), key=lambda i: values[i])
    result = [0.0] * len(values)
    start = 0
    while start < len(values):
        end = start + 1
        while end < len(values) and values[order[end]] == values[order[start]]:
            end += 1
        rank = (start + end - 1) / 2
        for index in order[start:end]:
            result[index] = rank
        start = end
    return result


def pearson(xs, ys):
    mean_x, mean_y = fmean(xs), fmean(ys)
    dx = sum((x - mean_x) ** 2 for x in xs)
    dy = sum((y - mean_y) ** 2 for y in ys)
    if dx == 0 or dy == 0:
        return None
    return sum((x - mean_x) * (y - mean_y) for x, y in zip(xs, ys)) / math.sqrt(dx * dy)


def spearman(xs, ys):
    return pearson(ranks(xs), ranks(ys))


def auc(high_values, low_values):
    if not high_values or not low_values:
        return None
    wins = 0.0
    for high in high_values:
        for low in low_values:
            wins += high > low
            wins += 0.5 * (high == low)
    return wins / (len(high_values) * len(low_values))


def association_summary(rows):
    by_bucket = {}
    for bucket in sorted({row["bucket"] for row in rows}):
        subset = [row for row in rows if row["bucket"] == bucket]
        two = [row for row in subset if not row["bounce3"]]
        three = [row for row in subset if row["bounce3"]]
        by_bucket[bucket] = {
            "rows": len(subset),
            "two_label_rows": len(two),
            "three_label_rows": len(three),
            "auc_width_for_three_label": auc(
                [row["width"] for row in three], [row["width"] for row in two]
            ),
            "auc_s2_for_three_label": auc(
                [row["s2"] for row in three], [row["s2"] for row in two]
            ),
            "spearman_s2_vs_stored_sys": spearman(
                [row["s2"] for row in subset], [row["stored_sys"] for row in subset]
            ),
            "mean_capacity_over_width_two_label": fmean(
                row["stored_capacity"] / row["width"] for row in two
            ) if two else None,
            "mean_capacity_over_width_three_label": fmean(
                row["stored_capacity"] / row["width"] for row in three
            ) if three else None,
        }
    return by_bucket


def load_prepared(table_path, provenance_path):
    if not table_path and not provenance_path:
        return {}
    if not table_path or not provenance_path:
        raise ValueError("--table and --provenance must be supplied together")
    for name, path in (("table", table_path), ("provenance", provenance_path)):
        actual = sha256(path)
        if actual != EXPECTED_SHA256[name]:
            raise ValueError(f"{name} SHA-256 mismatch: {actual}")
    table_rows = read_jsonl(table_path)
    table = {row["poly_id"]: row for row in table_rows}
    if len(table) != len(table_rows):
        raise ValueError("prepared poly_id is not unique")
    prepared = {}
    for row in read_jsonl(provenance_path):
        if row.get("role") == "random_product_sample":
            if row["source_name"] in prepared:
                raise ValueError("prepared source_name is not unique")
            prepared[row["source_name"]] = table[row["poly_id"]]
    return prepared


def analyze(args):
    actual_hashes = {"raw": sha256(args.raw), "classes": sha256(args.classes)}
    for name, actual in actual_hashes.items():
        if actual != EXPECTED_SHA256[name]:
            raise ValueError(f"{name} SHA-256 mismatch: {actual}")
    raw_rows = read_jsonl(args.raw)
    class_rows = read_jsonl(args.classes)
    raw_by_name = {row["name"]: row for row in raw_rows}
    class_by_name = {row["name"]: row for row in class_rows}
    if len(raw_by_name) != len(raw_rows) or len(class_by_name) != len(class_rows):
        raise ValueError("raw or class name is not unique")
    if set(raw_by_name) != set(class_by_name):
        raise ValueError("raw and class name sets differ")
    prepared = load_prepared(args.table, args.provenance)
    if prepared and set(prepared) != set(raw_by_name):
        raise ValueError("prepared and raw name sets differ")

    selected = []
    bucket_counts = Counter()
    for raw in raw_rows:
        bucket = f"{raw['k']}x{raw['m']}"
        if args.per_bucket_limit is not None and bucket_counts[bucket] >= args.per_bucket_limit:
            continue
        selected.append(raw)
        bucket_counts[bucket] += 1

    rows = []
    mismatches = []
    phenotype_counts = defaultdict(Counter)
    by_bucket = defaultdict(lambda: {"rows": 0, "matches": 0, "max_float_relative_error": 0.0})
    for raw in selected:
        classes = class_by_name[raw["name"]]
        expected_fields = {
            "k": raw["k"],
            "m": raw["m"],
            "producer_bounces": raw["bounces"],
            "stored_volume": raw["volume"],
            "stored_capacity": raw["capacity"],
            "stored_sys": raw["sys"],
        }
        if any(classes[key] != value for key, value in expected_fields.items()):
            raise ValueError(f"raw/class field mismatch: {raw['name']}")
        a2_exact = frac(classes["class_minima"]["2"]["action_exact"])
        if args.association_from_retained_a2:
            value = a2_exact
            witnesses = []
            match = None
        else:
            q_vertices, p_vertices = factor_vertices(raw)
            value, witnesses = two_bounce_width(q_vertices, p_vertices)
            match = value == a2_exact
            if not match and len(mismatches) < 20:
                mismatches.append(
                    {"name": raw["name"], "formula": str(value), "retained_a2": str(a2_exact)}
                )
        width_float = float(value)
        relative_error = abs(width_float - classes["class_minima"]["2"]["action"]) / width_float
        bucket = f"{raw['k']}x{raw['m']}"
        if not args.association_from_retained_a2:
            boundary = "edge_interior_present" if any(w["edge_interior"] for w in witnesses) else "vertices_only"
            norm_support = (
                "support_tie_present"
                if any(w["max_support_count"] > 1 or w["min_support_count"] > 1 for w in witnesses)
                else "unique_extrema_only"
            )
            phenotype = f"{boundary}+{norm_support}"
            phenotype_counts[bucket][phenotype] += 1
            by_bucket[bucket]["rows"] += 1
            by_bucket[bucket]["matches"] += match
            by_bucket[bucket]["max_float_relative_error"] = max(
                by_bucket[bucket]["max_float_relative_error"], relative_error
            )
        item = {
            "name": raw["name"],
            "bucket": bucket,
            "bounce3": float(raw["bounces"] == 3),
            "stored_capacity": raw["capacity"],
            "stored_volume": raw["volume"],
            "stored_sys": raw["sys"],
            "width": width_float,
            "s2": width_float * width_float / (2 * raw["volume"]),
            "capacity_term": 2 * math.log(raw["capacity"]),
            "width_term": 2 * math.log(width_float),
            "selection_term": 2 * math.log(raw["capacity"] / width_float),
        }
        item.update(factor_controls(raw))
        if prepared:
            item.update(prepared[raw["name"]])
        rows.append(item)

    adjustments = {
        "bucket_only": [],
        "generator_coordinates": [
            "q_mean_log_support", "p_mean_log_support",
            "q_sd_log_support", "p_sd_log_support",
            "q_min_angle_gap", "p_min_angle_gap",
        ],
    }
    if prepared:
        adjustments.update(
            {
                "ridge_distribution": [
                    "ridge_symp_area_normalized_entropy", "ridge_symp_area_max_share"
                ],
                "generator_plus_ridge_distribution": [
                    "q_mean_log_support", "p_mean_log_support",
                    "q_sd_log_support", "p_sd_log_support",
                    "q_min_angle_gap", "p_min_angle_gap",
                    "ridge_symp_area_normalized_entropy", "ridge_symp_area_max_share",
                ],
            }
        )
    return {
        "schema": "product-bounce-width-shortcut/v1",
        "formula": "min_{d in boundary(P-P)} h_{Q-Q}(d)",
        "formula_status": (
            "agent-derived exact proposition conditional on the repository's cited "
            "shortest-billiard/non-translatable-polygon interface"
        ),
        "inputs": {
            "raw": {"path": args.raw, "sha256": actual_hashes["raw"]},
            "classes": {"path": args.classes, "sha256": actual_hashes["classes"]},
            **(
                {
                    "table": {"path": args.table, "sha256": sha256(args.table)},
                    "provenance": {"path": args.provenance, "sha256": sha256(args.provenance)},
                }
                if prepared
                else {}
            ),
        },
        "selection": {
            "per_bucket_limit": args.per_bucket_limit,
            "rows": len(rows),
            "rows_by_bucket": dict(sorted(bucket_counts.items())),
        },
        "numeric_width_source": (
            "target-derived retained A2, used only for post-target association bookkeeping"
            if args.association_from_retained_a2
            else "independent exact difference-body computation from factor vertices"
        ),
        "analytic_fixtures": fixture_results(),
        "exact_validation": {
            "rows": 0 if args.association_from_retained_a2 else len(rows),
            "exact_matches": sum(bucket["matches"] for bucket in by_bucket.values()),
            "exact_mismatches": (
                0
                if args.association_from_retained_a2
                else len(rows) - sum(bucket["matches"] for bucket in by_bucket.values())
            ),
            "first_mismatches": mismatches,
            "by_bucket": dict(sorted(by_bucket.items())),
            "phenotypes_by_bucket": {
                bucket: dict(sorted(counts.items()))
                for bucket, counts in sorted(phenotype_counts.items())
            },
        },
        "capacity_component_decomposition": {
            name: component_model(rows, controls) for name, controls in adjustments.items()
        },
        "associations": association_summary(rows),
        "interpretation_boundary": {
            "independent": "W2 is computed from factor vertices without capacity targets or orbit words",
            "tautological": "s2 is defined from W2 and volume; its identity is not validation",
            "post_target": "producer bounce label and stored sys/capacity are target-derived outcomes",
            "selection_term": "capacity/W2 equals one for a two-bounce winner and A3/A2 for a three-bounce winner",
            "association_numeric_source": (
                "target-derived retained A2; association results are structural bookkeeping, "
                "not an independent width-feature validation or proposer test"
                if args.association_from_retained_a2
                else "independently computed W2 on the selected validation rows"
            ),
        },
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--fixtures", action="store_true")
    parser.add_argument("--raw")
    parser.add_argument("--classes")
    parser.add_argument("--table")
    parser.add_argument("--provenance")
    parser.add_argument("--per-bucket-limit", type=int)
    parser.add_argument("--association-from-retained-a2", action="store_true")
    parser.add_argument("--out")
    args = parser.parse_args()
    if args.fixtures and not args.raw:
        print(json.dumps({"analytic_fixtures": fixture_results()}, indent=2, sort_keys=True))
        return
    if not args.raw or not args.classes or not args.out:
        parser.error("--raw, --classes, and --out are required unless --fixtures is used alone")
    result = analyze(args)
    Path(args.out).parent.mkdir(parents=True, exist_ok=True)
    Path(args.out).write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
