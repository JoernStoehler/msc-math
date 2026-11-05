#!/usr/bin/env python3
"""Frozen sparse symbolic regression; labels isolated from fresh predictions.
Requires installed numpy/scipy; no capacity evaluator. Writes only --out.
"""
from __future__ import annotations
import argparse,csv,gzip,hashlib,importlib.util,json,math,time
from pathlib import Path
import numpy as np
from scipy.spatial import ConvexHull
from scipy.stats import spearmanr

J=np.block([[np.zeros((2,2)),np.eye(2)],[-np.eye(2),np.zeros((2,2))]])
DEFAULT_COV='topics/empirical-studies/sys-datascience/methods/canonical-vertex-covariance/artifacts/current/per_polytope.jsonl'
DEFAULT_WIDTH='patterns/product-shape/features.csv.gz'

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def readrows(p):
 op=gzip.open if p.suffix=='.gz' else open
 with op(p,'rt') as f:return [json.loads(l) for l in f]
def rational(v):
 s=str(v)
 if '/' in s:
  a,b=s.split('/');return float(a)/float(b)
 return float(s)
def mat(v):return np.array([[rational(x) for x in row] for row in v])
def primal(a):
 vv=[]
 for i in range(len(a)):
  for j in range(i):
   aa=a[[i,j]]
   if abs(np.linalg.det(aa))<1e-12:continue
   v=np.linalg.solve(aa,np.ones(2))
   if max(a@v)<1+1e-9:vv.append(v)
 vv=np.array(vv);return vv[ConvexHull(vv).vertices]
def difference(v):
 d=(v[:,None,:]-v[None,:,:]).reshape(-1,2);return d[ConvexHull(d).vertices]
def polar(v):
 eq=ConvexHull(v).equations;return eq[:,:2]/(-eq[:,2,None])
def c2_value(dp,dq):
 eq=ConvexHull(dp).equations;return float(np.min(-eq[:,2]/np.max(eq[:,:2]@polar(dq).T,axis=1)))
def centroid(v):
 h=ConvexHull(v);p=v[h.simplices];ws=np.abs(np.linalg.det(p))/24
 return np.sum(ws[:,None]*np.sum(p,axis=1)/5,axis=0)/sum(ws),sum(ws)
def feature(A,B):
 p=primal(A);q=primal(B);V=ConvexHull(p).volume*ConvexHull(q).volume
 v=np.array([np.r_[aa,bb] for aa in p for bb in q]);a=np.block([[A,np.zeros_like(A)],[np.zeros_like(B),B]])
 g,vc=centroid(v);assert abs(vc/V-1)<1e-7;b=a/(1-a@g)[:,None]
 alpha=np.max(np.abs(b@J@b.T));M=V*ConvexHull(b).volume
 ep=np.roll(p,-1,axis=0)-p;eq=np.roll(q,-1,axis=0)-q
 R=np.sum(np.abs(ep@eq.T))/np.sqrt(V)
 cp=(p-p.mean(axis=0)).T@(p-p.mean(axis=0))/len(p);cq=(q-q.mean(axis=0)).T@(q-q.mean(axis=0))/len(q)
 lam=np.sort(np.linalg.eigvals(cp@cq).real);assert min(lam)>0;rho=np.sqrt(lam[1]/lam[0])
 dp=difference(p);dq=difference(q);W=c2_value(dp,dq)**2/(2*V)
 D=ConvexHull(dp).volume*ConvexHull(dq).volume/(16*V)
 return dict(R=float(R),rho=float(rho),A=float(alpha*np.sqrt(V)),M=float(M),W=float(W),D=float(D))
def hcode(s,mod):return int(hashlib.sha256(s.encode()).hexdigest()[:8],16)%mod

