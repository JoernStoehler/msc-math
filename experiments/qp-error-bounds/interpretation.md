# Interpretation breadcrumb: QP numerical bounds

This is a research breadcrumb, not source truth or a theorem statement.  The
wide-row contract and formula definitions live in [README.md](README.md),
[`analyze.py`](analyze.py), [`coverage_ledger.json`](coverage_ledger.json),
and the 101-entry [`formula_inventory.json`](formula_inventory.json).

## Current packet and hypotheses

The tracked packet under `artifacts/broad/` is regenerated only after the
producer, analyzer, validator, and mutation suite are final.  Its manifest
and `analysis.json` are the quantitative source for row counts and timings;
those counts are packet counts, not prevalence estimates.  Production-
visited/retained sigma states are unavailable per row because the route exposes
aggregate totals only.

Working hypotheses (all provisional):

* A beta inverse-KKT radius is a promising candidate only when its residual,
  inverse, selected center, and exact reference are the same KKT target.  It
  covered 75/75 eligible comparisons here, as did the `1e-9` static beta
  baseline; this is a narrow feasibility result, not a proof.
* The current q residual correction is a solver diagnostic, not a total
  binary64 error bound.  `local.q_residual_diagnostic.v1` covered 744/1,606
  eligible rows (46.3%) and `local.q_correction_quadratic.v1` covered
  714/1,587 (45.0%).  The first-order candidate and raw-Q inverse-radius
  candidate covered 827/827 and 1,163/1,163 eligible rows respectively, but
  those comparisons are still conditional on eligibility and target identity.
* A scalar error comparison cannot certify a predicate, an action interval, or
  a minimizing set unless the relevant endpoint/selection margin and all
  population assumptions are also covered.

## What comparisons mean

For a row with an exact oracle, `E <= B` is evidence that this evaluator
enclosed this target and center on this row.  It does not establish a theorem,
coverage outside the declared population, an online fallback guarantee,
global HK capacity for a capped stream, or transfer to the intended algebraic
HKO input.  A `B/E` tail is a sharpness diagnostic, not a safety margin unless
the evaluator's hypotheses justify it.

An undercoverage row directly falsifies the candidate *as implemented for that
target/center*.  It may identify an omitted rounding term or an atom mismatch;
it does not by itself refute a repaired formula.  Conversely, `unavailable`
means a missing oracle/atom, not a successful bound.  Keep the following
targets separate in every comparison:

1. the original rational input;
2. the exact rational value represented by stored binary64 bits;
3. transformed/preprocessed inputs; and
4. an intended algebraic object, only where a genuine algebraic oracle exists.

The packet has (1) and (2), but no algebraic HKO oracle and no separate
transformed-target population.  A center plus an error scalar is not an
interval; reciprocal action endpoints need positive-q guards and explicit
endpoint atoms.  Population route counts must not be read as per-sigma recall.

## Named counterexamples and current signals

The executable demonstrations in
[`../../dev-quadratic-program/src/route_demonstrations/README.md`](../../dev-quadratic-program/src/route_demonstrations/README.md)
are the source-backed names to keep attached to interpretation:

* `beta_margin_indeterminate`: HKO sigma `[0,1,6,7,3,4,5,9]` is exactly
  beta-positive with a minimum beta around `2e-17`, while f64 reports a zero
  margin/indeterminate predicate.  A strict f64 `beta > 0` filter would lose
  it; this is not evidence that the exact orbit is invalid.
* `near_singular_kkt_false_positive`: HKO sigma `[1,8,7,3,4,5,9]` has f64
  beta predicate true (margin about `0.069`) but exact stored-dyadic KKT
  predicate false.  Its large production q/action uncertainty is the intended
  refusal signal, not a certified capacity candidate.  The rank-deficient
  HKO sigma `[1,7,2,8,4,6,5]` is another f64-true/exact-false boundary row.
* [`q_error_bound_not_certificate.rs`](../../dev-quadratic-program/src/route_demonstrations/q_error_bound_not_certificate.rs)
  shows sigma `[0,1,7,3,9,5]`: f64 beta looks comfortably positive, yet the
  exact stored-dyadic q error exceeds the stored `q_error_bound`.  The scalar
  can be close while the bound is not a certificate.
* [`literal_f64_pruning.rs`](../../dev-quadratic-program/src/route_demonstrations/literal_f64_pruning.rs)
  uses the pruning-roundoff fixture: naive literal f64 pruning selects
  `[0,4,3,1,2]`, while the exact reference minimizer is `[0,3,1,4,2]`.
  Conservative retention repairs this particular pruning miss but remains an
  f64 route, not a proof.
* `hypercube_exact_zero_beta_boundary` has q near `-1.1e-32` and no positive
  action; it tests that zero/negative q is guarded rather than inverted.
  `f64_value_not_certificate` separately records a correct-looking scalar with
  an undecided, multiply-minimizing set.

