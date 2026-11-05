#!/usr/bin/env python3
"""Explicit nine-parameter product gauge; generic float/Fraction/Q(sqrt5) arithmetic.
The reference triangle consists of primal vertices 0,2,4 (not three adjacent
vertices): its interior contains zero, ensuring every affine image polygon
contains zero. No capacity target or optimization occurs in these functions.
"""
import math
from functools import cmp_to_key

def dot(a,b):return sum(x*y for x,y in zip(a,b))
def det(a,b):return a[0]*b[1]-a[1]*b[0]
def mat(A,x):return [dot(row,x) for row in A]
def transpose(A):return [list(x) for x in zip(*A)]
def inverse(A):
 c=det(A[0],A[1]);return [[A[1][1]/c,-A[0][1]/c],[-A[1][0]/c,A[0][0]/c]]
def mm(A,B):return [[dot(a,b) for b in transpose(B)] for a in A]
def polygon(a):
 out=[]
 for i,x in enumerate(a):
  y=a[(i+1)%len(a)];c=det(x,y);out.append([(y[1]-x[1])/c,(x[0]-y[0])/c])
 return out

def centroid(a):
 vv=polygon(a);cs=[det(vv[i],vv[(i+1)%len(vv)]) for i in range(len(vv))];den=3*sum(cs);return [sum((vv[i][c]+vv[(i+1)%len(vv)][c])*cs[i] for i in range(len(vv)))/den for c in range(2)]

def canonical(a,b,reference_A,reference_B):
 """Fix A's labelled vertex triangle, center B and fix B normal0 projection.
 Returns None only when the final labelled Bnormal projection has wrongsign.
 """
 assert len(a)==len(b)==5;pv=polygon(a);rv=polygon(reference_A)
 E=[[pv[j][c]-pv[0][c] for j in [2,4]] for c in range(2)];R=[[rv[j][c]-rv[0][c] for j in [2,4]] for c in range(2)];L=mm(R,inverse(E));Ltinv=transpose(inverse(L));lp=mat(L,pv[0]);t=[rv[0][c]-lp[c] for c in range(2)];aa=[]
 for x in a:
  u=mat(Ltinv,x);den=1+dot(u,t);assert den>0;aa.append([v/den for v in u])
 bb=[mat(L,x) for x in b];c=centroid(bb);bc=[]
 for x in bb:
  den=1-dot(x,c);assert den>0;bc.append([v/den for v in x])
 scale=bc[0][1]/reference_B[0][1]
 if scale<=0:return None
 bb=[[x/scale for x in u] for u in bc];return aa,bb

def angular(a):
 def cmp(u,v):
  hu=int(u[1]<0 or (u[1]==0 and u[0]<0));hv=int(v[1]<0 or (v[1]==0 and v[0]<0))
  if hu!=hv:return hu-hv
  c=det(u,v);return (-1 if c>0 else 1) if c else 0
 return sorted(a,key=cmp_to_key(cmp))

def candidates(a,b,reference_A,reference_B):
 """Unlabelled candidate set: 2roles x 2commonorientations x 5A x 5B starts.
 Normals are first sorted CCW, making the set independent of input row order.
 """
 a,b=angular(a),angular(b)
 for swapped in [False,True]:
  x,y=(b,a) if swapped else(a,b)
  for reverse in [False,True]:
   u,v=(x[::-1],y[::-1]) if reverse else(x,y)
   for ia in range(5):
    for ib in range(5):
     result=canonical(u[ia:]+u[:ia],v[ib:]+v[:ib],reference_A,reference_B)
     if result is not None:yield dict(swapped=swapped,reverse=reverse,A_start=ia,B_start=ib),result

def defect(aa,bb,reference_A,reference_B,gradients):
 z=[x for rows in [aa,bb] for u in rows for x in u];z0=[x for rows in [reference_A,reference_B] for u in rows for x in u];delta=[x-y for x,y in zip(z,z0)];return max(dot(g,delta) for g in gradients)

def invariant(a,b,reference_A,reference_B,gradients):
 best=None;count=0
 for labels,(aa,bb) in candidates(a,b,reference_A,reference_B):
  value=defect(aa,bb,reference_A,reference_B,gradients);count+=1
  if best is None or value<best['defect']:best=dict(defect=value,labels=labels,A=aa,B=bb)
 assert best is not None;best['valid_candidates']=count;return best
