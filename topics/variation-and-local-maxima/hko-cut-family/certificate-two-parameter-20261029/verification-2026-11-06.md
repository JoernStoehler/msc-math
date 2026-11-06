# Independent verification — 6 November 2026

**Verdict: holds as a partial certificate.** The completed geometry, exact volume,
support inclusion and strictly positive feasible eight-label objective are correct.
The packet does **not** establish the exact capacity, exact systolic ratio, a new
record, or an improvement over a family envelope. Those conclusions remain
undetermined. No contradiction was found in the claims actually made by its
README; the prospective global argument in COVERAGE-PROOF has unmet premises.

This check concerns only `a=99/125, t=7/20` in this two-parameter packet. The
separately assigned one-parameter t=7/20 certificate was not checked or replayed.
Starting checkout HEAD was `e5975fab`; existing shared edits were left untouched.
Only this new report is delivered; PLAN and STATUS remain for the receiving
coordinator to reconcile under the user's explicit one-file constraint.

## Exact result and decisive implication

Write tau = sqrt(5 - 2 sqrt(5)), the root in (0,1) of
`tau^4 - 10 tau^2 + 5 = 0`. The body is the retained N0 inequalities together
with `a N0 + t (D + 3 N0/5)`, as defined in this packet. In particular the
moving rows have radial coefficient 99/125; this is a different body from
the sibling one-parameter example.

Independent exhaustive arithmetic gives 75 distinct simple vertices, all twenty
facets genuine, and exactly the retained 120-edge adjacency matrix. The original
HKO body H has 25 vertices. Every original row has support 1 on H and every
moving row has support 571/500, proving `H/(571/500) subset K`.

The exact four-volume is

```
V = 12188906067413/2122975008000
    - (7700705900357/20264761440000) tau^2
  = 5.54083616487837901... .
```

The word `(0,6,5,2,19,11,8,4)` admits strictly positive weights satisfying
`sum beta = 1` and `sum beta_i N_word[i] = 0`. Solving its stationary linear
system afresh and evaluating the ordered pairing sum gives

```
Q8 = tau (5328025872481 - 3162037824 tau^2) / 26435588952481
   = 0.146386929474032054... .
```

The decisive step is the direction of the bound. Haim–Kislev's finite formula
is `c_EHZ(K) = 1/(2 Qmax)`. Extend these eight weights by zeros to all twenty
facets, and reverse the ordering to match the displayed source convention.
This is an admissible member of the source maximum, so `Qmax >= Q8 > 0`, hence

```
c_EHZ(K) <= 1/(2 Q8) = 3.41560549016567943... .
sys(K) <= 1/(8 V Q8^2) = 1.05276175989459229... .
```

The second inequality is an **upper** bound on the ratio. It does not prove
that the ratio exceeds any benchmark. Feasibility requires neither boundary
placement of this particular word nor its global optimality. The stationary
solve is a way to obtain the witness, not a proof of its global maximality.
The source formula was checked directly in the locally retained paper
[`EHZ-polytopes.tex`](../../../literature/sources/hk2017/EHZ-polytopes.tex),
Theorem labelled `formula_theorem`, lines 66–76, and its unit-normal/height
conversion against
[`hk2017-qp-conventions.tex`](../../../capacity-and-qp/proofs/hk2017-qp-conventions.tex).
The published capacity formula is used as a mathematical input, not reproved here.
Likewise the packet's further coarse objective bound follows from the inclusion
and the known value of c(H); this run independently checks the inclusion, not
the known HKO capacity theorem.

## Independent method and executed checks

No retained Python producer, common module, exclusion kernel or C++ traversal
was imported or executed. Only mathematical input rows were transcribed from
`common.py`. The implementation below uses SymPy 1.14.0's algebraic field
arithmetic and fresh Gaussian elimination, with exact rational interval Horner
sign decisions. It does not use Sage arithmetic or the packet's sign kernel.
All nonzero sign decisions resolved with tau between
`7265425280053/10^13` and `7265425280054/10^13`; endpoint polynomial signs and
strict decrease on (0,1) identify the desired root.

All 4,845 four-row subsets were examined, with 1,305 singular systems discarded.
Every remaining intersection was tested against every inequality; the accepted
active sets match the retained receipt exactly. Each has exactly four active
rows. Affine rank three of each facet's vertex set was checked by exact minors.
Boundedness also follows directly from the two five-row base blocks: each sums
to zero and spans its coordinate plane. Thus a recession direction must pair
nonpositively with their positive dependence and hence pair to zero with all
base rows, forcing that direction to vanish. The origin strictly satisfies all
inequalities. These facts make exhaustive vertex enumeration sufficient.

