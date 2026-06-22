#!/usr/bin/env python3
"""Tests for summarize_observations.py."""

from __future__ import annotations

import csv
import json
import tempfile
from pathlib import Path
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parent))
from summarize_observations import read_jsonl, summarize_numeric, summarize_predicates, write_outputs


def test_summaries_group_numeric_and_predicate_rows() -> None:
    rows = [
        {
            "event": "observation",
            "mode": "smoke",
            "algorithm": "projection_kkt",
            "variable": "q",
            "context_kind": "sigma_node",
            "object_family": "known_polytope",
            "input_pair_kind": "rational_source_to_f64",
            "sample_policy": "fixture",
            "oracle_kind": "exact_rational",
            "f64": 1.25,
            "oracle_f64": 1.0,
            "abs_error": 0.25,
            "rel_error": 0.25,
        },
        {
            "event": "predicate_observation",
            "mode": "smoke",
            "algorithm": "projection_kkt",
            "predicate": "beta_positive",
            "context_kind": "sigma_node",
            "object_family": "known_polytope",
            "input_pair_kind": "binary64_input_to_exact",
            "sample_policy": "fixture",
            "oracle_kind": "exact_binary64_input",
            "oracle_binary": True,
            "f64_trinary": "false",
            "disagrees_with_oracle": True,
        },
    ]

    numeric = summarize_numeric(rows)
    predicates = summarize_predicates(rows)

    assert numeric[0]["count"] == 1
    assert numeric[0]["with_oracle"] == 1
    assert numeric[0]["max_abs_error"] == 0.25
    assert predicates[0]["input_pair_kind"] == "binary64_input_to_exact"
    assert predicates[0]["disagreements"] == 1


def test_cli_outputs_csv_and_report() -> None:
    with tempfile.TemporaryDirectory() as tmp:
        run_dir = Path(tmp)
        events_path = run_dir / "events.jsonl"
        rows = [
            {
                "event": "context_started",
                "object_id": "known:simplex",
                "sigma": [0, 2, 1, 3, 4],
                "input_pair_kind": "rational_source_to_f64",
                "sample_policy": "smoke_known_winner",
            },
            {
                "event": "observation",
                "mode": "smoke",
                "algorithm": "projection_kkt",
                "variable": "beta",
                "context_kind": "sigma_node",
                "object_family": "known_polytope",
                "input_pair_kind": "rational_source_to_f64",
                "sample_policy": "smoke_known_winner",
                "oracle_kind": "exact_rational",
                "f64": 1.0,
                "oracle_f64": 1.0,
                "abs_error": 0.0,
            },
            {
                "event": "observation",
                "mode": "smoke",
                "algorithm": "matrix_assembly",
                "variable": "sigma_min_c",
                "context_kind": "sigma_node",
                "object_family": "known_polytope",
                "input_pair_kind": "rational_source_to_f64",
                "sample_policy": "smoke_known_winner",
                "f64": 0.5,
            },
            {
                "event": "observation",
                "mode": "smoke",
                "algorithm": "f64_capacity",
                "variable": "capacity",
                "context_kind": "f64_capacity_scan_row",
                "object_id": "retained:row",
                "object_family": "random",
                "input_pair_kind": "f64_input_with_stored_capacity_label",
                "sample_policy": "verification_manifest",
                "sigma": [0, 1],
                "f64": 2.0,
                "comparison_label_kind": "stored_artifact_label",
                "comparison_label_f64": 2.5,
                "comparison_label_abs_difference": 0.5,
            },
            {
                "event": "observation",
                "mode": "smoke",
                "algorithm": "f64_capacity",
                "variable": "omega_indeterminate_count",
                "context_kind": "f64_capacity_scan_row",
                "object_family": "generated_product_f64",
                "input_pair_kind": "f64_input_with_exact_audit",
                "sample_policy": "verification_manifest",
                "f64": 4.0,
            },
            {
                "event": "predicate_observation",
                "mode": "smoke",
                "algorithm": "projection_kkt",
                "predicate": "beta_positive",
                "context_kind": "sigma_node",
                "object_family": "known_polytope",
                "input_pair_kind": "rational_source_to_f64",
                "sample_policy": "smoke_known_winner",
                "oracle_kind": "exact_rational",
                "oracle_binary": True,
                "f64_trinary": "true",
                "disagrees_with_oracle": False,
            },
        ]
        events_path.write_text(
            "".join(json.dumps(row) + "\n" for row in rows),
            encoding="utf-8",
        )

        write_outputs(read_jsonl(events_path), events_path, run_dir)

        with (run_dir / "processed" / "numeric_summary.csv").open(encoding="utf-8") as handle:
            csv_rows = list(csv.DictReader(handle))
        assert any(row["variable"] == "beta" for row in csv_rows)
        report = (run_dir / "report.md").read_text(encoding="utf-8")
        assert "Emitted Context Bank" in report
        assert "Oracle-Backed f64 Measurements" in report
        assert "Stored Comparison Labels" in report
        assert "Predicate Agreement Diagnostics" in report
        assert "Conditioning And Solver Diagnostics Without Oracle" in report
        assert "| known:simplex | [0, 2, 1, 3, 4] | rational_source_to_f64 | smoke_known_winner |" in report
        assert "| retained:row | [0, 1] | f64_input_with_stored_capacity_label | f64_capacity | capacity | 2 | 2.5 | 0.5 |" in report
        assert "| rational_source_to_f64 | matrix_assembly | sigma_min_c | smoke_known_winner | 1 | 0.5 | 0.5 |" in report
        assert "| f64_input_with_exact_audit | f64_capacity | omega_indeterminate_count | verification_manifest | 1 | 4 | 4 |" in report
        assert "Exact-binary64-input predicate disagreements: 0" in report
        assert "Interpretation Boundary" in report

def test_report_marks_predicates_not_measured_when_absent() -> None:
    with tempfile.TemporaryDirectory() as tmp:
        run_dir = Path(tmp)
        events_path = run_dir / "events.jsonl"
        events_path.write_text(
            json.dumps(
                {
                    "event": "observation",
                    "mode": "smoke",
                    "algorithm": "f64_capacity",
                    "variable": "capacity",
                    "context_kind": "f64_capacity_scan_row",
                    "object_id": "row",
                    "object_family": "random",
                    "input_pair_kind": "f64_input_with_exact_audit",
                    "sample_policy": "verification_manifest",
                    "f64": 1.0,
                    "oracle_kind": "exact_audit",
                    "oracle_f64": 1.0,
                    "abs_error": 0.0,
                }
            )
            + "\n",
            encoding="utf-8",
        )
        write_outputs(read_jsonl(events_path), events_path, run_dir)
        report = (run_dir / "report.md").read_text(encoding="utf-8")

    assert "Predicate observations: 0" in report
    assert "Predicate disagreements: not measured in this run." in report
    assert "No predicate disagreements were observed in this run." not in report


class SummarizeObservationsUnittest(unittest.TestCase):
    def test_summaries_group_numeric_and_predicate_rows(self) -> None:
        test_summaries_group_numeric_and_predicate_rows()

    def test_cli_outputs_csv_and_report(self) -> None:
        test_cli_outputs_csv_and_report()

    def test_report_marks_predicates_not_measured_when_absent(self) -> None:
        test_report_marks_predicates_not_measured_when_absent()


if __name__ == "__main__":
    unittest.main()
