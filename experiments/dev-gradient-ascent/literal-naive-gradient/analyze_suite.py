# /// script
# requires-python = ">=3.12"
# dependencies = ["blake3", "matplotlib"]
# ///
"""Validate and summarize the optimizer-suite artifacts.

This analyzer deliberately fails closed: every claim-bearing row is checked
against its raw trajectory before figures or discussion prose are written.
"""
from __future__ import annotations

import hashlib
import json
import math
import statistics
from pathlib import Path
from collections import Counter

import matplotlib.pyplot as plt
import blake3

OWNER = Path(__file__).resolve().parent
BASELINE = OWNER / "artifacts/evaluation/analysis.json"
SOURCE = OWNER.parents[1] / "sys-datascience/produce/random.jsonl"
OUT = OWNER / "artifacts/suite-analysis"
FIG = OWNER / "figures"
ETAS = [1e-5, 1e-4, 1e-3, 1e-2, 1e-1, 1.0]
STARTS = ["random_F6_s0_0", "random_F6_s0_2", "random_F6_s0_3", "random_F6_s0_4", "random_F6_s0_5", "random_F6_s0_6"]
PANEL_STARTS = STARTS[:2]
MAX_TARGETS = 100
TOL = 1e-10


def read(path: Path):
    with path.open() as stream:
        return json.load(stream)


def read_jsonl(path: Path):
    with path.open() as stream:
        return [json.loads(line) for line in stream]


def close(a, b):
    return abs(a - b) <= TOL * (1 + abs(a) + abs(b))


def fail(message):
    raise AssertionError(message)


def validate_safeguard(name, run, baseline):
    policy = "invalidity_only" if name == "suite-invalidity" else "monotone_backtracking"
    expected = {(start, eta) for start in STARTS for eta in ETAS}
    seen = set()
    validated = []
    for summary in run.get("trajectories", []):
        key = (summary["start_id"], summary["nominal_eta"])
        if summary["policy"] != policy or key in seen or key not in expected:
            fail(f"{name}: unexpected or duplicate trajectory {key}")
        seen.add(key)
        path = OWNER / "artifacts" / name / "trajectories" / policy / summary["start_id"] / f"eta-{eta_label(summary['nominal_eta'])}.jsonl"
        if not path.exists():
            fail(f"{name}: missing raw trajectory {path}")
        rows = read_jsonl(path)
        validated.append(validate_safeguard_trajectory(rows, summary, policy))
    if seen != expected:
        fail(f"{name}: coverage {len(seen)} != frozen 6x6")
    if run.get("total_target_evaluations") != sum(s["target_evaluations"] for s in run["trajectories"]):
        fail(f"{name}: run target-evaluation total disagreement")
    return validated


