# KKT Trust Search Ledger

This file records the current search state for a trusted f64 or f64+fallback
QP/KKT route. It is not a thesis claim and not a proof file. Use it to resume
the search without re-running old false starts or losing useful ideas.

Source status:

- Current compact audit code lives in this packet.
- The broader scratch source was
  `.worktrees/f64-error-bound-audit/experiments/dev-f64-capacity/kkt_error_audit/`.
- The scratch result cache was `/tmp/f64-policy-final/`; treat it as a
  regression oracle, not tracked source truth.
- These scratch paths are historical pointers only. They may be absent after
  workspace cleanup; do not try to recover deleted worktrees before resuming.
  The durable resume state is this ledger, `README.md`, and
  `docs/route-consumer-matrix.md`.

## Design Pattern Inventory

These are patterns worth using in the trusted route search. They are not a
complete workflow and should not be treated as rules that always dominate local
clarity.

| Pattern | What it means here | Why it matters |
| --- | --- | --- |
| theorem-backed ternary predicates | Computed data values are `True`, `False`, or `Indet`; `True` and `False` require proof, `Indet` makes no claim. | Prevents f64 predicates from silently becoming guesses. |
| propositions as comments/contracts | Always-true facts, possibly conditional on control flow, are written near the code that uses them. | Keeps proof facts distinct from runtime data. |
| proof-carrying error radii | Compute an error bound only when its hypotheses are known; otherwise return unavailable/`Indet`. | Avoids using a formula outside its proof domain. |
| exact binary64 oracle | Compare f64 output against exact rational arithmetic on the exact same binary64 input. | Falsifies numerical predicates without conflating input-rounding error. |
| capacity isolation | A `True` sigma certifies audited capacity only if its action upper bound beats every not-`False` competitor's lower bound. | Distinguishes harmless `Indet` rows from capacity-relevant ambiguity. |
| branch-local reasoning | A proposition can become available inside a branch, e.g. after checking `delta < 1`. | Lets code use conditional proof facts without making global claims. |
| control-flow simplification | Combine ternary values and proof facts explicitly, e.g. `Indet AND False = False`, unavailable radius implies `Indet`. | Keeps proof state and predicate state readable during route development. |

## Search Ledger

### QP-NUM-001: optimized additive rounding envelope

- Question: can the certified inverse-defect predicate avoid scalar interval
  multiplication while retaining a proof for every determinate decision?
- Candidate: compute ordinary matrix products, then enlarge each entry by a
  proved dot-product rounding term `gamma_n * (|A| |B|)`, the propagated
  exact-input assembly interval, and an explicit underflow allowance. Perform
  the final norm reductions outward.
- Why this is not an arbitrary tolerance: every added term corresponds to a
  named rounding or input error and must be proved to dominate it. A fixed
  `atol + rtol * |value|` without such a derivation remains heuristic.
- Expected value: high. The retained profile attributes 47.42 ms of 57.71 ms
  certified-guard time to scalar inverse-defect enclosure. A batched envelope
  directly targets that bottleneck.
- Smallest useful successor: use batched ordinary matrix products for the
  center and absolute-product terms, then enlarge them analytically; compare
  against the scalar-interval control and exact binary64 oracle before timing.
- Outcome updates: identical-or-more-conservative certificates plus a material
  speedup selects this as the production-candidate guard; any missing enclosure
  falsifies the formula or implementation; no speedup stops this route without
  disturbing the current certified control.
- Result: the batched implementation now uses only f64 matrix products,
  outward reductions, a conservative dot-product rounding factor, propagated
  KKT-entry intervals, and a gradual-underflow allowance. On the retained
  13,891-word profile cohort it took 71.08 ms versus 88.09 ms for scalar
  intervals (1.24x speedup), with identical aggregate decisions and no exact
  fallback.
- Falsification: all 14,241 word decisions matched the scalar-interval route.
  An exact-binary64 audit of 249 ordinary, scaled, and near-singular systems
  found no wrong determinate decision or beta/Q radius failure. Fifteen of 44
  near-singular systems were indeterminate. Maximum observed
  exact-error/radius ratios were 0.00384 for beta and 0.00246 for Q.
