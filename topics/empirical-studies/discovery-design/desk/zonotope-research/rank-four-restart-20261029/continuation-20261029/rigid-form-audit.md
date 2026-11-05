# Independent mathematical audit: rigid-form reduction

29 October 2026, 18:28 UTC. Reviewer: `/root/rank_four_audit`.

**Verdict: PASS for the stated necessary-condition reduction.** I found no
substantive gap in the compactness, deletion, support-minimality, form-space,
incidence, or product arguments. The surviving `dim L=1` case remains open:
this is neither a proof of `4 Psi <= Phi^2` for arbitrary size nor an equality
classification. The final illustration that the surviving configuration type
actually occurs is also valid.

The reviewed draft is `rigid-form-reduction.md`, SHA-256
`00c6525799f49963cf79af4af761628e2e6d6a7723ec31d45b3bb719dee40549`.
I read that complete draft in the assigned checkout
`/workspaces/msc-math-viterbo-continuation`, and consulted the retained
`rank-four-restart-20261029/nonproduct-equality.md` and its component audit.
The review does not establish novelty or any capacity implication. No project
files were edited by this reviewer. Root owns integration and cleanup.

## Extremizer and deleted blocks

At fixed original size `n`, the feasible set is compact: skew symmetry and
rank at most four are closed conditions, and `Phi=1` bounds every entry.
Continuity gives an attained maximum, and the finite integer-valued support
size has a minimum among maximizers. Removing zero rows causes no problem:
every later competitor of the smaller size may be padded with zero rows to
the original size, preserving rank, `Phi`, `Psi`, and support count.

The deleted-block estimate has the correct coefficient. For a distinguished
index `i`, every surviving Pfaffian has the form

`W_ij W_kl - W_ik W_jl + W_il W_jk`.

After applying the triangle inequality, each product of an incident edge and
a disjoint old edge belongs to exactly one such four-set. Their sum is at
most `a Phi(W_0)`; allowing old edges meeting the incident edge only enlarges
that bound. If `rank W_0<=2`, all old four-Pfaffians vanish, so
`4 Psi <= 4a Phi(W_0) <= (a+Phi(W_0))^2`. This excludes every rank-deficient
deleted block at the selected counterexample. Its remaining generators
therefore span all of `V`, making each incident pairing functional `a_i` a
norm.

## The row deformation really reduces support

Let `H_i` be the span of the zero-neighbors. Nondegeneracy of `omega` gives
`K_i=H_i^omega` and `dim K_i=4-dim H_i`. On the prescribed sign cone,
`a_i=ell_i` is linear, and `ell_i(v_i)=a>0`. The slice has local affine
dimension `dim K_i-1`: every currently nonzero pairing is strict at `v_i`,
so it imposes no further local affine equation. Thus if `dim H_i<=2`, the
slice has positive dimension.

It is a bounded polytope because it lies on the radius-`a` sphere of the
norm `a_i`. A vertex cannot retain all strict sign inequalities in a
positive-dimensional affine slice. Hence it gains an additional zero
pairing, while every old zero stays zero. The Pfaffian sum is convex in the
moving generator, since all Pfaffians containing it are linear before
taking absolute values. Writing `v_i` as a convex combination of vertices
provides one vertex with objective at least `M`. Global maximality then
gives equality, and support-minimality gives the required contradiction.

## Lorentz signature and nondegeneracy

The identity relating the four-Pfaffians is exact, including signs: evaluating
`eta wedge eta/2=q(eta) vol` on each ordered generator quadruple gives its
Pfaffian, and the same evaluation for `omega` has coefficient one. Thus
`Psi(W_eta)=|q(eta)|M`.

The local-minimum assertion follows from normalization, not from a hidden
restriction on admissible competitors. When `q(eta)=1` and `N(eta)<1`, the
matrix `W_eta/N(eta)` has `Phi=1`, rank at most four, and objective
`M/N(eta)^2>M`, a contradiction. Here `N` is a norm because the generators
span `V`.