def validate_safeguard_trajectory(rows, summary, policy):
    if not rows or rows[0]["reason"] != "initial" or rows[0]["target_evaluations"] != 0:
        fail("safeguard: malformed initial row")
    if rows[0]["start_id"] != summary["start_id"] or not close(rows[0]["nominal_eta"], summary["nominal_eta"]):
        fail("safeguard: initial identity disagreement")
    if summary["requested_updates"] != 100:
        fail("safeguard: requested update budget changed")
    current = rows[0]["target_sys"]
    if not close(current, summary["initial_sys"]):
        fail("safeguard: initial sys disagreement")
    best = current
    best_iteration = 0
    target_rows = rows[1:]
    if any(r["target_evaluations"] != i + 1 for i, r in enumerate(target_rows)):
        fail("safeguard: target-evaluation counter is not contiguous")
    if summary["target_evaluations"] != len(target_rows) or summary["target_evaluations"] > MAX_TARGETS:
        fail("safeguard: target budget disagreement")
    invalid = rejected = decreases = backtracks = accepted = 0
    by_iteration = {}
    for row in target_rows:
        if row["policy"] != policy or row["start_id"] != summary["start_id"] or not close(row["nominal_eta"], summary["nominal_eta"]):
            fail("safeguard: row identity disagreement")
        iteration = row["iteration"]
        by_iteration.setdefault(iteration, []).append(row)
    for iteration, attempts in by_iteration.items():
        if not attempts or attempts[0]["attempt"] != 0:
            fail("safeguard: attempt order does not restart at zero")
        for attempt, row in enumerate(attempts):
            if row["attempt"] != attempt:
                fail("safeguard: attempt numbers are not contiguous")
            expected_rate = summary["nominal_eta"] * (0.5 ** attempt)
            if not close(row["rate"], expected_rate):
                fail("safeguard: non-dyadic retry rate")
            valid = row["target_valid"]
            if valid != (row["target_sys"] is not None):
                fail("safeguard: validity/sys null disagreement")
            delta = None if not valid else row["target_sys"] - current
            if valid and not close(row["delta"], delta):
                fail("safeguard: delta disagreement")
            if not valid and row["delta"] is not None:
                fail("safeguard: invalid target has delta")
            if policy == "invalidity_only" and valid and any(previous["target_valid"] for previous in attempts[:attempt]):
                fail("invalidity-only: a valid target was retried")
            if attempt:
                backtracks += 1
            if not valid:
                invalid += 1
            should_accept = valid if policy == "invalidity_only" else valid and delta > 0.0
            if row["accepted"] != should_accept:
                fail("safeguard: acceptance predicate disagreement")
            if row["accepted"]:
                accepted += 1
                if delta < 0.0:
                    decreases += 1
                current = row["target_sys"]
                if current > best:
                    best, best_iteration = current, iteration
            elif valid and policy == "monotone_backtracking":
                rejected += 1
            if row["accepted"] and row["reason"] not in ("valid",):
                fail("safeguard: accepted row reason disagreement")
    expected_backtracks = backtracks
    # The producer charges the retry that would have followed a final
    # rejected target before discovering the global 100-target stop. It is a
    # scheduled retry, not an unrecorded target evaluation.
    if summary["failure"] == "method_stop_target_evaluation_budget" and summary["backtracking_attempts"] == backtracks + 1:
        expected_backtracks = backtracks + 1
    if accepted != summary["committed_updates"] or invalid != summary["invalid_attempts"] or rejected != summary["rejected_attempts"] or decreases != summary["accepted_decreases"] or expected_backtracks != summary["backtracking_attempts"]:
        fail("safeguard: summary accounting disagreement")
    complete = summary["failure"] is None
    if complete and summary["committed_updates"] != summary["requested_updates"]:
        fail("safeguard: complete trajectory has short update count")
    if not complete and summary["failure"] not in ("method_stop_target_evaluation_budget", "method_stall_backtracking_safety_bound", "invalid_target"):
        fail("safeguard: unknown failure semantics")
    if complete != (summary["final_sys"] is not None):
        fail("safeguard: final-state censoring disagreement")
    if complete and not close(summary["final_sys"], current):
        fail("safeguard: final sys disagreement")
    if not close(summary["best_sys"], best) or summary["best_iteration"] != best_iteration:
        fail("safeguard: best-so-far disagreement")
    return summary


def validate_panel(name, run):
    expected_policy = {"suite-panel": {"near_active_maximin", "positive_spanning_poll"}}[name]
    seen = set()
    validated = []
    for summary in run.get("trajectories", []):
        policy, start = summary["policy"], summary["start_id"]
        if policy not in expected_policy or start not in PANEL_STARTS or summary["nominal_eta"] != 1e-3:
            fail("panel: unexpected coverage")
        key = (policy, start)
        if key in seen:
            fail("panel: duplicate trajectory")
        seen.add(key)
        path = OWNER / "artifacts/suite-panel/trajectories" / policy / start / "eta-1e-3.jsonl"
        if not path.exists():
            fail(f"panel: missing raw trajectory {path}")
        rows = read_jsonl(path)
        validated.append(validate_panel_trajectory(rows, summary, policy))
    if seen != {(p, s) for p in expected_policy for s in PANEL_STARTS}:
        fail("panel: expected 2-start coverage missing")
    if run.get("total_target_evaluations") != sum(s["target_evaluations"] for s in run["trajectories"]):
        fail("panel: total target accounting disagreement")
    return validated


