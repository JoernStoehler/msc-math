#!/usr/bin/env python3
"""Exact closure-star pilot; no paired-capacity enumeration or target reads."""
import argparse, hashlib, importlib.util, itertools, json, math, sys, time
from fractions import Fraction as F
from pathlib import Path
sys.set_int_max_str_digits(0)
R=Path('/workspaces/msc-math-worktrees/patterns-2026-10-17')
D=Path(__file__).resolve().parent
def module(name,path):
 s=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
sh=module('chord_shadow',R/'patterns/upper-tail/signed-prefix-response/analyze.py')
E=module('chord_q5',R/'patterns/upper-tail/conditional-coupling/quadratic.py').Q5
def serial(v):
 if isinstance(v,F):return str(v)
 if isinstance(v,E):return v.serial()
 if isinstance(v,dict):return {k:serial(x) for k,x in v.items()}
 if isinstance(v,(tuple,list)):return [serial(x) for x in v]
 return v
def dot(a,b):return sum(x*y for x,y in zip(a,b))
def tr(a):return list(map(list,zip(*a)))
def mv(a,v):return [dot(u,v) for u in a]
def mm(a,b):return [[dot(u,v) for v in tr(b)] for u in a]
def inverse(a):
 n=len(a);a=[u[:]+[F(i==j) for j in range(n)] for i,u in enumerate(a)]
 for j in range(n):
  k=next(i for i in range(j,n) if a[i][j]);a[j],a[k]=a[k],a[j];z=a[j][j];a[j]=[v/z for v in a[j]]
  for i in range(n):
   if i!=j:
    z=a[i][j];a[i]=[x-z*y for x,y in zip(a[i],a[j])]
 return [u[n:] for u in a]
def centroid(a):
 vv=[]
 for u,v in itertools.combinations(a,2):
  determinant=sh.cross(u,v)
  if not determinant:continue
  x=[(v[1]-u[1])/determinant,(u[0]-v[0])/determinant]
  if all(dot(w,x)<=1 for w in a) and x not in vv:vv.append(x)
 mean=[sum(v[k] for v in vv)/len(vv) for k in range(2)];vv.sort(key=lambda v:math.atan2(float(v[1]-mean[1]),float(v[0]-mean[0])))
 twice=sum(sh.cross(vv[i],vv[(i+1)%len(vv)]) for i in range(len(vv)));assert twice>0
 cen=[sum((vv[i][k]+vv[(i+1)%len(vv)][k])*sh.cross(vv[i],vv[(i+1)%len(vv)]) for i in range(len(vv)))/(3*twice) for k in range(2)]
 aa=[[v/(1-dot(u,cen)) for v in u] for u in a];weights=[F(0)]*len(a)
 for i,v in enumerate(vv):
  w=vv[(i+1)%len(vv)];active=[j for j,u in enumerate(a) if dot(u,v)==dot(u,w)==1];assert len(active)==1
  weights[active[0]]=sh.cross([v[k]-cen[k] for k in range(2)],[w[k]-cen[k] for k in range(2)])/twice
 assert min(weights)>0 and sum(weights)==1 and mv(tr(aa),weights)==[0,0]
 return aa,weights,twice/2,cen
def projector(a):
 n=len(a);h=[[F(1)]*n]+tr(a);z=mm(mm(tr(h),inverse(mm(h,tr(h)))),h)
 return [[F(i==j)-z[i][j] for j in range(n)] for i in range(n)]
def unique(vs):
 out=[]
 for v in vs:
  if v not in out:out.append(v)
 return out
def star(c,p):
 points=[c]
 for u in tr(p):
  if not any(u):continue
  low=max(-x/v for x,v in zip(c,u) if v>0);high=min(-x/v for x,v in zip(c,u) if v<0)
  points.extend([[x+t*v for x,v in zip(c,u)] for t in [low,high]])
 return unique(points)
