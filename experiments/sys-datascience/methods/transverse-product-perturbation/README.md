# Same-combinatorics transverse product perturbations

This target-free packet tests whether small seeded perturbations of normalized
dual inequalities `a_i . x <= 1` can leave the exact product face lattice while
moving in a declared Euclidean local section transverse to scale, translation,
and the linear `Sp(4)` orbit. The tangent span is constructed explicitly in
`(q1,q2,p1,p2)` order:

* scale: `δa_i = -a_i`;
* translation by `t`: `δa_i = -(a_i . t)a_i`;
* `X in sp(4)`: `δa_i = -X^T a_i`.

The actual floating-point span is SVD-orthonormalized. Projection onto its
Euclidean orthogonal complement is a declared local section, not a canonical
quotient metric. Finite differences of the exact scale, normalized translation,
and `exp(±hX)` dual actions independently check the formulas.

The retained smoke has one deterministic product fixture for each `3x3`,
`4x4`, `4x6`, and `6x6` facet bucket, three projected directions per fixture,
and identity through several fractions of a binary-searched first exact
incidence/full-dimensionality/validity failure. Every response is reconstructed
through the exact rational cache and checks boundedness, full dimensionality,
irredundancy, volume, labeled incidence, and a volume-normalized reconstruction.
Rows compare direct displacement, the orbit-section residual, Euclidean and
symplectic dual-Gram features, and geometric block-product residuals. Product
combinatorics and geometric/Lagrangian productness are kept distinct.

Run the bounded smoke:

```bash
CARGO_TARGET_DIR=/workspaces/msc-math/target cargo run -p exp-sys-landscape \
  --bin sys-datascience-transverse-product-perturbation -- \
  --out-dir /tmp/transverse-product-perturbation --seed 20260715
```

`rows.jsonl` and `report.json` are target-free geometry artifacts. They do not
evaluate `sys` or capacity, establish a population effect, imply a canonical
quotient metric, or prove that a perturbation law is product-independent.
