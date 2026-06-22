# Flow-Graph Algorithm Proof-Recovery Notes

Status: section-local recovery note for
`thesis/flow-graph-algorithm-ch2021.tex`. Not source truth and not active
thesis prose.

Live algorithm/control source:
`crates/symplectic/src/algorithms/flow_graph/README.md`.

## Live Thesis Blocked State

This is the durable tracking place for the current FG thesis blockage. Future
agents should read this section before discussing, drafting, or reviewing
`thesis/flow-graph-algorithm-ch2021.tex`.

The reader-facing FG chapter is blocked upstream. The empty scaffold in
`thesis/flow-graph-algorithm-ch2021.tex` is not the active problem to solve by
writing prose. The active problem is to reconstruct a correct theorem/proof
route for the flow-graph capacity computation and check its correspondence with
the current exact Rust implementation.

Do not infer this blocked state from source code or tests. Source code and
tests show implementation behavior; they do not record the thesis-level reason
why reader-facing prose must wait.

Current active surface:

- recover and verify the mathematical FG theorem route;
- separate theorem hypotheses from implementation input and rejection behavior;
- compare the recovered proof route with `exact_search.rs`, `exact_tube.rs`,
  and the flow-graph README contract;
- record the resulting theorem/correspondence state before drafting thesis
  prose.

Current non-active surface:

- publication prose in `thesis/flow-graph-algorithm-ch2021.tex`;
- style review of that prose;
- PDF review for this section.

The chapter becomes active only after the theorem/proof route and Rust
correspondence are stable enough to state what capacity claim the thesis can
honestly make.

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

- The active thesis already states and proves the simple-minimizer result as
  `thm:generalized-reeb-simple-minimizer` in
  `thesis/generalized-reeb-orbits-polytopes.tex`; that theorem is sourced to
  HK2017 Theorem 1.5 (`simple_loop_theorem` in
  `papers/hk2017/EHZ-polytopes.tex`);
- `research/tube-algorithm.md` records Jörn's accepted clarification that
  nonzero `omega_0` is only needed on geometrically possible trajectory
  transitions; the algorithm does not require every facet pair to have
  nonzero `omega_0`;
- `research/tube-algorithm-raw-jorn-2026-05-04.md` records the raw tube model,
  including action as elapsed time along stored Reeb segments, tube gluing,
  action cutoff by a halfspace, and exhaustive simple-word search;
- exact closed-word output now reconstructs segment times and requires every
  segment time to be strictly positive before returning `PositiveOrbit`;
- exact search rejects nonempty facet-pair zero-`omega_0` candidates before
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
- Do not mix theorem hypotheses with implementation input representation. The
  mathematical theorem starts with \(K\subset\mathbb R^4\) a bounded convex
  polytope with \(0\in\operatorname{int}(K)\). Rational normalized facet data
  belongs to the exact Rust/input-realization layer unless a theorem statement
  specifically needs that representation.
- Before asking Jörn to review an FG theorem statement, check that the statement
  is written as mathematics, not implementation prose: no "the code accepts",
  "the flow graph regards", "current", "supported", or similar hidden-state
  language in theorem hypotheses; no circular reachability phrases where an
  algebraic or geometric condition can be stated directly; implementation
  representation, f64 behavior, and QP comparison must be separate paragraphs.

## Source Pointers

- Algorithm/control: `crates/symplectic/src/algorithms/flow_graph/README.md`
- Exact search: `crates/symplectic/src/algorithms/flow_graph/exact_search.rs`
- Exact closed words/tubes: `crates/symplectic/src/algorithms/flow_graph/exact_tube.rs`
- f64 diagnostics/fallback: `crates/symplectic/src/algorithms/flow_graph/f64_tube_search.rs`
- f64/rejection tests: `crates/symplectic/src/algorithms/flow_graph/tests_e2e_prediction.rs`
- Experiments: `experiments/dev-flow-graph/README.md`
- Active generalized-Reeb/simple-minimizer background:
  `thesis/generalized-reeb-orbits-polytopes.tex`
