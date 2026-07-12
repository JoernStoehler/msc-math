# Observed General Gradient Ascent: Retained Fixed Panel

This packet retains a completed fixed-`F=10` panel for the current
`iterative_observed_multi_direction_probe` candidate. It is a method-result
packet, not a local-maximality certificate or a source of new `sys > 1`
examples.

## Retention and validation

The tracked raw artifact is
[`artifacts/retained-panel.jsonl`](artifacts/retained-panel.jsonl): the exact,
unmodified concatenation in seed order of the twelve external one-row inputs
`/tmp/observed-general-retained-panel-42-53/seed-{42..53}.jsonl`.

Regenerate the tracked artifact and summary from those raw inputs with:

```bash
python3 analyze_retained_panel.py \
  --input-dir /tmp/observed-general-retained-panel-42-53 \
  --out-dir artifacts
```

The standard-library analyzer validates exactly seeds 42 through 53, one
newline-terminated JSON row per source file, shared schema and parameters
(except each row's one-element seed list), successful completion, trace length
and accepted moves, trace/endpoint statuses, finite attempt statuses, and the
aggregate costs. It writes [`artifacts/summary.json`](artifacts/summary.json).

## Fixed run parameters

Every row records `run_mode = retained_preflight`, a one-element seed list,
branch threshold `1e-3`, action window `1e-2`, observed acceptance threshold
`max(0, 1e-3 * abs(base_sys))`, trace steps `1e-3,1e-4`, trace cap `8`, and
endpoint steps `1e-3,1e-4,1e-5,1e-6`. Each accepted trace move recomputes the
branch-derived directions at the new state. The endpoint scan tests every
generated direction against every endpoint step.

## Direct results

All 12 runs completed without an operational failure, and all 12 accepted all
eight trace moves. The mean `sys` increase was `0.011565`; its seedwise range
was `0.003579` to `0.042455`. Measured total compute was `400.889 s`, with 204
finite-step evaluations and 405,772 capacity-orbit iterations.

All 12 traces stopped at the iteration cap rather than the candidate's
all-generated-directions stop condition. Consequently every endpoint result is
`not_evaluable_trace_did_not_stop`, and all 12 exhaustive endpoint scans found
at least one above-threshold finite move. The panel therefore supports
systematic finite-step ascent progress and an observed bounded cost profile;
it refutes, and does not support, a heuristic endpoint claim at cap 8.

The full per-seed values, statuses, and costs are in
[`artifacts/summary.json`](artifacts/summary.json), not duplicated here.

## Raw naming defect

The retained raw rows still use schema
`gradient_ascent_observed_general_smoke_v1` and run IDs of the form
`observed-general-smoke-seed-N`, although their raw `purpose` is
`retained_mode_one_seed_preflight`. This is a naming defect in the producer
surface. The analyzer flags it in `naming_defects`; the artifact preserves it
rather than silently relabeling historical run identity.

## Boundary

The finite endpoint condition would concern only the candidate-generated
directions and checked steps after the trace actually stops. It does not cover
all nearby directions or branches, and it is weaker than local maximality.
Here it is not even evaluable, because every run exhausted cap 8 while finite
above-threshold moves remained. See
[`../../dev-gradient-ascent/METHOD-CANDIDATE.md`](../../dev-gradient-ascent/METHOD-CANDIDATE.md)
and [`../../dev-gradient-ascent/PROMOTION-READINESS.md`](../../dev-gradient-ascent/PROMOTION-READINESS.md)
for the broader candidate limits.
