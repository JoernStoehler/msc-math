# Fixed-marginal factor-coupling atlas

This target-free packet asks how much geometric breadth comes from dependence
between two planar factors when their current one-factor law is held fixed.
The current law is the IID fan used by `alternative-generator-smoke`:
independent uniform normal angles and independent uniform support heights in
`[0.8,1.2)`, with rejection until every prescribed facet is active.  This
producer uses an exact-uniform mixture copula separately for angle primitives
and height primitives.  For each primitive draw independent
`U,V~Uniform(0,1)` and `B~Bernoulli(rho)`, then set `U_Q=U` and `U_P=U` when
`B=1`, otherwise `U_P=V`.  The factor-local angle and height streams remain
independent, so each unconditioned one-factor marginal is the current law
exactly and the pair has Pearson dependence `rho`.  Each factor receives an independent uniform global rotation for the
`rotation=uniform` population; fixed `zero` and `pi/4` relative rotations are
diagnostic slices and are never pooled with it.

At `rho=0` the two factors are independent before conditioning.  At `rho=1`
the primitive streams are exactly shared, hence the factors are congruent up to
their relative rotation and separate area normalizations.  The construction
theorem applies before conditioning only: all-active facet rejection and exact
product reconstruction are a coupled selection boundary, so the retained
factor marginal is not claimed to remain identical without a finite-panel
diagnostic.  The report records both the construction controls and this
limitation.

## Artifacts and command

```text
cargo test -p exp-sys-landscape --bin sys-datascience-generator-factor-coupling-atlas
cargo run -p exp-sys-landscape --release --bin sys-datascience-generator-factor-coupling-atlas -- \
  --out-dir experiments/sys-datascience/methods/generator-factor-coupling-atlas/artifacts \
  --seed 20260715 --attempts 64 --rows-per-arm 1
```

The default panel has `rho=0,1/2,0.9,1`, equal-side products `4x4` and `6x6`,
three independent seeds (`seed`, `seed+1`, `seed+2`), and separate uniform,
zero-angle, and `pi/4` relative-rotation populations.  Each retained row has
linked `pairing_id`, exact-after-rationalization product volume, canonical
factor normal/height payloads, a rotation-quotiented factor distance, and
primitive uniform streams.  `batch-report.json` contains attempt counts,
failures, endpoint/corruption controls, finite-panel dependence summaries,
source and `Cargo.lock` hashes, and the output-row hash.  `manifest.json` adds
the report hash and stable replay command.  No target or `sys` value is
evaluated, and no rho is selected or ranked.

Replay verification uses two clean output directories and byte comparisons:

```text
cargo run -p exp-sys-landscape --release --bin sys-datascience-generator-factor-coupling-atlas -- --out-dir /tmp/factor-a --seed 20260715 --attempts 64 --rows-per-arm 1
cargo run -p exp-sys-landscape --release --bin sys-datascience-generator-factor-coupling-atlas -- --out-dir /tmp/factor-b --seed 20260715 --attempts 64 --rows-per-arm 1
cmp /tmp/factor-a/coupling-rows.jsonl /tmp/factor-b/coupling-rows.jsonl
cmp /tmp/factor-a/batch-report.json /tmp/factor-b/batch-report.json
```

Allowed use is generator plumbing, marginal/dependence diagnostics, and
geometric hypothesis formation.  The packet does not establish exchangeability
after conditioning, monotonicity in `rho`, a best coupling, or any target
transfer result.  Exact feature arms beyond product volume are deliberately
deferred; this keeps the first packet cheap and copy-local.
