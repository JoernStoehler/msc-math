# CH2021 Six-Vertex Exact Reproduction

This packet reconstructs the rational six-vertex example displayed in
Chaidez--Hutchings (2021), computes its exact geometry, and exhaustively solves
the active-word Haim--Kislev (2017) rational KKT problem. Its purpose is the E3
premise gate: establish whether this one displayed body is an exact
`F = 9`, `sys = 1` equality fixture in the repository.

Run from the repository root:

```bash
cargo run -p dev-capacity-validation --release --bin ch2021-six-vertex
```

The command writes `report.json` in this directory. A release build is
required for the exhaustive run. Exact geometry should take well below a
second; the 125,664 exact KKT solves are expected to take seconds to a few
minutes depending on the machine and build cache. The current pruned and
unpruned f64 routes run afterward as secondary regression checks.

## Frozen predictions

The repaired pre-run probabilities were:

- 65%: exact geometry and exact HK equality pass, both f64 routes agree, and
  there is no Lagrangian 2-face.
- 20%: the same exact equality passes, but an actual Lagrangian 2-face is
  present. This changes the CH ordinary-flow interpretation, not the HK2017
  capacity conclusion.
- 10%: exact geometry and exhaustive exact HK are determinate, but a current
  f64 route fails or disagrees.
- 5%: source-coordinate, rank, polar, incidence, or exact-volume
  reconstruction fails.

These predictions were fixed before target evaluation. They are scientific
priors, not calibrated frequencies.

## Interpretation and stop rule

A passing exact gate certifies only the displayed rational body: it is a
full-dimensional six-vertex, nine-facet polytope with the reported exact
incidence and volume; exhaustive HK2017 gives the reported exact EHZ capacity;
and its systolic ratio is exactly one. HK2017 applies to every convex polytope,
including one with Lagrangian 2-faces.

If an actual 2-face has zero symplectic pairing, the packet does **not** claim
that CH2021's ordinary combinatorial Reeb flow is well posed or that its
ordinary-flow capacity corollary directly applies. The packet also does not
certify the paper's claimed families, combinatorial Zollness, open-dense
coverage by minimizers, local maximality, a stabilizer, orientation response,
or any nearby landscape.

Stop E3 after this one body. On geometry failure, incomplete exact enumeration,
exact identity failure, or f64 mismatch/failure, do not promote an operational
search seed. Preserve a determinate exact mathematical result even when a
secondary f64 route fails. No orientation, perturbation, family, or adaptive
successor run belongs to this packet.

## Sources and conventions

- CH source coordinates and claims:
  `papers/ch2021/s1_introduction_and_main_results.tex`.
- CH ordinary-flow 2-face criterion:
  `papers/ch2021/s3_reeb_dynamics_on_polytopes.tex`.
- General convex-polytope capacity formula:
  `papers/hk2017/EHZ-polytopes.tex`, Theorem 1.1.
- Exact active-word convention and solver contract:
  `formal/hk2017-qp-core.tex`.

Coordinates are read directly as `(q1,q2,p1,p2)`. The arithmetic vertex
centroid is subtracted before polarizing. Polar dual vectors use
`<a_i,x> <= 1`; they are not negated. Exact rational values in the report are
serialized as strings.
