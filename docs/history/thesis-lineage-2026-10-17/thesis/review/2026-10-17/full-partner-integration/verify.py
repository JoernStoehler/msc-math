from pathlib import Path
from fractions import Fraction as F
from itertools import combinations
import json,hashlib,argparse
parser=argparse.ArgumentParser();parser.add_argument("--patterns",type=Path,required=True);parser.add_argument("--results",type=Path,required=True);parser.add_argument("--source",type=Path,required=True);parser.add_argument("--out",type=Path,required=True);args=parser.parse_args()
P=args.patterns
R=args.results
def f(v): return [F(x) for x in v]
def dot(a,b):return sum(x*y for x,y in zip(a,b))
def det(a,b):return a[0]*b[1]-a[1]*b[0]
def hull(pts):
 pts=sorted(set(pts))
 if len(pts)<3:return pts
 def half(seq):
  out=[]
  for x in seq:
   while len(out)>1 and det(tuple(out[-1][i]-out[-2][i] for i in (0,1)),tuple(x[i]-out[-1][i] for i in (0,1)))<=0:out.pop()
   out.append(x)
  return out
 return half(pts)[:-1]+half(pts[::-1])[:-1]
def area(v):return abs(sum(det(v[i],v[(i+1)%len(v)]) for i in range(len(v))))/2
def polygon(ns):
 vs=[]
 for a,b in combinations(ns,2):
  d=det(a,b)
  if d:
   v=((b[1]-a[1])/d,(a[0]-b[0])/d)
   if all(dot(n,v)<=1 for n in ns):vs.append(v)
 return hull(vs)
def symp(a,b):return a[0]*b[2]+a[1]*b[3]-a[2]*b[0]-a[3]*b[1]
def actual(rows,word,alpha,gamma):
 w=[x/2 for x in alpha+gamma]
 return sum(w[i]*w[j]*symp(rows[i],rows[j]) for ii,i in enumerate(word) for j in word[ii+1:])
def closure(ns,w):
 assert all(x>=0 for x in w) and sum(w)==1
 assert all(sum(x*n[c] for x,n in zip(w,ns))==0 for c in (0,1))
def edgepoly(ns,w,rotate=False):
 # Independent exact angular sort through the convex hull of ray intersections is unnecessary:
 # retain source cyclic normal order (validated later by convex path area and hull).
 from functools import cmp_to_key
 def cmp(a,b):
  ha=int(a[1]<0 or (a[1]==0 and a[0]<0)); hb=int(b[1]<0 or (b[1]==0 and b[0]<0))
  if ha!=hb:return -1 if ha<hb else 1
  d=det(a,b);return -1 if d>0 else 1 if d<0 else 0
 es=[tuple(x*c for c in n) for n,x in zip(ns,w) if x]
 if rotate:es=[(-e[1],e[0]) for e in es]
 es.sort(key=cmp_to_key(cmp));v=(F(0),F(0));out=[]
 for e in es:out.append(v);v=tuple(v[c]+e[c] for c in (0,1))
 assert v==(0,0)
 return hull(out)
def mixed(u,v):
 w=hull([tuple(a[i]+b[i] for i in (0,1)) for a in u for b in v])
 return (area(w)-area(u)-area(v))/2
counts=dict(bodies=0,closures=0,direct_4D=0,dual_certificates=0,containment_coefficients=0,mixed_area_identities=0)
def cert(ns,co,w,d):
 t=F(d['t']);z=f(d['z']);slack=[t+dot(z,n)-c for n,c in zip(ns,co)]
 assert min(slack)>=0 and dot(w,co)==t==F(d['primal'])
 assert slack==f(d['slack'])
 assert all(x*s==0 for x,s in zip(w,slack))
 counts['dual_certificates']+=1
D=json.loads(R.read_text()); src=args.source; assert hashlib.sha256(src.read_bytes()).hexdigest()==D['source_sha256']; names={r['name'] for r in D['rows']}; originals={}
for line in src.open():
 r=json.loads(line)
 if r['name'] in names: originals[r['name']]=r
