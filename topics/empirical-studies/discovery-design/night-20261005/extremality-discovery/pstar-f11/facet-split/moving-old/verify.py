# /// script
# dependencies = ["sympy"]
# ///
"""Exact premises: added-contact balance, equality continuation, and flat-direction jet."""
import itertools,json,sys
from pathlib import Path
import sympy as S
from sympy.polys.matrices import DomainMatrix as DM
root=Path(__file__).resolve().parent
repo=next(p for p in root.parents if (p/'formal').is_dir() and (p/'experiments').is_dir())
sys.path.insert(0,str(repo/'topics/variation-and-local-maxima/hko-local-maximum/non-hko/code'))
from family_exact import family,O,word_data,gradient,continuation_spec,sym_columns
packet=json.loads((root/'certificate.json').read_text())
cert=json.loads((repo/'topics/variation-and-local-maxima/hko-local-maximum/non-hko/certificate/certificate_base.json').read_text())
v,s,r=S.symbols('v s r');loc={'v':v,'s':s,'r':r};F=S.QQ.frac_field(v,s,r);zero=F.zero
A,vg,_,_=family(v,s);A0,vg0,_,_=family(S.Rational(1,2),S.Rational(1,12))
base={v:S.Rational(1,2),s:S.Rational(1,12),r:S.Rational(1,1200)}
def parse(z):return S.sympify(z,locals=loc)
def ff(z):return F.from_sympy(z)
def regular(z):assert S.cancel(z).as_numer_denom()[1].subs(base)!=0
sig=packet['contact']['word'];b=S.Matrix([parse(z) for z in packet['contact']['beta']]);mu=S.Matrix([parse(z) for z in packet['contact']['mu']]);B,C,H,M,rhs=word_data(A,sig)
y=b.col_join(mu)
assert (DM.from_Matrix(M).convert_to(F)*DM.from_Matrix(y).convert_to(F)-DM.from_Matrix(rhs).convert_to(F)).is_zero_matrix
assert b[0]==r
for z in y:regular(z)
assert min(b.subs(base))>0
# The same old five pivots define a positive continuation for the split section.
pivots=[0,2,3,4,6]
assert C[:,pivots].subs(base).det()==-S.Rational(4608,49)
bf=DM.from_Matrix(b).convert_to(F)
assert (bf.transpose()*DM.from_Matrix(H).convert_to(F)*bf).to_list()[0][0]==F.one
AA=[[ff(z) for z in A[i,:]] for i in range(10)];VG=[[ff(z) for z in vg[i,:]] for i in range(10)]
def grow(word,bb,mm):
 row=[-2*z for rr in VG for z in rr]
 for i,lab in enumerate(word):
  vv=[sum((bb[j]*AA[ll][k]*(1 if j>i else -1 if j<i else 0) for j,ll in enumerate(word)),zero) for k in range(4)]
  ov=[vv[2],vv[3],-vv[0],-vv[1]]
  for k in range(4):row[4*lab+k]-=4*bb[i]*(ov[k]-mm[k])
 return row
bb=[ff(z) for z in b];mm=[ff(z) for z in mu];GR=grow(sig,bb,mm)
W=[ff(parse(z)) for z in packet['W']]
assert sum((x*y for x,y in zip(GR,W)),zero)==zero
assert [F.to_sympy(z).subs(base) for z in W]==list(S.Matrix([[0,0,0,0]]*6+[[S.Rational(1,2),S.Rational(1,3),0,0],[S.Rational(3,2),-1,0,0],[1,0,0,0],[0,0,0,0]]).reshape(40,1))
for z in packet['W']:regular(parse(z))
# Match the maintained proof's uniquely normalized four-row W construction.
rows=[]
for wi in [0,12,23,22]:
 w=cert['sections'][wi];pp,rr,free,y0=continuation_spec(A0,w)
 _,_,_,oldM,oldrhs=word_data(A,w['sigma']);MF=DM.from_Matrix(oldM).convert_to(F);RF=DM.from_Matrix(oldrhs).convert_to(F)
 yy=[ff(z) for z in y0];freey=DM.from_list([[yy[i]] for i in free],F)
 sol=MF.extract(rr,pp).inv()*(RF.extract(rr,[0])-MF.extract(rr,free)*freey)
 for k,i in enumerate(pp):yy[i]=sol.to_list()[k][0]
 assert (MF*DM.from_list([[z] for z in yy],F)-RF).is_zero_matrix
 n=len(w['sigma']);rows.append(grow(w['sigma'],yy[:n],yy[n:]))