def grammar(logprim,names):
 ee=[];meta=[]
 def add(name,x,cost,formula):ee.append(x);meta.append(dict(name=name,cost=cost,formula=formula))
 for i,name in enumerate(names):add(name,logprim[:,i],0,name)
 for i in range(len(names)):
  for j in range(i+1,len(names)):
   ni,nj=names[i],names[j];x,y=logprim[:,i],logprim[:,j]
   for q in [-2.,-1.,-.5,.5,1.,2.]:
    add(f'product({ni},{nj}^{q:g})',x+q*y,2,f'{ni}*{nj}^({q:g})')
    add(f'product({nj},{ni}^{q:g})',y+q*x,2,f'{nj}*{ni}^({q:g})')
   for q in [.5,1.,2.]:
    add(f'powermean({ni},{nj},{q:g})',(np.logaddexp(q*x,q*y)-np.log(2))/q,4,f'(({ni}^({q:g})+{nj}^({q:g}))/2)^(1/({q:g}))')
    add(f'powermean({ni},{nj},{-q:g})',-(np.logaddexp(-q*x,-q*y)-np.log(2))/q,4,f'(({ni}^({-q:g})+{nj}^({-q:g}))/2)^(1/({-q:g}))')
   add(f'min({ni},{nj})',np.minimum(x,y),1,f'min({ni},{nj})')
   add(f'max({ni},{nj})',np.maximum(x,y),1,f'max({ni},{nj})')
   add(f'ratio({ni},{nj})',x-y,1,f'{ni}/{nj}')
 return np.array(ee).T,meta

def tailmask(y,b,mask):
 tail=np.zeros(len(y),bool)
 for g in sorted(set(b[mask])):
  ii=np.flatnonzero(mask&(b==g));n=math.ceil(.1*len(ii));tail[ii[np.argsort(y[ii],kind='stable')[-n:]]]=True
 return tail

def fitall(E,y,mask):
 x=E[mask];z=y[mask];xm=x.mean(axis=0);ym=z.mean();xx=x-xm;den=np.sum(xx*xx,axis=0);num=(z-ym)@xx
 sl=np.divide(num,den,out=np.zeros_like(num),where=den>1e-20);return ym-sl*xm,sl

def select(E,y,b,mask,folds,meta):
 score=np.zeros(E.shape[1]);count=0
 for fold in sorted(set(folds[mask])):
  val=mask&(folds==fold);train=mask&~val
  assert val.any() and train.any()
  a,sl=fitall(E,y,train);tail=tailmask(y,b,val)
  ve=a+E[val]*sl-y[val,None];te=a+E[tail]*sl-y[tail,None]
  score+=.5*np.mean(ve*ve,axis=0)+.5*np.mean(te*te,axis=0);count+=1
 score/=count;best=float(np.min(score));ties=np.flatnonzero(score<=best+1e-9);idx=min(ties,key=lambda i:(meta[i]['cost'],meta[i]['name']))
 return int(idx),score

def metrics(y,p,b):
 e=np.expm1(p-y);scores=[];rp=[];ry=[]
 for g in sorted(set(b)):
  ii=np.flatnonzero(b==g);n=math.ceil(.1*len(ii));tt=ii[np.argsort(y[ii],kind='stable')[-n:]];sel=ii[np.argsort(p[ii],kind='stable')[-n:]]
  rank=float(spearmanr(y[tt],p[tt]).statistic) if len(tt)>2 and np.std(p[tt])>1e-14 else None
  scores.append(dict(bucket=str(g),n=len(ii),tail_n=n,tail_spearman=rank,top_decile_recall=float(len(set(tt)&set(sel))/n),tail_median_abs_relative_error=float(np.median(np.abs(np.expm1(p[tt]-y[tt])))),tail_p90_abs_relative_error=float(np.quantile(np.abs(np.expm1(p[tt]-y[tt])),.9))))
 tail=tailmask(y,b,np.ones(len(y),bool))
 def basic(yy,pp):
  err=np.expm1(pp-yy);den=np.sum((yy-yy.mean())**2)
  return dict(n=len(yy),log_R2=float(1-np.sum((pp-yy)**2)/den) if den>0 else None,log_RMSE=float(np.sqrt(np.mean((pp-yy)**2))),median_abs_relative_error=float(np.median(abs(err))),p90_abs_relative_error=float(np.quantile(abs(err),.9)),max_abs_relative_error=float(max(abs(err))),median_prediction_over_target=float(np.median(np.exp(pp-yy))))
 valid=[r['tail_spearman'] for r in scores if r['tail_spearman'] is not None and np.isfinite(r['tail_spearman'])]
 return dict(all=basic(y,p),upper_decile=basic(y[tail],p[tail]),mean_bucket_tail_spearman=float(np.mean(valid)) if valid else None,mean_bucket_top_decile_recall=float(np.mean([r['top_decile_recall'] for r in scores])),by_bucket=scores)

