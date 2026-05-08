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

Developer contract, verification commands, and rejected alternatives live in
`DEVELOPMENT.md`.
