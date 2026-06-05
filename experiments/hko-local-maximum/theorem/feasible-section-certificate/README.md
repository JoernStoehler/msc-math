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

The tracked `candidate-certificate.json` is a standalone list of finite row
choices for the exact constructor. Its diagnostic source fields are debug
provenance only.

Sage has two roles.

1. `construct_exact_witness.sage.py` may solve exact systems to produce
   witness values. This constructor is not the proof-facing file.
2. `verify_feasible_section_witness.sage.py` reloads the witness and checks
   matrix-vector equations, vector equality, rank, positivity, action equality,
   symmetry annihilation, and the convex-combination certificate. It uses
   explicit exceptions, not Python `assert`, so the checks are not stripped by
   optimized Python mode.

Both Sage files use the ordered number field where
`t` is the unique real root of `t^4 - 10 t^2 + 5` in `(0,1)`, equivalently
`t = tan(pi/5)`.

The thesis proof should cite the verifier propositions and the formal
implication, not the Rust search method.

## Files

| File | Role |
| --- | --- |
| `main.rs` | Rust exporter for a 26-row candidate selected from the active-branch diagnostic; it checks the selected source rows against hard-coded sigma, minor, fixed-index, and fixed-hint identities before writing. |
| `candidate-certificate.json` | Tracked candidate choices from Rust. Not proof-facing by itself. |
| `construct_exact_witness.sage.py` | Exact witness constructor. It may solve exact systems. |
| `feasible-section-witness.json` | Exact witness values consumed by the verifier. |
| `verify_feasible_section_witness.sage.py` | Exception-based proof-facing verifier. |
| `verification-summary.json` | Verifier result summary. |
| `sanity_check_formulas.sage.py` | Numerical finite-difference smoke checks for formula bug detection. Not proof evidence. |
| `sanity-check-report.md` | Current tracked output of the formula sanity checks. Not proof evidence. |

## Current Pipeline

Refresh the numerical source diagnostic:

```bash
cargo run -p exp-hko-local-maximum --release --bin hko-active-branch-diagnostic
```

Refresh the tracked candidate:

```bash
cargo run -p exp-hko-local-maximum --release --bin hko-feasible-section-certificate -- --canonical --input experiments/hko-local-maximum/theorem/active-branch-diagnostic/smoke-active-branch-diagnostic.json
```

`--canonical` deliberately requires `--input`, because the diagnostic source is
ignored local data and should not be hidden in the default command. The input
path is written by the diagnostic command above and is expected to be absent
from git. The JSON files in this folder, including `candidate-certificate.json`,
`feasible-section-witness.json`, and `verification-summary.json`, are tracked
outputs of the certificate pipeline.

Construct the exact witness:

```bash
sage -python experiments/hko-local-maximum/theorem/feasible-section-certificate/construct_exact_witness.sage.py
```

Verify the exact witness:

```bash
sage -python experiments/hko-local-maximum/theorem/feasible-section-certificate/verify_feasible_section_witness.sage.py
```

### Ancillary Formula Sanity Checks

These checks are not part of the theorem certificate. They are cheap
finite-difference smoke checks for formula, sign, coordinate-order, and
normalization mistakes.

Run the default all-row formula sanity check:

```bash
sage -python experiments/hko-local-maximum/theorem/feasible-section-certificate/sanity_check_formulas.sage.py
```

For a shorter smoke pass, run:

```bash
sage -python experiments/hko-local-maximum/theorem/feasible-section-certificate/sanity_check_formulas.sage.py --rows 3 --directions 1 --eps-min-power 3 --eps-max-power 6
```

The current tracked output is `sanity-check-report.md`.

This script prints deterministic markdown tables. It compares exact derivative
rows against central finite differences for selected witness rows. Full
40-dimensional directions check beta and action derivatives. Lagrangian-product
directions check beta, action, sys, and volume derivatives. The checked formulas
are:

- feasible-section beta derivative;
- feasible-section action derivative;
- systolic upper-branch derivative, including the volume normalization;
- volume derivative along Lagrangian-product-preserving directions, using the
  elementary product of planar areas as the finite-difference volume oracle;
- direct numerical annihilation of the symmetry tangent columns by the selected
  `D sys` rows.

The sanity script is deliberately not part of the proof. It is meant to catch
formula, sign, coordinate-order, and normalization mistakes before thesis
writing. It does not check the full 40-dimensional volume derivative by
independent polytope-volume finite differences; Sage's inexact polyhedron
volume backend was too fragile near the degenerate product point for a cheap
smoke packet.

Current verifier summary:

- 26 selected rows;
- exact row rank `25`;
- exact symmetry tangent rank `15`;
- positive exact lambdas summing to `1`;
- exact lambda-weighted row sum `0`.
