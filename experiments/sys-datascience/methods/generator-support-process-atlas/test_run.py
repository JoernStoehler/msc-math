#!/usr/bin/env python3

import importlib.util
import math
import unittest
from pathlib import Path


MODULE_PATH = Path(__file__).with_name("run.py")
SPEC = importlib.util.spec_from_file_location("support_process_atlas", MODULE_PATH)
assert SPEC is not None and SPEC.loader is not None
atlas = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(atlas)


class SupportProcessAtlasTests(unittest.TestCase):
    def test_sigma_zero_is_equal_support_identity(self) -> None:
        rng = atlas.deterministic_rng("sigma-zero-control")
        angles = [0.1, 1.4, 2.8, 4.0, 5.2]
        latent = [
            max(-atlas.CLIPPED_GAUSSIAN_BOUND, min(atlas.CLIPPED_GAUSSIAN_BOUND, atlas.standard_normal(rng)))
            for _ in angles
        ]
        center = atlas.mean(latent)
        supports = [math.exp(0.0 * (value - center)) for value in latent]
        self.assertEqual(supports, [1.0] * len(angles))
        self.assertEqual(atlas.support_process_metrics(supports)["log_support_roughness"], 0.0)

    def test_coherent_field_is_smoother_than_equal_variance_iid_sample(self) -> None:
        n = 16
        rng = atlas.deterministic_rng("iid-roughness-control", 0)
        iid_raw = [atlas.standard_normal(rng) for _ in range(n)]
        iid_center = atlas.mean(iid_raw)
        iid_centered = [value - iid_center for value in iid_raw]
        iid_scale = 0.1 / atlas.population_sd(iid_centered)
        iid_logs = [iid_scale * value for value in iid_centered]
        cosine_raw = [math.cos(2.0 * math.pi * index / n) for index in range(n)]
        cosine_scale = 0.1 / atlas.population_sd(cosine_raw)
        smooth_logs = [cosine_scale * value for value in cosine_raw]
        self.assertAlmostEqual(atlas.population_sd(iid_logs), atlas.population_sd(smooth_logs), places=14)
        iid = atlas.support_process_metrics([math.exp(value) for value in iid_logs])
        smooth = atlas.support_process_metrics([math.exp(value) for value in smooth_logs])
        self.assertGreater(iid["log_support_roughness"], smooth["log_support_roughness"])
        self.assertLess(iid["log_support_adjacency_correlation"], smooth["log_support_adjacency_correlation"])

    def test_fans_and_latents_replay_without_redraw(self) -> None:
        first = atlas.generate_rows((17,), (4,), 3)
        second = atlas.generate_rows((17,), (4,), 3)
        self.assertEqual(atlas.stable_json(first), atlas.stable_json(second))
        _, attempts = first
        grouped = {}
        for row in attempts:
            grouped.setdefault(row["fan_id"], []).append(row)
        self.assertEqual(len(grouped), 3)
        for rows in grouped.values():
            self.assertEqual({row["arm"] for row in rows}, set(atlas.ARMS))
            self.assertEqual(len({row["latent_id"] for row in rows}), len(atlas.ARMS))

    def test_complete_subset_requires_every_arm_and_retains_failure(self) -> None:
        _, attempts = atlas.generate_rows((31,), (4,), 1)
        attempts[0]["accepted"] = False
        attempts[0]["failure_reason"] = "forced_test_failure"
        attempts[0]["complete_paired_subset"] = False
        fan_id = attempts[0]["fan_id"]
        for row in attempts:
            if row["fan_id"] == fan_id:
                row["complete_paired_subset"] = False
        self.assertEqual(atlas.mark_complete_fans(attempts), [])
        self.assertEqual(atlas.failure_counts(attempts)["forced_test_failure"], 1)

    def test_equal_source_distances_collapse(self) -> None:
        _, attempts = atlas.generate_rows((41,), (6,), 8)
        equal = [row for row in attempts if row["arm"] == "equal" and row["accepted"]]
        self.assertGreater(len(equal), 0)
        for row in equal:
            self.assertEqual(row["metrics"]["source_support_l2"], 0.0)
            self.assertEqual(row["metrics"]["source_support_linf"], 0.0)
            self.assertEqual(row["metrics"]["source_vertex_rms"], 0.0)

    def test_empty_complete_stratum_is_reported_not_omitted(self) -> None:
        _, attempts = atlas.generate_rows((53,), (4,), 1)
        for row in attempts:
            row["complete_paired_subset"] = False
        metric_rows = atlas.metric_summary_rows(attempts, "complete_paired")
        self.assertEqual(len(metric_rows), len(atlas.ARMS))
        self.assertTrue(all(row["attempted"] == 0 for row in metric_rows))
        cv_rows = atlas.cv_matching_rows(attempts, True)
        self.assertEqual(len(cv_rows), len(atlas.SMOOTH_IID_CV_COMPARISONS))
        self.assertTrue(all(row["status"] == "not_evaluable_no_accepted_shapes" for row in cv_rows))
        monotonic = atlas.sigma_monotonicity_rows(attempts, True)
        self.assertEqual(len(monotonic), 4)
        self.assertTrue(all(not row["evaluable"] for row in monotonic))


if __name__ == "__main__":
    unittest.main()
