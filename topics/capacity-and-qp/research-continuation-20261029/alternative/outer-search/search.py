# /// script
# requires-python = ">=3.11"
# dependencies = ["numpy>=2.0", "scipy>=1.14", "sympy>=1.13"]
# ///
"""Bounded numerical outer search over relative vertical offsets.
Each trial solves a height-coalesced approximate horizontal-shift LP; saved candidates are rescored on original points.
The LP coalesces near-equal heights within 1e-6 and perturbs the model; direct hull checks use the unmodified points. No global 2D certificate.
"""
import importlib.util, json, time, signal
from pathlib import Path
import numpy as np
from scipy.optimize import linprog, minimize
from scipy.spatial import ConvexHull
ROOT=Path('/workspaces/msc-math-viterbo-continuation')
FIX=ROOT/'topics/products/pentagon-arbitrary-partner/local-cover/pentagon_fixture.py'
spec=importlib.util.spec_from_file_location('fixture',FIX); f=importlib.util.module_from_spec(spec); spec.loader.exec_module(f)
data=f.make_fixture()
def val(c):
    x=f.Z
    for q in c: x=x*f.tau+f.F(f.sp.Rational(q))
    return float(f.F.to_sympy(x).evalf())
T=[np.array([[val(c) for c in p] for p in s['vertices']],float) for s in data['normal_shapes']]
trans=np.array([[val(c) for c in t] for t in data['candidate_translations']],float)
H=val(data['candidate_K_area'])
k=len(T)
# Normalize common vertical translation so anchor offset is zero.
hko=trans[:,1]-trans[0,1]

calls=0; failures=[]; best={'area':float('inf')}; starts=[]; t0=time.monotonic()
def objective(s, retain=False):
    global calls,best
    calls+=1
    points=[]
    for j,g in enumerate(T): points.append(g+np.array([0.,s[j]]) )
    allp=np.vstack(points); rawys=sorted(set(allp[:,1]))
    # Approximate outer-search only: coalesce vertical levels within 1e-6 to avoid ill-conditioned LP rows.
    # The selected candidate is always evaluated on the unmodified points by an independent hull.
    ys=[]; idx={}
    for y in rawys:
        if not ys or y-ys[-1] > 1e-6: ys.append(y)
        idx[y]=len(ys)-1
    m=len(ys); n=2*m+k-1
    rows=[]; rhs=[]
    for j,g in enumerate(points):
        for i,(a,y) in enumerate(g):
            r=idx[y]
            row=np.zeros(n); row[r]=1
            if j: row[2*m+j-1]=-1
            rows.append(row);rhs.append(a)
            row=np.zeros(n); row[m+r]=-1
            if j: row[2*m+j-1]=1
            rows.append(row);rhs.append(-a)
    gaps=np.diff(ys)
    for r in range(1,m-1):
        scale=max(gaps[r-1],gaps[r])
        row=np.zeros(n); row[r-1]=-gaps[r]/scale; row[r]=(gaps[r-1]+gaps[r])/scale;row[r+1]=-gaps[r-1]/scale
        rows.append(row);rhs.append(0)
        row=np.zeros(n); row[m+r-1]=gaps[r]/scale;row[m+r]=-(gaps[r-1]+gaps[r])/scale;row[m+r+1]=gaps[r-1]/scale
        rows.append(row);rhs.append(0)
    w=np.zeros(m)
    for r,gap in enumerate(gaps):w[r]+=gap/2;w[r+1]+=gap/2
    c=np.r_[-w,w,np.zeros(k-1)]
    ans=linprog(c,A_ub=np.array(rows),b_ub=np.array(rhs),bounds=[(None,None)]*n,method='highs')
    if not ans.success:
        failures.append({'call':calls,'message':ans.message,'min_gap':float(np.min(gaps)) if len(gaps) else 0})
        return 1e3
    area=float(ans.fun)
    # Independently score actual uncoalesced points at the proposed horizontal shifts.
    shifts=np.r_[0,ans.x[2*m:]]
    actual_points=np.vstack([g+np.array([shifts[j],s[j]]) for j,g in enumerate(T)])
    actual_area=float(ConvexHull(actual_points).volume)
    if actual_area<best['area']:
        best={'area':actual_area,'lp_approx_area':area,'ratio':actual_area/H,'vertical_offsets':np.array(s).tolist(),'horizontal_shifts':shifts.tolist(),
              'lp_primal':ans.x.tolist(),'lp_slacks':ans.ineqlin.residual.tolist(),'lp_marginals':ans.ineqlin.marginals.tolist(),
              'height_levels':m,'variables':n,'rows':len(rows),'call':calls,'actual_hull_area':actual_area}
    return area

