#!/usr/bin/env python3
"""
Benchmark EHZ capacity computation across facet counts.

Goal: Build and run the benchmark binary on random polytopes, fit an exponential
      timing model T(F) = a · b^F, produce a figure, and update the timing model.
Input: Rust benchmark binary (crates/datasets/src/bin/benchmark.rs)
Output: experiments/profiling/benchmark.csv (raw data),
        experiments/profiling/timing_model.json (updated model),
        experiments/figures/benchmark_timing.png (figure)
"""

import csv
import json
import subprocess
import sys
import time
from collections import defaultdict
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
CRATES_DIR = REPO_ROOT / "crates"
BINARY = CRATES_DIR / "target" / "release" / "benchmark"
PROFILING_DIR = REPO_ROOT / "experiments" / "profiling"
FIGURES_DIR = REPO_ROOT / "experiments" / "figures"

CSV_PATH = PROFILING_DIR / "benchmark.csv"
MODEL_PATH = PROFILING_DIR / "timing_model.json"


def build():
    """Build benchmark binary with cargo build --release."""
    print("Building benchmark binary (release)...")
    result = subprocess.run(
        ["cargo", "build", "--release", "--bin", "benchmark"],
        cwd=CRATES_DIR,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        print("Build failed:", file=sys.stderr)
        print(result.stderr, file=sys.stderr)
        sys.exit(1)
    print("Build OK.")


def run_benchmark():
    """Run benchmark binary, capturing CSV from stdout."""
    PROFILING_DIR.mkdir(parents=True, exist_ok=True)
    print(f"\nRunning benchmark -> {CSV_PATH}")
    t0 = time.time()
    result = subprocess.run(
        [str(BINARY)],
        capture_output=True,
        text=True,
    )
    elapsed = time.time() - t0
    # Print progress from stderr
    if result.stderr.strip():
        for line in result.stderr.strip().split("\n"):
            print(f"  {line}")
    if result.returncode != 0:
        print("Benchmark failed", file=sys.stderr)
        sys.exit(1)
    # Write stdout (CSV) to file
    CSV_PATH.write_text(result.stdout)
    n_data_lines = len([line for line in result.stdout.strip().split("\n") if line and not line.startswith("facets")])
    print(f"  {n_data_lines} data rows in {elapsed:.1f}s")


def load_csv():
    """Load benchmark CSV as list of dicts."""
    rows = []
    with open(CSV_PATH) as f:
        reader = csv.DictReader(f)
        for row in reader:
            rows.append({
                "facets": int(row["facets"]),
                "sample": int(row["sample"]),
                "time_ms": float(row["time_ms"]),
                "capacity": float(row["capacity"]) if row["capacity"] != "NA" else None,
                "iterations": int(row["iterations"]),
            })
    return rows


def compute_stats(rows):
    """Compute per-facet-count summary statistics."""
    by_f = defaultdict(list)
    for r in rows:
        by_f[r["facets"]].append(r["time_ms"])

    stats = []
    prev_median = None
    for f in sorted(by_f.keys()):
        times = sorted(by_f[f])
        n = len(times)
        median = times[n // 2] if n % 2 == 1 else (times[n // 2 - 1] + times[n // 2]) / 2
        mean = sum(times) / n
        growth = median / prev_median if prev_median else None
        stats.append({
            "F": f,
            "N": n,
            "median_ms": median,
            "mean_ms": mean,
            "min_ms": min(times),
            "max_ms": max(times),
            "growth": growth,
        })
        prev_median = median
    return stats


def fit_model(stats):
    """Fit exponential model T(F) = a * b^F via log-linear regression."""
    F = np.array([s["F"] for s in stats])
    log_t = np.log(np.array([s["median_ms"] / 1000.0 for s in stats]))  # seconds

    # log(T) = log(a) + F * log(b)
    A = np.column_stack([np.ones_like(F), F])
    coeffs, residuals, _, _ = np.linalg.lstsq(A, log_t, rcond=None)
    log_a, log_b = coeffs
    a = np.exp(log_a)
    b = np.exp(log_b)

    # R^2
    predicted = log_a + F * log_b
    ss_res = np.sum((log_t - predicted) ** 2)
    ss_tot = np.sum((log_t - np.mean(log_t)) ** 2)
    r_squared = 1 - ss_res / ss_tot

    return a, b, r_squared


def update_timing_model(a, b, r_squared, stats):
    """Write updated timing model to JSON."""
    model = {
        "model": {
            "type": "exponential",
            "formula": "T(F) = a * b^F",
            "parameters": {"a": a, "b": b},
            "units": "seconds",
            "source": "benchmark.py (random polytopes)",
        },
        "fit_quality": {
            "r_squared": r_squared,
            "data_points": len(stats),
            "facet_range": [stats[0]["F"], stats[-1]["F"]],
        },
    }
    MODEL_PATH.write_text(json.dumps(model, indent=2) + "\n")
    print(f"  Updated {MODEL_PATH}")


def plot_timing(rows, stats, a, b, r_squared):
    """Produce log-linear plot of timing vs facet count with fitted model."""
    FIGURES_DIR.mkdir(parents=True, exist_ok=True)

    fig, ax = plt.subplots(figsize=(8, 5))

    # Scatter individual data points
    F_all = [r["facets"] for r in rows]
    T_all = [r["time_ms"] / 1000.0 for r in rows]
    ax.scatter(F_all, T_all, alpha=0.3, s=15, color="steelblue", label="Individual samples")

    # Medians
    F_med = [s["F"] for s in stats]
    T_med = [s["median_ms"] / 1000.0 for s in stats]
    ax.scatter(F_med, T_med, s=60, color="darkblue", zorder=5, marker="D", label="Median")

    # Fitted model
    F_fit = np.linspace(min(F_all) - 0.5, max(F_all) + 0.5, 100)
    T_fit = a * b ** F_fit
    ax.plot(F_fit, T_fit, color="red", linewidth=2,
            label=f"$T(F) = {a:.2e} \\cdot {b:.2f}^F$ ($R^2 = {r_squared:.3f}$)")

    ax.set_yscale("log")
    ax.set_xlabel("Facet count $F$")
    ax.set_ylabel("Computation time (seconds)")
    ax.set_title("EHZ capacity computation time vs facet count (random polytopes)")
    ax.legend()
    ax.grid(True, alpha=0.3)
    fig.tight_layout()

    out = FIGURES_DIR / "benchmark_timing.png"
    fig.savefig(out, dpi=150)
    plt.close(fig)
    print(f"  {out.name}: {len(rows)} data points, {len(stats)} facet counts")


def print_summary(stats, a, b, r_squared):
    """Print summary table and model to stdout."""
    print("\n=== Timing Model ===")
    print(f"T(F) = {a:.2e} * {b:.2f}^F seconds (R^2 = {r_squared:.3f})")
    print(f"Growth rate: {b:.2f}x per facet")
    print()
    print(f"{'F':>3} {'N':>3} {'Median':>10} {'Mean':>10} {'Min':>10} {'Max':>10} {'Growth':>7}")
    for s in stats:
        g = f"{s['growth']:.1f}x" if s["growth"] else "-"
        print(f"{s['F']:>3} {s['N']:>3} {s['median_ms']:>10.1f} {s['mean_ms']:>10.1f} "
              f"{s['min_ms']:>10.1f} {s['max_ms']:>10.1f} {g:>7}")


def main():
    build()
    run_benchmark()

    rows = load_csv()
    print(f"\nLoaded {len(rows)} data points from {CSV_PATH}")

    stats = compute_stats(rows)
    a, b, r_squared = fit_model(stats)

    print_summary(stats, a, b, r_squared)
    update_timing_model(a, b, r_squared, stats)
    plot_timing(rows, stats, a, b, r_squared)

    print("\nDone.")


if __name__ == "__main__":
    main()
