#!/usr/bin/env sage -python
"""
Numerical sanity checks for the HKO feasible-section formula packet.

These checks are not proof evidence.  They compare selected exact derivative
rows against central finite differences in deterministic random directions to
catch sign, indexing, and normalization mistakes.

The coordinate order is (q1, q2, p1, p2).  The first five HKO facets are q-plane
facets and the last five are p-plane facets.  Full 40-dimensional directions
are used for beta/action checks.  Lagrangian-product-preserving directions are
used for volume/sys checks, because those have an independent cheap volume
oracle from planar polygon areas.
"""

from __future__ import annotations

import argparse
import json
import math
import random
import runpy
from pathlib import Path

from sage.all import RDF, matrix, vector


PACKET_DIR = Path(__file__).resolve().parent
VERIFY_PATH = PACKET_DIR / "verify_feasible_section_witness.sage.py"
WITNESS_PATH = PACKET_DIR / "feasible-section-witness.json"


def exact_to_float(x):
    return float(x)


def dot(row, direction):
    return sum(row[idx] * direction[idx] for idx in range(len(row)))


def relative_error(estimate, target):
    scale = max(1.0, abs(estimate), abs(target))
    return abs(estimate - target) / scale


def unflatten_direction(direction):
    return [direction[4 * idx : 4 * idx + 4] for idx in range(10)]


def add_scaled_direction(duals, direction, scale):
    pieces = unflatten_direction(direction)
    return [
        [duals[facet][coord] + scale * pieces[facet][coord] for coord in range(4)]
        for facet in range(len(duals))
    ]


def omega_float(a, b):
    return a[0] * b[2] + a[1] * b[3] - a[2] * b[0] - a[3] * b[1]


def q_value_float(duals, sigma, beta):
    total = 0.0
    for i in range(1, len(sigma)):
        for j in range(i):
            total += beta[i] * beta[j] * omega_float(duals[sigma[j]], duals[sigma[i]])
    return total


def constraint_matrix_float(duals, sigma):
    return matrix(
        RDF,
        [[duals[facet][row] for facet in sigma] for row in range(4)]
        + [[1.0 for _ in sigma]],
    )


def beta_section_float(duals, row, beta0):
    sigma = row["sigma"]
    minor_columns = row["minor_columns"]
    fixed_indices = row["fixed_beta_indices"]
    C = constraint_matrix_float(duals, sigma)
    e = vector(RDF, [0.0, 0.0, 0.0, 0.0, 1.0])
    beta = [0.0 for _ in sigma]
    fixed_values = vector(RDF, [beta0[idx] for idx in fixed_indices])
    rhs = e
    if fixed_indices:
        rhs = rhs - C[:, fixed_indices] * fixed_values
    beta_I = C[:, minor_columns].solve_right(rhs)
    for local_idx, beta_idx in enumerate(minor_columns):
        beta[beta_idx] = float(beta_I[local_idx])
    for beta_idx in fixed_indices:
        beta[beta_idx] = beta0[beta_idx]
    return beta


def branch_values(duals, row, beta0):
    beta = beta_section_float(duals, row, beta0)
    q = q_value_float(duals, row["sigma"], beta)
    action = 1.0 / (2.0 * q)
    return beta, action


def area_from_halfspaces(normals):
    vertices = []
    for i in range(len(normals)):
        for j in range(i + 1, len(normals)):
            a = normals[i]
            b = normals[j]
            det = a[0] * b[1] - a[1] * b[0]
            if abs(det) < 1e-12:
                continue
            x = [(b[1] - a[1]) / det, (a[0] - b[0]) / det]
            if all(n[0] * x[0] + n[1] * x[1] <= 1.0 + 1e-9 for n in normals):
                if not any(math.hypot(x[0] - y[0], x[1] - y[1]) < 1e-8 for y in vertices):
                    vertices.append(x)
    if len(vertices) < 3:
        raise ValueError("area check found fewer than three vertices")
    cx = sum(v[0] for v in vertices) / len(vertices)
    cy = sum(v[1] for v in vertices) / len(vertices)
    vertices.sort(key=lambda v: math.atan2(v[1] - cy, v[0] - cx))
    total = 0.0
    for idx, v in enumerate(vertices):
        w = vertices[(idx + 1) % len(vertices)]
        total += v[0] * w[1] - v[1] * w[0]
    return abs(total) / 2.0


