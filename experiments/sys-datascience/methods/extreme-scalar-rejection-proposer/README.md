# Extreme Scalar Rejection Proposer

## Question

Can frozen scalar-feature rules select promising random-product candidates
before capacity or `sys` is computed?

This packet tests the generated-candidate proposer stage: generate random
Lagrangian-product candidates, compute cheap scalar features, freeze selected
and matched-baseline candidate ids before `sys`, and evaluate `sys` only for
that selected-or-baseline union.

Current tracked evidence includes the original 100k `promising-scalars` packet
and the independent frozen ridge-concentration validation under
`artifacts/100k-ridge-concentration-validation/`. The latter tests whether a
concentration add-on improves on the low-ridge scalar boundary. Both compact
packets intentionally omit full generated geometry and feature caches;
regenerate them from their durable configs when row-level inspection is needed.

Evidence-stage boundary:

- `../statistical-associations/` screens many retained-table scalar covariates
  against already computed `sys`.
- `../tail-rule-mining/` reports retained-table low/high single-feature
  filters on rows whose `sys` values are already known.
- This packet evaluates a generated-candidate scalar-filter proposer: scalar
  selection happens before `sys` is computed for the selected-or-baseline union.

## Runner

Durable 100k packet command:

```bash
cargo run --manifest-path experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/Cargo.toml --release -- \
  --config experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/configs/100k-promising-scalars-durable.json
```

The durable config writes to
`experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/artifacts/100k-promising-scalars/`.

Stages:

- `geometry`: writes `candidate-geometry-cache.jsonl`.
- `features`: writes `candidate-feature-table.jsonl`.
- `selection`: freezes selected and baseline candidates in
  `selected-candidates-before-sys.jsonl` and records `selection-plan.json`.
- `sys`: evaluates only selected or baseline candidate ids and writes
  `sys-evaluation-cache.jsonl`.
- `reports`: writes `selection-summary.tsv`, `evaluation-report.json`, and
  `pipeline-summary.json`.

Fresh `sys` stages use the production product QP route and write
`evaluated-target.v3` rows with outward capacity bounds, exact rational
capacity, the product-closure candidate-family label, a deterministic exact
minimizing word, and the tied-minimizer count. The stored scalar is accepted
only when the bounds certify `1e-10` relative error. A v2 cache from the
legacy billiard/orbit route remains historical evidence but is not a cache hit
for current code; use a new output directory rather than appending mixed
schemas.

Reports read frozen `selection-plan.json` and
`selected-candidates-before-sys.jsonl` instead of recomputing selection
semantics from current CLI/config arguments.

The runner supports default single-rule mode, explicit multi-rule mode, and the
curated `promising-scalars` rule set used by the tracked 100k packet. Scalar
rules are allowlisted accessors over `CandidateFeatureRow` fields, not JSON
reflection.

### Frozen canonical-vertex covariance packet

The candidate-feature v3 schema adds the geometry-only feature
`vertex_covariance_rho = nu2 / nu1`, where `nu1 <= nu2` are the Williamson
eigenvalues of the population covariance of the canonical distinct primal
vertices in `(q1,q2,p1,p2)` order. The row also records both Williamson
eigenvalues, ordinary-eigenvalue conditioning, distinct/expected vertex counts,
and an explicit eligibility status. The frozen selector excludes ineligible
rows before forming every arm.

The two `configs/covariance-rho-frozen-seed-*.json` files freeze two 50,000-row
product populations. Run only `geometry`, `features`, and `selection` before
portfolio review; do not use `all`, because `all` continues into capacity
evaluation. `assemble_covariance_rho_frozen_manifest.py` verifies and combines
the two per-seed selection artifacts, including exact per-stratum arm counts and
the shared control's disjointness. The large caches and combined manifest live
under `/tmp/joern/covariance-line/frozen-packet`, not in Git.

After a separately authorized capacity run has written fresh evaluation cache
files, `analyze_covariance_rho_validation.py` is the only reader for the frozen
three-arm decision. It accepts the combined manifest plus those fresh cache
files, rejects row-identity/membership mismatches and missing rows, and writes
the predeclared rho-control/rho-ridge intervals and verdict. Its `--self-test`
uses synthetic rows only; it does not read or evaluate the frozen population.
The completed compact packet is
`artifacts/covariance-rho-frozen-validation/`.

## Compact Artifact

Tracked artifact:

- `artifacts/100k-promising-scalars/README.md`: provenance, audit state, and
  interpretation boundary.
- `prompt.md`: executor packet brief.
- `command-output.txt`: captured runner output.
- `resolved-run-config.json`: resolved run configuration.
- `selected-candidates-before-sys.jsonl`: 1675 selected-or-baseline rows frozen
  before `sys`.
- `selection-plan.json`: frozen selection rules, budgets, and union counts.
- `sys-evaluation-cache.jsonl`: 1675 evaluated selected-or-baseline rows.
- `selection-summary.tsv`: per-selection and union matched summaries.
- `evaluation-report.json` and `pipeline-summary.json`: compact generated
  reports.
- `target-field-audit.txt`: target-field audit output from before trimming the
  full pre-target caches.

Omitted generated caches:

- `candidate-geometry-cache.jsonl`: about 406 MB in the source run.
- `candidate-feature-table.jsonl`: about 241 MB in the source run.

The omitted caches are not required to read the compact result reports. They are
required to rerun the exact three-file target-field audit recorded in
`target-field-audit.txt`. The artifact README records the local `/tmp` cache
location used as non-required provenance if those files still exist.

## Current Evidence

2026-07-11 independent ridge-concentration validation:

- a fresh 100k random-product sample tested one rule frozen before `sys`;
- the predeclared descriptive incremental-enrichment criterion passed;
- no evaluated candidate had `sys > 1`;
- `artifacts/100k-ridge-concentration-validation/review.md` records the review
  and the unresolved terminology boundary between a validated sub-threshold
  enrichment proposer and a proposer for finding `sys > 1`.

Recorded run date: 2026-07-01.

100k `promising-scalars` compact packet:

- 30 selection sets from the current `promising-scalars` rule set.
- 1200 selected rows summed over sets.
- 485 unique selected rows.
- 1195 unique baseline rows.
- 1675 unique selected-or-baseline rows.
- Runner output reports `cached_rows=0` for the `sys` stage and appended 1675
  rows.
- Target-field audit passed before cache trimming:
  `checked 3 pre-target JSONL artifact(s): no forbidden keys`.
- Maximum evaluated `sys`: `0.867546058507634`.
- Maximum selected `sys`: `0.867546058507634`.
- No evaluated candidate had `sys > 1`.

These accepted target rows use the historical `evaluated-target.v2` route.
They are not relabelled as v3 production certificates; their compact packet
and reports remain frozen.

Interpretation boundary:

- This is 100k generated random-product candidate evidence for the configured
  `promising-scalars` rule set, not a theorem and not evidence outside this
  generator/configuration.
- Interpret comparisons through the per-selection and matched-baseline rows in
  `selection-summary.tsv`; avoid pooled proposer claims.
- The run supports the claim that the current scalar rules can be evaluated as
  pre-`sys` generated-candidate proposers at 100k scale. It does not by itself
  support a near-counterexample claim.

## Boundaries

This packet is random-product only. Generic random candidates should reuse the
same stage artifact names and add source fields rather than widening the
evaluated retained-table pipeline.

The direct geometry stage is product-specific. If this packet is extended
beyond Lagrangian products, do not reuse the direct product vertex and incidence
construction.
