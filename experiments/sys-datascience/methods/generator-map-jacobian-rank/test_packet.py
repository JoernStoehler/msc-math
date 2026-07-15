#!/usr/bin/env python3

import importlib.util
from pathlib import Path
import sys

import numpy as np

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("rank_packet", HERE / "analyze.py")
module = importlib.util.module_from_spec(spec); sys.modules[spec.name] = module; assert spec.loader is not None; spec.loader.exec_module(module)


def test_chart_similarity_and_label_invariance() -> None:
    polygon = np.array([[-1., -.7], [.8, -.9], [1.2, .4], [.1, 1.1], [-.9, .6]])
    base = module.body_chart(polygon)
    angle = .37; c, s = np.cos(angle), np.sin(angle)
    transformed = 3.2 * polygon @ np.array([[c, -s], [s, c]]).T + np.array([7., -2.])
    relabeled = np.roll(transformed[::-1], 2, axis=0)
    assert np.allclose(base.vector, module.body_chart(relabeled).vector, atol=2e-10)


def test_expected_generator_ranks_small() -> None:
    cases = [module.baseline_base(4, 11), module.dirichlet_base(4, 11, 4), module.regular_base(4, 11), module.zonogon_base(4, 11), module.mutation_base(4, 11), module.primal_hull_base(4, 11)]
    for base in cases:
        result = module.finite_difference_base(base)
        assert result["status"] == "ok", (base.law, result)
        assert result["expected_rank_matched_all_steps"], (base.law, result["primary_ranks"])


def test_calibrations_expose_threshold_dependence() -> None:
    calibration = module.synthetic_calibrations()
    assert calibration["full_rank"]["rank_by_relative_threshold"]["1e-07"] == 7
    assert calibration["three_gauge_directions"]["rank_by_relative_threshold"]["1e-07"] == 4
    near = calibration["near_singular"]["rank_by_relative_threshold"]
    assert len(set(near.values())) > 1


def test_discrete_state_changes_fail_closed() -> None:
    polygon = np.array([[-1., -1.], [1., -1.], [1., 1.], [-1., 1.]])
    def evaluator(latent: np.ndarray) -> module.Evaluation:
        state = ("side", int(np.sign(latent[0])))
        return module.Evaluation(polygon, state)
    base = module.Base("synthetic", "state-boundary", 4, 0, np.array([0.0]), 0, 0, evaluator)
    result = module.finite_difference_base(base)
    assert result["status"] == "perturbation-failure"
    assert result["reason"] == "discrete-topology-or-active-set-change"


def test_mutation_open_saturated_stratum_has_lower_rank() -> None:
    for n in (4, 6, 8):
        ordinary = module.mutation_base(n, 11)
        latent = ordinary.latent.copy()
        spacing = module.TAU / n
        offset = 1
        for _ in range(4):
            latent[offset : offset + n] = .2 * spacing + .05
            offset += 2 * n
        saturated = module.Base(ordinary.law, "all-angle-increments-positive-saturated", n, ordinary.seed, latent, n - 3, n - 3, ordinary.evaluator)
        result = module.finite_difference_base(saturated)
        assert result["status"] == "ok"
        assert result["expected_rank_matched_all_steps"], (n, result["primary_ranks"])
        assert result["base_discrete_state_summary"]["clipped_angle_increment_count"] == 4 * n


def test_invalid_polygon_fails_closed() -> None:
    nonconvex = np.array([[0., 0.], [2., 0.], [1., .2], [2., 2.], [0., 2.]])
    try:
        module.body_chart(nonconvex)
    except module.MapFailure as error:
        assert str(error) == "non-strict-convexity"
    else:
        raise AssertionError("nonconvex chart input did not fail")


if __name__ == "__main__":
    test_chart_similarity_and_label_invariance()
    test_expected_generator_ranks_small()
    test_calibrations_expose_threshold_dependence()
    test_discrete_state_changes_fail_closed()
    test_mutation_open_saturated_stratum_has_lower_rank()
    test_invalid_polygon_fails_closed()
    print("generator-map Jacobian-rank tests passed")
