#!/usr/bin/env python3

import importlib.util
import json
from pathlib import Path
import tempfile
import unittest


HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("within_quality", HERE / "analyze.py")
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)


class WithinDistributionTests(unittest.TestCase):
    def test_frame_view_removes_cyclic_start_and_rotation(self):
        base = module.regular_polygon(8)
        left = module.normalize_vertices(base)
        right = module.normalize_vertices(module.affine_shape(base, 0, 0, 0.47, 3))
        self.assertGreater(module.raw_distance(left, right), 1e-2)
        self.assertLess(module.procrustes_distance(left, right), 1e-10)

    def test_duplicate_and_outlier_diagnostics(self):
        rows = [module.validate_row(r, i + 1) for i, r in enumerate(module.synthetic_rows(per_case=8))]
        duplicated = [r for r in rows if r["population"] == "duplicated"]
        report = module.summarize_stratum(duplicated)
        self.assertGreater(report["views"]["frame_adjusted"]["pair_distances"]["duplicate_pair_fraction"], 0.1)
        contaminated = [r for r in rows if r["population"] == "contaminated-outliers"]
        out = module.summarize_stratum(contaminated)["views"]["frame_adjusted"]["outlier_sensitivity"]
        self.assertGreater(abs(out["relative_change"]), 0.05)

    def test_strata_do_not_pool_population_or_side_count(self):
        rows = [module.validate_row(r, i + 1) for i, r in enumerate(module.synthetic_rows(per_case=3))]
        report = module.analyze(rows)
        self.assertEqual(report["strata_count"], 9)
        self.assertTrue(all(item["side_count"] == 8 for item in report["strata"]))

    def test_good_turing_warns_at_small_n(self):
        rows = [module.validate_row(r, i + 1) for i, r in enumerate(module.synthetic_rows(per_case=4))]
        summary = module.occupancy([r for r in rows if r["population"] == "broad"])
        self.assertEqual(summary["good_turing_status"], "small-sample-limit")

    def test_jsonl_adapter_and_report_are_serializable(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "rows.jsonl"
            module.write_jsonl(module.synthetic_rows(per_case=2), path)
            loaded = module.load_rows(path)
            report = module.analyze(loaded)
            json.dumps(report)
            self.assertEqual(report["rows"], 18)


if __name__ == "__main__":
    unittest.main()