- Active CH2021 background:
  `thesis/generalized-reeb-orbits-polytopes.tex`,
  `subsec:generalized-reeb-orbits-polytopes-ch2021`
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

History audit on 2026-06-22 found the proof-evolution chain with:

```bash
git log --all --oneline -- \
  formal/tube-algorithm.tex \
  research/tube-algorithm.md \
  research/tube-algorithm-raw-jorn-2026-05-04.md \
  crates/symplectic/src/algorithms/tube/mod.rs \
  thesis/tube-algorithm.tex \
  thesis/tube-algorithm-notes.md \
  crates/symplectic/src/algorithms/flow_graph/README.md \
  thesis/flow-graph-algorithm-ch2021-content.md
```

Relevant history anchors:

| Commit | Why it matters |
| --- | --- |
| `e7b14830` | first agent-written thesis tube-algorithm section; includes symplectic-polytope, directed two-face, tube, rotation, pruning, and Type 2 discussion; unreviewed historical surface, not active proof |
| `17d174f1` | fixes to that old thesis section, including action formula, CH2021 citation targets, and gap markers; still agent-written and not active proof |
| `153ef067` | raw Jörn tube-algorithm notes now preserved as `research/tube-algorithm-raw-jorn-2026-05-04.md` |
| `25ba2efc` | first generic Rust tube search implementation predecessor |
| `bf2b6b0e` | first formal tube proof surface; already flagged isolated raw-tube-equivalence blocker |
| `b911ae74` | adds supported regular input and affine-bijection/gluing regular map contract |
| `3d9e080f` | first-theorem/action-window route with pairwise nonzero \(\omega_0\), nonsingular relevant fixed-point equations, and zero-time collapse |
| `15631732` | separates closed search domains from strict-time tube output and classifies singular fixed sets by positive time |
| `89d74c03` | adds positive-time linear dependence and small-word exclusions |
| `5d5bc1cb` | adds long-word determinant genericity witnesses |
| `60af5134` | tightens theorem-code correspondence in the predecessor tube implementation/evidence note |
| `25dd8b9a` | latest inspected formal route: pairwise nonzero \(\omega_0\), dual-vertex general position, finite-orbit regularity, unsupported positive singular boundary |
| `69b3a50a` | creates the live flow-graph README and thesis content-note surfaces |

Do not infer that the latest historical proof route is mathematically best.
The history shows how earlier agents repaired specific gaps; the next proof
work still has to choose and verify the route against the active thesis and
current `flow_graph` code.

Older February thesis draft warning:

- `git show 17d174f1:thesis/tube-algorithm.tex` contains rotation pruning,
  CH2021 Type 1/Type 2 language, and a proof sketch assuming no Type 2
  minimum-action orbit. The live implementation deliberately does not use
  CH2021 rotation pruning, and the current thesis packet must not import the
  Type 2 exclusion as if it had been reviewed.
- The February draft's "symplectic polytope" condition is stated using
  two-dimensional faces. The current Rust validators use nonempty facet-pair
  candidates, which is a different and generally stronger rejection surface.

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
- prove the correspondence between the simple minimizers from
  `thm:generalized-reeb-simple-minimizer` and the FG words enumerated by
  `for_each_sigma_pruned_by_transition`;
- migrate the tube gluing and action-cutoff lemmas from recovered material into
  active proof text if theorem-strength thesis prose needs them.

## Math Source Audit Before Theorem Drafting

Status: working audit, not active thesis prose and not a theorem statement.
Use this section before drafting or asking Jörn to review an FG theorem.

Do not present active theorem prose until the items below are either imported as
lemmas, deliberately excluded by hypotheses, or marked as unsupported
implementation behavior. Previous agents repeatedly broke the wording by
redefining tube objects, mixing theorem hypotheses with implementation inputs,
or replacing already proved facts by assumptions.

