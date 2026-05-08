# algebraic-numbers acceptance checks

This file is the executable evidence checklist for `SPEC.md`. It is not the
spec: passing these checks does not replace review of the feature and semantic
contracts.

## Commands

Run:

```bash
cargo test -p algebraic-numbers
cargo clippy -p algebraic-numbers --all-targets -- -D warnings
cargo run -p algebraic-numbers --example q_sqrt5_vector
```

## Required Evidence Surface

The `q_sqrt5_*` tests and example must cover exactly the current public
ergonomics target:

- `Vector4<Qsqrt5> + Vector4<Qsqrt5>`;
- `alpha * alpha == 5` in `Q[sqrt(5)]`;
- `2 * alpha`, `alpha * BigRational`, and `BigRational * alpha`;
- exact sign and ordering around `2 < sqrt(5) < 3`;
- division by a nonzero `Algebraic<F>`.
