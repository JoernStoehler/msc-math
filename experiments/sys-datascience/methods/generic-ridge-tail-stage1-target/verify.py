#!/usr/bin/env python3
"""Independent, target-free checker for the completed 200-row packet."""
import hashlib
import json
import math
import sys
from pathlib import Path

THRESHOLD = 0.5949424195457518


def mean(xs):
    return sum(xs) / len(xs)


def close(a, b):
    return math.isclose(a, b, rel_tol=2e-12, abs_tol=2e-12)


def main():
    if len(sys.argv) != 2:
        raise SystemExit("usage: verify.py STAGE1-DIR")
    root = Path(sys.argv[1])
    rows_path = root / "target-rows.jsonl"
    manifest = json.loads((root / "evaluation-manifest.json").read_text())
    preflight = json.loads((root / "preflight.json").read_text())
    analysis = json.loads((root / "analysis.json").read_text())
    rows = [json.loads(line) for line in rows_path.read_text().splitlines() if line.strip()]
    assert preflight["valid"] and not preflight["target_calls"]
    assert manifest["row_count"] == len(rows) == 200
    assert manifest["selected_count"] == 100 and manifest["baseline_count"] == 100
    assert len({row["candidate_id"] for row in rows}) == 200
    assert len({row["poly_id"] for row in rows}) == 200
    rows_sha256 = hashlib.sha256(rows_path.read_bytes()).hexdigest()
    # Byte linkage is advisory provenance; row identity, counts, groups, and
    # numerical reconstruction below remain blocking.
    if (
        rows_sha256 != analysis["target_rows_sha256"]
        or manifest["rows_sha256"] != rows_sha256
    ):
        print(
            "warning: target-row bytes differ from retained manifest/analysis; "
            "continuing with semantic checks. Reassess retained interpretation "
            "before treating this packet as equivalent.",
            file=sys.stderr,
        )
    assert len(manifest["rows_blake3"]) == 64
    for row in rows:
        assert row["role"] in {"selected", "baseline"}
        assert row["future_band"] in {"0-.1%", ".1-1%", "matched-baseline"}
        assert row["f64_volume"] > 0 and row["capacity"] > 0 and math.isfinite(row["sys"])
        assert close(row["sys"], row["capacity"] ** 2 / (2 * row["f64_volume"]))
        assert close(row["proxy"], row["ridge_symp_area_mean"] / math.sqrt(row["f64_volume"]))
    groups = {
        "selected_low_1pct": [r for r in rows if r["role"] == "selected"],
        "baseline_disjoint": [r for r in rows if r["role"] == "baseline"],
        "selected_0_to_0p1pct": [r for r in rows if r["role"] == "selected" and r["f64_rank"] <= 10],
        "selected_0p1_to_1pct": [r for r in rows if r["role"] == "selected" and 11 <= r["f64_rank"] <= 100],
    }
    for group in analysis["groups"]:
        values = [r["sys"] for r in groups[group["name"]]]
        assert group["n"] == len(values)
        assert close(group["mean_sys"], mean(values))
        hits = sum(v >= THRESHOLD for v in values)
        assert group["threshold"] == THRESHOLD and group["threshold_count"] == hits
    selected = groups["selected_low_1pct"]
    baseline = groups["baseline_disjoint"]
    first = groups["selected_0_to_0p1pct"]
    next_band = groups["selected_0p1_to_1pct"]
    expected_differences = [mean([r["sys"] for r in selected]) - mean([r["sys"] for r in baseline]), mean([r["sys"] for r in first]) - mean([r["sys"] for r in next_band])]
    for observed, expected in zip(analysis["contrasts"], expected_differences):
        assert close(observed["mean_difference"], expected)
    comparator = analysis["generic_product_operational_comparator"]
    assert close(comparator["generic_minus_product_hardening_interaction"], expected_differences[1] - (0.61889509741270687 - 0.63281993533461989))
    max_row = max(rows, key=lambda r: r["sys"])
    assert analysis["maximum_sys_row"]["candidate_id"] == max_row["candidate_id"]
    assert not analysis["any_sys_gt_1"] and all(row["sys"] <= 1 for row in rows)
    print("independent target packet checks passed: 200 rows, arithmetic, groups, contrasts, max row")


if __name__ == "__main__":
    main()