### Current recovery result

The main mathematics is not absent, but it is split across surfaces with
different proof status and different hypotheses.

| Ingredient | Source found | Status for thesis use |
| --- | --- | --- |
| Generalized Reeb orbits on polytopes, active words, base-point recovery, action \(A=T\) | `thesis/generalized-reeb-orbits-polytopes.tex` | active thesis text |
| Existence of a capacity-realizing simple orbit | `thm:generalized-reeb-simple-minimizer` | active thesis theorem; do not re-assume |
| CH2021 Type 1 flow-graph correspondence and smoothing-limit background | `thesis/generalized-reeb-orbits-polytopes.tex`, `subsec:generalized-reeb-orbits-polytopes-ch2021`; paper source `papers/ch2021/` | active thesis background; not by itself a proof that the Rust exact search computes capacity |
| Transition feasibility for \(F_i\to F_j\) | `formal/search-pruning-correctness.tex`, `lem:transition-feasibility` | useful but wrapped in `unverified`; proof-status conflict with file header needs cleanup before thesis reliance |
| Raw strict tube object and affine representation idea | `research/tube-algorithm-raw-jorn-2026-05-04.md` | raw Jörn note; source material, not polished theorem text |
| Local nonzero-\(\omega_0\) clarification | `research/tube-algorithm.md` | accepted source note; theorem proof still needs corresponding local-hypothesis rewrite |
| Primitive affine map, gluing, action restriction, closed fixed points | `git show 3d9e080f:formal/tube-algorithm.tex` and `git show 25dd8b9acb8aeaaa6aa3abd80fc6d95db00c4747:formal/tube-algorithm.tex` | recovered proof material; explicitly agent-written and unverified |
| Zero-time boundary treatment | `git show 3d9e080f:formal/tube-algorithm.tex`, `lem:tube-zero-time-collapse`; current `exact_tube.rs` strict-time filter | older proof has explicit collapse lemma; current code filters instead of returning boundary points |
| Short-word exclusion and finite-orbit regular route | `git show 25dd8b9acb8aeaaa6aa3abd80fc6d95db00c4747:formal/tube-algorithm.tex` | recovered unverified route using pairwise nonzero \(\omega_0\), dual-vertex general position, and long-word determinant regularity |
| Exact implementation realization | `exact_tube.rs`, `exact_search.rs`, `flow_graph/README.md` | code/test/control-surface fact; separate from mathematical theorem hypotheses |

### Proof routes that must not be blended

There are at least two recovered theorem routes. They solve different
problems and use different hypotheses.

1. First-theorem/action-window route from `3d9e080f`:
   - assumes pairwise nonzero \(\omega_0(a_i,a_j)\) for all distinct facets;
   - assumes nonsingular closed fixed-point equations for the words relevant
     in the requested action window;
   - handles zero-time boundary points by `lem:tube-zero-time-collapse`;
   - treats singular fixed-point equations as outside the first milestone.

2. Later dual-general-position route from `25dd8b9...`:
   - assumes pairwise nonzero \(\omega_0(a_i,a_j)\);
   - assumes dual-vertex general position;
   - proves length \(2\) words empty, length \(3\) search domains singular, and
     positive-time words force linear dependence;
   - uses dual-vertex general position to exclude positive-time fixed points
     for words of length at most \(4\);
   - uses finite-orbit regularity only for length-at-least-\(5\) words;
   - proves a conditional capacity corollary under those assumptions.

The current exact implementation contract is a third surface:

- it rejects zero-\(\omega_0\) on every nonempty directed facet-pair candidate,
  which can be stronger than rejecting only trajectory-feasible transitions;
- it does not state or check dual-vertex general position as a named input
  predicate;
- it reports positive-action singular fixed sets as unsupported instead of
  resolving them;
