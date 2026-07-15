# Matched Cartan anisotropy intervention

This target-free packet separates Euclidean singular-value magnitude from the
pairing of expanding and contracting directions by the fixed symplectic form.
It consumes the eight retained identity bases from
`generator-orientation-smoke/artifacts/panel-2-per-bucket/rows.jsonl` rather
than regenerating them.  The source bytes, base IDs, and exact geometry IDs are
bound in `report.json`.

For every base and every frozen rational level `t in {1, 5/4, 3/2, 2}`, the
producer emits the same-base pair

```
S_t = diag(t, t^-1, t^-1, t),   S_t^T J S_t = J,
N_t = diag(t, t^-1, t, t^-1),   det(N_t) = 1.
```

Both arms have the same singular-value multiset
`{t,t,t^-1,t^-1}`.  Their canonical pair weights for `(q1,p1)` and `(q2,p2)`
are `(1,1)` and `(t^2,t^-2)`, respectively.  The exact positive-diagonal
control in `report.json` verifies the quotient identity: for positive diagonal
determinant-one `D`, the unique representative is `N_t` with
`t^2=d1*d3=d2^-1*d4^-1`; this is only the diagonal Cartan quotient, not a
classification of `Sp(4)\\SL(4)\\Sp(4)`.  The report's exact noncanonical
fixture is `D=diag(1,1,4,1/4)`, `t=2`, and
`A=diag(1/2,2,2,1/2)`, with checks for positive determinant-one input,
`A^T J A=J`, nonidentity `A`, and exact reconstruction `D=A N_t`.

## Run and artifacts

From the repository root, after building the current clean source revision:

```bash
cargo run -p exp-sys-landscape --release \
  --bin sys-datascience-generator-cartan-anisotropy -- \
  --out-dir experiments/sys-datascience/methods/generator-cartan-anisotropy/artifacts/panel-2-per-bucket
```

The retained packet contains:

- `rows.jsonl`: 64 rows (8 bases × 4 levels × 2 arms), with exact-rational
  matrices and pair weights, determinant, singular values, exact/floating
  symplectic residuals, reconstruction/incidence/volume checks, and direct
  Euclidean/symplectic signatures plus compact exact-feature responses;
- `paired.jsonl` and `paired.tsv`: 32 matched S-versus-N rows, never pooled
  across bases or levels; TSV is the compact human-readable table;
- `report.json`: source revision/tree and tracked-clean state, source and
  producer hashes, row hashes/counts, the quotient control, and interpretation
  boundaries.

Replay is byte-checked by running the same command into a temporary directory
and comparing the two row/pair hashes in `report.json` (timings are not stored).
The retained-schema check is also runnable with
`python3 experiments/sys-datascience/methods/generator-cartan-anisotropy/test_packet.py`.
The command fails closed on source-panel loss, duplicate identities,
reconstruction failure, incidence/volume failure, or any missing requested row.

## What this packet can and cannot say

The retained geometry shows whether matched singular spectra can nevertheless
produce distinct symplectic signatures, whether the chosen `t` ladder is
monotone in the target-free feature view, and whether base-level sign changes
occur.  It is a finite paired geometry intervention, not a population sample.

It does **not** evaluate `sys` or capacity, make a target or tail claim, infer a
population effect, claim an intrinsic quotient distance, classify the full
double coset, or treat the symplectic control as new coverage for consumers
that are orbit-invariant.  Any later target pilot must be separately frozen and
reviewed against these exact paired IDs.

The retained rational source payload is transformed by exact rational
inverse-transpose matrices and reconstructed through the exact polar API.
Stored exact dual coordinates, incidence, volume, and symplectic signatures
are checked before f64 diagnostic summaries are emitted.  Floating singular
values and residual norms remain diagnostic views only.

The current row hashes intentionally replace the prior f64-roundtrip packet:
the old rows hash began `f621f33a`, while the exact-path rows hash begins
`4eea1d9c`.  The report records this regeneration reason.
