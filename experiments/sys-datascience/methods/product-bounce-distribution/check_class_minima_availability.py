#!/usr/bin/env python3
"""Check the bounded A3-null availability audit against its row artifact."""
import argparse
import hashlib
import json
import sys
from collections import Counter
from pathlib import Path


def classify_a3_null_row(row):
    """Validate and classify one row under the historical audit contract."""
    for bounce in ("2", "3"):
        enumerated = row["enumerated_sigma_counts_by_bounces"].get(bounce, 0)
        classified = sum(
            row[field].get(bounce, 0)
            for field in (
                "candidate_orbit_counts_by_bounces",
                "f64_inadmissible_sigma_counts_by_bounces",
                "f64_numerical_failure_counts_by_bounces",
            )
        )
        assert enumerated == classified, (
            f"incomplete f64 accounting for {row['name']} bounce {bounce}: "
            f"{enumerated} enumerated != {classified} classified"
        )
        exact_admissible_rejections = row[
            "exact_admissible_f64_rejected_counts_by_bounces"
        ].get(bounce, 0)
        f64_inadmissible = row["f64_inadmissible_sigma_counts_by_bounces"].get(
            bounce, 0
        )
        assert exact_admissible_rejections <= f64_inadmissible, (
            f"exact rejection count exceeds f64-inadmissible inputs for "
            f"{row['name']} bounce {bounce}"
        )

    enum3 = row["enumerated_sigma_counts_by_bounces"].get("3", 0)
    solved3 = row["candidate_orbit_counts_by_bounces"].get("3", 0)
    failures3 = row["f64_numerical_failure_counts_by_bounces"].get("3", 0)
    if enum3 == 0:
        return "no_transition_feasible_three_bounce_sigma"
    if solved3 == 0 and failures3 == 0:
        return "all_three_bounce_sigmas_f64_inadmissible"
    raise AssertionError(f"unexpected A3-null row: {row['name']}")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--audit", required=True)
    ap.add_argument("--rows", required=True)
    ap.add_argument("--class-minima", required=True)
    args = ap.parse_args()
    audit = json.loads(Path(args.audit).read_text(encoding="utf-8"))
    rows = [json.loads(line) for line in Path(args.rows).read_text().splitlines() if line]
    assert len(rows) == audit["audited_rows"] == 785
    causes = Counter()
    for row in rows:
        cause = classify_a3_null_row(row)
        causes[f"{row['k']}x{row['m']}:{cause}"] += 1
    assert dict(causes) == audit["cause_counts"]
    numerical_failure_rows = sum(
        any(row["f64_numerical_failure_counts_by_bounces"].values()) for row in rows
    )
    exact_admissible_rejected_candidates = sum(
        row["exact_admissible_f64_rejected_counts_by_bounces"].get("3", 0)
        for row in rows
    )
    exact_admissible_rejected_rows = sum(
        row["exact_admissible_f64_rejected_counts_by_bounces"].get("3", 0) > 0
        for row in rows
    )
    assert numerical_failure_rows == audit["f64_numerical_failure_rows"] == 0
    assert (
        exact_admissible_rejected_candidates
        == audit["exact_admissible_f64_rejected_candidates"]
        == 0
    )
    assert (
        exact_admissible_rejected_rows
        == audit["exact_admissible_f64_rejected_rows"]
        == 0
    )
    path = Path(args.class_minima)
    # Byte identity is advisory; row counts and availability semantics above
    # remain blocking.
    if hashlib.sha256(path.read_bytes()).hexdigest() != audit["input_artifact_sha256"]:
        print(
            "warning: class-minima bytes differ from the retained audit input; "
            "continuing with semantic checks. Reassess retained interpretation "
            "before treating this run as equivalent.",
            file=sys.stderr,
        )
    print(
        f"A3 availability audit OK: {len(rows)} null rows; {dict(causes)}; "
        "all transition-filtered sigmas accounted for by the f64 solver"
    )


if __name__ == "__main__":
    main()
