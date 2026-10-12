# /// script
# dependencies = ["sympy"]
# ///
"""Independent incident-slack exact cap check; no producer import or writes."""
import json,sys,itertools
from pathlib import Path
import sympy as S
here=Path(__file__).resolve().parent
repo=next(p for p in here.parents if (p/'formal').is_dir())
sys.path.insert(0,str(repo/'experiments/hko-local-maximum/non-hko/code'))
from family_exact import family
A,_,KX,QX=family(S.Rational(1,2),S.Rational(1,12));D=S.Matrix([[1,-1,0,1]])
vertices=[S.Matrix(list(KX[i,:])+list(QX[j,:])) for i in range(6) for j in range(4)]
positives=[x for x in vertices if (A[0,:]*x)[0]==1 and (D*x)[0]>0]
assert len(positives)==1
x=positives[0];incident=[i for i in range(10) if (A[i,:]*x)[0]==1];assert incident==[0,1,6,7]
N=A[incident,:];assert N.det()==-S.Rational(256,7)
gamma=D*N.inv();h=(D*x)[0];assert gamma==S.Matrix([[-S.Rational(2,3),S.Rational(7,12),S.Rational(3,8),S.Rational(1,8)]]) and h==S.Rational(5,12)
heights=[(D*y)[0] for y in vertices if (A[0,:]*y)[0]==1];assert min(abs(z) for z in heights)==S.Rational(1,12)
nonincident=[i for i in range(10) if i not in incident]
limiting=[x]
for j in [1,2,3]:
 slack=S.zeros(4,1);slack[j]=h/gamma[j];y=N.inv()*(S.ones(4,1)-slack);assert (D*y)[0]==0 and (A[0,:]*y)[0]==1
 assert all((A[i,:]*y)[0]<1 for i in nonincident)
 limiting.append(y)
 print('facet-edge intercept',j,list(y),'nonincident slack',[1-(A[i,:]*y)[0] for i in nonincident])
assert all(all((A[i,:]*y)[0]<1 for i in nonincident) for y in limiting)
L=h**4/(24*abs(N.det())*gamma[1]*gamma[2]*gamma[3]);assert L==S.Rational(625,497664)
tau=S.symbols('tau');volume=tau*h**4/(24*abs(N.det())*gamma[1]*gamma[2]*gamma[3]*(1+tau*gamma[0]))
assert S.cancel(volume-625*tau/(497664-331776*tau))==0
assert S.limit(volume/tau,tau,0)==L
print('PASS unique positive vertex, sign margin1/12, regular incident matrix, positive gamma123 and STRICT limiting cap-vertex nonincident slacks.')
print('PASS exact removed simplex volume',volume,'; L=',L,'; coefficient denominator positive for tau near0.')
packet=json.loads((here/'family-policies.json').read_text())
if 'cap' in packet:print('packet cap keys:',packet['cap'].keys())
