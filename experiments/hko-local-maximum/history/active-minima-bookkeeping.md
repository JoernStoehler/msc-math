# Active-Minima Bookkeeping

Status: route-history and thesis-explanation note. This is not a proof object.

The current HKO proof route uses explicit feasible HK2017 beta sections as
upper branches. It does not justify singular rows as nearby optimizing KKT
branches.

Facts retained for thesis navigation:

- The current diagnostic records `150` active rows at the HKO point.
- Of these, `44` are nonsingular six-facet rows and `106` are singular
  seven-facet rows.
- The nonsingular rows alone cover rank `23` of the `25`-dimensional quotient
  slice in the f64 diagnostic.
- The padded-once diagnostic did not find a nonsingular minimum-action
  replacement route.
- This is why the current theorem route keeps singular positive-beta
  seven-facet rows, but interprets them through explicit feasible beta
  sections.

The theorem-facing replacement is the feasible-section certificate in
`../theorem/`. Its Sage verifier checks positivity,
closure plus normalization, selected full-rank minors, action equality,
feasible-section derivative equations, symmetry annihilation, row rank `25`,
and a positive exact convex relation.

Source pointers:

- `research/hko-local-maximum-status.md`
- `research/hko-local-maximum-proof-route-note.md`
- `research/hko-local-maximum-proof-control-packet.md`
- `../theorem/README.md`
- `../theorem/verification-summary.json`
- `../theorem/active_branch_diagnostic.rs`
- `../smooth-only-rank-defect/summary.json`
