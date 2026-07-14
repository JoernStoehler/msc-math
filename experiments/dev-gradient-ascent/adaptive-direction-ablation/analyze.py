import json, pathlib, sys
root=pathlib.Path(sys.argv[1] if len(sys.argv)>1 else "artifacts")
files=sorted((root/"trajectories").glob("*/*/*.jsonl")); assert files
rows=[]
for f in files:
 rs=[json.loads(x) for x in f.read_text().splitlines() if x.strip()]; assert rs[0]["iteration"]==0
 prev=0
 for r in rs:
  assert r["target_evaluations"]>=prev; prev=r["target_evaluations"]
  if r["iteration"]>0:
   assert abs(r["direction_norm"]-1)<2e-8
   assert r["target_evaluations"]==r["iteration"]
   if r["accepted"]: assert r["target_valid"] and r["delta"]>0
 rows += rs[1:]
policies=sorted({r["policy"] for r in rows}); starts=sorted({r["start_id"] for r in rows}); radii=sorted({r["initial_radius"] for r in rows})
multi=[r for r in rows if r["genuinely_multi_branch"]]; distinct=0
for s in starts:
 for rad in radii:
  x=[r for r in rows if r["start_id"]==s and r["initial_radius"]==rad and r["iteration"]==1]
  if len(x)>1 and any(max(abs(a-b) for a,b in zip(x[0]["direction_flat"],q["direction_flat"]))>1e-8 for q in x[1:]): distinct+=1
obs=[]
for p in policies:
 for s in starts:
  for rad in radii:
   x=[r for r in rows if r["policy"]==p and r["start_id"]==s and r["initial_radius"]==rad]
   if x: obs.append({"policy":p,"start":s,"radius":rad,"proposals":len(x),"accepted":sum(r["accepted"] for r in x),"best_gain":max(r["best_sys"] for r in x)-x[0]["best_sys"]})
out={"trajectory_files":len(files),"proposal_rows":len(rows),"multi_branch_rows":len(multi),"multi_branch_fraction":len(multi)/len(rows),"distinct_direction_start_radius_pairs":distinct,"pair_count":len(starts)*len(radii),"policies":policies,"observed":obs}
(root/"analysis.json").write_text(json.dumps(out,indent=2)+"\n")
(root/"summary.json").write_text(json.dumps({"trajectory_files":len(files),"total_target_evaluations":sum(len([r for r in rows if r["policy"]==p and r["start_id"]==s and r["initial_radius"]==rad]) for p in policies for s in starts for rad in radii),"proposals":len(rows),"policies":policies},indent=2)+"\n")
fig=root/"figures"; fig.mkdir(exist_ok=True)
for p in policies:
 pts=[r for r in rows if r["policy"]==p]; mx=max(r["best_sys"] for r in pts); n=max(r["target_evaluations"] for r in pts)
 lines=[]
 for s in starts:
  for rad in radii:
   x=sorted((r for r in pts if r["start_id"]==s and r["initial_radius"]==rad),key=lambda r:r["target_evaluations"])
   lines.append(" ".join(str(r["target_evaluations"])+","+str(r["best_sys"]) for r in x))
 svg="<svg xmlns=\"http://www.w3.org/2000/svg\" width=900 height=500><text x=20 y=25>"+p+" best-so-far sys</text>"
 for j,line in enumerate(lines):
  ps=[]
  for pair in line.split():
   x,y=map(float,pair.split(",")); ps.append(f"{40+820*x/max(n,1):.1f},{450-390*y/mx:.1f}")
  joined=" ".join(ps); svg+=f"<polyline fill=\"none\" stroke=\"#{(j*7919)%0xffffff:06x}\" points=\"{joined}\"/>"
 (fig/("best-so-far-"+p+".svg")).write_text(svg+"</svg>")
disc="# Adaptive direction ablation\n\nGenerated from "+str(len(files))+" trajectories (n="+str(len(starts)*len(radii))+" paired start/radius cells).\n\nDirect observations: "+str(len(multi))+" / "+str(len(rows))+" proposals had a nominal multi-branch set; "+str(distinct)+" start/radius pairs had distinct first directions. The six generic starts were single-branch in smoke, so retained output is the separate narrow-gap hard-state slice. Its base state has four nominal branches; later accepted states collapse to one. This deterministic evidence is descriptive and does not establish stationarity or local maximality.\n"
(root/"DISCUSSION.md").write_text(disc)
print(json.dumps(out,indent=2))
