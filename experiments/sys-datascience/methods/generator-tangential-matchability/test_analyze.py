#!/usr/bin/env python3

import copy
import importlib.util
import json
import tempfile
import unittest
from pathlib import Path

SPEC = importlib.util.spec_from_file_location("generator_tangential_matchability", Path(__file__).with_name("analyze.py"))
assert SPEC and SPEC.loader
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


def fixture(rows_per_bucket=2, scope="fixture-v1"):
    rows = []
    seed = 31
    for bucket in MODULE.BUCKETS:
        for row_index in range(rows_per_bucket):
            attempt = row_index + 1
            pairing_id = f"altgen-v2/scope={scope}/factorial/seed={seed}/row={row_index}/attempt={attempt}/{bucket}"
            for law, parameter in MODULE.ARMS.items():
                rows.append(
                    {
                        "schema": MODULE.ROW_SCHEMA,
                        "sample_id": f"altgen-v2/scope={scope}/{law}/param={parameter}/seed={seed}/row={row_index}/attempt={attempt}/{bucket}",
                        "law": law,
                        "parameter": parameter,
                        "law_version": MODULE.LAW_VERSION,
                        "identity_scope": scope,
                        "seed": seed,
                        "row_index": row_index,
                        "attempt": attempt,
                        "attempts": attempt + 1,
                        "rejections": attempt,
                        "pair_bucket": bucket,
                        "accepted": True,
                        "validation_status": "survived",
                        "pairing_id": pairing_id,
                        "factor_q_area": 1.0,
                        "factor_p_area": 1.0,
                        "factor_q_support_cv": 0.1 if law in ("factorial-baseline", "factorial-p") else 0.0,
                        "factor_p_support_cv": 0.2 if law in ("factorial-baseline", "factorial-q") else 0.0,
                        "factor_q_gap_cv": 0.3,
                        "factor_p_gap_cv": 0.4,
                        "factor_q_isoperimetric_ratio": 0.7 if law in ("factorial-baseline", "factorial-p") else 0.8,
                        "factor_p_isoperimetric_ratio": 0.6 if law in ("factorial-baseline", "factorial-q") else 0.9,
                        "volume": 1.0,
                        "capacity": None,
                        "sys": None,
                        "iterations": None,
                        "target_ms": 0.0,
                        "generation_ms": 1.0,
                        "validation_ms": 2.0,
                    }
                )
    status_counts = {"survived": len(rows)}
    per_arm = []
    for law, parameter in MODULE.ARMS.items():
        group = [row for row in rows if row["law"] == law]
        per_arm.append(
            {
                "law": law,
                "parameter": parameter,
                "rows": len(group),
                "accepted_rows": len(group),
                "survived_rows": len(group),
                "max_attempts_observed": max(row["attempts"] for row in group),
                "factor_metric_count": 2 * len(group),
            }
        )
    manifest = {
        "schema": MODULE.MANIFEST_SCHEMA,
        "law_version": MODULE.LAW_VERSION,
        "identity_scope": scope,
        "seed": seed,
        "max_attempts_per_row": 8,
        "pairs": list(MODULE.BUCKETS),
        "rows": len(rows),
        "status_counts": status_counts,
        "per_arm": per_arm,
        "source_revision": "0" * 40,
        "command": (
            "target/release/sys-datascience-alternative-generator-smoke "
            "--out-dir /tmp/fixture --seed 31 --attempts 8 --runtime-cap-ms 2000 "
            f"--rows-per-law {rows_per_bucket} --only-family factorial --identity-scope {scope}"
        ),
    }
    return rows, manifest


