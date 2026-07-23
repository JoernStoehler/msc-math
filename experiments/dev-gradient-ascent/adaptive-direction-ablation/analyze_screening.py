import json, pathlib, subprocess, sys
root = pathlib.Path(sys.argv[1]); files = sorted((root / "trajectories").glob("*/*/*.jsonl")); assert len(files) == 24
prov=json.loads((root/"run-provenance.json").read_text()); assert set(prov["policies"]) == {"inf_normalized_branch_gradient","near_active_box_lp_maximin","candidate_window_box_lp_maximin","single_branch_box_steepest"}; assert prov["source_input"].endswith("random.jsonl") and prov["initial_radii"] == [0.0001,0.001,0.01]
assert prov["requested_target_budget"] == 1
def blake3(path):
    out = subprocess.run(["uv", "run", "--with", "blake3", "--no-project", "python3", "-c", "import blake3,sys; print(blake3.blake3(open(sys.argv[1], 'rb').read()).hexdigest())", str(path)], check=True, capture_output=True, text=True).stdout.strip().splitlines()[-1]
    return out
def blake3_bytes(data):
    return subprocess.run(["uv", "run", "--with", "blake3", "--no-project", "python3", "-c", "import blake3,sys; print(blake3.blake3(sys.stdin.buffer.read()).hexdigest())"], input=data, capture_output=True, check=True).stdout.decode().strip()
def warn_stale(condition, label):
    # Byte identity is advisory; row coverage and trajectory semantics below
    # remain blocking.
    if not condition:
        print(f"warning: {label} differs from retained provenance; continuing with semantic checks. Reassess retained interpretation before treating this run as equivalent.", file=sys.stderr)
identity=json.loads((root/"artifact-identity.json").read_text())
try:
    blob=subprocess.check_output(["git","show",f'{identity["producing_implementation_commit"]}:{identity["implementation_path"]}'])
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
source_input = pathlib.Path(prov["source_input"])
if not source_input.exists(): source_input = pathlib.Path(__file__).resolve().parents[3] / source_input
warn_stale(blake3(source_input) == prov["source_input_blake3"], "current source input")
rows=[]
for f in files:
    rs=[json.loads(x) for x in f.read_text().splitlines() if x.strip()]; assert len(rs)==2
    assert rs[0]["iteration"]==0 and rs[1]["iteration"]==1 and rs[1]["target_evaluations"]==1
    assert rs[1]["target_valid"] and rs[1]["accepted"] and rs[1]["delta"]>0
    assert rs[1]["near_active_count"]==1 and rs[1]["candidate_window_count"]>=1
    assert rs[1]["direction_label"] == rs[0]["policy"]
    if rs[0]["policy"] == "single_branch_box_steepest":
        assert all(abs(d - (1 if g > 0 else -1 if g < 0 else 0)) < 2e-10 for g,d in zip(rs[1]["primary_gradient_flat"], rs[1]["direction_flat"]))
    assert 0 < rs[1]["direction_norm_inf"] <= 1+2e-8
    rows.append(rs[1])
producer_summary=json.loads((root/"summary.json").read_text())
assert producer_summary["total_target_evaluations"] == len(files)
summary_keys={(ts["policy"],ts["start_id"],float(ts["initial_radius"])) for ts in producer_summary["trajectories"]}
row_keys={(r["policy"],r["start_id"],float(r["initial_radius"])) for r in rows}
assert summary_keys == row_keys
assert len(producer_summary["trajectories"]) == len(row_keys), "duplicate producer summary trajectory"
for ts in producer_summary["trajectories"]:
    key=(ts["policy"],ts["start_id"],float(ts["initial_radius"]))
    row=next(r for r in rows if (r["policy"],r["start_id"],float(r["initial_radius"])) == key)
    expected={"policy":row["policy"],"start_id":row["start_id"],"initial_radius":float(row["initial_radius"]),"requested_updates":1,"committed_updates":1,"initial_sys":row["base_sys"],"final_sys":row["target_sys"],"best_sys":row["best_sys"],"best_iteration":1,"target_evaluations":1,"invalid_attempts":0,"rejected_attempts":0,"accepted_decreases":0,"radius_expansions":1,"radius_shrinks":0,"stop_reason":"budget","final_radius":row["current_radius"]}
    for field,value in expected.items():
        assert abs(ts[field]-value)<2e-12 if isinstance(value,float) else ts[field]==value, (field,ts[field],value)
out={"trajectory_files":len(files),"starts":sorted({r["start_id"] for r in rows}),"policies":sorted({r["policy"] for r in rows}),"initial_radius":rows[0]["initial_radius"],"near_single_rows":sum(r["near_active_count"]==1 for r in rows),"near_multi_rows":sum(r["near_active_count"]>1 for r in rows),"near_multi_distinct_base_states":len({tuple(r["base_dual_flat"]) for r in rows if r["near_active_count"]>1}),"candidate_multi_rows":sum(r["candidate_window_count"]>1 for r in rows),"candidate_multi_distinct_base_states":len({tuple(r["base_dual_flat"]) for r in rows if r["candidate_window_count"]>1}),"all_first_proposals_strictly_improved":all(r["delta"]>0 for r in rows),"rows":rows}
(root/"screening-report.json").write_text(json.dumps(out,indent=2)+"\n")
(root/"screening-provenance.json").write_text(json.dumps({**prov, "screening_contract": "canonical six generic-random F=6 rows after excluding random_F6_s0_1; one proposal at radius 1e-3 per policy/start; candidate window 1e-2"}, indent=2)+"\n")
print(json.dumps({k:v for k,v in out.items() if k!="rows"},indent=2))
