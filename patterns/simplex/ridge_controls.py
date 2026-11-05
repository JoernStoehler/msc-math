"""Exact equal-ridge/different-ratio simplex controls; standard library only."""
from fractions import Fraction as F
from itertools import combinations,permutations
import json

def omega(u,v):return sum(u[i]*v[i+2]-u[i+2]*v[i] for i in range(2))
def solve(A,b):
 A=[[F(t) for t in row]+[F(rhs)] for row,rhs in zip(A,b)]
 for k in range(len(b)):
  p=next(i for i in range(k,len(b)) if A[i][k]);A[k],A[p]=A[p],A[k];t=A[k][k];A[k]=[x/t for x in A[k]]
  for i in range(len(b)):
   if i!=k:
    t=A[i][k];A[i]=[x-t*y for x,y in zip(A[i],A[k])]
 return [row[-1] for row in A]
def determinant(A):
 n=len(A);ans=F(0)
 for p in permutations(range(n)):
  sign=(-1)**sum(p[i]>p[j] for i,j in combinations(range(n),2));term=F(sign)
  for i in range(n):term*=A[i][p[i]]
  ans+=term
 return ans

def control(t):
 g=[[F(1),F(0),F(0),F(0)],[F(0),F(0),F(-2),F(0)],[F(-1),F(1),F(-2),F(0)],[F(t),F(0),F(-2),F(-4)]]
 g.append([-sum(row[j] for row in g) for j in range(4)]);a=[[5*z for z in row] for row in g]
 vertices=[solve([row for j,row in enumerate(a) if j!=i],[1]*4) for i in range(5)]
 assert all(sum(x*y for x,y in zip(row,v))<=1 for row in a for v in vertices)
 assert all(sum(x*y for x,y in zip(a[i],vertices[i]))<1 for i in range(5))
 edges=[[vertices[i][j]-vertices[0][j] for j in range(4)] for i in range(1,5)];V=abs(determinant(edges))/24
 B=[[omega(u,v) for v in g] for u in g];assert all(sum(row)==0 for row in B)
 Q=max(sum(B[p[i]][p[j]] for i,j in combinations(range(5),2)) for tail in permutations(range(1,5)) for p in [(0,)+tail]);c=1/(2*Q);sys=c*c/(2*V)
 ridge_sum=F(0)
 for i,j in combinations(range(5),2):
  vs=[v for k,v in enumerate(vertices) if k not in [i,j]]
  u=[x-y for x,y in zip(vs[1],vs[0])];v=[x-y for x,y in zip(vs[2],vs[0])];area=abs(omega(u,v))/2
  assert area==12*V*abs(B[i][j]);ridge_sum+=area
 return dict(dual_vertices=[[str(z) for z in row] for row in a],volume=str(V),Q=str(Q),capacity=str(c),sys=str(sys),ridge_sum=str(ridge_sum),R_squared=str(ridge_sum*ridge_sum/V))

def verify():
 A=control(F(0));B=control(F(1,2))
 assert A['volume']==B['volume']=='1/192'
 assert A['R_squared']==B['R_squared']=='363'
 assert A['sys']=='6/25' and B['sys']=='3/8'
 return dict(status='PASS',same_volume='1/192',same_R_squared='363',controls=[A,B],assumptions=['HK formula; unique strictly positive simplex closure'])
if __name__=='__main__':print(json.dumps(verify(),indent=2))
