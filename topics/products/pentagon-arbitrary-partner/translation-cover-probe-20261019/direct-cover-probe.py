# /// script
# requires-python = ">=3.11"
# dependencies = ["numpy>=2.0", "scipy>=1.14", "sympy>=1.13"]
# ///
"""Bounded exploratory parallel-slide translation-cover rehearsal; no global claim."""
import argparse,hashlib,importlib.util,json,os,resource,time
from pathlib import Path
import numpy as np
import scipy
from scipy.spatial import ConvexHull
from scipy.optimize import linprog

parser=argparse.ArgumentParser();parser.add_argument('--out',type=Path,required=True);args=parser.parse_args()
if args.out.exists():raise FileExistsError(args.out)
resource.setrlimit(resource.RLIMIT_CPU,(50,55))
started=time.monotonic();deadline=started+55
fixture_path=Path('/workspaces/msc-math/topics/products/pentagon-arbitrary-partner/local-cover/pentagon_fixture.py')
spec=importlib.util.spec_from_file_location('fixture',fixture_path);fixture=importlib.util.module_from_spec(spec);spec.loader.exec_module(fixture)
exact=fixture.make_fixture();tau=float(np.sqrt(5-2*np.sqrt(5)))
def value(coeffs):
 out=0.
 for c in coeffs:out=out*tau+float(fixture.sp.Rational(c))
 return out
T=np.asarray([[[value(c) for c in v] for v in t['vertices']] for t in exact['normal_shapes']])
known=np.asarray([[value(c) for c in v] for v in exact['candidate_translations']]);known-=known[0]
baseline=value(exact['candidate_K_area']);diameter=float(np.max(np.linalg.norm(T[:,None,:,:]-T[None,:,:,:],axis=-1)))
box=2*max(diameter,1.)
seed=20261019;rng=np.random.default_rng(seed)
reflection=[]
for tri in T:
 found=[]
 for j,other in enumerate(T):
  a=sorted(tuple(np.round(p,12)) for p in -tri);b=sorted(tuple(np.round(p,12)) for p in other)
  if a==b:found.append(j)
 assert len(found)==1,found
 reflection.append(found[0])
reflected=-known[reflection];reflected-=reflected[0]
starts=[('HKO',known),('reflected_HKO',reflected),('common_center',np.zeros((10,2)))]
for scale in [.02,.08,.2]:
 shifts=known+rng.normal(size=(10,2))*scale;shifts-=shifts[0];starts.append((f'HKO_perturb_{scale}',shifts))
for scale in [.12,.25,.5,.12,.25,.5]:
 shifts=rng.normal(size=(10,2))*scale;shifts-=shifts[0];starts.append((f'remote_{len(starts)}_{scale}',shifts))
