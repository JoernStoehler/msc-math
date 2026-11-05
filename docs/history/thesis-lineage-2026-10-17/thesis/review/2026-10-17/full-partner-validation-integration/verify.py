from pathlib import Path
from fractions import Fraction as F
from itertools import combinations
import json,hashlib,argparse,csv,gzip
parser=argparse.ArgumentParser();parser.add_argument("--patterns",type=Path,required=True);parser.add_argument("--results",type=Path,required=True);parser.add_argument("--source",type=Path,required=True);parser.add_argument("--out",type=Path,required=True);parser.add_argument("--sample",type=Path,required=True);args=parser.parse_args()
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

def sparse_vertices(ns):
 out=[]
 for i,j in combinations(range(len(ns)),2):
  x,y=ns[i],ns[j]
  if det(x,y)==0 and dot(x,y)<0:
   c=next(c for c in (0,1) if x[c]);t=-y[c]/(x[c]-y[c]);w=[F(0)]*len(ns);w[i]=t;w[j]=1-t;out.append(w)
 for ids in combinations(range(len(ns)),3):
  x,y,z=[ns[i] for i in ids];num=[det(y,z),det(z,x),det(x,y)];den=sum(num)
  if not den or min(q/den for q in num)<=0:continue
  w=[F(0)]*len(ns)
  for i,q in zip(ids,num):w[i]=q/den
  out.append(w)
 return out

def sparse_word(rows,word,w):
 assert sorted(word)==list(range(len(rows))) and all(t>=0 for t in w)
 # Both block masses are 1/2, and each weighted factor closes.
 left=[i for i,r in enumerate(rows) if any(r[:2])];right=[i for i in range(len(rows)) if i not in left]
 assert sum(w[i] for i in left)==sum(w[i] for i in right)==F(1,2)
 assert all(sum(w[i]*r[c] for i,r in enumerate(rows))==0 for c in range(4))
 return sum(w[i]*w[j]*symp(rows[i],rows[j]) for ii,i in enumerate(word) for j in word[ii+1:])

def below_hko(x):return x<=F(3,5) or (5*x-3)**2<=5

D=json.loads(R.read_text()); src=args.source; assert hashlib.sha256(src.read_bytes()).hexdigest()==D['source_sha256']; names={r['name'] for r in D['rows']}; originals={}
for line in src.open():
 r=json.loads(line)
 if r['name'] in names: originals[r['name']]=r
assert len(originals)==len(names)
for r in D['rows']: assert r['geometry']==originals[r['name']]['dual_vertices_rational'] and r['k']==originals[r['name']]['k']
counts.update(sparse_tie_checks=0,comparator_words=0,width_checks=0)
comparison=[]
with gzip.open(P/'patterns/upper-tail/tail-rows.jsonl.gz','rt') as fh: retained_triangles={r['name']:r for r in map(json.loads,fh) if r['name'] in names}
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

 # Check deterministic first sparse maximizer using independent closed-form barycentrics.
 for v in s['variants']:
  for ns,co,w in [(a if v['first']=='A' else b,f(v['first_coefficients']),f(v['alpha'] if v['first']=='A' else v['gamma'])),(b if v['first']=='A' else a,f(v['second_coefficients']),f(v['gamma'] if v['first']=='A' else v['alpha']))]:
   candidates=sparse_vertices(ns);chosen=max(candidates,key=lambda z:dot(z,co))
   assert chosen==w
   counts['sparse_tie_checks']+=1
 new=F(s['capacity']);assert new==F(row['capacity'])
 cp=row['comparators']
 # All incumbent components retain direct feasible words; no label is used.
 t=cp['triangle'];assert t==retained_triangles[row['name']];ids=t['selected_support_indices'];ww=f(t['certificate']['weights']);indices=ids[0]+[k+j for j in ids[1]]
 localrows=[rows[i] for i in indices];q=sparse_word(localrows,t['certificate']['word'],ww)
 assert q==F(t['certificate']['exact_Q']) and 1/(2*q)==F(row['triangle_capacity'])
 c=cp['contact']['best'];ids=c['supports'];indices=ids[0]+[k+j for j in ids[1]];ww=f(c['weights']);q=sparse_word([rows[i] for i in indices],c['word'],ww)
 assert q==F(c['Q']) and 1/(2*q)==F(c['capacity'])==F(row['contact_capacity'])
 pr=cp['prior_response'];w=pr['witness'];al=f(w['alpha']);ga=f(w['gamma']);closure(a,al);closure(b,ga);q=actual(rows,w['word'],al,ga)
 assert q==F(w['Q']) and 1/(2*q)==F(w['capacity'])==F(row['prior_response_capacity'])
 for v in cp['canonical']:
  q=actual(rows,v['word'],ac,bc);assert q==F(v['Q']) and 1/(2*q)==F(v['capacity'])
  counts['comparator_words']+=1
 assert F(row['canonical_capacity'])==min(F(v['capacity']) for v in cp['canonical'])
 # Width from independent maximal polar-difference contacts.
 ax=[tuple(c/(1+max(-dot(n,p) for p in av)) for c in n) for n in a]
 bx=[tuple(c/(1+max(-dot(n,p) for p in bv)) for c in n) for n in b]
 width=1/max(abs(dot(x,y)) for x in ax for y in bx)
 assert width==F(row['width_capacity'])==F(cp['width_capacity'])
 assert F(row['contact_capacity'])<=width
 inc=min(F(row[z+'_capacity']) for z in ['width','triangle','contact','prior_response','canonical'])
 assert inc==F(row['incumbent_capacity'])==F(cp['incumbent_capacity'])
 ratio=new**2/(2*aa*ab);iratio=inc**2/(2*aa*ab)
 assert ratio==F(row['exact_ratio_bound']) and iratio==F(row['exact_incumbent_ratio'])
 assert ratio<=1 and below_hko(ratio) and iratio<=1 and below_hko(iratio)
 hist=originals[row['name']]['capacity'];assert hist==row['historical_capacity']
 assert abs(float(new)/hist-row['capacity_over_historical'])<1e-14
 comparison.append(dict(name=row['name'],role=row['role'],new_over_incumbent=float(new/inc),exact_new_over_incumbent=str(new/inc),gap=float(new)/hist-1,gain=float(1-new/inc),ratio=float(ratio)))
 counts['comparator_words']+=3;counts['width_checks']+=1;counts['bodies']+=1

