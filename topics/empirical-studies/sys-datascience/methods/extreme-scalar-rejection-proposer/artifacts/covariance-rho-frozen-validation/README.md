# Frozen Canonical-Vertex Covariance Validation

Status: completed prospective same-generator validation, 2026-07-12. The
frozen low-`vertex_covariance_rho` arm passed its rho-control criterion; it was
competitive with, but not better than, the frozen ridge cascade. This is the
compact durable packet. `pre-target/` is immutable selection evidence and
`evaluation/` is its exact target output.

## Frozen Design And Decision

Two independent product-generator seeds (`2026071201`, `2026071202`) each
produced 50,000 candidates across ten `(k,m)` buckets, with heights in
`[0.8,1.2]`. In every seed/bucket stratum, selection retained 25 low-rho rows,
25 rows from the existing ridge cascade (bottom 1% ridge sum, then bottom 50%
maximum-share), and 25 deterministic controls disjoint from their union.

The combined manifest has 1,436 unique rows: 500 memberships in each arm,
64 rho/ridge overlaps, and disjoint controls. Primary success required a
20-stratum rho-control estimate at least `0.08`, a two-sided 95% interval with
positive lower bound, positive aggregates in both seeds, and at least 7/10
positive seed-pooled bucket effects. After primary success, rho was competitive
when the rho-ridge lower 95% bound exceeded `-0.05`, and better only when it
exceeded zero. A one-sided rho-control upper bound below `0.08` was the frozen
meaningful-negative criterion. Any `sys > 1` required independent verification.

The generated verdict and technical review are in `evaluation/`. No row had
`sys > 1`; the primary criterion passed, rho was competitive, and the
rho-ridge interval did not support saying rho was better.

## Provenance And Retained Inputs

`pre-target/frozen-selected-candidates-before-sys.jsonl` is the sole target
list. The two per-seed selection manifests and plans document its assembly.
`target-field-audit.txt` records the passed pre-target key-path audit over both
direct feature tables and the combined manifest. `SHA256SUMS` identifies every
retained compact input/output and the source/config/reader identities.

Some immutable generated JSON and audit records retain their original scratch
paths as historical command/input labels. They are not dependencies: this
directory's relative files and `SHA256SUMS` are the portable identity map and
are sufficient for interpretation and integrity checking.

The two deterministic geometry caches (about 213 MB each) and feature tables
(about 148 MB each) are intentionally not tracked here. They are not necessary
to interpret or verify the frozen selection, exact evaluation cache, verdict,
or review: their SHA-256 identities and eligibility counts are retained in
`pre-target/frozen-manifest-summary.json`, and the target reader validates all
evaluation rows against the retained manifest. They are only needed for
row-level pre-target geometry/feature inspection or full regeneration.

## Regeneration

Run from the repository root, choosing a fresh scratch directory in `OUT`.
The frozen configs and source hashes in `SHA256SUMS` must match before treating
a regenerated selection as this packet.

```bash
OUT=/path/to/fresh-scratch/covariance-rho
for seed in 2026071201 2026071202; do
  cargo run --manifest-path experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/Cargo.toml --release -- \
    --config experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/configs/covariance-rho-frozen-seed-$seed.json \
    --stage geometry --out-dir "$OUT/seed-$seed"
  cargo run --manifest-path experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/Cargo.toml --release -- \
    --config experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/configs/covariance-rho-frozen-seed-$seed.json \
    --stage features --out-dir "$OUT/seed-$seed"
  cargo run --manifest-path experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/Cargo.toml --release -- \
    --config experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/configs/covariance-rho-frozen-seed-$seed.json \
    --stage selection --out-dir "$OUT/seed-$seed"
done
python3 experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/assemble_covariance_rho_frozen_manifest.py \
  --root "$OUT" \
  --code experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/src/main.rs \
  --config experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/configs/covariance-rho-frozen-seed-2026071201.json \
  --config experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/configs/covariance-rho-frozen-seed-2026071202.json
```

Audit the two regenerated feature tables and their combined manifest before
any target call with `check_no_target_fields.py`. Capacity evaluation must use
only the frozen per-seed selection files and geometry caches, writing fresh
caches. Then run `analyze_covariance_rho_validation.py` with the combined
manifest and both fresh caches. Do not regenerate, reselection, add rows, or
change decision rules after target evaluation begins.

## Claim Boundary

Allowed: prospective generated-candidate evidence that this exact low-rho rule
enriches `sys` on this random-product height law.

Prohibited: a capacity theorem, a geometric mechanism claim, a direction flip
or subset rule, a calibrated probability of `sys > 1`, transfer beyond this
generator, or a claim that rho beats ridge.