- it filters exact positive output by reconstructing strict segment times.

Therefore the next theorem cannot be obtained by copying either recovered
formal route unchanged. The shortest honest next proof task is a correspondence
audit: choose one theorem route, then either strengthen the implementation
contract to that route or rewrite the proof hypotheses to match the current
exact contract.

### Located math sources

- Active thesis base definitions:
  `thesis/generalized-reeb-orbits-polytopes.tex` defines
  \(K=\{x:\langle a_i,x\rangle\le1\}\), irredundant facets
  \(F_i\), contact-normalized directions \(R_i=2J_0a_i\), generalized Reeb
  orbits, simple words, action \(A=T\), and
  `thm:generalized-reeb-simple-minimizer`.
- Transition feasibility:
  `formal/search-pruning-correctness.tex`, `lem:transition-feasibility`, gives
  a mathematical definition and criterion for a transition \(F_i\to F_j\).
  This is not an implementation predicate.
- Raw tube source:
  `research/tube-algorithm-raw-jorn-2026-05-04.md` defines a tube as a set of
  generalized Reeb trajectories plus strict times, not as a tuple of
  breakpoints.
- Organized May source:
  `research/tube-algorithm.md` records accepted clarifications, including
  closed-tube combinatorics `(a_1,...,a_k,a_1,a_2)`, fixed points on the start
  two-face, action cutoff by a halfspace, and the local/non-global nature of
  the needed nonzero-\(\omega_0\) condition.
- Earlier formal proof, first-theorem version:
  `git show 3d9e080f:formal/tube-algorithm.tex` contains labels
  `def:tube-supported-input`, `def:tube-action-window-regular`,
  `def:tube-data`, `def:tube-primitive`,
  `lem:tube-primitive-affine-bijection`, `lem:tube-intersection`,
  `lem:tube-action-restriction`, `lem:tube-closed-fixed-point`,
  `lem:tube-zero-time-collapse`, `alg:tube-exhaustive-simple-word-search`,
  `prop:tube-search-correctness-supported`, and
  `cor:tube-capacity-conditional`.
- Later recovered formal proof:
  `git show 25dd8b9acb8aeaaa6aa3abd80fc6d95db00c4747:formal/tube-algorithm.tex`
  adds dual-vertex general position, short-word exclusions, positive-time
  linear dependence, determinant genericity, finite-orbit regularity, and the
  finite-orbit-regular capacity corollary.
- Implementation behavior:
  `crates/symplectic/src/algorithms/flow_graph/exact_tube.rs` and
  `exact_search.rs` implement exact closed-word/tube resolution, strict
  segment-time filtering, exact exhaustive search, zero-\(\omega_0\) rejection
  for nonempty facet-pair transitions, and unsupported positive singular
  outcomes.

### Terminology that must not be changed casually

- A tube is the mathematical object from the raw source, represented in the
  formal/code path by affine tube data: start/end polygons, affine map, affine
  action, sequence, and cutoff.
- Closed polygonal/tube search domains use \(\tau_r\ge0\). Returned orbits for
  a displayed word require \(\tau_r>0\) for every segment.
- A tuple of breakpoints and times is a trajectory candidate or fixed-point
  witness for a represented closed tube. It is not itself the tube object.
- Zero-time boundary fixed points are not new orbits for the displayed word;
  older formal material uses `lem:tube-zero-time-collapse`, and current exact
  code filters them by reconstructing strict segment times.
- Singular closed fixed-point sets are not silently absent. Earlier formal
  versions exclude them by regularity assumptions; current code reports
  unsupported positive singular outcomes.

### Missing or unstable theorem pieces

These are the pieces that must be resolved before active thesis theorem prose.

1. Tube definition and representation:
   migrate a clean definition of the mathematical tube and represented affine
   tube data. Do not redefine a tube as a breakpoint tuple.

