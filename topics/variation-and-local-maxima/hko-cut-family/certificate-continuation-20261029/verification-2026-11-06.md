# Independent verification of the t = 7/20 HKO certificate

6 November 2026. Target: this directory's fixed-body certificate, absorbed
from `research/viterbo-continuation-20261029`; inspected checkout HEAD
`e5975fab6053c7ffab4b74b13fb9bfb20bb6901a`. This review concerns only the
single-parameter t = 7/20 certificate, not the two-parameter partial certificate,
the whole cut family, or the selected thesis.

## Verdict

**Holds.** The advertised fixed-body capacity and volume are independently
verified. No failing or unverified step remains in this certificate, conditional
on the cited general capacity/simple-minimizer theorem. The exact comparisons
also establish its stated improvements over HKO and the retained t=3/10 value.

## Method and independence

I used **independent exact recomputation and a step-by-step proof check**.
The new checker below reconstructs the twenty rows directly from the formulas
in the parent family README. It imports no repository Python modules, calls no
repository producer, and uses no branch capacity or pruning kernel. General
purpose SageMath number-field linear algebra supplies exact solves and
nullspaces; signs use its exact real-algebraic embedding. The only finite
certificate inputs are `exclusions.txt` (treated as untrusted proposed
positive-curvature words), `graph.txt` and `leaves.txt` (compared with freshly
computed sets). No saved KKT classifications or weights are trusted.

Luna performed read-only file triage. I wrote and ran the independent code and
checked the mathematics. A separate Sol reviewer checked the subtle geometric
reduction, curvature inheritance, and singular-family argument; its coverage
and conclusions appear below. No original tracked file was changed. The
existing modified STATUS/PLAN and other concurrent edits were left untouched;
their separate project-level adoption question remains with the receiving owner.

## Recomputed predicates

The independent run completed successfully (exit status 0):

| Predicate | Fresh result |
| --- | --- |
| Exact geometry | 75 simple vertices; all 20 inequalities are genuine facets |
| Transition graph | 120 edges; exactly matches `graph.txt` |
| Inclusion support maxima | 1 for original rows; 57/50 for moving rows |
| Curvature | All 99,085 proposed exclusions have a positive direction on `ker C` |
| Exhaustive DFS | 348,716 prefixes; 52,306 surviving cycles |
| Canonical leaves | 2,839, maximum length 14; exact equality with `leaves.txt` |
| All leaf systems | 22 closure-inconsistent; 1,961 stationary families at or below target; 856 above-target families infeasible |
| Attaining word | Fresh exact solve has positive normalized closure weights and Q equal to the claimed target |
| Qmax | `tau (8456329321 - 4892944 tau^2) / 41964992141` |
| rho | `1.052345598871062...` |
| rho minus retained t=3/10 value | Positive by exact comparison; `0.00043146004157479465...` |
| rho minus HKO | Positive by exact comparison; `0.0051320033711041175...` |

The leaf totals partition all 2,839 words. They differ from the original receipt's
classification because I rejected **every** above-target affine KKT family by
feasibility, including nonsingular systems, and grouped below-target singular
and nonsingular systems together. There is no unresolved singular case.
The full fresh run took 678.90 seconds; the separate exact volume check took
7.98 seconds. Displayed decimals are descriptive; all decisive comparisons were exact.

Volume received an additional independent exact check, avoiding the packet's
Lawrence formula: cone each facet to the origin; subdivide the facet from its
vertex centroid to its ridges; subdivide each ridge from its vertex centroid
to its edges. Each facet–ridge–edge flag gives a 4-simplex with rows consisting
of the two centroids and the two edge endpoints. Summing absolute determinants
divided by 24 over **900 flags** gives exactly

```text
6644112217/1156332480 - (2199353203/5781662400) tau^2.
```

All vertices and face incidences in that decomposition were exact. Positivity
of the centroids' convex combinations puts them in relative interiors; the
resulting cones partition their faces. Simplicity ensures that intersecting
pairs of facets give ridges and intersecting triples give edges. The code checks
that each such ridge has at least three vertices and each edge exactly two.
A separate floating SciPy hull with exact determinant summation agreed, but is
only a supplementary check; its floating combinatorics are not a proof premise.

## Proof implications checked

