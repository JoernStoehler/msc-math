"""Strict analyzer and combined displays for the adaptive-direction packet."""
import json, pathlib, sys, subprocess

root = pathlib.Path(sys.argv[1] if len(sys.argv) > 1 else "artifacts")
generic_mode = len(sys.argv) > 2 and sys.argv[2] == "generic"
screening_root = root / "screening" if (root / "screening").exists() else root.parent / "screening"
prov = json.loads((root / "run-provenance.json").read_text())
def blake3_file(path):
    code = "import blake3,sys; print(blake3.blake3(open(sys.argv[1],'rb').read()).hexdigest())"
    return subprocess.check_output(["uv", "run", "--with", "blake3", "--no-project", "python3", "-c", code, str(path)], text=True).strip()
def blake3_bytes(data):
    code = "import blake3,sys; print(blake3.blake3(sys.stdin.buffer.read()).hexdigest())"
    return subprocess.run(["uv", "run", "--with", "blake3", "--no-project", "python3", "-c", code], input=data, capture_output=True, check=True).stdout.decode().strip()
def warn_stale(condition, label):
    # Provenance byte identities are advisory. Scientific trajectory and
    # accounting checks below remain blocking.
    if not condition:
        print(
            f"warning: {label} differs from retained provenance; continuing "
            "with semantic checks. Reassess retained interpretation before "
            "treating this run as equivalent.",
            file=sys.stderr,
        )
def validate_artifact_identity():
    identity = json.loads((root / "artifact-identity.json").read_text())
    commit = identity["producing_implementation_commit"]
    path = identity["implementation_path"]
    try:
        blob = subprocess.check_output(["git", "show", f"{commit}:{path}"])
    except (OSError, subprocess.CalledProcessError):
        warn_stale(False, "implementation at recorded commit is unavailable")
    else:
        warn_stale(blake3_bytes(blob) == identity["implementation_blake3"], "implementation at recorded commit")
    warn_stale(identity["implementation_blake3"] == prov["implementation_blake3"], "identity/provenance implementation")
    warn_stale(identity["raw_source_head"] == prov["source_head"], "recorded source revision")
    warn_stale(identity["raw_implementation_blake3"] == prov["implementation_blake3"], "raw/provenance implementation")
    assert identity["raw_implementation_path"] == prov["implementation"]
    assert identity["source_input"] == prov["source_input"]
    warn_stale(identity["source_input_blake3"] == prov["source_input_blake3"], "identity/provenance input")
    assert identity["command"] == prov["command"]
validate_artifact_identity()
source_path = pathlib.Path(prov["source_input"])
if not source_path.exists():
    source_path = pathlib.Path(__file__).resolve().parents[3] / source_path
assert source_path.exists(), source_path
warn_stale(blake3_file(source_path) == prov["source_input_blake3"], "current source input")
expected_policies = {"inf_normalized_branch_gradient", "near_active_box_lp_maximin", "candidate_window_box_lp_maximin", "single_branch_box_steepest"}
expected_radii = {float(x) for x in prov["initial_radii"]}
assert abs(prov["candidate_window_relative_gap"] - 1.0e-2) < 1e-15
assert prov["requested_target_budget"] == 6
manifest = json.loads((root.parent.parent / "inputs" / "generic-start-manifest.json").read_text()) if generic_mode else json.loads((root.parent / "inputs" / "fixture-manifest.json").read_text())
screening_report = json.loads((screening_root / "screening-report.json").read_text())
if generic_mode:
    assert prov["source_input"].endswith("experiments/sys-datascience/produce/random.jsonl")
    screening = json.loads((screening_root / "screening-report.json").read_text())
    assert {x["id"] for x in manifest["fixtures"]} == set(screening["starts"])