def baselines(logprim,names):
 n=len(logprim);r=logprim[:,names.index('R')];h=logprim[:,names.index('rho')]
 return dict(constant=np.ones((n,1)),all_loglinear=np.c_[np.ones(n),logprim],old_R_rho_quadratic=np.c_[np.ones(n),r,h,r*r,r*h,h*h])
def scoreset(rows,base,primnames,challenge,basefeature):
 y=np.log([r['sys'] for r in rows]);b=np.array([r['bucket'] for r in rows]);lp=np.log([[r[k]/base[k] for k in primnames] for r in rows]);E,meta=grammar(lp,primnames)
 names=[r['name'] for r in rows];bucketfold=np.array([hcode(z,3) for z in b]);namefold=np.array([hcode(z,3) for z in names]);outerhash=np.array([hcode(z,5)==0 for z in names]);allmask=np.ones(len(y),bool)
 pred=np.empty(len(y));selections=[];coefs={};baselineX=baselines(lp,primnames);baselinepred={k:np.empty(len(y)) for k in baselineX}
 for bucket in sorted(set(b)):
  test=b==bucket;train=~test;idx,inner=select(E,y,b,train,bucketfold,meta);a,sl=fitall(E[:,[idx]],y,train);pred[test]=a[0]+sl[0]*E[test,idx]
  selections.append(dict(bucket=bucket,selected=meta[idx],a=float(a[0]),b=float(sl[0]),inner_score=float(inner[idx])))
  for k,X in baselineX.items():baselinepred[k][test]=X[test]@np.linalg.lstsq(X[train],y[train],rcond=None)[0]
 idx,score=select(E,y,b,~outerhash,namefold,meta);a,sl=fitall(E[:,[idx]],y,~outerhash);hp=a[0]+sl[0]*E[outerhash,idx]
 hashcoef=dict(selected=meta[idx],a=float(a[0]),b=float(sl[0]),inner_score=float(score[idx]))
 finalidx,finalscore=select(E,y,b,allmask,bucketfold,meta);a,sl=fitall(E[:,[finalidx]],y,allmask)
 final=dict(selected=meta[finalidx],a=float(a[0]),b=float(sl[0]),scale=float(np.exp(a[0])),inner_score=float(finalscore[finalidx]),normalization=base)
 cp=np.log([[r[k]/base[k] for k in primnames] for r in challenge]);ce,cm=grammar(cp,primnames);assert cm==meta
 cpred=np.exp(a[0]+sl[0]*ce[:,finalidx]);final['HKO_prediction']=float(np.exp(a[0]))
 order=sorted(range(len(meta)),key=lambda i:(finalscore[i],meta[i]['cost'],meta[i]['name']))
 result=dict(rows=len(y),primitives=primnames,grammar_size=len(meta),LOBO=metrics(y,pred,b),name_hash_holdout=metrics(y[outerhash],hp,b[outerhash]),outer_bucket_selections=selections,name_hash_selected=hashcoef,full_corpus_selected=final,inner_leaderboard=[dict(expression=meta[i],score=float(finalscore[i])) for i in order[:20]],diagnostic_baselines={})
 for k,X in baselineX.items():
  ph=X[outerhash]@np.linalg.lstsq(X[~outerhash],y[~outerhash],rcond=None)[0]
  result['diagnostic_baselines'][k]=dict(LOBO=metrics(y,baselinepred[k],b),name_hash_holdout=metrics(y[outerhash],ph,b[outerhash]))
 return result,pred,cpred