1. **Finite capacity formula and geometric word.** I checked Theorems 1.1 and
   1.5 in [Haim–Kislev, arXiv:1712.03494v3](https://arxiv.org/pdf/1712.03494v3),
   printed pages 2–3. Support-normalizing the normals converts weighted support
   heights into `sum beta = 1`. Reversing the paper's word order maps its
   pairing order to the packet's convention. A minimizing characteristic has
   distinct positive normal segments. Each segment lies in its labelled facet:
   its normal belongs to the normal cone, so the segment's points maximize that
   normal's linear functional. Successive facets meet. With velocity `-J N_i`,
   the incoming derivative of the next inequality is `omega(N_i,N_j) >= 0`.
   Thus some global maximizing positive-support word is a simple cycle in the
   freshly verified graph. This requires only necessity, not realization of
   arbitrary graph cycles. Zero-length segments are removed before applying
   the positive-support argument.
2. **Symmetry and pruning.** All twenty label actions were independently checked
   to come from linear maps taking every normal to its specified image, with
   symplectic form preserved or negated. Antisymplectic actions reverse word
   order. Cyclic rotation preserves the objective on closure vectors and the
   restricted Hessian on `ker C`. A positive-curvature vector in `ker C` can
   be padded by zeros on any contiguous superword. An interior maximum cannot
   have such a direction. Hence canonical full-word and suffix pruning preserve
   the minimizing cycle's positive support. The fresh DFS has no length cap;
   it visits every simple directed cycle not excluded by a verified direction.
3. **Stationary points, including singular families.** A positive-support maximum
   satisfies equality-constrained KKT equations after dependent constraint rows
   are removed. A consistent singular KKT family has constant objective:
   `Cz=0`, `Gz+C^T eta=0` imply that both the linear change and the quadratic
   change in Q vanish. Projection of the KKT kernel to beta is injective
   because `C` has full row rank. Its feasible parameter set is bounded by the
   normalized weight simplex. If nonempty, a bounded polyhedron has a vertex
   with enough independent active coordinate constraints, including in lower
   dimension. Enumerating every such active set is therefore complete. The
   fresh checker directly applies this feasibility argument to every
   above-target stationary family; it does **not** use the packet's coarse
   capacity-inclusion shortcut for 33 singular cases.
4. **Attainment and ratio.** A fresh KKT solve for `(0,6,5,2,19,11,8,4)` checks
   strictly positive weights, exact closure, normalization, and the advertised
   objective. Combining attainment with the exhaustive upper bound yields
   `c = 1/(2 Qmax)` and `rho = 1/(8 V Qmax^2)`. Exact sign tests compare the
   resulting ratio with `(3+sqrt(5))/5` and with the retained certified t=3/10
   scalar formulas. The earlier body's complete certificate was not rerun;
   that comparison uses its previously established capacity value.

The external simple-minimizer/capacity theorem is a mathematical input, not a
new theorem proved by this finite computation. No family-wide optimum or
world-record conclusion follows.

## Reproduction and retained independent code

Scratch work is in `/tmp/hko-verify/`; it is not committed or promised permanent.
The source is embedded below so the new verification can be reproduced without
those scratch files. Save the two code blocks as `verify.py` and `volume.py` in
that directory. Run with the checkout's Sage Python:

```bash
OPENBLAS_NUM_THREADS=1 /workspaces/msc-math/.local-environments/miniforge/envs/sage/bin/python /tmp/hko-verify/verify.py
OPENBLAS_NUM_THREADS=1 /workspaces/msc-math/.local-environments/miniforge/envs/sage/bin/python /tmp/hko-verify/volume.py
```

The first script additionally needs NumPy/SciPy for its supplementary hull
cross-check; the second volume calculation is exact and does not use them.
Resource allowance: about 30 minutes and about $1 API-equivalent spend. Actual
provider charges are not available to this process; no exact dollar claim is
made. Execution times below measure the local checks, not future guarantees.

Independent conceptual review: native Sol reviewer
`/root/review_subtle_reduction` found no objection in the assigned graph,
curvature, or singular-family reductions, conditional on the arithmetic replay.
It inspected the fresh main checker but did not execute it. The reviewer did
not inspect or execute the supplementary exact face-volume script. The execution
results in this note were observed by the primary verifier, not that reviewer.

Fresh input and code SHA-256 hashes (one explanatory comment in the main
checker was corrected after execution; executable statements are unchanged):

```text
670f8db8e516ccaec60b772e8de1c4b3cb79dc0dc77a7b3f819ea956790e5e55  verify.py
459d44817d77d361474eac67d37e5277962d208f6fcc7c4eb9f3985a450cc1f8  volume.py
6a08c03e93f534f98280706075ec4d4706d1f1c3d6bd934ddc8aeaf217dfbbb1  exclusions.txt
9963757f794de576e1babc4000036c088807ce2387fe9b183ee829a264c33ba8  graph.txt
281c5f985d0dc45445e56493825ae503505c3a577026ebf717a7ffbc9eb148fe  leaves.txt
```

Fresh structured receipt:

```json
{
  "geometry": {
    "vertices": 75,
    "edges": 120,
    "volume": "-2199353203/5781662400*u^2 + 6644112217/1156332480",
    "volume_numeric": 5.54504905144233,
    "triangles": 400,
    "inclusion": "57/50"
  },
  "curvature": {
    "verified": 99085
  },
  "coverage": {
    "prefixes": 348716,
    "cycles": 52306,
    "leaves": 2839,
    "max_length": 14
  },
  "stationary": {
    "closure_inconsistent": 22,
    "bounded_stationary": 1961,
    "above_target_infeasible": 856
  },
  "verdict": {
    "Q": "-4892944/41964992141*u^3 + 8456329321/41964992141*u",
    "rho": 1.052345598871062,
    "delta_t03": 0.00043146004157479465,
    "delta_H": 0.0051320033711041175,
    "seconds": 678.8963520526886
  }
}
```

### Independent main checker: verify.py

```python
from sage.all import *
from itertools import combinations
from pathlib import Path
import json,time,hashlib
start=time.time()
P=Path('/workspaces/msc-math/topics/variation-and-local-maxima/hko-cut-family/certificate-continuation-20261029')
R=PolynomialRing(QQ,'x'); x=R.gen()
F=NumberField(x**4-10*x*x+5,'u',embedding=AA(5-2*AA(5).sqrt()).sqrt()); u=F.gen()
def sign(a): return AA(a).sign()
s=(5-u*u)/2; a=(3-s)/2; b=u*(1+s)/2; t=QQ(7)/20
N0=matrix(F,[(1,u,0,0),(-a,b,0,0),(1-s,0,0,0),(-a,-b,0,0),(1,-u,0,0),(0,0,u,-1),(0,0,b,a),(0,0,0,s-1),(0,0,-b,a),(0,0,-u,-1)])
D=matrix(F,[(-3*r[0]/5,-3*r[1]/5,-r[1]/(s-1),r[0]/(s-1)) if i<5 else (r[3]/(s-1),-r[2]/(s-1),-3*r[2]/5,-3*r[3]/5) for i,r in enumerate(N0.rows())])
N=N0.stack(N0+t*D)
J=matrix(F,[[0,0,1,0],[0,0,0,1],[-1,0,0,0],[0,-1,0,0]])
W=N*J*N.transpose(); target=u*(8456329321-4892944*u*u)/41964992141
summary={}
def log(k,v):
 summary[k]=v; print(k,v,'elapsed',round(time.time()-start,2),flush=True)
 Path('/tmp/hko-verify/result.json').write_text(json.dumps(summary,indent=2))
def geom(A):
 verts={}
 for I in combinations(range(A.nrows()),4):
  M=A[list(I),:]
  if not M.is_invertible(): continue
  v=M.solve_right(vector(F,[1]*4))
  slack=vector(F,[1]*A.nrows())-A*v
  if all(sign(h)>=0 for h in slack):
   active=tuple(i for i,h in enumerate(slack) if not h)
   verts[tuple(v)]=active
 return verts
V=geom(N); assert len(V)==75 and all(len(I)==4 for I in V.values())
assert set(sum((list(I) for I in V.values()),[]))==set(range(20))
Gph=[[0]*20 for _ in range(20)]
for I in V.values():
 for i in I:
  for j in I:
   if i!=j and sign(W[i,j])>=0: Gph[i][j]=1
frozen=[[int(v) for v in line.split()] for line in (P/'graph.txt').read_text().splitlines()]
assert Gph==frozen
# Independent volume from pyramids over triangulated facets via exact determinant sums.
# Supplementary SciPy hull check; decisive exact volume is in volume.py.
poly=Polyhedron(vertices=list(V),base_ring=AA)
# use scipy ConvexHull boundary triangulation in 4D, with exact determinants per tetrahedron
import numpy as np
from scipy.spatial import ConvexHull
vs=list(V); hull=ConvexHull(np.array([[float(z) for z in v] for v in vs]))
vol=sum(abs(matrix(F,[vs[int(i)] for i in tet]).det()) for tet in hull.simplices)/24
vclaim=F(6644112217)/1156332480-F(2199353203)/5781662400*u*u
assert vol==vclaim
H=geom(N0); supports=[max((r.dot_product(vector(F,v)) for v in H),key=lambda z:AA(z)) for r in N.rows()]
assert supports==[F(1)]*10+[F(57)/50]*10
log('geometry',{'vertices':len(V),'edges':sum(map(sum,Gph)),'volume':str(vol),'volume_numeric':float(vol),'triangles':len(hull.simplices),'inclusion':'57/50'})
# Symmetries independently validated as actual linear maps and pairing signs.
perms=[]
for refl in [False,True]:
 for swap in [0,1]:
  for shift in range(5):
   perm=[]
   for j in range(20):
    block=j//10; f=(j%10)//5; i=j%5
    perm.append(10*block+5*(f^swap)+((4-i if refl else i)+shift)%5)
   piv=[0,1,5,6]; A=N[piv,:].solve_right(N[[perm[i] for i in piv],:])
   assert N*A==N[perm,:]
   assert A*J*A.transpose()==(-J if refl else J)
   perms.append((perm,refl))
def canon(w):
 variants=[]
 for p,rev in perms:
  z=[p[i] for i in w]
  if rev: z.reverse()
  k=z.index(min(z)); variants.append(tuple(z[k:]+z[:k]))
 return min(variants)
def readwords(name):
 out=[]
 for line in (P/name).read_text().splitlines():
  z=list(map(int,line.split())); assert z[0]==len(z)-1
  out.append(tuple(z[1:]))
 return out
def matrices(w):
 C=N[list(w),:].transpose().stack(matrix(F,1,len(w),[1]*len(w)))
 G=matrix(F,len(w),len(w))
 for i in range(len(w)):
  for j in range(i+1,len(w)): G[i,j]=G[j,i]=W[w[i],w[j]]
 return C,G
# A symmetric form has a positive direction iff it fails negative semidefiniteness.
# LDL congruence with arbitrary negative pivots; zero diagonal/nonzero row gives indefiniteness.
def has_positive(B):
 while B.nrows():
  ds=[sign(B[i,i]) for i in range(B.nrows())]
  if 1 in ds: return True
  neg=next((i for i,d in enumerate(ds) if d<0),None)
  if neg is None: return not B.is_zero()
  ix=[i for i in range(B.nrows()) if i!=neg]
  c=B[ix,neg]; B=B[ix,ix]-c*c.transpose()/B[neg,neg]
 return False
ex=readwords('exclusions.txt'); assert len(ex)==99085 and len(set(ex))==len(ex)
for k,w in enumerate(ex):
 C,G=matrices(w); Z=C.right_kernel().basis_matrix()
 assert has_positive(Z*G*Z.transpose()),('bad exclusion',w)
 if k%10000==0: print('curvature',k,round(time.time()-start,1),flush=True)
log('curvature',{'verified':len(ex)})
# Exhaustive simple cycles; no reliance on frozen leaves for completeness.
exset=set(ex); leaves=set(); counts=[0,0]; minex=min(map(len,ex))
def dfs(w,used):
 counts[0]+=1
 if len(w)>=minex:
  for k in range(len(w)-minex+1):
   if canon(w[k:]) in exset: return
 if len(w)>1 and Gph[w[-1]][w[0]]:
  counts[1]+=1; leaves.add(canon(w))
 for j in range(w[0]+1,20):
  if not (used>>j)&1 and Gph[w[-1]][j]: dfs(w+[j],used|(1<<j))
for i in range(20): dfs([i],1<<i)
frozenleaves=set(readwords('leaves.txt')); assert leaves==frozenleaves
log('coverage',{'prefixes':counts[0],'cycles':counts[1],'leaves':len(leaves),'max_length':max(map(len,leaves))})
# Verify every stationary affine family directly. No coarse inclusion shortcut.
statistics={}; best=F(0); attainers=[]
for k,w in enumerate(sorted(leaves)):
 C,G=matrices(w); e=vector(F,[0,0,0,0,1]); n=len(w)
 if C.rank()!=C.augment(e).rank(): category='closure_inconsistent'
 else:
  ix=C.transpose().pivots(); C=C[list(ix),:]; e=vector(F,[e[i] for i in ix]); m=C.nrows()
  B=block_matrix([[G,C.transpose()],[C,zero_matrix(F,m,m)]])
  rhs=vector(F,[0]*n+list(e))
  if B.rank()!=B.augment(rhs).rank(): category='stationary_inconsistent'
  else:
   sol=B.solve_right(rhs); beta=sol[:n]; Z=B.right_kernel().basis_matrix(); q=beta.dot_product(G*beta)/2
   for row in Z.rows():
    z=row[:n]; assert C*z==0 and beta.dot_product(G*z)==0 and z.dot_product(G*z)==0
   if sign(q-target)<=0: category='bounded_stationary'
   else:
    d=Z.nrows(); feasible=None
    if not d:
     if all(sign(v)>=0 for v in beta): feasible=beta
    else:
     T=Z[:,:n].transpose(); assert T.rank()==d
     for I in combinations(range(n),d):
      A=T[list(I),:]
      if not A.is_invertible(): continue
      y=A.solve_right(-vector(F,[beta[i] for i in I])); z=beta+T*y
      if all(sign(v)>=0 for v in z): feasible=z; break
    assert feasible is None,('ABOVE TARGET',w,str(q),str(feasible))
    category='above_target_infeasible'
 statistics[category]=statistics.get(category,0)+1
 if k%500==0: print('leaves',k,statistics,round(time.time()-start,1),flush=True)
log('stationary',statistics)
# Solve target word independently, rather than parse stored weights.
w=(0,6,5,2,19,11,8,4); C,G=matrices(w); m=C.nrows(); n=len(w)
B=block_matrix([[G,C.transpose()],[C,zero_matrix(F,m,m)]])
z=B.solve_right(vector(F,[0]*n+[0,0,0,0,1])); beta=z[:n]
assert all(sign(v)>0 for v in beta) and C*beta==vector(F,[0,0,0,0,1])
assert beta.dot_product(G*beta)/2==target
rho=1/(8*vol*target**2)
v03=F(300748771)/52178240-F(99664289)/260891200*u*u
rho03=1/(8*v03*(205*u/1019)**2)
assert sign(rho-rho03)>0 and sign(rho-(3+s)/5)>0
log('verdict',{'Q':str(target),'rho':float(rho),'delta_t03':float(rho-rho03),'delta_H':float(rho-(3+s)/5),'seconds':time.time()-start})
```

### Exact face-volume checker: volume.py

```python
from pathlib import Path
exec(Path('/tmp/hko-verify/verify.py').read_text().split('# Independent volume')[0])
# Exact barycentric subdivision of each 3D facet, coned to the origin.
# Each term follows a facet > ridge > edge flag. A ridge is triangulated
# from its vertex centroid to its edges; the facet is coned from its centroid.
volume=F(0); flags=0
for i in range(20):
 fv=[vector(F,v) for v,I in V.items() if i in I]
 fc=sum(fv,vector(F,[0]*4))/len(fv)
 for j in range(20):
  if i==j: continue
  rv=[vector(F,v) for v,I in V.items() if i in I and j in I]
  if not rv: continue
  assert len(rv)>=3
  rc=sum(rv,vector(F,[0]*4))/len(rv)
  for k in range(20):
   if k in (i,j): continue
   ev=[vector(F,v) for v,I in V.items() if i in I and j in I and k in I]
   if not ev: continue
   assert len(ev)==2
   M=matrix(F,[fc,rc]+ev); d=M.det(); assert d
   volume+=(d if sign(d)>0 else -d)/24; flags+=1
expected=F(6644112217)/1156332480-F(2199353203)/5781662400*u*u
assert volume==expected
print('EXACT_FACE_VOLUME',volume,'flags',flags,'seconds',time.time()-start,flush=True)
Path('/tmp/hko-verify/exact-volume.json').write_text(json.dumps({'volume':str(volume),'flags':flags,'seconds':time.time()-start},indent=2))
```
