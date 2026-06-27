# Legacy Ascent And Continuation Debt

This note preserves the nontrivial value from the old fixed-`F` ascent and
variable-`F` continuation surfaces. It is not an active method packet and does
not reopen optimizer work.

For the current random/product data-science thesis slice, start with
`experiments/sys-datascience/README.md` and
`experiments/sys-datascience/methods/README.md`.

## Legacy Search Observations

These observations are bounded empirical context from the legacy
`experiments/sys-landscape/` families, not local-maximality evidence and not a
candidate-proposer claim.

- `random-sample`: 70 legacy random generic-polytope rows, max `sys = 0.739`,
  no `sys > 1`.
- `random-product-sample`: 100 legacy random Lagrangian-product rows,
  max `sys = 0.794`, no `sys > 1`.
- `gradient-ascent-general`: 10 fixed-`F` general-ascent seeds,
  max `sys = 0.9030`, no `sys > 1`; all seeds used escape logic.
- `gradient-ascent-products`: 12 fixed-`F` Lagrangian-product ascent seeds,
  max `sys = 0.8727`, no `sys > 1`.
- `variable-f-ascent`: 90 continuation trials, including random-seed RQ2 rows
  and 10 RQ1 local-maxima starts; gains from `F = 10` to `F = 11` were common
  but remained below `1`.

Allowed use: cite this only as legacy bounded search context. Do not use it as
the active random/product method table, as proof that no `sys > 1` examples
exist, or as evidence that ascent endpoints were local maxima.

## Preserved Design Lessons

- Keep ascent/continuation outside the active random/product method table unless
  Jörn explicitly reopens that separate thesis slice.
- Reopened optimizer/local-`sys` work needs its own entry point, evidence
  standard, and return rule. Do not evaluate a prototype without declaring
  whether the reported candidate is the final iterate, best-so-far, or another
  selection rule.
- Endpoint diagnostics from the old packets were finite grid checks. They did
  not rule out smaller steps, ungenerated directions, omitted branches, or
  nearby branch-domain effects.
- Branch changes are mechanism data, not failure predicates. They may explain
  objective drops, step-size collapse, or endpoint improvability, but a run does
  not fail merely because selected sigmas or sampled branches changed.
- Reopened producers should preserve exact rational endpoint geometry, stable
  `poly_id`/`state_id` identities, lineage fields, per-seed reproducibility,
  and canonicalized output order.

## Rejected Or Deferred Routes

- Do not rebuild a one-off wide table from committed JSONL as the default
  data-science surface.
- Do not promote raw witness-permutation transfer as a global cache across
  unrelated seeds.
- Do not use a continuation strategy that increases `F` before fixed-`F` ascent
  as the default without a measured comparison.
- Do not treat reduced-model outcomes as final claims before exact witness
  verification.

## Reopen Criteria

Witness-guided continuation remains a possible future design, not retained
thesis evidence. Reopen it only as a separate work packet with:

- explicit witness-oracle instrumentation in fixed-`F` ascent paths;
- persisted top-`m` and within-gap witness sets plus diagnostics for exact
  evaluations;
- a benchmark bank using `variable-f-ascent` endpoints and existing exact rows;
- comparisons among minimizer-only, top-`m`, within-gap, parent-cache, and
  hybrid witness sets;
- exact-check fallback for accepted reduced-model candidates;
- stop criteria based on exact-call reduction, safe pruning, and retained
  exact-failing no-improvement evidence.
