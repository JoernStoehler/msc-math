# Sys-Datascience P2 Synthesis, 2026-07-08

Use: parent-level synthesis after executing and reviewing P2 tiny retained-table
missing baseline. This is coordination evidence and thesis-claim boundary
control; generated metrics remain source-truthed in
`../methods/standard-baseline-p2/artifacts/`. P5 later audited
mechanism/tail wording and superseded the "P5 next" state here; see
`p5-mechanism-tail-thesis-use-audit-2026-07-08.md` for current continuation.

Execution packet:

```text
../methods/standard-baseline-p2/
```

Review verdict:

```text
../methods/standard-baseline-p2/review.md
```

## What Ran

P2 ran the compact missing retained-table standard-method set identified by the
P1 audit:

- lasso regression;
- elastic-net regression;
- histogram gradient boosting regression;
- elastic-net logistic high-tail classification;
- histogram gradient boosting high-tail classification;
- feature-family ablation for combinatorial-count and ridge symplectic-area
  feature families.

The packet uses grouped holdout by `capacity_source:facet_count`. It uses active
invariant numeric features only, not source/provenance metadata as model input.

Data note: the in-place prepared LFS table in this worktree was stale relative
to the active invariant feature schema. P2 therefore rebuilt the current-schema
full retained prepared table in `/tmp/sys-ds-p2-current-full`, with hashes:

- `polytope-table.jsonl`:
  `49825d7636246f71f4ebd419cf0ccbc86e39e6b7f43d4b03e889bb85e4887aea`;
- `polytope-provenance-table.jsonl`:
  `6ff88a5accce9a7ec7e5a494107350b0974b2ce0268ea44caae36a18a7494ef2`.

The stale-table mismatch was a useful loud workflow failure: the active helper
refused to run on missing current invariant columns instead of silently using a
weaker schema.

## Result

P2 did not find a trusted `sys > 1` row and did not validate a
generated-candidate proposer.

Retained-table facts from `standard-baseline-p2`:

- rows: 14,336 trusted random/product rows;
- active invariant numeric features: 45;
- maximum observed `sys`: `0.86258589584944`;
- rows with `sys > 1`: 0.

P2 confirms strong held-out in-table signal:

- lasso regression: `R^2 = 0.6110233222992789`;
- elastic-net regression: `R^2 = 0.620742184420513`;
- histogram gradient boosting regression: `R^2 = 0.8784238138483205`;
- elastic-net logistic high-tail classifier: average precision
  `0.55296791481307`;
- histogram gradient boosting high-tail classifier: average precision
  `0.7034314626146508`.

Feature-family ablation points mostly to ridge symplectic-area features under
this split:

- ridge-only gradient boosting regression: `R^2 = 0.8872276246501958`;
- combinatorial-only gradient boosting regression: `R^2 =
  0.04314696727456602`;
- ridge-only high-tail classifier average precision:
  `0.7054296104152254`;
- combinatorial-only high-tail classifier average precision:
  `0.19185015669933783`.

## Parent Interpretation

P2 closes the concrete missing executor named by P1 for broad retained-table
ordinary-method wording. Together with P1's disposition reasons for skipped
families, this materially reduces the standard-method coverage gap for the
retained random/product method table.

P2 does not close:

- generated-candidate proposer claims;
- broader random-distribution or producer-variant claims;
- calibrated hit-rate or density claims;
- mechanism-theorem claims;
- universal "standard data science was exhausted" wording.

The strongest safe update is:

> On the retained random/product table, the named ordinary method packets now
> include the P2 sparse/shrinkage, gradient-boosting, high-tail-classification,
> and feature-family-ablation baselines. These baselines found strong in-table
> signal, especially in ridge symplectic-area features, but still no `sys > 1`
> row and no validated generated-candidate proposer.

## Next Ranking

1. P5 mechanism/tail thesis-use audit is now the default next read-only packet.
   P2 strengthens the ridge-feature story and standard-method coverage, so the
   next useful step is to decide what wording is safe before choosing more
   execution.
2. P4 generated-candidate proposer closure/rescue design remains conditional.
   P2 did not reveal a new non-ridge interaction; reopen P4 only if proposer
   wording remains important after P5 or if a frozen generated-candidate rule is
   named.
3. High-complexity bucket-extension producer run remains a compute packet
   candidate from P3, not an automatic next run. Launch only after a packet card
   names cost, route, artifacts, and review standard.
4. Bounded retained-table thesis writeup is closer after P2, but still needs a
   source-map/wording pass and must not silently become full-slice closure.

## Workflow Update

The parent loop has now passed a real execution/review/synthesis test, not only
read-only P1/P3 design probes. Remaining workflow risk shifts to whether the
next read-only P5 audit and any later compute packet maintain the same claim
boundary discipline.

P5 audit has since run. Use
`p5-mechanism-tail-thesis-use-audit-2026-07-08.md` for the current
continuation.
