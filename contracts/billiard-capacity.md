# billiard-capacity

## Correspondence Contract

- Formal surface: `formal/library/algorithms.tex:\ref{alg:billiard}`,
  `formal/library/algorithms.tex:\ref{thm:billiard-characterization}`,
  `formal/library/algorithms.tex:\ref{thm:bounce-bound}`.
- Durable Rust surface: `crates/symplectic/src/algorithms/billiard/`.
- Independent comparison surface: `experiments/verification/sage/` once the
  reusable validator is promoted there.
- Shared semantic claim: the billiard-specific search computes the same
  capacity quantity on the Lagrangian-product input class identified by the
  formal assumptions.

## Verification Contract

- Crate-local tests cover smoke/regression behavior on supported inputs.
- Larger comparison and profiling suites belong in `experiments/verification/`
  and experiment-owned benchmark directories.
- Known gap during this migration: no single durable Sage validator file has
  been promoted yet.
