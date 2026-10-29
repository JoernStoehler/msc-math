"""Independent exact check of the new literal witness; no submitted imports."""
from fractions import Fraction as F
class E:
 def __init__(self,x=0):
  self.c=x.c if isinstance(x,E) else tuple(map(F,x))+(F(0),)*(4-len(x)) if isinstance(x,(tuple,list)) else (F(x),F(0),F(0),F(0))
 def __add__(self,y):return E([a+b for a,b in zip(self.c,E(y).c)])
 __radd__=__add__
 def __neg__(self):return E([-a for a in self.c])
 def __sub__(self,y):return self+-E(y)
 def __rsub__(self,y):return E(y)+-self
 def __mul__(self,y):
  z=[F(0)]*7
  for i,a in enumerate(self.c):
   for j,b in enumerate(E(y).c):z[i+j]+=a*b
  for i in (6,5,4):z[i-2]+=10*z[i];z[i-4]-=5*z[i]
  return E(z[:4])
 __rmul__=__mul__
 def __bool__(self):return any(self.c)
 def __eq__(self,y):return self.c==E(y).c
 def __truediv__(self,y):
  y=E(y);cols=[(y*E([0]*k+[1])).c for k in range(4)]
  inv=solve([[cols[j][i] for j in range(4)] for i in range(4)],[F(1),F(0),F(0),F(0)])
  return self*E(inv)
 def __rtruediv__(self,y):return E(y)/self
 def __repr__(self):return str(self.c)
def solve(A,b):
 A=[list(row)+[x] for row,x in zip(A,b)];n=len(A)
 for i in range(n):
  p=next(k for k in range(i,n) if A[k][i]);A[i],A[p]=A[p],A[i];v=A[i][i];A[i]=[x/v for x in A[i]]
  for k in range(n):
   if k!=i:
    v=A[k][i];A[k]=[x-v*y for x,y in zip(A[k],A[i])]
 return [row[-1] for row in A]
def dot(a,b):return sum(x*y for x,y in zip(a,b))
def mul(c,x):return [c*y for y in x]
def add(x,y):return [a+b for a,b in zip(x,y)]
def J2(x):return [-x[1],x[0]]
def Jc(x):return [-x[2],-x[3],x[0],x[1]]
def om(x,y):return dot(Jc(x),y)
t=E([0,1]);u=t*t;r=(5-u)/2;zero=[E(),E()]
n=[[E(1),t],[-(3-r)/2,t*(1+r)/2],[1-r,E()],[-(3-r)/2,-t*(1+r)/2],[E(1),-t]]
a=[v+zero for v in n]+[zero+mul(-1,J2(v)) for v in n]
word=[0,6,5,2,1,8,4]
b=[v/200 for v in [17+5*u,25+5*u,25+5*u,44-8*u,E(8),50-10*u,31+3*u]]
assert sum(b)==1
assert all(dot(b,[a[j][k] for j in word])==0 for k in range(4))
Q=sum(b[i]*b[j]*om(a[word[i]],a[word[j]]) for i in range(7) for j in range(i+1,7))
assert Q==t/5
p=[sum(b[j]*(om(a[word[j]],a[word[i]]) if j<i else om(a[word[i]],a[word[j]])) for j in range(7) if j!=i) for i in range(7)]
M=[a[j]+[E(1)] for j in word];lam=solve(M[:5],p[:5])
assert all(dot(row,lam)==v for row,v in zip(M,p)) and lam[4]==2*t/5
T=5/(2*t);x=mul(-T,lam[:4]);hinges=[]
for label,weight in zip(word,b):
 ell=dot(J2(n[label]),x[2:])/(r-1) if label<5 else dot(mul(-1,J2(a[label][2:])),x[:2])/(r-1)
 hinges.append(ell-E(F(3,5)));x=add(x,mul(2*T*weight,Jc(a[label])))
expected=[v/40 for v in [-59+5*u,-9-5*u,-9-5*u,-9-5*u,E(8),-9-5*u,-45+3*u]]
assert hinges==expected
assert b[4]*hinges[4]==E(F(1,125))
S=lambda x:J2(x[2:])+mul(-1,J2(x[:2]))
D=[mul(-E(F(3,5)),v)+mul(1/(r-1),J2(v)) for v in n]+[mul(-1/(r-1),J2(a[i+5][2:]))+mul(-E(F(3,5)),a[i+5][2:]) for i in range(5)]
for i in range(10):assert S(a[i])==a[(i+5)%10] and S(D[i])==D[(i+5)%10]
basis=[[E(int(i==j)) for i in range(4)] for j in range(4)]
for v in basis:
 assert S(S(v))==v
 for w in basis:assert om(S(v),S(w))==om(v,w)
print('PASS: independently reconstructed literal witness closure, normalization, Q=tau/5; full KKT system; all seven hinge identities; response concentrated on label 1 with 1/125; factor-exchange symplectic involution and velocity equivariance.')