The latest packet's residual/quadratic undercoverage is concentrated in the
small simplex/hypercube rows: examples have exact Q error around `4e-16` to
`1e-14` while the candidate B is around `1e-26` to `1e-30`.  This is a concrete
signal that residual correction alone omits input/assembly/solve effects.
The two row-level `true|false_unsound` beta (and q) predicate categories are
consistent with the named HKO near-singular/rank-deficient failures; do not
average them away as ordinary noise.

## Adding or refining a bound evaluator

Add a registry entry and evaluator beside the existing local formulas.  Record
expression, target, center, hypotheses, arithmetic model, source, consumers,
implementation status, and a precise unavailable reason.  Compute it for all
rows whose atoms exist, including rows that production would reject; preserve
the proposal and lifecycle state as annotations.  Derived `E` and `B` must
share target and center.

Use analyzer reports to inspect `E <= B`, undercoverage/outliers, `B/E` (with
exact-zero handling), consumer margins, and the same result filtered by all,
maximum-Q, minimum-action, production-visited/retained, minimizer, and
low-action-window cohorts.  Add missing terms or atoms before changing a
threshold.  Separate stored-dyadic and original-rational comparisons, and add
exact-negative, near-zero-beta, q-near-zero/reciprocal-sensitive, and
beta-invalid high-Q/window cases without treating post-selected regressions as
prevalence.

## Consumer questions and missing populations

Before promotion, ask: does the candidate preserve the exact-positive beta
predicate; enclose Q and reciprocal action endpoints; preserve the exact
minimum-action set; and support the intended fallback scope (`BoundSafe`,
`MinimaSafe`, or `AllSafe`)?  What is the consumer margin to the runner-up or
window cutoff, and which sigmas were never observed because route state is only
an aggregate?

Current gaps are explicit in the ledger: no checked-in exact raw-sysext
beta-invalid/high-Q predictor population; no intended-algebraic HKO oracle;
and no prevalence-controlled square-times-square near-zero-Q selector.  The
packet also lacks per-sigma production visited/retained state and complete
transformed-target comparisons.

## Retained-exact route evidence

The focused artifact under `artifacts/retained-exact/` evaluates whether
exact-solving every sigma retained by the current f64 route is usable for
scalar minima, active-word minimizers, and a fixed exact 5% action window.
Regenerate it with `bash experiments/qp-error-bounds/run_retained_exact.sh`;
the producer and mutation validator are `src/retained_exact.rs`,
`validate_retained_exact.py`, and `test_retained_exact.py`.

The four deterministic rows preserve the direct observations. All four
exact-all references are available over the same supplied stream. Retained
exact and exact-all agree on scalar minimum, exact active-word minimizer set,
and exact 5% window in each row. In pinned q4:p5, ordinary f64 minimization
omits tied word `[0,4,1,2,7,6]`, while retained exact includes it. In
triangle×square, 26 of 30 retained candidates are f64-indeterminate and
exact-rejected; the four f64-true candidates are exact-accepted. These are
regression observations, not prevalence estimates.

Timings in `analysis.json` separate candidate generation, ordinary
`MinimaSafe`, retained exact recheck, and exact-all reference. Exact-all is a
reference over the supplied transition/product-block stream, not global HK
recall. Exact rechecking is exact over the retained set and cannot recover
candidates rejected before retention. The target is stored-binary64-rational
(the intended trigonometric/algebraic triangle×square target remains
unavailable). The result bears on scalar/minimizer/window consumers only; it
says nothing about exact multipliers, derivatives, recovery, or production
consumer replacement.

The ordinary current-vs-retained comparison is a diagnostic, not an API claim:
the current scalar is the production `MinimaSafe` f64 `min_action`; its
diagnostic minimizer grouping uses absolute action tolerance `1e-12`; and its
5% window is independently formed from the current f64 minimum as
`action <= (21/20)·min_action` over the `MinimaSafe` returned list. Current
scalar agreement uses absolute f64 tolerance `1e-12`; current minimizer/window
agreement uses canonical sigma-set equality (serialization order is not
semantic). Retained-vs-exact-all agreement uses exact rational equality and
canonical sigma-set equality, and is explicitly conditioned on the supplied
stream reference.

For exact rejection, the API exposes only “no admissible positive-Q witness”.
The packet records that narrow unavailable reason without guessing whether the
cause was a singular/inconsistent system or nonpositive beta/Q.

The runner requires a clean producing tree and records a reachable source
commit plus full git tree identity. The artifact can be committed afterward as
a separate child commit; provenance refers to the producing source snapshot,
not recursively to generated outputs. Row timers exclude compilation,
fixture/exact-geometry setup, and Python analysis/validation; exact-all timing
includes complete-stream enumeration, exact solves, and sorting.

Remaining high-value unknowns are scaling on larger candidate streams,
indeterminate prevalence outside these fixtures, and whether an intended
algebraic HKO oracle changes any decision. This packet does not claim them.

## Multi-centre soundness and production-policy trial

