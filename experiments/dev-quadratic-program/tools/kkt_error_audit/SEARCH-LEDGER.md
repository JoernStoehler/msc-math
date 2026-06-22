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
- Evidence status: current compact smoke has no observed unsound predicate rows.
  On HKO it is often unavailable, so HKO remains fallback-required. This is
  empirical evidence plus a standard proof target, not a completed production
  theorem in this repo.
- Use: main current theorem-shaped beta predicate candidate.
- Next action: formalize the proof obligation and add f64 rounding conditions
  before route promotion.

### Beta-radius Q bound

- Insight: a beta error radius can bound Q error:
  `|Q(beta_f64)-Q(beta_exact)| <= R ||H beta_f64||_1 + 1/2 R^2 sum_ij |H_ij|`.
- Mechanism: current compact audit computes
  `verified_inverse_beta_radius_q_bound`.
- Evidence status: current compact smoke has no observed coverage failures for
  the verified-inverse Q bound. The bound omits a separate binary64 rounding
  term for production use.
- Use: current candidate for action intervals in capacity isolation.
- Next action: formalize the exact arithmetic statement first, then add the f64
  rounding term for `H`, `beta_f64`, and computed `Q`.

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

The next high-value work is not another broad import. It is to develop a
proof-backed trusted one-sigma f64 KKT solve contract:

1. State exact mathematical propositions for beta positivity and Q/action
   intervals.
2. Implement runtime predicates that return `True`, `False`, `Indet`, or
   unavailable according to those propositions.
3. Track branch-local proof facts in code, not in stale report prose.
4. Use exact binary64 audits to falsify candidate formulas before route
   promotion.
5. Promote to production route logic only after theorem statements, code
   contracts, and smoke/regression evidence agree.

The current compact audit supports this target by testing the verified-inverse
beta radius and beta-radius Q bound while keeping current f64 behavior visible
as a falsified baseline.
