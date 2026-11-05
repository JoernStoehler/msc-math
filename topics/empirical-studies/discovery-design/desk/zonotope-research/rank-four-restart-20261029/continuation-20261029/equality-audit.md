# Independent audit: equality-preserving additions

29 October 2026, 18:43 UTC. Reviewer: `/root/rank_four_audit`.

**Verdict: PASS for the conditional Lagrangian-addition lemma, both explicit
ten-direction equality families, and all fifteen stated two-addition deficit
polynomials.** The unrestricted rank-four inequality, general copositivity,
and a classification of equality remain open. Nothing checked here implies a
capacity statement or novelty claim.

The inspected packet is
`topics/empirical-studies/discovery-design/desk/zonotope-research/rank-four-restart-20261029/continuation-20261029/`
in `/workspaces/msc-math-viterbo-continuation`. I read both complete notes and
both checkers, ran their standard-library checks, and independently computed
the additional rational checks described below. I did not edit project
sources; the author corrected the terminology and added pair-coverage
assertion during this review. Root owns retention, integration, and cleanup.

## Conditional addition lemma

The statement and coefficient proof are correct with the supplied
symplectic-volume convention `mu=omega^2/2`. For `h,k` in the Lagrangian
plane and `q_i=Rp_i`, the defining relation
`det_L(h,q_i)=omega(h,z_i)` gives, directly from the four-Pfaffian formula,

```
mu(h,k,z_i,z_j)
 = -det_L(h,q_i) det_L(k,q_j)
   +det_L(h,q_j) det_L(k,q_i)
 = -det_L(h,k) det_L(q_i,q_j).
```

Thus the absolute-value factorization used for the quadratic term has no
missing symplectic-volume factor. The components of the old generators
lying in `L` make no difference. The stated identity can also serve as the
definition of the rotated projection; no metric-dependent property of a
quarter turn is needed beyond that identity.

Pairings between added vectors vanish, so `Phi(Z+tP)=Phi(Z)+tM`.
There are no determinants containing three added vectors. The contact
conditions give linear coefficient `Phi(Z)M/2` in `Psi`. The quadratic
coefficient is exactly `area(P)area(Q)/16`.

The mixed-area factor is correct. The planar segment formula gives
`area(P+sQ)=area(P)+4Ms+area(Q)s^2`, because
`M=sum_{a,i}|det_L(h_a,q_i)|`. If `Q=cP`, comparison with
`area((1+cs)P)` yields
`4M=2c area(P)` and `area(Q)=c^2 area(P)`, hence
`area(P)area(Q)=4M^2`. The quadratic coefficient in `Psi` is consequently
`M^2/4`, which proves the asserted identity for all `t>=0`.
The argument also remains valid for degenerate planar zonotopes: all these
identities hold with zero area. Parallel or repeated generators introduce
no incorrect multiplicities.

The example satisfies the hypotheses exactly. An independent calculation
using `9|det4|` for each absolute Pfaffian reproduced the contact pairs
`(8,128),(8,128),(36,576),(12,192)`. Direct evaluation reproduced all eight
listed rotated projections. Grouping their parallel segments gives the four
coefficient ratios `36`, proving `Q=36P` as zonotopes. Here `M=14/3`, so
the stated `Phi` and `Psi` polynomials follow. This lemma's `P` is the
second explicit addition in `equality-extension.md` divided by `36`;
their parameters therefore satisfy `t=36s`.

## Both ten-direction families

`python3 equality-extension.py` passed. Its expansion is a proof over the
whole interval `s>=0`, because every direction is fixed and each generator
weight is nonnegative there. Absolute values of pairings and Pfaffians
therefore factor into constant absolute values times products of those
weights. No unstated sign-chamber stability is needed.

The resulting coefficient lists are:

| Family | `Phi` | `Psi` |
|---|---|---|
| Balanced parallelotope addition | `32+168s+144s^2` | `256+2688s+9360s^2+12096s^3+5184s^4` |
| Lagrangian addition | `32+168s` | `256+2688s+7056s^2` |

Each is exactly `4Psi=Phi^2`. The first summand has just the two disjoint
nonzero pairings `72,-72`; the second has none. The ten listed directions
are pairwise nonparallel, and all generator weights are positive for
`s>0`. The first two added segments increase existing lengths, while the
other two introduce new directions. The retained checks also confirm
independence of every triple of the original eight directions.

