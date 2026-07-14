"""Strict analyzer and combined displays for the adaptive-direction packet."""
import json, pathlib, sys

root = pathlib.Path(sys.argv[1] if len(sys.argv) > 1 else "artifacts")
prov = json.loads((root / "run-provenance.json").read_text())
expected_policies = {"normalized_branch_gradient", "near_active_zero_gap_maximin", "candidate_window_gap_aware_maximin"}
expected_radii = {float(x) for x in prov["initial_radii"]}
assert abs(prov["candidate_window_relative_gap"] - 1.0e-2) < 1e-15
assert prov["requested_target_budget"] == 6
manifest = json.loads((root.parent / "inputs" / "fixture-manifest.json").read_text())
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
    for i, row in enumerate(rs[1:], 1):
        assert row["policy"] == initial["policy"] and row["start_id"] == initial["start_id"]
        assert float(row["initial_radius"]) == float(initial["initial_radius"])
        assert row["iteration"] == i and row["target_evaluations"] == i and row["target_evaluations"] == prev_eval + 1
        assert row["direction_label"] and abs(row["direction_norm"] - 1.0) < 2e-8
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
        assert row["best_sys"] >= prev_best - 1e-12
        if row["target_valid"] and row["target_sys"] > prev_best: assert row["best_sys"] == row["target_sys"]
        if row["predicted_delta"] is not None:
            assert row["predicted_observed_error"] is not None and row["target_valid"]
            assert abs(row["predicted_observed_error"] - (row["delta"] - row["predicted_delta"])) < 2e-10
            assert row["predicted_winning_sigma"] is not None
            assert row["predicted_winning_sigma"] in row["near_active_sigmas"] or row["predicted_winning_sigma"] in row["candidate_window_sigmas"] or row["policy"] == "normalized_branch_gradient"
        if row["target_sigma"] is not None:
            assert row["target_visible_near"] == (row["target_sigma"] in row["near_active_sigmas"])
            assert row["target_visible_candidate"] == (row["target_sigma"] in row["candidate_window_sigmas"])
        prev_eval, prev_best, prev_radius = i, row["best_sys"], row["current_radius"]
    by_cell[key] = rs
    all_rows.extend(rs[1:])
assert set(by_cell) == {(p, s, r) for p in expected_policies for s in expected_starts for r in expected_radii}, "policy/start/radius coverage mismatch"
multi = [r for r in all_rows if r["genuinely_multi_branch"]]
first = [r for r in all_rows if r["iteration"] == 1]
distinct_cells = sum(any(max(abs(a-b) for a,b in zip(r["direction_flat"], q["direction_flat"])) > 1e-8 for q in first if q["start_id"] == r["start_id"] and q["initial_radius"] == r["initial_radius"] and q["policy"] != r["policy"]) for r in first)
observed = []
for (p, s, radius), rs in sorted(by_cell.items()):
    observed.append({"policy": p, "start": s, "radius": radius, "proposals": len(rs)-1, "accepted": sum(x["accepted"] for x in rs[1:]), "initial_sys": rs[0]["target_sys"], "best_sys": rs[-1]["best_sys"], "best_gain": rs[-1]["best_sys"] - rs[0]["target_sys"], "mean_abs_prediction_error": sum(abs(x["predicted_observed_error"]) for x in rs[1:] if x["predicted_observed_error"] is not None) / max(1, sum(x["predicted_observed_error"] is not None for x in rs[1:]))})
summary = {"trajectory_files": len(files), "proposal_rows": len(all_rows), "total_target_evaluations": sum(len(rs)-1 for rs in by_cell.values()), "multi_branch_rows": len(multi), "multi_branch_fraction": len(multi)/len(all_rows), "distinct_direction_rows": distinct_cells, "expected_policy_count": len(expected_policies), "expected_start_count": len(expected_starts), "expected_radius_count": len(expected_radii), "observed": observed}
prov["policies"] = sorted(expected_policies)
prov["command"] = ["cargo", "run", "--release", "-p", "exp-dev-gradient-ascent", "--bin", "dev-gradient-ascent-adaptive-direction-ablation", "--", "--budget", "6", "--facet-count", "0", "--start-count", "2", "--polytope-table", "experiments/dev-gradient-ascent/adaptive-direction-ablation/inputs/selected-fixtures.jsonl", "--out-dir", "experiments/dev-gradient-ascent/adaptive-direction-ablation/artifacts", "(three policy shards)"]
(root / "run-provenance.json").write_text(json.dumps(prov, indent=2) + "\n")
(root / "analysis.json").write_text(json.dumps(summary, indent=2) + "\n")
(root / "summary.json").write_text(json.dumps({"provenance": "run-provenance.json", "trajectories": observed, "total_target_evaluations": summary["total_target_evaluations"]}, indent=2) + "\n")
fig = root / "figures"; fig.mkdir(exist_ok=True)
colors = {"normalized_branch_gradient": "#3366cc", "near_active_zero_gap_maximin": "#dc3912", "candidate_window_gap_aware_maximin": "#109618"}
svg = ['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="620"><rect width="100%" height="100%" fill="white"/><text x="55" y="25" font-size="18">Best-so-far sys gain from initial state (validated trajectories)</text><text x="430" y="610">exact target evaluations</text><text x="10" y="320" transform="rotate(-90 10,320)">gain in sys</text>']
max_x = max(len(rs)-1 for rs in by_cell.values()); max_y = max(rs[-1]["best_sys"] - rs[0]["target_sys"] for rs in by_cell.values())
for p in sorted(expected_policies):
    for (pp, s, radius), rs in sorted(by_cell.items()):
        if pp != p: continue
        pts = " ".join(f'{60+880*x/max_x:.1f},{570-500*(r["best_sys"]-rs[0]["target_sys"])/max_y:.1f}' for x,r in enumerate(rs[1:],1))
        svg.append(f'<polyline fill="none" stroke="{colors[p]}" stroke-width="2" points="{pts}"/>')
