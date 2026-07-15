# Alternative-source tangential ridge/rho transfer

This method owns the target-free `alternative-source-transfer-v1` packet. It
is deliberately narrower than the stopped generator line: only the jointly
admissible, separately area-normalized `factorial-both` law is implemented,
for buckets `4x6` and `6x6`, with master seed `2026071601`. For each bucket the
producer scans row indices `0..4000`, up to 128 deterministic attempts per
row, and retains the first 3,200 exact-feature-eligible rows in row order.
Incomplete buckets fail closed.

The producer stores exact dual/primal geometry, incidence, exact volume, and a
fingerprint. The feature stage computes only target-free canonical vertex
covariance rho and the normalized symplectic ridge sum/max-share fields. The
selector freezes bottom 0.005 rho (16 memberships), then the bottom 0.01 ridge
sum followed by bottom 0.5 max-share (16 memberships), plus one disjoint hash
control of 16 per bucket using domain
`frozen-canonical-vertex-covariance-control-v1` and seed `2026071299`.
Overlaps remain memberships and geometry is retained once; no backfill is
performed. The frozen union is at most 96 unique rows.

## Commands

From the repository root, target-free production is:

```text
cargo run --release --manifest-path experiments/sys-landscape/Cargo.toml \
  --bin sys-datascience-alternative-source-transfer -- produce OUT
cargo run --release --manifest-path experiments/sys-landscape/Cargo.toml \
  --bin sys-datascience-alternative-source-transfer -- features OUT
cargo run --release --manifest-path experiments/sys-landscape/Cargo.toml \
  --bin sys-datascience-alternative-source-transfer -- select OUT
cargo run --release --manifest-path experiments/sys-landscape/Cargo.toml \
  --bin sys-datascience-alternative-source-transfer -- validate OUT
python3 experiments/sys-datascience/methods/alternative-source-transfer/validate_packet.py OUT --validate-only
```

`validate_packet.py` checks SHA-256 identity, target-field leakage, source and
logical-cell uniqueness, selected/control geometry uniqueness, exact arm
counts, disjoint controls, and the 96-row cap. `analyze.py` is the later
manifest-gated post-target reader: it accepts only a complete target file on
the frozen union and does not produce targets or call capacity. Empty,
partial, mismatched, or nonnumeric target artifacts fail closed.

## Evidence boundary

No command in this method computes capacity, `sys`, bounce labels, or target
outputs. A clean manifest is a pre-target implementation handoff, not target
authorization or evidence of transfer. Later results may support only the
finite, equal-bucket-weighted rho/control and ridge/control estimands on this
single fresh area-normalized factorial-both source. They do not establish a
paired support-height effect, mechanism, population stability, or transfer to
other buckets/laws. Any `sys > 1` row requires independent geometry/capacity
verification and the portfolio owner's decision.

## Narrow provenance

The law/feature semantics are a narrow fresh-domain translation of the
reviewed `factorial-both` construction from `research/generator-transfer`
commit `fd9c3e7df08d8c9d04491b8ebbb7b2628d2df32e`:
joint latent baseline/tangential admissibility, common support height one,
separate factor area normalization, exact product reconstruction, canonical
vertex covariance, and ordered two-face symplectic area. The breadth generator
families, target backend, orientation arms, and paired baseline/q/p arms are
not imported. The BLAKE3 seed preimage places the latent-law identity before
the master seed (the reviewed owner places the master seed first); this is an
intentional fresh-domain byte-serialization translation, not a byte-exact
replay. It leaves the named IID random-law and conditioning estimand unchanged
and is covered by the semantic construction tests. The current producer
revision and Cargo lock hash are recorded in `manifest.json`.

## Future target command (unauthorized)

Only after a second independent pre-target review returns `GO` may the exact
stored-geometry evaluator be run:

```text
TRANSFER_EVALUATOR_BUILD=<immutable-build-id> \
cargo run --release --manifest-path experiments/sys-landscape/Cargo.toml \
  --bin sys-datascience-alternative-source-transfer-evaluator -- \
  evaluate experiments/sys-datascience/methods/alternative-source-transfer/artifacts/transfer-v1 \
  experiments/sys-datascience/methods/alternative-source-transfer/artifacts/transfer-v1/target-evaluations.jsonl
```

This command is not authorized or run in the current repair. It validates the
three frozen SHA-256 artifacts, joins only the 91 stored selected IDs to their
stored exact geometry, evaluates each unique candidate once, and atomically
finalizes schema `alternative-source-transfer-target-v1`. It has no resume or
cache path and never regenerates source/selection. `analyze.py` may consume the
completed target file only after that gate and writes a report atomically when
passed `--write`.
