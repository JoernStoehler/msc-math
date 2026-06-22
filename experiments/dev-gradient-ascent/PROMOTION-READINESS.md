# Promotion Readiness Packet

Status: decision packet for the current development candidate. This is not an
approval to promote the method or use it as thesis evidence.

Scope note: this packet records the state of one gradient-ascent candidate. It
is not the current scope controller for branch-cartography or for studying
`sys(a)` and HK branch behavior across local, semi-local, and effectively
global perturbation scales. See [CHARTER.md](CHARTER.md) and
[branch-cartography/README.md](branch-cartography/README.md) before treating
promotion as the next action.

Source charter: [CHARTER.md](CHARTER.md).
Current candidate: [METHOD-CANDIDATE.md](METHOD-CANDIDATE.md).

## Candidate

Retained development candidate:

```text
iterative_observed_multi_direction_probe
```

At each trace state, the method recomputes the active `sys` state, collects
near-active HK sigma branches, generates local directions, tries all generated
directions and configured finite steps, and accepts the first recomputed
`sys` improvement above the effective threshold

```text
max(min_observed_delta, min_observed_relative_delta * abs(base_sys)).
```

The checked development parameters are:

- trace steps: `1e-3,1e-4`;
- endpoint scan steps: `1e-3,1e-4,1e-5,1e-6`;
- relative improvement threshold: `1e-3`;
- absolute improvement threshold: `0`;
- branch threshold: `1e-3` for large-gap and narrow-gap retained checks;
- branch threshold: `0.01` for high-degeneracy retained checks;
- trace iteration cap: `8`.

These are candidate parameters, not yet approved thesis constants.

## Retained Endpoint Condition

The current checked endpoint condition is finite and heuristic:

```text
After the trace stops, all generated post-stop directions and endpoint steps in
the checked grid have observed sys improvement at most
1e-3 * abs(endpoint_sys).
```

This condition is weaker than local maximality. It is the checked endpoint
condition currently supported by the six-fixture panel. Stronger endpoint
conditions would require further method work, broader computation, or a
different diagnostic.

The condition is not a certificate that no smaller step, ungenerated
direction, omitted branch, or nearby branch-domain effect improves `sys`.

## Counterfactual Next Paths

This packet should not force the next decision into only accepting or rejecting
the finite endpoint condition. Current plausible next paths include:

- promote with the finite endpoint condition and explicit caveats;
- add adaptive continuation for positive-below-threshold endpoint scan rows;
- generate richer post-stop directions before deciding what endpoint condition
  is acceptable;
- grow the retained panel with batch or LICCA orchestration;
- reduce endpoint scan cost before broadening the panel.

These are current examples, not a complete list. The next choice should be made
by expected thesis value, not by the fact that this packet happens to exist.

## Evidence

Current retained panel:

- aggregate output:
  `/tmp/dev-gradient-ascent-current-retained-panel-aggregate-v3`;
- endpoint scan magnitude output:
  `/tmp/dev-gradient-ascent-current-retained-panel-endpoint-scan-report`;
- run-trace behavior output:
  `/tmp/dev-gradient-ascent-current-retained-panel-run-trace-report`;
- source branch diagnostic:
  `/tmp/dev-gradient-ascent-branch-diagnostic-allsafe-check`.

Panel shape:

- selected fixtures: `6`;
- degeneracy counts:
  `large_gap = 2`, `narrow_gap = 2`, `high_degeneracy = 2`;
- successful run directories: `6`;
- failed probe rows: `0`;
- failed endpoint scan rows: `0`.

Endpoint finite-scan result:

- endpoint scan rows: `72`;
- above-threshold endpoint scan rows: `0`;
- positive-below-threshold endpoint scan rows: `46`;
- nonpositive endpoint scan rows: `26`;
- largest positive observed endpoint delta:
  `7.383249465420239e-4`;
- largest ratio to the effective endpoint threshold: about `0.756`;
- per-regime largest ratios to threshold:
  `large_gap = 0.723`, `narrow_gap = 0.756`, `high_degeneracy = 0.631`.

Trace behavior result:

- trace rows: `22`;
- accepted rows: `16`;
- method-stop rows: `6`;
- accepted rows with negative predicted delta: `5`;
- accepted rows after at least one earlier direction was rejected: `7`;
- accepted rows with positive observed delta but negative predicted delta:
  `5`.

Compute budget:

- aggregate wall time from compute-budget reports: about `1679s`;
- per-regime wall times:
  `large_gap = 569s`, `narrow_gap = 489s`, `high_degeneracy = 622s`;
