# Scientific interpretation

Interpretation date: 2026-07-13. This interpretation uses the reviewed v2
artifacts identified by `artifacts/provenance.json`. It does not rerun the
fixed-sigma producer or alter generated artifacts.

## Question and evidence slice

For each retained random planar product, let `A2` and `A3` be the exact minimum
actions among the existing solved two- and three-bounce candidate streams, and
write

```text
g = (A3 - A2) / A2.
```

The packet asks whether small `|g|` is associated with an A2/A3 minimizer pair
using the same six facets in a different cyclic order. The evidence slice is
the seed-42 random-product generator with 1,024 accepted rows in each
`3 <= k <= m <= 6` bucket and support heights in `[0.8,1.2]`. Of 10,240 rows,
9,455 have both class minima; the 785 A3-null rows are availability states and
are excluded from every numeric gap and shuffle comparison.

The measured objects are exact KKT candidate words and their exact actions.
They are not automatically a classification of all physical billiards. The
primary same-support words did pass the retained recovery checks, but the
packet contains no valid physical contact-distance measurement.

## Observations

### Support sharing is enriched near class ties

- 3,559 of 9,455 complete rows (37.64%) have at least one exact A2/A3
  minimizer pair with the same unordered six-facet support.
- Among the 1,899 rows with `|g| <= 0.01`, 937 (49.34%) share support, versus
  2,622 of 7,556 (34.70%) in the complement.
- At the tighter `|g| <= 0.001` threshold, 184 of 334 rows (55.09%) share
  support, versus 3,375 of 9,121 (37.00%) in the complement.
- At `|g| <= 0.05`, the corresponding rates are 42.78% and 31.99%.
- In each of the nine nontrivial buckets other than 3x3, the support-sharing
  rate among the 1% near ties exceeds that bucket's complete-row rate. The 3x3
  bucket is not evidence for this comparison: every six-facet word necessarily
  uses all six available facets.

The deterministic complete-row, within-bucket shuffle gives mean support
equality 12.65% and 95th percentile 13.00%, compared with the observed 37.64%.
This control shows that the observed minimizer supports agree much more often
than supports paired between unrelated retained rows from the same bucket. It
is a finite deterministic comparison, not a sampling-theory p-value.

Support sharing is therefore a real organizing feature of the retained near
ties. It is not a universal explanation. Even at `|g| <= 0.01`, 962 of 1,899
rows have different supports. Moreover, the overall support-sharing rate falls
from the mechanically forced 100% in 3x3 to 17.38% in 6x6 while the bucket
median `|g|` falls from 0.0690 to 0.0233. Shared support alone cannot explain
the facet-count trend.

### Every retained same-support pair is one cyclic-order swap

The 3,559 same-support rows yield 10,471 pair records because exact tied
representatives are retained pairwise. There are three records on 3,353 rows
and two on 206 rows; these records are not independent observations.

After independently rotating both cyclic words and applying the retained
deterministic alignment rule, every one of the 10,471 pairs differs by exactly
one nonzero q/p order term. The earlier disposable recount with 3,064 one-term
and 495 two-term rows held the stored A3 cut fixed. It is obsolete for cyclic
words; the symmetric convention is the appropriate durable convention.

Tie multiplicity does not create a competing row-level result here. The class
gap range is zero on every row with pairs, and the retained beta-product range
is also zero. The pair multiplicity should still be preserved because it
records distinct cyclic representatives and prevents pair counts from being
mistaken for row denominators.

### Both exact factors vary with the gap

For the one-swap pairs, the median beta product is approximately 0.00575, the
median normalized pairing factor is approximately 6.95, and the median
absolute normalized gap is approximately 0.0320. Across the ten buckets, the
pair-level Spearman association with `|g|` is 0.639--0.762 for the beta product
and 0.389--0.514 for the normalized pairing factor.

