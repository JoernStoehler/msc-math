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
t=QQ(3)/10;N=N0.stack(N0+t*D)
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
 return [list(map(int,line.split()))[1:] for line in (out/name).read_text().splitlines()]
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
def feasible_inequalities(rows):
 # beta=b0+Zy lies in simplex, and KKT nullspace projects injectively to beta.
 # Thus a nonempty parameter polytope is compact and has a vertex with d
 # independent active beta_i=0 inequalities. Enumerate all such bases exactly.
 from itertools import combinations
 d=len(rows[0])-1
 if d==0:return all(emb(r[0])>=0 for r in rows)
 for inds in combinations(range(len(rows)),d):
  M=matrix(K,[[rows[i][j+1] for j in range(d)] for i in inds])
  if M.det()==0:continue
  yy=M.solve_right(vector(K,[-rows[i][0] for i in inds]))
  if all(emb(r[0]+vector(K,r[1:])*yy)>=0 for r in rows):return True
 return False
start=time.monotonic()
best=QQ(205)/1019*u;arg=[0,1,6,7,3,12,9];weights=None;counts={'inconsistent':0,'singular':0,'negative':0,'nonnegative':0,'singular_consistent':0,'singular_inconsistent':0,'singular_coarse_excluded':0,'singular_vertex_excluded':0,'singular_feasible':0,'singular_below_target':0};leaves=load('leaves.txt')
for k,word in enumerate(leaves):
 if k%100==0:print('progress',k,'/',len(leaves),'elapsed',round(time.monotonic()-start,2),flush=True)
 C,H=CH(word);n=len(word);e=vector(K,[0,0,0,0,1])
 if C.rank()!=C.augment(e.column()).rank():counts['inconsistent']+=1;continue
 inds=list(C.transpose().pivots());C=C.matrix_from_rows(inds);e=vector(K,[e[i] for i in inds]);r=C.nrows()
 M=block_matrix(K,[[H,C.transpose()],[C,zero_matrix(K,r,r)]])
 rhs=vector(K,[0]*n+list(e));singular=M.det()==0
 if singular:
  counts['singular']+=1
  if M.rank()!=M.augment(rhs.column()).rank():counts['singular_inconsistent']+=1;continue
  counts['singular_consistent']+=1
 sol=M.solve_right(rhs);b=vector(K,sol[:n])
 if not singular and any(emb(v)<0 for v in b):counts['negative']+=1;continue
 if singular:
  ker=M.right_kernel_matrix()
  assert all((H*zz[:n]+C.transpose()*zz[n:]).is_zero() for zz in ker.rows())
  assert all((C*zz[:n]).is_zero() for zz in ker.rows())
  assert all(e*zz[n:]==0 for zz in ker.rows())
  qconst=b*H*b/2
  if emb(qconst-best)<=0:counts['singular_below_target']+=1;continue
  if emb(qconst-(u/5)*(QQ(28)/25)**2)>0:counts['singular_coarse_excluded']+=1;continue
  rows=[[b[i]]+[zz[i] for zz in ker.rows()] for i in range(n)]
  if not feasible_inequalities(rows):counts['singular_vertex_excluded']+=1;continue
  counts['singular_feasible']+=1
 counts['nonnegative']+=1;q=b*H*b/2
 if emb(q-best)>0:best=q;arg=word;weights=list(map(str,b))
 if k%1000==0:print('leaves',k,'/',len(leaves),'elapsed',round(time.monotonic()-start,2),'best',float(emb(best)),flush=True)
summary={'parameter':str(t),'curvature_exclusion_input_count_separately_verified':len(load('exclusions.txt')),'checked_leaf_words':len(leaves),'counts':counts,'best_Q':str(best),'best_Q_numeric':float(emb(best)),'word':arg,'weights':weights,'seconds':time.monotonic()-start}
(out/'exact-refined-leaves-summary.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps(summary,indent=2),flush=True)
