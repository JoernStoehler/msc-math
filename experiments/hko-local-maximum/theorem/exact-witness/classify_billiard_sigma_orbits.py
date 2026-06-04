#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["sympy"]
# ///

"""Quotient the directed-feasible HKO billiard sigma surface by HKO symmetries.

Goal: measure how many directed-feasible HKO sigma words remain after
      quotienting by the order-10 HKO symplectic symmetry group and cyclic
      relabeling of the same cyclic word.
Input Artifacts: experiments/hko-local-maximum/theorem/exact-witness/billiard-sigma-counts.json
Output Artifacts: experiments/hko-local-maximum/theorem/exact-witness/billiard-sigma-orbits.json
"""

from __future__ import annotations

import json
from itertools import permutations
from pathlib import Path

from sympy import Matrix, simplify, sqrt


ROOT = Path(__file__).resolve().parents[4]
EXPERIMENT_DIR = Path(__file__).resolve().parent
COUNTS_PATH = EXPERIMENT_DIR / "billiard-sigma-counts.json"
OUTPUT_PATH = EXPERIMENT_DIR / "billiard-sigma-orbits.json"


DELTA = {0: 1, 1: 2, 2: 3, 3: 4, 4: 0, 5: 6, 6: 7, 7: 8, 8: 9, 9: 5}
PHI = {0: 5, 1: 6, 2: 7, 3: 8, 4: 9, 5: 0, 6: 1, 7: 2, 8: 3, 9: 4}


def compose_facet_maps(left: dict[int, int], right: dict[int, int]) -> dict[int, int]:
    return {facet: left[right[facet]] for facet in range(10)}


def symmetry_group() -> list[dict[int, int]]:
    group = []
    current = {facet: facet for facet in range(10)}
    for power in range(5):
        if power == 0:
            current = {facet: facet for facet in range(10)}
        elif power == 1:
            current = DELTA
        else:
            current = compose_facet_maps(DELTA, current)
        group.append(current)
        group.append(compose_facet_maps(PHI, current))
    return group


GROUP = symmetry_group()


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


def canonical_cyclic_word(word: list[int]) -> tuple[int, ...]:
    return min(tuple(word[index:] + word[:index]) for index in range(len(word)))


def sigma_orbit(word: list[int]) -> set[tuple[int, ...]]:
    orbit = set()
    for facet_map in GROUP:
        transformed = [facet_map[facet] for facet in word]
        orbit.add(canonical_cyclic_word(transformed))
    return orbit


def directed_feasible_sigmas() -> dict[int, list[list[int]]]:
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

    sigmas_by_k: dict[int, list[list[int]]] = {2: [], 3: []}
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
                            sigmas_by_k[k].append(sigma)
    return sigmas_by_k


def classify_sigma_words(words: list[list[int]]) -> list[dict[str, object]]:
    canonical_to_word = {
        canonical_cyclic_word(word): word
        for word in words
    }
    unseen = set(canonical_to_word)
    orbits = []
    while unseen:
        representative = min(unseen)
        orbit_keys = sigma_orbit(list(representative))
        members = sorted(key for key in canonical_to_word if key in orbit_keys)
        for member in members:
            unseen.discard(member)
        orbit_lengths = sorted({len(member) for member in members})
        if len(orbit_lengths) != 1:
            raise ValueError(f"mixed word lengths in one orbit: {orbit_lengths}")
        orbits.append(
            {
                "canonical_representative": list(representative),
                "word_length": orbit_lengths[0],
                "n_members": len(members),
                "members": [list(member) for member in members],
            }
        )
    return sorted(orbits, key=lambda entry: (entry["word_length"], entry["n_members"], entry["canonical_representative"]))


def main() -> None:
    counts = json.loads(COUNTS_PATH.read_text())
    sigmas_by_k = directed_feasible_sigmas()

    total_directed = sum(len(words) for words in sigmas_by_k.values())
    expected_directed = counts["count_ladder"]["directed_feasible_sigma_words"]
    if total_directed != expected_directed:
        raise RuntimeError(
            f"directed sigma count mismatch: generated {total_directed} vs recorded {expected_directed}"
        )

    orbits_by_k = {
        str(k): classify_sigma_words(words)
        for k, words in sigmas_by_k.items()
    }
    total_orbits = sum(len(orbits) for orbits in orbits_by_k.values())

    payload = {
        "input_artifact": COUNTS_PATH.name,
        "equivalence_relation": (
            "Directed-feasible sigma words are quotiented by cyclic rotation of "
            "the same cyclic word and by the order-10 HKO symplectic symmetry "
            "group generated by diagonal 72-degree rotation and q/p exchange."
        ),
        "symmetry_generators": {
            "delta": DELTA,
            "phi": PHI,
        },
        "count_summary": {
            "directed_feasible_sigma_words": total_directed,
            "directed_feasible_sigma_orbits": total_orbits,
            "scaling_factor_after_quotient": total_directed / total_orbits,
            "per_k": {
                str(k): {
                    "directed_feasible_sigma_words": len(sigmas_by_k[k]),
                    "directed_feasible_sigma_orbits": len(orbits_by_k[str(k)]),
                }
                for k in (2, 3)
            },
        },
        "sigma_orbits_per_k": orbits_by_k,
        "theorem_use": (
            "This artifact records the size of the directed-feasible sigma "
            "surface after quotienting by the actual HKO orbit symmetries. It "
            "does not solve the KKT/action problem, but it measures the finite "
            "combinatorial input size for the large exact route before any "
            "further exact pruning."
        ),
    }
    OUTPUT_PATH.write_text(json.dumps(payload, indent=2) + "\n")
    print(f"Wrote {OUTPUT_PATH.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
