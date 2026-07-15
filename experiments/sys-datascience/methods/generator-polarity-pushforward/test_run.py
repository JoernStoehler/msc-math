import hashlib
import copy
import json
import subprocess
import tempfile
import unittest
from pathlib import Path


HERE = Path(__file__).parent
FILES = ("panel.jsonl", "diversity.tsv", "product-arms.jsonl", "fixtures.json", "manifest.json", "REPORT.md")
import sys
sys.path.insert(0, str(HERE))
import run


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
            self.assertTrue(all("all fields below are exact" not in r["rationalization"] for r in rows))
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
            panel = [json.loads(line) for line in (out / "panel.jsonl").read_text().splitlines()]
            source_ids = {row["source_id"] for row in panel}
            image_ids = {image_id for row in panel for image_id in row["image_ids"].values()}
            self.assertEqual(len(source_ids), 72)
            self.assertEqual(len(image_ids), 72 * 4)
            self.assertTrue(all(row["q_source_id"] in source_ids and row["p_source_id"] in source_ids for row in arms))
            self.assertTrue(all((row["q_polar_image_id"] is None or row["q_polar_image_id"] in image_ids) and (row["p_polar_image_id"] is None or row["p_polar_image_id"] in image_ids) for row in arms))
            groups = {}
            for row in arms:
                groups.setdefault(row["pair_id"], {})[row["arm"]] = row
            self.assertTrue(all(set(group) == {"QxP", "QpolarxP", "QxPpolar", "QpolarxPpolar"} for group in groups.values()))
            for group in groups.values():
                base = group["QxP"]
                self.assertTrue(all(row["q_source_id"] == base["q_source_id"] and row["p_source_id"] == base["p_source_id"] for row in group.values()))
                self.assertIsNone(base["q_polar_image_id"])
                self.assertIsNone(base["p_polar_image_id"])
                self.assertIsNotNone(group["QpolarxP"]["q_polar_image_id"])
                self.assertIsNotNone(group["QxPpolar"]["p_polar_image_id"])
            bad = copy.deepcopy(arms)
            qpolar = next(row for row in bad if row["arm"] == "QpolarxP")
            other_image = next(row["image_ids"]["centroid"] for row in panel if row["source_id"] != qpolar["q_source_id"])
            qpolar["q_polar_image_id"] = other_image
            with self.assertRaises(RuntimeError):
                run.validate_product_arm_linkage(bad, panel)
            diversity_header = (out / "diversity.tsv").read_text().splitlines()[0].split("\t")
            self.assertIn("directed_polar_to_source_nearest_paired_included", diversity_header)
            self.assertIn("directed_polar_to_source_nearest_leave_pair_out", diversity_header)
            self.assertIn("paired_source_wins", diversity_header)
            manifest = json.loads((out / "manifest.json").read_text())
            self.assertEqual(manifest["seeds"], [20260715, 20260716, 20260717])
            self.assertTrue(all(all(value == 8 for value in counts.values()) for counts in manifest["seed_stratum_counts"].values()))
            exact_boundary = manifest["exact_boundary"]
            self.assertIn("rational vertices/facets/incidence/areas/centroids/polars/Mahler", exact_boundary)
            self.assertIn("stored residual numbers are f64 diagnostics", exact_boundary)
            self.assertIn("/tmp/polarity-artifacts/", (out / "REPORT.md").read_text())

    def test_fail_closed_minimum_panel(self):
        proc = subprocess.run(["python3", str(HERE / "run.py"), "--per-stratum", "23"], cwd=HERE.parents[4], text=True, capture_output=True)
        self.assertNotEqual(proc.returncode, 0)


if __name__ == "__main__":
    unittest.main()