- Status: selected pre-production enclosure. The theorem and operation-count
  contract are now `lem:kkt-batched-defect-enclosure` and
  `rem:kkt-batched-binary64-contract` in
  `formal/hk2017-qp-precision.tex`. Production migration remains blocked on
  independent adversarial review and its findings.
- Source check: `nalgebra 0.35.0` dispatches dynamic matrices larger than five
  to `matrixmultiply 0.3.10`; that crate uses runtime-selected ordinary or
  fused multiply-add kernels. Current KKT sizes fit in one inner-dimension
  block. Fused operations have no larger error than the separately rounded
  multiply/add model, multiplication by GEMM's `alpha = 1` is exact, and the
  implementation's use of a full epsilon with `2n` operations leaves an
  additional factor-two rounding budget. Non-finite results return
  indeterminate and gradual underflow is checked at runtime. This supports the
  implemented contract. Focused exact-rational tests also cover the final
  subtraction from the right-hand side or identity and gradual underflow.

### QP-NUM-003: rejected scalar analytic-envelope implementation

- Experiment: replace scalar interval residual/defect multiplication with
  ordinary scalar dot products plus separately accumulated `gamma_n`, input
  interval, and underflow terms.
- Population: retained seed-99599604 length-at-least-five cohort, 13,891 words,
  nine interleaved release runs.
- Observation: it preserved the same best action and required zero exact
  fallbacks, but took 177.35 ms versus 90.04 ms for the scalar-interval control.
  Its defect phase took 130.46 ms versus 49.41 ms.
- Interpretation: the extra scalar absolute-value, error, and outward-reduction
  loops cost more than the simple interval operations. This rejects the scalar
  implementation, not the analytic enclosure formula or a batched
  matrix-product implementation.
- Disposition: prototype deleted. Raw disposable output:
  `/tmp/qp-analytic-envelope-ablation.txt`.

### QP-NUM-002: deferred legacy numerical-contract audit

- Scope: `saddle_point_solver`, `projection_solver`, `constraint_solver`, and
  their consumers still contain static rank, consistency, beta, Q, and
  stationarity thresholds.
- Current correction: public saddle-point comments and tests no longer call
  the static beta/Q labels or legacy Q diagnostic certificates.
- Deferred checks:
  1. trace every threshold to its consumers and determine whether a wrong
     determinate result can affect the returned capacity;
  2. audit the projection solver's retained-eigenspace Q-bound statement rather
     than assuming it is an exact-input certificate;
  3. replace threshold-based rank/consistency decisions with a theorem-backed
     ternary boundary or exact fallback wherever the production route needs a
     mathematical claim;
  4. rename or remove `q_error_bound` after consumers migrate, so its legacy
     field name cannot regain certificate semantics.
- Status: deferred until the direct-predicate guard is selected. These legacy
  paths must not be promoted as trusted production logic meanwhile.

### Exact-Q sign after exact fallback

- Insight: exact admissibility must use `q_exact.is_positive()` and exact
  reciprocal action before any f64 conversion.
- Failure: the ordinary orbit-search wrapper and the provisional
  general-route experiment compared `q_exact_f64` with zero/an f64 epsilon. A
  positive rational can underflow to zero, producing a false rejection.
- Status: fixed in the active QP worktree; the production regression is
  `exact_positive_action_tests_rational_sign_before_conversion`.
- Use: treat `q_exact_f64` only as an output/comparison convenience. Never use
  it for an exact predicate.

### Current f64 admissibility threshold

- Insight: a fixed beta margin can be a cheap baseline for empirical f64 route
  behavior.
- Mechanism: current route labels `min(beta_f64) > 1e-9` as admissible.
- Evidence status: falsified as a sound predicate on HKO in the current compact
  smoke: `13` `True/false` rows for `hko2024_f64`.
- Use: keep only as a baseline comparison and regression warning.
- Next action: do not promote or defend it as trusted logic.

### Fixed epsilon sweeps

- Insight: sweeping `abs(beta_margin) > eps` is a cheap way to see whether a
  static threshold could work.
- Mechanism in old scratch: `epsilon_rule_summary` over several eps values.
- Evidence status: historical exploratory evidence only. Static thresholds are
  structurally suspect because they are not tied to the KKT residual, inverse
  quality, branch stability, or arithmetic rounding.
