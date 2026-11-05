#!/usr/bin/env python3
"""Exact interval certification and symbolic Lawrence volume for t in [3/10,2/5]."""
from sage.all import *
from itertools import combinations
import json,time
import random
import sys
random.seed(2901)
start=time.time()
P=PolynomialRing(QQ,'z'); z=P.gen(); f=z**4-10*z**2+5
root=AA.polynomial_root(f,RIF('0.72','0.73')); K=NumberField(f,'tau',embedding=root); u=K.gen(); emb=K.hom([root],AA)
T=PolynomialRing(K,'t'); x=T.gen(); F=T.fraction_field(); t=F(x)
s=(5-u**2)/2; a=(3-s)/2; b=u*(1+s)/2
N0=[vector(K,[1,u,0,0]),vector(K,[-a,b,0,0]),vector(K,[-(s-1),0,0,0]),vector(K,[-a,-b,0,0]),vector(K,[1,-u,0,0]),vector(K,[0,0,u,-1]),vector(K,[0,0,b,a]),vector(K,[0,0,0,s-1]),vector(K,[0,0,-b,a]),vector(K,[0,0,-u,-1])]
def d(i,n):
 if i<5:return vector(K,[-3*n[0]/5,-3*n[1]/5,-n[1]/(s-1),n[0]/(s-1)])
 return vector(K,[n[3]/(s-1),-n[2]/(s-1),-3*n[2]/5,-3*n[3]/5])
N=[vector(F,[F(N0[i][j])+t*F(d(i,N0[i])[j]) for j in range(4)]) for i in range(10)]
N+= [vector(F,[F(N0[i][j])+t*F(d(i,N0[i])[j]) for j in range(4)]) for i in range(10)]
# Actually labels 10+i are N0_i + t D_i; preserve labels in order.
N=[vector(F,[F(N0[i][j]) for j in range(4)]) for i in range(10)]+[vector(F,[F(N0[i][j])+t*F(d(i,N0[i])[j]) for j in range(4)]) for i in range(10)]
AP=PolynomialRing(AA,'y'); y=AP.gen(); lo=AA(QQ(3)/10); hi=AA(QQ(2)/5)
def mapped(poly): return AP([emb(c) for c in poly.list()])
def roots_interval(poly):
 if poly==0:return []
 return [r for r,m in mapped(poly).roots(AA) if r.imag()==0 and lo<=r.real()<=hi]
def signat(poly,q): return AA(emb(poly(q))) if poly else AA(0)
qbase=QQ(3)/10
comb=list(combinations(range(20),4)); vertex_sets=[]
Nbase=[]
for i in range(10): Nbase.append(vector(K,N0[i]))
for i in range(10): Nbase.append(vector(K,[N0[i][j]+qbase*d(i,N0[i])[j] for j in range(4)]))
for ids in comb:
 M=matrix(K,[Nbase[i] for i in ids])
 if M.det()==0: continue
 v=M.solve_right(vector(K,[1]*4)); sl=[1-Nbase[i]*v for i in range(20)]
 if any(emb(x)<0 for x in sl): continue
 active=tuple(i for i,x in enumerate(sl) if x==0)
 assert len(active)==4, ('nonsimple feasible base vertex',ids,active)
 vertex_sets.append(active)
vertex_sets=sorted(set(vertex_sets)); assert len(vertex_sets)==75
print('enumerated 75 exact vertices at t=3/10',time.time()-start,flush=True,file=sys.stderr)
# Track precisely these 75 feasible bases. Nonzero determinant and strict
# positivity of every inactive slack on the entire interval preserve each
# simple vertex; boundedness/continuation rules out new vertices (lemma noted
# for independent review in the durable account).
for ids in vertex_sets:
 M=matrix(F,[N[i] for i in ids]); det=M.det()
 assert det!=0 and not roots_interval(det.numerator()), ('rank event',ids)
 v=M.solve_right(vector(F,[1]*4))
 for i,n in enumerate(N):
  if i in ids: continue
  sl=1-n*v
  assert sl!=0 and not roots_interval(sl.numerator()), ('slack event',ids,i)
  value=AA(emb(sl.numerator()(qbase)))/AA(emb(sl.denominator()(qbase)))
  assert value>0, ('not strict',ids,i,value)
