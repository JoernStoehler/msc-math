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
    by_pair = {}
    for row in rows:
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
    assert report["diagonal_quotient_control"]["reconstruction_passed"] is True
    print("Cartan packet checks passed: 64 rows, 32 pairs, byte hashes and controls valid")


if __name__ == "__main__":
    main()
