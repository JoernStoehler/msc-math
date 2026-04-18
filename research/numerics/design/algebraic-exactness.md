<!--
Purpose: design note for the experiment-first algebraic exactness spike.
Context: implements an experiment-owned exact geometry + selected-KKT path for
HKO-style inputs with algebraic dual-vertex coordinates, without changing the
library core.
-->

# Algebraic Exactness

## Research Question

Can we get proof-grade exact geometry and selected exact KKT certification for
HKO-style polytopes whose intended coordinates live in an algebraic extension of
`Q`, while keeping the main library and its rational caches unchanged?

## Method

- Implement an experiment-owned exact ordered-field interface in
  `experiments/numerics/src/algebraic/`.
- Implement one concrete field first:
  `Q[t]/(t^4 - 10 t^2 + 5)` with `t = tan(pi/5)`.
- Construct exact 4D polytopes over that field, including HKO's dual vertices.
- Run selected exact KKT solves on:
  - rational controls;
  - an HKO capacity-achieving sigma;
  - an HKO rank-deficient sigma.
- Compare the exact algebraic results against the current dyadic-rational HKO
  path in the library.

## Generated Data

- `experiments/numerics/algebraic-exactness/exact-polytopes.jsonl`
- `experiments/numerics/algebraic-exactness/exact-kkt-comparison.jsonl`

## Success Criteria

- Exact HKO construction does not rely on `Polytope4D::from_f64`.
- Rational controls reproduce the expected exact geometry and exact KKT values.
- The selected HKO exact KKT rows run over the pentagon field and produce a
  stable `f64` comparison against the current library approximation.

## Non-Goals

- No migration of `library::Polytope4D`.
- No change to `library/src/database.rs` or the mirrored rational JSONL caches.
- No arbitrary finite-extension backend in v1.
- No exhaustive exact HKO sigma sweep in v1.

## Commands

```bash
cargo build -p dev-numerical-analysis --release
cargo run -p dev-numerical-analysis --release --bin num-algebraic-exactness
```

## Follow-Up Design Note

- Scalar-API planning for a reusable real-algebraic arithmetic layer lives in
  [algebraic-scalar-api.md](/workspaces/msc-math/.codex/worktrees/algebraic-exactness-spike/research/numerics/design/algebraic-scalar-api.md).
