#!/usr/bin/env python3
"""
Run Rust dataset generation binary and summarize results.

Goal: Build and run the datasets binary, then print summary statistics.
Input: Rust source code in crates/ (compiled with cargo build --release).
Output: experiments/data/polytopes.jsonl, experiments/data/acceptance.jsonl
"""
import json
import subprocess
import sys
import time
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent.parent
CRATES_DIR = REPO_ROOT / "crates"
BINARY = CRATES_DIR / "target" / "release" / "datasets"
DATA_DIR = REPO_ROOT / "experiments" / "data"

POLYTOPE_OUTPUT = DATA_DIR / "polytopes.jsonl"
SWEEP_OUTPUT = DATA_DIR / "acceptance.jsonl"


def build():
    """Build datasets binary with cargo build --release."""
    print("Building datasets binary (release)...")
    result = subprocess.run(
        ["cargo", "build", "--release", "--bin", "datasets"],
        cwd=CRATES_DIR,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        print("Build failed:", file=sys.stderr)
        print(result.stderr, file=sys.stderr)
        sys.exit(1)
    print("Build OK.")


def run_dataset():
    """Run 'datasets dataset' subcommand to generate polytope JSONL."""
    DATA_DIR.mkdir(parents=True, exist_ok=True)
    print(f"\nGenerating polytope dataset -> {POLYTOPE_OUTPUT}")
    t0 = time.time()
    result = subprocess.run(
        [str(BINARY), "dataset", str(POLYTOPE_OUTPUT)],
        capture_output=True,
        text=True,
    )
    elapsed = time.time() - t0
    print(result.stderr.strip())
    if result.returncode != 0:
        print("dataset command failed", file=sys.stderr)
        sys.exit(1)
    print(f"  Elapsed: {elapsed:.1f}s")


def run_sweep():
    """Run 'datasets sweep' subcommand to generate acceptance sweep JSONL."""
    DATA_DIR.mkdir(parents=True, exist_ok=True)
    print(f"\nRunning acceptance sweep -> {SWEEP_OUTPUT}")
    t0 = time.time()
    result = subprocess.run(
        [str(BINARY), "sweep", str(SWEEP_OUTPUT)],
        capture_output=True,
        text=True,
    )
    elapsed = time.time() - t0
    print(result.stderr.strip())
    if result.returncode != 0:
        print("sweep command failed", file=sys.stderr)
        sys.exit(1)
    print(f"  Elapsed: {elapsed:.1f}s")


def load_jsonl(path):
    """Load a JSONL file as a list of dicts."""
    with open(path) as f:
        return [json.loads(line) for line in f if line.strip()]


def summarize_polytopes(rows):
    """Print summary statistics for the polytope dataset."""
    print(f"\n=== Polytope Dataset: {len(rows)} rows ===")
    sources = {}
    for r in rows:
        s = r["source"]
        sources[s] = sources.get(s, 0) + 1
    print("Sources:")
    for s, n in sorted(sources.items()):
        print(f"  {s}: {n}")

    facet_counts = [r["facet_count"] for r in rows]
    print(f"Facet counts: min={min(facet_counts)}, max={max(facet_counts)}")

    sys_vals = [r["sys"] for r in rows]
    print(f"Systolic ratio: min={min(sys_vals):.4f}, max={max(sys_vals):.4f}")

    vol_times = [r["time_volume_ms"] for r in rows]
    cap_times = [r["time_capacity_ms"] for r in rows]
    print(f"Avg volume time: {sum(vol_times)/len(vol_times):.3f} ms")
    print(f"Avg capacity time: {sum(cap_times)/len(cap_times):.3f} ms")


def summarize_sweep(rows):
    """Print formatted table of acceptance sweep results."""
    print(f"\n=== Acceptance Sweep: {len(rows)} configs ===")
    print(f"{'F':>3} {'h_min':>6} {'h_max':>6} {'accepted':>10} {'ratio':>8}")
    for r in rows:
        print(
            f"{r['facet_count']:>3} "
            f"{r['h_min']:>6.1f} "
            f"{r['h_max']:>6.1f} "
            f"{r['n_accepted']:>10}/{r['n_total']} "
            f"{r['acceptance_ratio']:>8.4f}"
        )


def main():
    build()
    run_dataset()
    run_sweep()

    polytopes = load_jsonl(POLYTOPE_OUTPUT)
    summarize_polytopes(polytopes)

    sweep = load_jsonl(SWEEP_OUTPUT)
    summarize_sweep(sweep)


if __name__ == "__main__":
    main()
