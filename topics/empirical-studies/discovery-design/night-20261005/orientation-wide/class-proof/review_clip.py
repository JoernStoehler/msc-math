"""Independent open-feasibility partition reconstruction; read-only."""
from review_signs import *
from functools import cmp_to_key
from itertools import combinations
import hashlib
U=E(0,'1/5');ROOTCACHE={}
def sqrt_field(q):
 if q.sign()<0:return None
 if q==z:return z
 norm=ns['rational_sqrt'](q.a*q.a-5*q.b*q.b)
 if norm is None:return None
 for sign in (1,-1):
  a=ns['rational_sqrt']((q.a+sign*norm)/2)
  if a:
   v=E(a,q.b/(2*a))
   if v*v==q:return v if v.sign()>0 else -v
 if q.b==0:
  b=ns['rational_sqrt'](q.a/5)
  if b is not None:return E(0,b)
 return None
def roots(poly):
 p=trim(poly)
 if degree(p)<=0:return []
 p=monic(p);key=tuple(c.key() for c in p)
 if key in ROOTCACHE:return ROOTCACHE[key]
 if len(p)==2:got=[V(-p[0])]
 else:
  assert len(p)==3
  c,b,a=p;dis=b*b-4*c;sq=sqrt_field(dis)
  if dis.sign()<0:got=[]
  elif sq is not None:got=[V((-b+sq)/2),V((-b-sq)/2)]
  else:
   critical=-b/2;cuts=[z,U]
   if critical.sign()>0 and (U-critical).sign()>0:cuts.insert(1,critical)
   got=[]
   for lo,hi in zip(cuts,cuts[1:]):
    if (lo*lo+b*lo+c).sign()*(hi*hi+b*hi+c).sign()<0:
     branch=0 if (2*((lo+hi)/2)+b).sign()<0 else 1
     rr=ns['Root'](c,b,branch,lo,hi);ns['registry'].setdefault(rr.key,rr)
     got.append(V(z,one,ns['registry'][rr.key]))
 got={v.key():v for v in got if compare(v,V(z))>=0 and compare(v,V(U))<=0}
 out=sorted(got.values(),key=cmp_to_key(compare));ROOTCACHE[key]=out;return out
def sample(lo,hi):
 while True:
  l=lo.p if lo.root is None else lo.root.hi
  h=hi.p if hi.root is None else hi.root.lo
  if (h-l).sign()>0:return V((l+h)/2)
  if lo.root is not None:lo.root.refine()
  if hi.root is not None:hi.root.refine()
