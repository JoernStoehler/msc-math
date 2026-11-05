import importlib.util
import argparse
from pathlib import Path
R=Path('/workspaces/msc-math-worktrees/patterns-2026-10-17')
def load(n,p):
 s=importlib.util.spec_from_file_location(n,p);m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
p=load('p',R/'patterns/upper-tail/chord-star/probe.py'); cut=load('cut',R/'patterns/upper-tail/spectral-chord/cut.py')
E=p.E;d=E(0,1)
a=[[E(1),E(0)],[(d-1)/4,E(1)],[-(d+1)/4,(d-1)/2],[-(d+1)/4,-(d-1)/2],[(d-1)/4,E(-1)]];b=[[-x[1],x[0]] for x in a]
aa=a+ [[(a[i][k]+a[(i+1)%5][k])/2 for k in range(2)] for i in range(5)]
P=p.projector(aa)
import json,time
start=time.perf_counter();delta=E(1)/1000
A=a+ [[(a[i][k]+a[(i+1)%5][k])/(2*(1-delta)) for k in range(2)] for i in range(5)]
B=[[-u[1],u[0]] for u in A]
def inspect(A,B):
 A,cA,areaA,zA=p.centroid(A);B,cB,areaB,zB=p.centroid(B)
 PA,PB=p.projector(A),p.projector(B);XA,XB=p.star(cA,PA),p.star(cB,PB)
 CA,CB=p.sh.closures(A),p.sh.closures(B); variants=[]
 for sign in [1,-1]:
  word=p.sh.shadow_word(A,B,sign)
  if sign<0:word=word[::-1]
  for role,points,others in [('A',XA,CB),('B',XB,CA)]:
   for index,x in enumerate(points):
    co=p.sh.coefficients(A,B,word,x,'B' if role=='A' else 'A');v=max(others,key=lambda y:p.dot(co,y))
    alpha,gamma=(x,v) if role=='A' else (v,x);q=p.sh.Q(A,B,word,alpha,gamma)
    assert q==p.dot(co,v)
    variants.append(dict(role=role,index=index,Q=q,word=word,alpha=alpha,gamma=gamma))
 q=max(v['Q'] for v in variants);cap=1/(2*q);vol=areaA*areaB
 width=1/max(abs(p.dot(u,v)) for u in cut.steps(A) for v in cut.steps(B))
 return dict(A=A,B=B,centers=[zA,zB],cone_centers=[cA,cB],projectors=[PA,PB],stars=[XA,XB],closure_vertices=[CA,CB],candidates=variants,Q=q,capacity=cap,volume=vol,ratio=cap*cap/(2*vol),width=width,width_ratio=width*width/(2*vol),areaA=areaA)
z=inspect(A,B)
assert z['centers']==[[0,0],[0,0]]
assert len(z['cone_centers'][0])==10 and min(z['cone_centers'][0])>0
assert z['ratio']>(3+d)/5
# Full limiting qstar = exact response to old central closure. Added normals
# are redundant at zero and every closure value can be reduced onto old rows.
central=[E(1)/5]*5;CA=p.sh.closures(a);CB=p.sh.closures(b);q0=[]
for sign in [1,-1]:
 word=p.sh.shadow_word(a,b,sign)
 if sign<0:word=word[::-1]
 for role,x,others in [('A',central,CB),('B',central,CA)]:
  co=p.sh.coefficients(a,b,word,x,'B' if role=='A' else 'A');q0.append(max(p.dot(co,y) for y in others))
assert max(q0)==d/25
bb=[[-u[1],u[0]] for u in aa]; central10=central+[E(0)]*5
limit_responses=[]
for sign in [1,-1]:
 word=p.sh.shadow_word(aa,bb,sign)
 if sign<0:word=word[::-1]
 for role,x,others in [('A',central10,p.sh.closures(bb)),('B',central10,p.sh.closures(aa))]:
  co=p.sh.coefficients(aa,bb,word,x,'B' if role=='A' else 'A')
  values=[p.dot(co,y) for y in others]; assert max(values)<=d/25
  limit_responses.append(dict(role=role,word=word,coefficients=co,values=values))
assert max(max(r['values']) for r in limit_responses)==d/25
P0=p.projector(aa)
assert all(min(P0[j][i] for j in range(5,10))<0<max(P0[j][i] for j in range(5,10)) for i in range(10))
z.update(delta=delta,limiting_normals=aa,limiting_projector=P0,limit_responses=limit_responses,limiting_Q=d/25,limiting_capacity=5*d/2,limiting_ratio=(3+d)/4,elapsed=time.perf_counter()-start)
out=argparse.ArgumentParser();out.add_argument('--out',type=Path,default=Path('/tmp/msc-math/facet-refinement-finite.json'));args=out.parse_args();args.out.parent.mkdir(parents=True,exist_ok=True)
args.out.write_text(json.dumps(p.serial(z),indent=2)+'\n')
print('STAR',float(z['capacity']),float(z['ratio']),'WIDTH',float(z['width']),float(z['width_ratio']),'LIMIT',float(z['limiting_ratio']),'sec',z['elapsed'],flush=True)
print('stars',list(map(len,z['stars'])),'closure vertices',list(map(len,z['closure_vertices'])))
