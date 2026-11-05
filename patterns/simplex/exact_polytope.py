from fractions import Fraction as F
from itertools import combinations, permutations
from time import monotonic
import json
pairs=list(combinations(range(4),2))
def Bof(x):
 B=[[0]*5 for _ in range(5)]
 for (i,j),t in zip(pairs,x): B[i][j]=t; B[j][i]=-t
 for i in range(4): B[i][4]=-sum(B[i]); B[4][i]=-B[i][4]
 return B
perms=[(0,)+p for p in permutations(range(1,5))]
def ordersums(x):
 B=Bof(x)
 return [sum(B[p[i]][p[j]] for i in range(5) for j in range(i+1,5)) for p in perms]
cols=[ordersums([int(i==j) for i in range(6)]) for j in range(6)]
A=[list(row) for row in zip(*cols)]
def solve(M):
 M=[list(row)+[1] for row in M]; prev=1
 for k in range(6):
  p=next((i for i in range(k,6) if M[i][k]),None)
  if p is None:return None
  M[k],M[p]=M[p],M[k]
  pivot=M[k][k]
  for i in range(k+1,6):
   for j in range(k+1,7):
    num=M[i][j]*pivot-M[i][k]*M[k][j]
    assert num%prev==0
    M[i][j]=num//prev
   M[i][k]=0
  prev=pivot
 x=[F(0)]*6
 for k in range(5,-1,-1):x[k]=(F(M[k][6])-sum(M[k][j]*x[j] for j in range(k+1,6)))/M[k][k]
 return tuple(x)
def pf(x):return x[0]*x[5]-x[1]*x[4]+x[2]*x[3]
t0=monotonic(); V=set(); fullrank=0
for inds in combinations(range(24),6):
 x=solve([A[i] for i in inds])
 if x is None:continue
 fullrank+=1
 if x in V:continue
 if all(sum(a*t for a,t in zip(row,x))<=1 for row in A):V.add(x)
V=sorted(V)
T=set()
for a,b,c in combinations(range(5),3):
 B=[[0]*5 for _ in range(5)]
 for i,j in [(a,b),(b,c),(c,a)]: B[i][j]=1;B[j][i]=-1
 v=tuple(F(B[i][j]) for i,j in pairs);T.add(v);T.add(tuple(-t for t in v))
assert set(V)==T
assert all(isinstance(t,F) for v in V for t in v)
assert {tuple(-t for t in row) for row in A}==set(map(tuple,A))
assert fullrank>0
assert all(pf(v)==0 for v in V)
W=[[pf(tuple(a+b for a,b in zip(v,w))) for w in V] for v in V]
assert {x for row in W for x in row}=={-1,0,1}
tri={str(s):sum(all(W[a][b]==s for a,b in [(i,j),(i,k),(j,k)]) for i,j,k in combinations(range(20),3)) for s in [-1,1]}
assert tri=={'-1':0,'1':0}
out={'all_active_bases':134596,'independent_active_bases':fullrank,'vertices':len(V),'triangle_circuit_equality':True,'positive_graph_triangles':tri['1'],'negative_graph_triangles':tri['-1'],'seconds':monotonic()-t0,'V':[[int(t) for t in v] for v in V], 'W':[[int(t) for t in row] for row in W], 'A':A}
print(json.dumps(out,indent=2))
