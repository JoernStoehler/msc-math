"""Independent reconstruction of Appendix C using SymPy number-field arithmetic.
This checks the printed witness; it does not run the thesis's Sage program.
"""
from pathlib import Path
import json, time
import sympy as sp
from sympy.polys.domains import QQ
from sympy.polys.matrices import DomainMatrix as DM
if not __debug__:
 raise RuntimeError("Do not run this check with Python assertions disabled")
root=Path(__file__).resolve().parent; tm=time.time()
rows=json.loads((root/'hko_rows.json').read_text())
assert len(rows) == 26
x=sp.Symbol('x');ts=sp.Symbol('t')
K=QQ.algebraic_field((sp.Poly(x**4-10*x**2+5,x),sp.sqrt(5-2*sp.sqrt(5))))
t=K.unit;Z=K.zero;O=K.one
s=(5-t*t)/2; al=(3-s)/2;be=t*(1+s)/2;d=s-1
avec=[[1,t,0,0],[-al,be,0,0],[-d,0,0,0],[-al,-be,0,0],[1,-t,0,0],[0,0,t,-1],[0,0,be,al],[0,0,0,d],[0,0,-be,al],[0,0,-t,-1]]
a=[[K.convert(y) for y in v] for v in avec]
def dm(v):return DM(v,(len(v),len(v[0])),K)
def om(u,v):return u[0]*v[2]+u[1]*v[3]-u[2]*v[0]-u[3]*v[1]
def fexpr(st):
 poly=sp.Poly(sp.sympify(st.replace('^','**')),ts)
 return sum((K.convert(co)*t**mon[0] for mon,co in poly.terms()),Z)
def num(y):return float(K.to_sympy(y).evalf(35))
V=25*(5+s)/32; A=5*t-t**3/2;qm=O/(2*A);mu=K.convert(sp.Rational(25,32))+5*s/16
vr=[-mu*y for v in a for y in v]
R=[]; Bs=[];minbeta=1
for idx,(word,I,J,U) in enumerate(rows):
 w=[i-1 for i in word]; ii=[i-1 for i in I];jj=[i-1 for i in J];m=len(w)
 assert len(set(w)) == m and all(0 <= i < 10 for i in w)
 assert sorted(ii+jj) == list(range(m))
 C=dm([[a[i][j] for i in w] for j in range(4)]+[[O]*m])
 ci=C.extract(range(5),ii);inv=ci.inv()
 u=dm([[fexpr(st)] for st in U]);e=dm([[Z],[Z],[Z],[Z],[O]])
 sol=(inv*(e-C.extract(range(5),jj)*u)).to_list()
 b=[Z]*m
 for pos,bi in zip(ii,sol):b[pos]=bi[0]
 for pos,bi in zip(jj,u.to_list()):b[pos]=bi[0]
 assert C*dm([[v] for v in b])==e
 assert all(sp.sign(K.to_sympy(v)) == 1 for v in b)
 minbeta=min(minbeta,*(num(v) for v in b))
 q=sum((b[i]*b[j]*om(a[w[j]],a[w[i]]) for i in range(m) for j in range(i)),Z)
 assert q==qm,(idx,q,qm)
 # independent stationarity/envelope calculation instead of differentiating a section
 H=dm([[Z if i==j else om(a[w[min(i,j)]],a[w[max(i,j)]]) for j in range(m)] for i in range(m)])
 Hb=H*dm([[v] for v in b]);lam=ci.transpose().inv()*Hb.extract(ii,[0])
 assert C.transpose()*lam==Hb,idx
 l=[v[0] for v in lam.to_list()]
 dq=[Z]*40
 for i in range(m):
  for j in range(i):
   for k in range(4):
    unit=[Z]*4;unit[k]=O
    dq[4*w[j]+k]+=b[i]*b[j]*om(unit,a[w[i]])
    dq[4*w[i]+k]+=b[i]*b[j]*om(a[w[j]],unit)
 for k,fac in enumerate(w):
  for coord in range(4):dq[4*fac+coord]-=b[k]*l[coord]
 rr=[-A*dq[i]/(2*V*qm**2)-A**2*vr[i]/(2*V**2) for i in range(40)]
 R.append(rr); Bs.append(b)
print('all 26 exact closure and Q and stationarity identities passed; min beta approx',minbeta, 'seconds',time.time()-tm,flush=True)
# construct Lie algebra generators independently as blocks A,B,C
G=[]
for i in range(2):
 for j in range(2):
  g=[[Z]*4 for _ in range(4)];g[i][j]=O;g[2+j][2+i]=-O;G.append(g)
for off in (0,2):
 for i,j in [(0,0),(0,1),(1,1)]:
  g=[[Z]*4 for _ in range(4)];g[off+i][2-off+j]=O;g[off+j][2-off+i]=O;G.append(g)
T=[[-v[k]*y for v in a for y in v] for k in range(4)]
T.append([-y for v in a for y in v])
for g in G:T.append([-sum((g[j][i]*v[j] for j in range(4)),Z) for v in a for i in range(4)])
MD=dm(R);TD=dm(T)
assert (MD*TD.transpose()).is_zero_matrix
print('exact symmetry annihilation passed',time.time()-tm,flush=True)
import numpy as np
rn=np.array([[num(y) for y in r] for r in R]);tn=np.array([[num(y) for y in r] for r in T])
_,sing,vh=np.linalg.svd(rn.T);ln=vh[-1];ln/=sum(ln)
print('numeric singular values:',sing,flush=True)
print('numeric lambda:',ln,'min',min(ln),'max',max(ln),flush=True)
np.savez(root/'hko_numeric_check.npz',R=rn,T=tn,lambda_=ln)
print('begin exact ranks and kernel',time.time()-tm,flush=True)
rankT=TD.rank();rankM=MD.rank()
assert rankM == 25 and rankT == 15
print('exact ranks',rankM,rankT,'seconds',time.time()-tm,flush=True)
ker=MD.transpose().nullspace().to_list()
assert len(ker)==1
lam=ker[0];scale=sum(lam,Z);lam=[y/scale for y in lam]
assert (dm([lam])*MD).is_zero_matrix
print('exact normalized nullvector computed; sign checking',time.time()-tm,flush=True)
# SymPy's algebraic sign calculation is exact, unlike the numerical diagnostic above.
sgn=[sp.sign(K.to_sympy(y)) for y in lam]
print('lambda signs:',sgn, 'seconds',time.time()-tm,flush=True)
assert sgn==[1]*26
(root/'hko_audit_result.json').write_text(json.dumps({'exact_rank_R':rankM,'exact_rank_T':rankT,'exact_positive_relation':True,'exact_positive_weights':True,'minimum_beta_approx':minbeta,'lambda_exact_in_t':[str(K.to_sympy(y)) for y in lam],'lambda_approx':[num(y) for y in lam],'elapsed_seconds':time.time()-tm},indent=2))
print('INDEPENDENT HKO WITNESS AUDIT PASSED',flush=True)