def lagrangian_product_volume(duals):
    q_normals = [[duals[idx][0], duals[idx][1]] for idx in range(5)]
    p_normals = [[duals[idx][2], duals[idx][3]] for idx in range(5, 10)]
    return area_from_halfspaces(q_normals) * area_from_halfspaces(p_normals)


def deterministic_lagrangian_direction(rng):
    direction = []
    for facet in range(10):
        for coord in range(4):
            if facet < 5 and coord >= 2:
                direction.append(0.0)
            elif facet >= 5 and coord < 2:
                direction.append(0.0)
            else:
                direction.append(rng.gauss(0.0, 1.0))
    norm = math.sqrt(sum(entry * entry for entry in direction))
    return [entry / norm for entry in direction]


def deterministic_full_direction(rng):
    direction = [rng.gauss(0.0, 1.0) for _idx in range(40)]
    norm = math.sqrt(sum(entry * entry for entry in direction))
    return [entry / norm for entry in direction]


def load_packet():
    verifier = runpy.run_path(str(VERIFY_PATH))
    witness = json.loads(WITNESS_PATH.read_text())
    K = verifier["number_field_from_witness"](witness["field"])
    sqrt5, exact_duals = verifier["exact_hko_geometry"](K)
    volume, volume_row = verifier["volume_data"](K, sqrt5, exact_duals)
    symmetry_labels, symmetry_columns, _sp4_checks = verifier["symmetry_basis"](K, exact_duals)
    return verifier, witness, exact_duals, volume, volume_row, symmetry_labels, symmetry_columns


def parse_row(verifier, K, row):
    beta0 = [exact_to_float(entry) for entry in verifier["vector_from_json"](K, row["beta0_power_basis"])]
    d_beta = [
        [exact_to_float(entry) for entry in beta_row]
        for beta_row in verifier["matrix_rows_from_json"](K, row["d_beta_power_basis"])
    ]
    d_action = [
        exact_to_float(entry)
        for entry in verifier["vector_from_json"](K, row["d_action_flat_power_basis"])
    ]
    d_sys = [
        exact_to_float(entry)
        for entry in verifier["vector_from_json"](K, row["d_sys_flat_power_basis"])
    ]
    return beta0, d_beta, d_action, d_sys


def update_maxima(maxima, key, estimate, target):
    maxima[f"{key}_abs"] = max(maxima[f"{key}_abs"], abs(estimate - target))
    maxima[f"{key}_rel"] = max(maxima[f"{key}_rel"], relative_error(estimate, target))


def update_best(best, keys, maxima):
    for key in keys:
        best[f"{key}_abs"] = min(best[f"{key}_abs"], maxima[f"{key}_abs"])
        best[f"{key}_rel"] = min(best[f"{key}_rel"], maxima[f"{key}_rel"])


def make_maxima(keys, initial):
    maxima = {}
    for key in keys:
        maxima[f"{key}_abs"] = initial
        maxima[f"{key}_rel"] = initial
    return maxima


def check_args(args, row_count):
    if args.rows < 1:
        raise ValueError("--rows must be positive")
    if args.rows > row_count:
        raise ValueError(f"--rows must be at most {row_count}")
    if args.directions < 1:
        raise ValueError("--directions must be positive")
    if args.eps_min_power > args.eps_max_power:
        raise ValueError("--eps-min-power must be at most --eps-max-power")
    if args.fail_threshold <= 0:
        raise ValueError("--fail-threshold must be positive")
    if args.symmetry_threshold <= 0:
        raise ValueError("--symmetry-threshold must be positive")


