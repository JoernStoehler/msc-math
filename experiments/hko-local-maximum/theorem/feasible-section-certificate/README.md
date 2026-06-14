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

The Sage verifier uses the ordered number field where
`t` is the unique real root of `t^4 - 10 t^2 + 5` in `(0,1)`, equivalently
`t = tan(pi/5)`.

## Files

| File | Role |
| --- | --- |
| `main.rs` | Rust `generate` stage for the 26-entry finite selection. |
| `witness.json` | Tracked finite verifier input from Rust. Not proof-facing by itself. |
| `verify.sage.py` | SageMath verifier; computes exact data in memory and checks the exact predicate above. |
| `verification-summary.json` | Verifier result summary. |

## Current Pipeline

Refresh the numerical source diagnostic:

```bash
cargo run -p exp-hko-local-maximum --release --bin hko-active-branch-diagnostic
```

Generate the tracked finite data:

```bash
cargo run -p exp-hko-local-maximum --release --bin hko-feasible-section-generate -- --canonical --input experiments/hko-local-maximum/theorem/active-branch-diagnostic/smoke-active-branch-diagnostic.json
```

`--canonical` deliberately requires `--input`, because the diagnostic source is
ignored local data and should not be hidden in the default command. The input
path is written by the diagnostic command above and is expected to be absent
from git. The tracked pipeline outputs in this folder are `witness.json` and
`verification-summary.json`.

Verify the exact certificate predicate:

```bash
sage -python experiments/hko-local-maximum/theorem/feasible-section-certificate/verify.sage.py
```

Current verifier summary:

- 26 selected entries;
- exact covector rank `25`;
- exact symmetry tangent rank `15`;
- positive exact lambdas summing to `1`;
- exact lambda-weighted covector sum `0`.
