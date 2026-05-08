# algebraic-numbers

Exact scalar crate for real algebraic numbers in a statically chosen field
`Q[alpha]`.

An `Algebraic<F>` stores rational coefficients in the basis
`1, alpha, ..., alpha^(degree - 1)`. The marker type `F` supplies the monic
minimal polynomial and a rational isolating interval for the chosen real root.

Current scope excludes:

- no runtime parent/ring objects;
- no automatic construction of larger fields;
- no `f64` exact scalar implementation;
- no nalgebra `RealField`/`ComplexField` implementation;
- no matrix solve or diagonalization layer until a caller needs it.

Current first tests live under `tests/q_sqrt5_*`. The runnable example
`examples/q_sqrt5_vector.rs` is the current witness for
`nalgebra::Vector4<Algebraic<Sqrt5>>` ergonomics.

Developer spec, architecture notes, verification commands, and rejected
alternatives live in `DEVELOPMENT.md`.