2. Primitive tube lemma:
   recover the statement that a supported primitive triple gives affine tube
   data and identify the exact hypothesis needed for the denominator. This is
   `lem:tube-primitive-affine-bijection` in recovered material.

3. Local nonzero-\(\omega_0\) hypothesis:
   decide whether the theorem uses the simple stronger all-pairs condition,
   the intersecting-facet-pair condition, or the intended local transition
   condition. If using the local condition, rewrite every use of pairwise
   nonzero \(\omega_0\) in the recovered proof. The current code rejects
   nonempty facet-pair zero-\(\omega_0\), which is stronger than the accepted
   "geometrically possible transition" wording if nonempty-but-infeasible
   facet pairs occur.

4. Transition convention and signs:
   reconcile the transition definition/sign convention from
   `formal/search-pruning-correctness.tex` with the FG primitive triple
   convention `(p,i,n)` and the code's `omega_signs[(previous,current)] >= 0`,
   `omega_signs[(current,next)] >= 0` checks.

5. Tube gluing/intersection:
   migrate `lem:tube-intersection`: compatible represented tubes glue by
   composition, pullback/intersection of start polygons, and action addition.
   This is where empty subtubes imply containing tubes are empty.

6. Action cutoff:
   migrate `lem:tube-action-restriction`: reducing the action cutoff is an
   affine halfspace restriction on the start polygon.

7. Closed fixed points:
   migrate `lem:tube-closed-fixed-point`: fixed points of the closed affine map
   correspond to closed candidates for the displayed word. Keep the distinction
   between closed-domain fixed points and positive-time orbits.

8. Zero-time behavior:
   recover or replace `lem:tube-zero-time-collapse` from the earlier formal
   version, and compare it to current strict segment-time filtering in
   `exact_tube.rs`.

9. Short-word and regularity strategy:
   choose one route:
   - first-theorem/action-window regularity: exclude singular fixed-point
     equations in the searched window; or
   - later recovered route: use dual-vertex general position, positive-time
     linear dependence, short-word exclusions, and finite-orbit regularity for
     length-at-least-five words.
   Do not mix these routes silently.

10. Capacity conclusion:
    after the simple-minimizer theorem, prove that a capacity-realizing simple
    orbit is represented by the chosen tube search under the selected
    hypotheses. Do not list simple-minimizer existence as a hypothesis. The
    active theorem already gives a simple minimizer; the FG proof obligation is
    to show that the chosen tube search represents every capacity-realizing
    simple orbit that satisfies the selected regularity/nonzero conditions.

11. Exact Rust correspondence:
    after the mathematical theorem is stable, separately state what exact Rust
    realizes: rational facet data, precomputed matching facet-intersection and
    \(\omega_0\)-sign matrices, strict-time filtering, zero-\(\omega_0\)
    rejection, and unsupported positive singular outcomes. This is not part of
    the theorem hypotheses unless explicitly made a representation theorem.

12. f64 and QP/HK separation:
    f64 diagnostics and QP/HK scalar comparisons are outside theorem proof.
    They belong in implementation/evaluation prose after the theorem claim is
    stable.

### Current theorem-drafting block

Do not draft active theorem prose yet. The next useful durable step is to
recover the lemma chain above into a formal/proof-facing file or a cleaner
section-local proof skeleton. Only after that should the active FG theorem be
written for Jörn review.

Minimal next proof packet:

1. Create a proof-facing FG/tube section that imports the active
   generalized-Reeb notation and `thm:generalized-reeb-simple-minimizer`.
2. Copy or restate only the recovered lemmas needed for primitive tubes,
   gluing, action restriction, closed fixed points, and zero-time treatment,
   preserving `unverified` status until reviewed.
3. Choose exactly one regularity route for the theorem statement.
4. Add a separate implementation-correspondence paragraph for the exact Rust
   path, including stricter or different code predicates.
5. Only then draft reader-facing thesis prose.
