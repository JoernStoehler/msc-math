#!/usr/bin/env python3
"""Exact reduced KKT action branches for three HKO words over Q(tau)(a,b)."""
from sage.all import *
from itertools import combinations
import json,time
import sys
start=time.time()
P=PolynomialRing(QQ,'z'); z=P.gen(); f=z**4-10*z**2+5
tau_real=AA.polynomial_root(f,RIF('0.72','0.73'))
K=NumberField(f,'tau',embedding=tau_real); tau=K.gen(); emb=K.hom([tau_real],AA)
R=PolynomialRing(K,names=('a','b')); a,b=R.gens(); F=R.fraction_field(); a=F(a); b=F(b)
s=(5-tau**2)/2; alpha=(3-s)/2; beta=tau*(1+s)/2
N0=matrix(K,[[1,tau,0,0],[-alpha,beta,0,0],[-(s-1),0,0,0],[-alpha,-beta,0,0],[1,-tau,0,0],[0,0,tau,-1],[0,0,beta,alpha],[0,0,0,s-1],[0,0,-beta,alpha],[0,0,-tau,-1]])
J2=matrix(K,[[0,-1],[1,0]])
T=block_matrix(K,[[zero_matrix(K,2),-J2],[J2,zero_matrix(K,2)]])
Nnew=a*matrix(F,N0)+b*matrix(F,N0*T)
N=[vector(F,N0.row(i)) for i in range(10)]+[vector(F,Nnew.row(i)) for i in range(10)]
J=matrix(K,[[0,0,1,0],[0,0,0,1],[-1,0,0,0],[0,-1,0,0]])
words=[[0,1,6,7,3,12,9],[0,6,5,2,19,11,8,4],[0,4,6,15,2,19,8]]
candidate_a=F(QQ(99)/125); candidate_b=F(QQ(7)/80*(s+1))
def branch(word):
 m=len(word); C=matrix(F,5,m,lambda i,j:N[word[j]][i] if i<4 else F(1))
 rhs=vector(F,[0,0,0,0,1]); piv=None
 for inds in combinations(range(m),5):
  CP=C.matrix_from_columns(inds)
  if CP.det()!=0: piv=inds;break
 assert piv is not None
 free=[j for j in range(m) if j not in piv]; CP=C.matrix_from_columns(piv); CF=C.matrix_from_columns(free)
 U=vector(F,[0]*m); dep=CP.solve_right(rhs)
 for k,j in enumerate(piv):U[j]=dep[k]
 Z=matrix(F,m,len(free))
 for k,j in enumerate(free):Z[j,k]=1
 depZ=-(CP.solve_right(CF))
 for k,j in enumerate(piv):
  for l in range(len(free)):Z[j,l]=depZ[k,l]
 ns=[N[j] for j in word]
 G=matrix(F,m,m,lambda i,j:ns[i]*J*ns[j] if i<j else (ns[j]*J*ns[i] if j<i else 0))
 H=Z.transpose()*G*Z; r=-Z.transpose()*G*U
 assert H.det()!=0
 x=H.solve_right(r); weights=U+Z*x; Q=(weights*G*weights)/2
 assert C*weights==rhs
 wc=[v(a=candidate_a,b=candidate_b) for v in weights]
 qv=Q(a=candidate_a,b=candidate_b)
 assert all(emb(K(v))>0 for v in wc)
 c=(U*G*U)/2
 return {'word':word,'pivot_columns':list(piv),'free_columns':free,'reduced_H':[ [str(H[i,j]) for j in range(H.ncols())] for i in range(H.nrows()) ],'reduced_r':[str(v) for v in r],'constant_c':str(c),'action_formula':'Q = c - (1/2) r^T H^(-1) r','det_H':str(H.det()),'candidate_weights':[str(v) for v in wc],'candidate_weights_float':[float(emb(K(v))) for v in wc],'candidate_Q':str(qv),'candidate_Q_float':float(emb(K(qv)))},Q
out=[]
Qbranches=[]
for w in words:
 o,q=branch(w);out.append(o);Qbranches.append(q); print('branch',w,'done',time.time()-start,flush=True,file=sys.stderr)
curves=[]
near_a=F(QQ(792663)/1000000); near_b=F(QQ(282252)/1000000)
for i,j in combinations(range(len(out)),2):
 diff=Qbranches[i]-Qbranches[j]; spec=diff(a=candidate_a); pp=spec.numerator().univariate_polynomial(); roots=[r for r,m in pp.roots(AA) if r.imag()==0 and AA(QQ(1)/5)<r.real()<AA(QQ(2)/5)]
 diff_b=diff(b=near_b); ppa=diff_b.numerator().univariate_polynomial(); roots_a=[r for r,m in ppa.roots(AA) if r.imag()==0 and AA(QQ(7)/10)<r.real()<AA(QQ(9)/10)]
 near_diff=diff(a=near_a,b=near_b)
 curves.append({'i':i,'j':j,'implicit_crossing_equation':'(c_i det(H_i)-1/2 r_i^T adj(H_i) r_i) det(H_j) - (c_j det(H_j)-1/2 r_j^T adj(H_j) r_j) det(H_i) = 0','candidate_difference':out[i]['candidate_Q_float']-out[j]['candidate_Q_float'],'crossings_at_a_99_over_125_b_in_0.2_0.4':[str(r) for r in roots],'crossings_at_b_0_282252_a_in_0_7_0_9':[str(r) for r in roots_a],'near_optimizer_Q_difference':float(emb(K(near_diff)))})
print(json.dumps({'ring':'Frac(Q(tau)[a,b])','branches':out,'crossings':curves,'near_optimizer_decimal_input':{'a':'0.792663','b':'0.282252'},'seconds':time.time()-start},indent=2))
