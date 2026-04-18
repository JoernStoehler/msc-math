<!--
Purpose: scope a narrow SageMath cross-check for the exact HKO/local-maximum
program.
Context: the repo now has `real-algebraic`, `symplectic::exact`, and one exact
consumer (`hko-gradient-analysis --exact-bank`). This note defines the first
experiment that uses Sage as an independent oracle instead of building a second
general exact pipeline.
-->

# Sage Validation

## Research question

Does an independent SageMath implementation reproduce the selected exact
single-orbit HKO/control calculations that the Rust exact path currently uses
for certification?

Secondary question:

How expensive is Sage on the same selected exact kernels compared with the Rust
exact path?

## Scope

First packet only:

- selected sigma bank already used by `hko-gradient-analysis --exact-bank`;
- exact one-sigma KKT solve;
- exact action, `q`, `beta`, and `∂c/∂a`;
- small timing baseline on the same selected kernels.

Out of scope for this packet:

- exhaustive exact sigma search;
- exact combinatorics comparisons;
- active-gradient span/rank proof record;
- broad library dependence on Sage.

## Method

1. Rust experiment exports a canonical input artifact for Sage.
2. Each row includes:
   - target polytope;
   - field tag;
   - sigma;
   - dual vertices serialized by canonical coefficient vectors;
   - Rust exact outputs for `q`, action, `beta`, and `∂c/∂a`;
   - Rust kernel timings on the same row.
3. Sage reads that artifact, reconstructs the ordered field and dual vertices,
   solves the same KKT system independently, computes the same derivative data,
   and writes a comparison report.
4. The report records exact equality checks plus `f64` diffs for easier reading.
5. The report also records Sage timing for the same kernel shape.

## Artifacts

| File | Role |
|---|---|
| `experiments/hko-local-maximum/sage-validation/main.rs` | Rust exporter for Sage input rows |
| `experiments/hko-local-maximum/sage-validation/analyze.py` | Sage-driven validator and timing script |
| `experiments/hko-local-maximum/sage-validation/sage-validation-input.jsonl` | Canonical selected-bank exact export |
| `experiments/hko-local-maximum/sage-validation/sage-validation-report.jsonl` | Sage-vs-Rust validation report |

Smoke outputs:

- `smoke-sage-validation-input.jsonl`
- `smoke-sage-validation-report.jsonl`

## Initial bank

Reuse the current exact certification bank entries:

- HKO winning sigma;
- HKO rank-deficient sigma;
- current float-best HKO sigma;
- two nearby near-optimal HKO sigmas;
- simplex control sigma.

## Acceptance checks

```bash
cargo build -p exp-hko-local-maximum --release --bin hko-sage-validation
cargo run -p exp-hko-local-maximum --release --bin hko-sage-validation -- --smoke
cargo run -p exp-hko-local-maximum --release --bin hko-sage-validation -- --canonical
cd experiments/hko-local-maximum/sage-validation && sage -python analyze.py --smoke
cd experiments/hko-local-maximum/sage-validation && sage -python analyze.py --canonical
```

Healthy first packet:

- every selected row reconstructs in Sage;
- every row solves in Sage;
- Sage exact outputs match the Rust exact outputs on the selected bank;
- timing rows exist for both Rust and Sage;
- the experiment note makes the current limitation explicit: this validates the
  selected exact bank, not yet the full active-gradient span/rank record.

## Current observation

Local run on 2026-04-18:

- all 6 selected rows solved in Sage;
- Sage matched Rust exactly on `q`, action, `beta`, and `∂c/∂a` for every row;
- both HKO rank-deficient rows were recovered with kernel dimension `1`;
- the current timing picture is mixed rather than one-sided:
  - HKO solve kernels: Sage and Rust are within the same order of magnitude;
  - HKO gradient kernels: Sage is currently faster on this selected bank;
  - rational simplex control: Rust is currently faster.

## Next packet if this succeeds

Use the same export + Sage-check path for the HKO active-gradient linear-algebra
record behind the 15D flat-space claim.