angles=[0.,np.pi/2,np.pi/5,3*np.pi/5,np.pi/10,7*np.pi/10]*2
config={'seed':seed,'starts':12,'per_start_eval_limit':300,'direction_updates':len(angles),'max_cuts_per_direction':18,'shift_box':box,'direction_angles':angles,'wall_limit_seconds':55,'cpu_soft_limit_seconds':50,'cpu_hard_limit_seconds':55,'pid':os.getpid(),'fixture_path':str(fixture_path),'fixture_sha256':hashlib.sha256(fixture_path.read_bytes()).hexdigest(),'script_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'numpy_version':np.__version__,'scipy_version':scipy.__version__}
args.out.parent.mkdir(parents=True,exist_ok=True);inputs=args.out.with_name(args.out.stem+'-inputs.json')
inputs.write_text(json.dumps({'config':config,'triangles':T.tolist(),'HKO_translations':known.tolist(),'HKO_area':baseline,'initial_starts':[{'name':name,'translations':p.tolist()} for name,p in starts]},indent=2)+'\n')
print(json.dumps({'pid':os.getpid(),'inputs':str(inputs),'HKO_area':baseline}),flush=True)
rows=[];global_best=(float('inf'),None);stopped='completed bounded schedule'
for name,initial in starts:
 if time.monotonic()>deadline:stopped='wall deadline';break
 count=0
 def evaluate(translations,direction=None):
  global count
  count+=1
  if count>300:raise RuntimeError('objective budget exceeded')
  if time.monotonic()>deadline:raise TimeoutError('wall deadline')
  points=(T+translations[:,None,:]).reshape(-1,2);H=ConvexHull(points);idx=H.vertices;P=points[idx]
  # CCW shoelace derivative dA/dp_i applied to common slide direction.
  if direction is None:return float(H.volume),None,idx
  delta=np.roll(P,1,axis=0)-np.roll(P,-1,axis=0)
  scalar=.5*(delta[:,0]*direction[1]-delta[:,1]*direction[0])
  grad=np.zeros(10);np.add.at(grad,idx//3,scalar)
  return float(H.volume),grad[1:],idx
 current=initial.copy();area,_,_=evaluate(current);initial_area=area;steps=[];best=current.copy();best_area=area;failure=None
 try:
  for angle in angles:
   if count>=280:break
   direction=np.array([np.cos(angle),np.sin(angle)]);base=current.copy();shifts=np.zeros(9);cuts=[];best_shift=shifts.copy();direction_best=area;last_lower=None;max_cut_violation=0.;termination='cut budget'
   for iteration in range(18):
    trial=base.copy();trial[1:]+=shifts[:,None]*direction
    area_trial,gradient,idx=evaluate(trial,direction)
    if area_trial<direction_best:direction_best=area_trial;best_shift=shifts.copy()
    if area_trial<best_area:best_area=area_trial;best=trial.copy()
    # Existing supporting planes must stay below every newly evaluated area.
    if cuts:
     violation=max(float(g@shifts+offset-area_trial) for g,offset in cuts)
     max_cut_violation=max(max_cut_violation,violation)
     if violation>1e-8:raise ArithmeticError(f'invalid numerical cut {violation}')
    cuts.append((gradient,float(area_trial-gradient@shifts)))
    Aub=np.asarray([np.r_[g,-1.] for g,offset in cuts]);bub=np.asarray([-offset for g,offset in cuts])
    lp=linprog(np.r_[np.zeros(9),1.],A_ub=Aub,b_ub=bub,bounds=[(-box,box)]*9+[(0,None)],method='highs')
    if not lp.success:raise ArithmeticError(f'cut LP failed: {lp.message}')
    last_lower=float(lp.fun)
    if direction_best-last_lower<=1e-10:
     termination='parallel box gap tolerance';break
    shifts=lp.x[:9]
   current=base.copy();current[1:]+=best_shift[:,None]*direction;area=direction_best
   steps.append({'angle':angle,'evaluations':iteration+1,'area':area,'lower_bound_in_shift_box':last_lower,'gap_in_shift_box':None if last_lower is None else area-last_lower,'termination':termination,'best_shift_boundary_hit':bool(np.max(np.abs(best_shift))>=box-1e-8),'maximum_observed_cut_violation':max_cut_violation})
   if best_area<baseline-1e-8:stopped='apparent improvement requires independent exact reconstruction';break
 except (TimeoutError,RuntimeError,ArithmeticError) as exc:
  failure=str(exc);stopped=f'owned stop: {exc}'
 row={'name':name,'initial_area':initial_area,'best_area':best_area,'best_over_HKO':best_area/baseline,'objective_evaluations':count,'best_translations':best.tolist(),'best_hull_vertices':ConvexHull((T+best[:,None,:]).reshape(-1,2)).vertices.tolist(),'steps':steps,'failure_or_stop':failure}
 rows.append(row)
 if best_area<global_best[0]:global_best=(best_area,name)
 print(json.dumps({'start':name,'evaluations':count,'best_over_HKO':best_area/baseline,'failure':failure}),flush=True)
 args.out.write_text(json.dumps({'status':'exploratory floating point only; not a global or local optimality certificate','config':config,'HKO_area':baseline,'elapsed_seconds':time.monotonic()-started,'process_cpu_seconds':time.process_time(),'stop_reason':stopped,'results':rows},indent=2)+'\n')
 if failure or best_area<baseline-1e-8:break
print(json.dumps({'completed_starts':len(rows),'best_start':global_best[1],'best_over_HKO':global_best[0]/baseline,'stop_reason':stopped,'output':str(args.out),'wall_seconds':time.monotonic()-started}),flush=True)
