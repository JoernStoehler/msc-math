#!/usr/bin/env sage -python
from sage.all import *
R = PolynomialRing(QQ, "x")
x = R.gen()
f=x**4-10*x**2+5
F=NumberField(f,'t',embedding=AA.polynomial_root(f,RIF(0,1)))
t=F.gen(); s=(5-t**2)/2; al=(3-s)/2; be=t*(1+s)/2; d=s-1
a=[vector(F,v) for v in [(1,t,0,0),(-al,be,0,0),(-d,0,0,0),(-al,-be,0,0),(1,-t,0,0),(0,0,t,-1),(0,0,be,al),(0,0,0,d),(0,0,-be,al),(0,0,-t,-1)]]
def om(u,v): return u[0]*v[2]+u[1]*v[3]-u[2]*v[0]-u[3]*v[1]
def C(w): return matrix(F,[[a[i][j] for i in w] for j in range(4)]+[[F(1)]*len(w)])
def q(w,b): return sum(b[i]*b[j]*om(a[w[j]],a[w[i]]) for i in range(1,len(w)) for j in range(i))
# (one-based word, solved positions I, fixed positions J, exact fixed values)
rows=[
([1,2,8,4,10,6],[1,2,3,4,5],[6],['t^2/40+1/8']),
([1,2,8,7,4,10],[1,2,3,4,5],[6],['-t^2/20+1/4']),
([2,3,8,9,5,4,6],[1,2,3,4,5],[6,7],['6909830056250513/10^17','-t^2/20+1/4']),
([1,8,7,3,4,10],[1,2,3,4,5],[6],['-t^2/20+1/4']),
([2,9,8,5,4,6],[1,2,3,4,5],[6],['-t^2/20+1/4']),
([2,8,4,5,6,10],[1,2,3,4,5],[6],['t^2/40+1/8']),
([1,6,2,8,5,4,10],[1,2,3,4,5],[6,7],['36180339887499/(2*10^14)','t^2/40+1/8']),
([1,7,8,4,3,9,10],[1,2,3,4,5],[6,7],['(8090169943749473*t^2+23368810393753689)/(4*10^17)','18090169943749473/10^17']),
([1,5,7,6,3,10,9],[1,2,3,4,5],[6,7],['(4045084971874731*t^2+11684405196876883)/(2*10^17)','9045084971874731/(5*10^16)']),
([2,8,4,3,9,5,6],[1,2,3,4,5],[6,7],['1809016994374947/10^16','-t^2/20+1/4']),
([2,3,9,8,4,5,6],[1,2,3,4,5],[6,7],['2261271242968689/(125*10^14)','-t^2/20+1/4']),
([2,8,9,5,7,6,3],[1,2,3,4,5],[6,7],['18090169943749473/10^17','t^2/40+1/8']),
([1,7,8,4,10,2],[1,2,3,4,5],[6],['t^2/40+1/8']),
([1,7,4,3,9,10],[1,2,3,4,5],[6],['t^2/40+1/8']),
([2,9,5,6,7,3],[1,2,3,4,5],[6],['t^2/40+1/8']),
([2,8,3,9,5,6],[1,2,3,4,5],[6],['-t^2/20+1/4']),
([2,8,4,5,10,6],[1,2,3,4,5],[6],['t^2/40+1/8']),
([1,7,3,2,8,4,10],[1,2,3,4,5],[6,7],['18090169943749507/10^17','-t^2/20+1/4']),
([1,5,6,7,3,2,9],[1,2,3,4,5],[6,7],['6909830056250547/10^17','-t^2/20+1/4']),
([1,7,3,9,5,6,2],[1,2,3,4,5],[6,7],['t^2/40+1/8','863728757031319/(125*10^14)']),
([1,8,5,4,6,10,2],[1,2,3,4,5],[6,7],['t^2/40+1/8','7463176480767021/(5*10^16)']),
([1,6,2,8,7,4,10],[1,2,3,4,5],[6,7],['-t^2/20+1/4','1809016994374947/10^16']),
([1,5,7,4,3,9,10],[1,2,3,4,6],[5,7],['9045084971874741/(5*10^16)','t^2/40+1/8']),
([1,7,3,4,10,9,5],[1,2,3,4,5],[6,7],['t^2/40+1/8','3454915028125259/(5*10^16)']),
([1,7,6,3,10,9,5],[1,2,3,4,5],[6,7],['452254248593737/(25*10^14)','t^2/40+1/8']),
([2,8,9,5,4,6,3],[1,2,3,4,5],[6,7],['-t^2/20+1/4','12028993467659169/10^17'])]
def val(z): return F(sage_eval(z, locals={'t': t}))
def section(W):
 w,I,J,u=W; w=[i-1 for i in w]; I=[i-1 for i in I]; J=[i-1 for i in J]
 assert len(set(w))==len(w) and all(0<=i<10 for i in w); assert sorted(I+J)==list(range(len(w))); cm=C(w); assert cm[:,I].det()!=0; U=vector(F,[val(z) for z in u]); b=[None]*len(w)
 x=cm[:,I].solve_right(vector(F,[0,0,0,0,1])-cm[:,J]*U)
 for k,i in enumerate(I): b[i]=x[k]
 for k,j in enumerate(J): b[j]=U[k]
 b=vector(F,b); assert cm*b==vector(F,[0,0,0,0,1]) and all(z>0 for z in b)
 return w,I,J,b