For volume, a different construction avoids the retained Lawrence formula.
For each facet choose its vertex average, and for each polygonal ridge choose
its vertex average. Each ridge edge and ridge average form a triangle; cone
that triangle first to the facet average and then to the origin. Simple
four-dimensional incidence identifies an edge by three shared active labels.
Summing absolute determinants divided by 4! over all 900 facet–ridge–edge flags
reproduces the claimed volume exactly. Interiors of these cones are disjoint;
shared ridge incidences belong to different facet cones. This is a boundary
triangulation with rational/algebraic interior points, not a floating volume fit.

The sign-tested coactivity graph matches every entry in `graph.txt`. The fresh
13-by-13 stationary solve checks all five equality constraints, positivity of
every weight and the exact Q8 identity. Re-running the finalized script passed
in about 12 seconds. Scratch code and output are in `/tmp/twoparam-verify/`;
the complete script is retained below so this report is reproducible without
relying on temporary storage. It reads only the two original receipt files at
the end for comparison; none of their numerical answers drives the computation.

## Unfinished global route and review

`partial-state.json`, WORKSTATE and both prior partial audits consistently
report that the curvature work stopped before all 99,085 proposals were checked.
No accepted exclusion array was persisted; no fresh cycle coverage or complete
leaf KKT check is supplied. This run does not rerun those stages or endorse any
individual exclusion. Completing them, or replacing them with a global proof,
is the concrete next step if exact capacity or an improved ratio is wanted.
No family optimization or envelope theorem follows from this verification.

A Luna agent performed bounded read-only triage. A separate Sol agent
`/root/sol_review` independently reviewed the decisive capacity-bound implication,
the fresh implementation and the boundary-cone volume counting. Sol reviewer `/root/sol_review` approved report-v1 and found no material mathematical or implementation issue within this bounded review. Parent remains responsible for the verdict and
integration. Work stayed within the requested 30-minute ceiling; API-equivalent
spend was not measured, so a precise dollar total is not claimed.

## Reproduction script

Extract this Python block to `/tmp/twoparam-verify/check.py` and run with a Python
having SymPy 1.14.0 (the existing local Sage environment's Python supplied SymPy;
no Sage module is imported). All geometry and sign decisions use exact arithmetic.

