# Generator sampling-law fidelity and exchangeability audit

This copy-local, target-free analyzer asks whether retained generator rows
support the sampling-law and effective-independent-unit assumptions that later
distribution comparisons would need.  It consumes the generator-zoo accepted
factor/product artifacts and, when supplied, natural-law smoke rows.  It does
not call `sys`, rerun a target backend, or select geometry by a target field.

## What it checks

- deterministic replay when two independently generated row files are passed;
- sample-ID/seed/batch lineage, duplicate accepted `q`-factor geometry,
  serialization order, and product terminal acceptance/exhaustion records;
- circular resultant and Kuiper diagnostics for declared uniform-angle rows;
- Dirichlet simplex constraints and Beta-marginal PIT diagnostics for retained
  repulsive-gap geometry; and
- terminal-status/schema readiness for natural shared-latent rows.

The report treats one accepted product logical row (represented by its `q`
factor) as the factor-marginal effective unit.  It intentionally does not
double-count paired `q,p` factors as independent.  A small smoke cannot accept
a null law.  In particular, cross-seed agreement is only a consistency check.

The natural shared-latent v1 rows retain aggregate CVs rather than gap/support
latent vectors or factor geometry.  The report therefore records
`not_auditable_from_retained_schema` for its coordinatewise rho and
shared-seed checks, along with the cheapest producer amendment.  Similarly,
the zoo rows have accepted attempts but no rejected-proposal ledger, so they
cannot establish proposal-versus-accepted conditioning.

## Reproduce

The zoo JSONL inputs are LFS-managed.  Fetch only the required inputs first if
they are pointers:

```text
git lfs pull --include='experiments/sys-datascience/methods/generator-zoo-smoke/artifacts/factor-shapes.jsonl,experiments/sys-datascience/methods/generator-zoo-smoke/artifacts/product-smoke.jsonl'
python3 experiments/sys-datascience/methods/generator-law-fidelity/analyze.py --self-test
python3 experiments/sys-datascience/methods/generator-law-fidelity/analyze.py \
  --natural-rows experiments/sys-datascience/methods/generator-law-fidelity/artifacts/natural-smoke/smoke-rows.jsonl \
  --natural-report experiments/sys-datascience/methods/generator-law-fidelity/artifacts/natural-smoke/batch-report.json
```

To test actual deterministic replay, run the existing natural-law producer
twice to different disposable directories with identical arguments, then add
`--replay-left DIR1/smoke-rows.jsonl --replay-right DIR2/smoke-rows.jsonl`.
The comparison is exact after omitting wall-clock fields.

`artifacts/report.json` is machine-readable and binds the input bytes, analyzer
hash, repository revision/tree, and tracked-clean predicate.  Its output is
readiness evidence for future target-free generator comparisons, not an
acceptance certificate for any generator population or target transfer.
