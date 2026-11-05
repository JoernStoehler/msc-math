#!/usr/bin/env python3
"""One unchanged outer chord-star plus exact original-centroid sandwich."""
import gzip,hashlib,importlib.util,json,sys,time
from fractions import Fraction as F
from pathlib import Path
sys.set_int_max_str_digits(0)
D=Path(__file__).resolve().parent;R=Path('/workspaces/msc-math-worktrees/patterns-2026-10-17')
spec=importlib.util.spec_from_file_location('prune_chord',D/'probe.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
def main():
 t=time.perf_counter();source=D/'facet-prediction.json';pred=json.loads(source.read_text() if source.exists() else gzip.decompress((D/'facet-certificates.json.gz').read_bytes()));z=pred['rows'][0]['selection'];outer=[];selection=[]
 for key,normal,weight in zip(['A','B'],[z['centered_A'],z['centered_B']],z['canonical_closures']):
  a=[[F(t) for t in v] for v in normal];c=list(map(F,weight));chosen=c.index(min(c));assert sum(x==min(c) for x in c)==1;ao=[u for i,u in enumerate(a) if i!=chosen]
  cent,cc,area,origin=m.centroid(ao);assert area>0 and min(cc)>0;vertices=m.sh.old.vertices(ao);supports=[max(m.dot(u,v) for v in vertices) for u in a];radius=max(supports);assert radius>=1;outer.append(ao)
  selection.append(dict(factor=key,deleted_index=chosen,cone_volume_weight=c[chosen],original_centered_normals=a,outer_normals_at_original_centroid=ao,outer_vertices_at_original_centroid=vertices,all_original_supports_on_outer=supports,R=radius))
 feature=m.select(*outer);ra,rb=[x['R'] for x in selection];lower=feature['eta']*feature['capacity']/(ra*rb);upper=feature['capacity'];combined_lower=max(F(z['lower_capacity']),lower);combined_upper=min(F(z['capacity']),upper)
 out=dict(rows=[dict(name=pred['rows'][0]['name']+'_pruned_outer',selection=feature)],pruning=selection,lower_capacity=lower,upper_capacity=upper,combined_lower=combined_lower,combined_upper=combined_upper,plan_sha256=hashlib.sha256((D/'PRUNING-PLAN.md').read_bytes()).hexdigest(),elapsed_seconds=time.perf_counter()-t)
 path=D/'pruning-prediction.json';assert not path.exists();path.write_text(json.dumps(m.serial(out),indent=2)+'\n')
 truth=json.loads((R/'patterns/upper-tail/facet-transfer/results.json').read_text())['rows'][0];ct=F(truth['capacity_true']);assert combined_lower<=ct<=combined_upper
 summary=dict(name=pred['rows'][0]['name'],eta_out=float(feature['eta']),R_A=float(ra),R_B=float(rb),C_out=float(feature['capacity']),lower_capacity=float(lower),combined_lower=float(combined_lower),combined_upper=float(combined_upper),combined_interval_factor=float(combined_upper/combined_lower),combined_lower_over_true=float(combined_lower/ct),upper_over_true=float(combined_upper/ct),deleted_indices=[x['deleted_index'] for x in selection],predictions_sha256=hashlib.sha256(path.read_bytes()).hexdigest(),elapsed_seconds=out['elapsed_seconds'])
 (D/'pruning-summary.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps(summary,indent=2))
if __name__=='__main__':main()
