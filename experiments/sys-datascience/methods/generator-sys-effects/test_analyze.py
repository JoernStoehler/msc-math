#!/usr/bin/env python3

import importlib.util
import tempfile
import unittest
from pathlib import Path

SPEC = importlib.util.spec_from_file_location("generator_sys_effects", Path(__file__).with_name("analyze.py"))
assert SPEC and SPEC.loader
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class GeneratorSysEffectsTests(unittest.TestCase):
    def test_paired_factorial_contrasts(self):
        rows = []
        for law, value in [
            ("factorial-baseline", 10.0),
            ("factorial-q", 8.0),
            ("factorial-p", 7.0),
            ("factorial-both", 6.0),
        ]:
            rows.append({"law": law, "pairing_id": "pair", "pair_bucket": "4x6", "sys": value})
        effect = MODULE.paired_effects(rows, MODULE.FACTORIAL)[0]
        self.assertTrue(effect["complete"])
        self.assertEqual(effect["contrasts"]["factorial_interaction"], 1.0)

    def test_actual_pilot_is_complete_and_censored_explicitly(self):
        source = Path(__file__).parents[1] / "alternative-generator-smoke/artifacts/target-pilot/smoke-rows.jsonl"
        rows, digest = MODULE.load_rows(source)
        report, witnesses = MODULE.analyze(rows, digest)
        self.assertEqual(report["row_count"], 68)
        self.assertEqual(report["evaluated_sys_rows"], 42)
        self.assertEqual(report["validation_status_counts"]["runtime_cap"], 25)
        self.assertEqual(report["validation_status_counts"]["invalid_or_low_acceptance"], 1)
        self.assertEqual(len(witnesses), 42)
        self.assertLess(report["factorial_3x3_negative_control_max_minus_min"], 1e-15)

    def test_write_outputs(self):
        with tempfile.TemporaryDirectory() as temporary:
            MODULE.write_outputs(Path(temporary), {"schema": "test"}, [])
            self.assertTrue((Path(temporary) / "report.json").is_file())
            self.assertTrue((Path(temporary) / "witnesses.tsv").is_file())


if __name__ == "__main__":
    unittest.main()
