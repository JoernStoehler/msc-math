# /// script
# dependencies = ["numpy", "scipy"]
# ///
"""Bounded sign-ascent LP probe for the matched-projection slope inequality.
Numerical diagnostics only; a candidate violation requires exact replay.
"""
import json,time
from pathlib import Path
from itertools import combinations
import numpy as np
from scipy.optimize import linprog
rng=np.random.default_rng(291026)
started=time.monotonic();records=[]
for case in range(12):
 n=8 if case<6 else 10
 g=np.ones(n) if case%3==0 else rng.integers(1,6,size=n).astype(float)
 h=(np.arange(n) if case%3==0 else rng.integers(-7,8,size=n)).astype(float)
 C=np.outer(g,h)-np.outer(h,g); cn=sum(abs(C[i,j]) for i,j in combinations(range(n),2))
 if not cn:continue
 edges=list(combinations(range(n),2));ei={e:i for i,e in enumerate(edges)};N=len(edges)
 def entry(row,i,j,a):
  if i==j:return
  row[ei[(min(i,j),max(i,j))]] += a*(1 if i<j else -1)
 constraints=[]
 for i,j,k,l in combinations(range(n),4):
  row=np.zeros(N)
  for a,b,c in [(i,j,C[k,l]),(k,l,C[i,j]),(i,k,-C[j,l]),(j,l,-C[i,k]),(i,l,C[j,k]),(j,k,C[i,l])]:entry(row,a,b,c)
  constraints.append(row)
 T=[]
 for l in range(n):
  for i,j,k in combinations(range(n),3):
   row=np.zeros(N);entry(row,i,j,C[k,l]);entry(row,i,k,-C[j,l]);entry(row,j,k,C[i,l]);T.append(row)
 T=np.array(T);E=np.array(constraints)
 aub=np.vstack([np.c_[np.eye(N),-np.eye(N)],np.c_[-np.eye(N),-np.eye(N)],np.r_[np.zeros(N),np.ones(N)]])
 bub=np.r_[np.zeros(2*N),1.]
 aeq=np.c_[E,np.zeros(E.shape)];beq=np.zeros(len(E))
 best=0.;bestw=None
 for start in range(8):
  r=rng.normal(size=n);s=rng.normal(size=n)
  W=np.outer(g,r)-np.outer(r,g)+np.outer(h,s)-np.outer(s,h)
  w=np.array([W[i,j] for i,j in edges]);w/=sum(abs(w))
  for iteration in range(12):
   sign=np.sign(T@w);objective=T.T@sign
   res=linprog(np.r_[-objective,np.zeros(N)],A_ub=aub,b_ub=bub,A_eq=aeq,b_eq=beq,bounds=[(None,None)]*N+[(0,None)]*N,method='highs')
   assert res.success,res.message
   old=np.sum(np.abs(T@w));w=res.x[:N];val=np.sum(np.abs(T@w))
   if val>best:best=val;bestw=w.copy()
   if val-old<1e-9:break
  if best>cn*(1+1e-8):break
 ratio=best/cn
 print('case',case,'n',n,'ratio',ratio,'seconds',time.monotonic()-started,flush=True)
 records.append({'case':case,'n':n,'g':g.tolist(),'h':h.tolist(),'ratio':ratio,'edges':edges,'W':bestw.tolist()})
 if ratio>1+1e-8:
  print('CANDIDATE VIOLATION; exact check needed',flush=True);break
 if time.monotonic()-started>60:break
Path('/tmp/msc-math/viterbo-continuation-20261029/rank-four/reduction/slope-lp-results.json').write_text(json.dumps(records,indent=2)+'\n')
