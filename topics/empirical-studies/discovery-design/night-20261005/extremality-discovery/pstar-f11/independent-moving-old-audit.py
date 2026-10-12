# /// script
# dependencies = ["sympy"]
# ///
"""Read-only independent base derivative audit for the moving-old review."""
import hashlib,itertools,json
from pathlib import Path
import sympy as S
root=Path(__file__).resolve().parent
repo=next(p for p in root.parents if (p/'formal').is_dir() and (p/'experiments').is_dir())
raw=(root/'facet-split/moving-old/certificate.json').read_bytes();cert=json.loads(raw)
geometry=json.loads((root/'vertex-cones/witnesses.json').read_text());A=S.Matrix(geometry['normal_rows']);K=S.Matrix(geometry['q_vertices']);T=S.Matrix(geometry['p_vertices'])
v,s,r=S.symbols('v s r');loc={'v':v,'s':s,'r':r};base={v:S.Rational(1,2),s:S.Rational(1,12),r:S.Rational(1,1200)}
word=cert['contact']['word'];b=S.Matrix([S.sympify(z,locals=loc).subs(base) for z in cert['contact']['beta']]);C=A[word,:].T.col_join(S.ones(1,len(word)));piv=[0,2,3,4,6]
assert C*b==S.Matrix([0,0,0,0,1]);assert min(b)>0
def om(a,c):return a[0]*c[2]+a[1]*c[3]-a[2]*c[0]-a[3]*c[1]
L={};mid={}
for i in range(10):
    n=A[i,:2] if i<6 else A[i,2:];P=K if i<6 else T
    ends=[P[k,:] for k in range(P.rows) if (n*P[k,:].T)[0]==1];assert len(ends)==2;e=ends[1]-ends[0]
    L[i]=abs(e[0]/n[1]) if n[1]!=0 else abs(e[1]/n[0]);mid[i]=(ends[0]+ends[1])/2
vg=[]
for i in range(10):
    vg.append(list(-L[i]*mid[i]/2)+[-L[i]/12,0] if i<6 else [0,0]+list(-L[i]*mid[i]))
added=[]
for lab in range(10):
    for coord in range(4):
        da=[S.zeros(1,4) for _ in word];db=S.zeros(len(word),1)
        if lab in word:
            pos=word.index(lab);da[pos][coord]=1;rhs=S.zeros(5,1);rhs[coord]=-b[pos];sol=C[:,piv].inv()*rhs
            for k,z in zip(piv,sol):db[k]=z
        dq=sum((db[i]*b[j]+b[i]*db[j])*om(list(A[a,:]),list(A[bb,:]))+b[i]*b[j]*(om(list(da[i]),list(A[bb,:]))+om(list(A[a,:]),list(da[j]))) for i,a in enumerate(word) for j,bb in enumerate(word) if i<j)
        added.append(-4*dq-2*vg[lab][coord])
assert added==list(map(S.Rational,cert['balance']['gradient_new']))
old=json.loads((repo/'topics/variation-and-local-maxima/hko-local-maximum/non-hko/certificate/certificate_base.json').read_text());G=S.Matrix(old['gradient_rows']);lam=S.Matrix(cert['balance']['lambda_old']);gamma=S.Rational(cert['balance']['gamma'])
assert min(lam)>0 and sum(lam)+gamma==1 and gamma==S.Rational(1,125)
assert G.T*lam+gamma*S.Matrix(added)==S.zeros(40,1)
# Direct duplicated-label first derivative, independently of response/KKT formula.
delta=S.Rational(cert['split']['delta']);D=S.Matrix([list(map(S.Rational,cert['split']['D']))]);tau=S.symbols('tau')
nw=[10]+word;B=A.col_join(A[0,:]+tau*D);bn=S.Matrix([delta]+list(b));bn[1]-=delta
db=S.zeros(len(nw),1);sol=C[:,piv].inv()*(-delta*D.T.col_join(S.zeros(1,1)))
for k,z in zip(piv,sol):db[k+1]=z
bet=bn+tau*db
assert all(S.expand(z)==0 for z in B[nw,:].T*bet);assert sum(bet)==1
score=S.expand(sum(bet[i]*bet[j]*om(list(B[a,:]),list(B[bb,:])) for i,a in enumerate(nw) for j,bb in enumerate(nw) if i<j))
ell=2*score.diff(tau).subs(tau,0);assert ell==S.Rational(1,480000)
c=S.Rational(cert['split']['c']);x=S.symbols('x');I1=S.integrate((x-c)*(1-S.Rational(4,3)*x),(x,c,S.Rational(3,4)));loss=S.Rational(5,12)*I1
assert loss==S.Rational(1,86400000);margin=gamma*ell-loss;assert margin==S.Rational(11,2160000000)>0
print('PASS independent merged-gradient derivative, positive full40 cancellation, direct split response and cap first derivative.')
print('weighted split margin:',margin)
print('certificate sha256:',hashlib.sha256(raw).hexdigest())
