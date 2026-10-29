"""Exact radial hinge maximization on every complete-simple chart."""
import json,itertools,time
from pathlib import Path
from fractions import Fraction as F
import exact_core as E
ROOT=Path(__file__).parent;start=time.time()
prim=json.loads((ROOT/'primitives.json').read_text());cat=json.loads((ROOT/'catalogue.json').read_text())
def dec(c):return E.K([F(x) for x in c])
V=[]
for key in ['triangles_q','triangles_p']:
 part=[]
 for ids,ws in prim[key]:
  v=[E.ZERO]*10
  for i,w in zip(ids,ws):v[i]=dec(w)/2
  part.append(v)
 V.append(part)
def grad(sig,b):
 return [sum((b[j]*(E.omega(E.A[sig[j]],E.A[sig[i]]) if j<i else E.omega(E.A[sig[i]],E.A[sig[j]])) for j in range(len(sig)) if i!=j),E.ZERO) for i in range(len(sig))]
T=1/(2*E.Q0);N=E.SQRT5-1
def J(a):return [-a[2],-a[3],a[0],a[1]]
def profile(sig,b,I,inv):
 g=grad(sig,b);lam=E.mv(inv,[g[i] for i in I]);assert lam[4]==2*E.Q0
 assert all(E.dot(lam[:4],E.A[lab])+lam[4]==gi for lab,gi in zip(sig,g))
 x=E.scale(-T,lam[:4]);h=[]
 for lab,bw in zip(sig,b):
  end=E.add(x,E.scale(2*T*bw,J(E.A[lab])))
  a=E.A[lab]
  if lab<5:v=[-a[1]/N,a[0]/N];ell=E.dot(v,x[2:]);assert ell==E.dot(v,end[2:])
  else:v=[a[3]/N,-a[2]/N];ell=E.dot(v,x[:2]);assert ell==E.dot(v,end[:2])
  h.append(ell-E.K(F(3,5)));x=end
 return h
results=[];globalbest=E.ZERO
for idx,ch in enumerate(cat['charts']):
 sig=ch['sigma'];qr=[i for i in range(5) if ch['q_vertices_mask']&(1<<i)];pr=[i for i in range(5) if ch['p_vertices_mask']&(1<<i)]
 assert sorted([len(qr),len(pr)]) in [[1,1],[1,2]]
 ends=[]
 for qi,pi in itertools.product(qr,pr):
  b=[V[0][qi][i]+V[1][pi][i] for i in sig]
  assert E.qeval(E.A,sig,b)==E.Q0;ends.append(b)
 b0=ends[0];bd=E.add(ends[-1],E.scale(-1,b0));mid=E.add(b0,E.scale(E.K(F(1,2)),bd))
 assert all(E.sign(b)>0 for b in mid)
 M=[E.A[lab]+[E.ONE] for lab in sig]
 for inds in itertools.combinations(range(len(sig)),5):
  try:inv=E.inv([M[i] for i in inds]);I=inds;break
  except ArithmeticError:pass
 h0=profile(sig,b0,I,inv);h1=profile(sig,ends[-1],I,inv);hd=E.add(h1,E.scale(-1,h0))
 assert E.qeval(E.A,sig,mid)==E.Q0
 breaks=[E.ZERO,E.ONE]
 for a,b in zip(h0,hd):
  if b:
   z=-a/b
   if E.sign(z)>0 and E.sign(1-z)>0 and z not in breaks:breaks.append(z)
 import functools
 breaks.sort(key=functools.cmp_to_key(lambda x,y:E.sign(x-y)))
 cand=list(breaks)
 for l,r in zip(breaks[:-1],breaks[1:]):
  m=(l+r)/2;active=[i for i in range(len(sig)) if E.sign(h0[i]+m*hd[i])>0]
  aa=sum((bd[i]*hd[i] for i in active),E.ZERO)
  bb=sum((bd[i]*h0[i]+b0[i]*hd[i] for i in active),E.ZERO)
  if E.sign(aa)<0:
   z=-bb/(2*aa)
   if E.sign(z-l)>0 and E.sign(r-z)>0:cand.append(z)
 best=E.ZERO;arg=E.ZERO
 for z in cand:
  value=sum(((b0[i]+z*bd[i])*(h0[i]+z*hd[i]) for i in range(len(sig)) if E.sign(h0[i]+z*hd[i])>0),E.ZERO)
  if E.sign(value-best)>0:best=value;arg=z
 if E.sign(best-globalbest)>0:globalbest=best
 results.append({'chart':idx,**ch,'maximum':best.coefficients(),'maximum_numeric':E.nominal(best),'arg':arg.coefficients(),'breaks':len(breaks),'candidates':len(cand)})
delta=E.K(F(2,5));area=5*E.TAU/(1+E.TAU**2);volume=delta**3/(3*E.TAU*area);gap=volume-globalbest
assert E.sign(gap)>0
out={'status':'exact arithmetic optimization on190 complete simple charts; enumeration coverage argument separate',
 'chart_count':len(results),'maximum':globalbest.coefficients(),'maximum_numeric':E.nominal(globalbest),
 'volume':volume.coefficients(),'volume_numeric':E.nominal(volume),'positive_gap':gap.coefficients(),'gap_numeric':E.nominal(gap),
 'log_systolic_derivative':(4*gap).coefficients(),'log_systolic_derivative_numeric':E.nominal(4*gap),
 'seconds':time.time()-start,'charts':results}
(ROOT/'exact-results.json').write_text(json.dumps(out,indent=2)+'\n')
print({k:v for k,v in out.items() if k!='charts'},flush=True)