def run(args):
    verifier, witness, exact_duals, volume, volume_row, symmetry_labels, symmetry_columns = load_packet()
    check_args(args, len(witness["rows"]))
    K = verifier["number_field_from_witness"](witness["field"])
    base_duals = [[exact_to_float(entry) for entry in dual] for dual in exact_duals]
    volume_row_float = [exact_to_float(entry) for entry in volume_row]
    rows = witness["rows"][: args.rows]
    eps_values = [10.0 ** (-power) for power in range(args.eps_min_power, args.eps_max_power + 1)]
    rng = random.Random(args.seed)
    full_directions = [deterministic_full_direction(rng) for _idx in range(args.directions)]
    lagrangian_directions = [
        deterministic_lagrangian_direction(rng) for _idx in range(args.directions)
    ]

    print("# HKO formula finite-difference sanity check")
    print()
    print("Status: sanity-check evidence only, not part of the proof.")
    print(f"Rows checked: {len(rows)} of {len(witness['rows'])}")
    print(f"Full 40-dimensional directions checked: {args.directions}")
    print(f"Lagrangian-product-preserving directions checked: {args.directions}")
    print(f"Epsilon sweep: {', '.join(f'{eps:.0e}' for eps in eps_values)}")
    print("Full directions check beta/action derivatives.")
    print("Lagrangian-product-preserving directions check beta/action/sys/volume.")
    print("Volume is checked by central finite differences of the elementary")
    print("product-of-planar-areas formula, not by a general 4D volume backend.")
    print()
    print("## Full 40-dimensional beta/action checks")
    print()
    print("| eps | max beta abs | max beta rel | max action abs | max action rel |")
    print("| --- | ---: | ---: | ---: | ---: |")
    full_best = make_maxima(["beta", "action"], float("inf"))
    for eps in eps_values:
        maxima = make_maxima(["beta", "action"], 0.0)
        for direction in full_directions:
            plus_duals = add_scaled_direction(base_duals, direction, eps)
            minus_duals = add_scaled_direction(base_duals, direction, -eps)
            for row in rows:
                beta0, d_beta, d_action, _d_sys = parse_row(verifier, K, row)
                beta_plus, action_plus = branch_values(plus_duals, row, beta0)
                beta_minus, action_minus = branch_values(minus_duals, row, beta0)
                for beta_idx in range(len(beta0)):
                    beta_fd = (beta_plus[beta_idx] - beta_minus[beta_idx]) / (2.0 * eps)
                    beta_linear = dot(d_beta[beta_idx], direction)
                    update_maxima(maxima, "beta", beta_fd, beta_linear)

                action_fd = (action_plus - action_minus) / (2.0 * eps)
                action_linear = dot(d_action, direction)
                update_maxima(maxima, "action", action_fd, action_linear)

        update_best(full_best, ["beta", "action"], maxima)
        print(
            f"| {eps:.0e} | "
            f"{maxima['beta_abs']:.3e} | {maxima['beta_rel']:.3e} | "
            f"{maxima['action_abs']:.3e} | {maxima['action_rel']:.3e} |"
        )

    print()
    print("## Lagrangian-product beta/action/sys/volume checks")
    print()
    print("| eps | max beta abs | max beta rel | max action abs | max action rel | max sys abs | max sys rel | max volume abs | max volume rel |")
    print("| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |")
    lagrangian_best = make_maxima(["beta", "action", "sys", "volume"], float("inf"))
    for eps in eps_values:
        maxima = make_maxima(["beta", "action", "sys", "volume"], 0.0)
        for direction in lagrangian_directions:
            plus_duals = add_scaled_direction(base_duals, direction, eps)
            minus_duals = add_scaled_direction(base_duals, direction, -eps)

            volume_plus = lagrangian_product_volume(plus_duals)
            volume_minus = lagrangian_product_volume(minus_duals)
            volume_fd = (volume_plus - volume_minus) / (2.0 * eps)
            volume_linear = dot(volume_row_float, direction)
            update_maxima(maxima, "volume", volume_fd, volume_linear)

            for row in rows:
                beta0, d_beta, d_action, d_sys = parse_row(verifier, K, row)
                beta_plus, action_plus = branch_values(plus_duals, row, beta0)
                beta_minus, action_minus = branch_values(minus_duals, row, beta0)
                for beta_idx in range(len(beta0)):
                    beta_fd = (beta_plus[beta_idx] - beta_minus[beta_idx]) / (2.0 * eps)
                    beta_linear = dot(d_beta[beta_idx], direction)
                    update_maxima(maxima, "beta", beta_fd, beta_linear)

                action_fd = (action_plus - action_minus) / (2.0 * eps)
                action_linear = dot(d_action, direction)
                update_maxima(maxima, "action", action_fd, action_linear)

                sys_plus = action_plus * action_plus / (2.0 * volume_plus)
                sys_minus = action_minus * action_minus / (2.0 * volume_minus)
                sys_fd = (sys_plus - sys_minus) / (2.0 * eps)
                sys_linear = dot(d_sys, direction)
                update_maxima(maxima, "sys", sys_fd, sys_linear)

        update_best(lagrangian_best, ["beta", "action", "sys", "volume"], maxima)
        print(
            f"| {eps:.0e} | "
            f"{maxima['beta_abs']:.3e} | {maxima['beta_rel']:.3e} | "
            f"{maxima['action_abs']:.3e} | {maxima['action_rel']:.3e} | "
            f"{maxima['sys_abs']:.3e} | {maxima['sys_rel']:.3e} | "
            f"{maxima['volume_abs']:.3e} | {maxima['volume_rel']:.3e} |"
        )

    symmetry_columns_float = [
        [exact_to_float(entry) for entry in column] for column in symmetry_columns
    ]
    max_symmetry_dot = 0.0
    for row in rows:
        _beta0, _d_beta, _d_action, d_sys = parse_row(verifier, K, row)
        for column in symmetry_columns_float:
            max_symmetry_dot = max(max_symmetry_dot, abs(dot(d_sys, column)))
    print()
    print(f"Max |D sys row * symmetry tangent| over checked rows: {max_symmetry_dot:.3e}")
    print()
    print("Interpretation: for a correct first derivative, central-difference errors")
    print("should decrease as eps shrinks until floating-point noise dominates.")
    print("Large errors at all eps values are a formula/indexing bug signal.")

    finite_difference_rel = [
        full_best["beta_rel"],
        full_best["action_rel"],
        lagrangian_best["beta_rel"],
        lagrangian_best["action_rel"],
        lagrangian_best["sys_rel"],
        lagrangian_best["volume_rel"],
    ]
    worst_best_rel = max(finite_difference_rel)
    if worst_best_rel > args.fail_threshold:
        raise SystemExit(
            "finite-difference sanity check failed: "
            f"best relative error {worst_best_rel:.3e} exceeds {args.fail_threshold:.3e}"
        )
    if max_symmetry_dot > args.symmetry_threshold:
        raise SystemExit(
            "symmetry annihilation sanity check failed: "
            f"{max_symmetry_dot:.3e} exceeds {args.symmetry_threshold:.3e}"
        )


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--rows", type=int, default=26, help="number of witness rows to check")
    parser.add_argument("--directions", type=int, default=3, help="random directions per epsilon")
    parser.add_argument("--seed", type=int, default=20260605, help="deterministic random seed")
    parser.add_argument("--eps-min-power", type=int, default=2, help="largest epsilon is 10^-this")
    parser.add_argument("--eps-max-power", type=int, default=7, help="smallest epsilon is 10^-this")
    parser.add_argument(
        "--fail-threshold",
        type=float,
        default=1e-4,
        help="maximum allowed best relative finite-difference error",
    )
    parser.add_argument(
        "--symmetry-threshold",
        type=float,
        default=1e-10,
        help="maximum allowed absolute D sys dot symmetry tangent value",
    )
    args = parser.parse_args()
    try:
        run(args)
    except ValueError as exc:
        parser.error(str(exc))


if __name__ == "__main__":
    main()