- endpoint scan target orbit iterations: `23664`;
- trace target orbit iterations: `5332`.

## What This Supports

The current evidence supports the following claim shape:

```text
On a small retained panel covering the large-gap, narrow-gap, and
high-degeneracy regimes, the candidate observed multi-direction ascent reaches
endpoints that pass the implemented finite endpoint scan at relative threshold
1e-3. The candidate fixes checked failure modes where a predicted-only or
maximin-only policy missed recomputed finite improvements.
```

The evidence also supports keeping two design choices in the current
candidate:

- try later generated directions after an earlier direction fails;
- test finite steps even when the local branch model predicts a negative
  delta.

The high-degeneracy checked improvements depend on the second choice.

## What This Does Not Support

The current evidence does not support the statement:

```text
The produced endpoints are true local maxima of sys on the quotient.
```

Known gaps:

- endpoint scans still contain positive-below-threshold improvements;
- the direction set is generated by the current local branch model and can miss
  directions;
- branch/germ completeness is heuristic;
- the retained panel is small;
- the high-degeneracy retained checks use a wider branch threshold than the
  large-gap and narrow-gap checks;
- no fixed-`F` datascience rerun has used the candidate method yet;
- no reusable algorithm code has been promoted out of this development packet.

Related local-behavior diagnostics in
`experiments/sys-datascience/methods/local-behavior-prediction/` add a further
promotion caveat. In the current endpoint/top-sys smoke output
`/tmp/sys-local-behavior-ascent-top-candidate-gradient-fixed`, near-active
first-order predicted signs fail on some endpoint directions even when the
target minimizer is still inside the base candidate window. The
candidate-window analytic model with base branch gaps and per-branch actions
repairs those sign failures in this smoke output except for a case where the
target minimizer is outside the candidate window. Inspect
`local-behavior-candidate-gradient-summary.csv` and
`local-behavior-candidate-gradient-predictions.jsonl` before treating
near-active prediction as an endpoint acceptance or rejection rule. This points
toward a candidate-window-aware recentered method, not promotion of the current
near-active-only prediction model. The corrected endpoint/top window sweep in
the local-behavior packet makes `action_window_relative = 0.01` the plausible
next setting to test; narrower `0.003` misses too much on the smoke panel, and
wider `0.03` increases candidate branch count and local-state cost without a
source-stratified sign-prediction gain over `0.01`.

A first candidate-window-scored dev runner has now been checked on the
available retained-fixture overlap:
`/tmp/dev-gradient-ascent-candidate-window-overlap-panel-aggregate`,
`/tmp/dev-gradient-ascent-candidate-window-overlap-panel-endpoint-scan-report`,
and
`/tmp/dev-gradient-ascent-candidate-window-overlap-panel-run-trace-report`.
This overlap panel has five selected fixtures, not the full older six-fixture
panel, because one old large-gap random-product fixture is absent from the
current polytope table. On this overlap panel, candidate-window scoring keeps
endpoint scan rows below the configured relative threshold and removes the
accepted negative-prediction trace rows. Treat this as a promising development
check, not as promotion evidence: it changes scoring over the existing
near-active direction family and still does not certify endpoint local
maximality.

The first candidate-window direction-generator variant is not promoted. It
adds finite-step-indexed candidate-window maximin directions and tests each at
the generating step. On the same five-fixture overlap output
`/tmp/dev-gradient-ascent-candidate-window-directions-overlap-panel-*`, those
new directions are not selected in the trace and increase endpoint scan rows
and runtime without improving the endpoint-scan threshold-ratio diagnostic.
This weakens the direction-generator branch of the repair path, not the
candidate-window scoring branch.

A lower-threshold candidate-window replay was checked on the same five-fixture
overlap:

- `/tmp/dev-gradient-ascent-candidate-window-threshold1e-4-overlap-panel-aggregate`;
- `/tmp/dev-gradient-ascent-candidate-window-threshold1e-4-overlap-panel-endpoint-scan-report`;
- `/tmp/dev-gradient-ascent-candidate-window-threshold1e-4-overlap-panel-run-trace-report`.

This weakens the simple threshold-repair story. Lowering the relative
acceptance threshold from `1e-3` to `1e-4` makes the checked traces continue to
the iteration cap, but the post-stop endpoint scan still finds improvements
above the lowered threshold. It also reintroduces a candidate-window prediction
failure on the overlap panel. Current method implication: candidate-window
scoring is still a plausible diagnostic and scoring repair, but it is not yet
evidence for endpoint stability or for a clean first-order-guided method. The
next discriminating test should be a matched exhaustive replay/ranking audit of
generated direction/step pairs at hard trace states, comparing near-active and
candidate-window rankings against recomputed observed improvements.