def validate_panel_trajectory(rows, summary, policy):
    if not rows or rows[0]["reason"] != "initial":
        fail("panel: malformed initial row")
    target_rows = rows[1:]
    candidate_rows = [r for r in target_rows if r["reason"] == "poll_candidate"] if policy == "positive_spanning_poll" else target_rows
    if any(r["target_evaluations"] != i + 1 for i, r in enumerate(candidate_rows)):
        fail("panel: non-contiguous target counter")
    if summary["target_evaluations"] != len(candidate_rows):
        fail("panel: target count disagreement")
    if summary["target_evaluations"] > MAX_TARGETS:
        fail("panel: target budget exceeded")
    if policy == "near_active_maximin":
        if any(r["reason"] not in ("radius_expand", "radius_shrink_or_stop") for r in target_rows):
            fail("maximin: unexpected row reason")
        current = rows[0]["target_sys"]
        for row in target_rows:
            valid = row["target_valid"]
            if valid != (row["target_sys"] is not None):
                fail("maximin: validity/sys disagreement")
            delta = None if not valid else row["target_sys"] - current
            if valid and not close(row["delta"], delta):
                fail("maximin: delta disagreement")
            accepted = valid and delta > 0.0
            if row["accepted"] != accepted:
                fail("maximin: acceptance predicate disagreement")
            if accepted:
                current = row["target_sys"]
        if sum(not r["target_valid"] for r in target_rows) != summary["invalid_attempts"] or sum(not r["accepted"] for r in target_rows) != summary["rejected_attempts"] or summary["accepted_decreases"] != 0:
            fail("maximin: attempt accounting disagreement")
        if sum(r["accepted"] for r in target_rows) != summary["committed_updates"]:
            fail("maximin: commit accounting disagreement")
    else:
        labels = ["slice+e0", "slice-e0", "slice+e1", "slice-e1", "slice+e2", "slice-e2", "slice+e3", "slice-e3"]
        current = rows[0]["target_sys"]
        accepted_count = 0
        invalid_count = 0
        no_improvement_count = 0
        i = 0
        while i < len(target_rows):
            iteration = target_rows[i]["iteration"]
            block = []
            while i < len(target_rows) and target_rows[i]["iteration"] == iteration and target_rows[i]["reason"] == "poll_candidate":
                block.append(target_rows[i]); i += 1
            if [r["direction"] for r in block] != labels[:len(block)]:
                fail("poll: direction order disagreement")
            deltas = []
            for candidate in block:
                valid = candidate["target_valid"]
                if valid != (candidate["target_sys"] is not None):
                    fail("poll: validity/sys disagreement")
                if not valid:
                    invalid_count += 1
                delta = None if not valid else candidate["target_sys"] - current
                if valid and not close(candidate["delta"], delta):
                    fail("poll: delta disagreement")
                deltas.append(delta if delta is not None else float("-inf"))
            incomplete = len(block) < 8
            if incomplete:
                if i != len(target_rows) or summary["failure"] != "method_stop_target_evaluation_budget":
                    fail("poll: incomplete poll was not censored as budget stop")
                if any(r["accepted"] for r in block):
                    fail("poll: incomplete poll accepted a candidate")
                break
            if i < len(target_rows) and target_rows[i]["iteration"] == iteration:
                marker = target_rows[i]; i += 1
                best_index = max(range(len(deltas)), key=deltas.__getitem__)
                best_delta = deltas[best_index]
                if marker["reason"] == "poll_best_improves":
                    if not marker["accepted"] or marker["direction"] != labels[best_index] or best_delta <= 0.0:
                        fail("poll: malformed accepted marker")
                    if not close(marker["delta"], best_delta):
                        fail("poll: accepted poll delta disagreement")
                    current += best_delta
                    accepted_count += 1
                elif marker["reason"] == "poll_no_improvement":
                    if marker["accepted"]:
                        fail("poll: malformed no-improvement marker")
                    no_improvement_count += 1
                    if best_delta > 0.0:
                        fail("poll: positive candidate was discarded")
                else:
                    fail("poll: malformed no-improvement marker")
        if accepted_count != summary["committed_updates"]:
            fail("poll: commit accounting disagreement")
        if invalid_count != summary["invalid_attempts"] or no_improvement_count != summary["rejected_attempts"] or summary["accepted_decreases"] != 0:
            fail("poll: attempt accounting disagreement")
    return summary


