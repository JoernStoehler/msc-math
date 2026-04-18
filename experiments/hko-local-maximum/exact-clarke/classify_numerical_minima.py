#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# ///

"""Classify the current numerical exact minima into endpoint and equality-case families.

Goal: turn the frozen numerical exact-minimum surface into a durable Packet 2
      reconciliation artifact that separates six-facet endpoint classes from
      seven-facet equality-case classes and records the observed segment
      relations between them.
Input Artifacts: experiments/hko-local-maximum/gradient-analysis/hko-neighborhood-sensitivity.jsonl
Output Artifacts: experiments/hko-local-maximum/exact-clarke/numerical-family-reconciliation.json
"""

from __future__ import annotations

import json
from collections import defaultdict
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
INPUT_PATH = (
    ROOT
    / "experiments"
    / "hko-local-maximum"
    / "gradient-analysis"
    / "hko-neighborhood-sensitivity.jsonl"
)
OUTPUT_PATH = Path(__file__).resolve().parent / "numerical-family-reconciliation.json"

GRADIENT_ROUND_DIGITS = 12
EXACT_ACTION_TOL = 1e-12
SEGMENT_ERR_TOL = 1e-9
SEGMENT_T_TOL = 1e-9


@dataclass(frozen=True)
class OrbitRecord:
    subset: tuple[int, ...]
    permutation: tuple[int, ...]
    beta: tuple[float, ...]
    gradient: tuple[float, ...]
    gradient_key: tuple[float, ...]


def rounded_gradient_key(gradient: tuple[float, ...]) -> tuple[float, ...]:
    return tuple(round(value, GRADIENT_ROUND_DIGITS) for value in gradient)


def vec_sub(lhs: tuple[float, ...], rhs: tuple[float, ...]) -> tuple[float, ...]:
    return tuple(a - b for a, b in zip(lhs, rhs, strict=True))


def vec_dot(lhs: tuple[float, ...], rhs: tuple[float, ...]) -> float:
    return sum(a * b for a, b in zip(lhs, rhs, strict=True))


def vec_norm_sq(vec: tuple[float, ...]) -> float:
    return vec_dot(vec, vec)


def build_exact_orbits() -> list[OrbitRecord]:
    row = json.loads(INPUT_PATH.read_text().splitlines()[0])
    orbits = row["orbits"]
    gradients = row["per_orbit_d_sys_h"]
    best_action = min(orbit["action"] for orbit in orbits)

    exact: list[OrbitRecord] = []
    for orbit, gradient in zip(orbits, gradients, strict=True):
        if abs(orbit["action"] - best_action) >= EXACT_ACTION_TOL:
            continue
        gradient_tuple = tuple(float(value) for value in gradient)
        exact.append(
            OrbitRecord(
                subset=tuple(int(value) for value in orbit["subset"]),
                permutation=tuple(int(value) for value in orbit["permutation"]),
                beta=tuple(float(value) for value in orbit["beta"]),
                gradient=gradient_tuple,
                gradient_key=rounded_gradient_key(gradient_tuple),
            )
        )
    return exact


def collect_classes(
    exact_orbits: list[OrbitRecord], subset_size: int
) -> list[dict[str, object]]:
    grouped: dict[tuple[tuple[int, ...], tuple[float, ...]], list[OrbitRecord]] = defaultdict(list)
    for orbit in exact_orbits:
        if len(orbit.subset) != subset_size:
            continue
        grouped[(orbit.subset, orbit.gradient_key)].append(orbit)

    classes: list[dict[str, object]] = []
    for class_id, ((subset, _), members) in enumerate(sorted(grouped.items()), start=1):
        representative = members[0]
        classes.append(
            {
                "id": f"{subset_size}-facet-class-{class_id:02d}",
                "subset": list(subset),
                "count": len(members),
                "representative_permutation": list(representative.permutation),
                "representative_beta": list(representative.beta),
                "gradient": list(representative.gradient_key),
            }
        )
    return classes


def beta_multiset_prototypes(classes: list[dict[str, object]]) -> list[dict[str, object]]:
    grouped: dict[tuple[float, ...], list[dict[str, object]]] = defaultdict(list)
    for entry in classes:
        beta_multiset = tuple(
            sorted(round(float(value), GRADIENT_ROUND_DIGITS) for value in entry["representative_beta"])
        )
        grouped[beta_multiset].append(entry)

    prototypes: list[dict[str, object]] = []
    for prototype_id, (beta_multiset, entries) in enumerate(sorted(grouped.items()), start=1):
        prototypes.append(
            {
                "id": f"beta-multiset-{prototype_id:02d}",
                "beta_multiset": list(beta_multiset),
                "class_ids": [entry["id"] for entry in entries],
                "n_classes": len(entries),
                "representative_subsets": [entry["subset"] for entry in entries],
            }
        )
    return prototypes


def beta_profile(
    permutation: list[int], beta: list[float], facets: list[int]
) -> list[float]:
    by_facet = {facet: value for facet, value in zip(permutation, beta, strict=True)}
    return [round(float(by_facet.get(facet, 0.0)), GRADIENT_ROUND_DIGITS) for facet in facets]


