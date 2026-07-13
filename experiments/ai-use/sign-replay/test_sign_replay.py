#!/usr/bin/env python3
import importlib.util
import sys
import unittest
from pathlib import Path


MODULE_PATH = Path(__file__).with_name("sign_replay.py")
spec = importlib.util.spec_from_file_location("sign_replay", MODULE_PATH)
assert spec and spec.loader
module = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = module
spec.loader.exec_module(module)


class SourceTransformTests(unittest.TestCase):
    def setUp(self):
        self.source = (
            "    let coeff = pi.dot(&b_prime) / eigenvalues[i];\n"
            "    /// Q is constant when H = 0: Q = 0 for all beta in the constraint set.\n"
        )

    def test_replace_coefficient_is_single_and_directional(self):
        good = module.replace_coefficient(self.source, correct_sign=True)
        self.assertIn(module.GOOD_COEFFICIENT, good)
        self.assertNotIn(module.BAD_COEFFICIENT, good)
        bad = module.replace_coefficient(good, correct_sign=False)
        self.assertIn(module.BAD_COEFFICIENT, bad)

    def test_injection_rejects_missing_marker(self):
        with self.assertRaises(ValueError):
            module.inject_regression("no marker", module.CASES[0])

    def test_dry_run_has_eight_rows_and_expected_exception(self):
        rows = module.dry_run()
        self.assertEqual(len(rows), 8)
        expected = {(row["case"], row["sign"]): row["expected_pass"] for row in rows}
        self.assertFalse(expected[("55-min", "bad-positive")])
        self.assertFalse(expected[("55-ver", "bad-positive")])
        self.assertTrue(expected[("56-min", "bad-positive")])
        self.assertFalse(expected[("56-ver", "bad-positive")])
        for case in module.CASES:
            self.assertTrue(expected[(case.label, "correct-negative")])


if __name__ == "__main__":
    unittest.main()
