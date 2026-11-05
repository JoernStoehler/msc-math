#!/usr/bin/env python3
"""Exact KKT branches Q_w(t), weights, and sampled ratio bounds."""
from sage.all import *
import json

P=PolynomialRing(QQ,'z'); z=P.gen(); f=z**4-10*z**2+5
root=AA.polynomial_root(f,RIF('0.72','0.73'))
K=NumberField(f,'tau',embedding=root); tau=K.gen()
emb=K.hom([root],AA)
T=PolynomialRing(K,'t'); t=T.gen(); F=T.fraction_field(); t=F(t)
s=(5-tau**2)/2; a=(3-s)/2; b=tau*(1+s)/2
N0=[vector(K,[1,tau,0,0]),vector(K,[-a,b,0,0]),vector(K,[-(s-1),0,0,0]),vector(K,[-a,-b,0,0]),vector(K,[1,-tau,0,0]),vector(K,[0,0,tau,-1]),vector(K,[0,0,b,a]),vector(K,[0,0,0,s-1]),vector(K,[0,0,-b,a]),vector(K,[0,0,-tau,-1])]
def d(i,n):
 if i<5:return vector(K,[-3*n[0]/5,-3*n[1]/5,-n[1]/(s-1),n[0]/(s-1)])
 return vector(K,[n[3]/(s-1),-n[2]/(s-1),-3*n[2]/5,-3*n[3]/5])
N=N0+[vector(F,[F(N0[i][j])+t*F(d(i,N0[i])[j]) for j in range(4)]) for i in range(10)]
J=matrix(K,[[0,0,1,0],[0,0,0,1],[-1,0,0,0],[0,-1,0,0]])
def branch(word):
 m=len(word); ns=[N[i] for i in word]
 G=matrix(F,m,m,lambda i,j:ns[i]*J*ns[j] if i<j else (ns[j]*J*ns[i] if j<i else 0))
 C=matrix(F,5,m,lambda i,j:F(ns[j][i]) if i<4 else F(1))
 M=block_matrix(F,[[G,C.transpose()],[C,zero_matrix(F,5,5)]])
 sol=M.solve_right(vector(F,[0]*m+[0,0,0,0,1])); w=sol[:m]; Q=(w*G*w)/2
 assert C*w==vector(F,[0,0,0,0,1])
 return w,Q
words=[[0,1,6,7,3,12,9],[0,6,5,2,19,11,8,4]]
out=[]
for w in words:
 weights,Q=branch(w)
 out.append({'word':w,'weights':[str(x) for x in weights],'Q':str(Q),'Q_num':str(Q.numerator()),'Q_den':str(Q.denominator())})
 # Isolate every real weight zero and KKT pole in the unit parameter interval.
 AP=PolynomialRing(AA,'x'); x=AP.gen()
 def aapoly(poly): return AP([emb(c) for c in poly.list()])
 def roots01(poly):
  return sorted([float(r.real()) for r,m in aapoly(poly).roots(AA) if abs(float(r.imag()))<1e-12 and 0<float(r.real())<1])
 out[-1]['weight_zero_roots_in_0_1']=[roots01(r.numerator()) for r in weights]
 out[-1]['weight_pole_roots_in_0_1']=[roots01(r.denominator()) for r in weights]
Q7=branch(words[0])[1]; Q8=branch(words[1])[1]
cross=(Q7-Q8).numerator()
out.append({'comparison':'Q7=Q8','difference_numerator':str(cross),'crossing_roots_in_0_1':roots01(cross)})
print(json.dumps(out,indent=2))
