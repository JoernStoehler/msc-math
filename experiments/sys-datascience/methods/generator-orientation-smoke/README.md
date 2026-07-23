# Generator orientation smoke

This target-free producer checks whether the exact reconstruction boundary can
separate Euclidean rigid-motion semantics from symplectic alignment semantics
on area-normalized Lagrangian products. It applies identity, deterministic and
Haar `U(2)`, and deterministic and Haar `SO(4)` maps to each accepted base. It
does not call a capacity backend or compute `sys`.

The real coordinate order is `(q1,q2,p1,p2)`. Matrices act on primal points,
so dual normals are transformed by an explicitly computed inverse transpose.
The deterministic controls are exact signed permutations. Haar `U(2)` uses
complex Gaussian columns followed by modified Gram--Schmidt; Haar `SO(4)` uses
one seeded draw of real Gaussian columns followed by modified Gram--Schmidt,
then flips the final column if needed to obtain positive determinant. The
`SO(4)` draw is unconditioned: its symplectic residual is recorded and a draw
below the declared discrimination threshold fails the packet rather than being
resampled. Both Haar maps are deterministic from the recorded packet/map seeds.

Run the first-gate smoke without retaining artifacts:

```bash
cargo run -p exp-sys-datascience --release \
  --bin sys-datascience-generator-orientation-smoke -- \
  --out-dir /tmp/generator-orientation-smoke \
  --rows-per-bucket 1 --buckets 3x3,4x6
```

The output is `rows.jsonl` plus `report.json`. Exact payload equality is claimed
only for identity and the exact signed-permutation controls. Haar matrices and
their transformed duals are rounded `f64` values which the reconstruction
boundary converts to binary rationals. Consequently, the reconstructed rational
body is not claimed to be exactly `U(2)` or `SO(4)`, and exact omega signs at
structural zeros may change. Haar `U(2)` instead uses a tolerance-aware floating
symplectic-form check. This smoke is semantic/plumbing evidence, not population
evidence and not a `sys` result. Base exhaustion, map-generation rejection,
reconstruction rejection, or any semantic failure is retained in the rows and
report and makes the command exit nonzero after writing those artifacts.

## Retained target-free panel

The reviewed panel uses two independent bases in each of the `3x3`, `4x4`,
`4x6`, and `6x6` buckets, with all five map variants:

```bash
cargo run -p exp-sys-datascience --release \
  --bin sys-datascience-generator-orientation-smoke -- \
  --out-dir experiments/sys-datascience/methods/generator-orientation-smoke/artifacts/panel-2-per-bucket \
  --seed 20260714 --attempts 128 --rows-per-bucket 2 \
  --buckets 3x3,4x4,4x6,6x6
```

All eight bases and all 40 transformed rows reconstructed and passed the
declared checks. Every labeled-incidence signature and exact volume was
preserved within tolerance. The eight Haar `U(2)` maps have symplectic
residual at most `1.52e-15`; rationalization changes exact omega signs at
structural zeros as expected. The eight unconditioned Haar `SO(4)` maps have
symplectic residual between `1.02` and `3.72`, and their omega signatures
change while Euclidean/combinatorial checks remain fixed. Maximum relative
volume change is `1.23e-15`.

These are semantic controls and feasibility evidence. The panel does not yet
show that orientation changes `sys`, does not estimate a population effect,
and does not make the observed ridge association causal. Its next use is as a
reviewed source for a small paired feature/target pilot, not as a result by
itself.