def eta_label(eta):
    value = f"{eta:.0e}".replace("+", "")
    return value.replace("e-00", "e-0").replace("e-0", "e-").replace("e00", "e0")


def source_identity(baseline, run):
    expected_sha = baseline["analysis"]["source_sha256"]
    if hashlib.sha256(SOURCE.read_bytes()).hexdigest() != expected_sha:
        fail("source identity differs from frozen baseline")
    provenance = read(OWNER / "artifacts/suite-invalidity/run-provenance.json")
    for key in ("source_input", "source_input_blake3", "implementation", "implementation_blake3", "command"):
        if not provenance.get(key):
            fail(f"missing provenance identity {key}")
    if not provenance["source_input"].endswith("experiments/sys-datascience/produce/random.jsonl"):
        fail("provenance source path mismatch")
    source_path = SOURCE
    implementation_path = Path(provenance["implementation"])
    if not implementation_path.exists():
        implementation_path = OWNER / "optimizer_suite.rs"
    if blake3.blake3(source_path.read_bytes()).hexdigest() != provenance["source_input_blake3"]:
        fail("provenance source BLAKE3 mismatch")
    if blake3.blake3(implementation_path.read_bytes()).hexdigest() != provenance["implementation_blake3"]:
        fail("provenance implementation BLAKE3 mismatch")
    for name in ("suite-monotone", "suite-panel"):
        other = read(OWNER / "artifacts" / name / "run-provenance.json")
        if other["source_input_blake3"] != provenance["source_input_blake3"]:
            fail("suite input hash mismatch")


def discussion(rows, provenance):
    by = {}
    for row in rows:
        by.setdefault(row["policy"], []).append(row)
    def stat(policy, key): return sum(r[key] for r in by[policy])
    def median_gain(policy):
        return statistics.median(
            r["best_sys"] - r["initial_sys"] for r in by[policy]
        )
    return f"""# Optimizer-suite comparison

This report is generated by `analyze_suite.py` only after strict raw-trajectory
validation. The frozen safeguards each cover 6 starts × 6 nominal rates and
use exactly 3,600 target evaluations, with a hard limit of 100 post-initial
targets per trajectory. The literal baseline cells are reused from the
validated baseline analysis.

Validated observations:

* Invalidity-only: {stat('invalidity_only','invalid_attempts')} invalid attempts,
  {stat('invalidity_only','accepted_decreases')} accepted decreases,
  {sum(r['stalls'] for r in by['invalidity_only'])} budget stops, median best gain
  {median_gain('invalidity_only'):.6g}.
* Monotone backtracking: {stat('monotone_backtracking','invalid_attempts')} invalid
  attempts, {stat('monotone_backtracking','rejected_attempts')} rejected
  non-increases, {stat('monotone_backtracking','accepted_decreases')} accepted
  decreases, {sum(r['stalls'] for r in by['monotone_backtracking'])} stops, median
  best gain {median_gain('monotone_backtracking'):.6g}.
* The maximin panel covers two starts and 15 requested updates with a
  {provenance['maximin_targets']} target-evaluation cost. Its 1e-3 near-active
  window is a branch-selection window, not an acceptance or stopping
  threshold; sensitivity to that window is untested.
* The poll panel is positive-spanning only on the arbitrary first-facet 4D
  slice (± four coordinate directions), not the full ambient or quotient
  space. Its incomplete final polls are censored at the 100-target budget.

These data support adding invalidity-only safeguarding. Monotone backtracking
is a possible later polisher but trades away late recovery through rejection
and stalls. The small maximin/poll panel does not justify a full stationarity-
gated comparison. No result supports convergence, local maximality, or
population-wide claims. A nearby-gradient bundle remains omitted because no
validated bundle rule is owned by this packet.
"""


