# HKO Local Notes

Status: optional HKO-local technical background. This file is not the entry
point, not a theorem certificate, and not final thesis prose. Use
`README.md` for common context and `theorem/README.md` for the current
certificate interface.

This file keeps breadcrumbs that are useful for HKO theorem/thesis work but do
not belong in formal proof text, command documentation, code comments, or a
generated artifact.

## Selected Upper Branches

The current theorem route does not require a complete catalogue of all HK2017
branches near the HKO point.

For a selected feasible branch value `U_sigma(a)`, the capacity is at most the
selected branch action whenever the branch is valid, so

```text
sys(a) <= U_sigma(a).
```

A sufficient first-order certificate is therefore one-sided: for every nonzero
quotient-slice direction, at least one selected feasible upper branch has
negative first-order change at the HKO point. The exact certificate checks this
through selected rows whose projected covectors have rank `25` and a positive
exact convex relation summing to zero.

Source pointers:

- `theorem/README.md`
- `theorem/verification-summary.json`
- `formal/hko-feasible-section-upper-branches.tex`

## Why Singular Rows Enter

The f64 active-branch diagnostic found `150` positive active rows at the HKO
point. The split is:

- `44` nonsingular six-facet rows;
- `106` singular seven-facet rows.

The nonsingular rows alone do not currently close the slice-rank check: their
projected `D sys` matrix has numerical rank `23` in the `25`-dimensional
quotient slice. The smooth padded-once diagnostic also kept no nonsingular
minimum-action one-zero-beta replacement rows.

This is evidence for why the current certificate uses singular positive-beta
seven-facet rows. It is not a proof that every nonsingular-only repair is
impossible.

The theorem route does not justify those singular rows as nearby optimizing KKT
branches. It uses explicit feasible HK2017 beta sections as upper branches.
Sage verifies closure plus normalization, positivity, full-rank feasible-section
minors, exact action equality, exact derivative rows, symmetry annihilation,
row rank `25`, and a positive exact convex relation.

Source pointers:

- `smooth-only-rank-defect/summary.json`
- `smooth-only-rank-defect/README.md`
- `theorem/active_branch_diagnostic.rs`
- `theorem/README.md`
- `theorem/verification-summary.json`

## Endpoint Rows And The HKO Minimizing Family

Nonsingular active rows are compatible with the HKO minimizing family.

One diagnostic example is the nonsingular row
`sigma = [0, 1, 7, 3, 9, 5]`. Its q-plane boundary points are three pentagon
vertices, and the corresponding q-motion is a triangle. The family direction
can leave this six-facet support chart by adjoining an extra facet whose beta
coordinate is zero at the endpoint and positive along the seven-facet family.
In that enlarged seven-facet chart, the KKT matrix can be singular and carry
the family direction.

This is a thesis-explanation breadcrumb, not a separate certificate. It is
useful if a reader asks why isolated nonsingular rows can coexist with the
HKO2024 minimizing family.

Source pointers:

- `theorem/active_branch_diagnostic.rs`
- `smooth-only-rank-defect/summary.json`

## Old Exact Routes

Older endpoint/segment and representative-expansion routes did not become the
current theorem proof. Their runnable artifacts were removed from the live tree.
Git history is the archive if an old implementation must be inspected.

Current live replacements:

- exact geometry, volume, and symmetry checks are reconstructed by
  `theorem/verify.sage.py`;
- route-choice evidence for the smooth-only obstruction is summarized in
  `smooth-only-rank-defect/summary.json`;
- the theorem-facing certificate is `theorem/witness.json` plus
  `theorem/verify.sage.py`.
