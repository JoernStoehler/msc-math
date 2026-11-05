#!/usr/bin/env python3
"""Root independent strict-response/local-support certificate for retained rational body."""
import importlib.util,json,itertools,hashlib,argparse
from fractions import Fraction as F
from pathlib import Path

def rank(rows):
 a=[list(r) for r in rows]; k=0
 for j in range(len(a[0])):
  p=next((i for i in range(k,len(a)) if a[i][j]),None)
  if p is None: continue
  a[k],a[p]=a[p],a[k]; t=a[k][j]; a[k]=[v/t for v in a[k]]
  for i in range(k+1,len(a)):
   t=a[i][j]; a[i]=[u-t*v for u,v in zip(a[i],a[k])]
  k+=1
 return k

def blocks(word,support):
 z=[i for i in word if i in support]; b=[]
 for i in z:
  if b and (next(iter(b[-1]))<5)==(i<5): b[-1].add(i)
  else:b.append({i})
 if len(b)>1 and (next(iter(b[0]))<5)==(next(iter(b[-1]))<5):b[0]|=b.pop()
 return min(tuple(tuple(sorted(x)) for x in b[i:]+b[:i]) for i in range(len(b)))

def run(thesis,patterns,out):
 source=patterns/'patterns/upper-tail/near-hko-transfer/results.json'
 spec=importlib.util.spec_from_file_location('independent',thesis/'thesis/review/2026-10-17/near-hko-transfer-integration/owner-verify.py'); g=importlib.util.module_from_spec(spec);spec.loader.exec_module(g)
 r=json.loads(source.read_text())['rows'][-1];assert r['name']=='near_hko_s4_e100'
 a,b=([list(map(F,v)) for v in r[k]] for k in ['A','B']);ca,cb=g.closures(a),g.closures(b); assert len(ca)==len(cb)==5
 volume=g.area(a)*g.area(b);truth=max(g.prefix_value(a,b,x,y) for x in ca for y in cb);ratio=1/(8*truth**2*volume);assert ratio>1
 actual=[]; certificates=[]; words=[]
 E=[[F(1)]*25]
 E += [[b[j][k] if ii==i else F(0) for ii in range(5) for j in range(5)] for i in range(5) for k in range(2)]
 E += [[a[i][k] if jj==j else F(0) for i in range(5) for jj in range(5)] for j in range(5) for k in range(2)]
 assert rank(E)==17
 for v in r['variants']:
  g.audit_lp(a,b,v);w=v['word'];words.append(w);W=list(map(F,v['W']));sl=list(map(F,v['dual_slack']));active=[i for i,x in enumerate(W) if x>0];zero=[i for i,x in enumerate(W) if not x]
  assert len(active)==rank([[row[i] for i in active] for row in E])==17
  assert min(sl[i] for i in zero)>0 and all(sl[i]==0 for i in active)
  for kind in ['row','column']:
   for i in range(5):
    z=[W[5*i+j] for j in range(5)] if kind=='row' else [W[5*j+i] for j in range(5)];mass=sum(z);assert mass>0;z=[t/mass for t in z]
    if kind=='row':
     x=max(ca,key=lambda x:g.direct(a,b,w,x,z));one=g.direct(a,b,w,x,z);y=max(cb,key=lambda y:g.direct(a,b,w,x,y))
    else:
     y=max(cb,key=lambda y:g.direct(a,b,w,z,y));one=g.direct(a,b,w,z,y);x=max(ca,key=lambda x:g.direct(a,b,w,x,y))
    actual.append((v['shadow_sign'],kind,i,x,y,one,g.direct(a,b,w,x,y)))
  pairvals=[(g.direct(a,b,w,x,y),x,y) for x in ca for y in cb];wins=[(x,y) for q,x,y in pairvals if q==truth];assert len(wins)==1
  gap=truth-max(q for q,x,y in pairvals if (x,y)!=wins[0]);assert gap>0
  certificates.append(dict(shadow_sign=v['shadow_sign'],LP_basis_rank=17,positive_basis_entries=17,strict_nonbasic_slacks=8,minimum_positive_weight=str(min(W[i] for i in active)),minimum_nonbasic_slack=str(min(sl[i] for i in zero)),global_other_pair_gap=str(gap)))
 old=max(x[5] for x in actual);completed=max(x[6] for x in actual);assert old<truth==completed
 plus=next(v for v in r['variants'] if v['shadow_sign']==1);W=list(map(F,plus['W']));z=W[:5];z=[t/sum(z) for t in z];w=plus['word']
 vals=[g.direct(a,b,w,x,z) for x in ca];k=max(range(5),key=vals.__getitem__);x=ca[k];m1=vals[k]-max(v for i,v in enumerate(vals) if i!=k);assert m1>0
 vals=[g.direct(a,b,w,x,y) for y in cb];j=max(range(5),key=vals.__getitem__);y=cb[j];m2=vals[j]-max(v for i,v in enumerate(vals) if i!=j);assert m2>0 and vals[j]==truth
 support={i for i,t in enumerate(x+y) if t};assert blocks(words[0],support)==blocks(words[1],support)
 assert W[0]==0 and F(plus['dual_slack'][0])>0
 edge=[v for v in cb if v[0]==0]; assert len(edge)==2 and y in edge
 bad=next(v for v in edge if v!=y);k=next(i for i in range(5) if y[i]!=bad[i]);lam=(z[k]-bad[k])/(y[k]-bad[k]);assert 0<lam<1
 assert z==[lam*u+(1-lam)*v for u,v in zip(y,bad)]
 qbad=g.direct(a,b,w,x,bad);assert truth-old==(1-lam)*(truth-qbad)>0
 geo=[]
 for normals in [a,b]:
  p=min(abs(g.cross(u,v)) for u,v in itertools.combinations(normals,2));t=min(abs(g.cross((v[0]-u[0],v[1]-u[1]),(z[0]-u[0],z[1]-u[1]))) for u,v,z in itertools.combinations(normals,3));assert p>0 and t>0;geo.append({'pair_min':str(p),'affine_triple_min':str(t)})
 angular=[]
 for sign in [1,-1]:
  ns=a+[(-sign*v[1],sign*v[0]) for v in b];m=min(abs(g.cross(u,v)) for u,v in itertools.combinations(ns,2));assert m>0;angular.append(str(m));assert min(abs(u[1]) for u in ns)>0
 result={'status':'PASS','name':r['name'],'ratio':str(ratio),'ratio_float':float(ratio),'old_strict_objective_loss':str(truth-old),'completed_Q':str(completed),'positive_row0_first_response_margin':str(m1),'positive_row0_completion_margin':str(m2),'winning_supports':[[i for i,t in enumerate(z) if t] for z in [x,y]],'cyclic_same_factor_block_normal_form':blocks(words[0],support),'stable_face_mixture':{'gamma0_zero':True,'nonbasic_W00_strict_slack':str(plus['dual_slack'][0]),'good_weight':str(lam),'bad_support':[i for i,t in enumerate(bad) if t]},'LP_certificates':certificates,'geometry_chamber':geo,'angular_cross_minima':angular,'counts':{'exact_LPs':2,'conditional_seeds':20,'closure_pairs_prefix_oracle':25,'pair_word_objectives':50},'scope':'Independent rational base certificate plus structural word equivalence; analytic open-neighborhood implication requires separate written proof. No entire-HKO-neighborhood guarantee, radius, new data or manuscript insertion.','input_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'checker_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
 out.write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({k:result[k] for k in ['status','name','ratio_float','winning_supports','counts']}))
if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('--thesis',required=True,type=Path);p.add_argument('--patterns',required=True,type=Path);p.add_argument('--out',required=True,type=Path);o=p.parse_args();run(o.thesis,o.patterns,o.out)
