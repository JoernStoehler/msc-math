from common import *
from itertools import combinations
import hashlib

def feasible_weights(b,ker):
 # Projection of KKT kernel to beta is injective, since C has full row rank.
 # The beta simplex is compact; its parameter preimage is bounded. A nonempty
 # bounded polyhedron has a vertex with d independent beta_i=0 constraints.
 n=len(b);d=ker.nrows();Z=ker.matrix_from_columns(range(n)).transpose()
 assert Z.rank()==d
 if d==0:return b if all(emb(v)>=0 for v in b) else None
 for inds in combinations(range(n),d):
  B=Z.matrix_from_rows(inds)
  if B.det()==0:continue
  yy=B.solve_right(vector(K,[-b[i] for i in inds]));bb=b+Z*yy
  if all(emb(v)>=0 for v in bb):return bb
 return None

start=time.monotonic();att=json.loads((out/'exact-attainer.json').read_text())
best=K(att['Q']);arg=att['word'];weights=att['positive_exact_weights']
C,H=CH(arg);b=vector(K,list(map(K,weights)));assert C*b==vector(K,[0,0,0,0,1]) and all(emb(x)>0 for x in b) and b*H*b/2==best
geo=json.loads((out/'exact-geometry-summary.json').read_text())
coarse=1+2*t/5
assert K(geo['coarse_scale'])==coarse
counts={key:0 for key in ['inconsistent','singular','negative','nonnegative','singular_consistent','singular_inconsistent','singular_coarse_excluded','singular_vertex_excluded','singular_feasible','singular_below_target']};leaves=load('leaves.txt');records=[]
for k,word in enumerate(leaves):
 if k%100==0:print('progress',k,'/',len(leaves),'elapsed',round(time.monotonic()-start,2),flush=True)
 rec={'word':word};records.append(rec)
 C,H=CH(word);n=len(word);e=vector(K,[0,0,0,0,1])
 if C.rank()!=C.augment(e.column()).rank():counts['inconsistent']+=1;rec['outcome']='constraint_inconsistent';continue
 inds=list(C.transpose().pivots());C=C.matrix_from_rows(inds);e=vector(K,[e[i] for i in inds]);r=C.nrows()
 M=block_matrix(K,[[H,C.transpose()],[C,zero_matrix(K,r,r)]])
 rhs=vector(K,[0]*n+list(e));singular=M.det()==0;rec['singular']=singular
 if singular:
  counts['singular']+=1
  if M.rank()!=M.augment(rhs.column()).rank():counts['singular_inconsistent']+=1;rec['outcome']='KKT_inconsistent';continue
  counts['singular_consistent']+=1
 sol=M.solve_right(rhs);b=vector(K,sol[:n]);q=b*H*b/2;rec['Q']=str(q)
 if not singular and any(emb(v)<0 for v in b):counts['negative']+=1;rec['outcome']='negative_unique_stationary';continue
 if singular:
  ker=M.right_kernel_matrix()
  assert all((H*zz[:n]+C.transpose()*zz[n:]).is_zero() for zz in ker.rows())
  assert all((C*zz[:n]).is_zero() for zz in ker.rows())
  assert all(e*zz[n:]==0 for zz in ker.rows())
  assert q==-e*sol[n:]/2
  if emb(q-best)<=0:counts['singular_below_target']+=1;rec['outcome']='constant_below_attained_target';continue
  if emb(q-(u/5)*coarse**2)>0:counts['singular_coarse_excluded']+=1;rec['outcome']='constant_above_inclusion_bound';continue
  bb=feasible_weights(b,ker)
  if bb is None:counts['singular_vertex_excluded']+=1;rec['outcome']='exact_parameter_polytope_empty';continue
  b=bb;assert C*b==e and b*H*b/2==q
  counts['singular_feasible']+=1
 counts['nonnegative']+=1;rec['outcome']='feasible_stationary';assert all(emb(x)>=0 for x in b)
 if emb(q-best)>0:best=q;arg=word;weights=list(map(str,b))
summary={'parameter':str(t),'curvature_exclusion_input_count_separately_verified':len(load('exclusions.txt')),'checked_leaf_words':len(leaves),'counts':counts,'best_Q':str(best),'best_Q_numeric':float(emb(best)),'word':arg,'nonnegative_weights':weights,'coarse_Q_bound':str((u/5)*coarse**2),'input_sha256':{f:hashlib.sha256((out/f).read_bytes()).hexdigest() for f in ['leaves.txt','exclusions.txt','exact-attainer.json','exact-geometry-summary.json']},'seconds':time.monotonic()-start}
(out/'exact-leaves-summary.json').write_text(json.dumps(summary,indent=2)+'\n')
(out/'exact-leaves-detail.json').write_text(json.dumps(records,indent=2)+'\n')
print(json.dumps(summary,indent=2),flush=True)
