#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib"]
# ///

"""Profile the default test suite and identify slow tests.

Goal: Identify hot paths in the test suite and track performance over time.
Input Artifacts: No data file; runs `cargo test --lib` from `library/`.
Output Artifacts: profile.jsonl, logbook.jsonl, test_timing.png.

Pipeline:
1. Run full suite to get wall/CPU time
2. Run individual "candidate slow" tests for per-test timing
3. Write results to profile.jsonl and append to logbook.jsonl
4. Generate test_timing.png figure
"""

import json
import os
import subprocess
import sys
import time
from datetime import date
from pathlib import Path

# This experiment is nested one level deeper than experiments/<topic>/<experiment>.
REPO_ROOT = Path(__file__).resolve().parents[4]
CRATE_DIR = REPO_ROOT / "library"
OUT_DIR = Path(__file__).resolve().parent
PROFILE_JSONL = OUT_DIR / "profile.jsonl"
LOGBOOK_JSONL = OUT_DIR / "logbook.jsonl"
FIGURE_PATH = OUT_DIR / "test_timing.png"

# Tests known to be potentially slow. Updated manually when the test suite changes.
# An agent updating this experiment should review and extend this list.
CANDIDATE_SLOW_TESTS = [
    "algorithms::hk2017::tests_literature::simplex_capacity",
    "algorithms::hk2017::tests_literature::hypercube_capacity",
    "algorithms::hk2017::tests_literature::lagrangian_triangle_product_capacity",
    "algorithms::hk2017::tests_literature::triangle_square_capacity",
    "algorithms::hk2017::tests_literature::symplectic_triangle_square_capacity",
    "algorithms::hk2017::tests_literature::billiard_agrees_with_hk2017_on_small_lagrangian_products",
    "algorithms::hk2017::orbit_recovery::tests::hko_pentagon_recovery",
    "algorithms::hk2017::orbit_recovery::tests::hypercube_recovery",
    "algorithms::hk2017::orbit_recovery::tests::dwell_times_positive",
    "algorithms::hk2017::orbit_recovery::tests::breakpoint_count_consistency",
    "algorithms::hk2017::tests_conformality::capacity_conformality_simplex",
    "algorithms::hk2017::tests_symplectic_invariance::capacity_symplectomorphism_invariance_simplex",
    "algorithms::hk2017::tests_pruning::pruned_matches_unpruned_simplex",
    "geom::volume::tests::proptests::volume_scales_with_fourth_power",
    "random::tests::proptests::random_polytopes_pass_validation",
]


def get_commit_hash() -> str:
    """Get short commit hash of HEAD."""
    result = subprocess.run(
        ["git", "rev-parse", "--short", "HEAD"],
        capture_output=True, text=True, cwd=CRATE_DIR
    )
    if result.returncode != 0:
        print(f"Warning: could not get commit hash: {result.stderr.strip()}")
        return "unknown"
    return result.stdout.strip()


def get_cpu_count() -> int:
    """Get available CPU count."""
    return os.cpu_count() or 1


def run_full_suite() -> dict:
    """Run cargo test --lib and capture wall + CPU time."""
    print("Running full test suite...")
    start = time.monotonic()
    result = subprocess.run(
        ["bash", "-c", "time cargo test --lib"],
        capture_output=True, text=True, cwd=CRATE_DIR, timeout=600
    )
    wall = time.monotonic() - start

    # Parse test count from output
    n_passed = 0
    n_ignored = 0
    for line in (result.stdout + result.stderr).splitlines():
        if "test result:" in line:
            parts = line.split()
            for i, p in enumerate(parts):
                if p == "passed;":
                    n_passed = int(parts[i - 1])
                if p == "ignored;":
                    n_ignored = int(parts[i - 1])

    # Parse CPU time from bash 'time' output
    cpu_user = 0.0
    for line in result.stderr.splitlines():
        if line.strip().startswith("user"):
            # Parse "Xm Y.ZZZs" format
            time_str = line.split()[-1]
            if "m" in time_str:
                mins, secs = time_str.replace("s", "").split("m")
                cpu_user = float(mins) * 60 + float(secs)
            else:
                cpu_user = float(time_str.replace("s", ""))

    return {
        "wall_s": round(wall, 2),
        "cpu_s": round(cpu_user, 2),
        "n_passed": n_passed,
        "n_ignored": n_ignored,
    }


