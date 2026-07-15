"""Integrity checks for the distinct v2 rows, formulas, policies, and provenance."""
from __future__ import annotations

from fractions import Fraction
import json
import math
import subprocess
import sys
import tempfile
from pathlib import Path


EXPECTED_CASES = {
    "simplex_F5", "hypercube_F8", "triangle_times_square_tie", "ordinary_generated_F5",
    "pinned_q4_p5", "hko_beta_boundary", "hko_near_singular_false_acceptance",
    "hko_residual_q_failure", "hko_rank_deficient", "hypercube_exact_zero_beta_boundary",
}
EXPECTED_CENTERS = {
    "saddle_eig_accepted", "svd_lstsq_proposal", "projected_critical_proposal",
    "projected_max_margin_proposal", "lu_partial_pivot_proposal", "qr_proposal",
    "refined_svd_lstsq_proposal_qr_correction",
}


def fail(message: str) -> None:
    raise SystemExit(f"validation failed: {message}")


def rational(value: object, label: str) -> Fraction:
    if not isinstance(value, str) or value.count("/") != 1:
        fail(f"{label}: malformed rational")
    n, d = value.split("/")
    try:
        x = Fraction(int(n), int(d))
    except (ValueError, ZeroDivisionError):
        fail(f"{label}: invalid rational")
    if str(x.numerator) != n or str(x.denominator) != d:
        fail(f"{label}: rational is not reduced")
    return x


def validate_registry(registry: list[dict]) -> None:
    ids = [x.get("formula_id") for x in registry]
    outputs = [x.get("output_column") for x in registry]
    if len(ids) != len(set(ids)) or len(outputs) != len(set(outputs)):
        fail("duplicate formula ID or output column")
    for formula in registry:
        required = {"formula_id", "output_column", "dependencies", "center", "exact_target", "hypothesis_status", "status", "consumers", "unavailable_rule"}
        if required - formula.keys():
            fail(f"registry entry incomplete: {formula.get('formula_id')}")
        for dep in formula["dependencies"]:
            if not isinstance(dep, list) or len(dep) != 2:
                fail(f"malformed dependency for {formula['formula_id']}")
            if dep[1] not in (formula["center"], "exact_positive_witness", ""):
                fail(f"mixed-center dependency in {formula['formula_id']}")
        if "action" in formula["formula_id"] and "positive_q" not in formula["formula_id"] and formula["status"] != "unavailable":
            fail(f"action formula lacks explicit positive-Q prerequisite: {formula['formula_id']}")


