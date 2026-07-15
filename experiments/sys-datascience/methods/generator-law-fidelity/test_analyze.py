#!/usr/bin/env python3
"""Executable calibration tests for generator-law-fidelity."""

import importlib.util
import math
import random
import unittest
from pathlib import Path


MODULE = Path(__file__).with_name("analyze.py")
SPEC = importlib.util.spec_from_file_location("generator_law_fidelity", MODULE)
assert SPEC and SPEC.loader
AUDIT = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(AUDIT)


class CalibrationTests(unittest.TestCase):
    def test_required_synthetic_controls_are_detected(self) -> None:
        results = {item["control"]: item["passed"] for item in AUDIT.synthetic_calibrations()}
        self.assertTrue(results["duplicated_independent_units"])
        self.assertTrue(results["biased_angular_sector"])
        self.assertTrue(results["law_ordered_truncation"])
        self.assertTrue(results["wrong_correlation_arm"])

    def test_dirichlet_pit_detects_wrong_alpha(self) -> None:
        rng = random.Random(4)
        gap_rows = []
        for _ in range(100):
            values = [rng.gammavariate(4.0, 1.0) for _ in range(6)]
            total = sum(values)
            gap_rows.append([math.tau * value / total for value in values])
        self.assertGreater(AUDIT.dirichlet_diagnostics(gap_rows, 1.0, 6)["pit_ks"], 0.12)

    def test_replay_ignores_timing_only(self) -> None:
        left = [{"sample_id": "x", "generation_ms": 1.0, "validation_ms": 2.0}]
        right = [{"sample_id": "x", "generation_ms": 8.0, "validation_ms": 13.0}]
        self.assertEqual(AUDIT.check_replay(left, right)["status"], "pass")

    def test_wrong_correlation_arm_is_flagged(self) -> None:
        rng = random.Random(5)
        q = [rng.gauss(0.0, 1.0) for _ in range(200)]
        p = [rng.gauss(0.0, 1.0) for _ in range(200)]
        self.assertTrue(AUDIT.latent_correlation(q, p, 0.8)["flagged"])


if __name__ == "__main__":
    unittest.main()
