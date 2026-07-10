# Sys-Datascience P4 Generated-Candidate Closure, 2026-07-08

Use: design/closure packet for generated-candidate proposer wording after the
bounded retained-table source-map. This is not a new experiment and not thesis
prose.

Inputs read:

- `topics/generated-candidate-proposers.md`;
- `topics/geometric-feature-mechanisms.md`;
- `p5-mechanism-tail-thesis-use-audit-2026-07-08.md`;
- `../methods/extreme-scalar-rejection-proposer/README.md`;
- `../methods/extreme-scalar-rejection-proposer/artifacts/100k-promising-scalars/README.md`;
- `../methods/extreme-scalar-rejection-proposer/artifacts/100k-promising-scalars/evaluation-report.json`;
- `../methods/extreme-scalar-rejection-proposer/artifacts/100k-promising-scalars/selection-summary.tsv`;
- `../methods/extreme-scalar-rejection-proposer/artifacts/ridge-tail-1m-summary/role-summary.tsv`;
- `../methods/ridge-mechanism-discriminator/artifacts/current/hypothesis_rollup.tsv`.

## Source Facts

Current positive proposer status: none.

The durable 100k generated-candidate scalar packet:

- uses random Lagrangian-product candidates with `10,000` candidates per
  product bucket, `100,000` candidates total;
- freezes selection before `sys`;
- evaluates `1,675` selected-or-baseline rows;
- has no `sys > 1` row;
- has maximum evaluated and selected `sys = 0.867546058507634`;
- records target-field audit pass before the full pre-target caches were
  trimmed.

The 1M low-ridge summary remains boundary evidence:

- per-bucket low-ridge rules enrich selected rows;
- selected maxima remain below `sys = 1`, with the low-sum per-bucket summary
  topping out near `0.866920080910149`;
- count-only rules are weak compared with ridge rules.

The mechanism discriminator adds a possible rescue idea:

- among low-ridge selected 100k rows, high normalized entropy and low max/top-3
  share split mean `sys` upward by about `0.047` to `0.051` overall;
- this split was mined after `sys` and is diagnostic-only.

## Decision

Close scalar-filter generated-candidate proposers for the current thesis story.
Do not launch a two-feature rescue executor now.

Reason:

- The best current scalar generated-candidate packet is already
  selection-before-`sys` and finds no positive or near-counterexample.
- Larger single-scalar low-ridge evidence enriches but also supports
  extreme-tail Goodharting caution.
- The only concrete rescue idea is post-`sys` concentration conditioning, and
  the current runner does not have a settled conjunction-rule interface.
- Running a rescue now would mainly test a mined diagnostic rule, not close an
  already thesis-needed claim.

Safe thesis wording:

> A generated-candidate scalar-filter packet was run with selection frozen
> before `sys` evaluation. It enriched high-`sys` random-product candidates but
> found no `sys > 1` row and did not validate a candidate-proposer.

Do not write:

- "no candidate-proposer is possible";
- "low ridge filtering fails";
- "the proposer space is exhausted";
- "the concentration rescue was tested independently."

## Optional Future Rescue Design

If proposer wording becomes important enough to reopen this topic, use this
single design. Do not browse more features before freezing it.

Rule family:

- base rule: low ridge magnitude within each product bucket;
- add-on rule: high `ridge_symp_area_normalized_entropy` or low
  `ridge_symp_area_max_share`; choose one before independent evaluation;
- selection split: product-bucket matched, rule frozen before `sys`;
- validation: independent generated random-product seed or regenerated
  candidate pool; compare against scalar-only per-bucket low-ridge boundary;
- stop if max selected `sys` remains below about `0.90`, if the rule works in
  only one or two buckets, or if it fails to beat scalar-only low ridge in most
  buckets;
- escalate immediately if any independent evaluated row has `sys > 1`.

Implementation warning:

- This is a new packet because the current runner supports scalar-rule unions,
  not a settled two-feature conjunction-rule interface.
- The local full 100k feature cache used by the mechanism discriminator is not
  tracked; regenerate durable pre-target caches if row-level feature mining is
  needed.

## Parent Synthesis

P4 removes the main cheap proposer-design uncertainty from the current launch
board. The retained-table source-map can say "no validated generated-candidate
proposer" and cite the scalar generated-candidate packet plus this closure
decision.

Remaining non-writeup data-science branch:

- high-complexity producer/distribution extension from P3. This is now the only
  concrete remaining execution candidate if the parent wants stronger evidence
  beyond the retained producer contract.

Default continuation at P4 time was to prepare the high-complexity producer
compute packet, or move the bounded retained-table source-map into
thesis-facing companion/prose if the bounded story is accepted as sufficient.
The compute packet was later written as
`high-complexity-producer-compute-packet-2026-07-08.md`; use
`next-session-candidates.md` for the current decision board.
