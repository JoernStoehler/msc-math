import json
from pathlib import Path
import subprocess
import sys

import numpy as np
import pytest

import analyze


def test_view_invariance_and_disagreement():
    theta = np.linspace(0, 2 * np.pi, 6, endpoint=False)
    radii = np.array([1.0, 1.2, 0.8, 1.1, 0.9, 1.3])
    base = np.stack([radii * np.cos(theta), radii * np.sin(theta)], axis=1)
    rotation = np.array([[np.cos(0.73), -np.sin(0.73)], [np.sin(0.73), np.cos(0.73)]])
    variants = [np.roll(base, 2, axis=0), np.roll(base[::-1], 1, axis=0), base @ rotation.T + np.array([3.0, -2.0])]
    for variant in variants:
        assert np.linalg.norm(analyze.chord_view(base) - analyze.chord_view(variant)) < 1e-10
        assert np.linalg.norm(analyze.frame_view(base) - analyze.frame_view(variant)) < 1e-10
    assert np.linalg.norm(base.reshape(-1) - variants[-1].reshape(-1)) > 1.0


def test_synthetic_calibration(tmp_path: Path):
    report = analyze.synthetic_calibration(tmp_path)
    assert report["remote_mode_discovered"]
    assert report["contamination_not_population_region"]
    assert report["representation_raw_distance"] > report["representation_quotient_distance"]
    assert (tmp_path / "synthetic-calibrations.tsv").is_file()


def test_report_contract_after_run():
    report_path = Path(__file__).parent / "artifacts" / "analysis" / "report.json"
    if not report_path.exists():
        return
    report = json.loads(report_path.read_text())
    assert report["schema"] == analyze.SCHEMA
    assert set(report["policies"]) == {"passive_coreset", "offline_greedy_max", "offline_greedy_frame", "offline_greedy_chord"}
    assert report["rows"]["side_count"] == 6
    assert len(report["input_hashes"]) >= 4
    assert report["environment"]["numpy_requirement"] == "numpy==1.26.4"
    assert report["environment"]["numpy_version"] == "1.26.4"
    assert "selection_cost_ms" not in report
    assert report["selection_cost_artifact"] == "selection-cost-observation.json"
    for arm in report["arms"]:
        assert arm["offline_greedy_max_fixed_train_panel_selected_witnesses"]["not_intrinsic_arm_property"]
        assert arm["offline_greedy_max_fixed_train_panel_holdout_mean_nonredundant_views"]["not_intrinsic_arm_property"]


def test_byte_replay_under_pinned_environment(tmp_path: Path):
    assert np.__version__ == "1.26.4", "run tests with: uv run --with pytest --with numpy==1.26.4 python -m pytest"
    packet = Path(__file__).parent
    inputs = [
        "--train", str((packet / "artifacts/train/factor-shapes.jsonl").resolve()),
        "--holdout", str((packet / "artifacts/holdout/factor-shapes.jsonl").resolve()),
        "--producer-report", str((packet / "artifacts/train/factor-only-report.json").resolve()),
        "--producer-report", str((packet / "artifacts/holdout/factor-only-report.json").resolve()),
    ]
    runs = []
    for index in (1, 2):
        output = tmp_path / f"run{index}"
        subprocess.run([sys.executable, str((packet / "analyze.py").resolve()), *inputs, "--out-dir", str(output)], check=True)
        runs.append(output)
    deterministic = ("coreset-yield.tsv", "view-disagreement.tsv", "policy-summary.tsv", "generation-cost.tsv", "synthetic-calibrations.tsv", "report.json")
    for name in deterministic:
        assert (runs[0] / name).read_bytes() == (runs[1] / name).read_bytes(), name
    observation = json.loads((runs[0] / "selection-cost-observation.json").read_text())
    assert observation["nondeterministic"] and observation["excluded_from_byte_replay"]


def test_pool_contract_after_run():
    packet = Path(__file__).parent
    train = analyze.load(packet / "artifacts/train/factor-shapes.jsonl")
    holdout = analyze.load(packet / "artifacts/holdout/factor-shapes.jsonl")
    analyze.validate_pool_contract(train, holdout)
    assert {s.seed for s in train}.isdisjoint({s.seed for s in holdout})
    assert {s.sample_id for s in train}.isdisjoint({s.sample_id for s in holdout})
    with pytest.raises(ValueError, match="sample IDs overlap"):
        analyze.validate_pool_contract(train, train)
    with pytest.raises(ValueError, match="unbalanced"):
        analyze.validate_pool_contract(train[:-1], holdout)
