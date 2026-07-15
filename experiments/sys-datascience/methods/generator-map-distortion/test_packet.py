#!/usr/bin/env python3

import importlib.util
from pathlib import Path
import sys

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("distortion", HERE / "analyze.py"); module = importlib.util.module_from_spec(spec); sys.modules[spec.name] = module; assert spec.loader is not None; spec.loader.exec_module(module)


def test_reparameterization_counterexamples() -> None:
    cases = module.reparameterization_counterexamples()["coordinate_scaling"]
    assert cases["original"]["log_induced_body_density"] == 0
    assert abs(cases["scaled_diag_10_2"]["log_induced_body_density"]) < 1e-12
    assert cases["anisotropic_det_one_diag_10_point1"]["condition_number"] == 100
    assert cases["scaled_diag_10_2"]["log_pseudodeterminant"] != cases["original"]["log_pseudodeterminant"]


def test_dirichlet_density_contract() -> None:
    proportions = module.np.array([.18, .22, .27, .33])
    result = module.dirichlet_change_of_variables(proportions)
    assert result["log_body_volume_step_spread"] < 1e-5
    densities = result["law_densities_at_same_body_point"]
    assert set(densities) == {"alpha=1", "alpha=4", "alpha=16"}
    assert all(module.math.isfinite(value["log_generic_unlabeled_body_density_including_n_cyclic_preimages"]) for value in densities.values())
    for value in densities.values():
        assert abs(value["log_generic_unlabeled_body_density_including_n_cyclic_preimages"] - value["log_single_linked_label_branch_density_wrt_body_hausdorff_measure"] - module.math.log(4)) < 1e-12


def test_dirichlet_conditioning_normalizer() -> None:
    assert module.dirichlet_acceptance_normalizer(4, 1)["value"] == .5
    assert module.dirichlet_acceptance_normalizer(6, 1)["value"] == .8125
    assert module.dirichlet_acceptance_normalizer(8, 1)["value"] == .9375
    normalizer = module.dirichlet_acceptance_normalizer(4, 4)
    trials = 4 * 4 - 1
    expected = 1 - 4 * sum(module.math.comb(trials, k) for k in range(4)) / 2**trials
    assert abs(normalizer["value"] - expected) < 1e-15
    tiny_tail = module.dirichlet_acceptance_normalizer(8, 16)
    assert tiny_tail["value"] == 1.0
    assert tiny_tail["rejection_probability"] > 0
    assert tiny_tail["log_value"] < 0


def test_reflection_is_not_traversal_reversal() -> None:
    control = module.asymmetric_reflection_control()
    assert max(control["cyclic_start_chart_distances"]) < 1e-9
    assert control["same_vertex_set_traversal_reversal_chart_distance"] < 1e-9
    assert control["reflected_gap_sequence_chart_distance"] > 1e-3


def test_full_packet_contract() -> None:
    result = module.analyze(module.DEFAULT_RANK_REPORT)
    assert len(result["per_base_diagnostics"]) == 96
    assert len(result["stratum_summaries"]) == 24
    assert len(result["dirichlet_common_measure_comparisons"]) == 36
    assert len(result["dirichlet_reference_summaries"]) == 9
    assert all(item["change_of_variables"]["log_body_volume_step_spread"] < 1e-4 for item in result["dirichlet_common_measure_comparisons"])
    assert all(value.startswith("abandoned:") for key, value in result["density_dispositions"].items() if key not in {"equal_support_dirichlet_alpha_1_4_16", "regular_equal_support"})


if __name__ == "__main__": test_reparameterization_counterexamples(); test_dirichlet_density_contract(); test_dirichlet_conditioning_normalizer(); test_reflection_is_not_traversal_reversal(); test_full_packet_contract(); print("generator-map distortion tests passed")