# deterministic fixed starts, all relative to anchored first triangle
starts0=[('HKO',hko),('negated_HKO_offsets',-hko),('centered',np.zeros(k)),
 ('rational_remote_1',np.array([0]+[((j*3+1)%7-3)/20 for j in range(1,k)])),
 ('rational_remote_2',np.array([0]+[((j*j+2)%9-4)/15 for j in range(1,k)])),
 ('alternating',np.array([0]+[(-1)**j*0.18 for j in range(1,k)])),
 ('linear',np.linspace(0,.35,k)),('reverse_linear',np.linspace(0,-.35,k))]
# Perturbations around HKO are separate starts, intended to probe its outer landscape.
for q in range(4):
    d=np.array([0]+[(((j+2)*(q+3)*7)%11-5)/100 for j in range(1,k)])
    starts0.append((f'HKO_perturb_{q}',hko+d))

def outer_objective(v): return objective(np.r_[0.0,np.asarray(v)])

def alarm(sig,frame): raise TimeoutError('90 CPU-second budget reached')
signal.signal(signal.SIGPROF,alarm); signal.setitimer(signal.ITIMER_PROF,87)
try:
 for name,x0 in starts0:
    before=calls; begin=time.monotonic()
    initial_area=objective(x0)
    res=minimize(outer_objective,x0[1:],method='Powell',options={'maxfev':220,'maxiter':18,'xtol':2e-4,'ftol':1e-8})
    s=np.r_[0,res.x]
    starts.append({'name':name,'initial':x0.tolist(),'initial_area':initial_area,'calls_before':before,'calls_after':calls,
                   'best_area_after':best['area'],'best_ratio_after':best['ratio'],'powell_success':bool(res.success),
                   'powell_message':str(res.message),'powell_nfev':int(res.nfev),'final_offsets':s.tolist(),'final_fun':float(res.fun),
                   'seconds':time.monotonic()-begin})
    if best['area'] < H-1e-7: break
except TimeoutError:
    failures.append({'call':calls,'message':'CPU timer stopped search'})
finally:
 signal.setitimer(signal.ITIMER_PROF,0)
out={'scope':'Bounded numerical outer search; each vertical-offset trial optimizes horizontal shifts by envelope LP. No full-space proof.',
 'fixture':'exact regular-pentagon ten normal triangles from pentagon_fixture.py converted to float for outer search',
 'height_tie_policy':'Outer LP coalesces raw floating heights within 1e-6 to stabilize solver. This perturbs the sampled configuration; every saved best horizontal placement is rescored on the unmodified points by scipy ConvexHull. No exact claim from this approximation.',
 'HKO_area':H,'calls':calls,'wall_elapsed_seconds':time.monotonic()-t0,'starts':starts,'failures':failures,'best':best}
path=Path('/tmp/msc-math/viterbo-continuation-20261029/alternative/outer/results.json');path.write_text(json.dumps(out,indent=2)+'\n')
print(json.dumps({'calls':calls,'best_ratio':best['ratio'],'best_area':best['area'],'failure_count':len(failures),'starts':len(starts),'seconds':out['wall_elapsed_seconds']}))
