import sys,json,gzip,hashlib,time
from pathlib import Path
import numpy as np
from scipy.optimize import minimize
sys.path.insert(0,str(Path(__file__).parent));from analyze import mat,J,feature,centroid,planar_vertices
from scipy.spatial import ConvexHull

def center_opt(a,v,g):
 b=a/(1-a@g)[:,None];S=np.linalg.cholesky(np.cov(v.T,bias=True));A=b@S;i,j=np.triu_indices(len(a),1);w=np.abs((b@J@b.T)[i,j]);keep=w>1e-14*max(w);i=i[keep];j=j[keep];w=w[keep];alpha=max(w);ln=np.log(w/alpha)
 def fun(z):
  ds=1-A@z[:4];safe=np.maximum(ds,1e-100)
  return np.r_[ds-1e-8,np.log(safe[i])+np.log(safe[j])-ln-z[4]]
 def jac(z):
  ds=np.maximum(1-A@z[:4],1e-100);jg=np.c_[-A,np.zeros(len(A))];jh=np.c_[-A[i]/ds[i,None]-A[j]/ds[j,None],-np.ones(len(i))];return np.r_[jg,jh]
 res=minimize(lambda z:-z[4],np.zeros(5),jac=lambda z:np.array([0,0,0,0,-1.]),method='SLSQP',constraints=[{'type':'ineq','fun':fun,'jac':jac}],options={'ftol':1e-11,'maxiter':200});ds=1-A@res.x[:4];new=max(w/(ds[i]*ds[j]));return dict(success=bool(res.success),iterations=int(res.nit),alpha=float(new),gain=float(alpha/new),center=(g+S@res.x[:4]).tolist(),min_margin=float(min(fun(res.x))),log_reduction=float(res.x[4]))

def main():
 p=Path(__file__).parent;rows=[json.loads(l) for l in gzip.open(p/'features.jsonl.gz','rt')];buckets=sorted(set(r['bucket'] for r in rows));selected={r['name']:r for b in buckets for r in sorted((r for r in rows if r['bucket']==b),key=lambda r:hashlib.sha256(r['name'].encode()).hexdigest())[:8]};rr=[];start=time.perf_counter();base=Path.home()/'.cache/msc-math/artifacts/polytope-datasets/f7bc6be841e9d30741d5bf7ec8d4f0c0c74ec22745d3325b792e64cec333ca96/files'
 for fn in ['random.jsonl','random-product.jsonl']:
  for line in (base/fn).open():
   r=json.loads(line)
   if r['name'] not in selected:continue
   old=selected[r['name']];res=center_opt(mat(r['dual_vertices_rational']),mat(r['vertices_rational']),np.array(old['centroid']));rr.append(dict(name=r['name'],bucket=old['bucket'],sys=r['sys'],volume=r['volume'],mahler=old['mahler'],old_alpha=old['alpha'],capacity=r['capacity'],old_alpha_capacity=old['alpha']*r['capacity'],new_alpha_capacity=res['alpha']*r['capacity'],**res))
 out=dict(status='Prespecified144-row centering diagnostic; no new regression or predictive selection',rows=rr,elapsed_seconds=time.perf_counter()-start);(p/'centering-stable.json').write_text(json.dumps(out,indent=2)+'\n');print(dict(n=len(rr),success=sum(r['success'] for r in rr),max_minmargin=max(abs(r['min_margin']) for r in rr),kappa_range=[min(r['new_alpha_capacity'] for r in rr),max(r['new_alpha_capacity'] for r in rr)],median_gain=np.median([r['gain'] for r in rr]),elapsed=out['elapsed_seconds']))
if __name__=='__main__':main()
