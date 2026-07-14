"""Validate retained-exact evidence, provenance, and regression witnesses."""
from __future__ import annotations

from fractions import Fraction
import json
import math
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

EXPECTED = {
    "ordinary_generated_F5": ("hk_transition_pruned", "transition_complete"),
    "pinned_q4_p5": ("product_block", "transition_complete_product_block"),
    "triangle_square_tie": ("product_block", "transition_complete_product_block"),
    "pruning_roundoff": ("hk_transition_pruned", "transition_complete"),
}
REQUIRED = {
    "run_id", "schema_version", "case_id", "target_dual_vertices_exact",
    "candidates", "retained_exact_min_action", "retained_exact_window_sigmas",
    "scalar_agreement_current_vs_retained", "minimizer_agreement_current_vs_retained",
    "window_agreement_current_vs_retained", "agreement_rules",
}
REASON = "unavailable: exact solver returned no admissible positive-Q witness; this conflates singular/inconsistent systems and nonpositive beta or Q"


def fail(message: str) -> None:
    raise SystemExit(f"validation failed: {message}")


def rational(value: object, label: str) -> Fraction:
    if not isinstance(value, str) or value.count("/") != 1:
        fail(f"{label}: expected reduced n/d rational")
    n_text, d_text = value.split("/")
    try:
        n, d = int(n_text), int(d_text)
    except ValueError:
        fail(f"{label}: non-integer rational")
    if d <= 0 or math.gcd(abs(n), d) != 1:
        fail(f"{label}: rational is not reduced with positive denominator")
    return Fraction(n, d)


def unique_sigmas(values: object, label: str) -> list[tuple[int, ...]]:
    if not isinstance(values, list):
        fail(f"{label}: expected sigma list")
    result = []
    for sigma in values:
        if not isinstance(sigma, list) or not sigma or any(not isinstance(i, int) for i in sigma):
            fail(f"{label}: malformed sigma")
        key = tuple(sigma)
        if key in result:
            fail(f"{label}: duplicate sigma {sigma}")
        result.append(key)
    return result


def git(*args: str, cwd: Path) -> str:
    try:
        return subprocess.check_output(["git", *args], cwd=cwd, text=True).strip()
    except subprocess.CalledProcessError as exc:
        fail(f"git provenance command failed: {exc}")
        raise AssertionError


def validate_provenance(out: Path, manifest: dict[str, object]) -> None:
    repo = Path(__file__).resolve().parents[3]
    commit = manifest.get("source_revision")
    tree = manifest.get("source_tree")
    if not isinstance(commit, str) or not isinstance(tree, str):
        fail("manifest lacks source commit/tree")
    if manifest.get("source_content_id") != tree or manifest.get("source_content_id_kind") != "git_tree_oid":
        fail("manifest content identity is not the non-recursive source git tree")
    if subprocess.run(["git", "cat-file", "-e", f"{commit}^{{commit}}"], cwd=repo).returncode != 0:
        fail("source commit is not reachable")
    if git("rev-parse", f"{commit}^{{tree}}", cwd=repo) != tree:
        fail("manifest source tree does not match source commit")
    head = git("rev-parse", "HEAD", cwd=repo)
    if subprocess.run(["git", "merge-base", "--is-ancestor", commit, head], cwd=repo).returncode != 0:
        fail("source commit is not an ancestor of the validating checkout")
    if manifest.get("artifact_commit_contract") != "commit this generated directory as a separate child of source_revision":
        fail("manifest lacks two-commit artifact contract")


def validate_derived_outputs(out: Path) -> None:
    with tempfile.TemporaryDirectory(prefix="retained-exact-derived-") as temp:
        temp_out = Path(temp) / "artifact"
        temp_out.mkdir()
        shutil.copy2(out / "raw_rows.jsonl", temp_out / "raw_rows.jsonl")
        subprocess.run([sys.executable, str(Path(__file__).with_name("analyze_retained_exact.py")), str(temp_out)], check=True)
        for name in ("analysis.json", "summary.md"):
            if (temp_out / name).read_bytes() != (out / name).read_bytes():
                fail(f"derived {name} does not regenerate byte-identically from raw rows")