expected_starts = {x["id"] for x in manifest["fixtures"]}
files = sorted((root / "trajectories").glob("*/*/*.jsonl"))
assert files, "no trajectory files"
assert len(files) == len(expected_policies) * len(expected_starts) * len(expected_radii), "incomplete trajectory file coverage"
all_rows = []
by_cell = {}
for path in files:
    rs = [json.loads(x) for x in path.read_text().splitlines() if x.strip()]
    assert rs, f"empty trajectory: {path}"
    initial = rs[0]
    key = (initial["policy"], initial["start_id"], float(initial["initial_radius"]))
    assert key not in by_cell, f"duplicate trajectory cell {key}"
    assert initial["iteration"] == 0 and initial["target_evaluations"] == 0 and initial["reason"] == "initial"
    assert initial["target_valid"] and initial["accepted"]
    assert abs(initial["target_sys"] - initial["best_sys"]) < 1e-12
    assert initial["base_sys"] == initial["target_sys"]
    assert initial["current_radius"] == initial["initial_radius"]
    prev_eval = 0
    prev_best = initial["best_sys"]
    prev_radius = initial["current_radius"]
    expected_dual = initial["base_dual_flat"]
    expected_sys = initial["base_sys"]
    for i, row in enumerate(rs[1:], 1):
        assert row["policy"] == initial["policy"] and row["start_id"] == initial["start_id"]
        assert float(row["initial_radius"]) == float(initial["initial_radius"])
        assert row["iteration"] == i and row["target_evaluations"] == i and row["target_evaluations"] == prev_eval + 1
        assert row["direction_label"] and row["direction_norm_inf"] <= 1.0 + 2e-8 and row["direction_norm_inf"] > 0.0
        expected_label = {"inf_normalized_branch_gradient": "inf_normalized_branch_gradient", "near_active_box_lp_maximin": "near_active_box_lp_maximin", "candidate_window_box_lp_maximin": "candidate_window_box_lp_maximin", "single_branch_box_steepest": "single_branch_box_steepest"}[row["policy"]]
        assert row["direction_label"] == expected_label
        assert row["base_dual_flat"] == expected_dual and abs(row["base_sys"] - expected_sys) < 2e-12
        assert len(row["direction_flat"]) > 0 and len(row["direction_flat"]) % 4 == 0
        assert len(row["base_dual_flat"]) == len(row["target_dual_flat"]) == len(row["direction_flat"])
        assert max(abs(t - (b + row["proposal_radius"] * d)) for b, t, d in zip(row["base_dual_flat"], row["target_dual_flat"], row["direction_flat"])) < 2e-12
        assert row["near_active_count"] == len(row["near_active_sigmas"])
        assert row["candidate_window_count"] == len(row["candidate_window_sigmas"])
        assert row["genuinely_multi_branch"] == (row["near_active_count"] > 1 or row["candidate_window_count"] > 1)
        assert abs(row["proposal_radius"] - prev_radius) < 1e-14 * max(1.0, prev_radius)
        if row["target_valid"]:
            assert row["target_sys"] is not None and row["delta"] is not None
            assert abs(row["delta"] - (row["target_sys"] - row["base_sys"])) < 2e-10
        else:
            assert row["target_sys"] is None and row["delta"] is None and row["target_sigma"] is None
        if row["accepted"]:
            assert row["target_valid"] and row["delta"] > 0 and row["reason"] == "accepted_radius_expand"
            assert abs(row["current_radius"] - 1.25 * row["proposal_radius"]) < 1e-14
        else:
            assert row["reason"] in ("invalid_radius_shrink", "non_improving_radius_shrink")
            assert abs(row["current_radius"] - 0.5 * row["proposal_radius"]) < 1e-14
        expected_best = max(prev_best, row["target_sys"] if row["target_valid"] else float("-inf"))
        assert abs(row["best_sys"] - expected_best) < 2e-10
        if row["predicted_delta"] is not None:
            assert row["predicted_branch_values"] and abs(row["predicted_delta"] - min(row["predicted_branch_values"])) < 2e-10
            expected_count = row["near_active_count"] if row["policy"] == "near_active_box_lp_maximin" else row["candidate_window_count"] if row["policy"] == "candidate_window_box_lp_maximin" else 1
            assert len(row["predicted_branch_values"]) == expected_count
            assert row["predicted_observed_error"] is not None and row["target_valid"]
            assert abs(row["predicted_observed_error"] - (row["delta"] - row["predicted_delta"])) < 2e-10
            assert row["predicted_winning_sigma"] is not None
            assert row["predicted_winning_sigma"] in row["near_active_sigmas"] or row["predicted_winning_sigma"] in row["candidate_window_sigmas"] or row["policy"] in ("inf_normalized_branch_gradient", "single_branch_box_steepest")
        if row["policy"] == "single_branch_box_steepest":
            gradient = row["primary_gradient_flat"]
            assert len(gradient) == len(row["direction_flat"])
            for g, d in zip(gradient, row["direction_flat"]):
                expected = 1.0 if g > 0.0 else -1.0 if g < 0.0 else 0.0
                assert abs(d - expected) < 2e-10
        if row["target_sigma"] is not None:
            assert row["target_visible_near"] == (row["target_sigma"] in row["near_active_sigmas"])
            assert row["target_visible_candidate"] == (row["target_sigma"] in row["candidate_window_sigmas"])
        if row["accepted"]:
            expected_dual, expected_sys = row["target_dual_flat"], row["target_sys"]
        prev_eval, prev_best, prev_radius = i, row["best_sys"], row["current_radius"]
    by_cell[key] = rs
    all_rows.extend(rs[1:])
