# Current four-dimensional QP result

Status: pre-production research result, measured locally in release mode on
2026-07-23. The numbers below come from a fresh build of the current source.

## Result

Under its stated arithmetic and candidate-stream assumptions, the current best
correctness-guaranteed general route combines:

- exact/outward handling of words shorter than five;
- all-length `LBL^T` factorization;
- certified positive-curvature discovery and cyclic-order inheritance;
- curvature rejection before constructing an unused solution and inverse;
- a cheap normwise verified inverse-defect enclosure, followed only when
  necessary by the tighter batched enclosure;
- exact rational fallback only when the enclosure is indeterminate; and
- outward aggregation from candidate `Q` intervals to a capacity interval.

The product branch should continue using billiard enumeration and a
null-space-capable inner solver. Running exact products through the general
inverse-based route is correct but needlessly expensive. The current product
solver is a fast fixture-backed control, not newly certified for every product
input by this packet.

The faster empirical predicate is not the selected algorithm. Its ordinary
rounded residual and inverse-norm quantity has no sound enclosure theorem, and
exact fallback only on its indeterminate cases does not make determinate
answers safe.

## Verification

The verification packet compared the complete pruned route against exact
binary64 rational arithmetic.

| Cohort | Cases | Words | Capacity misses | Minimizer-class misses | Exact fallbacks |
| --- | ---: | ---: | ---: | ---: | ---: |
| generated general F5--F7 | 3 | 88 | 0 | 0 | 0 |
| billiard product control | 3 | 1,271 | 0 | 0 | 698 |

For every case, the exact capacity lay inside the propagated capacity interval.
The largest difference between the printed central value and exact capacity on
the general cases was `2.31e-14`.

Exact general enumeration and billiard enumeration agreed on the two small
products where enumerating both was cheap. The existing product solver agreed
with exact aggregation on the triangle product, triangle-square product, and
hypercube; its largest absolute error was `4.44e-16`.

Validation rejected duplicate dual vertices, near-redundant input, and the new
dual/primal infinity-norm range controls. Missing origin interior requests
fallback or rejection according to the existing validation policy. A drifted
non-product control remained accepted and was audited exactly.

## Numerical audit

The staged normwise-then-batched general predicate was compared with exact rational
arithmetic on 249 systems:

- 97 ordinary generated systems;
- 108 systems under scales from `1e-2` through `1e3`, including deliberate
  stresses outside the accepted input range; and
- 44 systems approaching singularity down to binary64 coincidence.

It produced:

- zero certified-true/exact-false decisions;
- zero certified-false/exact-true decisions;
- 15 indeterminate decisions, all resolved exactly;
- zero beta-radius violations;
- zero `Q`-radius violations;
- maximum observed beta-error/radius ratio `0.00149`; and
- maximum observed `Q`-error/radius ratio `0.00081`.

The normwise stage alone became indeterminate on all 27 sampled systems at
scale `1e3`; resolving those directly in exact arithmetic took about `4.9 s`.
The tighter second stage decided all 27, so the selected staged route used no
exact fallback there. The 15 remaining indeterminate systems all belong to the
deliberately near-singular cohort.

The product numerical control compared 178 sampled systems. It again found no
wrong determinate decision or radius violation, but returned indeterminate 150
times. This supports the product/general route split.

Focused tests separately cover:

- both matrix-matrix and matrix-vector multiplication paths;
- the final right-hand-side and identity subtraction;
- gradual-underflow allowance;
- exact rational-to-binary64 enclosure, including positive underflow;
- cyclic rotations versus reversed cyclic order; and
- maximum-`Q` capacity interval aggregation.

These finite comparisons test the implementation. The soundness claim rests on
the formal lemmas and pinned arithmetic contract, not on a lack of observed
counterexamples.

## Performance and alternatives

The comparable long-word cohort contains 13,891 systems. Medians are from nine
interleaved rounds.

| Variant | Median | Status |
| --- | ---: | --- |
| previous empirical inverse check | 75.56 ms | heuristic baseline |
| scalar outward enclosure with all-length pruning | 87.48 ms | verified control |
| batched enclosure with all-length pruning | 75.96 ms | tighter verified stage |
| normwise enclosure with all-length pruning | 40.19 ms | cheap verified stage |
| staged normwise then batched enclosure | 40.43 ms | selected general route |
| empirical predicate plus lazy exact fallback | 29.82 ms | fast but unsound |

The selected route is `1.87x` faster than the previous empirical baseline on
the same long-word cohort. The full general profile, including 350 short
words, took `45.33 ms` for 14,241 words and used no exact fallback.

