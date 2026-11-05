# Independent mathematical review — PASS

Reviewer: /root/intake_review. Read-only review of PLAN.md,
independent_verify.py and the accepted
topics/products/product-qp-six-facet-reduction.tex. No experiments rerun.

## Construction and invariant scope

The construction algebra is valid. Reflection in the y-axis fixes a y-axis
symmetric A and sends an x-axis symmetric B to −B, hence maps A+B to A−B
and preserves area. The two difference bodies are equal, since each is
(A−A)+(B−B). When summand edge directions have no parallel overlaps, the
Minkowski edge lists agree up to signs on B's edges and a permutation.
Thus every mixed ridge area |e_i·f_j| against a fixed partner Q agrees as a
multiset. The n+m factor-face ridges have zero symplectic area; there are nm
mixed ridges, possibly with further zeros. Matching the entire list and volume
therefore matches its volume-normalized version as well.

A separated pair rules out a universal **exact** systolic formula from that
multiset, including symmetric value-dependent weighting or any function of the
sorted values and multiplicities, even when supplied the common volume or
common factor difference bodies. It does not rule out useful approximation,
geometry/incidence-dependent weights, signed data, or all weighted ridge methods.
It says nothing specifically about the upper tail unless the actual values
place it there. Threshold discrimination is obstructed only for thresholds
strictly between the two values (with endpoint conventions specified).

For systolic values 0<a<b, one common scalar estimate has error at least
(b−a)/2 on one example. The best worst target-relative absolute error is
(b−a)/(a+b), attained at 2ab/(a+b). The smallest symmetric multiplicative
factor guarantee is sqrt(b/a). These quantify a limitation rather than saying
every approximate predictor fails.

## Independent verifier completeness

The verifier checks distinct vertices, strict positive turns, positive area,
every edge's genuine supporting inequality with exactly its two endpoints,
and positive support numbers. Together these establish the genuine convex
irredundant polygon geometry and origin interior. Every normal is nonzero.
Its all-pairs hull comparison separately checks common difference bodies.

All normalized closure vertices have at most three positive coordinates by the
accepted planar-product theorem. Singletons are impossible because normals
are nonzero. The verifier enumerates every two- and three-index support,
uses exact Fraction RREF, requires full column rank, consistency and strictly
positive weights, and checks mass/closure. Rank-deficient triples cannot be
vertices; their smaller positive supports are separately included. Its RREF
weight extraction is valid because every column is a pivot in accepted cases.

Every support pair is lifted with factor masses exactly one half. All cyclic
word representatives fix local label zero; the accepted cyclic-cut lemma
justifies this reduction. Both reversal orientations remain included. The
direct four-dimensional omega sum uses the stated HK sign convention. All
objectives and comparisons are exact. Hence its maximum equals Qmax and
c=1/(2Qmax) computes both exact capacities, conditional on the imported accepted
HK/product reduction. Feasible witnesses alone would not suffice; this complete
enumeration supplies the other inequality.

## Exact returned example

Read result.json, independent-verification.json and input-independent.json.
The returned seed uses A=conv{(−2,−1),(2,−1),(0,3)},
B=conv{(−1,−2),(3,0),(−1,2)} and
Q=conv{(−1,−1),(4,−1),(−1,2)}. The six listed P+ vertices are the successive
extreme sums of A+B; the listed P− is precisely their y-axis reflection.
The stated symmetry construction therefore matches the actual rational inputs.

The common P area is 44, Q area is 15/2 and product volume is 330. There are
27 two-faces: nine factor-face zeros and eighteen mixed edge-product faces,
two of which also have zero symplectic area. Thus the eleven zeros in the
complete matched list are correct. The result field
`nonzero_or_cross_ridge_count=18` should be described as **mixed-ridge count**,
not nonzero-ridge count; only sixteen entries are nonzero.

Both factors' closure counts agree across the two implementations: 8×1
vertices, all size three, with no positive size-two closures. Each capacity
therefore requires exactly 8×120=960 cyclic words. The independently returned
Qmax values 31/1200 and 37/1350 give capacities 600/31 and 675/37 by the
accepted convention. Half-factor weights in both witness records sum to one
and exactly match the normalized producer weights divided by two.

The repository systolic ratios are c²/(2V), namely 6000/10571 and
30375/60236. The worker's `sys2_plus`/`sys2_minus` fields denote these ratios,
not their squares. Their quotient is 87616/77841; the larger exceeds the
smaller by 9775/77841, about 12.56%. They force a worst target-relative
absolute prediction error of at least 9775/165457, about 5.91%, on this pair.
Both examples are below one. They neither test the actual above-one upper tail
nor rule out valid capacity upper bounds or useful approximate predictors.

Final verdict: **PASS** on the exact construction, complete-ridge accounting,
same-volume/difference-body claims, and exact capacity computation conditional
on the accepted HK/product theorem. No checks were rerun and no new
experiment was performed for this independent review. External novelty and
human mathematical acceptance are not claimed. Final user-visible wording
must preserve the exact-reconstruction versus approximation distinction above.