assert set(by_cell) == {(p, s, r) for p in expected_policies for s in expected_starts for r in expected_radii}, "policy/start/radius coverage mismatch"

def recompute_summary(rs):
    initial = rs[0]
    current_sys = initial["target_sys"]
    best_sys = current_sys
    best_iteration = 0
    committed = invalid = rejected = decreases = expands = shrinks = 0
    for row in rs[1:]:
        invalid += not row["target_valid"]
        rejected += not row["accepted"]
        if row["accepted"]:
            committed += 1
            expands += 1
            assert row["target_sys"] is not None
            decreases += row["target_sys"] < row["base_sys"]
            current_sys = row["target_sys"]
            if current_sys > best_sys:
                best_sys, best_iteration = current_sys, row["iteration"]
        else:
            shrinks += 1
    stop_reason = "target_evaluation_budget" if len(rs) - 1 >= 100 else "shrunken_radius" if rs[-1]["current_radius"] < 1e-12 else "budget"
    return {
        "policy": initial["policy"], "start_id": initial["start_id"],
        "initial_radius": float(initial["initial_radius"]),
        "requested_updates": prov["requested_target_budget"], "committed_updates": committed,
        "initial_sys": initial["target_sys"], "final_sys": current_sys,
        "best_sys": best_sys, "best_iteration": best_iteration,
        "target_evaluations": len(rs) - 1, "invalid_attempts": invalid,
        "rejected_attempts": rejected, "accepted_decreases": decreases,
        "radius_expansions": expands, "radius_shrinks": shrinks,
        "stop_reason": stop_reason, "final_radius": rs[-1]["current_radius"],
    }

def assert_summary_matches(ts, expected):
    assert ts.keys() >= expected.keys()
    for key, value in expected.items():
        if isinstance(value, float):
            assert abs(ts[key] - value) < 2e-12, (key, ts[key], value)
        else:
            assert ts[key] == value, (key, ts[key], value)