The beta association is consistently stronger in this descriptive summary,
but it does not establish that beta is the primary cause or mechanism. The two
quantities are exact multiplicative factors of the target, were computed after
the class minima were known, and are reported on repeated pair representatives.
Raw beta size also depends on the normalized facet presentation. The supported
observation is only that the near ties in this same-support stratum tend to
have smaller beta products and, less strongly, smaller normalized pairings.

### Recovery and eight-facet controls

All 10,471 primary pair records pass the fixed recovery thresholds; the maximum
retained closure error is about `2.53e-15`. This removes the narrow concern that
the primary algebra is built entirely from unrecoverable KKT words. It does
not measure contacts approaching one another and does not support a physical
closure route.

There are 321 rows containing 690 eight-facet A3 minimizer words. Every measured
exact KKT kernel dimension is zero. Thus the anticipated rank-deficient,
nonunique-beta explanation is falsified for these stored words. They remain
outside the six-facet primary analysis because their support and block
combinatorics differ, not because their KKT systems are singular.

## Exact algebraic identities

For a same-support pair, let `Q2` and `Q3` be the exact Haim--Kislev quadratic
values, so `A_b = 1/(2 Q_b)`. The producer verifies exactly that

```text
g = A3/A2 - 1 = Q2/Q3 - 1 = (Q2 - Q3)/Q3.
```

Under the symmetric cyclic alignment, exactly one q/p facet pair `(i,j)`
changes relative order. Using the exact A2 weights recorded for those facets,
the retained pair artifact satisfies

```text
|g| = (beta_i beta_j) * (2 |omega(a_i,a_j)| / Q3).
```

The final lead verified over all 10,471 pair records that the exact stored term
equals the exact stored `Q2-Q3`, and that the recorded beta factors multiply to
the recorded beta product. This is the strongest mathematical result of the
packet. It says that, on the observed same-support stratum, the two class
values are tightly coupled by a one-term exact decomposition on a common
active support.

The packet does not assert facetwise equality of the A2 and A3 beta vectors.
The decomposition therefore must not be read as a fixed-beta perturbation from
one physical orbit to the other.

The factorization is an identity, not independent predictive evidence. It
cannot by itself decide whether the small gap is caused by a support weight
approaching zero, a weak symplectic pairing, or a geometric degeneration.

## Inferences and current beliefs

1. **Shared-support cyclic-order competition is supported for a substantial
   subpopulation.** Confidence is high (about 90%) for this generator-local
   organizing statement because the support enrichment, exact one-swap
   identity, complete-only shuffle, and tie stability all point in the same
   direction.
2. **The original “two independent bottlenecks” framing is poor on that
   subpopulation.** The common support and one-term exact difference give a
   much tighter description. This conclusion does not extend to the 5,896
   complete rows without equal support.
3. **A beta-boundary mechanism remains possible but unestablished.** The beta
   product has the stronger component association in every bucket, so the
   algebraic beta/order branch survives as a hypothesis. Confidence that this
   is a presentation-independent geometric mechanism is only about 30--40%:
   no independent predictor, perturbation, or physical boundary distance was
   measured.
4. **Weak pairing is a coexisting rival, not a winner or a null.** Its
   association is positive in every bucket. Because beta product and pairing
   multiply exactly to the target, this packet cannot attribute relative
   causal importance to either.
5. **Support turnover remains necessary for a global account.** Different
   supports form the majority of complete rows and a slight majority even at
   the 1% threshold. Candidate density, support turnover, or another class
   relation may explain the remaining facet-count trend; this packet does not
   discriminate them.

## Original outcome branches after review

- **Branch A, beta/order degeneration:** survives only in weakened algebraic
  form. Support sharing and one-order-swap structure are strongly supported;
  beta products are descriptively smaller near ties. “Beta causes the
  degeneration” is not supported.
- **Branch B, weak pairing:** remains live alongside beta. It does not win the
  component comparison, but the comparison is not mechanistically
  identifying.
- **Branch C, cancellation:** not supported in the durable primary convention.
  Every pair has one changed term and cancellation ratio one. The earlier
  two-term rows were an artifact of holding one cyclic cut fixed.
