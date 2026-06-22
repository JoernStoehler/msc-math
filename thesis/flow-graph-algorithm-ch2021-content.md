# Flow-Graph Algorithm Proof-Recovery Notes

Status: section-local recovery note for
`thesis/flow-graph-algorithm-ch2021.tex`. Not source truth and not active
thesis prose.

Live algorithm/control source:
`crates/symplectic/src/algorithms/flow_graph/README.md`.

The active `.tex` file currently contains only the scaffold. Do not use this
file to write or approve reader-facing FG correctness prose. The next FG thesis
work is proof recovery: compare the May tube proof material with the current
`flow_graph` implementation, then decide what theorem or implementation-status
statement the thesis can honestly contain.

## Current Use

Use this note to avoid rediscovering where the proof material and code
boundaries are. If this file and the live flow-graph README disagree, refresh
this file from the README, code, tests, and validation results.

Source-backed implementation facts that may matter for proof recovery:

- HK2017 Theorem 1.5 (`simple_loop_theorem` in
  `papers/hk2017/EHZ-polytopes.tex`, indexed in `papers/citation-index.md`)
  already proves that a minimum-action closed characteristic on a convex
  polytope may be chosen simple; thesis legacy restates this as
  `thm:simple-minimizer` in `thesis/legacy/simple-minimizer-existence.tex`;
- `research/tube-algorithm.md` records Jörn's accepted clarification that
  nonzero `omega_0` is only needed on geometrically possible trajectory
  transitions; the algorithm does not require every facet pair to have
  nonzero `omega_0`;
- `research/tube-algorithm-raw-jorn-2026-05-04.md` records the raw tube model,
  including action as elapsed time along stored Reeb segments, tube gluing,
  action cutoff by a halfspace, and exhaustive simple-word search;
- exact closed-word output now reconstructs segment times and requires every
  segment time to be strictly positive before returning `PositiveOrbit`;
- exact search rejects geometrically possible zero-`omega_0` transitions before
  enumeration, including HKO and Lagrangian-product fixtures covered by tests;
- f64 diagnostic output and `capacity_f64` are not exact certificates; direct
  f64 positive words remain f64 outputs, while f64 error words receive exact
  closed-word status only after exact resolution;
- exact flow-graph retained-word checks are flow-graph-internal and do not use
  HK/QP as a retained-word oracle.

Thesis-role points discussed with Jörn but not yet turned into active prose:

- FG is meant to be a second serious implemented approach, not a failed route;
- FG can provide an independent scalar comparison against certified HK/QP on
  eligible examples;
- QP remains the practical workhorse because this FG implementation has narrower
  inputs, explicit degeneracy rejections, and no demonstrated performance
  advantage.

## Source Pointers

- Algorithm/control: `crates/symplectic/src/algorithms/flow_graph/README.md`
- Exact search: `crates/symplectic/src/algorithms/flow_graph/exact_search.rs`
- Exact closed words/tubes: `crates/symplectic/src/algorithms/flow_graph/exact_tube.rs`
- f64 diagnostics/fallback: `crates/symplectic/src/algorithms/flow_graph/f64_tube_search.rs`
- f64/rejection tests: `crates/symplectic/src/algorithms/flow_graph/tests_e2e_prediction.rs`
- Experiments: `experiments/dev-flow-graph/README.md`
- CH2021 background in thesis: `thesis/generalized-reeb-orbits-polytopes.tex`
- Paper source: `papers/ch2021/`

## Legacy Proof Surface Pointers

The May 2026 tube/flow-graph proof material is not lost, but it is not active
source truth. `research/tube-algorithm-raw-jorn-2026-05-04.md` preserves the
raw Jörn note: tube definition, affine primitive maps, tube composition, action
cutoff, closed fixed-point solving, and finite simple-word enumeration.

Deleted history contains a larger formal surface:

- latest inspected formal version:
  `git show 25dd8b9acb8aeaaa6aa3abd80fc6d95db00c4747:formal/tube-algorithm.tex`;
- related implementation predecessor:
  `git show 0ef7ab86f4685e574929c27777ab3030d12a3ba0:crates/symplectic/src/algorithms/tube/mod.rs`;
- cleanup/import commit:
  `69b3a50afa148c12bab18db2503b511b79ae4977`.

That formal file was explicitly marked agent-written and unverified. It
contains useful proof material for tube construction, gluing, action cutoff,
strict positive-time filtering, finite simple-word search, and conditional
capacity correctness. Its stated theorem path uses stronger hypotheses than
the live implementation intends: pairwise nonzero `omega_0`, dual-vertex
general position, and finite-orbit regularity. Use it as recovery material to
audit and migrate proof arguments, not as a direct thesis source and not as the
final hypothesis list.

## Proof Recovery Audit Snapshot

The recovered proof distinguishes the closed polygonal search domain from the
strict tube/orbit object:

- `def:tube-data` uses `tau_r >= 0` for the closed search domain;
- the same definition says a point belongs to the represented tube exactly when
  all segment times `tau_r` are positive;
- `alg:tube-exhaustive-simple-word-search` records a fixed point only after
  reconstructing segment times and checking that they are all positive;
- `prop:tube-search-correctness-finite-orbit-regular` relies on that positive
  segment-time filter.

Current exact code matches the closed-domain side: `primitive_tube` adds
nonnegative-time halfspaces, `ExactPolygon::contains` uses closed membership,
and `solve_closed_tube` solves exact fixed points. The 2026-06-21 repair also
matches the strict-output side for nonsingular fixed points: before returning
`PositiveOrbit`, `exact_tube.rs` reconstructs exact segment times from the
fixed start point and cyclic word and requires every segment time to be
strictly positive. Positive total action alone is no longer enough for exact
positive-orbit output.

Remaining proof-recovery work is the hypothesis/correspondence map, not the
existence of simple minimizers and not this strict-time filter. The current
known audit targets are:

- replace the recovered proof's pairwise nonzero `omega_0` hypothesis by the
  accepted local condition on geometrically possible transitions, or state why
  the proof still needs the stronger condition;
- compare dual-vertex general position in the recovered proof with the current
  fixture/data-generation assumptions;
- compare finite-orbit regularity in the recovered proof with current exact
  rejection of `UnsupportedPositiveSingular`;
- reconcile positive transition signs with the current transition-matrix
  orientation;
- prove the correspondence between HK2017 simple minimizers and the FG words
  enumerated by `for_each_sigma_pruned_by_transition`;
- migrate the tube gluing and action-cutoff lemmas from recovered material into
  active proof text if theorem-strength thesis prose needs them.