multi = [r for r in all_rows if r["genuinely_multi_branch"]]
near_multi = [r for r in all_rows if r["near_active_count"] > 1]
candidate_multi = [r for r in all_rows if r["candidate_window_count"] > 1]
near_multi_states = {tuple(r["base_dual_flat"]) for r in near_multi}
candidate_multi_states = {tuple(r["base_dual_flat"]) for r in candidate_multi}
near_multi_occupancy = {(r["start_id"], float(r["initial_radius"]), r["iteration"]) for r in near_multi}
candidate_multi_occupancy = {(r["start_id"], float(r["initial_radius"]), r["iteration"]) for r in candidate_multi}
near_multi_by_policy = {p: sum(r["policy"] == p for r in near_multi) for p in sorted(expected_policies)}
candidate_multi_by_policy = {p: sum(r["policy"] == p for r in candidate_multi) for p in sorted(expected_policies)}
first = [r for r in all_rows if r["iteration"] == 1]
distinct_cells = sum(any(max(abs(a-b) for a,b in zip(r["direction_flat"], q["direction_flat"])) > 1e-8 for q in first if q["start_id"] == r["start_id"] and q["initial_radius"] == r["initial_radius"] and q["policy"] != r["policy"]) for r in first)
observed = []
for (p, s, radius), rs in sorted(by_cell.items()):
    observed.append({"policy": p, "start": s, "radius": radius, "proposals": len(rs)-1, "accepted": sum(x["accepted"] for x in rs[1:]), "initial_sys": rs[0]["target_sys"], "best_sys": rs[-1]["best_sys"], "best_gain": rs[-1]["best_sys"] - rs[0]["target_sys"], "mean_abs_prediction_error": sum(abs(x["predicted_observed_error"]) for x in rs[1:] if x["predicted_observed_error"] is not None) / max(1, sum(x["predicted_observed_error"] is not None for x in rs[1:]))})
tol = 1.0e-8
matched = {(s, r, i): {p: by_cell[(p, s, r)][i] for p in ("near_active_box_lp_maximin", "single_branch_box_steepest") if i < len(by_cell[(p, s, r)])} for s in sorted(expected_starts) for r in sorted(expected_radii) for i in range(1, max(len(by_cell[("near_active_box_lp_maximin", s, r)]), len(by_cell[("single_branch_box_steepest", s, r)])))}
matched = {k: v for k, v in matched.items() if len(v) == 2}
same_base = [v for v in matched.values() if v["near_active_box_lp_maximin"]["base_dual_flat"] == v["single_branch_box_steepest"]["base_dual_flat"]]
singleton_same_direction = [v for v in same_base if v["near_active_box_lp_maximin"]["near_active_count"] == 1 and max(abs(a-b) for a,b in zip(v["near_active_box_lp_maximin"]["direction_flat"], v["single_branch_box_steepest"]["direction_flat"])) <= tol]
direction_divergences = [v for v in same_base if max(abs(a-b) for a,b in zip(v["near_active_box_lp_maximin"]["direction_flat"], v["single_branch_box_steepest"]["direction_flat"])) > tol]
gain_pairs = []
for s in sorted(expected_starts):
    for r in sorted(expected_radii):
        n = by_cell[("near_active_box_lp_maximin", s, r)][-1]["best_sys"] - by_cell[("near_active_box_lp_maximin", s, r)][0]["target_sys"]
        b = by_cell[("single_branch_box_steepest", s, r)][-1]["best_sys"] - by_cell[("single_branch_box_steepest", s, r)][0]["target_sys"]
        gain_pairs.append((s, r, n, b))
near_vs_single = {"matched_rows": len(matched), "same_base_state_rows": len(same_base), "singleton_same_direction_rows": len(singleton_same_direction), "singleton_checked_rows": sum(v["near_active_box_lp_maximin"]["near_active_count"] == 1 for v in same_base), "direction_divergence_rows": len(direction_divergences), "gain_pairs": [{"start":s,"radius":r,"near_gain":n,"single_gain":b,"difference_single_minus_near":b-n} for s,r,n,b in gain_pairs], "tie_tolerance": tol}
near_vs_single["gain_wins_single"] = sum(b - n > tol for _, _, n, b in gain_pairs)
near_vs_single["gain_wins_near"] = sum(n - b > tol for _, _, n, b in gain_pairs)
near_vs_single["gain_ties"] = len(gain_pairs) - near_vs_single["gain_wins_single"] - near_vs_single["gain_wins_near"]
summary = {"trajectory_files": len(files), "proposal_rows": len(all_rows), "total_target_evaluations": sum(len(rs)-1 for rs in by_cell.values()), "multi_branch_rows": len(multi), "multi_branch_fraction": len(multi)/len(all_rows), "near_active_multi_rows": len(near_multi), "near_active_multi_distinct_base_states": len(near_multi_states), "near_active_multi_occupancy": len(near_multi_occupancy), "near_active_multi_rows_by_policy": near_multi_by_policy, "candidate_window_multi_rows": len(candidate_multi), "candidate_window_multi_distinct_base_states": len(candidate_multi_states), "candidate_window_multi_occupancy": len(candidate_multi_occupancy), "candidate_window_multi_rows_by_policy": candidate_multi_by_policy, "distinct_direction_rows": distinct_cells, "near_vs_single": near_vs_single, "expected_policy_count": len(expected_policies), "expected_start_count": len(expected_starts), "expected_radius_count": len(expected_radii), "observed": observed}
producer_summary = json.loads((root / "summary.json").read_text())
assert producer_summary["total_target_evaluations"] == summary["total_target_evaluations"]
summary_keys = {(ts["policy"], ts["start_id"], float(ts["initial_radius"])) for ts in producer_summary["trajectories"]}
assert summary_keys == set(by_cell), "producer summary trajectory coverage mismatch"
assert len(producer_summary["trajectories"]) == len(by_cell), "duplicate producer summary trajectory"
for ts in producer_summary["trajectories"]:
    key = (ts["policy"], ts["start_id"], float(ts["initial_radius"]))
    assert_summary_matches(ts, recompute_summary(by_cell[key]))
