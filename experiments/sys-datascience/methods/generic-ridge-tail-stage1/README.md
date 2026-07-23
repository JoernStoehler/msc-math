# Generic F=10 ridge-tail stage one

Status: completed and independently reviewed target-free stage-one packet. Its
full 10,000-candidate replay passed before target exposure; the sibling
`generic-ridge-tail-stage1-target/` packet later evaluated exactly the frozen
200 rows. The target-free artifacts remain the source for selection, geometry,
numerical, and pre-exposure provenance, not for the resulting `sys` claims.

## Scientific object

This packet samples exactly 10,000 accepted generic, non-product `F=10`
polytopes at heights `[0.8, 1.2]` with a fresh deterministic seed. It ranks
them by `ridge_symp_area_mean_over_volume_sqrt`. The ridge-area mean is required
because generic `F=10` ridge counts vary. Production uses
`volume_from_incidence_f64` for the proxy denominator. Its lowest 1% is a
frozen 100-row future target panel; a deterministic disjoint 100-row baseline
is matched to the same single `F=10` population.

The target-free population artifacts contain no capacity or `sys` values. The
frozen selected panel has disjoint `0-.1%` (10 rows) and `.1-1%` (90 rows)
bands. A singleton `.01%` row has no evidential role. The later target rows and
analysis are stored alongside this packet only because the target evaluator is
their owning sibling; they do not retroactively change the target-free
selection contract.

## Numerical gate and production

The engineering smoke compares `volume_from_incidence_f64` against the
rational-arithmetic result of `volume_from_incidence_exact` on all retained
seed-42 generic `F=10` rows. The latter returns a `BigRational`, which the
packet converts to f64 for proxy ranking and reporting. The generated summary
owns relative error, rank agreement, bottom-tail membership,
screen recall, measured path runtimes, and the retained-table high-`sys`
threshold. The audit also reports the implied change in retained `sys` if f64
volume replaces rational-arithmetic volume while capacity is held fixed.
Production is permitted only when the f64 bottom-1% membership is stable
against the rational-volume reference and all numerical results are finite and
positive.

The rational-volume comparison is a one-time retained-data audit, not a
production screening cascade. New candidate ranking uses f64 incidence volume,
and the later target evaluation must compute capacity and `sys` together with
that same f64 volume. Production computes no rational-arithmetic volumes.

Run from the repository root:

```bash
cargo run --release --manifest-path experiments/sys-datascience/methods/generic-ridge-tail-stage1/Cargo.toml -- \
  smoke \
  --random-path experiments/sys-datascience/produce/random.jsonl \
  --table-path experiments/sys-datascience/prepare/polytope-table.jsonl \
  --out experiments/sys-datascience/methods/generic-ridge-tail-stage1/artifacts/smoke-summary.json \
  --workers 12

cargo run --release --manifest-path experiments/sys-datascience/methods/generic-ridge-tail-stage1/Cargo.toml -- \
  produce \
  --out-dir experiments/sys-datascience/methods/generic-ridge-tail-stage1/artifacts/stage1 \
  --seed 20260714 --count 10000 --workers 8 \
  --smoke-summary experiments/sys-datascience/methods/generic-ridge-tail-stage1/artifacts/smoke-summary.json \
  --high-sys-threshold 0.5949424195457518 \
  --threshold-definition 'retained generic F=10 nearest-rank empirical 90th percentile; future exceedance is sys >= threshold'

cargo run --release --manifest-path experiments/sys-datascience/methods/generic-ridge-tail-stage1/Cargo.toml -- \
  validate \
  --out-dir experiments/sys-datascience/methods/generic-ridge-tail-stage1/artifacts/stage1

# Historical irreversible exposure gate used before the completed target run:
# replay all 10,000 candidates and regenerate the frozen selection, baseline,
# and 200 rational geometries. Do not run this against the committed artifacts
# from current Main.
cargo run --release --manifest-path experiments/sys-datascience/methods/generic-ridge-tail-stage1/Cargo.toml -- \
  full-validate \
  --out-dir experiments/sys-datascience/methods/generic-ridge-tail-stage1/artifacts/stage1 \
  --workers 12
```

The smoke inputs are LFS files and must be hydrated before the first command.
Do not hand-edit generated JSON or JSONL.

The `produce` and `full-validate` commands above document the completed run;
they are not a current execution queue. An exact full replay belongs in a
disposable worktree at executable replay commit
`310ec8bbb6123a34df8bbdb151717d6cee0fccf7`, with that worktree's LFS inputs
hydrated. That replay validates against the frozen production/dependency
snapshot `4f7adddec513f4abc95dcc905d1299611ff28f28`. The integrated history
retains both commits. Do not regenerate the committed panel or expose a new
target arm without a new portfolio decision.

The committed `full-validation.json` is historical output from original line
commit `4f7adddec513f4abc95dcc905d1299611ff28f28`. Its absolute
`source_hashes.path` strings identify the production worktree used at the time;
they are historical aliases, not checkout requirements. The validation code
reports drift from the frozen commit and source closure as a staleness warning
rather than rejecting a semantically valid replay. The
ridge-tail integration deliberately retains that original history so the
closure remains available from a clean checkout of merged Main.

## Artifacts and use boundary

- `artifacts/smoke-summary.json`: numerical gate and frozen retained-table
  threshold;
- `artifacts/stage1/manifest.json`: generator, seed, hashes, counts, f64
  selection boundary, timings, resource use, volume contract, and
  target-exposure declaration;
- `artifacts/stage1/selection.jsonl`: the frozen lowest 1% with f64 volumes,
  proxy values, and ranks;
- `artifacts/stage1/panel-geometries.jsonl`: rational geometry for the frozen
  selected and baseline panel, without targets;
- `artifacts/stage1/validation.json`: count, ordering, disjointness,
  deterministic-subset, artifact-hash, and forbidden-target-field checks.
- `artifacts/stage1/full-validation.json`: the promoted full 10,000-row
  target-free replay check. It records artifact/source byte identities as
  advisory provenance and validates the complete population, selection,
  baseline, panel, row/geometry fields, and target-free boundary.

Historical allowed use before target exposure was independent review of the
generator, proxy, retained numerical audit, f64 cutoff, baseline, provenance,
and absence of target fields. Target evaluation meant the later joint
computation of capacity and `sys`. Current readers should use the sibling
target packet for enrichment and stopping claims. The target-free packet alone
must not be treated as target evidence or authority to scale to 100k/1M.
