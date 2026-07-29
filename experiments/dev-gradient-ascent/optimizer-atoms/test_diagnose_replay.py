"""Unit tests for schema-v2 optimizer predictor diagnostics."""

from __future__ import annotations

import importlib.util
import sys
from pathlib import Path

import numpy as np


def load_module():
    directory = Path(__file__).parent
    sys.path.insert(0, str(directory))
    path = directory / "diagnose_replay.py"
    spec = importlib.util.spec_from_file_location("diagnose_replay", path)
    assert spec and spec.loader
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def test_affine_cause_prioritizes_target_domain_failures() -> None:
    module = load_module()
    base = {
        "predicted_winning_sigma": [0, 1],
        "predicted_winner_matches_target": False,
        "predicted_winner_target_transition_feasible": True,
        "predicted_winner_target_raw_status": "ok",
        "predicted_winner_target_raw_normalized_beta_margin": 0.2,
    }
    assert module.affine_cause(base) == "different target-admissible branch"
    assert (
        module.affine_cause(
            {**base, "predicted_winner_target_raw_normalized_beta_margin": -0.2}
        )
        == "target beta nonpositive"
    )
    assert (
        module.affine_cause(
            {**base, "predicted_winner_target_transition_feasible": False}
        )
        == "target transition blocked"
    )


def test_winner_cause_rows_preserves_denominators() -> None:
    module = load_module()
    rows = [
        {
            "normalized_distance": 5.0e-5,
            "omission_class": "covered_by_anchor_universe",
            "anchor_raw_normalized_beta_margin": 0.1,
        },
        {
            "normalized_distance": 5.0e-5,
            "omission_class": "anchor_transition_blocked",
            "anchor_raw_normalized_beta_margin": -0.1,
        },
    ]
    result = module.winner_cause_rows(rows)[0]
    assert result["targets"] == 2
    assert result["covered_by_anchor_universe"] == 1
    assert result["anchor_transition_blocked"] == 1
    assert result["future_winner_beta_nonpositive_at_anchor"] == 1


def test_rank_uses_average_rank_for_ties() -> None:
    module = load_module()
    assert np.array_equal(
        module.rank(np.asarray([2.0, 1.0, 2.0, 4.0])),
        np.asarray([1.5, 0.0, 1.5, 3.0]),
    )
