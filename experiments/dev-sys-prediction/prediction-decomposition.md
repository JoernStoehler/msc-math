# Prediction-error decomposition

This contract describes `candidate_window_decomposition` and
`fixed_winner_decomposition` in `src/prediction_cloud.rs`. It applies to the
local-radius observations and step-ranking audits that call those helpers.
It does not establish the accuracy of their branch or target evaluators.

## Quantities and identity

At a base body, the candidate-window predictor selects a word using the
minimum of its base `sys` gap plus its directional linear prediction. Let:

- `P` be the resulting predicted target `sys`;
- `T` be the recomputed target `sys` from the target cache;
- `W` be the minimum finite target `sys` among base-window words accepted by
  the legacy f64 KKT reevaluation;
- `B` be the target `sys` from reevaluating the predicted winning word.

When these quantities are available, ordinary subtraction gives

```text
P - T = (P - B) + (B - W) + (W - T).
          fixed     selection   window
```

This telescoping identity explains the decomposition; it does not prove a
causal attribution. In particular, `W - T` can reflect numerical acceptance
or evaluation differences, not only a target minimizer outside the window.
The target-winner membership fields compare stored words separately; tied
words need not represent distinct physical orbits.

The compatibility fields `linearization_error = P - W` and
`sigma_set_error = W - T` also telescope. `sum_residual` checks that two-term
identity in floating point. It does not check the three-term decomposition,
branch completeness, or solver correctness.

## Evaluation and missing values

`fixed_sigma_action` calls `solve_kkt_for_dual_vertices`, accepts only its
`Feasible` outcome, and returns finite `0.5 / q_corrected`. These are legacy
numerical thresholds, not an exact fixed-word certificate. Window evaluation
omits every unavailable action and nonfinite `sys`; an empty accepted window
gives `None`. Historical fields and profiler keys containing `exact` retain
their names for compatibility, but do not denote exact arithmetic here.

The target cache is a different evaluation boundary; exact rational geometry
or volume upstream does not make this f64 window reevaluation exact. Changing
either evaluator is a scientific change requiring its own comparison, even
if the subtraction identity continues to hold.

The fixed-winner split additionally needs its base action gradient, matching
direction dimensions, and finite positive linearly predicted volume. If that
split is unavailable, its fields remain absent even when `P`, `T` and `W`
are available. Missing fields must not be interpreted as zero error.

## Action and volume split

Write `S(c,V) = c^2/(2V)`, with reevaluated action `c`, target volume `V`, and
linear predictions `c_lin`, `V_lin`. The implementation defines

```text
action part = S(c_lin, V) - S(c, V)
volume part = S(c, V_lin) - S(c, V)
residual    = (P - S(c, V)) - action part - volume part.
```

The field named `capacity_volume_interaction_error` is this residual.
`P` is the linear `sys` prediction, not `S(c_lin, V_lin)`, so the residual is
not solely the nonlinear interaction of the two substituted quantities.
These are signed diagnostic terms; cancellation and unavailable evaluations
limit error-source classifications based on their magnitudes.
