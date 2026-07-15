#!/usr/bin/env python3
"""Fast semantic checks for the target-free dimension packet."""

import importlib.util
from pathlib import Path
import tempfile

import numpy as np


HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("dimension", HERE / "analyze.py")
module = importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(module)


def test_plane_and_noise_ordering() -> None:
    cases = module.synthetic_cases(3, 400)
    plane = module.assess(cases["plane_2d"][0], [8, 16])
    line = module.assess(cases["anisotropic_noisy_line"][0], [8, 16])
    assert 1.5 < plane["global_pca"]["participation_ratio"] < 2.2
    assert line["global_pca"]["participation_ratio"] < plane["global_pca"]["participation_ratio"]


def test_duplicate_guard_and_mixture_connectivity() -> None:
    cases = module.synthetic_cases(4, 300)
    duplicate = module.assess(cases["duplicate_contaminated_plane"][0], [8])
    assert duplicate["twonn"]["duplicate_or_tied_rows"] > 0
    mixture = module.assess(cases["two_component_plane_mixture"][0], [8])
    assert mixture["knn_connectivity"][0]["component_count"] >= 2


def test_dual_views_remove_facet_order_only() -> None:
    vertices = np.array([[-1., -1.], [1., -1.], [1., 1.], [-1., 1.]])
    dual = module.polygon_duals(vertices)
    assert dual.shape == (4, 2)
    assert np.all(np.sum(dual * vertices, axis=1) <= 1 + 1e-9)


def test_calibrated_mass_and_chart_coverage_are_separate() -> None:
    points = module.synthetic_cases(5, 120)["plane_2d"][0]
    region = module.calibrated_mass_region(points, seed=9)
    assert region["status"] == "ok"
    assert 0 <= region["law_mass_coverage"]["holdout_fraction"] <= 1
    assert 0 <= region["reference_chart_coverage_Q"]["fraction"] <= 1
    assert "not a selected/mixed stratum" in region["law_mass_coverage"]["meaning"]
    assert "neither support coverage" in region["reference_chart_coverage_Q"]["meaning"]


if __name__ == "__main__":
    test_plane_and_noise_ordering()
    test_duplicate_guard_and_mixture_connectivity()
    test_dual_views_remove_facet_order_only()
    test_calibrated_mass_and_chart_coverage_are_separate()
    print("dimension packet tests passed")
