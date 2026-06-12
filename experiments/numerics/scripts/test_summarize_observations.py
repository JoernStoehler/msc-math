#!/usr/bin/env python3
"""Tests for summarize_observations.py."""

from __future__ import annotations

import csv
import json
import tempfile
from pathlib import Path

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
        assert "Predicate Agreement Diagnostics" in report
        assert "Conditioning And Solver Diagnostics Without Oracle" in report
        assert "| known:simplex | [0, 2, 1, 3, 4] | rational_source_to_f64 | smoke_known_winner |" in report
        assert "| rational_source_to_f64 | matrix_assembly | sigma_min_c | smoke_known_winner | 1 | 0.5 | 0.5 |" in report
        assert "Exact-binary64-input predicate disagreements: 0" in report
        assert "Interpretation Boundary" in report


if __name__ == "__main__":
    test_summaries_group_numeric_and_predicate_rows()
    test_cli_outputs_csv_and_report()
