#!/usr/bin/env python3
"""Frozen centered-polar pilot; existing numerical labels only, no capacity producer."""
import argparse,csv,gzip,hashlib,json,math,time
from pathlib import Path
import numpy as np
from scipy.spatial import ConvexHull
from scipy.optimize import minimize
from scipy.stats import spearmanr
J=np.block([[np.zeros((2,2)),np.eye(2)],[-np.eye(2),np.zeros((2,2))]])
SOURCE={'random.jsonl':'a21ac62ba5c9496ef631d3cce74e8b663764516b76e9d4725f1e517d8dd55f9f','random-product.jsonl':'66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736'}
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def num(v):return float(v) if '/' not in str(v) else float(str(v).split('/')[0])/float(str(v).split('/')[1])
def mat(v):return np.array([[num(z) for z in a] for a in v])
def centroid(v):
 h=ConvexHull(v);p=v[h.simplices];ws=np.abs(np.linalg.det(p))/24
 return np.sum(ws[:,None]*np.sum(p,axis=1)/5,axis=0)/sum(ws),sum(ws)
def feature(a,v,V):
 g,vcheck=centroid(v);b=a/(1-a@g)[:,None];h=ConvexHull(b);alpha=np.max(np.abs(b@J@b.T));return dict(alpha=float(alpha),alpha_root_volume=float(alpha*math.sqrt(V)),mahler=float(V*h.volume),centroid=g.tolist(),volume_check=float(vcheck),polar_volume=float(h.volume))
def optimized(a,g):
 i,j=np.triu_indices(len(a),1);w=np.abs((a@J@a.T)[i,j]);keep=w>1e-14*np.max(w);i=i[keep];j=j[keep];w=w[keep]
 def margins(z):
  ds=1-a@z[:4]
  return np.r_[ds-1e-10,np.log(np.maximum(ds[i],1e-300))+np.log(np.maximum(ds[j],1e-300))-np.log(w)-z[4]]
 ds=1-a@g;alpha=np.max(w/(ds[i]*ds[j]));z=np.r_[g,-math.log(alpha)]
 res=minimize(lambda z:-z[4],z,method='SLSQP',constraints=[{'type':'ineq','fun':margins}],options={'ftol':1e-10,'maxiter':200})
 d=1-a@res.x[:4];new=np.max(w/(d[i]*d[j]));return dict(success=bool(res.success),iterations=int(res.nit),alpha=float(new),centroid_alpha=float(alpha),gain=float(alpha/new),center=res.x[:4].tolist(),min_margin=float(np.min(margins(res.x))))
def metrics(y,p):
 e=np.exp(p-y)-1;den=np.sum((y-y.mean())**2)
 return dict(n=len(y),log_R2=float(1-np.sum((p-y)**2)/den) if den else None,log_RMSE=float(np.sqrt(np.mean((p-y)**2))),median_abs_relative_error=float(np.median(abs(e))),p90_abs_relative_error=float(np.quantile(abs(e),.9)),max_abs_relative_error=float(max(abs(e))),mean_relative_error=float(np.mean(e)),spearman=float(spearmanr(y,p).statistic) if np.ptp(y)>0 and np.ptp(p)>0 else None)
def designs(x,m):
 o=np.ones(len(x));return {'alpha_linear':np.c_[o,x],'alpha_quadratic':np.c_[o,x,x*x],'mahler_linear':np.c_[o,m],'mahler_quadratic':np.c_[o,m,m*m],'alpha_mahler_linear':np.c_[o,x,m],'alpha_mahler_quadratic':np.c_[o,x,m,x*x,x*m,m*m]}
def planar_vertices(a):
 vv=[]
 for i in range(len(a)):
  for j in range(i):
   aa=a[[i,j]]
   if abs(np.linalg.det(aa))<1e-12:continue
   v=np.linalg.solve(aa,np.ones(2))
   if max(a@v)<1+1e-9:vv.append(v)
 return np.array(vv)
def challenge_features(p):
 data=json.loads(p.read_text());out=[]
 for r in data['rows']:
  A=mat(r['A']);B=mat(r['B']);va=planar_vertices(A);vb=planar_vertices(B);V=ConvexHull(va).volume*ConvexHull(vb).volume
  v=np.array([np.r_[aa,bb] for aa in va for bb in vb]);a=np.block([[A,np.zeros_like(A)],[np.zeros_like(B),B]])
  out.append(dict(name=r['name'],family=r['family'],epsilon=r['epsilon'],**feature(a,v,V)))
 return out