class TangentialMatchabilityTests(unittest.TestCase):
    def test_complete_target_free_fixture(self):
        rows, manifest = fixture()
        MODULE.validate_packet(rows, manifest, 2, "fixture-v1")
        report = MODULE.analyze(rows, manifest, "rows", "manifest", 2, "fixture-v1")
        self.assertEqual(report["row_count"], 24)
        self.assertEqual(report["structurally_complete_pair_count"], 6)
        self.assertEqual(report["accepted_complete_pair_count"], 6)
        self.assertEqual(report["pairs_with_rejected_rows"], 0)
        self.assertEqual(report["normalization_agreement"]["max_product_volume_abs_error"], 0.0)
        self.assertTrue(
            all(
                item["max_abs_delta"] == 0.0
                for item in report["paired_expected_geometry_equalities"]
            )
        )

    def test_canonical_unaccepted_row_is_not_an_accepted_complete_pair(self):
        rows, manifest = fixture()
        row = rows[0]
        row["accepted"] = False
        row["validation_status"] = "invalid_or_low_acceptance"
        row["rejections"] = row["attempts"]
        row["volume"] = None
        row["sample_id"] = (
            f"altgen-v2/scope={row['identity_scope']}/{row['law']}/param={row['parameter']}/"
            f"seed={row['seed']}/row={row['row_index']}/outcome=exhausted/{row['pair_bucket']}"
        )
        for factor in ("q", "p"):
            row[f"factor_{factor}_area"] = None
            for metric in MODULE.FACTOR_METRICS:
                row[f"factor_{factor}_{metric}"] = None

        manifest["status_counts"] = {"invalid_or_low_acceptance": 1, "survived": 23}
        arm = next(entry for entry in manifest["per_arm"] if entry["law"] == row["law"])
        arm["accepted_rows"] -= 1
        arm["survived_rows"] -= 1
        arm["factor_metric_count"] -= 2

        MODULE.validate_packet(rows, manifest, 2, "fixture-v1")
        report = MODULE.analyze(rows, manifest, "rows", "manifest", 2, "fixture-v1")
        self.assertEqual(report["structurally_complete_pair_count"], 6)
        self.assertEqual(report["accepted_complete_pair_count"], 5)
        self.assertEqual(report["pairs_with_rejected_rows"], 1)
        self.assertEqual(report["accepted_rows"], 23)
        self.assertEqual(report["rejected_rows"], 1)
        with tempfile.TemporaryDirectory() as temporary:
            MODULE.write_outputs(Path(temporary), report)
            summary = (Path(temporary) / "summary.md").read_text()
        self.assertIn("structurally complete four-arm grids: 6", summary)
        self.assertIn("Accepted-complete geometry pairs: 5", summary)
        self.assertIn("Structural pairs containing rejected rows: 1", summary)

    def test_missing_arm_row_is_rejected(self):
        rows, manifest = fixture()
        rows.pop()
        manifest["rows"] -= 1
        manifest["status_counts"]["survived"] -= 1
        with self.assertRaisesRegex(MODULE.AnalysisError, "expected 24 factorial rows"):
            MODULE.validate_packet(rows, manifest, 2, "fixture-v1")

    def test_target_payload_is_rejected(self):
        rows, manifest = fixture()
        rows[0]["sys"] = 0.5
        with self.assertRaisesRegex(MODULE.AnalysisError, "contains capacity or sys"):
            MODULE.validate_packet(rows, manifest, 2, "fixture-v1")

    def test_manifest_target_flag_is_rejected(self):
        rows, manifest = fixture()
        manifest["command"] += " --target"
        with self.assertRaisesRegex(MODULE.AnalysisError, "cannot contain --target"):
            MODULE.validate_packet(rows, manifest, 2, "fixture-v1")

    def test_truncated_jsonl_is_rejected(self):
        rows, _ = fixture()
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "rows.jsonl"
            path.write_text("\n".join(json.dumps(row) for row in rows))
            with self.assertRaisesRegex(MODULE.AnalysisError, "no final newline"):
                MODULE.load_rows(path)

    def test_split_pairing_identity_is_rejected(self):
        rows, manifest = fixture()
        rows = copy.deepcopy(rows)
        rows[0]["pairing_id"] += "-alias"
        with self.assertRaisesRegex(MODULE.AnalysisError, "noncanonical"):
            MODULE.validate_packet(rows, manifest, 2, "fixture-v1")

    def test_mismatched_scoped_sample_identity_is_rejected(self):
        rows, manifest = fixture()
        rows[0]["sample_id"] += "-wrong"
        with self.assertRaisesRegex(MODULE.AnalysisError, "sample identity.*noncanonical"):
            MODULE.validate_packet(rows, manifest, 2, "fixture-v1")

    def test_duplicate_manifest_arm_is_rejected_before_mapping(self):
        rows, manifest = fixture()
        manifest["per_arm"][-1] = copy.deepcopy(manifest["per_arm"][0])
        with self.assertRaisesRegex(MODULE.AnalysisError, "arm keys must be unique"):
            MODULE.validate_packet(rows, manifest, 2, "fixture-v1")


if __name__ == "__main__":
    unittest.main()