print('all 75 bases remain nonsingular with strict inactive slacks on interval',time.time()-start,flush=True,file=sys.stderr)
# Same exact Lawrence formula used by frozen certificate. Rank/slack analysis
# above proves this vertex set is complete throughout the closed interval.
def lawrence(ell):
 total=F(0)
 for ids in vertex_sets:
  M=matrix(F,[N[i] for i in ids]); v=M.solve_right(vector(F,[1]*4)); det=M.det()
  detsign=1 if AA(emb(det(qbase)))>0 else -1
  slopes=ell*M.inverse()
  if not all(k!=0 and not roots_interval(k.numerator()) for k in slopes): return None
  total+=(ell*v)**4/(24*detsign*det*prod(slopes))
 return F(total)
chosen=[]
for e in [[1,2,4,8],[1,3,9,27],[1,7,23,83],[1,10,100,1000],[1,11,37,101]]+[[random.randrange(1,1000) for _ in range(4)] for __ in range(30)]:
 value=lawrence(vector(F,e))
 if value is not None: chosen.append(value)
 if len(chosen)==2: break
assert len(chosen)==2
volume,volume2=chosen
assert volume==volume2
print('assembled and cross-checked Lawrence rational functions',time.time()-start,flush=True,file=sys.stderr)
volume=F(volume)
print('degree numerator/denominator',volume.numerator().degree(),volume.denominator().degree(),time.time()-start,flush=True,file=sys.stderr)
assert not roots_interval(volume.denominator()) and not roots_interval(volume.numerator())
V03=QQ(300748771)/52178240-QQ(99664289)/260891200*u**2
V04=QQ(5954147)/1039680-QQ(1968673)/5198400*u**2
assert volume(qbase)==V03 and volume(QQ(2)/5)==V04
# Exact stationary Q branch for the eight-label witness.
word=[0,6,5,2,19,11,8,4]; ns=[N[i] for i in word]; m=len(word)
J=matrix(K,[[0,0,1,0],[0,0,0,1],[-1,0,0,0],[0,-1,0,0]])
def Qbranch(word):
 ns=[N[i] for i in word]; m=len(word)
 G=matrix(F,m,m,lambda i,j:ns[i]*J*ns[j] if i<j else (ns[j]*J*ns[i] if j<i else 0))
 C=matrix(F,5,m,lambda i,j:ns[j][i] if i<4 else F(1))
 M=block_matrix(F,[[G,C.transpose()],[C,zero_matrix(F,5,5)]])
 w=M.solve_right(vector(F,[0]*m+[0,0,0,0,1]))[:m]
 return (w*G*w)/2
Q8=Qbranch([0,6,5,2,19,11,8,4]); Q7=Qbranch([0,1,6,7,3,12,9])
assert Q8(QQ(2)/5)==QQ(2042071)/10113991*u-QQ(1444)/10113991*u**3
assert Q7(qbase)==QQ(205)/1019*u
for q in [Q7,Q8]:
 assert not roots_interval(q.numerator()) and not roots_interval(q.denominator())
 assert AA(emb(q(qbase)))>0
R8=1/(8*volume*Q8**2); R7=1/(8*volume*Q7**2)
d8=R8.derivative(); d7=R7.derivative(); crit8=roots_interval(d8.numerator()); crit7=roots_interval(d7.numerator())
cross=(Q7-Q8).numerator(); roots_cross=roots_interval(cross)
assert len(crit7)==0 and len(crit8)==0
assert d7(qbase)>0 and d8(QQ(2)/5)<0
assert len(roots_cross)==1
crossing=roots_cross[0] if roots_cross else None
qrat=QQ(17)/50
out={'interval':['3/10','2/5'],'vertex_count':int(len(vertex_sets)),'volume_t':str(volume),'volume_numerator_degree':int(volume.numerator().degree()),'volume_denominator_degree':int(volume.denominator().degree()),'Q7_t':str(Q7),'Q8_t':str(Q8),'rho7_feasible_upper_t':str(R7),'rho8_feasible_upper_t':str(R8),'rho7_derivative_num_roots':[str(r) for r in crit7],'rho8_derivative_num_roots':[str(r) for r in crit8],'branch_Q_crossings':[str(r) for r in roots_cross],'rho7_at_3_10':float(R7(qbase)),'rho7_at_cross':float(R7(crossing)) if crossing else None,'rho8_at_cross':float(R8(crossing)) if crossing else None,'rho7_at_17_50':float(R7(qrat)),'rho8_at_17_50':float(R8(qrat)),'rho8_at_2_5':float(R8(QQ(2)/5)),'derivative7_at_3_10':float(d7(qbase)),'derivative8_at_2_5':float(d8(QQ(2)/5)),'seconds':float(time.time()-start)}
print(json.dumps(out,indent=2))