prod=[24,25,28,29,32,33,36,37]
restricted=DM.from_list([[row[k] for k in prod] for row in rows],F)
assert restricted.extract(range(4),range(4)).det()!=zero
assert W[32]==F.one and all(W[i]==zero for i in range(40) if i not in prod[:5])
assert (restricted*DM.from_list([[W[k]] for k in prod],F)).is_zero_matrix
# Exact feasible q-only W jet, holding all p weights fixed.
db=[ff(parse(z)) for z in packet['q_W_jet']]
for z in packet['q_W_jet']:regular(parse(z))
assert sum(db,zero)==zero
assert all(db[i]==zero for i,lab in enumerate(sig) if lab>=6)
assert all(sum((db[i]*AA[lab][k]+bb[i]*W[4*lab+k] for i,lab in enumerate(sig)),zero)==zero for k in range(4))
def om(a,c):return a[0]*c[2]+a[1]*c[3]-a[2]*c[0]-a[3]*c[1]
e0=[[bb[i]*z for z in AA[lab]] for i,lab in enumerate(sig)]
e1=[[db[i]*AA[lab][k]+bb[i]*W[4*lab+k] for k in range(4)] for i,lab in enumerate(sig)]
assert all(edge[2]==edge[3]==zero for edge in e1)
assert sum((om(e1[i],e0[j])+om(e0[i],e1[j]) for i,j in itertools.combinations(range(len(sig)),2)),zero)==zero
assert sum((om(e1[i],e1[j]) for i,j in itertools.combinations(range(len(sig)),2)),zero)==zero
# Added gradient annihilates all maintained family/symmetry tangents too.
symmetry=sym_columns(A)
for col in [*(symmetry[:,j] for j in range(symmetry.cols)),A.diff(v).reshape(40,1),A.diff(s).reshape(40,1)]:
 assert sum((GR[k]*ff(col[k]) for k in range(40)),zero)==zero
# Recompute all base old gradients; accept a strict positive added-contact balance.
Grows=[]
for w in cert['sections']:
 oldb=S.Matrix(w['beta']);_,oldC,oldH,_,_=word_data(A0,w['sigma']);mu5=oldC[:,w['pivot']].T.inv(method='DM')*(oldH*oldb)[w['pivot'],0]
 Grows.append(gradient(A0,vg0,w['sigma'],oldb,mu5[:4,0]))
G=S.Matrix.vstack(*Grows);assert G==S.Matrix(cert['gradient_rows'])
Gr=S.Matrix([[F.to_sympy(z).subs(base) for z in GR]])
assert G.to_DM().rank()==22 and G.col_join(Gr).to_DM().rank()==22
lam=S.Matrix(packet['balance']['lambda_old']);gamma=S.Rational(packet['balance']['gamma'])
assert gamma==S.Rational(1,125) and min(lam)>0 and sum(lam)+gamma==1
assert G.T*lam+gamma*Gr.T==S.zeros(40,1)
ell=S.Rational(packet['split']['capacity_loss']);volume=S.Rational(packet['split']['volume_loss'])
margin=gamma*ell-volume
assert margin>0 and margin==S.Rational(packet['balance']['strict_split_margin'])
print('PASS: QQ(v,s,r) stationary contact, rank-five pivot, and full family/symmetry/W annihilation.')
print('PASS: exact feasible q-only W jet gives Q=1/2 along the whole line; base upper curvature is inherited -13/432.')
print('PASS: gamma=1/125, 24 strictly positive old coefficients, full rank22 gradient balance; gamma*ell-L =',margin)
