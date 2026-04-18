# Gradient Analysis: Logbook

Split from the original `gradient-is-zero/` experiment (Phase A: sensitivity analysis + gradient ascent).

## Status

**Active.** Data generated, figures produced.

## How to run

```bash
cargo run -p exp-hko-local-maximum --release --bin hko-gradient-analysis
uv run analyze.py
```

## Files

| File | Role |
|---|---|
| `main.rs` | Rust binary: sensitivity analysis + gradient ascent |
| `analyze.py` | Python figures + analysis |
| `hko-neighborhood-sensitivity.jsonl` | Gradients at HKO2024 (1 row, all 44 orbit gradients inline) |
| `hko-neighborhood-ascent.jsonl` | Gradient ascent trajectory (1 row) |
| `hko-neighborhood-gradient.png` | Bar chart of d_sys/d_h_k |
| `hko-neighborhood-orbits.png` | Orbit structure visualization |

## Next Packet: Exact Certification Bank

### Goal

Add a small exact-certification mode to `hko-gradient-analysis` that uses the
merged `symplectic::exact` kernel on a fixed hand-picked sigma bank and records
continuous-value agreement against the current dyadic/`f64` path.

### Scope

- Add a fixed sigma bank for HKO2024 plus one rational control.
- Add a CLI mode such as `--exact-bank`.
- For each bank entry, compute on both paths when available:
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
- Write a small experiment-owned artifact:
  - smoke/default: `gradient-analysis/smoke-exact-certification-bank.jsonl`
  - explicit canonical refresh only: `gradient-analysis/exact-certification-bank.jsonl`

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
```

The packet is successful when the new mode produces a small exact bank artifact
and the reported continuous-value comparisons are stable on the selected sigmas.
