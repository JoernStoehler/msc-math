#!/usr/bin/env python3
"""
Analyze orbit recovery results across polytope dataset.

Goal: Summarize the curated orbit-recovery validation dataset.
      Identify pass/fail counts, resolution-path coverage, and worst margins.
Input: experiments/verification/orbit-recovery/orbit-recovery.jsonl
Output: Summary statistics printed to stdout.
"""

import json
import sys
from pathlib import Path
from collections import Counter

EXPERIMENT_DIR = Path(__file__).resolve().parent
DATA_FILE = EXPERIMENT_DIR / "orbit-recovery.jsonl"
GEOMETRY_TOL = 1e-6
ACTION_TOL = 1e-5


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
    families = Counter(r["family"] for r in rows)
    known = [r for r in rows if r["family"] == "known"]
    random = [r for r in rows if r["family"] == "random"]
    lagrangian = [r for r in rows if r["family"] == "lagrangian_product"]

    print(f"{'='*60}")
    print(f"Orbit Recovery Validation Summary")
    print(f"{'='*60}")
    print(f"Total polytopes: {n}")
    for family in sorted(families):
        print(f"  {family:20s} {families[family]:4d}")
    print()

    # Overall pass/fail
    n_pass = sum(
        1 for r in rows
        if r["closure_error"] < GEOMETRY_TOL
        and r["on_facet_error"] < GEOMETRY_TOL
        and r["inside_k_error"] < GEOMETRY_TOL
        and r["action_error"] < ACTION_TOL
    )
    n_fail = n - n_pass
    print(f"Pass: {n_pass}/{n} ({100*n_pass/n:.1f}%)")
    print(f"Fail: {n_fail}/{n}")
    print()

    print("Resolution path distribution:")
    resolution_counts = Counter(r["resolution"] for r in rows)
    for resolution in sorted(resolution_counts.keys()):
        pct = 100 * resolution_counts[resolution] / n
        print(f"  {resolution:20s} {resolution_counts[resolution]:4d} ({pct:5.1f}%)")
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
    print(f"  {'Name':35s} {'F':>3s} {'dim':>4s} {'resolution':>14s} {'violation':>12s} {'closure':>12s} {'action_err':>12s}")
    for r in known:
        print(
            f"  {r['name']:35s} {r['facet_count']:3d} {r['solution_dim']:4d} {r['resolution']:>14s} "
            f"{r['max_violation']:12.2e} {r['closure_error']:12.2e} {r['action_error']:12.2e}"
        )
    print()

    print(f"Random rows: {len(random)}")
    print(f"Lagrangian-product rows: {len(lagrangian)}")
    print()

    # Failures detail
    failures = [
        r for r in rows
        if r["closure_error"] >= GEOMETRY_TOL
        or r["on_facet_error"] >= GEOMETRY_TOL
        or r["inside_k_error"] >= GEOMETRY_TOL
        or r["action_error"] >= ACTION_TOL
    ]
    if failures:
        print(f"FAILURES ({len(failures)}):")
        for r in failures:
            print(f"  {r['name']} [{r['family']}|{r['resolution']}]: viol={r['inside_k_error']:.2e}, "
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
