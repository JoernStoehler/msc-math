#!/usr/bin/env python3
"""Exhaustive exact sparse-product HK scorer; standard library only.

Imports the established product sparse closure-vertex reduction, not the
two-shadow theorem: every cyclic order of every active support is enumerated.
"""
from fractions import Fraction as F
from itertools import combinations, permutations
from pathlib import Path
import argparse,hashlib,json,time

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def dot(a,b):return sum(x*y for x,y in zip(a,b))
def det(a,b):return a[0]*b[1]-a[1]*b[0]
def serial(v):
 if isinstance(v,F):return str(v)
 if isinstance(v,dict):return {k:serial(x) for k,x in v.items()}
 if isinstance(v,(tuple,list)):return [serial(x) for x in v]
 return v
def solve(M,b):
 rows=[[F(v) for v in r]+[F(x)] for r,x in zip(M,b)];n=len(rows)
 for j in range(n):
  p=next((i for i in range(j,n) if rows[i][j]),None)
  if p is None:return None
  rows[p],rows[j]=rows[j],rows[p];v=rows[j][j];rows[j]=[x/v for x in rows[j]]
  for i in range(n):
   if i!=j:
    v=rows[i][j];rows[i]=[x-v*y for x,y in zip(rows[i],rows[j])]
 return [r[-1] for r in rows]
def closures(a):
 out=[]
 for ids in combinations(range(len(a)),2):
  i,j=ids
  if det(a[i],a[j]) or dot(a[i],a[j])>=0:continue
  k=next(k for k in range(2) if a[i][k]);w=-a[j][k]/(a[i][k]-a[j][k]);v=[F(0)]*len(a);v[i]=w;v[j]=1-w
  if v not in out:out.append(v)
 for ids in combinations(range(len(a)),3):
  w=solve([[F(1)]*3,[a[i][0] for i in ids],[a[i][1] for i in ids]],[1,0,0])
  if w is None or min(w)<0:continue
  v=[F(0)]*len(a)
  for i,x in zip(ids,w):v[i]=x
  if v not in out:out.append(v)
 for v in out:assert min(v)>=0 and sum(v)==1 and all(sum(x*u[k] for x,u in zip(v,a))==0 for k in range(2))
 return out
def pair_optimum(a,b,alpha,gamma):
 ia=[i for i,x in enumerate(alpha) if x];ib=[i for i,x in enumerate(gamma) if x];active=ia+[len(a)+j for j in ib]
 pair=[(i,len(a)+j,alpha[i]*gamma[j]*dot(a[i],b[j])/4) for i in ia for j in ib];best=None;word=None;count=0;ties=0
 for tail in permutations(active[1:]):
  w=(active[0],)+tail;pos={v:i for i,v in enumerate(w)};q=sum(c if pos[i]<pos[j] else -c for i,j,c in pair);count+=1
  if best is None or q>best:best=q;word=w;ties=1
  elif q==best:ties+=1
 return dict(A_support=ia,B_support=ib,Q=best,word=word,cyclic_orders=count,maximizing_order_count=ties)
def main():
 ap=argparse.ArgumentParser();ap.add_argument('--directory',type=Path,required=True);o=ap.parse_args();d=o.directory;t0=time.perf_counter();frozen=json.loads((d/'frozen-manifest.json').read_text());assert sha(d/'sample.json')==frozen['sample_sha256'];sample=json.loads((d/'sample.json').read_text());area=F(sample['area']);rows=[]
 for s in sample['rows']:
  a=[[F(x) for x in r] for r in s['A']];b=[[F(x) for x in r] for r in s['B']];ca=closures(a);cb=closures(b);pairs=[pair_optimum(a,b,u,v) for u in ca for v in cb];q=max(p['Q'] for p in pairs);cap=1/(2*q);ratio=cap*cap/(2*area*area);win=[p for p in pairs if p['Q']==q]
  rows.append(dict(name=s['name'],closure_count_A=len(ca),closure_count_B=len(cb),Q=q,capacity=cap,ratio=ratio,volume=area*area,pairs=pairs,winners=win,above_one=ratio>1,capacity_float=float(cap),ratio_float=float(ratio)))
 lo=min(r['ratio'] for r in rows);hi=max(r['ratio'] for r in rows);clo=min(r['capacity'] for r in rows);chi=max(r['capacity'] for r in rows)
 out=dict(status='PASS' if lo>1 and hi/lo>F(102,100) else 'FAIL_FROZEN_TARGET',source_sample_sha256=sha(d/'sample.json'),score_sha256=sha(Path(__file__)),rows=rows,relative_ratio_separation=hi/lo-1,relative_capacity_separation=chi/clo-1,common_prediction_minimum_worst_relative_ratio_error=(hi-lo)/(hi+lo),relative_capacity_separation_float=float(chi/clo-1),relative_ratio_separation_float=float(hi/lo-1),common_prediction_minimum_worst_relative_ratio_error_float=float((hi-lo)/(hi+lo)),cyclic_orders_evaluated=sum(p['cyclic_orders'] for r in rows for p in r['pairs']),elapsed_seconds=time.perf_counter()-t0)
 (d/'results.json').write_text(json.dumps(serial(out),indent=2)+'\n');print(json.dumps({k:serial(v) for k,v in out.items() if k!='rows'},indent=2));print(json.dumps([{k:serial(r[k]) for k in ['name','ratio_float','capacity_float','above_one','closure_count_A','closure_count_B']} for r in rows],indent=2))
if __name__=='__main__':main()
