#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# ///

"""
Goal: Summarize geometric recovery results for trusted minimum sigma rows.
Input: experiments/verification/orbit-recovery/orbit-recovery.jsonl and
       experiments/verification/orbit-recovery/orbit-recovery-orbits.jsonl
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
        EXPERIMENT_DIR / f"{prefix}orbit-recovery.jsonl",
        EXPERIMENT_DIR / f"{prefix}orbit-recovery-orbits.jsonl",
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
    print(f"Orbit Recovery Summary ({'smoke' if smoke else 'full'})")
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
        print("Recovery summary:")
        trusted_counts = [row["trusted_min_orbit_count"] for row in ok_rows]
        print(
            f"  trusted orbits / polytope: min={min(trusted_counts)}, "
            f"median={sorted(trusted_counts)[len(trusted_counts)//2]}, max={max(trusted_counts)}"
        )
        print(f"  worst sigma-action rebuild error: {max(row['worst_sigma_action_error'] for row in ok_rows):.2e}")
        print(f"  worst closure error:            {max(row['worst_closure_error'] for row in ok_rows):.2e}")
        print(f"  worst on-facet error:           {max(row['worst_on_facet_error'] for row in ok_rows):.2e}")
        print(f"  worst inside-K error:           {max(row['worst_inside_k_error'] for row in ok_rows):.2e}")
        print(f"  worst action error:             {max(row['worst_action_error'] for row in ok_rows):.2e}")
        print()
        print("Timing:")
        print(
            f"  Rebuild one-sigma KKT: total={sum(row['time_rebuild_ms'] for row in ok_rows)/1000:.2f}s, "
            f"mean={sum(row['time_rebuild_ms'] for row in ok_rows)/len(ok_rows):.1f}ms"
        )
        print(
            f"  Geometric recovery:    total={sum(row['time_recovery_ms'] for row in ok_rows)/1000:.2f}s, "
            f"mean={sum(row['time_recovery_ms'] for row in ok_rows)/len(ok_rows):.1f}ms"
        )
        print()

    if detail_rows:
        statuses = Counter(row["recovery_status"] for row in detail_rows)
        print("Per-orbit recovery statuses:")
        for status in sorted(statuses):
            print(f"  {status:24s} {statuses[status]:4d}")
        valid = sum(1 for row in detail_rows if row.get("passes_geometric_checks") is True)
        invalid = sum(1 for row in detail_rows if row.get("passes_geometric_checks") is False)
        print(f"  Geometrically valid recovered orbits:   {valid}")
        print(f"  Geometrically invalid recovered orbits: {invalid}")
        print()

    failed = [row for row in summary_rows if not row["passes_validation"]]
    if failed:
        print("Failures:")
        for row in failed:
            reasons = "; ".join(row.get("failure_reasons", [])) or "no reason recorded"
            print(f"  {row['name']} [{row.get('failure_stage', 'unknown')}]: {reasons}")
    else:
        print("No failures. All trusted minimum orbits passed recovery.")


if __name__ == "__main__":
    main()
