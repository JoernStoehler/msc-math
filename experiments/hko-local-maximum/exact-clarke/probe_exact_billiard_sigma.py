#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["sympy"]
# ///

"""Probe exact quartic KKT solve cost on the HKO billiard sigma surface.

Goal: estimate whether an exact route that starts from all directed-feasible
      billiard sigma words for HKO2024 (`6240` at the current pruning rung)
      looks computationally feasible in the current sympy-based environment.
Input Artifacts: experiments/hko-local-maximum/exact-clarke/billiard-sigma-counts.json
Output Artifacts: experiments/hko-local-maximum/exact-clarke/billiard-exact-probe.json
"""

from __future__ import annotations

import argparse
import json
import random
import time
from itertools import permutations
from pathlib import Path

from sympy import Matrix, Rational, linsolve, simplify, sqrt, symbols


ROOT = Path(__file__).resolve().parents[3]
EXPERIMENT_DIR = Path(__file__).resolve().parent
COUNTS_PATH = EXPERIMENT_DIR / "billiard-sigma-counts.json"
OUTPUT_PATH = EXPERIMENT_DIR / "billiard-exact-probe.json"


def overlap(lhs: tuple[int, ...], rhs: tuple[int, ...]) -> bool:
    return bool(set(lhs) & set(rhs))


def non_overlapping_selections(blocks: list[tuple[int, ...]], k: int) -> list[tuple[tuple[int, ...], ...]]:
    selections: list[tuple[tuple[int, ...], ...]] = []
    current: list[tuple[int, ...]] = []

    def rec(start: int) -> None:
        if len(current) == k:
            selections.append(tuple(current))
            return
        for index in range(start, len(blocks)):
            block = blocks[index]
            if any(overlap(block, chosen) for chosen in current):
                continue
            current.append(block)
            rec(index + 1)
            current.pop()

    rec(0)
    return selections


def exact_hko_duals() -> list[Matrix]:
    t = sqrt(5 - 2 * sqrt(5))
    sqrt5 = (5 - t**2) / 2
    alpha = (3 - sqrt5) / 2
    beta = t * (1 + sqrt5) / 2
    sec36 = sqrt5 - 1
    return [
        Matrix([1, t, 0, 0]),
        Matrix([-alpha, beta, 0, 0]),
        Matrix([-sec36, 0, 0, 0]),
        Matrix([-alpha, -beta, 0, 0]),
        Matrix([1, -t, 0, 0]),
        Matrix([0, 0, t, -1]),
        Matrix([0, 0, beta, alpha]),
        Matrix([0, 0, 0, sec36]),
        Matrix([0, 0, -beta, alpha]),
        Matrix([0, 0, -t, -1]),
    ]


def omega(lhs: Matrix, rhs: Matrix):
    j0 = Matrix([[0, 0, -1, 0], [0, 0, 0, -1], [1, 0, 0, 0], [0, 1, 0, 0]])
    return simplify((lhs.T * j0 * rhs)[0])


def sign(expr) -> int:
    numeric = float(expr.evalf(50))
    if numeric > 1e-12:
        return 1
    if numeric < -1e-12:
        return -1
    return 0


def directed_feasible_sigmas() -> list[list[int]]:
    duals = exact_hko_duals()

    facet_count = len(duals)
    facet_intersection_is_nonempty = [[False] * facet_count for _ in range(facet_count)]
    for index in range(5):
        next_index = (index + 1) % 5
        facet_intersection_is_nonempty[index][next_index] = True
        facet_intersection_is_nonempty[next_index][index] = True
    for index in range(5, 10):
        next_index = 5 + ((index - 5 + 1) % 5)
        facet_intersection_is_nonempty[index][next_index] = True
        facet_intersection_is_nonempty[next_index][index] = True
    for q_index in range(5):
        for p_index in range(5, 10):
            facet_intersection_is_nonempty[q_index][p_index] = True
            facet_intersection_is_nonempty[p_index][q_index] = True

    directed_transition_is_allowed = [[False] * facet_count for _ in range(facet_count)]
    for lhs in range(facet_count):
        for rhs in range(facet_count):
            directed_transition_is_allowed[lhs][rhs] = (
                lhs != rhs
                and facet_intersection_is_nonempty[lhs][rhs]
                and sign(omega(duals[lhs], duals[rhs])) >= 0
            )

    q_blocks = [(idx,) for idx in range(5)]
    p_blocks = [(idx,) for idx in range(5, 10)]
    for index in range(5):
        next_index = (index + 1) % 5
        q_blocks.append((index, next_index))
        q_blocks.append((next_index, index))
        p_blocks.append((index + 5, next_index + 5))
        p_blocks.append((next_index + 5, index + 5))

    sigmas: list[list[int]] = []
    for k in (2, 3):
        q_selections = non_overlapping_selections(q_blocks, k)
        p_selections = non_overlapping_selections(p_blocks, k)
        for q_selection in q_selections:
            for p_selection in p_selections:
                for q_perm in permutations(range(k - 1)):
                    for p_perm in permutations(range(k)):
                        sigma: list[int] = []
                        sigma.extend(q_selection[0])
                        sigma.extend(p_selection[p_perm[0]])
                        for round_index in range(1, k):
                            sigma.extend(q_selection[1 + q_perm[round_index - 1]])
                            sigma.extend(p_selection[p_perm[round_index]])
                        if all(
                            directed_transition_is_allowed[sigma[idx]][
                                sigma[(idx + 1) % len(sigma)]
                            ]
                            for idx in range(len(sigma))
                        ):
                            sigmas.append(sigma)
    return sigmas