These route timings start after validation and candidate construction. A
separate nine-round end-to-end profile on the same eight F5--F12 inputs gives:

| Route | Validation | Exact transition and cycles | Candidate processing | Total |
| --- | ---: | ---: | ---: | ---: |
| previous empirical inverse check | 1.26 ms | 20.06 ms | 76.51 ms | 97.89 ms |
| batched verified route | 1.26 ms | 20.12 ms | 80.30 ms | 101.70 ms |
| selected staged verified route | 1.28 ms | 20.09 ms | 44.64 ms | 65.98 ms |

The selected route is therefore `1.48x` faster end to end on this cohort.
It also processes 350 short words by one-sided outward rejection; the previous
empirical reproduction omits words shorter than five.

Holding the numerical predicate fixed exposes the combinatorial gain. The
previous route factors and tests all 13,891 long-word systems. Curvature
inheritance reduces this to 4,535 factorizations and 3,817 beta/Q guards:
67.4% of factorizations are skipped. With the old empirical predicate on that
same pruned route, candidate processing takes 29.82 ms. The selected verified
check spends about 12.07 ms on the 3,817 survivors: 4.79 ms constructing norm
bounds, 0.77 ms on residuals, 5.53 ms on inverse defects, and 0.17 ms on final
decisions. Moving curvature rejection before the solution and inverse avoids
wasted work on 718 direct obstructions and saves about 1.3 ms.

The empirical control is `2.4x` faster than the selected route, but that is not
a valid correctness/performance trade: it can make an unrepairable wrong
determinate decision.

Earlier ablations also reject these alternatives:

- allocation-heavy cyclic-subsequence lookup dominated runtime; the retained
  mask-and-position lookup gives the same result without that cost;
- obstruction cutoffs eight, nine, and all-length remain within about one
  percent, so the all-length route removes an unnecessary policy and backend
  split;
- scalar outward defect evaluation spends most of its time constructing the
  inverse defect; batching removes about one fifth of total time;
- four auxiliary batched products erased the combinatorial speedup; replacing
  them by proved induced-norm bounds restores it, while staged retry retains
  the tighter scale behavior;
- packing the solution and inverse right-hand sides into one factor solve
  changed runtime by less than measurement noise and was discarded;
- a manual small-matrix defect loop raised that phase from about 5.4 ms to
  22.4 ms and was discarded;
- unchecked direct solves are faster but have no safe predicate;
- the symmetric eigensolver is slower and has known scale-dependent errors.

For products, the general control took about `650 ms` over the three fixtures,
including 698 exact fallbacks. The existing product solver medians were
approximately `0.16 ms`, `2.09 ms`, and `34.44 ms`. Code uniformity would not
justify that regression.

### Product near-singularity follow-up

A disposable follow-up tested whether the product branch could replace the
full-KKT eigensolver by solving the max-margin linear program directly. It
lost. On the 4,051 exact-transition HKO product words, Clarabel took about
`565 ms`, versus `190 ms` for the old KKT eigensolver and `103 ms` for the
existing constraints-first projected solver. With a `1e-14` dead zone, the LP
identified the same 300 exact-positive robust words as the projected solver;
it added no useful positive decisions and still produced 320 exact-false
positive labels before certification. The exploratory executable was removed.

The more informative comparison used the proved product-feasibility and
curvature pruning first. It reduced the production-shaped 6,240-word HKO stream
to 285 survivors. Exact binary64 arithmetic found:

- all 285 KKT matrices were consistent and exactly full-rank;
- 115 unique solutions had positive beta and positive Q; and
- 170 unique solutions were inadmissible.

Thus the old solver's problem here is not genuine exact degeneracy. Its strict
eigenvalue tier treats small but nonzero eigenvalues as a null space and shifts
beta: it labelled all 285 survivors positive, including all 170 exact-false
cases. The projected solver labelled 254 positive, including 139 exact-false
cases.

A plain LU solve took about `0.6 ms` for all 285 survivors and agreed with exact
arithmetic on 273: 115 positive and 158 negative. It returned indeterminate on
10 exact-negative systems and made two false-positive decisions. Those two
systems had exact beta margins of order `-1e15` to `-1e16`; their f64 inverses
could not be verified. Exact fallback for exactly these 12 diagnostic failures
cost about `87 ms`.

Neither the uniform verified-inverse radius nor an exploratory componentwise
Neumann enclosure certified any of the 170 negative systems; both safely
returned indeterminate. Exact-solving all 170 took about `1.4 s`, and
exact-solving all 285 survivors took about `2.0 s`. Therefore neither direct LP
nor ordinary f64 inverse certification is the product solution.

