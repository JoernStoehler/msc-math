#!/usr/bin/env python3
"""Focused deterministic, invariance, and calibration checks for compare.py."""

from pathlib import Path
import sys

import numpy as np

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import compare  # noqa: E402


def test_calibration_controls():
    report = compare.calibration(123)
    by_name = {row["case"]: row for row in report["cases"]}
    assert by_name["location_change"]["energy_v"] > by_name["same_law_split"]["energy_v"]
    assert by_name["disjoint_modes"]["mixing"]["cross_nearest_neighbor_mixing"] < by_name["same_law_split"]["mixing"]["cross_nearest_neighbor_mixing"]
    assert by_name["high_dim_noise"]["mmd_bandwidths"]["0.5"] != by_name["high_dim_noise"]["mmd_bandwidths"]["4"]


def test_views_and_determinism():
    quadrilateral = np.array([[-1.0, -0.8], [1.3, -0.5], [0.8, 1.1], [-0.9, 0.9]])
    q_reference = compare._views(quadrilateral)
    for angle in np.linspace(0.0, 2 * np.pi, 37, endpoint=False):
        q_candidate = compare._views(np.roll(compare._rotate(quadrilateral, float(angle)), 1, axis=0))
        assert np.linalg.norm(q_reference["canonicalized"] - q_candidate["canonicalized"]) < 2e-10

    theta = np.linspace(0, 2 * np.pi, 6, endpoint=False)
    v = np.c_[np.cos(theta), np.sin(theta)] * np.array([1.0, 1.2, 0.9, 1.15, 0.95, 1.1])[:, None]
    row = {"schema": compare.SCHEMA, "sample_id": "a", "population": "a", "side_count": 6, "vertices_ccw": v.tolist()}
    reference = compare._views(v)
    for angle in np.linspace(0.0, 2 * np.pi, 17, endpoint=False):
        rotated = np.roll(compare._rotate(v, float(angle)), 2, axis=0)
        candidate = compare._views(rotated)
        assert np.linalg.norm(reference["raw_ordered"] - candidate["raw_ordered"]) > 1e-3
        assert np.linalg.norm(reference["canonicalized"] - candidate["canonicalized"]) < 1e-10
        assert np.linalg.norm(reference["chord_multiset_quotient"] - candidate["chord_multiset_quotient"]) < 1e-10
    row["_views"] = reference
    rotated = np.roll(compare._rotate(v, 0.37), 2, axis=0)
    row2 = {"schema": compare.SCHEMA, "sample_id": "b", "population": "b", "side_count": 6, "vertices_ccw": rotated.tolist(), "_views": compare._views(rotated)}
    first = compare.compare_stratum([row], [row2], "chord_multiset_quotient", 9)
    second = compare.compare_stratum([row], [row2], "chord_multiset_quotient", 9)
    assert first == second
    assert first["median_cross_distance"] < 1e-10
    assert first["sample_size_status"] == "uncalibrated_descriptive"
    assert first["disposition"] == "descriptive_only_do_not_treat_as_estimate_or_ranking"


if __name__ == "__main__":
    test_calibration_controls()
    test_views_and_determinism()
    print("ok")