- Use: keep as cheap baseline if needed, not as a serious final route.
- Next action: re-add only for a targeted comparison against a theorem-shaped
  policy.

### Plain beta-radius predicate

- Insight: if `||beta_f64 - beta_exact||_inf <= R`, then comparing
  `min(beta_f64)` to `R` gives a sound ternary predicate.
- Mechanism: `beta_radius_verdict(min_beta, R)`.
- Evidence status: the ternary implication is mathematically sound conditional
  on the radius claim. The old plain radius
  `||K_f64^{-1}||_inf ||K_f64 x_f64-b_f64||_inf` was empirically false on edge
  fixtures when interpreted against the exact binary64 KKT system.
- Use: keep the predicate pattern; do not keep the old plain radius as a proof.
- Next action: pair the predicate only with a radius whose hypotheses are
  checked.

### Verified-inverse beta radius

- Insight: an approximate inverse can certify an inverse norm for the exact
  binary64 KKT matrix.
- Mechanism: for exact KKT matrix `K`, computed inverse `B`, and candidate
  `x_hat`, compute
  `rho = ||K x_hat-b||_inf`, `delta = ||I-KB||_inf`, and, if `delta < 1`,
  `R = ||B||_inf rho/(1-delta)`.
- Evidence status: the exact inverse-defect statement is now
  `lem:kkt-verified-inverse-defect`; the batched binary64 implementation
  supplies its outward residual and defect hypotheses. Exact fallback remains
  necessary when the defect test fails.
- Use: selected beta predicate in the pre-production general route.
- Next action: independent proof/code review before migration.

### Beta-radius Q bound

- Insight: the symmetric KKT identity `Q = -xi/2` makes a separate
  quadratic-form perturbation bound unnecessary.
- Mechanism: the componentwise verified-solution radius directly encloses the
  last multiplier `xi`; outward scaling by `-1/2` encloses exact `Q`.
- Evidence status: proved in `cor:kkt-beta-q-from-xi` and covered by the
  exact-binary64 Q-radius audit.
- Use: selected Q and action interval route. Keep the older beta-radius
  quadratic formula only as historical comparison.
- Next action: independent proof/code review before migration.

### Legacy residual KKT Q bound

- Insight: old solver stored a residual-based `q_error_bound`.
- Mechanism: `q_error_bound = 4.5 * residual_norm^2 / abs_lambda_min`.
- Evidence status: falsified by compact smoke as a bound against exact binary64
  Q; generated-random has many coverage failures.
- Use: keep as baseline and as evidence that current action intervals are not
  theorem-grade.
- Next action: do not use for trusted capacity isolation.

### Projection predicate

- Insight: projection/reduced-Hessian methods may provide an alternate beta or
  branch view.
- Mechanism in old scratch: projection diagnostics and projection verdicts.
- Evidence status: old scratch README records unguarded projection falsified on
  HKO and edge fixtures.
- Use: possible diagnostic or future algorithm variant; not part of compact v2.
- Next action: re-explore only with a precise theorem target and exact oracle.

### Curvature and reduced-Hessian Q ideas

- Insight: small eigenvalues need not imply large Q uncertainty if movement is
  bounded inside the feasible beta region; bounds may sum direction-wise using
  `min(residual/gamma, curvature/width)`-style terms.
- Mechanism in old scratch: reduced-Hessian diagnostics and several Q-bound
  candidates.
- Evidence status: old scratch README records some curvature/sum-min variants
  falsified or inapplicable. The broader idea is still potentially useful, but
  the old implementation is not a clean route.
- Use: idea bank for future Q-bound work, not current audit surface.
- Next action: if resumed, start from an explicit mathematical statement over
  the constrained beta polytope and test it against exact binary64 rows.

## Current Resume Target

The trusted one-sigma contract, cyclic obstruction route, exact fallback, and
capacity interval aggregation now have a retained implementation, formal
statements, exact-oracle audits, and a profile packet in
`tools/general_algorithm_ablation/`.

The next action is an independent adversarial proof/code review. If its
findings are resolved without invalidating the route, design the production API
and migrate one consumer. Keep the legacy static-threshold and residual-Q paths
visibly heuristic until their consumers have moved.