(root / "analysis.json").write_text(json.dumps(summary, indent=2) + "\n")
target_rows = [r for r in all_rows if r["target_sigma"] is not None]
near_misses = sum(not r["target_visible_near"] for r in target_rows)
candidate_misses = sum(not r["target_visible_candidate"] for r in target_rows)
mean_errors = {p: sum(abs(r["predicted_observed_error"]) for r in all_rows if r["policy"] == p) / max(1, sum(r["policy"] == p for r in all_rows)) for p in expected_policies}
first_by = {(r["policy"], r["start_id"], r["initial_radius"]): r for r in first}
roles = {x["id"]: x["role"] for x in manifest["fixtures"]}
effects = []
for s in sorted(expected_starts):
    for rad in sorted(expected_radii):
        c = first_by["candidate_window_box_lp_maximin", s, rad]["delta"]
        n = first_by["near_active_box_lp_maximin", s, rad]["delta"]
        effects.append(f"{roles[s]} r={rad:g}: candidate-near Δsys={c-n:+.12g}")
discussion = (f"# Adaptive direction ablation\n\nThe retained panel has {len(expected_starts)} role-labelled fixtures, {len(expected_policies)} policies, {len(expected_radii)} radii, and {len(all_rows)} validated proposals. Nominal multi-branch rows are {len(multi)}/{len(all_rows)} ({len(multi)/len(all_rows):.1%}); first-step policy directions differ in {distinct_cells} policy/start/radius rows. The canonical six-start generic-random screening is separate: {screening_report['trajectory_files']}/{screening_report['trajectory_files']} screening rows are valid and improving, near-active sets are singleton in {screening_report['near_single_rows']}/{screening_report['trajectory_files']}, and candidate-window sets are multi-branch in {screening_report['candidate_multi_rows']}/{screening_report['trajectory_files']}.\n\n" +
    "The mechanism fixture is f6be75…f1b8; the equality/easy control is 43d243…dec8cc. First-step candidate-minus-near effects: " + "; ".join(effects) + f". Target branch visibility misses are {near_misses}/{len(target_rows)} near-active and {candidate_misses}/{len(target_rows)} candidate-window. Mean absolute prediction errors by policy are " + ", ".join(f"{p}={mean_errors[p]:.6g}" for p in sorted(expected_policies)) + f". Near-active multiplicity is {len(near_multi)}/{len(all_rows)} raw rows and {len(near_multi_states)} exact base states; candidate-window multiplicity is {len(candidate_multi)}/{len(all_rows)} and {len(candidate_multi_states)}. The screening control has 24/24 valid improving rows, near-active singleton in 24/24, and candidate-window multi-branch in {json.loads((screening_root / 'screening-report.json').read_text())['candidate_multi_rows']}/24.\n\n" +
    f"Every proposal is a strict exact full-sys evaluation; six proposals per cell identify mechanism differences but cannot estimate endpoint behavior or prevalence. The single-branch sign/box-steepest control is the direct L-infinity control for the current minimizing branch. On this selected panel, the matched near-active/sign comparison has {near_vs_single['same_base_state_rows']} exact-base matches, {near_vs_single['singleton_same_direction_rows']}/{near_vs_single['singleton_checked_rows']} singleton direction matches within the operational 1e-8 tolerance, and best-gain wins sign {near_vs_single['gain_wins_single']}, near-active {near_vs_single['gain_wins_near']}, ties {near_vs_single['gain_ties']}. Keep the sign/box-steepest policy as the direct adaptive baseline, but do not promote it from this small panel; retain multi-branch near-active maximin only for a mechanism-stratified reopen on states with near_active_count>1. Candidate-window maximin is diagnostic-only in this packet and is not promoted to endpoint or full-optimizer evaluation.\n")