def time_individual_test(test_name: str) -> float:
    """Run a single test and return its wall-clock duration in seconds."""
    result = subprocess.run(
        ["cargo", "test", "--lib", "--", test_name],
        capture_output=True, text=True, cwd=CRATE_DIR, timeout=300
    )
    # Parse "finished in X.XXs" from output
    for line in (result.stdout + result.stderr).splitlines():
        if "finished in" in line:
            parts = line.split("finished in")
            time_str = parts[-1].strip().rstrip("s")
            try:
                return float(time_str)
            except ValueError:
                pass
    print(f"Warning: could not parse timing for {test_name}")
    return -1.0


def profile_candidates() -> list[dict]:
    """Time each candidate slow test individually (sequential, no contention)."""
    results = []
    print(f"Profiling {len(CANDIDATE_SLOW_TESTS)} candidate slow tests...")
    for i, test in enumerate(CANDIDATE_SLOW_TESTS):
        short = test.split("::")[-1]
        duration = time_individual_test(test)
        results.append({"test": test, "duration_s": round(duration, 3)})
        status = f"{duration:.2f}s" if duration >= 0 else "FAILED"
        print(f"  [{i+1}/{len(CANDIDATE_SLOW_TESTS)}] {short}: {status}")
    return results


def generate_figure(per_test: list[dict]):
    """Generate a bar chart of test durations."""
    try:
        import matplotlib
        matplotlib.use("Agg")
        import matplotlib.pyplot as plt
        sys.path.insert(0, str(REPO_ROOT / "experiments"))
        from figure_config import setup, FIGSIZE_SINGLE
        setup()
    except ImportError:
        print("matplotlib not available, skipping figure generation")
        return

    # Sort by duration, show top 15
    sorted_tests = sorted(per_test, key=lambda x: x["duration_s"], reverse=True)
    top = sorted_tests[:15]

    names = [t["test"].split("::")[-1] for t in top]
    durations = [t["duration_s"] for t in top]

    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
    ax.barh(range(len(names)), durations, color="#4C72B0")
    ax.set_yticks(range(len(names)))
    ax.set_yticklabels(names)
    ax.set_xlabel("Duration (seconds)")
    ax.set_title("Test Suite: Slowest Tests (sequential, no contention)")
    ax.invert_yaxis()

    plt.tight_layout()
    plt.savefig(str(FIGURE_PATH))
    print(f"Figure saved: {FIGURE_PATH}")
    plt.close()


def main():
    print(f"Crate directory: {CRATE_DIR}")
    print(f"Output directory: {OUT_DIR}")
    print()

    # Step 1: Full suite
    suite = run_full_suite()
    print(f"Full suite: {suite['wall_s']}s wall, {suite['cpu_s']}s CPU, "
          f"{suite['n_passed']} passed, {suite['n_ignored']} ignored")
    print()

    # Step 2: Per-test profiling
    per_test = profile_candidates()
    print()

    # Step 3: Write profile.jsonl
    with open(PROFILE_JSONL, "w") as f:
        for entry in sorted(per_test, key=lambda x: -x["duration_s"]):
            f.write(json.dumps(entry) + "\n")
    print(f"Per-test results: {PROFILE_JSONL}")

    # Step 4: Append to logbook
    top5 = sorted(per_test, key=lambda x: -x["duration_s"])[:5]
    logbook_entry = {
        "date": str(date.today()),
        "commit": get_commit_hash(),
        "wall_s": suite["wall_s"],
        "cpu_s": suite["cpu_s"],
        "n_tests": suite["n_passed"],
        "cores": get_cpu_count(),
        "top5": [{"test": t["test"].split("::")[-1], "s": t["duration_s"]} for t in top5],
    }
    with open(LOGBOOK_JSONL, "a") as f:
        f.write(json.dumps(logbook_entry) + "\n")
    print(f"Logbook appended: {LOGBOOK_JSONL}")

    # Step 5: Figure
    generate_figure(per_test)

    # Summary
    print()
    print("=== Top 5 slowest tests ===")
    for t in top5:
        print(f"  {t['duration_s']:7.2f}s  {t['test'].split('::')[-1]}")


if __name__ == "__main__":
    main()
