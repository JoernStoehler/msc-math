"""Cheap retained-artifact checks; target-free and stdlib-only."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).parent / "artifacts" / "panel-2-per-bucket"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    report = json.loads((ROOT / "report.json").read_text())
    rows = [json.loads(line) for line in (ROOT / "rows.jsonl").read_text().splitlines()]
    pairs = [json.loads(line) for line in (ROOT / "paired.jsonl").read_text().splitlines()]
    assert report["observed_rows"] == report["passed_rows"] == len(rows) == 64
    assert report["failure_rows"] == 0 and report["pair_count"] == len(pairs) == 32
    assert report["source_dirty_tracked"] is False
    assert sha256(ROOT / "rows.jsonl") == report["output_rows_sha256"]
    assert sha256(ROOT / "paired.jsonl") == report["output_paired_sha256"]
    assert sha256(ROOT / "paired.tsv") == report["output_paired_tsv_sha256"]
    assert "exact rational" in report["regeneration_note"]
    by_pair = {}
    for pair in pairs:
        assert pair["singular_spectrum_exact_equal"] is True
    for row in rows:
        assert row["exact_matrix_action_matches"] is True
        assert row["reconstruction_passed"] is True
        assert row["euclidean_checks"]["exact_volume_matches_base"] is True
        assert row["euclidean_checks"]["singular_spectrum_control"] is True
        if row["map_family"] == "symplectic-control":
            assert row["base_signature"]["symplectic_gram_upper_exact"] == row["response_signature"]["symplectic_gram_upper_exact"]
        by_pair.setdefault((row["base_id"], row["t"]), {})[row["map_family"]] = row
    assert len(by_pair) == 32
    for arms in by_pair.values():
        assert set(arms) == {"symplectic-control", "non-symplectic-anisotropy"}
        s, n = arms.values()
        assert s["squared_singular_values_exact"] == n["squared_singular_values_exact"]
        if s["t"] == "1":
            assert s["matrix_exact_row_major"] == n["matrix_exact_row_major"]
            assert s["symplectic_residual_exact"] == n["symplectic_residual_exact"] == "0/1"
        else:
            assert s["symplectic_residual_exact"] == "0/1"
            assert n["symplectic_residual_exact"] != "0/1"
    quotient = report["diagonal_quotient_control"]
    assert quotient["input_positive"] is True
    assert quotient["input_determinant_exact"] == "1/1"
    assert quotient["input_determinant_one"] is True
    assert quotient["t_squared_exact"] == "4/1"
    assert quotient["symplectic_factor_residual_exact"] == "0/1"
    assert quotient["symplectic_factor_is_exact"] is True
    assert quotient["nonidentity_factor"] is True
    assert quotient["reconstruction_passed"] is True
    print("Cartan packet checks passed: 64 rows, 32 pairs, byte hashes and controls valid")


if __name__ == "__main__":
    main()