def main() -> None:
    out = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("artifacts/soundness-v2")
    required = ["raw_rows.jsonl", "policy_rows.jsonl", "formula_registry.json", "producer_summary.json", "analysis.json", "formula_evaluations.jsonl", "interpretation.md", "manifest.json"]
    for name in required:
        if not (out / name).exists() or not (out / name).read_text().strip():
            fail(f"missing or empty {name}")
    rows = [json.loads(x) for x in (out / "raw_rows.jsonl").read_text().splitlines() if x]
    policies = [json.loads(x) for x in (out / "policy_rows.jsonl").read_text().splitlines() if x]
    registry = json.loads((out / "formula_registry.json").read_text())
    evaluations = [json.loads(x) for x in (out / "formula_evaluations.jsonl").read_text().splitlines() if x]
    summary = json.loads((out / "producer_summary.json").read_text())
    manifest = json.loads((out / "manifest.json").read_text())
    if summary.get("row_count") != len(rows) or summary.get("policy_row_count") != len(policies):
        fail("truncated raw or policy artifact disagrees with producer summary")
    if {r.get("target_polytope_id") for r in rows} != EXPECTED_CASES:
        fail("case set differs from declared v2 fixtures")
    seen = set()
    for row in rows:
        if row.get("schema_version") != "qp-soundness-row-v2" or row.get("run_id") != "qp-soundness-v2":
            fail("raw schema/run mismatch")
        key = (row["target_polytope_id"], tuple(row["sigma_active_reeb_word"]))
        if key in seen:
            fail(f"duplicate lifecycle row {key}")
        seen.add(key)
        if row["sigma_length"] != len(key[1]) or not key[1]:
            fail(f"sigma length corruption {key}")
        if set(c["center_id"] for c in row["centers"]) != EXPECTED_CENTERS:
            fail(f"center set changed for {key}")
        if len(row["qp_constraint_matrix_c_f64"]) != 5 or len(row["qp_constraint_matrix_c_exact"]) != 5:
            fail(f"C shape corruption {key}")
        n = row["sigma_length"] + 5
        if len(row["kkt_augmented_matrix_m_f64"]) != n or len(row["kkt_augmented_matrix_m_exact"]) != n:
            fail(f"KKT shape corruption {key}")
        if row["exact_row_reduction_system_status"] == "inconsistent" and row["exact_positive_witness_status"] == "exists":
            fail(f"inconsistent system has positive witness {key}")
        if row["exact_positive_witness_status"] == "exists":
            q = rational(row["exact_positive_witness_q"], f"{key}.q")
            a = rational(row["exact_positive_witness_action"], f"{key}.action")
            if q <= 0 or a != 1 / (2 * q):
                fail(f"exact Q/action corruption {key}")
            if row["exact_action_availability"] != "available":
                fail(f"available exact action mislabeled {key}")
        elif row["exact_positive_witness_status"] == "exists_q_nonpositive":
            q = rational(row["exact_positive_witness_q"], f"{key}.nonpositive_q")
            if q > 0 or row["exact_positive_witness_action"] is not None or row["exact_action_availability"] != "unavailable":
                fail(f"nonpositive-Q exact distinction corrupted {key}")
        else:
            if row["exact_positive_witness_q"] is not None or row["exact_positive_witness_action"] is not None:
                fail(f"unavailable exact witness carries target values {key}")
    validate_registry(registry)
    registry_ids = {x["formula_id"] for x in registry}
    if {x.get("formula_id") for x in evaluations} - registry_ids:
        fail("formula evaluations contain an unregistered formula ID")
    coverage = json.loads((out / "analysis.json").read_text()).get("formula_coverage", {})
    if set(coverage) != registry_ids:
        fail("analysis does not report coverage for every registered formula")
    raw_by_case = {case: [r for r in rows if r["target_polytope_id"] == case] for case in EXPECTED_CASES}
    if not policies or {p.get("target_polytope_id") for p in policies} != EXPECTED_CASES:
        fail("policy join case coverage failure")
    for policy in policies:
        case_rows = raw_by_case[policy["target_polytope_id"]]
        if policy["supplied_stream_count"] != len(case_rows):
            fail(f"policy supplied-stream count mismatch {policy['target_polytope_id']}")
        if policy["policy_candidate_count"] < policy["policy_exact_accept_count"]:
            fail("policy accepted count exceeds policy candidate count")
        if policy["policy_id"] == "minimasafe_heuristic":
            if policy["policy_exact_resolution_count"] != 0 or policy["policy_min_action"] is not None or policy["policy_window_cutoff"] is not None:
                fail("MinimaSafe heuristic incorrectly carries exact aggregation output")
            if policy["policy_f64_min_action"] is not None and policy["policy_f64_window_cutoff"] is None:
                fail("MinimaSafe f64 minimum lacks its declared window cutoff")
        else:
            if policy["policy_exact_resolution_count"] != policy["policy_candidate_count"]:
                fail("exact policy did not record every attempted resolution")
            if any(policy[key] is not None for key in ("policy_f64_min_action", "policy_f64_window_cutoff")) and policy["policy_id"] != "selective_fallback_f64_anchored_window":
                fail("exact-only policy carries an undeclared f64 aggregate")
        cutoff = policy["policy_window_cutoff"]
        if cutoff is not None:
            c = rational(cutoff, "policy cutoff")
            m = rational(policy["policy_min_action"], "policy minimum")
            gap = Fraction.from_float(policy["requested_relative_gap"])
            # Binary f64 gap is the producer's declared calculation contract.
            if c != m * (1 + gap):
                fail("policy relative window corruption")
    if manifest.get("schema_version") != "qp-soundness-row-v2" or manifest.get("artifact_commit_contract") != "commit this generated directory as a separate child of source_revision":
        fail("manifest provenance contract mismatch")
    repo = Path(__file__).resolve().parents[3]
    commit = manifest.get("source_revision")
    tree = manifest.get("source_tree")
    if not isinstance(commit, str) or not isinstance(tree, str):
        fail("manifest lacks source identity")
    if subprocess.run(["git", "cat-file", "-e", f"{commit}^{{commit}}"], cwd=repo).returncode != 0:
        fail("source commit unavailable")
    actual_tree = subprocess.check_output(["git", "rev-parse", f"{commit}^{{tree}}"], cwd=repo, text=True).strip()
    if actual_tree != tree:
        fail("source tree provenance mismatch")
    with tempfile.TemporaryDirectory() as tmp:
        temp = Path(tmp)
        for name in ("raw_rows.jsonl", "formula_registry.json"):
            (temp / name).write_bytes((out / name).read_bytes())
        subprocess.run([sys.executable, str(Path(__file__).with_name("analyze_soundness_v2.py")), str(temp)], check=True)
        for name in ("analysis.json", "formula_evaluations.jsonl", "interpretation.md"):
            if (temp / name).read_bytes() != (out / name).read_bytes():
                fail(f"derived artifact is stale: {name}")
    print(f"validated {len(rows)} v2 raw rows and {len(policies)} policy rows")


if __name__ == "__main__":
    main()
