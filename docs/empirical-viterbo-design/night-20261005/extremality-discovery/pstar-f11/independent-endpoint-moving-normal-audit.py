# /// script
# dependencies = ["sympy"]
# ///
"""Read-only independent endpoint cap and equality-family HK derivative audit."""
import json
import hashlib
from pathlib import Path
import sympy as S
from sympy.polys.matrices import DomainMatrix as DM
root=Path(__file__).resolve().parent
repo=next(p for p in root.parents if (p/'formal').is_dir() and (p/'experiments').is_dir())
v,s,r,eta,delta=S.symbols('v s r eta delta')
u=1-v+v*v; h=S.Rational(1,2); a=h-s; b=h+s; t=v-h
d1=b/2-a*t;d2=b/2+a*t
A=S.Matrix([[0,2,0,0],[(h-t)/d1,2*s/d1,0,0],[(h+t)/d2,-2*s/d2,0,0],
            [0,-2,0,0],[-(h-t)/d1,-2*s/d1,0,0],[-(h+t)/d2,2*s/d2,0,0],
            [0,0,1/u,1/v],[0,0,1/(u-1),1/v],
            [0,0,1/(u-1),1/(v-1)],[0,0,1/u,1/(v-1)]])
packet=json.loads((repo/'docs/empirical-viterbo-design/night-20261005/extremality-discovery/pstar-f11/facet-split/moving-old/certificate.json').read_text())
word=packet['contact']['word']; loc={'v':v,'s':s,'r':r}
beta=S.Matrix([S.sympify(z,locals=loc) for z in packet['contact']['beta']])
C=A[word,:].T.col_join(S.ones(1,len(word)));piv=[0,2,3,4,6]
assert all(S.cancel(z)==0 for z in C*beta-S.Matrix([0,0,0,0,1]))
D=S.Matrix([[0,-2*(u-eta),1,0]])
F=S.QQ.frac_field(v,s,r,eta,delta)
old_dot=(DM.from_Matrix(C[:,piv]).convert_to(F).inv()*DM.from_Matrix(-delta*D.T.col_join(S.zeros(1,1))).convert_to(F)).to_Matrix()
nw=[10]+word; normals=A.col_join(A[0,:]); bb=S.Matrix([delta]+list(beta));bb[1]-=delta
db=S.zeros(len(nw),1)
for i,z in zip(piv,old_dot):db[i+1]=z
def om(x,y):return x[0]*y[2]+x[1]*y[3]-x[2]*y[0]-x[3]*y[1]
edges=[[F.from_sympy(z) for z in bb[i]*normals[lab,:]] for i,lab in enumerate(nw)]
jets=[[F.from_sympy(z) for z in db[i]*normals[lab,:]+(delta*D if lab==10 else S.zeros(1,4))] for i,lab in enumerate(nw)]
dq=F.to_sympy(sum((om(jets[i],edges[j])+om(edges[i],jets[j]) for i in range(len(nw)) for j in range(i+1,len(nw))),F.zero))
q0=sum((om(edges[i],edges[j]) for i in range(len(nw)) for j in range(i+1,len(nw))),F.zero)
assert q0==F.from_sympy(S.Rational(1,2))
q2=F.to_sympy(sum((om(jets[i],jets[j]) for i in range(len(nw)) for j in range(i+1,len(nw))),F.zero))
Rcoef=S.cancel(q2/delta**2)
assert not (Rcoef.free_symbols & {delta,r})
assert S.cancel(dq-delta*(eta-4*u*r))==0
adapt=S.cancel((2*dq).subs({r:eta/6,delta:eta/12}))
assert S.cancel(adapt-eta**2*(1-2*u/3)/6)==0
print('PASS direct QQ(v,s,r,eta,delta) duplicated-label HK derivative: DQ=delta*(eta−4ur).')
print('With r=eta/6, delta=eta/12: ell=eta²(1−2u/3)/6.')
print('PASS duplicate merge Q0=1/2 and quadratic coefficient Q2=delta² R(v,s,eta).')
coord=S.Matrix([[1,0,0,0],list(A[0,:]),list(A[6,:]),list(A[9,:])])
coef=coord.T.inv()*D.T
expected=S.Matrix([0,-(u-eta),u*v,u*(1-v)])
assert all(S.cancel(z)==0 for z in coef-expected)
assert S.cancel(coord.det()+2/(u*v*(1-v)))==0
assert S.cancel(sum(coef[1:])-eta)==0
print('PASS global cap coordinate identity D.x=eta+(u-eta)t0-uv*t6-u(1-v)*t9.')
print('PASS determinant = -2/[u*v*(1-v)].')
endpoint={v:S.Rational(1,2),s:S.Rational(1,12),r:0,eta:0,delta:0}
regular_data=[*packet['contact']['beta'],*packet['contact']['mu'],
              *packet['W'],*packet['q_W_jet'],*packet['new_gradient']]