```python
from sympy import QQ,sqrt
from itertools import combinations
from fractions import Fraction as R
import json,time
from pathlib import Path
F=QQ.algebraic_field(sqrt(5-2*sqrt(5))); u=F.unit; Z=F.zero; one=F.one
f=lambda p,q=1:F(QQ(p,q))
lo=R(7265425280053,10**13); hi=R(7265425280054,10**13)
assert lo**4-10*lo**2+5>0>hi**4-10*hi**2+5 and 0<lo<hi<1
# Interval Horner evaluation: exact rational proof of every nonzero sign.
def sign(x):
 if not x:return 0
 l=h=R(0)
 for c in x.to_list():
  products=[l*lo,l*hi,h*lo,h*hi]; c=R(int(c.numerator),int(c.denominator))
  l,h=min(products)+c,max(products)+c
 if l>0:return 1
 if h<0:return -1
 raise ValueError('unresolved interval')
def dot(a,b):return sum((x*y for x,y in zip(a,b)),Z)
def solve(A,b):
 M=[list(row)+[v] for row,v in zip(A,b)]; n=len(b)
 for j in range(n):
  p=next((i for i in range(j,n) if M[i][j]),None)
  if p is None:return None
  M[j],M[p]=M[p],M[j]; d=M[j][j]; M[j]=[x/d for x in M[j]]
  for i in range(n):
   if i!=j and M[i][j]:
    d=M[i][j]; M[i]=[x-d*y for x,y in zip(M[i],M[j])]
 return [row[-1] for row in M]
def det(A):
 M=[list(row) for row in A]; d=one
 for j in range(len(M)):
  p=next((i for i in range(j,len(M)) if M[i][j]),None)
  if p is None:return Z
  if p!=j:M[j],M[p]=M[p],M[j];d=-d
  pivot=M[j][j];d*=pivot
  for i in range(j+1,len(M)):
   c=M[i][j]/pivot
   for k in range(j+1,len(M)):M[i][k]-=c*M[j][k]
 return d
s=(5-u*u)/2; al=(3-s)/2; be=u*(1+s)/2
# Input rows transcribed as mathematics; no packet code imported or executed.
base=[[one,u,Z,Z],[-al,be,Z,Z],[1-s,Z,Z,Z],[-al,-be,Z,Z],[one,-u,Z,Z],
      [Z,Z,u,-one],[Z,Z,be,al],[Z,Z,Z,s-1],[Z,Z,-be,al],[Z,Z,-u,-one]]
a=f(99,125);t=f(7,20)
moving=[]
for i,n in enumerate(base):
 cross=[Z,Z,-n[1]/(s-1),n[0]/(s-1)] if i<5 else [n[3]/(s-1),-n[2]/(s-1),Z,Z]
 moving.append([a*x+t*y for x,y in zip(n,cross)])
N=base+moving
start=time.time()
def vertices(rows):
 out=[]; singular=0
 for ids in combinations(range(len(rows)),4):
  v=solve([rows[i] for i in ids],[one]*4)
  if v is None:singular+=1;continue
  sl=[one-dot(r,v) for r in rows]
  if all(sign(x)>=0 for x in sl):
   active=tuple(i for i,x in enumerate(sl) if not x)
   assert len(active)==4
   out.append((active,v))
 assert len({tuple(v) for ids,v in out})==len(out)
 return out,singular
V,sing=vertices(N); print('vertices',len(V),'singular systems',sing,'seconds',time.time()-start,flush=True)
H,hsing=vertices(base);print('H vertices',len(H),flush=True)
assert len(V)==75 and len(H)==25
supports=[]
for row in N:
 values=[dot(row,v) for ids,v in H];mx=values[0]
 for z in values[1:]:
  if sign(z-mx)>0:mx=z
 supports.append(mx)
assert supports==[one]*10+[f(571,500)]*10
# Each label must have an affine-three-dimensional face.
for i in range(20):
 face=[v for ids,v in V if i in ids]; v0=face[0]
 diffs=[[x-y for x,y in zip(v,v0)] for v in face[1:]]
 assert any(det([[r[j] for j in cols] for r in rr]) for rr in combinations(diffs,3) for cols in combinations(range(4),3))
print('all facets rank 3; exact support inclusion PASS',flush=True)
# Independent boundary triangulation: each 2-face is a polygon; fan from
# its vertex average, then cone to facet average and finally interior 0.
volume=Z; tetrahedra=0
for i in range(20):
 face=[(ids,v) for ids,v in V if i in ids];fc=[sum((v[k] for ids,v in face),Z)/len(face) for k in range(4)]
 for j in range(20):
  if j==i:continue
  ridge=[(ids,v) for ids,v in face if j in ids]
  if not ridge:continue
  assert len(ridge)>=3
  rc=[sum((v[k] for ids,v in ridge),Z)/len(ridge) for k in range(4)]
  edges=[(p,q) for p,q in combinations(ridge,2) if len(set(p[0])&set(q[0]))==3]
  assert len(edges)==len(ridge)
  for (_,v),(_,w) in edges:
   d=det([fc,rc,v,w]);sgn=sign(d);assert sgn
   volume+=sgn*d/24;tetrahedra+=1
claimed=f(12188906067413,2122975008000)-f(7700705900357,20264761440000)*u*u
assert volume==claimed
print('triangulation volume PASS',F.to_sympy(volume),'tetrahedra',tetrahedra,flush=True)
def omega(x,y):return x[0]*y[2]+x[1]*y[3]-x[2]*y[0]-x[3]*y[1]
edges=[(i,j) for i in range(20) for j in range(20) if i!=j and any(i in ids and j in ids for ids,v in V) and sign(omega(N[i],N[j]))>=0]
assert len(edges)==120
word=[0,6,5,2,19,11,8,4];m=len(word)
C=[[N[i][k] for i in word] for k in range(4)]+[[one]*m]
G=[[Z if i==j else omega(N[word[min(i,j)]],N[word[max(i,j)]]) for j in range(m)] for i in range(m)]
KKT=[G[i]+[C[k][i] for k in range(5)] for i in range(m)]+[row+[Z]*5 for row in C]
sol=solve(KKT,[Z]*m+[Z]*4+[one]);assert sol
b=sol[:m];assert all(sign(x)>0 for x in b)
assert [dot(row,b) for row in C]==[Z]*4+[one]
q=dot(b,[dot(row,b) for row in G])/2
assert q==u*(f(5328025872481)-f(3162037824)*u*u)/f(26435588952481)
print('8-word closure, positivity, stationary solution and Q PASS',flush=True)
print('Q',F.to_sympy(q),'capacity upper bound',F.to_sympy(one/(2*q)),flush=True)
packet=Path('/workspaces/msc-math/topics/variation-and-local-maxima/hko-cut-family/certificate-two-parameter-20261029')
receipt=json.loads((packet/'exact-geometry-summary.json').read_text())
assert sorted([list(ids) for ids,v in V])==sorted(receipt['vertex_active_sets'])
recorded=[]
for line in (packet/'graph.txt').read_text().splitlines():
 recorded.append(line)
adjacency=[list(map(int,line.split())) for line in recorded]
assert len(adjacency)==20 and all(len(row)==20 for row in adjacency)
assert {(i,j) for i in range(20) for j in range(20) if adjacency[i][j]}==set(edges)
print('graph adjacency exact match PASS',flush=True)
print('volume numeric',F.to_sympy(volume).evalf(18),'Q numeric',F.to_sympy(q).evalf(18),'capacity bound numeric',F.to_sympy(one/(2*q)).evalf(18),'ratio upper bound',F.to_sympy(one/(8*volume*q*q)).evalf(18),flush=True)
print('TOTAL seconds',time.time()-start)
```