The nonproduct inference is valid: all eight original segment directions
remain edge directions of the full-dimensional zonotope. In a linear
product of two planar bodies, its edge directions lie in the union of two
complementary planes. Each such plane could contain at most two of the
original eight directions because every original triple is independent.
The required eight directions cannot fit into their union.

## Contact data, normalization, and pairwise deficits

`python3 contact-replay.py` passed, including the newly added assertion that
the saved records cover all unordered pairs from the six nonnegative
additions. The actual data contain exactly fifteen distinct records.

The enumeration returns **32 directions up to sign**, corresponding to
64 norm-ball vertices; the author's corrected wording now says this.
It enumerates rank-three intersections of the eight central hyperplanes
defining `a(u)=||Au||_1+3||u||_1` and checks `b(u)=16a(u)` at each resulting
direction. The quadratic matrix formula agrees with expansion of the
deficit `Phi^2-4Psi`.

I independently verified the normalization used for the saved polynomials.
For normalized directions `v_i=u_i/a(u_i)`, placing weight `8` at each of
the eight original indices recovers the base generators up to sign.
Their segments are unchanged by those signs. Adding parameters times the
saved nonnegative ray coefficients therefore gives exactly the intended
Minkowski additions, with nonnegative total weights for all parameters
`s,t>=0`.

Separately from the author's Pfaffian expansion, I recomputed all fifteen
complete bivariate deficit polynomials using the coordinate determinant
formula `9|det(v_i,v_j,v_k,v_l)|`. Every coefficient agreed with the saved
JSON. Every retained monomial has positive coefficient and positive powers
of both parameters, and each polynomial is nonzero. Therefore every mixed
deficit is nonnegative for `s,t>=0` and strictly positive for `s,t>0`.
The exact polynomial identity establishes this over the full quadrant;
it is not a finite-parameter sampling result.

I also checked the discovery claims that the retained replay does not
explicitly recompute:

- The stored Schur complement is exact, and eliminating the four pivot
  coordinates removes every original-coordinate coupling. Its twenty-four
  remaining coordinates have exactly the twelve stored nonnegative
  two-coordinate null pairs.
- The quadratic form is indefinite. An exact witness is
  `x=ray[0]-(7/60)e_15`, using zero-based indices: `x^T H x=-49/3600`.
  Its negative new-direction coefficient explains why this alone gives no
  feasible counterexample.
- Independent determinant-based one-variable expansion gives identically
  zero deficit on rays `0,...,5`. On rays `6,...,11`, the deficit is
  respectively
  `(9/500,9/2000,9/500,9/2000,1/162,1/162)*t^4`.
  Their new-direction coefficients are nonnegative, and their negative
  coefficients occur only on originally positive lengths. Hence the signed
  variations are feasible for a sufficiently small positive parameter, and
  their strictly positive fourth-order deficits have the stated meaning.

The supporting checks above were bounded exact arithmetic, performed during
this audit without adding an alternative producer to the project. The
retained replay checks the contact enumeration, saved quadratic matrix,
twelve supplied quadratic null rays, and all fifteen mixed polynomials;
it does not currently recompute the Schur complement or the six signed
fourth-order coefficients. This is a replay-scope distinction, not a gap in
the checked mathematical claims.

## Reviewed file identities

The final re-read versions had these SHA-256 hashes:

```
lagrangian-addition.md  72350d8a526c2d2bfd9989c633f403ba2590f946abac2406b1dda6ddd767c0a9
equality-extension.md  a959407885fbce245d961dde8ff5d43a074e2d02aeda673c4d4b7b940c96185c
equality-extension.py  3bd5e2e744ae6efa5915f7b1a7dbcbd9de06831b8c8ea176d36ac7d3e185ebac
contact-replay.py      6a29d170e2d4a07e49dcf34d973151a15160cf3eeeb76d513cc2a47a365a8966
contact-calculus.json  3e545a8c07eaddf10b27b5cd0038ffe0cdd073024a8b5da5f91a54f7cc905c66
```

No substantive mathematical issue remains in these scoped claims. This
acceptance does not extend to general combinations of three or more added
rays, local maximality, or the unrestricted rank-four inequality.