- **Branch D, contact merging:** omitted. The attempted contact quantity did
  not implement the predeclared factor-vertex/edge-endpoint distance and was
  removed before interpretation. Recovery validity is not a replacement.
- **Branch E, support turnover:** survives. The shared-support account applies
  to 37.64% of complete rows and 49.34% of 1% near ties, not to the remainder.
- **Branch F, measurement failure:** does not apply to the repaired algebraic
  packet. The exact identities, populations, closure errors, source hashes,
  and controls needed for the bounded algebraic claim passed review. The
  physical measurement failed its design contract and was correctly omitted.

The anticipated eight-facet rank-deficiency rival is separately falsified:
all 690 measured kernels have dimension zero.

## Failed and omitted claims

This packet supplies no evidence for:

- physical three-bounce trajectories converging to two-bounce trajectories;
- vanishing or repeated contacts;
- a universal two-/three-bounce closure theorem;
- a causal explanation of the class gap or its facet-count trend;
- near ties as the explanation of the high-`sys` tail;
- a new candidate proposer, counterexample, or source of `sys > 1`;
- a claim about A3-null rows beyond the existing solved-stream availability
  contract;
- independence or statistical generalization beyond the named retained
  generator.

The omitted factor-swap fixture is not needed for the exact within-artifact
identity, but it limits claims that the descriptive component summaries have
been independently checked under factor relabeling.

## Strongest supported claim

Among the 9,455 retained random-product rows with both exact class minima,
equal A2/A3 facet support is enriched among small class gaps. On all 3,559
equal-support rows, every retained exact minimizer pair can be cyclically
represented with one changed q/p order pair, and its retained exact
decomposition has `Q2-Q3` equal to the corresponding single term formed from
the recorded A2 beta factors. This gives an exact algebraic description of
class coupling on a substantial generator-local subset, but no fixed-beta,
physical, or causal degeneration mechanism.

## Thesis relevance and disposition

The result is thesis-useful if the experimental chapter needs to explain why
the two finite bounce classes often track one another: it replaces vague
“correlated bottlenecks” language by a concrete common-support, one-cross-term
description. It is supporting structure about the capacity computation, not
evidence for Viterbo's inequality, positivity, or the search for a new
counterexample. Any thesis use should state the retained-generator boundary
and keep the exact identity separate from the empirical support-enrichment
observation.

This line should stop now. The packet has answered the bounded algebraic
question, killed cancellation and eight-facet singularity in the retained
primary setting, and exposed that the desired physical claim lacks a valid
measurement. More random-table diagnostics would have low expected thesis
value without a named physical or theorem-facing use.

## Reopen conditions and future discriminators

Reopen only if one of the following becomes a concrete thesis need:

1. A physical closure claim is needed. Then use a separately designed
   one-parameter family or structured control and measure actual factor
   vertices, edge endpoints, and contact positions before inspecting class
   gaps. Do not revive the removed breakpoint interpolation.
2. The generator-local result must generalize. Then freeze the current
   same-support/one-swap rule and test it on an independent seed, changed
   support-height distribution, or structured regular/polar family, with the
   current generator retained as discovery data.
3. The facet-count trend must be explained. Then target the different-support
   majority with a predeclared support-turnover or candidate-density quantity;
   do not add another beta proxy to the present packet.
4. A theorem-facing statement becomes valuable. Then prove a symbolic
   six-facet common-support order identity and identify hypotheses under which
   the relevant beta product or pairing must be small. The current exact rows
   may serve as examples, not proof.

## Reproduction and source truth

Detailed values live in `artifacts/summary.json` and the pair/row JSONL, not in
this file. Input/output hashes, source hashes, the fixed-sigma command, and the
587.52-second full runtime are recorded in `artifacts/provenance.json`.
`README.md` gives the reproduction commands. The current packet is ready for
further research reuse under the boundaries above; thesis wording would still
require Jörn/Kai selection and normal thesis review.
