#!/usr/bin/env python3
"""Frozen exact surrogate-ceiling test; no eta or full capacity oracle."""
import csv,gzip,hashlib,importlib.util,json,resource,sys,time
from fractions import Fraction as F
from pathlib import Path
sys.set_int_max_str_digits(0)
resource.setrlimit(resource.RLIMIT_CPU,(300,300))
R=Path('/workspaces/msc-math-worktrees/patterns-2026-10-17')
D=Path(__file__).resolve().parent
S=Path('/home/joern/.cache/msc-math/artifacts/polytope-datasets/f7bc6be841e9d30741d5bf7ec8d4f0c0c74ec22745d3325b792e64cec333ca96/files/random-product.jsonl')
def mod(name,path):
 s=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
p=mod('star_crux',R/'patterns/upper-tail/chord-star/probe.py')
cut=mod('width_crux',R/'patterns/upper-tail/spectral-chord/cut.py')
v=mod('direct_crux',R/'patterns/upper-tail/chord-star/verify.py')
H=(3+p.E(0,1))/5
def select(a,b):
 a,ac,aa,ca=p.centroid(a);b,bc,ab,cb=p.centroid(b)
 pa,pb=p.projector(a),p.projector(b);xa,xb=p.star(ac,pa),p.star(bc,pb)
 va,vb=p.sh.closures(a),p.sh.closures(b)
 for normals,points in [(a,xa),(b,xb)]:
  assert all(min(x)>=0 and sum(x)==1 and p.mv(p.tr(normals),x)==[0,0] for x in points)
 variants=[]
 for sign in [1,-1]:
  word=p.sh.shadow_word(a,b,sign)
  if sign<0:word=word[::-1]
  candidates=[]
  for role,points,other in [('A',xa,vb),('B',xb,va)]:
   for index,x in enumerate(points):
    co=p.sh.coefficients(a,b,word,x,'B' if role=='A' else 'A')
    y=max(other,key=lambda z:p.dot(co,z));alpha,gamma=(x,y) if role=='A' else (y,x)
    alpha=[F(t) if isinstance(t,int) else t for t in alpha];gamma=[F(t) if isinstance(t,int) else t for t in gamma]
    q=p.sh.Q(a,b,word,alpha,gamma)
    assert q>=0 and v.direct(a,b,word,alpha,gamma)==q
    candidates.append(dict(role=role,index=index,Q=q,alpha=alpha,gamma=gamma))
  variants.append(dict(sign=sign,word=word,best=max(candidates,key=lambda z:z['Q']),candidate_Q=[z['Q'] for z in candidates]))
 best=max(variants,key=lambda z:z['best']['Q']);q=best['best']['Q'];cap=1/(2*q)
 return dict(centered_A=a,centered_B=b,centers=[ca,cb],canonical_closures=[ac,bc],projectors=[pa,pb],stars=[xa,xb],closure_vertices=[va,vb],variants=variants,best=best,capacity=cap,volume=aa*ab,ratio=cap*cap/(2*aa*ab))
def main():
 start=time.perf_counter();source_hash=hashlib.sha256(S.read_bytes()).hexdigest()
 assert source_hash=='66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736'
 with gzip.open(R/'patterns/product-shape/features.csv.gz','rt') as f:
  names=[r['name'] for r in csv.DictReader(f) if (int(r['k']),int(r['m'])) in [(5,5),(5,6),(6,6)] and p.E(F(r['sys2']))>H]
 frozen=[line[2:] for line in (D/'PLAN.md').read_text().splitlines() if line.startswith('- random_')]
 assert names==frozen and len(names)==14
 geometry={}
 for line in S.open():
  r=json.loads(line)
  if r['name'] in names:
   duals=[[F(x) for x in u] for u in r['dual_vertices_rational']];k=r['k']
   assert all(u[2:]==[0,0] for u in duals[:k]) and all(u[:2]==[0,0] for u in duals[k:])
   geometry[r['name']]=dict(name=r['name'],A=[u[:2] for u in duals[:k]],B=[u[2:] for u in duals[k:]])
 assert len(geometry)==14
 (D/'geometry.json').write_text(json.dumps(p.serial([geometry[name] for name in names]),indent=2)+'\n')
 # Same algebraic HKO geometry as the retained construction, no eta work.
 d=p.E(0,1);a=[[p.E(1),p.E(0)],[(d-1)/4,p.E(1)],[-(d+1)/4,(d-1)/2],[-(d+1)/4,-(d-1)/2],[(d-1)/4,p.E(-1)]];b=[[-x[1],x[0]] for x in a]
 h=select(a,b);assert h['capacity']==5 and h['ratio']==H
 (D/'hko.json').write_text(json.dumps(p.serial(dict(status='PASS',capacity=h['capacity'],ratio=h['ratio'],best=h['best'])),indent=2)+'\n')
 rows=[];stop='all_frozen_rows_survive'
 for name in names:
  g=geometry[name];z=select(g['A'],g['B'])
  width=1/max(abs(p.dot(u,w)) for u in cut.steps(g['A']) for w in cut.steps(g['B']))
  wr=width*width/(2*z['volume']);assert p.E(wr)>H
  ceiling=min(width,z['capacity']);ratio=ceiling*ceiling/(2*z['volume']);failed=p.E(ratio)>H
  rows.append(dict(name=name,selection=z,width=width,width_ratio=wr,ceiling=ceiling,ratio=ratio,surrogate_counterexample=failed))
  print(name,'width',float(wr),'star',float(z['ratio']),'min',float(ratio),'FAIL' if failed else 'PASS',flush=True)
  out=dict(rows=rows,status='counterexample' if failed else 'running',frozen_names=names,source_sha256=source_hash,plan_sha256=hashlib.sha256((D/'PLAN.md').read_bytes()).hexdigest(),script_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),elapsed_seconds=time.perf_counter()-start,cpu_seconds=time.process_time(),limits='No eta, paired capacity oracle, historical target join, fitting or theorem claim')
  (D/'results.json').write_text(json.dumps(p.serial(out),indent=2)+'\n')
  if failed:stop='first_strict_surrogate_counterexample';break
 out.update(status=stop,elapsed_seconds=time.perf_counter()-start,cpu_seconds=time.process_time())
 (D/'results.json').write_text(json.dumps(p.serial(out),indent=2)+'\n')
 print(stop,'wall',out['elapsed_seconds'],'cpu',out['cpu_seconds'])
if __name__=='__main__':main()
