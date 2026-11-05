# /// script
# dependencies = ["numpy==2.5.4", "scipy==1.18.1", "matplotlib==3.10.8"]
# ///
"""Geometry-only width bounds and central-symmetrization contrasts.

Run via `uv run analyze.py --source /path/random-product.jsonl --out /tmp/...`.
Does not regenerate inputs or recompute historical capacities. Only output dir
is written. exact_controls.py separately verifies the rational counterexample.
"""
import argparse,csv,gzip,hashlib,json,time
from pathlib import Path
import numpy as np
from scipy.spatial import ConvexHull
from scipy.stats import spearmanr
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from exact_controls import verify

SOURCE_SHA='66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736'
def hull(v): return ConvexHull(v)
def primal(a):
    a=a[np.argsort(np.arctan2(a[:,1],a[:,0]))]
    return np.array([np.linalg.solve(np.stack([a[i],a[(i+1)%len(a)]]),np.ones(2)) for i in range(len(a))])
def difference(v):
    d=(v[:,None,:]-v[None,:,:]).reshape(-1,2)
    return d[hull(d).vertices]
def polar(v):
    eq=hull(v).equations
    return eq[:,:2]/(-eq[:,2,None])
def c2_value(dp,dq):
    eq=hull(dp).equations
    return float(np.min(-eq[:,2]/np.max(eq[:,:2]@polar(dq).T,axis=1)))
def boundary_c2(dp,dq):
    # Independent direct boundary minimization: convex piecewise-linear
    # support function on each edge, minimized at endpoints or line crossings.
    best=float('inf')
    for i,start in enumerate(dp):
        delta=dp[(i+1)%len(dp)]-start
        intercept=dq@start; slope=dq@delta
        t=[0.,1.]
        for j in range(len(dq)):
            for k in range(j):
                den=slope[j]-slope[k]
                if abs(den)>1e-15:
                    x=(intercept[k]-intercept[j])/den
                    if 0<=x<=1: t.append(x)
        best=min(best,float(np.min(np.max(intercept[:,None]+slope[:,None]*np.array(t)[None,:],axis=0))))
    return best

def quantiles(values):
    return dict(zip(['min','q10','median','q90','max'],map(float,np.quantile(values,[0,.1,.5,.9,1]))))
def aggregate(rows):
    gain=np.array([r['original_over_sym'] for r in rows])
    return dict(n=len(rows),original_sys=quantiles([r['sys'] for r in rows]),
                original_over_sym=quantiles(gain),symmetrization_increases_sys=int(sum(gain<1-1e-9)),
                symmetrization_decreases_sys=int(sum(gain>1+1e-9)),
                c_over_c2=quantiles([r['c_over_c2'] for r in rows]),
                width_bound_above_one=sum(r['sys2']>1 for r in rows))

def figures(rows,out):
    eligible=[r for r in rows if r['k']>=5]
    fig,ax=plt.subplots(2,2,figsize=(11,8),layout='constrained')
    p=np.array([[-1,-1],[2,-1],[-1,2]])
    sp=difference(p)/2; rot=np.array([[0,-1],[1,0]])
    for a,m,label,sym in [(ax[0,0],np.eye(2),'P × P','2/9'),(ax[0,1],rot,'P × JP','8/9')]:
        for v,color,name in [(p,'#2877ad','P'),(p@m.T,'#c96b19','Q')]:
            vv=v[hull(v).vertices];vv=np.vstack([vv,vv[0]])
            a.plot(*vv.T,color=color,lw=2,label=name)
        for v,color in [(sp,'#2877ad'),(sp@m.T,'#c96b19')]:
            vv=np.vstack([v,v[0]]);a.plot(*vv.T,color=color,lw=1.4,ls='--')
        a.set_aspect('equal');a.set_title(f'{label}: ratio 1/2 → {sym}')
        a.set_xlabel('Solid: original factors; dashed: central symmetrizations')
        a.set_xlim(-2.2,2.2);a.set_ylim(-2.2,2.2);a.grid(alpha=.15)
    a=ax[1,0]
    asym=np.array([r['asym_p']*r['asym_q'] for r in eligible])
    a.scatter(asym,[r['sys'] for r in eligible],s=5,alpha=.3,color='#2877ad',rasterized=True)
    a.set(xlabel='Area asymmetry product α(P) α(Q)',ylabel='Original historical systolic ratio',title='Observed asymmetry association is negative')
    a=ax[1,1]
    a.scatter(asym,[r['original_over_sym'] for r in eligible],s=5,alpha=.3,color='#2877ad',rasterized=True)
    a.axhline(1,color='#b1433c',ls='--');a.set(xlabel='Area asymmetry product α(P) α(Q)',ylabel='Original ratio / symmetrized ratio',title='Yet symmetrization usually decreases the ratio')
    fig.suptitle('Symmetry is an intervention, not a sign inferred from correlation\nBottom panels: 3,072 retained 5×5, 5×6 and 6×6 products',fontsize=14)
    fig.savefig(out/'symmetrization.png',dpi=160);plt.close(fig)
    fig,a=plt.subplots(figsize=(7,5),layout='constrained')
    a.scatter([r['sys2'] for r in eligible],[r['sys'] for r in eligible],s=7,alpha=.35,color='#2877ad')
    passing=[r for r in eligible if r['sys2']>1]
    a.scatter([r['sys2'] for r in passing],[r['sys'] for r in passing],s=20,color='#c96b19',label='26 pass width > 1 screen')
    a.plot([0,1.4],[0,1.4],ls='--',color='gray',label='Capacity equals two-bounce ceiling')
    a.axvline(1,ls=':',color='gray');a.axhline(1,ls=':',color='gray');a.set(xlim=(0,1.38),ylim=(0,1.05),xlabel='Geometry-only two-bounce ratio ceiling s₂',ylabel='Original historical systolic ratio',title='The width screen is cheap; its survivors are false positives')
    a.legend(fontsize=8);fig.savefig(out/'width-screen.png',dpi=160);plt.close(fig)

