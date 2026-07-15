# Fixed-marginal factor-coupling atlas

This target-free packet asks how much geometric breadth comes from dependence
between two planar factors when their current one-factor law is held fixed.
The current law is the IID fan used by `alternative-generator-smoke`:
independent uniform normal angles and independent uniform support heights in
`[0.8,1.2)`, with rejection until every prescribed facet is active.  This
producer uses a Gaussian copula separately for angle primitives and height
primitives.  For each primitive, `Z_Q,Z_P` are standard normal with
`Corr(Z_Q,Z_P)=rho`, and `U=Phi(Z)`.  The factor-local angle and height streams
remain independent, so each unconditioned one-factor marginal is the current
law.  Each factor receives an independent uniform global rotation for the
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
linked `pairing_id`, exact product volume, factor shape summaries, a
rotation-quotiented factor distance, generation/validation cost, and primitive
construction controls.  `batch-report.json` contains attempt counts,
failures, endpoint/corruption controls, finite-panel dependence summaries, and
the source revision/tree captured before output creation.  No target or `sys`
value is evaluated, and no rho is selected or ranked.

Allowed use is generator plumbing, marginal/dependence diagnostics, and
geometric hypothesis formation.  The packet does not establish exchangeability
after conditioning, monotonicity in `rho`, a best coupling, or any target
transfer result.  Exact feature arms beyond product volume are deliberately
deferred; this keeps the first packet cheap and copy-local.