V=QQ(25)*(5+s)/QQ(32); A=5*t-t**3/QQ(2); qm=1/(2*A)
vr=vector(F,sum((list(-(QQ(25)/QQ(32)+QQ(5)*s/QQ(16))*v) for v in a),[]))
def drow(w,I,J,b):
 cm=C(w); db=[[F(0)]*40 for _ in w]
 for z in range(40):
  facet,coord=divmod(z,4); rhs=vector(F,[0]*5)
  if facet in w: rhs[coord]=-b[w.index(facet)]
  xi=cm[:,I].solve_right(rhs); assert cm[:,I]*xi==rhs
  for k,i in enumerate(I): db[i][z]=xi[k]
 for j in J: assert all(x==0 for x in db[j])
 dq=[]
 for z in range(40):
  facet,coord=divmod(z,4); u=vector(F,[1 if k==coord else 0 for k in range(4)]); x=F(0)
  for i in range(1,len(w)):
   for j in range(i):
    x+=(db[i][z]*b[j]+b[i]*db[j][z])*om(a[w[j]],a[w[i]])
    if w[j]==facet: x+=b[i]*b[j]*om(u,a[w[i]])
    if w[i]==facet: x+=b[i]*b[j]*om(a[w[j]],u)
  dq.append(x)
 return vector(F,[A/V*(-x/(2*qm**2))-A**2/(2*V**2)*vr[i] for i,x in enumerate(dq)])
# Build the fifteen symmetry columns, then verify all rows, rank, and kernel.
Jm=matrix(F,[[0,0,-1,0],[0,0,0,-1],[1,0,0,0],[0,1,0,0]])
T=[vector(F,sum((list(-v[k]*v) for v in a),[])) for k in range(4)]
T += [vector(F,sum((list(-v) for v in a),[]))]
G=[matrix(F,g) for g in [
[[1,0,0,0],[0,0,0,0],[0,0,-1,0],[0,0,0,0]],[[0,1,0,0],[0,0,0,0],[0,0,0,0],[0,0,-1,0]],
[[0,0,0,0],[1,0,0,0],[0,0,0,-1],[0,0,0,0]],[[0,0,0,0],[0,1,0,0],[0,0,0,0],[0,0,0,-1]],
[[0,0,1,0],[0,0,0,0],[0,0,0,0],[0,0,0,0]],[[0,0,0,1],[0,0,1,0],[0,0,0,0],[0,0,0,0]],
[[0,0,0,0],[0,0,0,1],[0,0,0,0],[0,0,0,0]],[[0,0,0,0],[0,0,0,0],[1,0,0,0],[0,0,0,0]],
[[0,0,0,0],[0,0,0,0],[0,1,0,0],[1,0,0,0]],[[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,1,0,0]]]]
for X in G:
 assert X.transpose()*Jm+Jm*X==0
 T.append(vector(F,sum((list(-X.transpose()*v) for v in a),[])))
rr=[]
assert len(rows)==26
for W in rows:
 w,I,J,b=section(W); assert q(w,b)==qm; rr.append(drow(w,I,J,b))
M=matrix(F,[list(r) for r in rr]); assert M.rank()==25 and matrix(F,[list(c) for c in T]).rank()==15
for r in rr:
 for c in T: assert r.dot_product(c)==0
kernel=M.transpose().right_kernel().basis(); assert len(kernel)==1
lam=kernel[0]
if all(x<0 for x in lam): lam=-lam
assert all(x>0 for x in lam); lam=lam/sum(lam); assert lam*M==0
print('HKO appendix core passed: rows=26 rank=25 symmetry-rank=15')
