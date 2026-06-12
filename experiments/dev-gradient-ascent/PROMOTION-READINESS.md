# Promotion Readiness Packet

Status: decision packet for the current development candidate. This is not an
approval to promote the method or use it as thesis evidence.

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
