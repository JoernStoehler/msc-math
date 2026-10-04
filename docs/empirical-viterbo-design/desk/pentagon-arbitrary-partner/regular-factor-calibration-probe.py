# /// script
# requires-python = ">=3.11"
# dependencies = ["numpy", "scipy"]
# ///
"""Small floating-point probe of the reference-body calibration for n=3,5,7.

This is preliminary planning evidence, not an exact certificate. It enumerates
positive three-normal closures and minimizes the dilation needed to translate
each normalized triangle into B=(Q-Q)^polar/2. No existing producer/data is read
or replaced. The output is printed for retention in a separate probe artifact.
"""

import itertools
import json

import numpy as np
from scipy.optimize import linprog


def probe(n):
    angles = np.arange(n) * 2 * np.pi / n
    normals = np.column_stack((np.cos(angles), np.sin(angles)))
    vertices = np.column_stack((np.cos(angles + np.pi / n), np.sin(angles + np.pi / n)))
    differences = np.array([a - b for a in vertices for b in vertices if np.linalg.norm(a - b) > 1e-12])
    # t.dot(d) + max(T.dot(d)) <= r/2 describes T+t contained in rB.
    inequalities = np.column_stack((differences, -0.5 * np.ones(len(differences))))
    templates = []
    h = np.cos(np.pi / n)
    for triple in itertools.combinations(range(n), 3):
        selected = normals[list(triple)]
        weights = np.linalg.solve(np.vstack((selected.T, np.ones(3))), [0, 0, 1])
        if min(weights) <= 1e-10:
            continue
        steps = weights[:, None] * selected / h
        triangle = np.array([[0, 0], steps[0], steps[0] + steps[1]])
        supports = np.max(triangle @ differences.T, axis=0)
        result = linprog([0, 0, 1], A_ub=inequalities, b_ub=-supports,
                         bounds=[(None, None), (None, None), (0, None)], method="highs")
        if not result.success:
            raise RuntimeError(result.message)
        templates.append({"normal_indices": list(triple), "minimum_cover_dilation": float(result.fun),
                          "translation": result.x[:2].tolist(), "triangle": triangle.tolist(),
                          "maximum_constraint_residual": float(np.max(inequalities @ result.x + supports))})
    worst = max(templates, key=lambda row: row["minimum_cover_dilation"])
    return {"sides": n, "positive_closure_triples": len(templates), "worst_template": worst,
            "all_templates_fit_numerically": worst["minimum_cover_dilation"] <= 1 + 1e-9}


if __name__ == "__main__":
    print(json.dumps({"evidence_strength": "floating-point exploratory LP, not a proof",
                      "cases": [probe(n) for n in (3, 5, 7)]}, indent=2))
