import json, pathlib, subprocess, sys
root = pathlib.Path(sys.argv[1]); files = sorted((root / "trajectories").glob("*/*/*.jsonl")); assert len(files) == 18
prov=json.loads((root/"run-provenance.json").read_text()); assert set(prov["policies"]) == {"inf_normalized_branch_gradient","near_active_box_lp_maximin","candidate_window_box_lp_maximin"}; assert prov["source_input"].endswith("random.jsonl") and prov["initial_radii"] == [0.0001,0.001,0.01]
assert prov["requested_target_budget"] == 1
def blake3(path):
    out = subprocess.run(["uv", "run", "--with", "blake3", "--no-project", "python3", "-c", "import blake3,sys; print(blake3.blake3(open(sys.argv[1], 'rb').read()).hexdigest())", str(path)], check=True, capture_output=True, text=True).stdout.strip().splitlines()[-1]
    return out
implementation = pathlib.Path(prov["implementation"])
if not implementation.exists(): implementation = pathlib.Path(__file__).with_name("main.rs")
source_input = pathlib.Path(prov["source_input"])
if not source_input.exists(): source_input = pathlib.Path(__file__).resolve().parents[3] / source_input
assert blake3(source_input) == prov["source_input_blake3"]
assert blake3(implementation) == prov["implementation_blake3"]
rows=[]
for f in files:
    rs=[json.loads(x) for x in f.read_text().splitlines() if x.strip()]; assert len(rs)==2
    assert rs[0]["iteration"]==0 and rs[1]["iteration"]==1 and rs[1]["target_evaluations"]==1
    assert rs[1]["target_valid"] and rs[1]["accepted"] and rs[1]["delta"]>0
    assert rs[1]["near_active_count"]==1 and rs[1]["candidate_window_count"]>=1
    assert 0 < rs[1]["direction_norm_inf"] <= 1+2e-8
    rows.append(rs[1])
out={"trajectory_files":len(files),"starts":sorted({r["start_id"] for r in rows}),"policies":sorted({r["policy"] for r in rows}),"initial_radius":rows[0]["initial_radius"],"near_single_rows":sum(r["near_active_count"]==1 for r in rows),"candidate_multi_rows":sum(r["candidate_window_count"]>1 for r in rows),"all_first_proposals_strictly_improved":all(r["delta"]>0 for r in rows),"rows":rows}
(root/"screening-report.json").write_text(json.dumps(out,indent=2)+"\n")
(root/"screening-provenance.json").write_text(json.dumps({**prov, "screening_contract": "canonical six generic-random F=6 rows after excluding random_F6_s0_1; one proposal at radius 1e-3 per policy/start; candidate window 1e-2"}, indent=2)+"\n")
print(json.dumps({k:v for k,v in out.items() if k!="rows"},indent=2))
