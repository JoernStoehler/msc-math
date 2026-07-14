import json, pathlib, sys
root = pathlib.Path(sys.argv[1]); files = sorted((root / "trajectories").glob("*/*/*.jsonl")); assert len(files) == 18
rows=[]
for f in files:
    rs=[json.loads(x) for x in f.read_text().splitlines() if x.strip()]; assert len(rs)==2
    assert rs[0]["iteration"]==0 and rs[1]["iteration"]==1 and rs[1]["target_evaluations"]==1
    assert rs[1]["target_valid"] and rs[1]["accepted"] and rs[1]["delta"]>0
    assert rs[1]["near_active_count"]==1 and rs[1]["candidate_window_count"]>=1
    assert abs(rs[1]["direction_norm"]-1)<2e-8
    rows.append(rs[1])
out={"trajectory_files":len(files),"starts":sorted({r["start_id"] for r in rows}),"policies":sorted({r["policy"] for r in rows}),"initial_radius":rows[0]["initial_radius"],"near_single_rows":sum(r["near_active_count"]==1 for r in rows),"candidate_multi_rows":sum(r["candidate_window_count"]>1 for r in rows),"all_first_proposals_strictly_improved":all(r["delta"]>0 for r in rows),"rows":rows}
(root/"screening-report.json").write_text(json.dumps(out,indent=2)+"\n")
print(json.dumps({k:v for k,v in out.items() if k!="rows"},indent=2))
