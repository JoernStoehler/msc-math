#!/usr/bin/env python3
import hashlib,json,importlib.util,sys,time,resource
from fractions import Fraction as F
from pathlib import Path
from itertools import combinations
ROOT=Path('/workspaces/msc-math-worktrees/patterns-2026-10-17');OUT=Path(__file__).parent

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def load(name,p):
 s=importlib.util.spec_from_file_location(name,p);m=importlib.util.module_from_spec(s);sys.modules[name]=m;s.loader.exec_module(m);return m

def geometry(a,m):
 vv=[]
 for i,j in combinations(range(len(a)),2):
  v=m.solve([a[i],a[j]],[1,1])
  if v is not None and all(m.dot(u,v)<=1 for u in a) and v not in vv:vv.append(v)
 assert len(vv)==len(a)==6
 facet_counts=[sum(m.dot(u,v)==1 for v in vv) for u in a];assert facet_counts==[2]*6
 ca=m.vertices(a);avg=[sum(v[i] for v in ca)/len(ca) for i in range(len(a))];m.feasible(a,avg);assert min(avg)>0
 center=[sum(v[k] for v in vv)/len(vv) for k in range(2)];order=m.angular([[x-y for x,y in zip(v,center)] for v in vv]);vv=[vv[i] for i in order];area=abs(sum(m.det(vv[i],vv[(i+1)%len(vv)]) for i in range(len(vv))))/2;assert area>0
 return dict(primal_vertices=vv,facet_vertex_counts=facet_counts,closure_vertex_count=len(ca),positive_full_support_closure=avg,area=area)

def serial(x):
 if isinstance(x,F):return str(x)
 if isinstance(x,dict):return {k:serial(v) for k,v in x.items()}
 if isinstance(x,(list,tuple)):return [serial(v) for v in x]
 return x

def main():
 t=time.process_time();m=load('facet_independent',ROOT/'patterns/upper-tail/near-hko-transfer/verify.py');h=json.loads((ROOT/'patterns/upper-tail/conditional-coupling/hko-exact.json').read_text());base={k:[[F(x['rational'])+F(x['sqrt5'])*F(2236068,1000000) for x in u] for u in h[k]] for k in ['A','B']};rows=[];checks=[]
 for seed in range(6):
  raw=hashlib.sha256(('msc-math/thesis/facet-transfer/v1:'+str(seed)).encode()).digest();coords=[v+F(1,100)*F(byte%9-4,4) for v,byte in zip([x for k in ['A','B'] for u in base[k] for x in u],raw)];fac={'A':[coords[i:i+2] for i in range(0,10,2)],'B':[coords[i:i+2] for i in range(10,20,2)]}
  for delta in [F(1,1000),F(1,100)]:
   row=dict(name=f'facet_transfer_s{seed}_d{delta.denominator}',seed=seed,delta=delta,direction_digest=raw.hex(),cut_pairs={});gg={}
   for key,pos in [('A',seed%5),('B',(2*seed+1)%5)]:
    a=fac[key];angular=m.angular(a);i,j=angular[pos],angular[(pos+1)%5];cut=[(x+y)/(2*(1-delta)) for x,y in zip(a[i],a[j])];v=m.solve([a[i],a[j]],[1,1]);assert v is not None and all(m.dot(u,v)<=1 for u in a);assert m.dot(cut,v)>1;row[key]=a+[cut];row['cut_pairs'][key]=[i,j];gg[key]=geometry(row[key],m)
   rows.append(row);checks.append(dict(name=row['name'],geometry=gg))
 sample=dict(protocol='fresh fixed6x6rationalvertexcuts; no labels',rows=rows);assert len(rows)==12
 (OUT/'sample.json').write_text(json.dumps(serial(sample),indent=2)+'\n');(OUT/'geometry.json').write_text(json.dumps(serial(dict(status='PASS',checks=checks,cpu_seconds=time.process_time()-t)),indent=2)+'\n')
 sources=['patterns/upper-tail/conditional-coupling/hko-exact.json','patterns/upper-tail/conditional-coupling/analyze.py','patterns/upper-tail/signed-prefix-response/analyze.py','patterns/upper-tail/geometric-selector/analyze.py','patterns/upper-tail/near-hko-transfer/verify.py','patterns/upper-tail/lp-seeded-response/analyze.py','topics/products/product-qp-six-facet-reduction.tex']
 manifest=dict(sources={p:sha(ROOT/p) for p in sources},freeze={p:sha(OUT/p) for p in ['PLAN.md','freeze.py','analyze.py','sample.json','geometry.json']})
 (OUT/'frozen-manifest.json').write_text(json.dumps(manifest,indent=2)+'\n');print('GEOMETRY_PASS12',sha(OUT/'sample.json'),'CPU',time.process_time()-t)
if __name__=='__main__':main()