For `z` in the `B`-orthogonal tangent space, the displayed curve gives
`ell(z)=0` and then second derivative `-q(z)>=0`. Consequently
`q|L` is positive on `R omega` and negative semidefinite on its orthogonal
complement. If the restriction has a radical vector `z`, then `ell(z)=0`
and the whole affine line has `q=1` and `ell=1`. Its intersection with the
closed sign cone is a compact interval containing zero in its relative
interior: on that intersection `N=1`, and a norm has bounded unit sphere.
At either finite endpoint an additional pairing vanishes. This preserves
the objective and contradicts minimal support. Thus the restriction is
nondegenerate and has signature `(1,d-1)` as claimed.

## Active zero-pairing span and the two exclusions

One precise choice for the identification is
`c_ij = interior(v_j) interior(v_i) vol`. With the stated polarization,
`eta(v_i,v_j)=2 B(eta,c_ij)`. Each such form is decomposable or zero, so it
is null. Two forms from edges sharing a generator have zero wedge product
and are `B`-orthogonal. Defining active edges to mean the **zero pairings**,
the definition of `L` gives `F=L^perp=span{c_ij: active}`. This remains valid
when some generators are parallel and their bivectors are zero.

Orthogonal inertia subtraction gives signature `(2,4-d)` on `F`. For `d=4`
this positive definite space cannot be spanned by null vectors. For `d=3`,
its Witt index is one, whereas the incident active forms at any generator
span a totally isotropic plane: the already proved row condition gives
`dim(v_i wedge v_i^omega)=3-1=2`. These are genuine contradictions.

For `d=2`, the two null **lines** in the Lorentz plane are represented by
rank-two forms `eta,theta` with `eta wedge theta !=0`. Their kernel planes
are complementary. A vector with nonzero components in both planes has
common annihilator under `eta(v,.)` and `theta(v,.)` equal to the two-plane
spanned by its two components. Its zero-neighbors cannot span the required
three-dimensional hyperplane. Hence all generators lie in one of the two
kernel planes. Both forms, and therefore `omega`, have zero cross pairings.
Only the two-indices-in-each-plane Pfaffians survive, giving `Psi=A C` and
`Phi=A+C`. This proves the asserted contradiction without any genericity
assumption or graph enumeration.

## The remaining case is nonempty

A supporting read-only check by `/root/rank_four_audit/equality_check`
identified the following direct rigidity argument, which I checked.
For the paired-inverse family, use the first four generators as a basis,
with Gram matrix `A`, and write the other four as columns of
`C=t A^-1`. The cross-pairing matrix is `A C=tI`. All entries of `A` and
its skew inverse off the diagonal are nonzero in the retained chamber, so
the zero-neighbors of each generator are precisely three independent
generators in the opposite basis. They span the orthogonal hyperplane.

For any `eta` satisfying those zero constraints, let `X` be its skew
matrix in the first basis. The matrix `t X A^-1` must be diagonal, say `D`.
Thus `X=D A/t`. Skew symmetry implies `DA=AD`; since every off-diagonal
entry of `A` is nonzero, all four diagonal entries of `D` agree. Therefore
`eta` is proportional to `omega`. This proves `dim L=1` throughout the
specified nonzero-entry chamber, rather than merely for generic parameters.

An exact rational instance in the already audited self-dual subfamily is
`x=1,y=2,z=2`, so `p=9,t=3` and

```
A = [[ 0, 1, 2, 2],
     [-1, 0, 2,-2],
     [-2,-2, 0, 1],
     [-2, 2,-1, 0]],       W = [[A,3I],[-3I,A]].
```

Independent standard-library rational arithmetic gave `rank W=4`,
`Phi=32`, `Psi=256`, all eight zero-neighbor ranks equal to three, and rank
five for the linear equations defining `L`. These finite checks illustrate
the algebraic argument; they are not evidence for the unrestricted inequality.
The convenient alternative `A_ij=1` for all `i<j` has `dim L=1` but does
**not** lie in this equality chamber, since one Pfaffian matching term is
negative; it should not be used as an equality example.

## Nonblocking exposition improvements

The proof is valid as written. For a self-contained retained version, spell
out the zero-padding step, the `N`-normalization step, and the meaning of
active edges; give the contraction identification above; use “two null
lines” in the `d=2` paragraph; and attach the short paired-inverse rigidity
argument to the final illustrative claim. None changes the proposition.

No further mathematical conclusion about `d=1` was reviewed or approved.
