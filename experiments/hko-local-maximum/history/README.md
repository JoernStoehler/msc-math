# HKO Route History

This directory records route-history facts for the HKO local-maximum theorem
work. It is not the current theorem certificate.

The current theorem-facing certificate is
`../theorem/`. That packet uses Rust only as a generator: Sage reads
`witness.json`, computes exact data internally, verifies the exact finite
predicate, and
`formal/hko-feasible-section-upper-branches.tex` records the mathematical
implication from the verified finite certificate to the quotient-local theorem.

These notes preserve the parts of the older exact representative route that
still cash out in thesis success:

- the exact HKO field, coordinate, geometry, volume-row, and symmetry-rank
  setup;
- the active-minima bookkeeping that explains why singular seven-facet rows
  matter;
- the reason the older representative route did not close the theorem.

The raw older scripts and generated artifacts remain in `exact-witness/`
for now. They should not be cited as the current theorem certificate.
