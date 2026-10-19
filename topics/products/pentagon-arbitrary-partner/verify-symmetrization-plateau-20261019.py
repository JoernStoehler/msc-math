# /// script
# requires-python = ">=3.11"
# dependencies = ["numpy>=2.0", "scipy>=1.14"]
# ///
"""Bounded floating-point controls for the uniform pentagon plateau.

This reuses the existing onset LP; it does not rerun or overwrite a producer.
The theorem is analytic. LP residuals below are numerical corroboration only.
"""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path

import numpy as np
import scipy
from scipy.optimize import linprog
from scipy.spatial import ConvexHull

from evaluator import evaluate, regular, templates
from symmetrization import path, plateau_threshold


def construction_check(points: np.ndarray, width: float) -> dict:
    """Execute the proof's independent endpoint choice for each triangle."""
    eta = 3 - np.sqrt(5)
    t = eta / 2
    s = 1 - eta
    source_hull = ConvexHull(points)
    source_normals = source_hull.equations[:, :2]
    source_offsets = -source_hull.equations[:, 2]
    normals = np.concatenate([source_normals, -source_normals])
    hplus = np.max(normals @ points.T, axis=1)
    hminus = np.max(-normals @ points.T, axis=1)
    target_offsets = (1 - t) * hplus + t * hminus
    records = []
    for template in templates():
        vertices = width * np.asarray(template["vertices"])
        pairs = [(i, j) for i in range(3) for j in range(i)]
        i, j = max(pairs, key=lambda pair: np.linalg.norm(
            vertices[pair[0]] - vertices[pair[1]]))
        k = next(k for k in range(3) if k not in (i, j))
        d = vertices[i] - vertices[j]
        apex = vertices[k] - (vertices[i] + vertices[j]) / 2
        # Choose a in K with b=a-d in K; this is exactly d in K-K.
        lp = linprog(
            [0., 0.],
            A_ub=np.concatenate([source_normals, source_normals]),
            b_ub=np.concatenate([source_offsets,
                                 source_offsets + source_normals @ d]),
            bounds=[(None, None), (None, None)], method="highs")
        if not lp.success:
            raise RuntimeError(f"Endpoint LP: {lp.message}")
        midpoint = lp.x - d / 2
        placed = s * midpoint + np.asarray([-d / 2, d / 2, apex])
        slack = target_offsets[:, None] - normals @ placed.T
        records.append({"template_id": template["id"],
                        "minimum_slack": float(np.min(slack))})
    return {"t": float(t), "templates": records,
            "maximum_constraint_violation":
            max(0., -min(r["minimum_slack"] for r in records))}


def run() -> dict:
    irregular = np.asarray([[-1., -.6], [.6, -.9], [1.2, .1],
                            [.5, 1.1], [-.7, .8]])
    controls = {
        "regular_triangle_angle_0": regular(3),
        "regular_triangle_angle_0.17": regular(3, .17),
        "skinny_triangle": np.asarray([[-3., -.2], [3., -.2], [.1, .4]]),
        "irregular_pentagon": irregular,
        "translated_irregular": irregular + [23., -11.],
        "regular_pentagon_hko_angle": regular(5, 3 * np.pi / 10),
        "regular_decagon": regular(10),
        "elongated_irregular": irregular @ np.asarray([[10., 3.], [0., .2]]).T,
    }
    eta = 3 - np.sqrt(5)
    rows = []
    for name, points in controls.items():
        onset = plateau_threshold(points)
        width = onset["A2"]
        construction = construction_check(points, width)
        area0 = ConvexHull(points).volume
        difference = (points[:, None, :] - points[None, :, :]).reshape(-1, 2)
        area_difference = ConvexHull(difference).volume
        samples = []
        for t in [eta / 2, .45, .5, .55, 1 - eta / 2]:
            result = evaluate(path(points, t))
            area_formula = area0 + t * (1 - t) * (area_difference - 4 * area0)
            samples.append({"t": float(t),
                            "capacity_over_width": result["capacity"] / width,
                            "sys": result["sys"],
                            "area_formula_error": abs(result["area"] - area_formula)})
        row = {"name": name, "vertices": points.tolist(),
               "width": width, "first_plateau_t": onset["threshold"],
               "construction": construction, "central_samples": samples}
        if onset["threshold"] > eta / 2 + 1e-8:
            raise AssertionError(row)
        if construction["maximum_constraint_violation"] > 1e-8 * max(1., width):
            raise AssertionError(row)
        if max(abs(r["capacity_over_width"] - 1) for r in samples) > 1e-8:
            raise AssertionError(row)
        if max(r["area_formula_error"] for r in samples) > 1e-8 * max(1., area0):
            raise AssertionError(row)
        rows.append(row)
    return {
        "status": "floating-point corroboration; not a proof or optimality certificate",
        "numpy_version": np.__version__, "scipy_version": scipy.__version__,
        "script_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        "universal_sufficient_onset": float(eta / 2),
        "controls": rows,
    }


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path, required=True,
                        help="New output path; an existing file is never replaced.")
    args = parser.parse_args()
    if args.out.exists():
        raise FileExistsError(args.out)
    result = run()
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps({"status": result["status"],
                      "universal_sufficient_onset": result["universal_sufficient_onset"],
                      "onsets": {r["name"]: r["first_plateau_t"]
                                 for r in result["controls"]},
                      "output": str(args.out)}, indent=2))