def det(a):
 if len(a)==1:return a[0][0]
 return sum((-1)**j*a[0][j]*det([u[:j]+u[j+1:] for u in a[1:]]) for j in range(len(a)))
def containment(c,p,points,vertices):
 d=len(c)-3
 if not d:return dict(eta=F(1),dimension=0,facets=[])
 # A nonsingular principal projector minor supplies an exact tangent chart.
 ids=next(ids for ids in itertools.combinations(range(len(c)),d) if det([[p[i][j] for j in ids] for i in ids]))
 xs=[[x[i]-c[i] for i in ids] for x in points];vs=[[v[i]-c[i] for i in ids] for v in vertices];facets=[]
 if d==1:
  lo=min(x[0] for x in xs);hi=max(x[0] for x in xs);facets=[[1/hi],[-1/lo]]
 else:
  for comb in itertools.combinations(xs,d):
   edge=[[comb[k][j]-comb[0][j] for j in range(d)] for k in range(1,d)]
   normal=[(-1)**j*det([u[:j]+u[j+1:] for u in edge]) for j in range(d)]
   if not any(normal):continue
   height=dot(normal,comb[0]);sides=[dot(normal,x)-height for x in xs]
   if max(sides)>0 and min(sides)<0:continue
   if min(sides)>=0:normal=[-x for x in normal];height=-height
   assert height>0
   f=[x/height for x in normal]
   if f not in facets:facets.append(f)
 assert facets and all(dot(f,x)<=1 for f in facets for x in xs)
 extents=[max(dot(f,v) for v in vs) for f in facets];eta=min([F(1)]+[1/h for h in extents if h>0])
 assert eta>0 and all(eta*dot(f,v)<=1 for f in facets for v in vs)
 return dict(eta=eta,dimension=d,chart_indices=ids,facets=facets,extents=extents)
def select(a,b,hko=False):
 a,ac,area_a,center_a=centroid(a);b,bc,area_b,center_b=centroid(b);pa,pb=projector(a),projector(b);xa,xb=star(ac,pa),star(bc,pb);ca,cb=sh.closures(a),sh.closures(b)
 for normals,points in [(a,xa),(b,xb)]:
  assert all(min(x)>=0 and sum(x)==1 and mv(tr(normals),x)==[0,0] for x in points)
 if hko:
  assert all(v in xa for v in ca) and all(v in xb for v in cb)
  ea=eb=dict(eta=E(1),all_vertices_in_star=True)
 else:ea,eb=containment(ac,pa,xa,ca),containment(bc,pb,xb,cb)
 variants=[]
 for sign in [1,-1]:
  word=sh.shadow_word(a,b,sign)
  if sign<0:word=word[::-1]
  candidates=[]
  for kind,points,other in [('A',xa,cb),('B',xb,ca)]:
   for i,x in enumerate(points):
    co=sh.coefficients(a,b,word,x,'B' if kind=='A' else 'A');response=max(other,key=lambda y:dot(co,y));alpha,gamma=(x,response) if kind=='A' else (response,x)
    q=sh.Q(a,b,word,alpha,gamma);assert q>=0
    candidates.append(dict(kind=kind,index=i,alpha=alpha,gamma=gamma,Q=q,coefficients=co,response_values=[dot(co,y) for y in other]))
  variants.append(dict(sign=sign,word=word,candidates=candidates,best=max(candidates,key=lambda x:x['Q'])))
 best=max(variants,key=lambda x:x['best']['Q']);q=best['best']['Q'];eta=max(ea['eta'],eb['eta']);volume=area_a*area_b;cap=1/(2*q)
 return dict(capacity=cap,ratio=cap*cap/(2*volume),eta=eta,lower_capacity=eta*cap,lower_ratio=eta*eta*cap*cap/(2*volume),centered_A=a,centered_B=b,centers=[center_a,center_b],canonical_closures=[ac,bc],projectors=[pa,pb],stars=[xa,xb],closure_vertices=[ca,cb],containment=[ea,eb],volume=volume,best=best,variants=variants)
