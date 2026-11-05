#!/usr/bin/env python3
"""Ridge-matrix control/retained-target checks; NumPy only, no capacity producer.
Writes only --out. Exact proof premises are in REPORT.md; this is corroboration.
"""
import argparse,gzip,hashlib,json,math,time
from pathlib import Path
import numpy as np

SOURCE_SHA='66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736'
TABLE_SHA='466068ace1248c0d2ab3bb55bfa8269bf8597ad475ede6bbd3739d87500ddd45'
COV_SHA='88cba83c48996309d6f4f050adb44413d449acb873d3222c515040087a7c8301'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def primal(a):
    a=a[np.argsort(np.arctan2(a[:,1],a[:,0]))]
    return np.array([np.linalg.solve(np.stack([a[i],a[(i+1)%len(a)]]),np.ones(2)) for i in range(len(a))])
def area(p):return abs(np.sum(p[:,0]*np.roll(p[:,1],-1)-p[:,1]*np.roll(p[:,0],-1)))/2
def edges(p):return np.roll(p,-1,axis=0)-p
def quant(x):return dict(zip(['min','q10','median','q90','max'],map(float,np.quantile(x,[0,.1,.5,.9,1]))))
def aggregate(rows):
    q=np.array([r['sys_times_rmax_squared'] for r in rows]); top=sorted(rows,key=lambda r:r['sys'],reverse=True)[:math.ceil(.1*len(rows))]
    return dict(rows=len(rows),above_affine_value=int(sum(q>8/25+1e-10)),quantiles=quant(q),law_prediction_over_target=quant(8/25/q),upper_decile=dict(rows=len(top),quantiles=quant([r['sys_times_rmax_squared'] for r in top])),largest_violation=max(rows,key=lambda r:r['sys_times_rmax_squared']),largest_sys=max(rows,key=lambda r:r['sys']))
def controls():
    ph=2*np.pi*np.arange(5)/5; n=np.c_[np.cos(ph),np.sin(ph)];h=np.cos(np.pi/5);w=1+h;ell=2*np.sin(np.pi/5);A0=5*np.sin(np.pi/5)*h
    p=primal(n/h);e=edges(p);rng=np.random.default_rng(170526);J=np.array([[0.,-1.],[1.,0.]])
    errors=[];dets=[]
    for k in range(256):
        while True:
            G=rng.normal(size=(2,2));H=rng.normal(size=(2,2))
            if abs(np.linalg.det(G))>.2 and abs(np.linalg.det(H))>.2:break
        D=abs(np.linalg.det(G)*np.linalg.det(H));M=np.linalg.inv(G)@np.linalg.inv(H).T;m=abs(n@M@n.T).max();V=A0*A0*D
        B=(e@G.T)@(e@H.T).T;rmax=abs(B).max()/np.sqrt(V);sys=(w*w/m)**2/(2*V)
        errors.append(abs(sys*rmax*rmax/(8/25)-1));dets.append(int(np.sign(np.linalg.det(G)*np.linalg.det(H))))
        assert np.max(abs(J.T@G.T@H@J-np.linalg.det(G)*np.linalg.det(H)*M))<1e-12
    sums=[];sing=[];powers=[];sumsys=[];operr=[]
    for theta in np.linspace(0,np.pi/10,101):
        R=np.array([[np.cos(theta),-np.sin(theta)],[np.sin(theta),np.cos(theta)]]);B=e@(e@R.T).T;A=abs(B)
        powers.append([np.sum(A**power) for power in (2,4,6,8)]);sing.append(np.linalg.svd(B,compute_uv=False)[:2]);sums.append(A.sum()/A.max())
        operr.append(abs(np.linalg.svd(A,compute_uv=False)[0]/(A.sum()/5)-1))
        sys=(w*w/np.cos(theta))**2/(2*A0*A0);sumsys.append(sys*(A.sum()/A0)**2)
    powers=np.array(powers);sing=np.array(sing)
    return dict(affine_cases=256,negative_determinant_products=dets.count(-1),maximum_law_relative_error=max(errors),rotation_angles=101,power_sum_relative_drifts=dict(zip(['2','4','6','8'],map(float,np.ptp(powers,axis=0)/powers[0]))),signed_nonzero_singular_value_relative_drift=float(np.max(np.ptp(sing,axis=0)/sing[0])),max_sum_over_max_constant_error=float(max(abs(np.array(sums)-5*(1+np.sqrt(5))))),max_unsigned_operator_norm_relative_error=max(operr),max_rotation_sum_law_absolute_error=float(max(abs(np.array(sumsys)-(48+16*np.sqrt(5))))))
def main():
    ap=argparse.ArgumentParser();ap.add_argument('--repo',type=Path,default=Path(__file__).resolve().parents[2]);ap.add_argument('--source',type=Path,required=True);ap.add_argument('--out',type=Path,required=True);args=ap.parse_args();started=time.perf_counter()
    table_path=args.repo/'topics/empirical-studies/retrospective-revalidation/geometry-full/polytope-table.jsonl.gz';cov_path=args.repo/'topics/empirical-studies/sys-datascience/methods/canonical-vertex-covariance/artifacts/current/per_polytope.jsonl'
    assert sha(args.source)==SOURCE_SHA and sha(table_path)==TABLE_SHA and sha(cov_path)==COV_SHA
    with gzip.open(table_path,'rt') as f:table={r['poly_id']:r for r in map(json.loads,f)}
    cov={r['source_name']:r for r in map(json.loads,cov_path.open())};rows=[];ridge_err=[];volume_err=[]
    for r in map(json.loads,args.source.open()):
        a=np.array(r['dual_vertices']);p=primal(a[:r['k'],:2]);q=primal(a[r['k']:,2:]);v=area(p)*area(q);A=abs(edges(p)@edges(q).T);rmax=A.max()/np.sqrt(v)
        old=table[cov[r['name']]['poly_id']];assert old['sys']==r['sys']
        ridge_err.append(abs(rmax/old['ridge_symp_area_max_over_volume_sqrt']-1));volume_err.append(abs(v/r['volume']-1))
        rows.append(dict(name=r['name'],k=r['k'],m=r['m'],sys=r['sys'],normalized_maximum_ridge_area=float(rmax),sys_times_rmax_squared=float(r['sys']*rmax*rmax)))
    assert len(rows)==10240 and len(set(r['name'] for r in rows))==10240
    assert max(ridge_err)<1e-9 and max(volume_err)<1e-9
    out=dict(status='PASS',source_sha256=SOURCE_SHA,table_sha256=TABLE_SHA,covariance_join_sha256=COV_SHA,all_products=aggregate(rows),pentagon_products=aggregate([r for r in rows if r['k']==r['m']==5]),max_reconstructed_ridge_relative_error=max(ridge_err),max_reconstructed_volume_relative_error=max(volume_err),controls=controls(),label_contract='historical numerical capacities/sys and numerical volume; no new capacity evaluation; known regular/HKO controls separate from retained random data',elapsed_seconds=time.perf_counter()-started)
    args.out.mkdir(parents=True,exist_ok=True);(args.out/'summary.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
if __name__=='__main__':main()
