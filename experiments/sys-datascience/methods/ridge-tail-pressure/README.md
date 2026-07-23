# Stratified ridge-tail pressure

## Question and intended use

Does low `ridge_symp_area_sum_over_volume_sqrt` remain associated with higher
`sys` within the ten Lagrangian-product `(k,m)` strata, and do two frozen
pre-target ridge selections show a stable pattern as the selection becomes
more extreme?

This is an owner-local packet for further research.  It supplies a retained,
stratified descriptive diagnostic and a bounded pre-target selection comparison
that can motivate a non-confirmatory endpoint-path plumbing smoke.  It is not
evidence for a universal proposer, a selection-pressure curve, endpoint
exceedance, or a `sys > 1` probability.

`REPORT.md` gives the interpretation and evidence boundaries; the generated
TSVs own the detailed measurements.  `REVIEW.md` and `RESPONSE_TO_REVIEW.md`
record the independent review and accepted repairs.

## Inputs and provenance

The analyzer consumes the tracked retained prepared table
`../../prepare/polytope-table.jsonl`, tracked random-product producer
`../../produce/random-product.jsonl`, and the tracked 100k validation cache and
selection plan in `../extreme-scalar-rejection-proposer/artifacts/100k-ridge-concentration-validation/`.
The retained prepared table is the exact recovered version of the reviewed
scratch table.

`generated-input-snapshot/one-m-ridge-sum/` is the minimal exact 1M source
snapshot: frozen selection plan, selected-before-target rows, and evaluated
`sys` cache.  Its manifest records original hashes.  The 2.5 GB full 1M
feature table is deliberately not required: each selected/evaluated target row
already records its selection feature value.  Supply it only with
`--one-m-feature-table PATH` to run the optional identity audit. A byte
difference emits a staleness warning; row identities and feature joins decide
whether the table is usable.

## Reproduce

From this directory:

```bash
python3 analyze.py --out-dir .
```

For a clean comparison before overwriting retained outputs:

```bash
tmp=$(mktemp -d)
python3 analyze.py --out-dir "$tmp"
for f in *.tsv; do diff -u "$f" "$tmp/$f"; done
```

The normal command is independent of this worktree path.  It infers the repo
root from the packet; `--repo-root`, input-path overrides, and
`--one-m-snapshot-dir` support a relocated checkout or explicit audit inputs.

## Reopen boundary

Do not use the packet to make a selection-pressure or endpoint claim.  Reopen
the comparison only with a common-q, multi-seed (or repeated-run) design that
can separate selection pressure from run, population, and order-statistic
differences.  Any endpoint smoke motivated here remains a feasibility probe,
not validation of these associations.
