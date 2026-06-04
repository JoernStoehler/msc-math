#!/usr/bin/env sage
"""
Probe exact Sage KKT solve cost on symmetry-quotiented HKO sigma representatives.

Goal: estimate how practical the representative-first Sage route looks on the
      `628` directed-feasible sigma orbits before exact KKT/action pruning.
Input Artifacts: experiments/hko-local-maximum/theorem/exact-witness/billiard-sigma-orbits.json
Output Artifacts: experiments/hko-local-maximum/theorem/exact-witness/billiard-sage-probe.json
"""

from __future__ import annotations

import argparse
import json
import random
import time
from pathlib import Path

from sage.all import QQ, RDF, NumberField, PolynomialRing, matrix, vector


EXPERIMENT_DIR = Path(__file__).resolve().parent
INPUT_PATH = EXPERIMENT_DIR / "billiard-sigma-orbits.json"
OUTPUT_PATH = EXPERIMENT_DIR / "billiard-sage-probe.json"


def exact_hko_duals(K):
    t = K.gen()
    sqrt5 = (QQ(5) - t**2) / QQ(2)
    alpha = (QQ(3) - sqrt5) / QQ(2)
    beta = t * (QQ(1) + sqrt5) / QQ(2)
    sec36 = sqrt5 - QQ(1)
    return [
        vector(K, [QQ(1), t, QQ(0), QQ(0)]),
        vector(K, [-alpha, beta, QQ(0), QQ(0)]),
        vector(K, [-sec36, QQ(0), QQ(0), QQ(0)]),
        vector(K, [-alpha, -beta, QQ(0), QQ(0)]),
        vector(K, [QQ(1), -t, QQ(0), QQ(0)]),
        vector(K, [QQ(0), QQ(0), t, -QQ(1)]),
        vector(K, [QQ(0), QQ(0), beta, alpha]),
        vector(K, [QQ(0), QQ(0), QQ(0), sec36]),
        vector(K, [QQ(0), QQ(0), -beta, alpha]),
        vector(K, [QQ(0), QQ(0), -t, -QQ(1)]),
    ]


def omega(lhs, rhs, j0):
    return (lhs * j0 * rhs.column())[0]


def build_kkt_matrix(duals, sigma, j0):
    m = len(sigma)
    h_matrix = matrix(duals[0].base_ring(), m, m)
    for i in range(m):
        for j in range(i + 1, m):
            value = omega(duals[sigma[i]], duals[sigma[j]], j0)
            h_matrix[i, j] = value
            h_matrix[j, i] = value

    a_block = matrix(duals[0].base_ring(), [[duals[facet][coord] for coord in range(4)] for facet in sigma])
    ones = matrix(duals[0].base_ring(), m, 1, [QQ(1)] * m)
    top = h_matrix.augment(a_block).augment(ones)
    middle = a_block.transpose().augment(matrix(duals[0].base_ring(), 4, 4)).augment(
        matrix(duals[0].base_ring(), 4, 1)
    )
    bottom = matrix(duals[0].base_ring(), 1, m, [QQ(1)] * m).augment(
        matrix(duals[0].base_ring(), 1, 4)
    ).augment(matrix(duals[0].base_ring(), 1, 1))
    system = top.stack(middle).stack(bottom)
    rhs = vector(duals[0].base_ring(), [QQ(0)] * m + [QQ(0), QQ(0), QQ(0), QQ(0), QQ(1)])
    return system, rhs


def probe_sigma(duals, sigma, j0, real_embedding):
    system, rhs = build_kkt_matrix(duals, sigma, j0)
    start = time.perf_counter()
    rank = system.rank()
    augmented_rank = system.augment(rhs.column()).rank()
    elapsed_s = time.perf_counter() - start

    nullity = system.ncols() - rank
    consistent = rank == augmented_rank
    beta_min_float = None
    q_float = None

    if consistent and nullity == 0:
        solution = system.solve_right(rhs)
        beta_entries = [solution[index] for index in range(len(sigma))]
        beta_min_float = min(float(real_embedding(entry)) for entry in beta_entries)
        h_matrix = system[: len(sigma), : len(sigma)]
        beta_column = vector(system.base_ring(), beta_entries)
        q_value = (beta_column * h_matrix * beta_column.column())[0] / QQ(2)
        q_float = float(real_embedding(q_value))

    return {
        "sigma": sigma,
        "length": len(sigma),
        "elapsed_s": elapsed_s,
        "rank": int(rank),
        "augmented_rank": int(augmented_rank),
        "nullity": int(nullity),
        "consistent": consistent,
        "beta_min_float": beta_min_float,
        "q_float": q_float,
    }


def quantiles(values):
    if not values:
        return {}
    ordered = sorted(values)
    positions = {
        "min": 0,
        "median": len(ordered) // 2,
        "p90": int(0.9 * (len(ordered) - 1)),
        "max": len(ordered) - 1,
    }
    return {label: ordered[index] for label, index in positions.items()}


def normalize_json(value):
    if isinstance(value, dict):
        return {str(key): normalize_json(entry) for key, entry in value.items()}
    if isinstance(value, list):
        return [normalize_json(entry) for entry in value]
    if type(value).__name__ == "Integer":
        return int(value)
    if type(value).__name__ == "RealDoubleElement":
        return float(value)
    return value


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--limit", type=int, default=50)
    parser.add_argument("--seed", type=int, default=0)
    args = parser.parse_args()

    payload = json.loads(INPUT_PATH.read_text())
    representatives = []
    for per_k in payload["sigma_orbits_per_k"].values():
        for orbit in per_k:
            representatives.append(orbit["canonical_representative"])

    limit = min(int(args.limit), len(representatives))
    sample = random.Random(int(args.seed)).sample(representatives, limit)

    ring = PolynomialRing(QQ, "x")
    x = ring.gen()
    K = NumberField(x**4 - 10 * x**2 + 5, "t")
    duals = exact_hko_duals(K)
    j0 = matrix(K, [[0, 0, -1, 0], [0, 0, 0, -1], [1, 0, 0, 0], [0, 1, 0, 0]])
    real_embedding = K.embeddings(RDF)[0]

    results = [probe_sigma(duals, sigma, j0, real_embedding) for sigma in sample]
    elapsed_values = [entry["elapsed_s"] for entry in results]
    consistent = [entry for entry in results if entry["consistent"]]
    unique = [entry for entry in consistent if entry["nullity"] == 0]

    summary = {
        "sample_size": limit,
        "total_representatives_available": len(representatives),
        "elapsed_s": quantiles(elapsed_values),
        "n_consistent": len(consistent),
        "n_unique": len(unique),
        "n_with_free_parameters": len([entry for entry in consistent if entry["nullity"] > 0]),
        "n_inconsistent": len([entry for entry in results if not entry["consistent"]]),
        "projected_elapsed_s_for_all_representatives": (
            sum(elapsed_values) * len(representatives) / limit if limit else None
        ),
    }

    out = {
        "input_artifact": INPUT_PATH.name,
        "sample_seed": args.seed,
        "summary": summary,
        "sample_results": results,
        "theorem_use": (
            "This is a backend-feasibility probe for the representative-first "
            "Sage route. It does not certify minima or exact first-order rows; "
            "it only measures the exact KKT front-end cost on the symmetry-"
            "quotiented directed-feasible sigma surface."
        ),
    }
    OUTPUT_PATH.write_text(json.dumps(normalize_json(out), indent=2) + "\n")
    print(f"Wrote {OUTPUT_PATH.relative_to(EXPERIMENT_DIR.parent.parent.parent)}")


if __name__ == "__main__":
    main()