assert len(originals)==len(names)
for r in D['rows']: assert r['geometry']==originals[r['name']]['dual_vertices_rational'] and r['k']==originals[r['name']]['k']
prior=json.loads((P/'patterns/upper-tail/signed-prefix-response/incumbent-comparison.json').read_text());prior={r['name']:r for r in prior['rows']}
comparison=[]
for row in D['rows']:
 rows=[tuple(f(v)) for v in row['geometry']];k=row['k'];a=[v[:2] for v in rows[:k]];b=[v[2:] for v in rows[k:]]
 av=polygon(a);bv=polygon(b);aa=area(av);ab=area(bv);s=row['selection']
 assert aa==F(s['area_A']) and ab==F(s['area_B'])
 ac=f(s['canonical_alpha']);bc=f(s['canonical_gamma'])
 for ns,w in [(a,ac),(b,bc)]:closure(ns,w);counts['closures']+=1
 # Canonical polygons reproduce full primal partner after the quarter turn and area scale.
 for ns,w,pv,A in [(a,ac,av,aa),(b,bc,bv,ab)]:
  ev=edgepoly(ns,w,True);scaled=[tuple(x/(2*A) for x in v) for v in pv]
  assert area(ev)==area(scaled)
  for direction in [(1,0),(0,1),(1,1),(1,-1)]:
   assert max(dot(direction,v) for v in ev)-min(dot(direction,v) for v in ev)==max(dot(direction,v) for v in scaled)-min(dot(direction,v) for v in scaled)
 for v in s['variants']:
  al=f(v['alpha']);ga=f(v['gamma']);closure(a,al);closure(b,ga);counts['closures']+=2
  word=v['word'];assert sorted(word)==list(range(len(rows)))
  q=actual(rows,word,al,ga);assert q==F(v['Q'])==1/(2*F(v['capacity']));counts['direct_4D']+=1
  first=v['first'];firstal,firstga=(al,bc) if first=='A' else (ac,ga)
  qfirst=actual(rows,word,firstal,firstga);assert qfirst==F(v['first_Q']);counts['direct_4D']+=1
  ns,other,chosen,pv,A=(a,bc,al,bv,ab) if first=='A' else (b,ac,ga,av,aa)
  co=[]
  for j in range(len(ns)):
   basis=[F(int(i==j)) for i in range(len(ns))]
   co.append(actual(rows,word,basis,other) if first=='A' else actual(rows,word,other,basis))
  assert co==f(v['first_coefficients']);cert(ns,co,chosen,v['first_dual'])
  sign=v['sign'];orientation=-sign if first=='A' else sign
  geo=[max(dot(p,(-orientation*n[1],orientation*n[0])) for p in pv)/(4*A) for n in ns]
  assert geo==f(v['containment_coefficients']);cert(ns,geo,chosen,v['containment_dual']);assert dot(chosen,geo)==qfirst
  assert F(v['containment_radius'])==F(v['containment_dual']['t'])*4*A;counts['containment_coefficients']+=len(ns)
  ns2,w2=(b,ga) if first=='A' else (a,al)
  co2=[]
  for j in range(len(ns2)):
   basis=[F(int(i==j)) for i in range(len(ns2))]
   co2.append(actual(rows,word,al,basis) if first=='A' else actual(rows,word,basis,ga))
  assert co2==f(v['second_coefficients']);cert(ns2,co2,w2,v['second_dual'])
  assert q>=qfirst
  for x,y,objective in [(firstal,firstga,qfirst),(al,ga,q)]:
   u=edgepoly(a,x);vv=edgepoly(b,y,True)
   if sign<0:vv=[(-p[0],-p[1]) for p in vv]
   assert mixed(u,vv)==objective;counts['mixed_area_identities']+=1
 assert F(s['chosen']['Q'])==max(F(v['Q']) for v in s['variants'])
 new=F(s['capacity']);old=min(F(prior[row['name']]['exact_incumbent_capacity']),F(prior[row['name']]['exact_new_capacity']))
 comparison.append(dict(name=row['name'],principal=row['principal_top3'],new_over_strongest_prior=float(new/old),exact_gain=str(1-new/old),new_capacity=str(new),prior_capacity=str(old),new_over_historical=row['capacity_over_historical']))
 counts['bodies']+=1
# Exact known challenger recovery independent of numerical labels.
trap=next(r for r in D['rows'] if r['name']=='random_5x5_s1_64');chall=json.loads((P/'patterns/upper-tail/geometric-selector/falsifier.json').read_text())['feasible_challenger']

cap=next(F(chall[k]) for k in ['capacity','capacity_bound'] if k in chall)
assert F(trap['selection']['capacity'])==cap
out=dict(counts=counts,source_results_sha256=hashlib.sha256(R.read_bytes()).hexdigest(),checker_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),source_geometry_rows_match=True,exact_challenger_recovered=True,comparison=comparison,improvements=sum(F(r['exact_gain'])>0 for r in comparison),principal_improvements=sum(r['principal'] and F(r['exact_gain'])>0 for r in comparison),worst_numerical_gap=max(r['new_over_historical']-1 for r in comparison))
args.out.write_text(json.dumps(out,indent=2)+'\n')
print(json.dumps({k:v for k,v in out.items() if k!='comparison'},indent=2))
