import hashlib
import contextlib
import io
import json
import tempfile
import unittest
from pathlib import Path

import analyze


PACKET = Path(__file__).resolve().parent
TARGET = PACKET / "artifacts/target-rows.jsonl"
MANIFEST = PACKET / "artifacts/target-manifest.json"


class FrozenAnalyzerCalibration(unittest.TestCase):
    def test_positive_negative_and_ties(self):
        self.assertEqual(analyze.rank_average([1, 1, 2]), [1.5, 1.5, 3.0])
        self.assertAlmostEqual(analyze.spearman([1, 2, 3], [1, 2, 3]), 1.0)
        self.assertAlmostEqual(analyze.spearman([1, 2, 3], [3, 2, 1]), -1.0, places=12)
        self.assertLess(analyze.spearman([1, 1, 2], [3, 2, 1]), -0.8)

    def test_primary_gates(self):
        self.assertEqual(analyze.primary_disposition(0.0, [0.02] * 6 + [0.0, 0.0]), "supports_material_alignment_role")
        self.assertEqual(analyze.primary_disposition(0.0, [0.001] * 8), "contradicts_material_role_on_frozen_maps")
        self.assertEqual(analyze.primary_disposition(2e-8, [0.2] * 8), "ambiguous_numerical_control_failure")

    def _copy_artifacts(self, directory):
        target = Path(directory) / "target-rows.jsonl"
        manifest = Path(directory) / "target-manifest.json"
        target.write_bytes(TARGET.read_bytes())
        data = json.loads(MANIFEST.read_text())
        data["target_path"] = str(target)
        manifest.write_text(json.dumps(data, indent=2) + "\n")
        return target, manifest

    @staticmethod
    def _rows(path):
        return [json.loads(line) for line in path.read_text().splitlines()]

    @staticmethod
    def _write_rows(path, rows):
        path.write_text("\n".join(json.dumps(row, separators=(",", ":"), sort_keys=True) for row in rows) + "\n")

    def test_missing_row_and_duplicate_or_substituted_id_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            target, manifest = self._copy_artifacts(directory)
            rows = self._rows(target)[:-1]
            self._write_rows(target, rows)
            data = json.loads(manifest.read_text())
            data["target_sha256"] = hashlib.sha256(target.read_bytes()).hexdigest()
            manifest.write_text(json.dumps(data, indent=2) + "\n")
            with self.assertRaises(analyze.AnalysisError):
                analyze.validate_inputs(target, manifest)

            target, manifest = self._copy_artifacts(directory)
            rows = self._rows(target)
            rows[-1]["source_id"] = rows[0]["source_id"]
            self._write_rows(target, rows)
            data = json.loads(manifest.read_text())
            data["target_sha256"] = hashlib.sha256(target.read_bytes()).hexdigest()
            manifest.write_text(json.dumps(data, indent=2) + "\n")
            with self.assertRaises(analyze.AnalysisError):
                analyze.validate_inputs(target, manifest)

    def test_target_mutation_with_stale_manifest_hash_warns(self):
        with tempfile.TemporaryDirectory() as directory:
            target, manifest = self._copy_artifacts(directory)
            rows = self._rows(target)
            rows[0]["sys"] += 1e-3
            self._write_rows(target, rows)
            analyze._WARNED_STALE.clear()
            stderr = io.StringIO()
            with contextlib.redirect_stderr(stderr):
                analyze.validate_inputs(target, manifest)
            self.assertIn("manifest/target bytes", stderr.getvalue())
            self.assertIn("target retained bytes", stderr.getvalue())

    def test_evaluator_provenance_mismatch_warns(self):
        with tempfile.TemporaryDirectory() as directory:
            target, manifest = self._copy_artifacts(directory)
            rows = self._rows(target)
            rows[0]["evaluator_source_sha256"] = "0" * 64
            self._write_rows(target, rows)
            data = json.loads(manifest.read_text())
            data["target_sha256"] = hashlib.sha256(target.read_bytes()).hexdigest()
            manifest.write_text(json.dumps(data, indent=2) + "\n")
            original_target_hash = analyze.TARGET_SHA
            analyze.TARGET_SHA = data["target_sha256"]
            analyze._WARNED_STALE.clear()
            try:
                stderr = io.StringIO()
                with contextlib.redirect_stderr(stderr):
                    analyze.validate_inputs(target, manifest)
                self.assertIn("target-row evaluator/design provenance", stderr.getvalue())
            finally:
                analyze.TARGET_SHA = original_target_hash

    def test_failed_partial_and_incomplete_manifests_are_never_interpreted(self):
        with tempfile.TemporaryDirectory() as directory:
            for status in ("failed", "partial", "incomplete"):
                target, manifest = self._copy_artifacts(directory)
                data = json.loads(manifest.read_text())
                data.update({"status": status, "completed_rows": 3, "failure": "bounded timeout"})
                manifest.write_text(json.dumps(data, indent=2) + "\n")
                with self.assertRaisesRegex(analyze.AnalysisError, "not interpretable"):
                    analyze.analyze(target, manifest)

            target, manifest = self._copy_artifacts(directory)
            target.unlink()
            with self.assertRaises(analyze.AnalysisError):
                analyze.analyze(target, manifest)

    def test_feature_snapshot_manifest_warns_on_wrong_full_artifact_binding(self):
        with tempfile.TemporaryDirectory() as directory:
            target, manifest = self._copy_artifacts(directory)
            snapshot_manifest = Path(directory) / "orientation-feature-manifest.json"
            data = json.loads(analyze.FEATURE_REPORT.read_text())
            data["full_feature_sha256"] = "0" * 64
            snapshot_manifest.write_text(json.dumps(data, indent=2) + "\n")
            original_report = analyze.FEATURE_REPORT
            original_report_sha = analyze.FEATURE_REPORT_SHA
            analyze.FEATURE_REPORT = snapshot_manifest
            analyze.FEATURE_REPORT_SHA = hashlib.sha256(snapshot_manifest.read_bytes()).hexdigest()
            analyze._WARNED_STALE.clear()
            try:
                stderr = io.StringIO()
                with contextlib.redirect_stderr(stderr):
                    analyze.validate_inputs(target, manifest)
                self.assertIn("feature snapshot provenance", stderr.getvalue())
            finally:
                analyze.FEATURE_REPORT = original_report
                analyze.FEATURE_REPORT_SHA = original_report_sha

            target, manifest = self._copy_artifacts(directory)
            target.write_text(target.read_text().splitlines()[0] + "\n")
            with self.assertRaises(analyze.AnalysisError):
                analyze.analyze(target, manifest)

            target, manifest = self._copy_artifacts(directory)
            target.write_text("{not-json}\n")
            with self.assertRaises(analyze.AnalysisError):
                analyze.analyze(target, manifest)


if __name__ == "__main__":
    unittest.main()