A subsequent fraction-free integer spike solved the full-rank dyadic systems
exactly with zero classification or solution mismatches and was about seven
times faster than the generic rational solver. On all 4,051 exact-transition
HKO words it still took `5.39 s`, versus `38.58 s` for the generic rational
route, and 772 words were genuinely singular and still required the null-space
fallback. This remains the wrong runtime scale, so the spike was removed before
further optimization. The retained research direction is to simplify the QP
linear algebra using the Lagrangian-product block structure, not to pursue a
precision ladder or cheaper generic exact arithmetic.

The next product-specific direction comes from separating q- and p-facet
weights. Their closure constraints are independent, while the product QP
objective is bilinear between the two groups. Writing the total q-weight as
`s` and normalizing the q- and p-weights separately gives
`Q = s(1-s) B(alpha, gamma)`. For positive `B`, the mass variable optimizes at
`s = 1/2`.

This gives a six-facet bound for the capacity value, independently of the
twelve-facet billiard bound. Start with a global product-QP maximizer. With the
p-weights fixed, maximize the resulting linear functional over the normalized
q-closure polytope and choose a vertex maximizer; then do the same for the
p-weights. A vertex of either planar normalized closure polytope uses at most
three facets. The value cannot increase strictly because the starting point
was already globally maximal. Deleting the zero-weight positions preserves
the closure equations, cyclic order of the retained positions, and every term
of `Q`. The reference Haim--Kislev family contains the resulting cyclic word,
so a capacity maximizer exists with at most three q-facets and three p-facets.
This argument does not claim that every minimizing orbit has at most six
facets, or that arbitrary deletion preserves the syntax of the billiard block
enumerator.

The corresponding capacity algorithm should enumerate the vertices of the two
planar closure polytopes (positive closing pairs and triples), pair them,
enumerate the at most `5! = 120` cyclic orders of their combined support, and
evaluate the bilinear objective directly. It needs no KKT solve. A disposable
HKO check found five closing triples in each factor, hence only
`5 * 5 * 120 = 3,000` direct objective evaluations, and recovered
`Q = 0.14530850560107225` and capacity `3.440954801177932`. This independently
matches the full route. The earlier length-at-most-six check over 1,005
billiard-emitted words found the same value.

## Independent review

A fresh read-only adversarial review found four issues, all corrected before
the final evidence run:

- LLVM had constant-folded away the gradual-underflow check. The route now
  tests flush-to-zero and denormals-are-zero with opaque runtime operands once
  per route. When the arithmetic contract is absent it bypasses short-word,
  curvature, normwise, and batched floating-point checks and exact-resolves the
  original candidate stream. Release disassembly was checked for the two
  multiplications and conditional branch.
- The proof named `nalgebra 0.35.0` as the certified multiplication
  implementation. The products actually use the pinned workspace
  `nalgebra 0.33.3`; `0.35.0` supplies only Bunch--Kaufman factorization. The
  formal contract and reviewer guide now say this explicitly.
- Validation documented but did not enforce the sixteen-facet boundary. It now
  rejects larger inputs before the fixed-size label bitmask, with a regression
  test.
- Zero mismatch counters could pass on an empty cohort. Predicate,
  route-agreement, and product-agreement audits now require positive comparison
  counts.

A second read-only review covered the new normwise formula, global omega
assembly bound, staged fallback, and early curvature rejection. It found no
runtime soundness error. It did find two formal-description defects, both
corrected: the central residual and defect now explicitly include their final
rounded subtraction, and the staging remark now records factorization failure
and unsupported arithmetic as earlier exact-fallback paths.

The general F5--F7 fixtures contain no exact-positive candidate removed by
curvature pruning. The product hypercube control contains 128 such candidates,
so the end-to-end pruning comparison is exercised, but not on the random
general cohort.

## Remaining scope

This packet is ready for adversarial review, not production migration. The
review should try to falsify:

1. the normwise and batched rounding and underflow contracts;
2. the verified-inverse and `Q = -xi/2` propagation;
3. certified curvature projection and cyclic inheritance;
4. maximum-`Q` capacity interval aggregation;
5. exact fallback boundaries and rational conversion; and
6. the distinction between general and exact-product routes.

After review findings are resolved, the next action is to design the production
general API and migrate one consumer while preserving this packet as
verification, numerics, and performance evidence. The product branch separately
needs output-relevant exact resolution or a certified singular-capable wrapper.
