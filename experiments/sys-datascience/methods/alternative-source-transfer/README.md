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
cargo test --locked --manifest-path \
  experiments/sys-datascience/methods/alternative-source-transfer/Cargo.toml
```

`validate_packet.py` checks immutable SHA-256 identity, exact JSON field
schemas, target-field leakage, source and logical-cell uniqueness,
selected/control geometry uniqueness, exact arm and bucket counts, disjoint
controls, and the 96-row cap. `analyze.py` is the later manifest-gated
post-target reader: it accepts only a complete target file on the frozen union
and does not produce targets or call capacity. Empty, partial, mismatched,
boolean/nonfinite, or formula-inconsistent target artifacts fail closed.
The checked target identity enforces `capacity > 0`, `sys >= 0`, and
`sys = capacity^2/(2 volume)` to relative tolerance `1e-10`.

## Evidence boundary

The `produce`, `features`, `select`, and `validate` commands above do not
compute capacity, `sys`, bounce labels, or target outputs. `analyze.py` only
reads a complete frozen target artifact and never calls capacity. Before
exposure, a clean manifest was an implementation handoff rather than target
authorization or transfer evidence. The historical evaluator recorded below
is the sole target-producing command in this packet; its accepted output is
frozen and must not be rerun over the preserved bytes.

The accepted result supports only the finite, equal-bucket-weighted
rho/control and ridge/control estimands on this single fresh area-normalized
factorial-both source. It does not establish a paired support-height effect,
mechanism, population stability, or transfer to other buckets/laws. Any future
`sys > 1` row would require independent geometry/capacity verification and a
new portfolio decision.

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

## Accepted post-target result

The reviewed target artifact and frozen analysis are preserved under
`artifacts/transfer-v1/target-evaluations.jsonl` (SHA-256
`6016b66c5cad4af948b6d0188ccfa5f1d455b10093e5e2f61d303401fc0082f5`) and
`artifacts/transfer-v1/analysis.json` (SHA-256
`872b932cf38811184104a6bf46afe34f079c291fa5b6e9bc90e05a80df1d407a`). See
[`POST-TARGET-ACCOUNT.md`](POST-TARGET-ACCOUNT.md) and
`artifacts/transfer-v1/result-manifest.json` for the bounded interpretation,
provenance, and stop disposition. Both frozen selectors were classified
`strong_transfer` as finite-design enrichers in both tested buckets; there
were 91 unique targets, five rho/ridge overlaps, shared controls, one seed,
and zero `sys > 1` rows. This does not support threshold, mechanism, causal,
population, superiority, or theorem claims.

## Historical target command / reproduction (do not rerun)

The accepted artifact was produced only after independent review, using this
exact detached clean-worktree command (preserved here for provenance):

```text
git worktree add --detach /tmp/alternative-source-transfer-evaluator-5a573668 5a5736687dcd8ad10f4a682266fa24d1fe067efc
cd /tmp/alternative-source-transfer-evaluator-5a573668
cargo build --release --manifest-path experiments/sys-landscape/Cargo.toml \
  --bin sys-datascience-alternative-source-transfer-evaluator
cargo run --release --manifest-path experiments/sys-landscape/Cargo.toml \
  --bin sys-datascience-alternative-source-transfer-evaluator -- \
  evaluate experiments/sys-datascience/methods/alternative-source-transfer/artifacts/transfer-v1 \
  experiments/sys-datascience/methods/alternative-source-transfer/artifacts/transfer-v1/target-evaluations.jsonl
```

This exact command produced the accepted artifact in the detached clean
worktree reviewed for this packet; it must not be rerun over the preserved
bytes. It validates the three frozen SHA-256 artifacts, joins only the 91
stored selected IDs to their stored exact geometry, evaluates each unique
candidate once, and atomically finalizes schema
`alternative-source-transfer-target-v1`; it refuses to overwrite an existing
finalized target file. There is no build-label or environment-variable trust:
each row records an identity made from the compile-time evaluator source
digest, root `Cargo.lock` digest, the digest of the three capacity-backend
source files, the actual repository `HEAD`, and a clean-checkout flag. The
exact detached worktree build above is commit
`5a5736687dcd8ad10f4a682266fa24d1fe067efc`; `analyze.py` accepts only that
reviewed identity and records the target-file SHA-256 in its analysis JSON. It
has no resume or cache path and never regenerates source/selection.
