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
cargo run -p exp-sys-landscape --release \
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
