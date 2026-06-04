# HKO Feasible-Section Certificate

Status: theorem-certificate packet. The current witness verifies exactly in
Sage.

Purpose: turn the feasible-section upper-branch route from
`formal/hko-feasible-section-upper-branches.tex` into a small Rust/Sage
certificate.

## Trust Boundary

Rust is a generator. Rust may search, filter, rank, solve numerically, and
choose candidate rows. Rust output is not a proof object until Sage verifies
the exact propositions below.

Sage has two roles.

1. `construct_exact_witness.sage.py` may solve exact systems to produce
   witness values. This constructor is not the proof-facing file.
2. `verify_feasible_section_witness.sage.py` reloads the witness and checks
   matrix-vector equations, vector equality, rank, positivity, action equality,
   symmetry annihilation, and the convex-combination certificate.

The thesis proof should cite the verifier propositions and the formal
implication, not the Rust search method.

## Files

| File | Role |
| --- | --- |
| `main.rs` | Rust exporter for a 26-row candidate selected from the active-branch diagnostic. |
| `candidate-certificate.json` | Tracked candidate choices from Rust. Not proof-facing by itself. |
| `construct_exact_witness.sage.py` | Exact witness constructor. It may solve exact systems. |
| `feasible-section-witness.json` | Exact witness values consumed by the verifier. |
| `verify_feasible_section_witness.sage.py` | Assert-only proof-facing verifier. |
| `verification-summary.json` | Verifier result summary. |

## Current Pipeline

Refresh the numerical source diagnostic:

```bash
cargo run -p exp-hko-local-maximum --release --bin hko-active-branch-diagnostic
```

Refresh the tracked candidate:

```bash
cargo run -p exp-hko-local-maximum --release --bin hko-feasible-section-certificate -- --canonical
```

Construct the exact witness:

```bash
sage -python experiments/hko-local-maximum/theorem/feasible-section-certificate/construct_exact_witness.sage.py
```

Verify the exact witness:

```bash
sage -python experiments/hko-local-maximum/theorem/feasible-section-certificate/verify_feasible_section_witness.sage.py
```

Current verifier summary:

- 26 selected rows;
- exact row rank `25`;
- exact symmetry tangent rank `15`;
- positive exact lambdas summing to `1`;
- exact lambda-weighted row sum `0`.