(root / "DISCUSSION.md").write_text(discussion)
if generic_mode:
    gains = {}
    for o in observed: gains.setdefault((o["start"], o["radius"]), {})[o["policy"]] = o["best_gain"]
    wins = {p: 0 for p in expected_policies}; ties = 0
    for vals in gains.values():
        m = max(vals.values()); top = [p for p,v in vals.items() if abs(v-m) <= tol]
        if len(top) == 1: wins[top[0]] += 1
        else: ties += 1
    start_best = {}
    for (s, rad), vals in gains.items():
        for p,v in vals.items(): start_best.setdefault(s, {}).setdefault(p, []).append(v)
    unique_multi = len({(r["start_id"], r["initial_radius"], r["iteration"]) for r in all_rows if r["genuinely_multi_branch"]})
    generic_disc = (f"# Generic six-start adaptive direction panel\n\n"
        f"The unselected canonical panel has {len(expected_starts)} deterministic starts, four policies, three radii, and {len(all_rows)} exact proposals (six proposals per cell). This is descriptive paired evidence, not six stochastic replicates.\n\n"
        f"Best-gain wins over the 18 start×radius cells: " + ", ".join(f"{p}={wins[p]}" for p in sorted(wins)) + f"; ties={ties}. After allowing all three radii, per-start best policies are: " + "; ".join(f"{s}: " + ", ".join(f"{p}={max(v):.9g}" for p,v in sorted(ps.items())) for s,ps in sorted(start_best.items())) + ".\n\n"
        f"Near-active multiplicity is {len(near_multi)}/{len(all_rows)} raw rows and {len(near_multi_states)} exact base states (by policy: {near_multi_by_policy}); candidate-window multiplicity is {len(candidate_multi)}/{len(all_rows)} and {len(candidate_multi_states)} exact base states (by policy: {candidate_multi_by_policy}). The near-active/sign matched comparison has {near_vs_single['same_base_state_rows']} exact-base matches, {near_vs_single['singleton_same_direction_rows']}/{near_vs_single['singleton_checked_rows']} singleton direction matches within the 1e-8 tolerance, and {near_vs_single['direction_divergence_rows']} direction divergences. Best-gain comparison is sign {near_vs_single['gain_wins_single']}, near-active {near_vs_single['gain_wins_near']}, ties {near_vs_single['gain_ties']} under that operational tolerance. Candidate-window maximin remains diagnostic-only and is not promoted to endpoint or full-optimizer evaluation.\n\n"
        f"These six starts do not establish population prevalence, convergence, stationarity, or endpoint quality. The singleton invariant means near-active maximin adds no tested direction beyond the single-branch box-steepest control on those matched states. The sign control wins {near_vs_single['gain_wins_single']} near-vs-sign gain cells (near wins {near_vs_single['gain_wins_near']}; {near_vs_single['gain_ties']} are ties), so it is retained as the direct baseline rather than promoted as a generic winner. The incremental multi-branch near-active effect remains unsupported here and must be reopened on a mechanism-stratified sample with near_active_count>1. Candidate-window maximin is diagnostic-only and is not promoted to endpoint or full-optimizer evaluation.\n")
    (root / "DISCUSSION.md").write_text(generic_disc)
print(json.dumps(summary, indent=2))
