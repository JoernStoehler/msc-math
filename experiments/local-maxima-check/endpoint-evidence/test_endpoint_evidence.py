"""Tests for G1 evidence classification, adapters, and smoke output."""

from __future__ import annotations

import importlib.util
import json
import sys
import tempfile
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent


def load(name: str, filename: str):
    spec = importlib.util.spec_from_file_location(name, HERE / filename)
    assert spec and spec.loader
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


ANALYZE = load("g1_analyze_test", "analyze.py")
ADAPT = load("g1_adapt_test", "adapt.py")


def state(state_id: str, expected_ids: list[str], **updates):
    row = {
        "schema_version": 1,
        "row_type": "state",
        "state_id": state_id,
        "source": "synthetic",
        "control_role": "synthetic",
        "selection_kind": "current",
        "outcome_selected": False,
        "state_valid": True,
        "suite_id": "suite",
        "expected_probe_ids": expected_ids,
        "material_delta_sys": 1e-12,
        "positive_fixed_f_evidence": None,
        "facet_addition_evidence": None,
    }
    row.update(updates)
    return row


def probe(state_id: str, probe_id: str, delta: float | None, **updates):
    row = {
        "schema_version": 1,
        "row_type": "probe",
        "state_id": state_id,
        "probe_id": probe_id,
        "suite_id": "suite",
        "direction_family": "signed_basis",
        "direction_index": 0,
        "sign": 1,
        "radius": 1e-4,
        "step_norm": 1e-4,
        "base_sys": 1.0,
        "perturbed_sys": None if delta is None else 1.0 + delta,
        "delta_sys": delta,
        "state_valid": delta is not None,
        "same_incidence": True,
        "uncertainty": "nominal_f64",
        "failure": None if delta is not None else "failed",
    }
    row.update(updates)
    return row


def continuation(state_id: str, delta: float, **updates):
    row = {
        "schema_version": 1,
        "row_type": "continuation",
        "state_id": state_id,
        "continuation_id": "tail",
        "extra_compute_ms": 100.0,
        "extra_trusted_calls": 10,
        "delta_sys": delta,
        "path_length": 0.2,
        "displacement": 0.1,
        "stopping_reason": "budget",
        "state_valid": True,
        "required_for_classification": False,
    }
    row.update(updates)
    return row


class ClassificationTests(unittest.TestCase):
    def classify(self, state_row, probes=(), continuations=()):
        return ANALYZE.classify_state(state_row, list(probes), list(continuations))

    def test_valid_improvement_precedes_other_invalid_probe(self):
        result = self.classify(
            state("x", ["good", "bad"]),
            [probe("x", "good", 1e-4), probe("x", "bad", None)],
        )
        self.assertEqual(result["finite_classification"], "explicit_improvement_found")

    def test_continuation_rejects_poll_survivor(self):
        result = self.classify(
            state("x", ["a"]),
            [probe("x", "a", -1e-5)],
            [continuation("x", 2e-3)],
        )
        self.assertEqual(result["finite_classification"], "explicit_improvement_found")
        self.assertEqual(result["continuation_extra_trusted_calls"], 10)
        self.assertEqual(result["raw_probe_rows"][0]["probe_id"], "a")
        self.assertEqual(result["raw_continuation_rows"][0]["continuation_id"], "tail")

    def test_missing_failed_and_indeterminate_are_not_survivors(self):
        missing = self.classify(state("x", ["a", "b"]), [probe("x", "a", -1e-5)])
        failed = self.classify(state("x", ["a"]), [probe("x", "a", None)])
        indeterminate = self.classify(
            state("x", ["a"]),
            [probe("x", "a", -1e-5, uncertainty="indeterminate")],
        )
        for result in (missing, failed, indeterminate):
            self.assertEqual(
                result["finite_classification"], "invalid_or_indeterminate"
            )

    def test_complete_named_suite_survives(self):
        result = self.classify(
            state("x", ["a", "b"]),
            [probe("x", "a", -1e-5), probe("x", "b", -2e-5)],
        )
        self.assertEqual(result["finite_classification"], "finite_suite_survivor")
        self.assertEqual(result["valid_expected_probe_count"], 2)

    def test_no_suite_is_not_survivor(self):
        result = self.classify(state("x", [], suite_id=None))
        self.assertEqual(result["finite_classification"], "no_finite_suite")

    def test_stronger_evidence_slots_are_separate(self):
        result = self.classify(
            state(
                "x",
                ["a"],
                positive_fixed_f_evidence={"status": "proved"},
                facet_addition_evidence={"status": "named_finite_miss"},
            ),
            [probe("x", "a", -1e-5)],
        )
        self.assertEqual(result["finite_classification"], "finite_suite_survivor")
        self.assertEqual(result["positive_fixed_f_evidence"]["status"], "proved")
        self.assertEqual(
            result["facet_addition_evidence"]["status"], "named_finite_miss"
        )


class IntegrationTests(unittest.TestCase):
    def test_smoke_fixture_and_reports(self):
        with tempfile.TemporaryDirectory() as directory:
            out = Path(directory) / "out"
            ANALYZE.write_outputs(HERE / "fixtures" / "contract-events.ndjson", out)
            summary = json.loads((out / "summary.json").read_text())
            self.assertTrue(summary["all_declared_controls_passed"])
            self.assertEqual(summary["state_count"], 8)
            self.assertTrue((out / "REPORT.md").is_file())

    def test_lfs_pointer_has_specific_error(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "rows.jsonl"
            path.write_text(
                "version https://git-lfs.github.com/spec/v1\n"
                "oid sha256:123\nsize 1\n"
            )
            with self.assertRaisesRegex(ValueError, "Git LFS pointer"):
                list(ANALYZE.read_jsonl(path))

    def test_control_adapter_preserves_expected_evidence(self):
        rows = ADAPT.adapt_control_calibration(
            HERE / "fixtures" / "control-calibration-source.ndjson"
        )
        state_rows = [row for row in rows if row["row_type"] == "state"]
        self.assertEqual(len(state_rows), 2)
        self.assertEqual(
            {row["expected_finite_classification"] for row in state_rows},
            {"finite_suite_survivor", "explicit_improvement_found"},
        )

    def test_quotient_adapter_keeps_outcome_selection(self):
        rows = ADAPT.adapt_quotient(
            HERE / "fixtures" / "quotient-states-source.ndjson",
            HERE / "fixtures" / "quotient-probes-source.ndjson",
        )
        states = [row for row in rows if row["row_type"] == "state"]
        self.assertTrue(states[0]["outcome_selected"])
        self.assertEqual(states[0]["selection_kind"], "outcome_selected_control")
        self.assertEqual(
            states[0]["expected_finite_classification"],
            "explicit_improvement_found",
        )

    def test_quotient_selection_semantics_are_role_based(self):
        self.assertEqual(
            ADAPT.QUOTIENT_SELECTION_BY_ROLE[
                "positive_control_exact_theorem_local_maximum"
            ],
            ("predeclared_control", False),
        )
        self.assertEqual(
            ADAPT.QUOTIENT_SELECTION_BY_ROLE["unknown_frozen_high_best_so_far"],
            ("global_best_so_far", True),
        )
        self.assertEqual(
            ADAPT.QUOTIENT_SELECTION_BY_ROLE[
                "unknown_frozen_terminal_best_so_far"
            ],
            ("terminal_best_so_far", True),
        )
        self.assertEqual(
            ADAPT.QUOTIENT_SELECTION_BY_ROLE["held_out_optimizer_endpoint"],
            ("outcome_selected_checkpoint", True),
        )


if __name__ == "__main__":
    unittest.main()