def hko_control():
 d=E(0,1);a=[[E(1),E(0)],[(d-1)/4,E(1)],[-(d+1)/4,(d-1)/2],[-(d+1)/4,-(d-1)/2],[(d-1)/4,E(-1)]];b=[[-v[1],v[0]] for v in a];out=select(a,b,True);assert out['capacity']==5 and out['eta']==1 and out['ratio']==(3+d)/5
 return dict(status='PASS',capacity=out['capacity'],eta=out['eta'],ratio=out['ratio'],star_sizes=list(map(len,out['stars'])),closure_counts=list(map(len,out['closure_vertices'])),witness=out['best'])
def controls():
 base=json.loads((R/'patterns/upper-tail/contact-stability/global-counterexample-input.json').read_text());a=[[F(x) for x in v] for v in base['A']];b=[[F(x) for x in v] for v in base['B']];ta=[F(1,17),F(-1,31)];tb=[F(-1,23),F(1,37)]
 sources=[base,dict(name='permutation',A=[a[i] for i in [3,0,4,1,2]],B=[b[i] for i in [4,2,0,3,1]]),dict(name='factor_swap',A=b,B=a),dict(name='translation',A=[[x/(1+dot(u,ta)) for x in u] for u in a],B=[[x/(1+dot(u,tb)) for x in u] for u in b])]
 for name,g in [('positive_GL',[[F(2),F(1)],[F(0),F(1)]]),('negative_GL',[[F(2),F(1)],[F(0),F(-1)]])]:sources.append(dict(name=name,A=tr(mm(tr(inverse(g)),tr(a))),B=tr(mm(g,tr(b)))))
 rows=[]
 for s in sources:
  out=select([[F(x) for x in v] for v in s['A']],[[F(x) for x in v] for v in s['B']]);rows.append(dict(name=s['name'],capacity=out['capacity'],ratio=out['ratio'],eta=out['eta']))
 assert all((x['capacity'],x['ratio'],x['eta'])==(rows[0]['capacity'],rows[0]['ratio'],rows[0]['eta']) for x in rows)
 return dict(status='PASS',rows=rows)
def main():
 parser=argparse.ArgumentParser();parser.add_argument('mode',choices=['controls','predict']);o=parser.parse_args();t=time.perf_counter()
 if o.mode=='controls':out=dict(hko=hko_control(),invariance=controls())
 else:
  source=json.loads((R/'patterns/upper-tail/near-hko-transfer/sample.json').read_text())['rows'][:6]
  source += [{k:r[k] for k in ['name','A','B']} for r in json.loads((R/'patterns/upper-tail/conditional-coupling/results.json').read_text())['rows']]
  source += [json.loads((R/'patterns/upper-tail/contact-stability/global-counterexample-input.json').read_text())]
  geometry=[{k:s[k] for k in ['name','A','B']} for s in source];(D/'geometry.json').write_text(json.dumps(serial(geometry),indent=2)+'\n');rows=[]
  for s in geometry:
   z=select([[F(x) for x in v] for v in s['A']],[[F(x) for x in v] for v in s['B']]);rows.append(dict(name=s['name'],selection=z));print(s['name'],'C',float(z['capacity']),'eta',float(z['eta']),'S',float(z['ratio']),flush=True)
  out=dict(rows=rows)
 out.update(plan_sha256=hashlib.sha256((D/'PLAN.md').read_bytes()).hexdigest(),script_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),elapsed_seconds=time.perf_counter()-t)
 target=D/(o.mode+'.json' if o.mode=='controls' else 'predictions.json');assert not target.exists();target.write_text(json.dumps(serial(out),indent=2)+'\n');print('WROTE',target,'SECONDS',out['elapsed_seconds'])
if __name__=='__main__':main()
