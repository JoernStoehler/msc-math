# Orbit and incidence-preserving intervention zoo

This target-free producer begins with fresh reviewed-style Lagrangian products
and emits an identity control plus these intervention laws:

- Haar `U(2)`, Haar `SO(4)`, and the Haar `det=-1` reflection component of
  `O(4)`;
- an explicit `SO(4)` alignment ladder `U1 A_theta U2` at `theta=0,pi/2,pi`,
  with `A_pi=diag(-1,-1,1,1)` anti-symplectic but still in `SO(4)`;
- a restricted one-sided diagonal-times-`U(2)` `Sp(4)` intervention, with two
  independently bounded diagonal parameters. It is not Haar and not a generic
  `KAK`/double-compact Cartan law on noncompact `Sp(4)`;
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

`rows.jsonl` is the row-level producer artifact. Before writing either retained
file, `report.json` records the full repository `HEAD` revision and tree plus a
repo-wide tracked-clean predicate (untracked outputs ignored); these bind every
tracked path dependency rather than a short hand-maintained list. The freeze
therefore identifies the producer source commit before the generated artifact
is committed, avoiding self-reference. Producer and lockfile SHA-256 values
are convenient local checks, not the source-closure definition, and no
deletable `target/` binary is the executable identity. Timing fields are
one-run observations, not byte-reproducible freeze data; non-timing rows are
deterministic under the recorded seed and pinned source closure.
No capacity backend or `sys` value is called. Passing rows show only the
specific geometry/reconstruction contracts checked in that run; they do not
establish invariance of `sys`, a population effect, or a natural quotient law.
