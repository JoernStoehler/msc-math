# Adversarial review and disposition

A fresh read-only `gpt-5.6-sol` reviewer at high reasoning effort checked the
technical packet against the producer, raw traces, geometry route, branch
model, quotient basis, and derivative implementation.

## Verdict

The evidence is sufficient to choose a narrow derivative/KKT decomposition as
the next diagnostic. It is not sufficient to implement gradient sampling, to
classify the endpoint, or to attribute the failure to a geometry boundary.

## Material findings and disposition

1. **The retained evaluator is not a certified complete mathematical branch
   envelope.** `MinimaSafe` does not certify global capacity or a complete
   minimizing set, and its two returned rows need not be exact ties.
   The packet now distinguishes the mathematical `sys` from the implemented
   `hat(sys)` and removes active-set claims.

2. **The proposed nearby-gradient method mixed two algorithms.** Standard
   gradient sampling of `-hat(sys)` uses gradients actually realized at nearby
   differentiability points and a minimum-norm convex-hull calculation.
   Transporting branch values and gradients is a separate finite affine
   predictor. The packet now separates them and forbids arbitrary inactive
   near-minimizers in a stationarity test. The shared Rust helper named
   `clarke_directional_derivative_a` returns the minimum active-branch slope,
   not the Clarke generalized directional derivative. It has several existing
   consumers and is recorded here for a separate API/consumer audit rather
   than silently renamed in this experiment packet.

3. **The affine failure is cohort-wide.** Of 80 action-window max--min
   proposals, 52 decreased the evaluator output despite a represented target
   winner with positive affine prediction. Forty failures had determinate,
   unchanged geometry. All 40 displayed-winning-branch gradient proposals
   decreased. The packet and generated endpoint report now contain the
   state-level table; rank 2 is a positive max--min control and ranks 4 and 8
   are clean failure controls.

4. **The tested action derivative was described incorrectly.** It is an
   envelope formula evaluated from one KKT payload, not a derivative obtained
   by differentiating the KKT solution map. The packet now states the
   implementation correctly and makes KKT differentiation part of the audit.

5. **Unchanged f64 incidence is not an independent geometry control.** The
   next audit now compares f64 with exact-arithmetic reconstruction and volume
   at the base and smallest signed perturbations.

6. **One beta margin was already retained.** The displayed base winner has
   `beta_margin = 0.0323338906286978`; only the second row and target-word
   margins/ranks remain missing. The packet now records that distinction.

7. **The retained endpoint report omitted five proposals.** The producer
   tested one base call, ten max--min proposals, five displayed-winning-branch
   gradient proposals, and fifty signed-basis proposals. The report generator
   and reader READMEs now state the complete `1 + 10 + 5 + 50 = 66` accounting.

## Remaining reader questions

The next diagnostic is designed to answer the highest-value unresolved
questions:

- whether the two returned base rows are exact ties, overlapping intervals, or
  distinct KKT solutions;
- the rank, condition, residual, beta vector, and beta directional derivative
  of the relevant named words;
- whether transition, omega, and admissibility predicates remain fixed;
- whether f64 and exact-arithmetic incidence and volume agree;
- whether analytic named-action and volume derivatives match decreasing-step
  finite differences;
- why rank 2 succeeds while ranks 4 and 8 fail under clean geometry; and
- which consumers of the same derivative need revalidation if the defect is
  systematic.

Questions about derivative-free oblique coverage and endpoint stationarity
remain deliberately downstream. It is wasteful to interpret sampled gradients
before validating the gradients themselves.

## Review boundary

The review checked mathematical definitions, quantitative source
reconciliation, causal alternatives, and reproducibility. It did not prove
candidate-family completeness, local maximality, or correctness of the
derivative implementation. It made no code edits and ran no scientific
experiment.