The separate v2 packet under `artifacts/soundness-v2/` is the current trial
architecture for formula search. It contains 1,593 same-word rows across ten
declared streams, with explicitly named production saddle, SVD, projected,
LU, QR, and refined centres. Formula evaluations use a long-form registry
schema: the formula ID, centre, exact target, hypotheses, availability, value
kind, unit, and role remain explicit rather than being inferred from a generic
`q` or `error` column. Exact lifecycle states are also separate: this packet
observed 109 inconsistent systems, 1,333 consistent systems with no strictly
positive beta, 87 positive-beta/nonpositive-Q systems, and 64
positive-beta/positive-Q-action systems. These are selected packet counts, not
prevalence estimates.

The main formula observations are discriminating rather than promotional:

* The residual-squared/eigenvalue Q candidate is empirically unsound on 38--50
  of 60 exact-covered rows, depending on the solver centre. Typical failed
  bounds are around `1e-28` while the observed Q error is around `1e-15`.
  Its derived reciprocal-action intervals therefore also fail frequently.
  This supports the hypothesis that a proof-relevant formula must include
  input/assembly rounding and not only the residual of the rounded system.
* The inverse-singular-value beta-radius diagnostic and its
  first-plus-quadratic Q propagation covered all 60 unique exact positive-Q
  comparisons for each applicable centre. The radius-derived ternary beta
  predicates were sound on all 3,412 eligible comparisons: 2,368
  `false|false`, 744 `indeterminate|false`, 295 `true|true`, and 5
  `indeterminate|true`. This is high-value evidence for the shape of a bound,
  not a certificate: the inverse, singular value, residual, and assembled
  matrices are ordinary binary64 quantities, and unavailable rows are not
  counted as successful decisions.
* The one-shot f64-anchored exact fallback is not a generally usable policy.
  On the capped hypercube and triangle-times-square streams it can exact-reject
  every selected low-action candidate while a valid exact minimum exists
  outside the selected set. A useful selective policy must expand after exact
  rejection or stop only when proven action intervals exclude every remaining
  competitor.

The packet directly invokes the production candidate conversion and
`MinimaSafe` aggregator, separately from an unchecked saddle-feasible
no-fallback diagnostic. On the named stored-binary64-rational HKO
near-singular and rank-deficient words, production returns an
`AdmissibleF64` scalar near `3.44095480117793` although exact retained and
exact supplied-word replay find no positive-Q action. These are genuine
fixed-word/current-policy false acceptances. They do not by themselves show
that the full-polytope HKO capacity is wrong: the packet contains one named
word in each case and does not establish whether another valid word shadows
the false candidate in the complete candidate family. The HKO target is the
exact rational represented by stored binary64 coordinates, not the intended
algebraic polytope.

For the ordinary generated F5, capped simplex F5 and hypercube F8,
triangle-times-square, and pinned q4:p5 streams, the production scalar agrees
with exact supplied-stream replay within `1e-12`. Scalar agreement does not
extend to exact active-word ties. At zero gap production reports three q4
minimizer words where exact replay reports four; simplex, hypercube, and
triangle-times-square likewise lose exact-equality ties. The q4 relative
windows agree for positive tested gaps but not at zero; the triangle 20%
window also differs. `MinimaSafe` is therefore evidence about its scalar
contract, not a certified minimizer-set or low-action-window contract.

The representative v2 run takes about 30 seconds because exact row reduction
and positive-witness search dominate. All broad f64 solver variants together
take under 0.2 seconds. On q4:p5, direct production candidate solving takes
about 54 ms and `MinimaSafe` aggregation after candidate construction takes
microseconds; exact resolution counts are unavailable from the public
production result and are recorded as unavailable rather than zero. This
supports selective exact work, but only behind predicates and action intervals
whose soundness has actually been proved.

The next proof-search target should therefore combine: (i) residual and
conditioning terms for one named centre; (ii) explicit binary64 assembly/input
perturbation terms; (iii) beta, Q, and reciprocal-action propagation with
positive-endpoint guards; and (iv) a ternary rule that returns indeterminate
when any hypothesis or enclosure is unavailable. Rank-deficient critical sets
need a separate projected-set certificate or exact fallback, not a forced
inverse formula. Candidate-family enumeration/pruning soundness and the
consumer-specific scalar/minimizer/window aggregation argument remain
separate proof obligations.

## Promotion path

Empirical 100% is a falsification-screen result.  Promotion requires (i) a
fully specified target and rounding model, (ii) all perturbation/input/solver
terms and endpoint guards in the evaluator, (iii) exact or independently
verified oracle coverage over adversarial and ordinary strata, (iv) a proof of
the inequality and its hypotheses, and (v) a separate route-level argument
showing which candidate population and fallback mode the proof covers.  Only
then should the result move from this breadcrumb into `formal/` or a
production guarantee; algebraic-HKO claims additionally require a genuine
algebraic oracle.
