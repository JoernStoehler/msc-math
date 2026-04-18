# Gradient Analysis: Logbook

Split from the original `gradient-is-zero/` experiment (Phase A: sensitivity analysis + gradient ascent).

## Status

**Active.** Phase A data and the exact certification bank are generated.

## How to run

```bash
cargo run -p exp-hko-local-maximum --release --bin hko-gradient-analysis
cargo run -p exp-hko-local-maximum --release --bin hko-gradient-analysis -- --smoke
cargo run -p exp-hko-local-maximum --release --bin hko-gradient-analysis -- --exact-bank
cargo run -p exp-hko-local-maximum --release --bin hko-gradient-analysis -- --exact-bank --canonical
uv run analyze.py
```

## Files

| File | Role |
|---|---|
| `main.rs` | Rust binary: sensitivity analysis + gradient ascent |
| `analyze.py` | Python figures + analysis |
| `hko-neighborhood-sensitivity.jsonl` | Gradients at HKO2024 (1 row, all 44 orbit gradients inline) |
| `hko-neighborhood-ascent.jsonl` | Gradient ascent trajectory (1 row) |
| `exact-certification-bank.jsonl` | Canonical exact-vs-float certification bank for selected sigmas |
| `hko-neighborhood-gradient.png` | Bar chart of d_sys/d_h_k |
| `hko-neighborhood-orbits.png` | Orbit structure visualization |

## Exact Certification Bank

Current mode:
- `--exact-bank` writes `gradient-analysis/smoke-exact-certification-bank.jsonl`
  for smoke/default runs.
- `--exact-bank --canonical` refreshes
  `gradient-analysis/exact-certification-bank.jsonl`.

### Scope

- Fixed sigma bank for HKO2024 plus one rational control.
- For each bank entry, compute on both paths:
  - solve success / admissibility;
  - `q`;
  - action;
  - `beta`;
  - `∂c/∂a`.
- Compare only continuous quantities:
  - `|Δq|`;
  - `|Δaction|`;
  - `|Δbeta|_max`;
  - `|Δ∂c/∂a|_max`.

### Initial Sigma Bank

- HKO winning sigma from the exact sidecar: `[1,8,7,3,4,5,9]`.
- HKO rank-deficient sigma from the algebraic-exactness spike: `[1,7,2,8,4,6,5]`.
- Current float best sigma from the smoke artifact: `[0,1,7,3,9,5]`.
- Two nearby near-optimal HKO sigmas from the smoke artifact:
  `[0,1,7,6,3,9]` and `[0,6,7,2,3,9]`.
- Rational simplex control sigma: `[0,2,1,3,4]`.

### Non-Goals

- No exhaustive exact sigma sweep.
- No pass/fail oracle on incidence, adjacency, or `omega` sign matrices.
- No broad algebraic integration into `library::Polytope4D`.

### Acceptance Checks

```bash
cargo build -p exp-hko-local-maximum --release --bin hko-gradient-analysis
cargo run -p exp-hko-local-maximum --release --bin hko-gradient-analysis -- --smoke
cargo run -p exp-hko-local-maximum --release --bin hko-gradient-analysis -- --exact-bank
cargo run -p exp-hko-local-maximum --release --bin hko-gradient-analysis -- --exact-bank --canonical
```

The current mode is healthy when the exact bank artifact regenerates and the
reported continuous-value comparisons stay stable on the selected sigmas.
