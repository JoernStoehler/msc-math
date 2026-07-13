#!/usr/bin/env python3
"""Check the bounded A3-null availability audit against its row artifact."""
import argparse
import hashlib
import json
from collections import Counter
from pathlib import Path


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
        enum3 = row["enumerated_sigma_counts_by_bounces"].get("3", 0)
        solved3 = row["candidate_orbit_counts_by_bounces"].get("3", 0)
        if enum3 == 0:
            cause = "no_transition_feasible_three_bounce_sigma"
        elif solved3 == 0:
            cause = "all_three_bounce_sigmas_f64_inadmissible"
        else:
            raise AssertionError(f"unexpected A3-null row: {row['name']}")
        causes[f"{row['k']}x{row['m']}:{cause}"] += 1
        assert sum(row["f64_numerical_failure_counts_by_bounces"].values()) == 0
        assert row["exact_admissible_f64_rejected_counts_by_bounces"].get("3", 0) == 0
    assert dict(causes) == audit["cause_counts"]
    assert audit["f64_numerical_failure_rows"] == 0
    assert audit["exact_admissible_f64_rejected_candidates"] == 0
    path = Path(args.class_minima)
    assert hashlib.sha256(path.read_bytes()).hexdigest() == audit["input_artifact_sha256"]
    print(f"A3 availability audit OK: {len(rows)} null rows; {dict(causes)}")


if __name__ == "__main__":
    main()