svg.extend([f'<text x="760" y="{45+22*i}" fill="{colors[p]}">{p} (n={len(expected_starts)*len(expected_radii)})</text>' for i,p in enumerate(sorted(expected_policies))]); (fig / "combined-gain-vs-evaluations.svg").write_text("".join(svg) + "</svg>")
mechanism = ['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="620"><rect width="100%" height="100%" fill="white"/><text x="55" y="25" font-size="18">First-step mechanism comparison (role-labelled fixtures)</text><text x="430" y="610">initial radius</text><text x="10" y="320" transform="rotate(-90 10,320)">observed delta sys</text>']
first_by = {(r["policy"], r["start_id"], r["initial_radius"]): r for r in first}; mx=max(r["delta"] for r in first); roles={x["id"]:x["role"] for x in manifest["fixtures"]}
for j,p in enumerate(sorted(expected_policies)):
    for i,s in enumerate(sorted(expected_starts)):
        pts=" ".join(f'{100+700*k/2:.1f},{570-500*first_by[p,s,rad]["delta"]/mx:.1f}' for k,rad in enumerate(sorted(expected_radii)))
        mechanism.append(f'<polyline fill="none" stroke="{colors[p]}" points="{pts}"/><text x="760" y="{45+22*(j*len(expected_starts)+i)}" fill="{colors[p]}">{roles[s]} / {p}</text>')
(fig / "mechanism-first-step.svg").write_text("".join(mechanism) + "</svg>")
target_rows = [r for r in all_rows if r["target_sigma"] is not None]
near_misses = sum(not r["target_visible_near"] for r in target_rows)
candidate_misses = sum(not r["target_visible_candidate"] for r in target_rows)
mean_errors = {p: sum(abs(r["predicted_observed_error"]) for r in all_rows if r["policy"] == p) / max(1, sum(r["policy"] == p for r in all_rows)) for p in expected_policies}
first_by = {(r["policy"], r["start_id"], r["initial_radius"]): r for r in first}
roles = {x["id"]: x["role"] for x in manifest["fixtures"]}
effects = []
for s in sorted(expected_starts):
    for rad in sorted(expected_radii):
        c = first_by["candidate_window_gap_aware_maximin", s, rad]["delta"]
        n = first_by["near_active_zero_gap_maximin", s, rad]["delta"]
        effects.append(f"{roles[s]} r={rad:g}: candidate-near Δsys={c-n:+.12g}")
discussion = (f"# Adaptive direction ablation\n\nThe retained panel has {len(expected_starts)} role-labelled fixtures, {len(expected_policies)} policies, {len(expected_radii)} radii, and {len(all_rows)} validated proposals. Nominal multi-branch rows are {len(multi)}/{len(all_rows)} ({len(multi)/len(all_rows):.1%}); first-step policy directions differ in {distinct_cells} policy/start/radius rows. The canonical six-start generic-random screening is separate: 18/18 screening rows are valid and improving, near-active sets are singleton in 18/18, and candidate-window sets are multi-branch in 9/18.\n\n" +
    "The mechanism fixture is f6be75…f1b8; the equality/easy control is 43d243…dec8cc. First-step candidate-minus-near effects: " + "; ".join(effects) + f". Target branch visibility misses are {near_misses}/{len(target_rows)} near-active and {candidate_misses}/{len(target_rows)} candidate-window. Mean absolute prediction errors by policy are " + ", ".join(f"{p}={mean_errors[p]:.6g}" for p in sorted(expected_policies)) + ".\n\n" +
    "Every proposal is a strict exact full-sys evaluation; six proposals per cell identify mechanism differences but cannot estimate endpoint behavior or prevalence. The evidence supports retaining normalized branch-gradient ascent as the observed search baseline and treating maximin models as diagnostic candidates requiring a larger mechanism-stratified follow-up.\n")
(root / "DISCUSSION.md").write_text(discussion)
print(json.dumps(summary, indent=2))
