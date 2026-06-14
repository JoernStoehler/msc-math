# Smooth-Only Rank Defect

This directory records the f64 evidence for the clean failed route attempt:
use only the positive-beta active branches whose KKT matrix is nonsingular at
the HKO point.

That attempt does not currently close the first-order certificate. In the
current active-branch diagnostic, those `44` nonsingular branches give a
projected `D sys` matrix of numerical rank `23` in the `25`-dimensional quotient
slice. The all-row feasible-section route reaches rank `25`.

This is evidence from the Rust diagnostic, not theorem proof. The theorem proof
still uses `../theorem/witness.json`, `../theorem/verify.sage.py`, and
`formal/hko-feasible-section-upper-branches.tex`.

Run:

```bash
cargo run -p exp-hko-local-maximum --release --bin hko-active-branch-diagnostic
cargo run -p exp-hko-local-maximum --release --bin hko-smooth-only-rank-defect
```

The first command writes the ignored source diagnostic
`../theorem/smoke-active-branch-diagnostic.json`. The second command reads that
diagnostic and writes `summary.json`.
