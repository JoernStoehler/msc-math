from sage.all import *
from pathlib import Path
import json,time
out=Path(__file__).resolve().parent
R=PolynomialRing(QQ,'z');z=R.gen();poly=z**4-10*z**2+5
root=poly.roots(AA)[2][0];assert 0<root<1 and poly(root)==0;K=NumberField(poly,'u');u=K.gen();emb=K.hom([root],AA)
s=(5-u*u)/2;alpha=(3-s)/2;beta=u*(1+s)/2;N0=matrix(K,[[1,u,0,0],[-alpha,beta,0,0],[-(s-1),0,0,0],[-alpha,-beta,0,0],[1,-u,0,0],[0,0,u,-1],[0,0,beta,alpha],[0,0,0,s-1],[0,0,-beta,alpha],[0,0,-u,-1]])
D=matrix(K,10,4)
for i in range(10):
 n=N0.row(i)
 if i<5:D[i]=vector(K,[-QQ(3)/5*n[0],-QQ(3)/5*n[1],-n[1]/(s-1),n[0]/(s-1)])
 else:D[i]=vector(K,[n[3]/(s-1),-n[2]/(s-1),-QQ(3)/5*n[2],-QQ(3)/5*n[3]])
t=QQ(7)/20;a=QQ(99)/125;body_label='a=99/125,t=7/20';coarse_scale=a+t
assert coarse_scale==QQ(571)/500
N=N0.stack(a*N0+t*(D+QQ(3)/5*N0))
J=matrix(K,[[0,0,1,0],[0,0,0,1],[-1,0,0,0],[0,-1,0,0]]);O=N*J*N.transpose()
# Independently verify every symmetry used by the discovery quotient.
for ref in [0,1]:
 for swap in [0,1]:
  for rot in range(5):
   perm=[]
   for x in range(20):
    block=x//10;fac=x%10;f=fac//5;i=fac%5
    if ref:i=4-i
    i=(i+rot)%5
    if swap:f=1-f
    perm.append(block*10+f*5+i)
   inds=[0,1,5,6];T=N.matrix_from_rows(inds).solve_right(N.matrix_from_rows([perm[i] for i in inds]))
   assert N*T==N.matrix_from_rows(perm)
   assert O.matrix_from_rows_and_columns(perm,perm)==((-1)**ref)*O

def load(name):
 words=[]
 for line in (out/name).read_text().splitlines():
  row=list(map(int,line.split()));assert row[0]==len(row)-1
  assert len(set(row[1:]))==row[0] and all(0<=x<20 for x in row[1:])
  words.append(row[1:])
 assert len(set(map(tuple,words)))==len(words)
 return words
def CH(word):
 n=len(word);C=N.matrix_from_rows(word).transpose().stack(matrix(K,[[1]*n]));H=matrix(K,n,n)
 for i in range(n):
  for j in range(i+1,n):H[i,j]=H[j,i]=O[word[i],word[j]]
 return C,H
# Exact symmetric elimination detects a positive eigenvalue by congruence.
def positive(B):
 while B.nrows():
  n=B.nrows();diag=[emb(B[i,i]) for i in range(n)]
  if any(x>0 for x in diag):return True
  piv=next((i for i in range(n) if diag[i]<0),None)
  if piv is None:return any(B[i,j]!=0 for i in range(n) for j in range(i))
  rest=[i for i in range(n) if i!=piv];v=vector(K,[B[i,piv] for i in rest])
  B=B.matrix_from_rows_and_columns(rest,rest)-v.column()*v.row()/B[piv,piv]
 return False
