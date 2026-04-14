#!/usr/bin/env python3
"""
Analyze orbit recovery results across polytope dataset.

Goal: Validate base point recovery algorithm across known + random polytopes.
      Identify edge cases, distribution of solution dimension, violation levels.
Input: experiments/verification/orbit-recovery/orbit-recovery.jsonl
Output: Summary statistics printed to stdout.
"""

import json
import sys
from pathlib import Path
from collections import Counter

EXPERIMENT_DIR = Path(__file__).resolve().parent
DATA_FILE = EXPERIMENT_DIR / "orbit-recovery.jsonl"


def load_data():
    if not DATA_FILE.exists():
        print(f"File not found: {DATA_FILE}")
        print("Run Rust binary first: cargo run -p dev-capacity-validation --release --bin axioms-orbit-recovery")
        sys.exit(1)

    rows = []
    with open(DATA_FILE) as f:
        for line in f:
            line = line.strip()
            if line:
                rows.append(json.loads(line))
    return rows


def print_summary(rows):
    n = len(rows)
    known = [r for r in rows if r["source"] == "known"]
    random = [r for r in rows if r["source"] == "random"]

    print(f"{'='*60}")
    print(f"Orbit Recovery Validation Summary")
    print(f"{'='*60}")
    print(f"Total polytopes: {n} ({len(known)} known, {len(random)} random)")
    print()

    # Overall pass/fail
    tol_eq = 1e-8
    tol_ineq = 1e-6
    n_pass = sum(
        1 for r in rows
        if r["closure_error"] < tol_eq
        and r["on_facet_error"] < tol_eq
        and r["max_violation"] < tol_ineq
        and r["action_error"] < tol_eq
    )
    n_fail = n - n_pass
    print(f"Pass: {n_pass}/{n} ({100*n_pass/n:.1f}%)")
    print(f"Fail: {n_fail}/{n}")
    print()

    # Solution dimension distribution
    print("Solution dimension distribution (dim = 4 - rank(N_S)):")
    dim_counts = Counter(r["solution_dim"] for r in rows)
    for dim in sorted(dim_counts.keys()):
        pct = 100 * dim_counts[dim] / n
        print(f"  dim={dim}: {dim_counts[dim]:4d} ({pct:5.1f}%)")
    print()

    # Active facets distribution
    print("Active facets distribution:")
    active_counts = Counter(r["active_facets"] for r in rows)
    for af in sorted(active_counts.keys()):
        pct = 100 * active_counts[af] / n
        print(f"  active={af}: {active_counts[af]:4d} ({pct:5.1f}%)")
    print()

    # Error statistics
    print("Error statistics (across all polytopes):")
    for field, label in [
        ("closure_error", "Closure error"),
        ("on_facet_error", "On-facet error"),
        ("max_violation", "Max violation"),
        ("action_error", "Action error"),
    ]:
        vals = [r[field] for r in rows]
        print(f"  {label:20s}: max={max(vals):.2e}, median={sorted(vals)[len(vals)//2]:.2e}")
    print()

    # Known polytopes detail
    print("Known polytopes detail:")
    print(f"  {'Name':35s} {'F':>3s} {'dim':>4s} {'violation':>12s} {'closure':>12s} {'action_err':>12s}")
    for r in known:
        print(
            f"  {r['name']:35s} {r['facet_count']:3d} {r['solution_dim']:4d} "
            f"{r['max_violation']:12.2e} {r['closure_error']:12.2e} {r['action_error']:12.2e}"
        )
    print()

    # Failures detail
    failures = [
        r for r in rows
        if r["closure_error"] >= tol_eq
        or r["on_facet_error"] >= tol_eq
        or r["max_violation"] >= tol_ineq
        or r["action_error"] >= tol_eq
    ]
    if failures:
        print(f"FAILURES ({len(failures)}):")
        for r in failures:
            print(f"  {r['name']}: viol={r['max_violation']:.2e}, "
                  f"close={r['closure_error']:.2e}, "
                  f"action_err={r['action_error']:.2e}")
    else:
        print("No failures. All orbits validated successfully.")
    print()

    # Timing
    print("Timing:")
    cap_times = [r["time_capacity_ms"] for r in rows]
    rec_times = [r["time_recovery_ms"] for r in rows]
    print(f"  Capacity computation: total={sum(cap_times)/1000:.1f}s, "
          f"mean={sum(cap_times)/len(cap_times):.1f}ms")
    print(f"  Recovery:             total={sum(rec_times)/1000:.1f}s, "
          f"mean={sum(rec_times)/len(rec_times):.3f}ms")


def main():
    rows = load_data()
    print_summary(rows)


if __name__ == "__main__":
    main()