def main():
 ap=argparse.ArgumentParser();ap.add_argument('--source',type=Path,required=True);ap.add_argument('--repo',type=Path,required=True);ap.add_argument('--out',type=Path,required=True);ap.add_argument('--challenge',type=Path,required=True);ap.add_argument('--limit',type=int,default=0);z=ap.parse_args();z.out.mkdir(parents=True,exist_ok=True);t=time.perf_counter();rr=[];diag=[]
 cp=z.repo/'topics/empirical-studies/sys-datascience/methods/canonical-vertex-covariance/artifacts/current/per_polytope.jsonl';tp=z.repo/'topics/empirical-studies/retrospective-revalidation/geometry-full/polytope-table.jsonl.gz';assert sha(cp)=='88cba83c48996309d6f4f050adb44413d449acb873d3222c515040087a7c8301';assert sha(tp)=='466068ace1248c0d2ab3bb55bfa8269bf8597ad475ede6bbd3739d87500ddd45';cov={r['source_name']:r for r in map(json.loads,cp.open())}
 with gzip.open(z.repo/'topics/empirical-studies/retrospective-revalidation/geometry-full/polytope-table.jsonl.gz','rt') as f:table={r['poly_id']:r for r in map(json.loads,f)}
 for fn,expected in SOURCE.items():
  p=z.source/fn;assert sha(p)==expected
  for line in p.open():
   r=json.loads(line)
   if z.limit and len(rr)>=z.limit:break
   a=mat(r['dual_vertices_rational']);v=mat(r['vertices_rational']);ft=feature(a,v,r['volume']);assert abs(ft['volume_check']/r['volume']-1)<1e-7
   bucket=f"generic:F{r['facet_count']}" if fn=='random.jsonl' else f"product:{r['k']}x{r['m']}"
   ridge=table[cov[r['name']]['poly_id']]['ridge_symp_area_sum_over_volume_sqrt'] if r['name'] in cov else None
   if r['name'] in cov:assert r['sys']==cov[r['name']]['sys']==table[cov[r['name']]['poly_id']]['sys']
   rr.append(dict(name=r['name'],bucket=bucket,sys=r['sys'],capacity=r['capacity'],volume=r['volume'],ridge=ridge,**ft))
 with gzip.open(z.out/'features.jsonl.gz','wt') as f:
  for r in rr:f.write(json.dumps(r)+'\n')
 b=np.array([r['bucket'] for r in rr]);names=sorted(set(b));y=np.log([r['sys'] for r in rr]);x=np.log([r['alpha_root_volume'] for r in rr]);m=np.log([r['mahler'] for r in rr]);tail=np.zeros(len(rr),bool)
 for g in names:
  ii=np.flatnonzero(b==g);tail[ii[np.argsort(y[ii],kind='stable')[-math.ceil(.1*len(ii)):]]]=True
 test=np.array([int(hashlib.sha256(r['name'].encode()).hexdigest()[:8],16)%5==0 for r in rr]);scores={};coefs={};allpred={};perbucket=[]
 for name,X in designs(x,m).items():
  pred=np.zeros(len(y))
  for g in names:
   te=b==g;pred[te]=X[te]@np.linalg.lstsq(X[~te],y[~te],rcond=None)[0]
   perbucket.append(dict(model=name,bucket=g,all=metrics(y[te],pred[te]),upper_decile=metrics(y[te&tail],pred[te&tail])))
  c=np.linalg.lstsq(X,y,rcond=None)[0];coefs[name]=c.tolist();allpred[name]=pred
  ph=X[test]@np.linalg.lstsq(X[~test],y[~test],rcond=None)[0]
  scores[name]={'LOBO':metrics(y,pred),'LOBO_upper_decile':metrics(y[tail],pred[tail]),'hash_holdout':metrics(y[test],ph),'hash_holdout_upper_decile':metrics(y[test&tail],ph[tail[test]])}
  for pre in ['generic:','product:']:
   ix=np.char.startswith(b,pre);scores[name]['LOBO_'+pre]=metrics(y[ix],pred[ix]);scores[name]['LOBO_tail_'+pre]=metrics(y[ix&tail],pred[ix&tail])
   for train,label in [(ix,pre+'_to_other')]:scores[name][label]=metrics(y[~train],X[~train]@np.linalg.lstsq(X[train],y[train],rcond=None)[0])
  recalls=[]
  for g in names:
   ii=np.flatnonzero(b==g);n=math.ceil(.1*len(ii));chosen=ii[np.argsort(pred[ii])[-n:]];recalls.append(float(np.sum(tail[chosen])/n))
  scores[name]['mean_bucket_upper_decile_recall']=float(np.mean(recalls))
 cf=challenge_features(z.challenge);cx=np.log([r['alpha_root_volume'] for r in cf]);cm=np.log([r['mahler'] for r in cf]);cd=designs(cx,cm)
 for idx,r in enumerate(cf):r['predictions']={k:float(np.exp(cd[k][idx]@np.array(c))) for k,c in coefs.items()}
 (z.out/'blind-predictions.json').write_text(json.dumps(dict(challenge_sha256=sha(z.challenge),formula_plan_sha256=sha(Path(__file__).parent/'PLAN.md'),source_sha256=SOURCE,coefficients=coefs,rows=cf),indent=2)+'\n')
 # Fixed calibration constant frozen by regular HKO itself; diagnostic only.
 summaries=dict(plan_sha256=sha(Path(__file__).parent/'PLAN.md'),source_sha256=SOURCE,rows=len(rr),scores=scores,coefficients=coefs,by_bucket=perbucket,volume_check_max_relative=max(abs(r['volume_check']/r['volume']-1) for r in rr),elapsed_seconds=time.perf_counter()-t)
 (z.out/'results.json').write_text(json.dumps(summaries,indent=2)+'\n')
 with gzip.open(z.out/'heldout-predictions.csv.gz','wt') as f:
  w=csv.writer(f);w.writerow(['name','bucket','sys','upper_decile']+list(allpred))
  for i,r in enumerate(rr):w.writerow([r['name'],r['bucket'],r['sys'],int(tail[i])]+[float(np.exp(p[i])) for p in allpred.values()])
 print(json.dumps({k:{j:v for j,v in s.items() if j in ['LOBO','LOBO_upper_decile','mean_bucket_upper_decile_recall']} for k,s in scores.items()},indent=2));print('SECONDS',summaries['elapsed_seconds']);print('BLIND_SHA',sha(z.out/'blind-predictions.json'))
if __name__=='__main__':main()
