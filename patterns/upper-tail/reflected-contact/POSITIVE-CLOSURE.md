# A canonical single closure pair cannot be HKO sharp

Exact geometric obstruction, 17 October 2026. Not a claim against arbitrary
invariant scalar predictors or against set-valued contact-face rules.

For factor normal presentations A=(a_i), B=(b_j), let
C_A={alpha>=0:sum alpha=1,sum alpha_i a_i=0}, and similarly C_B.
The imported product HK reduction gives the true maximum Q_* over words and
closure pairs, with each factor carrying total mass1/2. For fixed word w,

Q_w(alpha,gamma)=1/4 sum_ij epsilon_ij(w) alpha_i gamma_j a_i.b_j.

## Exact all-word uniform closure at HKO

Use d=sqrt5 and the transformed HKO normals

A=[(1,0),((d-1)/4,1),(-(d+1)/4,(d-1)/2),
   (-(d+1)/4,-(d-1)/2),((d-1)/4,-1)], B=J A.

These are obtained from unit regular normals by T=diag(1,1/sin(2pi/5))
and the standard HKO relative rotation pi/2 (equivalent modulo pentagon
symmetry to pi/10). JT=(det T)T^(-T)J; hence the transformation equals a
product symplectic normal change followed by scaling B by1/sin(2pi/5).
The imported regular capacity therefore gives Q_*=1/10, c_*=5 in this
presentation. This normalization is also derived in the existing
conditional-coupling HKO packet; it is not a new capacity evaluation.

For u=(1/5,...,1/5), exact full word optimization gives

max_w Q_w(u,u)=sqrt5/25=(2/sqrt5)Q_*.

The computation is exhaustive over WORDS at these ONE fixed closure weights,
not over original capacity supports. Fix A_0 as cyclic cut. Its24 cyclic A
orders exhaust all possibilities. For each order, let U be its five prefix
points from weighted A steps. Each B step independently chooses a gap, and
its contribution is1/2 p.y. Thus the maximum for this A order is
1/2 sum_j max_{p in U} p.y_j. All24 exact maxima and direct attaining word
objectives are retained in hko-exact.json. The separate verifier enumerates
all9! full cyclic words without using this gap reduction.

Therefore the fixed-uniform-closure capacity upper bound has EXACT error

C_u/c_* = sqrt5/2 =1.1180339887...,
S_u/sys_* =5/4,
S_u =(3+sqrt5)/4 =1.3090169943... .

This sharpens the existing fixed-cone-volume packet's lower bound on that
rule's error. The main new conclusion below concerns a whole canonical
witness class, not just a more accurate constant for one stopped feature.

## Strictly positive closures cannot attain the true maximum

Let alpha,gamma be any strictly positive closures at HKO, and put
e_A=5 min_i alpha_i, e_B=5 min_j gamma_j in(0,1]. Then
alpha=e_A u+(1-e_A)alpha' and gamma=e_B u+(1-e_B)gamma', with
alpha',gamma' feasible whenever the corresponding coefficient is nonzero.
By bilinearity, Q_w(alpha,gamma) is a convex combination of its four values
on(u,u),(u,gamma'),(alpha',u),(alpha',gamma'). The first is at most
(2/sqrt5)Q_*; every other value is at most Q_*. Hence for EVERY word,

Q_w(alpha,gamma)/Q_* <=
  1-(1-2/sqrt5)e_A e_B <1.

The same bound holds after maximizing words. There is no claimed uniform
strict gap over all positive closure pairs: e_A e_B can approach zero as the
closures approach boundary maximizers. The quantitative formula explicitly
preserves this limitation. This proof uses only the exact uniform gap and
bilinearity, not a new capacity support classification.

## Equivariant single-witness obstruction

Suppose a rule assigns ONE pair of normalized closures to every product,
and the assigned weighted normal steps are equivariant under facet
permutations and product-preserving linear symplectic transformations.
No continuity, differentiability or positivity assumption is needed here.

At the standard regular HKO product, simultaneous planar rotation by2pi/5
is a product-preserving linear symplectic symmetry and cyclically permutes
the five normals in EACH factor. Equivariance of a single assigned pair
forces alpha_i=alpha_(i+1) and gamma_j=gamma_(j+1); normalization forces both
closures to be uniform. Therefore even its BEST possible word optimizer
returns the capacity upper bound sqrt5/2 times true capacity, and ratio
upper bound5/4 times HKO. The transformed presentation inherits the statement
by conjugacy of this symmetry.

This excludes all such canonical single-witness HK surrogates from being an
HKO-sharp route to the desired global ratio inequality. It covers a joint
single-pair rule as well as factorwise rules, since the SAME simultaneous
rotation acts on both factors. It does NOT assume arbitrary independent
rotation equivariance, which would exceed the symplectic symmetry group.

It does NOT exclude:
- invariant scalar formulas which are not defined through such a witness;
- rules choosing a nonequivariant witness but whose scalar output remains
  invariant by a separately proved mechanism;
- set-valued rules retaining a symmetry orbit of extremal closures;
- exact capacity computation, which can choose one maximizing witness by
  arbitrary tie-breaking without making the witness itself canonical.

The direct design implication is to retain a geometric CONTACT FACE rather
than replace it by its positive average. The separately frozen reflected-cover
probe tests that idea; this theorem supplies no theorem or performance
guarantee for the new selector.

Focused sources: repository/research/product/empirical entry points; prior
fixed-closure report; product six-facet and two-shadow proofs; existing
conditional-coupling HKO normalization; existing contact/mixed-area/spectral
obstructions. This is not an exhaustive literature search or novelty claim.
All work is local, exact control arithmetic; no thesis edit or public action.

[review: /root/closure_review]