def main():
    ap=argparse.ArgumentParser();ap.add_argument('--source',type=Path,required=True);ap.add_argument('--out',type=Path,required=True);args=ap.parse_args()
    started=time.perf_counter();digest=hashlib.sha256(args.source.read_bytes()).hexdigest();assert digest==SOURCE_SHA
    args.out.mkdir(parents=True,exist_ok=True)
    rows=[];checks=[]
    for index,line in enumerate(args.source.open()):
        r=json.loads(line);a=np.array(r['dual_vertices']);p=primal(a[:r['k'],:2]);q=primal(a[r['k']:,2:]);dp=difference(p);dq=difference(q);c2=c2_value(dp,dq)
        area_p=hull(p).volume;area_q=hull(q).volume
        alpha_p=hull(dp).volume/(4*area_p);alpha_q=hull(dq).volume/(4*area_q)
        s2=c2*c2/(2*area_p*area_q);sym_sys=s2/(alpha_p*alpha_q)
        row=dict(name=r['name'],k=r['k'],m=r['m'],sys=r['sys'],capacity=r['capacity'],c2=c2,
                 sys2=s2,asym_p=alpha_p,asym_q=alpha_q,c_over_c2=r['capacity']/c2,
                 sym_sys=sym_sys,original_over_sym=r['sys']/sym_sys,
                 volume_relative_error=abs(area_p*area_q/r['volume']-1),producer_bounces=r['bounces'])
        rows.append(row)
        if index%320==0 or (r['k']>=5 and s2>1):
            independent=boundary_c2(dp,dq);error=abs(independent/c2-1);assert error<1e-10
            checks.append(dict(name=r['name'],relative_error=error))
    assert len(rows)==10240 and len(set(r['name'] for r in rows))==10240
    assert max(r['c_over_c2'] for r in rows)<1+1e-9
    assert max(r['volume_relative_error'] for r in rows)<1e-9
    eligible=[r for r in rows if r['k']>=5];passing=[r for r in eligible if r['sys2']>1]
    with gzip.GzipFile(filename=str(args.out/'features.csv.gz'),mode='wb',mtime=0) as f:
        import io
        text=io.TextIOWrapper(f,encoding='utf8',newline='');writer=csv.DictWriter(text,fieldnames=list(rows[0]));writer.writeheader();writer.writerows(rows);text.flush()
    summary=dict(source_sha256=digest,source_rows=len(rows),label_contract='Original historical capacity/sys; numerical volumes. No old-target certification.',
                 formula='original/sym = alpha(P)*alpha(Q)*(c/A2)^2 (using common factor-area volume)',
                 all=aggregate(rows),eligible_side_counts=aggregate(eligible),eligible_width_survivors=aggregate(passing),
                 buckets={f'{k}x{m}':aggregate([r for r in rows if (r['k'],r['m'])==(k,m)]) for k,m in sorted(set((r['k'],r['m']) for r in rows))},
                 spearman_asymmetry_sys_all=float(spearmanr([r['asym_p']*r['asym_q'] for r in rows],[r['sys'] for r in rows]).statistic),
                 spearman_asymmetry_sys_eligible=float(spearmanr([r['asym_p']*r['asym_q'] for r in eligible],[r['sys'] for r in eligible]).statistic),
                 boundary_formula_checks=dict(n=len(checks),max_relative_error=max(c['relative_error'] for c in checks)),
                 maximum_volume_relative_error=max(r['volume_relative_error'] for r in rows),
                 minimum_distance_of_original_over_sym_from_one=min(abs(r['original_over_sym']-1) for r in rows),
                 width_survivor_rows=passing,exact_controls=verify())
    figures(rows,args.out);summary['elapsed_seconds']=time.perf_counter()-started
    (args.out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n');(args.out/'exact-controls.json').write_text(json.dumps(verify(),indent=2)+'\n')
    print(json.dumps(dict(status='PASS',rows=len(rows),boundary_checks=summary['boundary_formula_checks'],elapsed_seconds=summary['elapsed_seconds'])))
if __name__=='__main__':main()