S=json.loads(args.sample.read_text());assert [r['name'] for r in D['rows']]==[r['name'] for r in S['sample']]
assert not names.intersection(S['excluded20']);assert len(names)==36
assert set(S['excluded20'])=={r['name'] for r in json.loads((P/'patterns/upper-tail/full-partner-containment/results.json').read_text())['rows']}
for path,h in S['sources'].items():assert hashlib.sha256((P/path).read_bytes()).hexdigest()==h
with gzip.open(P/'patterns/product-shape/features.csv.gz','rt') as fh:rank_rows=list(csv.DictReader(fh))
expected=[]
for k,m in [(5,5),(5,6),(6,6)]:
 family=sorted([r for r in rank_rows if int(r['k'])==k and int(r['m'])==m],key=lambda r:(-float(r['sys']),r['name']))
 assert len(family)==1024
 chosen_tail=[r for r in family[:21] if r['name'] not in S['excluded20']][:9]
 chosen_bulk=sorted([r for r in family[103:] if r['name'] not in S['excluded20']],key=lambda r:hashlib.sha256(r['name'].encode()).hexdigest())[:3]
 for role,selected in [('principal_top2',chosen_tail),('hash_bulk_outside_top10',chosen_bulk)]:
  expected.extend(dict(name=r['name'],bucket=f'{k}x{m}',role=role,family_rank=1+family.index(r)) for r in selected)
assert expected==S['sample']

for r,sample in zip(D['rows'],S['sample']):
 for key in ['name','role','bucket','family_rank']:assert r[key]==sample[key]
tail=[r for r in comparison if r['role']=='principal_top2'];bulk=[r for r in comparison if r['role']!='principal_top2']
summary=dict(tail_n=len(tail),bulk_n=len(bulk),tail_within1pct=sum(r['gap']<=.01+1e-10 for r in tail),tail_within5pct=sum(r['gap']<=.05+1e-10 for r in tail),tail_numerical_agreement=sum(abs(r['gap'])<=1e-10 for r in tail),tail_improvements_ge1pct=sum(F(r['exact_new_over_incumbent'])<=F(99,100) for r in tail),tail_exact_strict_regressions=sum(F(r['exact_new_over_incumbent'])>1 for r in tail),exact5pct_obstruction=[r['name'] for r in tail if F(r['exact_new_over_incumbent'])>F(105,100)],largest_numerical_gap=max(r['gap'] for r in comparison),all_ratio_below1=True,new_ratio_exclusions=0)
assert summary['tail_within1pct']==D['summary']['tail']['new']['within_1pct']
assert summary['tail_within5pct']==D['summary']['tail']['new']['within_5pct']
assert summary['tail_improvements_ge1pct']==D['summary']['tail']['improvements_ge_1pct']
assert summary['exact5pct_obstruction']==D['summary']['exact_5pct_obstructions']
out=dict(counts=counts,summary=summary,comparison=comparison,source_geometry_rows_match=True,sample_sha256=hashlib.sha256(args.sample.read_bytes()).hexdigest(),results_sha256=hashlib.sha256(R.read_bytes()).hexdigest(),checker_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest())
args.out.write_text(json.dumps(out,indent=2)+'\n');print(json.dumps({k:v for k,v in out.items() if k!='comparison'},indent=2))