def main():
 ap=argparse.ArgumentParser();ap.add_argument('--repo',type=Path,required=True);ap.add_argument('--polar',type=Path,required=True);ap.add_argument('--challenge',type=Path,required=True);ap.add_argument('--plan',type=Path,required=True);ap.add_argument('--source',type=Path,required=True);ap.add_argument('--out',type=Path,required=True);z=ap.parse_args();z.out.mkdir(parents=True,exist_ok=True);t=time.perf_counter()
 covp=z.repo/DEFAULT_COV;widthp=z.repo/DEFAULT_WIDTH;cov={r['source_name']:r for r in readrows(covp)}
 with gzip.open(widthp,'rt') as f:width={r['name']:r for r in csv.DictReader(f)}
 rr=[]
 for r in readrows(z.polar):
  if r['name'] not in cov:continue
  cr=cov[r['name']];assert cr['sys']==r['sys'];w=width.get(r['name'])
  rr.append(dict(name=r['name'],bucket=r['bucket'],sys=r['sys'],R=r['ridge'],rho=cr['rho'],A=r['alpha_root_volume'],M=r['mahler'],**(dict(W=float(w['sys2']),D=float(w['asym_p'])*float(w['asym_q'])) if w else {})))
 data=json.loads(z.challenge.read_text());base=feature(mat(data['base']['A']),mat(data['base']['B']));challenge=[dict(name=r['name'],family=r['family'],epsilon=r['epsilon'],**feature(mat(r['A']),mat(r['B']))) for r in data['rows']]
 # Independently validate the product ridge and covariance reconstruction on 32 retained geometries.
 check=[]
 polarby={r['name']:r for r in rr}
 for i,line in enumerate(z.source.open()):
  if i%320:continue
  r=json.loads(line);a=mat(r['dual_vertices_rational']);ft=feature(a[:r['k'],:2],a[r['k']:,2:]);old=polarby[r['name']]
  errors={k:abs(ft[k]/old[k]-1) for k in ['R','rho','A','M','W','D']};check.append(dict(name=r['name'],relative_errors=errors));assert max(errors.values())<1e-8
 results={};preds={};allpred={}
 for lane,names,rows in [('all_bodies',['R','rho','A','M'],rr),('products',['R','rho','A','M','W','D'],[r for r in rr if r['bucket'].startswith('product:')])]:
  result,pred,cpred=scoreset(rows,base,names,challenge,base);results[lane]=result;preds[lane]=cpred;allpred[lane]=dict(zip([r['name'] for r in rows],map(float,np.exp(pred))))
 frozen=dict(plan_sha256=sha(z.plan),challenge_sha256=sha(z.challenge),challenge_rows=challenge,HKO_geometry=base,selected={k:v['full_corpus_selected'] for k,v in results.items()},predictions={r['name']:{k:float(p[i]) for k,p in preds.items()} for i,r in enumerate(challenge)},status='Frozen before any fresh challenge truth access by this lane')
 (z.out/'frozen-predictions.json').write_text(json.dumps(frozen,indent=2)+'\n')
 out=dict(plan_sha256=sha(z.plan),input_sha256={str(p):sha(p) for p in [z.polar,covp,widthp,z.challenge,z.source]},fresh_predictions_sha256=sha(z.out/'frozen-predictions.json'),results=results,geometry_reconstruction_checks=check,elapsed_seconds=time.perf_counter()-t)
 (z.out/'results.json').write_text(json.dumps(out,indent=2)+'\n')
 with gzip.open(z.out/'LOBO-predictions.jsonl.gz','wt') as f:
  for r in rr:f.write(json.dumps(dict(name=r['name'],bucket=r['bucket'],sys=r['sys'],**{k:v[r['name']] for k,v in allpred.items() if r['name'] in v}))+'\n')
 print(json.dumps({k:dict(grammar=v['grammar_size'],selected=v['full_corpus_selected'],LOBO={s:sc for s,sc in v['LOBO'].items() if s!='by_bucket'},name_hash={s:sc for s,sc in v['name_hash_holdout'].items() if s!='by_bucket'}) for k,v in results.items()},indent=2));print('FROZEN_PREDICTIONS_SHA256',out['fresh_predictions_sha256']);print('ELAPSED_SECONDS',out['elapsed_seconds'])
if __name__=='__main__':main()