def build_kkt_matrix(duals: list[Matrix], sigma: list[int]) -> tuple[Matrix, Matrix]:
    m = len(sigma)
    h_matrix = Matrix.zeros(m, m)
    for i in range(m):
        for j in range(i + 1, m):
            value = omega(duals[sigma[i]], duals[sigma[j]])
            h_matrix[i, j] = value
            h_matrix[j, i] = value

    a_block = Matrix([[duals[facet][coord] for coord in range(4)] for facet in sigma])
    ones = Matrix.ones(m, 1)
    top = h_matrix.row_join(a_block).row_join(ones)
    middle = a_block.T.row_join(Matrix.zeros(4, 4)).row_join(Matrix.zeros(4, 1))
    bottom = Matrix.ones(1, m).row_join(Matrix.zeros(1, 4)).row_join(Matrix.zeros(1, 1))
    matrix = top.col_join(middle).col_join(bottom)
    rhs = Matrix([0] * m + [0, 0, 0, 0, 1])
    return matrix, rhs


def probe_sigma(duals: list[Matrix], sigma: list[int]) -> dict[str, object]:
    matrix, rhs = build_kkt_matrix(duals, sigma)
    start = time.perf_counter()
    solve_kind = "unique_lu"
    free_parameter_count = 0
    beta_min_float = None
    q_float = None

    try:
        solution = matrix.LUsolve(rhs)
    except Exception:
        solve_kind = "singular_linsolve"
        variables = symbols(f"x0:{len(sigma) + 5}")
        solution = next(iter(linsolve((matrix, rhs), variables)))
        free_symbols = set().union(*(entry.free_symbols for entry in solution))
        free_parameter_count = len(free_symbols)

    elapsed_s = time.perf_counter() - start

    if free_parameter_count == 0:
        beta_entries = [simplify(entry) for entry in solution[: len(sigma)]]
        beta_min_float = min(float(entry.evalf(50)) for entry in beta_entries)
        h_matrix = matrix[: len(sigma), : len(sigma)]
        beta_column = Matrix(beta_entries)
        q_value = simplify(Rational(1, 2) * (beta_column.T * h_matrix * beta_column)[0])
        q_float = float(q_value.evalf(50))

    return {
        "sigma": sigma,
        "length": len(sigma),
        "solve_kind": solve_kind,
        "free_parameter_count": free_parameter_count,
        "elapsed_s": elapsed_s,
        "beta_min_float": beta_min_float,
        "q_float": q_float,
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--limit", type=int, default=200)
    parser.add_argument("--seed", type=int, default=0)
    args = parser.parse_args()

    counts_payload = json.loads(COUNTS_PATH.read_text())
    total_directed = counts_payload["count_ladder"]["directed_feasible_sigma_words"]

    duals = exact_hko_duals()
    sigmas = directed_feasible_sigmas()
    if len(sigmas) != total_directed:
        raise RuntimeError(
            f"directed sigma count mismatch: generated {len(sigmas)} vs recorded {total_directed}"
        )

    limit = min(args.limit, len(sigmas))
    sampled_sigmas = random.Random(args.seed).sample(sigmas, limit)

    total_start = time.perf_counter()
    rows = [probe_sigma(duals, sigma) for sigma in sampled_sigmas]
    total_elapsed_s = time.perf_counter() - total_start

    unique_rows = [row for row in rows if row["free_parameter_count"] == 0]
    singular_rows = [row for row in rows if row["free_parameter_count"] > 0]
    positive_unique = [
        row for row in unique_rows if row["beta_min_float"] is not None and row["beta_min_float"] > 1e-12
    ]

    payload = {
        "input_artifact": str(COUNTS_PATH.relative_to(ROOT)),
        "sample_limit": limit,
        "sample_seed": args.seed,
        "total_directed_feasible_sigma_words": len(sigmas),
        "total_elapsed_s": total_elapsed_s,
        "mean_elapsed_s_per_sigma": total_elapsed_s / limit,
        "projected_elapsed_s_for_full_6240": total_elapsed_s * len(sigmas) / limit,
        "solve_kind_counts": {
            "unique_lu": sum(row["solve_kind"] == "unique_lu" for row in rows),
            "singular_linsolve": sum(row["solve_kind"] == "singular_linsolve" for row in rows),
        },
        "free_parameter_histogram": {
            str(count): sum(row["free_parameter_count"] == count for row in rows)
            for count in sorted({row["free_parameter_count"] for row in rows})
        },
        "positive_unique_count": len(positive_unique),
        "sample_rows": rows[: min(25, len(rows))],
        "interpretation": (
            "This is a probe of exact quartic linear/KKT solve cost in the current "
            "sympy environment, not yet a theorem artifact. The projected full-6240 "
            "time estimates whether a direct exhaustive exact sigma route is practical "
            "before adding exact positivity and action-comparison bookkeeping."
        ),
    }

    OUTPUT_PATH.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    print(f"Wrote {OUTPUT_PATH.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