for expression in regular_data:
    denominator=S.cancel(S.sympify(expression,locals=loc)).as_numer_denom()[1]
    assert denominator.subs(endpoint)!=0
small_positions=[i for i,lab in enumerate(word) if lab in [0,2]]
eta_beta=[S.cancel(z.subs(r,eta/6)) for z in beta]
slopes=[S.cancel(eta_beta[i]/eta).subs(endpoint) for i in small_positions]
constants=[eta_beta[i].subs(endpoint) for i in range(len(word)) if i not in small_positions]
assert min(slopes)>0 and min(constants)>0
assert S.cancel((eta_beta[0]-eta/12)/eta).subs(endpoint)==S.Rational(1,12)
print('PASS endpoint regular contact/multiplier/W/jet/gradient denominators.')
print('PASS two small-weight eta slopes positive:',slopes,'; constant weights positive:',constants)
# Read the new packet, never its producer implementation. Recompute the added
# base gradient for arbitrary r from direct score differentiation and product
# facet integration, then verify its full forty-component positive balance.
raw=(root/'endpoint-moving-normal/certificate.json').read_bytes()
ep=json.loads(raw)
geometry=json.loads((root/'vertex-cones/witnesses.json').read_text())
A0=S.Matrix(geometry['normal_rows']);K=S.Matrix(geometry['q_vertices']);T=S.Matrix(geometry['p_vertices'])
base_family={v:S.Rational(1,2),s:S.Rational(1,12)}
br=beta.subs(base_family);C0=A0[word,:].T.col_join(S.ones(1,len(word)));inv=C0[:,piv].inv()
vg=[]
for lab in range(10):
    n=A0[lab,:2] if lab<6 else A0[lab,2:];P=K if lab<6 else T
    ends=[P[k,:] for k in range(P.rows) if (n*P[k,:].T)[0]==1]
    assert len(ends)==2
    edge=ends[1]-ends[0];length=abs(edge[0]/n[1]) if n[1]!=0 else abs(edge[1]/n[0]);mid=(ends[0]+ends[1])/2
    vg.append(list(-length*mid/2)+[-length/12,0] if lab<6 else [0,0]+list(-length*mid))
gr=[]
for lab in range(10):
    for coordi in range(4):
        dn=[S.zeros(1,4) for _ in word];jet=S.zeros(len(word),1)
        if lab in word:
            position=word.index(lab);dn[position][coordi]=1;rhs=S.zeros(5,1);rhs[coordi]=-br[position]
            for p,z in zip(piv,inv*rhs):jet[p]=z
        derivative=sum((jet[i]*br[j]+br[i]*jet[j])*om(A0[li,:],A0[lj,:])+br[i]*br[j]*(om(dn[i],A0[lj,:])+om(A0[li,:],dn[j])) for i,li in enumerate(word) for j,lj in enumerate(word) if i<j)
        gr.append(S.expand(-4*derivative-2*vg[lab][coordi]))
assert all(S.cancel(z-S.sympify(w,locals={'r':r}))==0 for z,w in zip(gr,ep['balance']['gradient_new']))
old=json.loads((repo/'experiments/hko-local-maximum/non-hko/certificate/certificate_base.json').read_text())
G=S.Matrix(old['gradient_rows']);lam=S.Matrix([S.sympify(z,locals={'r':r}) for z in ep['balance']['lambda_old']]);gam=S.Rational(ep['balance']['gamma'])
assert gam==S.Rational(1,100000)
assert S.cancel(sum(lam)+gam-1)==0
assert all(S.cancel(z)==0 for z in G.T*lam+gam*S.Matrix(gr))
rmax=S.Rational(ep['balance']['r_interval'][1]);mins=[];t=S.symbols('t')
for expression in lam:
    p=S.Poly(expression,r);degree=p.degree();p0=expression.subs(r,0);p1=expression.subs(r,rmax)
    if degree==0: coeff=[p0];reconstructed=p0
    elif degree==1: coeff=[p0,p1];reconstructed=p0*(1-t)+p1*t
    else:
        assert degree==2
        pm=expression.subs(r,rmax/2);middle=2*pm-(p0+p1)/2
        coeff=[p0,middle,p1];reconstructed=p0*(1-t)**2+2*middle*t*(1-t)+p1*t**2
    assert S.expand(reconstructed-expression.subs(r,rmax*t))==0
    assert min(coeff)>0;mins.extend(coeff)
assert min(mins)==S.Rational(ep['balance']['minimum_bernstein_coefficient'])
E=G.T.col_join(S.ones(1,24));bal=ep['balance']
assert E[bal['pivot_rows'],bal['pivot_columns']].det(method='domain-ge')==S.Rational(bal['pivot_determinant'])!=0
print('PASS independent full forty-gradient r-polynomial balance and reverse Bernstein reconstruction.')
print('PASS fixed augmented rank23 minor and certified positive interval r<=1/1200.')
print('endpoint certificate SHA256:',hashlib.sha256(raw).hexdigest())
