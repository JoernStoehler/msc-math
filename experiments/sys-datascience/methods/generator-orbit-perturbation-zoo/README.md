# Orbit and incidence-preserving intervention zoo

This target-free producer begins with fresh reviewed-style Lagrangian products
and emits an identity control plus these intervention laws:

- Haar `U(2)`, Haar `SO(4)`, and the Haar `det=-1` reflection component of
  `O(4)`;
- an explicit `SO(4)` alignment ladder `U1 A_theta U2` at `theta=0,pi/2,pi`,
  with `A_pi=diag(-1,-1,1,1)` anti-symplectic but still in `SO(4)`;
- a bounded Cartan-times-compact `Sp(4)` law which is explicitly not Haar on
  noncompact `Sp(4)`;
- a bounded-Weyl, coordinate-dependent `SL(4)` control (no normalized `GL+`
  duplicate); and
- a fixed-normal type-cone support perturbation, with epsilon derived from
  inactive incidence slacks and rejected/backtracked unless exact labeled
  incidence survives. This local rule is not claimed to be quotient-transverse.

Every row states its probability law and requested preservation contract for
the symplectic form, Euclidean inner product, volume, linear equivalence, face
lattice, and source incidence. It also retains law-specific IDs, seeds,
attempt/rejection counts, matrices or perturbations, condition numbers, exact
reconstruction status, raw ordered dual coordinates, Euclidean and symplectic
dual-Gram response signatures, and labeled incidence. These are redundant
comparison representations for later pair metrics, not a canonical metric.

Run the compact target-free smoke:

```bash
CARGO_TARGET_DIR=/workspaces/msc-math/target cargo run -p exp-sys-landscape \
  --bin sys-datascience-generator-orbit-perturbation-zoo -- \
  --out-dir /tmp/orbit-perturbation-zoo --seed 20260715
```

`rows.jsonl` is the row-level producer artifact; `report.json` records source
identity, resolved smoke parameters, statuses, and the interpretation boundary.
No capacity backend or `sys` value is called. Passing rows show only the
specific geometry/reconstruction contracts checked in that run; they do not
establish invariance of `sys`, a population effect, or a natural quotient law.
