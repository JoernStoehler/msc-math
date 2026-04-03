"""
Goal: Exploratory analysis of verify-numerics results.
Input: crates/exp-numerical-analysis/error-bounds/results_*.jsonl (CLI args)
Output: stdout (summary tables)

Bound checking and conjecture validation is in Rust tests:
  cargo test --test verify_numerics_tests

This script is for ad-hoc exploration: summaries, distributions,
identifying interesting cases for testdata/.
"""

import json
import sys
from pathlib import Path
from collections import defaultdict

EXPERIMENT_DIR = Path(__file__).resolve().parent


def load_data(paths):
    rows = []
    for path in paths:
        with open(path) as f:
            for line in f:
                rows.append(json.loads(line))
        print(f"Loaded {len(rows)} rows total (including {path})")
    return rows


def safe(v, default=float('nan')):
    if v is None:
        return default
    return v


def summary(rows):
    n = len(rows)
    if n == 0:
        print("No rows.")
        return

    # Verdict distribution
    verdicts_exact = defaultdict(int)
    verdicts_proj = defaultdict(int)
    for r in rows:
        verdicts_exact[r["verdict_exact"]] += 1
        verdicts_proj[r["verdict_projection"]] += 1

    print(f"\n{'='*60}")
    print(f"SUMMARY ({n} rows)")
    print(f"{'='*60}")

    print(f"\nExact verdicts:")
    for v, c in sorted(verdicts_exact.items()):
        print(f"  {v:20s} {c:>6d} ({100*c/n:.1f}%)")

    print(f"\nProjection verdicts:")
    for v, c in sorted(verdicts_proj.items()):
        print(f"  {v:20s} {c:>6d} ({100*c/n:.1f}%)")

    # m distribution
    by_m = defaultdict(int)
    for r in rows:
        by_m[r["m"]] += 1
    print(f"\nm distribution:")
    for m in sorted(by_m):
        print(f"  m={m}: {by_m[m]}")

    # Q errors (both feasible)
    both = [r for r in rows if r["verdict_exact"] == "feasible"
            and r["verdict_projection"] in ("true", "indeterminate")]
    if both:
        errs = [r["err_projection"] for r in both if r["err_projection"] == r["err_projection"]]
        if errs:
            errs.sort()
            print(f"\nProjection Q errors (n={len(errs)}):")
            print(f"  median: {errs[len(errs)//2]:.2e}")
            print(f"  p99:    {errs[int(len(errs)*0.99)]:.2e}")
            print(f"  max:    {errs[-1]:.2e}")

    # Beta errors
    beta_errs = [r["beta_err_projection"] for r in both
                 if r["beta_err_projection"] == r["beta_err_projection"]]
    if beta_errs:
        beta_errs.sort()
        print(f"\nProjection beta errors (n={len(beta_errs)}):")
        print(f"  median: {beta_errs[len(beta_errs)//2]:.2e}")
        print(f"  max:    {beta_errs[-1]:.2e}")

    # Eta bound summary
    eta_max_vals = [r["proj_eta_max"] for r in both
                    if r.get("proj_eta_max", float('nan')) == r.get("proj_eta_max", float('nan'))
                    and r.get("proj_eta_max", float('inf')) < float('inf')]
    if eta_max_vals:
        eta_max_vals.sort()
        print(f"\nEta bound max (finite only, n={len(eta_max_vals)}):")
        print(f"  median: {eta_max_vals[len(eta_max_vals)//2]:.2e}")
        print(f"  max:    {eta_max_vals[-1]:.2e}")

    eta_ratios = [r["proj_eta_ratio"] for r in both
                  if r.get("proj_eta_ratio", float('nan')) == r.get("proj_eta_ratio", float('nan'))
                  and r.get("proj_eta_ratio", 0) > 0]
    if eta_ratios:
        eta_ratios.sort()
        print(f"\nEta ratio (actual_err/eta, n={len(eta_ratios)}):")
        print(f"  median: {eta_ratios[len(eta_ratios)//2]:.4f}")
        print(f"  max:    {eta_ratios[-1]:.4f}")
        n_violations = sum(1 for r in eta_ratios if r > 1.0)
        if n_violations:
            print(f"  VIOLATIONS (ratio > 1): {n_violations}")

    # Certified margin
    cert = [r["proj_certified_margin"] for r in both
            if r.get("proj_certified_margin", float('nan')) == r.get("proj_certified_margin", float('nan'))
            and r.get("proj_certified_margin", float('-inf')) > float('-inf')]
    if cert:
        n_certified = sum(1 for c in cert if c > 0)
        print(f"\nCertified beta > 0: {n_certified}/{len(cert)}")

    # Eigendirection scaling
    cases_with_da = [r for r in both if r.get("proj_delta_alpha") and r.get("proj_eigenvalues")]
    if cases_with_da:
        eps = 2.22e-16
        ratios = []
        for r in cases_with_da:
            for da, gamma in zip(r["proj_delta_alpha"], r["proj_eigenvalues"]):
                if abs(gamma) > r.get("proj_eps_gamma", 0):
                    ratios.append(da * abs(gamma) / eps)
        if ratios:
            ratios.sort()
            print(f"\nEigendirection scaling |da|*|gamma|/eps_mach (n={len(ratios)}):")
            print(f"  median: {ratios[len(ratios)//2]:.2f}")
            print(f"  max:    {ratios[-1]:.2f}")


def main():
    if len(sys.argv) < 2:
        print(f"Usage: python3 {sys.argv[0]} <results1.jsonl> [results2.jsonl ...]")
        sys.exit(1)

    rows = load_data(sys.argv[1:])
    summary(rows)


if __name__ == "__main__":
    main()