def candidate_segment_relation(
    target: dict[str, object], endpoints: list[dict[str, object]]
) -> dict[str, object] | None:
    target_subset = set(target["subset"])
    target_gradient = tuple(target["gradient"])
    best: dict[str, object] | None = None

    for idx, first in enumerate(endpoints):
        first_subset = set(first["subset"])
        if not first_subset.issubset(target_subset):
            continue
        first_gradient = tuple(first["gradient"])
        for second in endpoints[idx + 1 :]:
            second_subset = set(second["subset"])
            if not second_subset.issubset(target_subset):
                continue
            if first_subset | second_subset != target_subset:
                continue
            second_gradient = tuple(second["gradient"])
            delta = vec_sub(second_gradient, first_gradient)
            denom = vec_norm_sq(delta)
            if denom <= 0.0:
                continue
            t = vec_dot(vec_sub(target_gradient, first_gradient), delta) / denom
            residual = vec_sub(
                target_gradient,
                tuple(
                    first_value + t * delta_value
                    for first_value, delta_value in zip(first_gradient, delta, strict=True)
                ),
            )
            error = vec_norm_sq(residual) ** 0.5
            if error > SEGMENT_ERR_TOL or t < -SEGMENT_T_TOL or t > 1.0 + SEGMENT_T_TOL:
                continue
            candidate = {
                "first_endpoint_id": first["id"],
                "second_endpoint_id": second["id"],
                "first_subset": first["subset"],
                "second_subset": second["subset"],
                "coefficient_on_first": round(1.0 - t, 12),
                "coefficient_on_second": round(t, 12),
                "residual_norm": error,
            }
            if best is None or (
                candidate["residual_norm"],
                candidate["first_endpoint_id"],
                candidate["second_endpoint_id"],
            ) < (
                best["residual_norm"],
                best["first_endpoint_id"],
                best["second_endpoint_id"],
            ):
                best = candidate

    return best


def main() -> None:
    exact_orbits = build_exact_orbits()
    endpoint_classes = collect_classes(exact_orbits, subset_size=6)
    equality_classes = collect_classes(exact_orbits, subset_size=7)
    endpoint_by_id = {entry["id"]: entry for entry in endpoint_classes}

    for equality_class in equality_classes:
        equality_class["segment_witness"] = candidate_segment_relation(
            equality_class, endpoint_classes
        )
        witness = equality_class["segment_witness"]
        if witness is not None:
            first = endpoint_by_id[witness["first_endpoint_id"]]
            second = endpoint_by_id[witness["second_endpoint_id"]]
            facets = sorted(set(equality_class["subset"]))
            t = float(witness["coefficient_on_second"])
            first_profile = beta_profile(
                first["representative_permutation"], first["representative_beta"], facets
            )
            second_profile = beta_profile(
                second["representative_permutation"], second["representative_beta"], facets
            )
            target_profile = beta_profile(
                equality_class["representative_permutation"],
                equality_class["representative_beta"],
                facets,
            )
            combined_profile = [
                round((1.0 - t) * first_value + t * second_value, GRADIENT_ROUND_DIGITS)
                for first_value, second_value in zip(first_profile, second_profile, strict=True)
            ]
            residual = max(
                abs(target - combined)
                for target, combined in zip(target_profile, combined_profile, strict=True)
            )
            equality_class["beta_profile_combination_witness"] = {
                "ordered_facets": facets,
                "first_profile": first_profile,
                "second_profile": second_profile,
                "target_profile": target_profile,
                "combined_profile": combined_profile,
                "max_abs_residual": residual,
            }

    size6_subsets = sorted({tuple(entry["subset"]) for entry in endpoint_classes})
    size7_subsets = sorted({tuple(entry["subset"]) for entry in equality_classes})
    size7_class_count_by_subset: dict[tuple[int, ...], int] = defaultdict(int)
    segment_coefficients: set[float] = set()
    for entry in equality_classes:
        size7_class_count_by_subset[tuple(entry["subset"])] += 1
        witness = entry["segment_witness"]
        if witness is not None:
            segment_coefficients.add(witness["coefficient_on_second"])

    payload = {
        "input_artifact": str(INPUT_PATH.relative_to(ROOT)),
        "n_exact_action_orbits": len(exact_orbits),
        "n_exact_size6_orbits": sum(1 for orbit in exact_orbits if len(orbit.subset) == 6),
        "n_exact_size7_orbits": sum(1 for orbit in exact_orbits if len(orbit.subset) == 7),
        "n_distinct_size6_subsets": len(size6_subsets),
        "n_distinct_size7_subsets": len(size7_subsets),
        "n_distinct_size6_gradient_classes": len(endpoint_classes),
        "n_distinct_size7_gradient_classes": len(equality_classes),
        "n_size7_subsets_with_multiple_gradient_classes": sum(
            1 for count in size7_class_count_by_subset.values() if count > 1
        ),
        "all_size7_classes_have_segment_witness": all(
            entry["segment_witness"] is not None for entry in equality_classes
        ),
        "all_size7_classes_have_beta_profile_combination_witness": all(
            "beta_profile_combination_witness" in entry for entry in equality_classes
        ),
        "segment_coefficients_on_second_endpoint": sorted(segment_coefficients),
        "size6_subsets": [list(subset) for subset in size6_subsets],
        "size7_subsets": [list(subset) for subset in size7_subsets],
        "size6_gradient_classes": endpoint_classes,
        "size7_gradient_classes": equality_classes,
        "size6_beta_multiset_prototypes": beta_multiset_prototypes(endpoint_classes),
        "size7_beta_multiset_prototypes": beta_multiset_prototypes(equality_classes),
    }

    OUTPUT_PATH.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    print(f"Wrote {OUTPUT_PATH.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