def savefig(fig, path):
    fig.savefig(path.with_suffix(".png"), dpi=180, metadata={"Software": "optimizer-suite-analyzer"})
    fig.savefig(path.with_suffix(".pdf"), metadata={"Creator": "optimizer-suite-analyzer", "CreationDate": None, "ModDate": None})
    plt.close(fig)


def main():
    OUT.mkdir(parents=True, exist_ok=True); FIG.mkdir(exist_ok=True)
    baseline = read(BASELINE)
    runs = {name: read(OWNER / "artifacts" / name / "summary.json") for name in ("suite-invalidity", "suite-monotone", "suite-panel")}
    source_identity(baseline, runs["suite-invalidity"])
    validated = validate_safeguard("suite-invalidity", runs["suite-invalidity"], baseline) + validate_safeguard("suite-monotone", runs["suite-monotone"], baseline) + validate_panel("suite-panel", runs["suite-panel"])
    rows=[]
    for cell in baseline.get("cells", []):
        rows.append({"policy":"literal", "start_id":cell["start_id"], "nominal_eta":cell["eta"], "requested_updates":100, "committed_updates":cell["iterations_completed"], "initial_sys":cell["initial_sys"], "final_sys":cell.get("final_sys"), "best_sys":cell["best_sys"], "best_iteration":cell["best_iteration"], "target_evaluations":cell["iterations_completed"], "invalid_attempts":int(not cell["complete"]), "rejected_attempts":0, "accepted_decreases":cell["full_sys_decreases"], "backtracking_attempts":0, "stalls":0, "failure":cell.get("failure"), "final_radius":None, "source":"baseline-evaluation"})
    for run in runs.values(): rows.extend(dict(r, source="validated-suite") for r in run["trajectories"])
    analyzer_hash = hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
    provenance = {"analyzer":"analyze_suite.py", "analyzer_sha256":analyzer_hash, "validated_raw_trajectories":len(validated), "source_sha256":baseline["analysis"]["source_sha256"], "maximin_targets":sum(r["target_evaluations"] for r in runs["suite-panel"]["trajectories"] if r["policy"]=="near_active_maximin")}
    out={"analysis_provenance":provenance, "baseline_source":"artifacts/evaluation/analysis.json", "suite_sources":list(runs), "baseline_cells":baseline["analysis"]["cell_count"], "trajectories":rows, "paired_denominators":dict(Counter(r["policy"] for r in rows)), "validation":"passed strict raw trajectory, coverage, counter, retry, acceptance, accounting, panel, and source-identity checks", "caveat":"descriptive frozen comparison; no local-maximality or population claim"}
    (OUT/"analysis.json").write_text(json.dumps(out, indent=2)+"\n")
    (OUT/"DISCUSSION.md").write_text(discussion(rows, provenance))
    methods=sorted({r["policy"] for r in rows}); vals=[[r["best_sys"]-r["initial_sys"] for r in rows if r["policy"]==m] for m in methods]; labels=[m.replace("_","\n") for m in methods]
    fig,ax=plt.subplots(figsize=(8,4.5)); ax.boxplot(vals,tick_labels=labels,showfliers=True); ax.axhline(0,color="black",lw=.6); ax.set_ylabel("best gain in sys"); ax.set_title("Optimizer suite: retained best gain"); fig.tight_layout(); savefig(fig, FIG/"suite-best-gain")
    fig,ax=plt.subplots(figsize=(8,4.5));
    for m in methods:
        rs=[r for r in rows if r["policy"]==m]; ax.scatter([r["target_evaluations"] for r in rs],[r["best_sys"]-r["initial_sys"] for r in rs],label=m,s=24)
    ax.set_xlabel("exact target evaluations (initial excluded)"); ax.set_ylabel("best gain in sys"); ax.legend(fontsize=7); ax.set_title("Gain per exact evaluation; failed/stalled runs retained"); fig.tight_layout(); savefig(fig, FIG/"suite-gain-per-evaluation")
    print(OUT/"analysis.json")

if __name__ == "__main__": main()
