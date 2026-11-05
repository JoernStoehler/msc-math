import itertools, json, sys
from fractions import Fraction
from pathlib import Path
import numpy as np
from scipy.optimize import differential_evolution
CERT=Path(__file__).resolve().parents[4]/'topics/capacity-and-qp/convex-systolic-optimization/certificates/hko-ascent'
sys.path.insert(0,str(CERT)); import exact_core as E
prim=json.loads((CERT/'expected/primitives.json').read_text()); cat=json.loads((CERT/'expected/catalogue.json').read_text())
def dec(c):return E.K([Fraction(x) for x in c])
V=[]
for key in ['triangles_q','triangles_p']:
 part=[]
 for ids,ws in prim[key]:
  v=[E.ZERO]*10
  for i,w in zip(ids,ws):v[i]=dec(w)/2
  part.append(v)
 V.append(part)
T=1/(2*E.Q0);N=E.SQRT5-1;SIN=E.TAU*(E.SQRT5+1)/4; m=(1-(E.SQRT5+1)/4)/2; h=(1+(E.SQRT5+1)/4)/2
r=np.sqrt(5);k=r-2;b=r/10;C=(5+2*r)/10

def J(a):return [-a[2],-a[3],a[0],a[1]]
def grad(sig,beta):
 return [sum((beta[j]*(E.omega(E.A[sig[j]],E.A[sig[i]]) if j<i else E.omega(E.A[sig[i]],E.A[sig[j]])) for j in range(len(sig)) if i!=j),E.ZERO) for i in range(len(sig))]
def coords(sig,beta,pivot,inv):
 g=grad(sig,beta);lam=E.mv(inv,[g[i] for i in pivot]);x=E.scale(-T,lam[:4]); out=[]
 for lab,bw in zip(sig,beta):
  if lab<5:
   a=E.A[lab]; e=[-a[1]/N,a[0]/N]
   xi=E.dot(e,x[:2])/E.K(SIN); eta=(E.dot(e,x[2:])-E.K(m))/E.K(h)
  else: out.append((E.ZERO,E.ZERO)); x=E.add(x,E.scale(2*T*bw,J(E.A[lab]))); continue
  out.append((xi,eta));x=E.add(x,E.scale(2*T*bw,J(E.A[lab])))
 return out
def nom(x):return E.nominal(x)
charts=[]
for ci,ch in enumerate(cat['charts']):
 sig=ch['sigma'];qr=[i for i in range(5) if ch['q_vertices_mask']>>i&1];pr=[i for i in range(5) if ch['p_vertices_mask']>>i&1]
 ends=[[V[0][qi][i]+V[1][pi][i] for i in sig] for qi,pi in itertools.product(qr,pr)]
 beta0,beta1=ends[0],ends[-1];bd=E.add(beta1,E.scale(-1,beta0));M=[E.A[lab]+[E.ONE] for lab in sig]
 for inds in itertools.combinations(range(len(sig)),5):
  try: inv=E.inv([M[i] for i in inds]);I=inds;break
  except ArithmeticError: pass
 c0=coords(sig,beta0,I,inv);c1=coords(sig,beta1,I,inv)
 charts.append((np.array([nom(x) for x in beta0]),np.array([nom(x) for x in bd]),np.array([[nom(x),nom(y)] for x,y in c0]),np.array([[nom(x),nom(y)] for x,y in c1])-np.array([[nom(x),nom(y)] for x,y in c0]),sig,ci))

# Three independent affine hinges on the consecutive q facets 0,1,2.
S=(0,1,2)

def maxresp(raw):
 coeff=np.asarray(raw,dtype=float).reshape(3,3)
 coeff=coeff/(np.abs(coeff).sum()+1e-16)
 best=0.
 for beta0,betad,x0,xd,word,_ in charts:
  ids=[j for j,label in enumerate(word) if label in S]
  if not ids:continue
  f0=[];fd=[]
  for j in ids:
   a,b,g=coeff[S.index(word[j])]
   f0.append(a*x0[j,0]+b*x0[j,1]+g)
   fd.append(a*xd[j,0]+b*xd[j,1])
  f0=np.asarray(f0);fd=np.asarray(fd)
  breaks=[0.,1.]
  for a,d in zip(f0,fd):
   if abs(d)>1e-14:
    z=-a/d
    if 0<z<1:breaks.append(z)
  breaks=sorted(set(breaks));cand=list(breaks)
  b0=beta0[ids];bd=betad[ids]
  for l,rgt in zip(breaks[:-1],breaks[1:]):
   mid=(l+rgt)/2; active=np.where(f0+mid*fd>0)[0]
   aa=np.sum(bd[active]*fd[active]);bb=np.sum(bd[active]*f0[active]+b0[active]*fd[active])
   if aa<0:
    z=-bb/(2*aa)
    if l<z<rgt:cand.append(z)
  for z in cand:best=max(best,np.sum((b0+z*bd)*np.maximum(0,f0+z*fd)))
 return best

# Exact xi-averaging formula and high-order quadrature in eta for each density piece.
xg,wg=np.polynomial.legendre.leggauss(96)
def E_xi(t,A):
 A=abs(A)
 if A<1e-14:return np.maximum(t,0)
 return np.where(t>=A,t,np.where(t<=-A,0,(t+A)**2/(4*A)))
def volume(profile):
 alpha,beta,gamma=profile
 total=0.
 for lo,hi,dens in [(-1.,k,lambda y:b*(y+3)),(k,1.,lambda y:C*(1-y))]:
  ys=(hi-lo)*xg/2+(hi+lo)/2; w=(hi-lo)*wg/2
  total+=np.sum(w*dens(ys)*E_xi(beta*ys+gamma,alpha))
 return total/10

def ratio(raw):
 a=np.asarray(raw,dtype=float).reshape(3,3)
 a=a/(np.abs(a).sum()+1e-16)
 vols=[volume(row) for row in a]
 if min(vols)<1e-8:return 10.0
 return maxresp(a.ravel())/sum(vols)

bounds=[(-1,1)]*9
runs=[]
for seed in (17,83):
 res=differential_evolution(ratio,bounds,seed=seed,popsize=7,maxiter=50,tol=2e-5,polish=True,updating='immediate',workers=1)
 coeff=np.asarray(res.x).reshape(3,3);coeff=coeff/(np.abs(coeff).sum()+1e-16)
 vols=[volume(row) for row in coeff]
 item={'seed':seed,'success':res.success,'message':res.message,'profiles_alpha_beta_gamma':coeff.tolist(),'facet_volumes':vols,'total_volume':sum(vols),'max_chart_response':maxresp(coeff.ravel()),'response_volume_ratio':ratio(res.x),'nfev':res.nfev}
 print(json.dumps(item,indent=2),flush=True);runs.append(item)
print('best ratio',min(x['response_volume_ratio'] for x in runs),flush=True)
