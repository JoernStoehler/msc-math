# /// script
# dependencies = ["numpy>=2.0", "scipy>=1.14"]
# ///
import json,time,os,resource
from pathlib import Path
import numpy as np
from scipy.spatial import ConvexHull
from scipy.optimize import linprog
resource.setrlimit(resource.RLIMIT_CPU,(20,25));start=time.monotonic()
root=Path('/tmp/msc-math/research-swarm-20261019/partner-geometry')
inputs=json.loads((root/'direct-cover-results-inputs.json').read_text());old=json.loads((root/'direct-cover-results.json').read_text())
T=np.asarray(inputs['triangles']);box=inputs['config']['shift_box'];direction=np.array([1.,0.]);rows=[]
for name in ['HKO','common_center','remote_10_0.25']:
 prior=next(r for r in old['results'] if r['name']==name);base=np.asarray(prior['best_translations']);x=np.zeros(9);cuts=[];trace=[];best=float('inf');bestx=x.copy();violation=0.
 for it in range(80):
  p=base.copy();p[1:]+=x[:,None]*direction;points=(T+p[:,None,:]).reshape(-1,2);H=ConvexHull(points);P=points[H.vertices];A=float(H.volume)
  delta=np.roll(P,1,axis=0)-np.roll(P,-1,axis=0);scalar=-.5*delta[:,1];grad=np.zeros(10);np.add.at(grad,H.vertices//3,scalar);g=grad[1:]
  if A<best:best=A;bestx=x.copy()
  if cuts:violation=max(violation,max(float(gg@x+bb-A) for gg,bb in cuts))
  cuts.append((g,float(A-g@x)))
  lp=linprog(np.r_[np.zeros(9),1.],A_ub=np.asarray([np.r_[gg,-1.] for gg,bb in cuts]),b_ub=np.asarray([-bb for gg,bb in cuts]),bounds=[(-box,box)]*9+[(0,None)],method='highs')
  assert lp.success
  trace.append({'evaluation':it+1,'shift':x.tolist(),'area':A,'lower_box_bound':float(lp.fun),'best_box_gap':best-float(lp.fun)})
  if best-float(lp.fun)<=1e-10:break
  x=lp.x[:9]
 final=base.copy();final[1:]+=bestx[:,None]*direction
 rows.append({'name':name,'additional_evaluations':it+1,'cumulative_evaluations_for_start':prior['objective_evaluations']+it+1,'best_over_HKO':best/old['HKO_area'],'remaining_box_gap_over_HKO':(best-float(lp.fun))/old['HKO_area'],'maximum_cut_violation':violation,'best_translations':final.tolist(),'trace':trace})
 assert rows[-1]['cumulative_evaluations_for_start']<=300
print(json.dumps({'pid':os.getpid(),'elapsed_seconds':time.monotonic()-start,'process_cpu_seconds':time.process_time(),'result_summaries':[{k:v for k,v in r.items() if k not in ['trace','best_translations']} for r in rows]},indent=2))
out=root/'direct-cover-refine-results.json';assert not out.exists();out.write_text(json.dumps({'status':'floating-point convex parallel-subproblem rehearsal only','pid':os.getpid(),'elapsed_seconds':time.monotonic()-start,'results':rows},indent=2)+'\n')
