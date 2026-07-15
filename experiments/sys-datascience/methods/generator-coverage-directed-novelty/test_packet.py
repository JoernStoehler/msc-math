import json
from pathlib import Path

import numpy as np

import analyze


def test_view_invariance_and_disagreement():
    theta = np.linspace(0, 2 * np.pi, 6, endpoint=False)
    base = np.stack([np.cos(theta), np.sin(theta)], axis=1)
    rot = base @ np.array([[0.0, -1.0], [1.0, 0.0]]) + np.array([3.0, -2.0])
    assert np.linalg.norm(analyze.chord_view(base) - analyze.chord_view(rot)) < 1e-10
    assert np.linalg.norm(analyze.frame_view(base) - analyze.frame_view(rot)) < 1e-10
    assert np.linalg.norm(base.reshape(-1) - rot.reshape(-1)) > 1.0


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
    assert set(report["policies"]) == {"passive_random", "adaptive_max", "adaptive_frame", "adaptive_chord"}
    assert report["rows"]["side_count"] == 6
    assert len(report["input_hashes"]) >= 4
