#!/usr/bin/env python3
"""Exact replay; standard library only, no optimization/prediction producer imports."""
import argparse,hashlib,importlib.util,json,time
from fractions import Fraction as F
from pathlib import Path
from itertools import combinations

def module(name,p):
 s=importlib.util.spec_from_file_location(name,p);m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def serial(v):
 if isinstance(v,F):return str(v)
 if isinstance(v,dict):return {k:serial(x) for k,x in v.items()}
 if isinstance(v,(tuple,list)):return [serial(x) for x in v]
 return v

def area(a,m):
 vv=[]
 for i,j in combinations(range(len(a)),2):
  v=m.solve([a[i],a[j]],[1,1])
  if v is not None and all(m.dot(u,v)<=1 for u in a) and v not in vv:vv.append(v)
 assert len(vv)==len(a)==6
 for u in a:assert sum(m.dot(u,v)==1 for v in vv)==2
 cv=m.vertices(a);avg=[sum(v[i] for v in cv)/len(cv) for i in range(len(a))];m.feasible(a,avg);assert min(avg)>0
 center=[sum(v[k] for v in vv)/len(vv) for k in range(2)];order=m.angular([[x-y for x,y in zip(v,center)] for v in vv]);vv=[vv[i] for i in order];return abs(sum(m.det(vv[i],vv[(i+1)%len(vv)]) for i in range(len(vv))))/2

def main():
 p=argparse.ArgumentParser();p.add_argument('--repo',type=Path,required=True);p.add_argument('--directory',type=Path,required=True);o=p.parse_args();r=o.repo;d=o.directory;t=time.process_time();m=module('independent_geometry',r/'patterns/upper-tail/near-hko-transfer/verify.py');f=json.loads((d/'frozen-manifest.json').read_text())
 for k,v in f['sources'].items():assert sha(r/k)==v
 for k,v in f['freeze'].items():assert sha(d/k)==v
 sample=json.loads((d/'sample.json').read_text());pred=json.loads((d/'predictions.json').read_text());out=json.loads((d/'results.json').read_text());assert out['predictions_sha256']==sha(d/'predictions.json');assert len(pred['rows'])==len(sample['rows'])==12;checks=[];certificates=seeds_checked=oracle_values=0
 for index,s in enumerate(pred['rows']):
  assert all(s[k]==sample['rows'][index][k] for k in ['name','A','B']);assert s['status']=='OK';a=[list(map(F,v)) for v in s['A']];b=[list(map(F,v)) for v in s['B']];n=len(a);nb=len(b);ca=m.vertices(a);cb=m.vertices(b);volume=area(a,m)*area(b,m);seeds=s['extraction']['seeds'];at=0
  for v in s['variants']:
   W=m.lp(a,b,v);certificates+=1
   for kind in ['row','column']:
    order=['A','B'] if kind=='row' else ['B','A']
    for k in range(n if kind=='row' else nb):
     vals=W[k*nb:k*nb+nb] if kind=='row' else [W[i*nb+k] for i in range(n)];mass=sum(vals)
     if not mass:continue
     seed=seeds[at];at+=1;assert (seed['shadow_sign'],seed['kind'],seed['index'])==(v['shadow_sign'],kind,k);assert F(seed['condition_mass'])==mass and seed['word']==v['word'];fixed=[x/mass for x in vals];alpha=None if kind=='row' else fixed;gamma=fixed if kind=='row' else None;expected=[]
     def best_response(which):
      cv=ca if which=='A' else cb;values=[m.q(a,b,v['word'],u,gamma) if which=='A' else m.q(a,b,v['word'],alpha,u) for u in cv];return cv[values.index(max(values))],max(values)
     for which in order:
      best,value=best_response(which)
      if which=='A':alpha=best
      else:gamma=best
      expected.append(dict(stage='initialization',factor=which,alpha=alpha[:],gamma=gamma[:],Q=value))
     visited={(tuple(alpha),tuple(gamma))};sweeps=0
     while True:
      sweeps+=1;changed=False
      for which in order:
       best,value=best_response(which);current=m.q(a,b,v['word'],alpha,gamma)
       if value>current:
        if which=='A':alpha=best
        else:gamma=best
        key=(tuple(alpha),tuple(gamma));assert key not in visited;visited.add(key);changed=True;expected.append(dict(stage='strict_update',factor=which,alpha=alpha[:],gamma=gamma[:],Q=value))
      if not changed:break
     assert m.canon(seed['states'])==m.canon(expected);assert m.canon(seed['terminal_alpha'])==alpha and m.canon(seed['terminal_gamma'])==gamma;value=m.q(a,b,v['word'],alpha,gamma);assert F(seed['Q'])==value and F(seed['capacity'])==1/(2*value) and seed['sweeps']==sweeps
   assert at<=len(seeds)
  assert at==len(seeds)==s['extraction']['candidate_count'];seeds_checked+=at;best=max(seeds,key=lambda v:F(v['Q']));assert s['extraction']['best']==best and s['extraction']['capacity']==best['capacity']
  if index>=len(out['rows']):continue
  row=out['rows'][index];assert row['name']==s['name'];table=[]
  for v in s['variants']:
   for ia,alpha in enumerate(ca):
    for ib,gamma in enumerate(cb):table.append(dict(shadow_sign=v['shadow_sign'],A_vertex=ia,B_vertex=ib,alpha=alpha,gamma=gamma,word=v['word'],Q=m.q(a,b,v['word'],alpha,gamma)))
  assert m.canon(table)==m.canon(row['oracle_candidates']);qt=max(v['Q'] for v in table);qp=F(best['Q']);assert qp<=qt and F(row['Q_predicted'])==qp and F(row['Q_true'])==qt;assert F(row['capacity_true'])==1/(2*qt) and F(row['volume'])==volume and F(row['ratio'])==1/(8*volume*qt*qt);assert row['exact_recovery']==(qp==qt) and F(row['relative_capacity_error'])==qt/qp-1;assert m.canon(row['winners'])==m.canon([v for v in table if v['Q']==qt]);oracle_values+=len(table);checks.append(dict(name=s['name'],exact_recovery=qp==qt,above_one=F(row['ratio'])>1,closure_vertex_counts=[len(ca),len(cb)],ratio=float(F(row['ratio']))))
  if qp<qt:assert index==len(out['rows'])-1
 assert len(checks)==out['scored_count'];assert out['unscored_names']==[s['name'] for s in pred['rows'][len(checks):]]
 result=dict(status='PASS',exact_LP_certificates=certificates,seeds=seeds_checked,oracle_values=oracle_values,checks=checks,cpu_seconds=time.process_time()-t,hashes={p:sha(d/p) for p in ['PLAN.md','freeze.py','analyze.py','sample.json','geometry.json','frozen-manifest.json','predictions.json','results.json','verify.py']});(d/'verification.json').write_text(json.dumps(serial(result),indent=2)+'\n');print('PASS',len(checks),'CPU',result['cpu_seconds'])
if __name__=='__main__':main()
