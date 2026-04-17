#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# ///

"""
Goal: Summarize the all-minimum sigma-validation dataset.
Input: experiments/verification/all-minimum/all-minimum.jsonl and
       experiments/verification/all-minimum/all-minimum-orbits.jsonl
Output: Summary statistics printed to stdout.
"""

import json
import sys
from collections import Counter
from pathlib import Path

EXPERIMENT_DIR = Path(__file__).resolve().parent


def parse_paths():
    smoke = len(sys.argv) > 1 and sys.argv[1] == "--smoke"
    if len(sys.argv) > 2 or (len(sys.argv) == 2 and not smoke):
        print("Usage: uv run analyze.py [--smoke]")
        sys.exit(2)
    prefix = "smoke-" if smoke else ""
    return (
        smoke,
        EXPERIMENT_DIR / f"{prefix}all-minimum.jsonl",
        EXPERIMENT_DIR / f"{prefix}all-minimum-orbits.jsonl",
    )


def load_jsonl(path):
    if not path.exists():
        print(f"File not found: {path}")
        sys.exit(1)
    rows = []
    with open(path) as f:
        for line in f:
            line = line.strip()
            if line:
                rows.append(json.loads(line))
    return rows


def main():
    smoke, summary_path, detail_path = parse_paths()
    summary_rows = load_jsonl(summary_path)
    detail_rows = load_jsonl(detail_path)

    print("=" * 72)
    print(f"All-Minimum Summary ({'smoke' if smoke else 'full'})")
    print("=" * 72)
    print(f"Total polytopes: {len(summary_rows)}")

    families = Counter(row["family"] for row in summary_rows)
    print("Families:")
    for family in sorted(families):
        print(f"  {family:24s} {families[family]:4d}")
    print()

    passed = sum(1 for row in summary_rows if row["passes_validation"])
    print(f"Pass: {passed}/{len(summary_rows)} ({100 * passed / max(len(summary_rows), 1):.1f}%)")
    print()

    ok_rows = [row for row in summary_rows if row["status"] == "ok"]
    if ok_rows:
        orbit_counts = [row["min_orbit_count"] for row in ok_rows]
        spreads = [row["observed_action_spread"] for row in ok_rows]
        intervals = [row["interval_width"] for row in ok_rows]
        print("Minimum-set summary:")
        print(
            f"  min orbits / polytope: min={min(orbit_counts)}, "
            f"median={sorted(orbit_counts)[len(orbit_counts)//2]}, max={max(orbit_counts)}"
        )
        print(f"  widest observed spread: {max(spreads):.2e}")
        print(f"  widest raw candidate interval: {max(intervals):.2e}")
        print(
            f"  exact-resolved minima: total={sum(row['admissible_exact_count'] for row in ok_rows)}, "
            f"max={max(row['admissible_exact_count'] for row in ok_rows)}"
        )
        print()

    if detail_rows:
        admissibility = Counter(row["admissibility"] for row in detail_rows)
        print("Per-orbit admissibility:")
        for key in sorted(admissibility):
            print(f"  {key:24s} {admissibility[key]:4d}")
        print(f"  Trusted minimum-orbit rows: {len(detail_rows)}")
        print()

    expected = [row for row in summary_rows if row.get("expected_min_orbit_count") is not None]
    if expected:
        print("Expected multiplicity checks:")
        for row in expected:
            status = "OK" if row.get("count_matches_expected") else "FAIL"
            print(
                f"  {row['name']:24s} expected={row['expected_min_orbit_count']:3d} "
                f"got={row['min_orbit_count']:3d} {status}"
            )
        print()

    failed = [row for row in summary_rows if not row["passes_validation"]]
    if failed:
        print("Failures:")
        for row in failed:
            reasons = "; ".join(row.get("failure_reasons", [])) or "no reason recorded"
            print(f"  {row['name']} [{row.get('failure_stage', 'unknown')}]: {reasons}")
    else:
        print("No failures. All selected polytopes passed.")


if __name__ == "__main__":
    main()
