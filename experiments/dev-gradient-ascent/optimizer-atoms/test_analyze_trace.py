"""Unit tests for optimizer atom trace analysis."""

from __future__ import annotations

import importlib.util
from pathlib import Path


def load_module():
    path = Path(__file__).with_name("analyze_trace.py")
    spec = importlib.util.spec_from_file_location("analyze_trace", path)
    assert spec and spec.loader
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def test_summary_separates_trajectory_phase() -> None:
    module = load_module()
    rows = []
    for phase, error in [("early", 0.1), ("late", 0.9)]:
        rows.append(
            {
                "algorithm_id": "a",
                "phase": phase,
                "start_id": phase,
                "distance": 0.01,
                "prediction_error": error,
                "sign_correct": True,
                "candidate_covered": True,
                "selected": True,
                "validated_delta": 0.01,
                "proposal_compute_ms": 2.0,
                "candidate_count": 3,
            }
        )
    summary = module.summarize(rows)
    assert [row["phase"] for row in summary] == ["early", "late"]
    assert summary[0]["median_abs_prediction_error"] < summary[1][
        "median_abs_prediction_error"
    ]