def main() -> None:
    out = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("artifacts/retained-exact")
    path = out / "raw_rows.jsonl"
    if not path.exists() or not path.read_text().strip():
        fail(f"missing or empty {path}")
    try:
        manifest = json.loads((out / "manifest.json").read_text())
        lines = path.read_text().splitlines()
        rows = [json.loads(line) for line in lines]
    except FileNotFoundError as exc:
        fail(f"missing artifact file: {exc.filename}")
    except json.JSONDecodeError as exc:
        fail(f"invalid JSON artifact: {exc}")
    validate_provenance(out, manifest)
    if {row.get("case_id") for row in rows} != set(EXPECTED):
        fail("case set is not the four deterministic fixtures")
    for row in rows:
        missing = REQUIRED - row.keys()
        if missing:
            fail(f"{row.get('case_id')}: missing fields {sorted(missing)}")
        case = row["case_id"]
        if row["schema_version"] != "qp-retained-exact-v1" or row["run_id"] != "retained-exact-v1":
            fail(f"{case}: schema/run mismatch")
        if (row["route"], row["completeness_contract"]) != EXPECTED[case]:
            fail(f"{case}: route/completeness invariant changed")
        if not row["target_input_kind"].startswith("stored_binary64_rational") or "unavailable" not in row["intended_algebraic_target"]:
            fail(f"{case}: target boundary changed")
        target = row["target_dual_vertices_exact"]
        if not isinstance(target, list) or not target:
            fail(f"{case}: missing target vertices")
        for i, vertex in enumerate(target):
            if not isinstance(vertex, list) or len(vertex) != 4:
                fail(f"{case}: malformed target vertex")
            for j, value in enumerate(vertex):
                rational(value, f"{case}.target[{i}][{j}]")
        candidates = row["candidates"]
        candidate_sigmas = unique_sigmas([c["sigma"] for c in candidates], f"{case}.candidates")
        if row["sigma_stream_count"] != row["f64_true_count"] + row["f64_indeterminate_count"] + row["f64_rejected_count"]:
            fail(f"{case}: stream strata do not add up")
        if row["f64_true_count"] + row["f64_indeterminate_count"] != len(candidates):
            fail(f"{case}: retained count does not match f64 strata")
        if row["retained_exact_resolution_count"] != len(candidates):
            fail(f"{case}: exact recheck did not resolve every retained candidate")
        statuses = {"AdmissibleF64": 0, "IndeterminateF64": 0}
        accepted: dict[tuple[int, ...], Fraction] = {}
        for candidate in candidates:
            status = candidate.get("f64_status")
            if status not in statuses:
                fail(f"{case}: invalid f64 status {status}")
            statuses[status] += 1
            key = tuple(candidate["sigma"])
            exact_status = candidate.get("retained_exact_status")
            reason = candidate.get("exact_resolution_reason")
            if exact_status == "accepted":
                q = rational(candidate.get("exact_q"), f"{case}.{key}.q")
                action = rational(candidate.get("exact_action"), f"{case}.{key}.action")
                if q <= 0 or action != 1 / (2 * q):
                    fail(f"{case}.{key}: exact action is not 1/(2Q) for positive Q")
                if reason != "exact solver returned an admissible positive-Q witness":
                    fail(f"{case}.{key}: accepted reason mismatch")
                accepted[key] = action
            elif exact_status == "rejected_exact":
                if candidate.get("exact_q") is not None or candidate.get("exact_action") is not None or reason != REASON:
                    fail(f"{case}.{key}: rejected candidate reason/value mismatch")
            else:
                fail(f"{case}.{key}: invalid exact status {exact_status}")
        if statuses != {"AdmissibleF64": row["f64_true_count"], "IndeterminateF64": row["f64_indeterminate_count"]}:
            fail(f"{case}: f64 status counts disagree")
        if row["retained_exact_accept_count"] != len(accepted) or row["retained_exact_reject_count"] != len(candidates) - len(accepted):
            fail(f"{case}: exact accept/reject counts disagree")
        min_action = rational(row["retained_exact_min_action"], f"{case}.retained_min")
        if min(accepted.values()) != min_action:
            fail(f"{case}: retained minimum disagrees with accepted exact actions")
        retained_mins = unique_sigmas(row["retained_exact_minimizer_sigmas"], f"{case}.retained_mins")
        if retained_mins != sorted(key for key, action in accepted.items() if action == min_action):
            fail(f"{case}: retained minimizer set disagrees with exact actions")
        cutoff = rational(row["retained_exact_window_cutoff"], f"{case}.retained_cutoff")
        if cutoff != min_action * Fraction(21, 20):
            fail(f"{case}: retained cutoff is not exact 21/20 minimum")
        retained_window = unique_sigmas(row["retained_exact_window_sigmas"], f"{case}.retained_window")
        if retained_window != sorted(key for key, action in accepted.items() if action <= cutoff):
            fail(f"{case}: retained window disagrees with exact actions")
        current_mins = unique_sigmas(row["current_minimizer_sigmas"], f"{case}.current_mins")
        current_window = unique_sigmas(row["current_window_sigmas"], f"{case}.current_window")
        current_min = row["current_min_action_f64"]
        current_cutoff = row["current_window_cutoff_f64"]
        if not isinstance(current_min, (int, float)) or not isinstance(current_cutoff, (int, float)) or not math.isclose(current_cutoff, current_min * 21 / 20, rel_tol=0, abs_tol=1e-14):
            fail(f"{case}: current f64 cutoff rule is inconsistent")
        expected_scalar = abs(current_min - float(min_action)) <= 1e-12
        expected_min = current_mins == retained_mins
        expected_window = current_window == retained_window
        if row["scalar_agreement_current_vs_retained"] != expected_scalar or row["minimizer_agreement_current_vs_retained"] != expected_min or row["window_agreement_current_vs_retained"] != expected_window:
            fail(f"{case}: current-vs-retained agreement field is stale or corrupted")
        all_min = row["exact_all_min_action"]
        if all_min is None:
            if any(row[key] is not None for key in ("scalar_agreement_retained_vs_all", "minimizer_agreement_retained_vs_all", "window_agreement_retained_vs_all")):
                fail(f"{case}: unavailable exact-all has non-null agreement fields")
            continue
        all_min_fraction = rational(all_min, f"{case}.exact_all_min")
        all_cutoff = rational(row["exact_all_window_cutoff"], f"{case}.exact_all_cutoff")
        if all_cutoff != all_min_fraction * Fraction(21, 20):
            fail(f"{case}: exact-all cutoff is inconsistent")
        all_mins = unique_sigmas(row["exact_all_minimizer_sigmas"], f"{case}.exact_all_mins")
        all_window = unique_sigmas(row["exact_all_window_sigmas"], f"{case}.exact_all_window")
        if row["scalar_agreement_retained_vs_all"] != (min_action == all_min_fraction) or row["minimizer_agreement_retained_vs_all"] != (retained_mins == all_mins) or row["window_agreement_retained_vs_all"] != (retained_window == all_window):
            fail(f"{case}: retained-vs-exact-all agreement field is stale or corrupted")
        if row["exact_all_stream_count"] != row["sigma_stream_count"] or row["exact_all_accept_count"] < row["retained_exact_accept_count"]:
            fail(f"{case}: exact-all stream invariant failed")
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
    validate_derived_outputs(out)
    print(f"validated {len(rows)} retained-exact case rows")


if __name__ == "__main__":
    main()
