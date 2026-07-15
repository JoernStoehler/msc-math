from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

import numpy as np

sys.path.insert(0, str(Path(__file__).parent))
from analyze import _canonical_vector, _load_rows, _validate_polygon, _validate_upstream_report  # noqa: E402

HERE = Path(__file__).parent
ANALYZE = HERE / "analyze.py"
INPUTS = sorted((HERE / "artifacts" / "input").glob("seed-*/*/factor-shapes.jsonl"))


def run(out: Path, *extra: str) -> None:
    subprocess.run(
        ["uv", "run", "--script", str(ANALYZE), "--out-dir", str(out), *extra],
        cwd=HERE,
        check=True,
        capture_output=True,
        text=True,
    )


def test_calibration_positive_and_negative_controls(tmp_path: Path) -> None:
    out = tmp_path / "calibration"
    fixture = tmp_path / "synthetic.jsonl"
    run(out, "--calibration-only", "--write-synthetic-fixture", str(fixture))
    report = json.loads((out / "calibration.json").read_text())
    assert report["method_selected"]
    assert report["pass_count"] == 7
    assert report["controls"]["circle"]["pass"]
    assert report["controls"]["disk"]["pass"]
    assert report["controls"]["separated_mixture"]["pass"]
    duplicate_density = report["controls"]["duplicates"]["observed"]["density"]
    assert duplicate_density["input_duplicate_fraction"] > 0.5
    assert duplicate_density["resampling"] == "bootstrap_with_replacement_multiplicities_retained"
    clouds = {json.loads(line)["cloud"] for line in fixture.read_text().splitlines()}
    assert len(clouds) == 7


def test_real_strata_are_side_and_population_separated(tmp_path: Path) -> None:
    out = tmp_path / "real"
    replay = tmp_path / "replay"
    # Expand the repeated option without relying on shell quoting.
    flat: list[str] = []
    for path in INPUTS:
        flat.extend(("--input", str(path)))
    run(out, *flat)
    result = json.loads((out / "real.json").read_text())
    assert result["stratum_count"] == 23
    assert all(item["status"] == "descriptive" for item in result["strata"])
    assert all(item["seed_count"] == 3 for item in result["strata"])
    assert result["prohibited"]
    run(replay, *flat)
    for name in ("calibration.json", "real.json", "report.json"):
        assert (out / name).read_bytes() == (replay / name).read_bytes()


def test_schema_failure_and_byte_replay(tmp_path: Path) -> None:
    bad = tmp_path / "bad.jsonl"
    bad.write_text(json.dumps({"schema": "wrong", "sample_id": "x"}) + "\n")
    with np.testing.assert_raises(ValueError):
        _load_rows([bad])
    bad_report = tmp_path / "bad-report.json"
    bad_report.write_text(json.dumps({"schema": "wrong", "source_dirty": True}) + "\n")
    with np.testing.assert_raises(ValueError):
        _validate_upstream_report(bad_report)


def test_polygon_quotient_invariance_and_reflection_boundary() -> None:
    polygon = np.asarray([[2.0, 0.0], [0.7, 1.5], [-1.0, 1.0], [-1.4, -0.8], [0.0, -1.3]])
    _validate_polygon(polygon)
    angle = 0.73
    rotation = np.asarray([[np.cos(angle), -np.sin(angle)], [np.sin(angle), np.cos(angle)]])
    transformed = np.roll(polygon, 2, axis=0) @ rotation.T * 3.7 + np.asarray([11.0, -4.0])
    np.testing.assert_allclose(_canonical_vector(polygon), _canonical_vector(transformed), atol=1e-12)

    reflected = polygon * np.asarray([1.0, -1.0])
    reflected = reflected[::-1]
    _validate_polygon(reflected)
    assert not np.allclose(_canonical_vector(polygon), _canonical_vector(reflected), atol=1e-6)

    with np.testing.assert_raises(ValueError):
        _validate_polygon(polygon[::-1])
    with np.testing.assert_raises(ValueError):
        _validate_polygon(np.asarray([[0.0, 0.0], [1.0, 0.0], [0.2, 0.2], [0.0, 1.0]]))
