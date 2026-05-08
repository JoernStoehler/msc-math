# algebraic-numbers

Small exact scalar crate for real algebraic numbers in a statically chosen
field `Q[alpha]`.

An `Algebraic<F>` stores rational coefficients in the basis
`1, alpha, ..., alpha^(degree - 1)`. The marker type `F` supplies the monic
minimal polynomial and a rational isolating interval for the chosen real root.

This is deliberately not a general computer-algebra system:

- no runtime parent/ring objects;
- no automatic construction of larger fields;
- no `f64` exact scalar implementation;
- no nalgebra `RealField`/`ComplexField` implementation;
- no matrix solve or diagonalization layer until a caller needs it.

Useful first tests live under `tests/q_sqrt5_*`. The runnable example
`examples/q_sqrt5_vector.rs` shows the intended ergonomics for
`nalgebra::Vector4<Algebraic<Sqrt5>>`.

`SPEC.md` is the normative contract: feature contract and semantic contract.
`ACCEPTANCE.md` is the executable evidence checklist. `DESIGN_NOTES.md` is
non-normative rationale for non-obvious choices and rejected alternatives.

Design notes:

- `ExactScalar` is explicit opt-in. This prevents types from becoming exact
  scalars just because their operator set happens to fit.
- Ordering is exact. It refines the isolating interval for `alpha` until the
  represented polynomial has a determined sign on that interval.
- A tempting shortcut is to evaluate at a float approximation of `alpha`; that
  loses exactly the guarantees this crate is supposed to provide.
