#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# ///

"""Count the HKO2024 billiard sigma surface at each pruning rung.

Goal: record the scale of the finite candidate surfaces relevant to the exact
      Clarke route, distinguishing raw billiard block words, directed-feasible
      sigma words, current valid KKT orbits, and current exact minima.
Input Artifacts: experiments/hko-local-maximum/gradient-analysis/hko-neighborhood-sensitivity.jsonl
Output Artifacts: experiments/hko-local-maximum/exact-clarke/billiard-sigma-counts.json
"""

from __future__ import annotations

import json
from itertools import permutations
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
EXPERIMENT_DIR = Path(__file__).resolve().parent
INPUT_PATH = (
    ROOT
    / "experiments"
    / "hko-local-maximum"
    / "gradient-analysis"
    / "hko-neighborhood-sensitivity.jsonl"
)
OUTPUT_PATH = EXPERIMENT_DIR / "billiard-sigma-counts.json"


def omega(lhs: tuple[float, float, float, float], rhs: tuple[float, float, float, float]) -> float:
    return lhs[0] * rhs[2] + lhs[1] * rhs[3] - lhs[2] * rhs[0] - lhs[3] * rhs[1]


def sign(value: float, eps: float = 1e-12) -> int:
    if value > eps:
        return 1
    if value < -eps:
        return -1
    return 0


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


def feasible_cycle(sigma: list[int], directed_adj: list[list[bool]]) -> bool:
    return all(directed_adj[sigma[idx]][sigma[(idx + 1) % len(sigma)]] for idx in range(len(sigma)))


def build_counts() -> dict[str, object]:
    row = json.loads(INPUT_PATH.read_text().splitlines()[0])
    dual_vertices = [tuple(vertex) for vertex in row["dual_vertices"]]

    # HKO is a pentagon x pentagon Lagrangian product with q-facets 0..4 and
    # p-facets 5..9. Undirected adjacency is pentagon adjacency within each
    # block plus complete q/p incidence.
    facet_count = len(dual_vertices)
    vertex_adj = [[False] * facet_count for _ in range(facet_count)]
    for index in range(5):
        next_index = (index + 1) % 5
        vertex_adj[index][next_index] = True
        vertex_adj[next_index][index] = True
    for index in range(5, 10):
        next_index = 5 + ((index - 5 + 1) % 5)
        vertex_adj[index][next_index] = True
        vertex_adj[next_index][index] = True
    for q_index in range(5):
        for p_index in range(5, 10):
            vertex_adj[q_index][p_index] = True
            vertex_adj[p_index][q_index] = True

    directed_adj = [[False] * facet_count for _ in range(facet_count)]
    for lhs in range(facet_count):
        for rhs in range(facet_count):
            directed_adj[lhs][rhs] = (
                lhs != rhs
                and vertex_adj[lhs][rhs]
                and sign(omega(dual_vertices[lhs], dual_vertices[rhs])) >= 0
            )

    q_blocks = [(idx,) for idx in range(5)]
    p_blocks = [(idx,) for idx in range(5, 10)]
    for index in range(5):
        next_index = (index + 1) % 5
        q_blocks.append((index, next_index))
        q_blocks.append((next_index, index))
        p_blocks.append((index + 5, next_index + 5))
        p_blocks.append((next_index + 5, index + 5))

    k_payload: dict[str, object] = {}
    raw_total = 0
    directed_total = 0

    for k in (2, 3):
        q_selections = non_overlapping_selections(q_blocks, k)
        p_selections = non_overlapping_selections(p_blocks, k)
        q_permutations = 1
        for value in range(2, k):
            q_permutations *= value
        p_permutations = 1
        for value in range(2, k + 1):
            p_permutations *= value

        raw_count = len(q_selections) * len(p_selections) * q_permutations * p_permutations
        directed_feasible = 0
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
                        if feasible_cycle(sigma, directed_adj):
                            directed_feasible += 1

        k_payload[str(k)] = {
            "q_block_selections": len(q_selections),
            "p_block_selections": len(p_selections),
            "q_permutations_after_cyclic_fix": q_permutations,
            "p_permutations": p_permutations,
            "raw_block_words": raw_count,
            "directed_feasible_sigma_words": directed_feasible,
            "formula": (
                f"{len(q_selections)} * {len(p_selections)} * "
                f"{q_permutations} * {p_permutations}"
            ),
        }
        raw_total += raw_count
        directed_total += directed_feasible

    current_near_optimal = row["orbits"]
    exact_minima = [orbit for orbit in current_near_optimal if abs(orbit["relative_gap"]) < 1e-12]

    return {
        "input_artifact": str(INPUT_PATH.relative_to(ROOT)),
        "theorem_question": (
            "How large is the finite billiard/HK2017 candidate surface for HKO2024 "
            "before and after the current pruning rungs?"
        ),
        "facet_split": {
            "q_facets": [0, 1, 2, 3, 4],
            "p_facets": [5, 6, 7, 8, 9],
        },
        "block_structure": {
            "word_shape": "([Q|QQ][P|PP])^k",
            "bounce_bound": "k <= 3",
            "single_blocks_per_side": 5,
            "ordered_adjacent_pair_blocks_per_side": 10,
            "total_blocks_per_side": 15,
        },
        "per_k_counts": k_payload,
        "count_ladder": {
            "raw_billiard_block_words": raw_total,
            "directed_feasible_sigma_words": directed_total,
            "current_valid_kkt_orbits": row["n_valid_orbits"],
            "current_near_optimal_orbits": row["n_near_optimal"],
            "current_exact_minima": len(exact_minima),
        },
        "scaling_vs_exact_minima": {
            "raw_billiard_block_words_over_exact_minima": raw_total / len(exact_minima),
            "directed_feasible_sigma_words_over_exact_minima": directed_total / len(exact_minima),
            "current_valid_kkt_orbits_over_exact_minima": row["n_valid_orbits"] / len(exact_minima),
        },
        "interpretation": (
            "The already-valid KKT surface is only a small multiple of the exact minima, "
            "but the theorem-native billiard sigma-word surface is much larger. "
            "An exhaustive exact route that starts from block-structured billiard words "
            "therefore pays a substantially larger front-end certification cost than a route "
            "that starts from the already-valid orbit surface."
        ),
    }


def main() -> None:
    payload = build_counts()
    OUTPUT_PATH.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    print(f"Wrote {OUTPUT_PATH.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
