# Sources and exact dependency roles

## [HK] Capacity theorem — mathematical input

Pazit Haim-Kislev, *On the symplectic size of convex polytopes*,
arXiv:1712.03494v3, 4 January 2019, Theorem 1.1.
https://arxiv.org/html/1712.03494v3

The proof uses the finite inverse-max formula after absorbing support heights
into the normal rows. Reversing all words handles the source's orientation
convention. The product reduction, feasible-section construction, and local
Taylor argument in this packet are explained deductions, not extra claims
attributed to this paper.

## [BMP] Discovery motivation — not a proof dependency

Alexey Balitskiy, Ivan Mitrofanov, Alexander Polyanskii,
*Triangle covering problems and the Viterbo inequality in the plane*,
arXiv:2603.12495v1, 12 March 2026, especially Section 5.2.
https://arxiv.org/html/2603.12495v1

Its quadrilateral/hexagon equality construction motivated the rational product
used here. The paper's product-class conclusions are not promoted to a theorem
about arbitrary nearby four-dimensional bodies. The packet independently
checks the specific equality family and capacity and supplies its own full
normal-chart local certificate. No bibliographic priority claim is made for
the underlying equality family.

## [HKO] Excluded comparison body

Pazit Haim-Kislev and Yaron Ostrover, *A Counterexample to Viterbo's Conjecture*,
arXiv:2405.16513v3.
https://arxiv.org/html/2405.16513v3

The user supplied the HKO ratio (3+sqrt(5))/5, ten facets, 25 vertices, and
accepted its fixed-ten-facet local maximality. HKO was calibrated numerically,
not reproved. Its vertex count separates the final example even under general
invertible affine transformations.

## [REPO] Inspected repository — guidance and calibration context

Jörn Stöhler, `JoernStoehler/msc-math`, pinned commit
`3929a8c7f0f766640ea78372848a5752a0ae7e08`.
https://github.com/JoernStoehler/msc-math/tree/3929a8c7f0f766640ea78372848a5752a0ae7e08

The connected GitHub tool confirmed this commit, its author timestamp
2026-09-19T08:04:13Z, and tree
`b6742251d1c34d6e103e1ebfb7c9e1898fb5f86e`.
The following were read at that revision:

- `docs/capacity-calculation-map.md` — full calculation-route and trust-boundary map.
- `experiments/local-maxima-check/control-calibration/PILOT-REPORT.md` — retained
  candidate statuses, including the distinction between finite probes and proof.
- `crates/symplectic/src/algorithms/capacity_4d/general.rs`, lines 1–250.
- `crates/symplectic/src/algorithms/capacity_4d/geometry.rs`, lines 1–260.

No repository checkout was compiled or modified. The production Rust capacity
route was not executed. This packet instead uses the supplied offline bridge
for discovery and a small, independently written exact product reduction and
candidate-specific certificate for final proof. Its results must not be
misattributed to the repository's production evaluator.

## Supplied handoff

The uploaded ZIP is preserved as `inputs/original_handoff.zip`. Its GOAL,
prompt, mathematical context, repository map, references, and provenance are
also available under `inputs/`. `discovery/code/localmax_tools.py` is the supplied
offline diagnostic bridge, retained without presenting it as a capacity oracle.
The six supplied calibration tests were actually rerun; their stdout is retained.

All external pages above were consulted in the execution session on
22 September 2026. Their HTML representations, not downloaded PDFs, were read.
No article or external figure is redistributed in this package.
