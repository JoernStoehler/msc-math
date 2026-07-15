import hashlib
import json
import subprocess
import tempfile
import unittest
from pathlib import Path


HERE = Path(__file__).parent
FILES = ("panel.jsonl", "diversity.tsv", "product-arms.jsonl", "fixtures.json", "manifest.json", "REPORT.md")


class PolarityPacketTests(unittest.TestCase):
    def run_packet(self, out):
        subprocess.run(
            ["python3", str(HERE / "run.py"), "--out-dir", str(out), "--seed", "20260715", "--per-stratum", "24"],
            check=True,
            cwd=HERE.parents[4],
            stdout=subprocess.PIPE,
            text=True,
        )

    def test_replay_and_contract(self):
        with tempfile.TemporaryDirectory() as d:
            a, b = Path(d) / "a", Path(d) / "b"
            self.run_packet(a)
            self.run_packet(b)
            for name in FILES:
                self.assertEqual(hashlib.sha256((a / name).read_bytes()).digest(), hashlib.sha256((b / name).read_bytes()).digest(), name)
            rows = [json.loads(line) for line in (a / "panel.jsonl").read_text().splitlines()]
            self.assertEqual(len(rows), 72)
            self.assertEqual({r["stratum"] for r in rows}, {3, 4, 6})
            self.assertTrue(all(r["bounded"] and r["irredundant"] for r in rows))
            self.assertTrue(all(r["preserved_double_residual"] == 0.0 for r in rows))
            self.assertTrue(all(r["centroid_double_residual"] == 0.0 for r in rows))
            fixtures = {r["fixture"]: r for r in json.loads((a / "fixtures.json").read_text())}
            self.assertEqual(fixtures["marked-double-polar"]["residual"], 0.0)
            self.assertEqual(fixtures["centroid-translation-covariance"]["residual"], 0.0)
            self.assertEqual(fixtures["symmetric-double-polar-negative-control"]["residual"], 0.0)
            self.assertEqual(fixtures["raw-origin-translation-failure"]["status"], "defined: raw origin remains interior")
            self.assertGreater(fixtures["raw-origin-translation-failure"]["raw_vs_centroid_residual"], 1e-3)
            self.assertEqual(fixtures["recenter-every-step-non-involution"]["status"], "fails")

    def test_metric_and_product_validity(self):
        with tempfile.TemporaryDirectory() as d:
            out = Path(d) / "out"
            self.run_packet(out)
            fixtures = {r["fixture"]: r for r in json.loads((out / "fixtures.json").read_text())}
            metric = fixtures["support-metric-invariance-controls"]
            for key in ("scale", "translation", "rotation_90", "reflection"):
                self.assertLess(metric[key], 1e-12, key)
            self.assertGreater(metric["distinct_triangle"], 1e-3)
            arms = [json.loads(line) for line in (out / "product-arms.jsonl").read_text().splitlines()]
            self.assertEqual(len(arms), 144)
            self.assertTrue(all(row["exact_reconstruction"] and row["incidence_valid"] for row in arms))
            self.assertTrue(all(row["normalized_volume"] == "1" for row in arms))
            self.assertTrue(all(all(count == 4 for count in row["vertex_incidence_counts"]) for row in arms))
            manifest = json.loads((out / "manifest.json").read_text())
            self.assertEqual(manifest["seeds"], [20260715, 20260716, 20260717])
            self.assertTrue(all(all(value == 8 for value in counts.values()) for counts in manifest["seed_stratum_counts"].values()))

    def test_fail_closed_minimum_panel(self):
        proc = subprocess.run(["python3", str(HERE / "run.py"), "--per-stratum", "23"], cwd=HERE.parents[4], text=True, capture_output=True)
        self.assertNotEqual(proc.returncode, 0)


if __name__ == "__main__":
    unittest.main()