def decpoly(p):return trim(list(map(m.decd,p)))
def bound(n,d):return (trim(n),trim(d))
def bvalue(b,u):return evaluate(b[0],u)/evaluate(b[1],u)
def decbound(b):return bound(decpoly(b['numerator']),decpoly(b['denominator']))
def neg(p):return scale(p,-one)
def sub(p,q):return add(p,neg(q))
def samebound(a,b):return sub(mul(a[0],b[1]),mul(b[0],a[1]))==[z]
def main():
 start=time.perf_counter();form=json.loads((root/'formulas.json').read_text());clipped=json.loads((root/'clipped.json').read_text());joint=json.loads((root.parent/'joint-denominators'/'joint-denominators.json').read_text())
 assert clipped['formula_input_sha256']==hashlib.sha256((root/'formulas.json').read_bytes()).hexdigest()
 events=json.loads((root/'feasibility-events.json').read_text());eventmap={(e['domain'],e['formula']):e for e in events['event_domains']};eventmatched=set()
 lookup={(f['vector'],tuple(f['fit_triple'])):i for i,f in enumerate(form['formulas'])}
 got={}
 for i,s in enumerate(clipped['open_segments']):
  key=(s['domain'],s['formula'],decv(s['left']).key(),decv(s['right']).key());assert key not in got;got[key]=s
 exclusions={(v['domain'],v['formula']):v for v in clipped['excluded_zero_edges']}
 matched=set();singletons=0;zeroedge_inc=0;partition_intervals=0;checked_refs=0
 def refcheck(ref,p):
  nonlocal checked_refs
  if p==[z]:assert ref=={'zero':True}
  else:
   assert p==scale(decpoly(clipped['polynomials'][int(ref['key'])]['coefficients']),m.decd(ref['multiplier']))
  checked_refs+=1
 for di,domain in enumerate(form['open_domains']):
  si,ti,ci=domain['incidence'];strip=joint['joint']['topology_strips'][si];substrip=strip['substrips'][ti];cell=substrip['cells'][ci]
  ulo,uhi=decv(substrip['left']),decv(substrip['right'])
  def cbound(rec):
   if rec is None:return None
   r=strip['curves'][rec[0]];return bound(decpoly(r['numerator']),decpoly(r['denominator']))
  initiallo=cbound(cell['left_boundary']) or bound([z],[one]);initialhi=cbound(cell['right_boundary'])
  for ids in combinations(range(5),3):
   fi=lookup[domain['vector'],ids];f=form['formulas'][fi];residuals=[tuple(map(decpoly,r)) for r in f['residuals']]
   zeroedges=[i for i,(c,a) in enumerate(residuals[5:],5) if c==a==[z]]
   if zeroedges:
    assert exclusions[di,fi]['zero_edge_rows']==zeroedges;zeroedge_inc+=1;continue
   assert (di,fi) not in exclusions
   bounds=[initiallo]+([] if initialhi is None else [initialhi])+[bound(neg(c),a) for c,a in residuals if a!=[z]]
   cuts={ulo.key():ulo,uhi.key():uhi}
   eventpolys=[p for pair in residuals for p in pair]
   eventpolys.extend(b[1] for b in bounds)
   eventpolys.extend(sub(mul(a[0],b[1]),mul(b[0],a[1])) for a,b in combinations(bounds,2))
   for p in eventpolys:
    for v in roots(p):
     if compare(ulo,v)<0 and compare(v,uhi)<0:cuts[v.key()]=v
   cuts=sorted(cuts.values(),key=cmp_to_key(compare))
   assert {v.key() for v in cuts[1:-1]}=={decv(d).key() for d in eventmap[di,fi]['internal_cuts']};eventmatched.add((di,fi))
   for left,right in zip(cuts,cuts[1:]):
    partition_intervals+=1;u=sample(left,right);lo,hi=initiallo,initialhi;feasible=True
    for c,a in residuals:
     cv,av=evaluate(c,u),evaluate(a,u)
     if av.sign()==0:
      if cv.sign()<0:feasible=False;break
     else:
      b=bound(neg(c),a)
      if av.sign()>0 and (bvalue(b,u)-bvalue(lo,u)).sign()>0:lo=b
      if av.sign()<0 and (hi is None or (bvalue(b,u)-bvalue(hi,u)).sign()<0):hi=b
    if not feasible or (hi is not None and ((bvalue(hi,u)-bvalue(lo,u)).sign()<0 or bvalue(hi,u).sign()<=0)):continue
    key=(di,fi,left.key(),right.key());s=got[key];matched.add(key)
    assert samebound(lo,decbound(s['lower']))
    assert (hi is None)==(s['upper'] is None)
    if hi is not None:assert samebound(hi,decbound(s['upper']))
    singleton=hi is not None and (bvalue(hi,u)-bvalue(lo,u)).sign()==0
    assert singleton==s['singleton'];singletons+=singleton
    q=list(map(decpoly,f['gap']))
    for check in s['checks']:
     n,d=decbound(check['bound'])
     refcheck(check['gap'],add(add(mul(q[0],mul(d,d)),mul(q[1],mul(n,d))),mul(q[2],mul(n,n))))
     refcheck(check['station_membership'],add(mul(q[1],d),scale(mul(q[2],n),E(2))))
    refcheck(s['stationary_gap'],sub(scale(mul(q[0],q[2]),E(4)),mul(q[1],q[1])))
    refcheck(s['q2'],q[2]);refcheck(s['q1'],q[1])
 assert matched==set(got) and eventmatched==set(eventmap)
 print(json.dumps({'status':'PASS independent complete open-feasibility clipping','open_segments':len(matched),'singleton_x_components':singletons,'excluded_zeroedge_incidences':zeroedge_inc,'partition_intervals':partition_intervals,'root_polynomials':len(ROOTCACHE),'signed_predicate_references':checked_refs,'elapsed_seconds':round(time.perf_counter()-start,3),'writes':False}))
if __name__=='__main__':main()