The first such audit is:

- `/tmp/dev-gradient-ascent-step-ranking-audit-threshold1e-4-large-gap-rank-0-check`;
- `/tmp/dev-gradient-ascent-step-ranking-audit-threshold1e-4-narrow-gap-rank-0-check`;
- `/tmp/dev-gradient-ascent-step-ranking-audit-threshold1e-4-narrow-gap-rank-1-check`;
- `/tmp/dev-gradient-ascent-step-ranking-audit-threshold1e-4-high-degeneracy-rank-0-check`;
- `/tmp/dev-gradient-ascent-step-ranking-audit-threshold1e-4-high-degeneracy-rank-1-check`;
- summary scratch:
  `/tmp/dev-gradient-ascent-step-ranking-audit-threshold1e-4-summary.json`.

On this small selected audit, candidate-window scoring has useful ranking
signal: across 40 traced states, it ranked the best recomputed improving checked
move first in 39 states. The older near-active rule did so in 25 states.

The same audit also shows that candidate-window scoring is not safe as a hard
rejection rule. In
`/tmp/dev-gradient-ascent-step-ranking-audit-threshold1e-4-narrow-gap-rank-0-check`,
iteration 2, the generated move `near_active_maximin_direction` at step
`0.001` has recomputed `Delta sys = +0.0007425813825379102`, above that row's
acceptance cutoff `0.00009773304353618649`. Candidate-window scoring predicts
`Delta sys = -15189216053351.336` for the same move and ranks it last among the
six checked moves. The older near-active rule predicts
`+0.0007413626661731293` and ranks it first. Current method implication:
candidate-window scoring remains a live repair candidate, but a
prediction-gated first-order method would be unsafe without a domain/failure
guard or a fallback to observed finite replay.

The existing audit outputs above do not record the branch witness that caused
the lower-envelope minimum, so do not classify this yet as a numerical
derivative bug, a branch-domain/model-validity failure, or unavoidable
pessimism from a wide candidate window. The current audit code now records that
witness; before more broad local compute is spent, rerun only this hard state
and inspect the winning candidate branch/orbit for the bad prediction. That
gives the branch-level handle needed to investigate the failure mode; it does
not by itself classify the failure.

## Promotion Question

Decision for Jörn/Kai:

```text
Is the finite endpoint condition above strong enough for the thesis-local
optimization claim, given the six-fixture retained panel, the remaining
positive-below-threshold rows, and the compute cost of growing the panel?
```

If yes, the next work is promotion/integration:

- move reusable implementation pieces into `exp-sys-landscape` or durable crate
  code;
- rerun fixed-`F` datascience producers with the promoted method;
- create an analysis packet that presents the retained method, endpoint
  condition, caveats, and small ablations/failure-mode evidence;
- keep this development packet as history, not thesis-facing evidence.

If no, the next work should be one of:

- tighten or replace the endpoint condition;
- add adaptive continuation for positive-below-threshold endpoint scan rows;
- grow the retained panel by batch/LICCA orchestration;
- add a different endpoint diagnostic that targets the missing local-maximum
  concern more directly.

## Readiness Against Charter

Current status against [CHARTER.md](CHARTER.md) promotion-readiness bullets:

| Charter readiness item | Current status |
| --- | --- |
| Method named and documented by algorithm, tolerances, stop reasons, failure modes, compute budget. | Mostly satisfied for development candidate; constants still need approval. |
| Endpoint diagnostics on documented retained sample, with rule, exclusions, biases. | Satisfied for a small six-fixture retained panel; sample is small and threshold differs for high-degeneracy. |
| Traces cover degeneracy regimes or explain gaps. | Satisfied for two fixtures in each current degeneracy regime. |
| Compute-budget reports enough to plan/rerun/reject fixed-`F` reruns. | Partially satisfied; local per-fixture costs are known, but production fixed-`F` cost is not measured. |
| Retained design choices have ablation/failure-mode/cost reasons. | Partially satisfied; failure-mode and run-trace reports support key choices, but no clean final ablation packet exists. |
| Bad-old-ascent failure modes are fixed/outside claim/still present. | Partially satisfied; two checked failure modes are fixed, but no exhaustive old-vs-new comparison exists. |
| Unresolved risks listed next to positive results. | Satisfied in this packet and METHOD-CANDIDATE. |
| Downstream integration points named. | Named, not implemented. |

Conclusion: ready to ask for a promotion decision, not ready to mark the
charter complete.
