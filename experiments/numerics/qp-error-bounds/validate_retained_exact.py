"""Validate retained-exact evidence and its narrow regression witnesses."""
from __future__ import annotations

import json
import sys
from pathlib import Path

EXPECTED = {"ordinary_generated_F5", "pinned_q4_p5", "triangle_square_tie", "pruning_roundoff"}
REQUIRED = {"run_id", "schema_version", "case_id", "target_dual_vertices_exact", "candidates", "retained_exact_min_action", "retained_exact_window_sigmas", "scalar_agreement_current_vs_retained", "minimizer_agreement_current_vs_retained", "window_agreement_current_vs_retained"}


def fail(message: str) -> None:
    raise SystemExit(f"validation failed: {message}")


def main() -> None:
    out = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("artifacts/retained-exact")
    path = out / "raw_rows.jsonl"
    if not path.exists():
        fail(f"missing {path}")
    manifest_path = out / "manifest.json"
    if not manifest_path.exists():
        fail("missing manifest.json")
    try:
        manifest = json.loads(manifest_path.read_text())
    except json.JSONDecodeError as exc:
        fail(f"invalid manifest: {exc}")
    if manifest.get("schema_version") != "qp-retained-exact-v1" or not manifest.get("source_content_id"):
        fail("manifest schema/source identity is incomplete")
    lines = path.read_text().splitlines()
    if not lines or any(not line.strip() for line in lines):
        fail("empty or blank raw JSONL line")
    try:
        rows = [json.loads(line) for line in lines]
    except json.JSONDecodeError as exc:
        fail(f"invalid JSONL: {exc}")
    if {row.get("case_id") for row in rows} != EXPECTED:
        fail("case set is not the four deterministic fixtures")
    for row in rows:
        missing = REQUIRED - row.keys()
        if missing:
            fail(f"{row.get('case_id')}: missing fields {sorted(missing)}")
        if row["schema_version"] != "qp-retained-exact-v1" or row["run_id"] != "retained-exact-v1":
            fail(f"{row['case_id']}: schema/run mismatch")
        if row["sigma_stream_count"] != row["f64_true_count"] + row["f64_indeterminate_count"] + row["f64_rejected_count"]:
            fail(f"{row['case_id']}: stream strata do not add up")
        if row["retained_exact_resolution_count"] != len(row["candidates"]):
            fail(f"{row['case_id']}: exact recheck did not resolve every retained candidate")
        if row["retained_exact_accept_count"] + row["retained_exact_reject_count"] != len(row["candidates"]):
            fail(f"{row['case_id']}: retained accept/reject strata do not add up")
        if row["exact_all_stream_count"] is not None and row["exact_all_stream_count"] != row["sigma_stream_count"]:
            fail(f"{row['case_id']}: exact-all stream differs from supplied stream")
        for candidate in row["candidates"]:
            if candidate["retained_exact_status"] == "accepted" and not (candidate["exact_q"] and candidate["exact_action"]):
                fail(f"{row['case_id']}: accepted candidate lacks exact rational values")
            if candidate["retained_exact_status"] == "rejected_exact" and (candidate["exact_q"] or candidate["exact_action"]):
                fail(f"{row['case_id']}: rejected candidate has a fabricated exact value")
    by_case = {row["case_id"]: row for row in rows}
    q4 = by_case["pinned_q4_p5"]
    tied = [0, 4, 1, 2, 7, 6]
    if tied in q4["current_minimizer_sigmas"] or tied not in q4["retained_exact_minimizer_sigmas"]:
        fail("q4:p5 tied active word regression is not preserved")
    if q4["minimizer_agreement_current_vs_retained"] or q4["window_agreement_current_vs_retained"]:
        fail("q4:p5 semantic delta was erased")
    tie = by_case["triangle_square_tie"]
    if tie["retained_exact_reject_count"] == 0 or not any(c["f64_status"] == "IndeterminateF64" and c["retained_exact_status"] == "rejected_exact" for c in tie["candidates"]):
        fail("exact rejection of retained indeterminate candidates is not preserved")
    print(f"validated {len(rows)} retained-exact case rows")


if __name__ == "__main__":
    main()
