#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# ///

"""Classify representative numerical minimizing permutations modulo HKO symmetries.

Goal: compress the current numerical exact-minimum class representatives by the
      10-element HKO symplectic symmetry group and cyclic reindexing, so Packet
      3 can track how many permutation-level representative families are still live.
Input Artifacts: experiments/hko-local-maximum/theorem/exact-witness/numerical-family-reconciliation.json
Output Artifacts: experiments/hko-local-maximum/theorem/exact-witness/numerical-permutation-orbits.json
"""

from __future__ import annotations

import json
from pathlib import Path


EXPERIMENT_DIR = Path(__file__).resolve().parent
INPUT_PATH = EXPERIMENT_DIR / "numerical-family-reconciliation.json"
OUTPUT_PATH = EXPERIMENT_DIR / "numerical-permutation-orbits.json"


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


def canonical_cyclic_permutation(permutation: list[int]) -> tuple[int, ...]:
    rotations = [
        tuple(permutation[index:] + permutation[:index])
        for index in range(len(permutation))
    ]
    return min(rotations)


def permutation_orbit(permutation: list[int]) -> set[tuple[int, ...]]:
    orbit = set()
    for facet_map in GROUP:
        transformed = [facet_map[facet] for facet in permutation]
        orbit.add(canonical_cyclic_permutation(transformed))
    return orbit


def classify(classes: list[dict[str, object]]) -> list[dict[str, object]]:
    representatives = {
        entry["id"]: list(entry["representative_permutation"])
        for entry in classes
    }
    unseen = set(representatives)
    orbits = []
    while unseen:
        representative_id = min(unseen)
        orbit_keys = permutation_orbit(representatives[representative_id])
        members = sorted(
            entry_id
            for entry_id, permutation in representatives.items()
            if canonical_cyclic_permutation(permutation) in orbit_keys
        )
        for entry_id in members:
            unseen.discard(entry_id)
        orbits.append(
            {
                "representative_id": representative_id,
                "member_ids": members,
                "n_members": len(members),
                "canonical_orbit_representatives": [list(key) for key in sorted(orbit_keys)],
            }
        )
    return sorted(orbits, key=lambda entry: (entry["n_members"], entry["member_ids"]))


def main() -> None:
    numerical = json.loads(INPUT_PATH.read_text())
    size6_orbits = classify(numerical["size6_gradient_classes"])
    size7_orbits = classify(numerical["size7_gradient_classes"])

    payload = {
        "input_artifact": INPUT_PATH.name,
        "symmetry_generators": {
            "delta": DELTA,
            "phi": PHI,
        },
        "equivalence_relation": (
            "Representative permutations are quotiented by the 10-element HKO "
            "symplectic symmetry group on facet labels and by cyclic rotation "
            "of the same cyclic orbit."
        ),
        "n_size6_representative_orbits": len(size6_orbits),
        "n_size7_representative_orbits": len(size7_orbits),
        "size6_permutation_orbits": size6_orbits,
        "size7_permutation_orbits": size7_orbits,
        "theorem_use": (
            "This artifact is a numerical Packet 3 planning surface. It does not "
            "prove which representative permutations survive exactification, but it records "
            "how much permutation-level diversity remains after quotienting by the "
            "obvious HKO symmetries."
        ),
    }
    OUTPUT_PATH.write_text(json.dumps(payload, indent=2) + "\n")


if __name__ == "__main__":
    main()
