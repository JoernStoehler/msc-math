import itertools,json,sys
from fractions import Fraction as F
from pathlib import Path
import numpy as np
from scipy.spatial import ConvexHull
root=Path('/workspaces/msc-math-viterbo-continuation')
cert=root/'topics/capacity-and-qp/convex-systolic-optimization/certificates/hko-ascent'
sys.path.insert(0,str(cert)); import exact_core as E
# Exact row transformation at the candidate input.
a=E.K(F(99,125)); b=7*(E.SQRT5+1)/80
T=[[E.ZERO,E.ZERO,E.ZERO,E.ONE],[E.ZERO,E.ZERO,-E.ONE,E.ZERO],[E.ZERO,-E.ONE,E.ZERO,E.ZERO],[E.ONE,E.ZERO,E.ZERO,E.ZERO]]
N0=E.A[:]
Nnew=[]
for n in N0:
 nT=[sum((n[k]*T[k][j] for k in range(4)),E.ZERO) for j in range(4)]
 Nnew.append(E.add(E.scale(a,n),E.scale(b,nT)))
N=N0+Nnew
word=[0,6,5,2,19,11,8,4];m=len(word)
O=[[E.omega(N[i],N[j]) for j in range(20)] for i in range(20)]
H=[[O[word[i]][word[j]] if i<j else O[word[j]][word[i]] if j<i else E.ZERO for j in range(m)] for i in range(m)]
C=[[N[word[j]][i] for j in range(m)] for i in range(4)]+[[E.ONE]*m]
K=[H[i]+[C[r][i] for r in range(5)] for i in range(m)] + [C[r]+[E.ZERO]*5 for r in range(5)]
rhs=[E.ZERO]*(m+4)+[E.ONE]
# Exact RREF for a potentially singular KKT system, setting free variables to zero.
Aug=[list(row)+[rhs[i]] for i,row in enumerate(K)]; piv=[];rr=0
for col in range(len(K[0])):
 cand=next((i for i in range(rr,len(Aug)) if Aug[i][col]),None)
 if cand is None:continue
 Aug[rr],Aug[cand]=Aug[cand],Aug[rr];d=Aug[rr][col];Aug[rr]=[x/d for x in Aug[rr]]
 for i in range(len(Aug)):
  if i!=rr and Aug[i][col]:
   d=Aug[i][col];Aug[i]=[x-d*y for x,y in zip(Aug[i],Aug[rr])]
 piv.append(col);rr+=1
 if rr==len(Aug):break
print('inconsistency',[(i,[x.coefficients() for x in row[-2:]]) for i,row in enumerate(Aug) if not any(row[:-1]) and row[-1]!=E.ZERO]);
print('rank',len(piv),'freecols',[i for i in range(len(K[0])) if i not in piv])
for i,col in enumerate(piv):
 print('pivot',col,'base',Aug[i][-1].coefficients(),'freecoef',Aug[i][11].coefficients())
sol=[E.ZERO]*len(K[0])
for i,col in enumerate(piv):sol[col]=Aug[i][-1]
w=sol[:m]
q=E.qeval(N,word,w)
closure=[sum((N[word[j]][i]*w[j] for j in range(m)),E.ZERO) for i in range(4)]
print('exact word feasibility',all(E.sign(x)>0 for x in w),[x.coefficients() for x in w])
print('closure', [x.coefficients() for x in closure], 'sum',sum(w,E.ZERO).coefficients())
print('Q coefficients',q.coefficients(),'nominal',E.nominal(q),'KKT residual exact',all(sum((K[i][j]*sol[j] for j in range(len(sol))),E.ZERO)==rhs[i] for i in range(len(rhs))))
# Independent NumPy reconstruction of all row intersections and Qhull volume.
tau=np.tan(np.pi/5);s=np.sqrt(5);al=(3-s)/2;be=tau*(1+s)/2
N0=np.array([[1,tau,0,0],[-al,be,0,0],[-(s-1),0,0,0],[-al,-be,0,0],[1,-tau,0,0],[0,0,tau,-1],[0,0,be,al],[0,0,0,s-1],[0,0,-be,al],[0,0,-tau,-1]],float)
J2=np.array([[0,-1],[1,0.]])
Tn=np.block([[np.zeros((2,2)),-J2],[J2,np.zeros((2,2))]])
a0=99/125;b0=7/20*np.cos(np.pi/5);Nf=np.vstack([N0,a0*N0+b0*(N0@Tn)])
verts=[]
for ids in itertools.combinations(range(20),4):
 M=Nf[list(ids)]
 if abs(np.linalg.det(M))<1e-10:continue
 x=np.linalg.solve(M,np.ones(4))
 if np.max(Nf@x)<=1+1e-8:verts.append(x)
V=np.unique(np.round(verts,10),axis=0)
print('geometry',len(V),ConvexHull(V).volume,'a,b',a0,b0)
print('facet rows with a numerical tight vertex',sum(np.min(np.abs(V@Nf[i]-1))<1e-7 for i in range(20)))
