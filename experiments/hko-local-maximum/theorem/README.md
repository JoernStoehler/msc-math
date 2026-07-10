# HKO Feasible-Section Certificate

Status: theorem-certificate packet. SageMath verifies the exact finite
certificate predicate.

Purpose: turn the feasible-section upper-branch route from
`formal/hko-feasible-section-upper-branches.tex` into a small Rust/Sage
certificate.

## Architecture

The pipeline has two stages.

1. `generate`: Rust writes `witness.json`, the finite verifier input selected
   from the active-branch diagnostic. Rust is not trusted for theorem
   correctness.
2. `verify`: SageMath reads `witness.json`, computes the needed exact
   algebraic data in memory, checks the exact finite certificate predicate, and
   writes `verification-summary.json`.

The theorem proof uses the verified predicate together with
`formal/hko-feasible-section-upper-branches.tex`. It does not trust the Rust
search method.

Shared exact-arithmetic contract: Rust selects finite data only. SageMath
reconstructs the HKO source objects and verifies them over the ordered number
field `Q(t)`, where `t` is the unique real root of
`t^4 - 10 t^2 + 5` in `(0,1)`, equivalently `t = tan(pi/5)`.

## Theorem Target

The intended theorem target is local maximality of HKO2024 in the ten-facet
dual-vertex chart, modulo translations, scaling, and the identity-component
linear symplectic action. It is not raw strict local maximality in ambient
`R^40`, and it is not a broad all-facet-count local maximality theorem.

The selected-row proof route is one-sided. It proves enough selected feasible
upper branches to force strict negative first-order change in every nonzero
quotient-slice direction. It does not require a complete catalogue of all
nearby HK2017 branches.

## Verifier Predicate

`verify.sage.py` reconstructs the HKO source objects from definitions, reads
`witness.json`, and checks:

- the ordered number field, HKO dual vertices, volume, volume derivative, and
  symmetry tangent generators;
- 26 selected HK2017 words with exact positive beta data, closure plus
  normalization, invertible feasible-section minors, and HKO action equality;
- exact feasible-section derivative equations and exact `D sys` covectors;
- exact annihilation of the 15-dimensional symmetry tangent space;
- exact covector rank `25`;
- positive exact coefficients summing to `1` whose weighted covector sum is
  `0`.

The verifier uses explicit exceptions, not Python `assert`, so the checks are
not stripped by optimized Python mode.

The witness file is an interface for exact verification, not a floating-point
proof object:

| Witness data | Trust role |
| --- | --- |
| `packet`, `witness_version`, `certificate_goal` | Packet identity and expected dimensions/counts checked by Sage. |
| `sigma`, `minor_columns_exact`, `fixed_beta_indices` | Proof-facing finite choices for each feasible-section chart. |
| `fixed_beta_values_f64` | Candidate-ordering hints; Sage reconstructs exact values over the ordered field before acceptance. |
| `action_f64`, `q_f64`, `beta_f64`, `d_sys_flat_f64`, `lambda_hint_f64`, `source_*` | Diagnostics and provenance only; they are not trusted as proof data. |
| `verification-summary.json` | Output of the exact verifier after all row, rank, and convex-certificate checks pass. |

The feasible-section rows, including the singular seven-facet rows, are checked
as explicit feasible upper branches. The certificate does not justify singular
rows as nearby optimizing KKT branches and does not trust f64 KKT-derived
gradients for theorem correctness.

## Files

| File | Role |
| --- | --- |
| `active_branch_diagnostic.rs` | Rust diagnostic generator for the local smoke input used by `generate.rs`. |
| `generate.rs` | Rust `generate` stage for the 26-entry finite witness selection. |
| `witness.json` | Tracked finite verifier input from Rust. Not proof-facing by itself. |
| `verify.sage.py` | SageMath verifier; computes exact data in memory and checks the exact predicate above. |
| `verify.sage.py.explained.md` | Explained full-source listing of `verify.sage.py`, organized by mathematical proof obligation. |
| `verification-summary.json` | Verifier result summary. |

## Current Pipeline

Refresh the numerical source diagnostic:

```bash
cargo run -p exp-hko-local-maximum --release --bin hko-active-branch-diagnostic
```

Generate the tracked finite data:

```bash
cargo run -p exp-hko-local-maximum --release --bin hko-feasible-section-generate -- --canonical --input experiments/hko-local-maximum/theorem/smoke-active-branch-diagnostic.json
```

`--canonical` deliberately requires `--input`, because the diagnostic source is
ignored local data and should not be hidden in the default command. The input
path is written by the diagnostic command above and is expected to be absent
from git. The tracked pipeline outputs in this folder are `witness.json` and
`verification-summary.json`.

Verify the exact certificate predicate:

```bash
sage -python experiments/hko-local-maximum/theorem/verify.sage.py
```

Current verifier summary:

- 26 selected entries;
- exact covector rank `25`;
- exact symmetry tangent rank `15`;
- positive exact lambdas summing to `1`;
- exact lambda-weighted covector sum `0`.
