# kkt-error-bounds

## Correspondence Contract

- Formal surfaces: `formal/hk2017-qp-core.tex`,
  `formal/hk2017-qp-precision.tex`.
- Durable Rust surface: `crates/symplectic/src/kkt/`.
- Experiment validation surface: `experiments/numerics/error-bounds/`.
- Independent comparison surface: `experiments/verification/sage/` when a
  reusable exact checker exists.
- Shared semantic claim: the fast KKT solve path, error-bound computations,
  and any lazy exact escalation obey the documented numeric decision contract.

## Verification Contract

- Crate-local tests cover solver smoke/regression cases.
- Broader numeric edge-case and bound-validation suites stay in
  `experiments/numerics/error-bounds/`.
- Known gap during this migration: the exact comparison surface is still
  partially topic-local rather than fully consolidated.
