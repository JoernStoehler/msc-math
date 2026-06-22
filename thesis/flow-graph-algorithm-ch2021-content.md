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

- The active thesis already states and proves the simple-minimizer result as
  `thm:generalized-reeb-simple-minimizer` in
  `thesis/generalized-reeb-orbits-polytopes.tex`; that theorem is sourced to
  HK2017 Theorem 1.5 (`simple_loop_theorem` in
  `papers/hk2017/EHZ-polytopes.tex`);
- `research/tube-algorithm.md` records Jörn's accepted clarification that
  nonzero `omega_0` is only needed on geometrically possible trajectory
  transitions; the algorithm does not require every facet pair to have
  nonzero `omega_0`;
- `formal/flow-graph-real-algorithm.tex` now uses the same nonempty
  facet-pair nonzero-\(\omega_0\) condition as the Rust exact and f64
  validators.  This condition implies the weaker physical-transition nonzero
  condition needed in the proof.  It is stronger than the minimal local
  condition recorded in `research/tube-algorithm.md`.
- `research/tube-algorithm-raw-jorn-2026-05-04.md` records the raw tube model,
  including action as elapsed time along stored Reeb segments, tube gluing,
  action cutoff by a halfspace, and exhaustive simple-word search;
- exact closed-word output now reconstructs segment times and requires every
  segment time to be strictly positive before returning `PositiveOrbit`;
- exact search rejects nonempty facet-pair zero-`omega_0` candidates before
  enumeration, including HKO and Lagrangian-product fixtures covered by tests;
- current f64 search output and `capacity_f64` are not exact certificates;
  direct f64 positive words remain f64 outputs, while f64 error words receive
  exact closed-word status only after exact resolution. A certified-f64 claim
  would need explicit sound predicate or error-bound analysis;
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

Remaining proof-recovery work is the implementation-correspondence map, not the
existence of simple minimizers, not this strict-time filter, and not the
nonempty facet-pair nonzero-\(\omega_0\) condition. The current known audit
targets are:

- keep the theorem/code distinction explicit for transition pruning:
  physical transition feasibility is stronger than nonempty facet-pair plus
  nonnegative `omega_0` unless a ridge/two-face sufficiency hypothesis applies;
  Rust uses the coarser condition as pruning, so false positives are allowed
  but false negatives would not be;
- separate proof-convenience hypotheses from runtime rejection boundaries:
  recovered proofs use dual-vertex general position and finite-orbit
  regularity, while the exact implementation directly detects the downstream
  exact singular fixed-point cases and reports positive-action singular fixed
  sets as unsupported;
- keep the determinant-generic statement relative to the primitive-denominator
  domain unless a later proof establishes density inside the space of valid
  irredundant facet presentations;
- keep the positive transition sign convention tied to the code convention:
  with \(\omega_0(u,v)=\langle J_0u,v\rangle\) and \(R_i=2J_0a_i\),
  `omega_signs[(p,i)] > 0` is the sign of \(\langle R_p,a_i\rangle\);
  the Rust `>=0` pruning check becomes strict on exact accepted inputs because
  nonempty zero-\(\omega_0\) facet-pairs are rejected before enumeration;
- review the active correspondence proof from
  `thm:generalized-reeb-simple-minimizer` to the support words represented by
  the idealized FG search;
- add an action-cutoff lemma only if theorem-strength thesis prose needs to
  discuss the Rust cutoff optimization, rather than only final threshold
  filtering.

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
| Local nonzero-\(\omega_0\) clarification | `research/tube-algorithm.md`; `formal/flow-graph-real-algorithm.tex`, `def:fg-nondegenerate-facet-presentation` and `lem:fg-local-transition-regularity-positive-sign` | accepted source note says the minimal condition is local; active unverified formal theorem and Rust validators use the stronger nonempty facet-pair condition |
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

The active idealized formal theorem in `formal/flow-graph-real-algorithm.tex`
is a third surface:

- it uses the nonempty facet-pair nonzero-\(\omega_0\) condition, matching the
  current Rust rejection boundary;
- it currently uses a global linear-independence condition and finite-orbit
  regularity as theorem hypotheses;
- it proves the idealized exact real-number search computes
  \(c_{\mathrm{EHZ}}\) under those hypotheses.

The current exact implementation contract is a fourth surface:

- it rejects zero-\(\omega_0\) on every nonempty directed facet-pair candidate,
  which can be stronger than rejecting only trajectory-feasible transitions;
- it does not need to name global dual-vertex general position at the API
  boundary if the downstream exact nonzero-denominator and nonsingular
  fixed-point cases are checked directly;
- it handles the finite-orbit-regularity boundary operationally: nonsingular
  fixed points are solved exactly, and positive-action singular fixed sets are
  reported as unsupported instead of silently used as capacity values;
- it is not a literal implementation of the active idealized algorithm's
  singular branch: Rust also classifies singular fixed sets and accepts
  exact zero-action singular cases as no-orbit outcomes;
- it filters exact positive output by reconstructing strict segment times.

Therefore the next thesis-facing step is not to choose the local
nonzero-\(\omega_0\) theorem route from scratch.  It is to review the active
idealized formal theorem against the current exact implementation contract and
then state the implementation theorem/caveat boundary honestly.

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

### Current theorem-piece status

These are the pieces that must be reviewed before active thesis theorem prose.
Several are now present in `formal/flow-graph-real-algorithm.tex`, but that
file is agent-written proof-development text and remains unapproved by Jörn.

1. Tube definition and representation:
   present in `def:fg-closed-tube-search-data`; review that it preserves the
   raw-source distinction between the mathematical tube and affine tube data.

2. Primitive tube lemma:
   present in `def:fg-primitive-tube` and `lem:fg-primitive-tubes-affine`;
   review the denominator/sign hypotheses against `exact_tube.rs`.

3. Local nonzero-\(\omega_0\) strength:
   the active idealized formal theorem and current code use the nonempty
   facet-pair condition.  This is stronger than the minimal local
   physical-transition condition from `research/tube-algorithm.md`.  Do not
   weaken the thesis theorem or validators unless a later task proves and tests
   the physical-transition validator.

4. Transition convention and signs:
   checked against `exact_tube.rs::primitive_tube` and
   `facet_adjacency.rs`: the signs agree with
   `lem:fg-local-transition-regularity-positive-sign` under the repo
   convention \(\omega_0(u,v)=\langle J_0u,v\rangle\).  Keep the separate
   caveat that the transition matrix is a necessary-condition pruning
   superset, not a physical-transition oracle.

5. Tube gluing/intersection:
   present in `lem:fg-tube-gluing`; review the compatibility, pullback,
   intersection, and action-addition statements against current code.

6. Action cutoff:
   not separately formalized as a lemma in the active formal theorem. Current
   algorithm/theorem wording uses threshold filtering after recording
   trajectories, while Rust also has an action-cutoff optimization. Add a
   separate action-cutoff lemma only if thesis prose needs to discuss that
   optimization.

7. Closed fixed points:
   present in `lem:fg-closed-tube-fixed-points`; review that it keeps the
   distinction between closed-domain fixed points and positive-time orbits.

8. Zero-time behavior:
   handled in the active route by strict segment-time filtering rather than by
   recovering `lem:tube-zero-time-collapse`. Review whether the theorem needs
   the collapse lemma; current exact code filters non-strict fixed points.

9. Short-word and regularity strategy:
   choose one route:
   - first-theorem/action-window regularity: exclude singular fixed-point
     equations in the searched window; or
   - later recovered route: use dual-vertex general position, positive-time
     linear dependence, short-word exclusions, and finite-orbit regularity for
     length-at-least-five words.
   Do not mix these routes silently.  The implementation-facing version should
   prefer direct exact denominator/fixed-point checks and typed unsupported
   outcomes when those are cheaper and clearer than exposing a global
   general-position condition.

10. Capacity conclusion:
    present in `thm:fg-real-capacity-correctness`; review the bridge from
    `thm:generalized-reeb-simple-minimizer` to the active-word support word.
    Do not list simple-minimizer existence as a hypothesis.

11. Exact Rust correspondence:
    after the mathematical theorem is stable, separately state what exact Rust
    realizes: rational facet data, precomputed matching facet-intersection and
    \(\omega_0\)-sign matrices, strict-time filtering, zero-\(\omega_0\)
    rejection, and unsupported positive singular outcomes. This is not part of
    the theorem hypotheses unless explicitly made a representation theorem.

12. f64 and QP/HK separation:
    f64 predicates can support a theorem-level implementation claim only when
    they are stated as sound predicates, for example a ternary
    true/false/indeterminate predicate where true and false imply the exact
    predicate result and indeterminate carries no decision.  The current f64
    search and QP/HK scalar comparisons are outside the idealized theorem
    proof.  They belong in implementation/evaluation prose after the theorem
    claim is stable.

### Current theorem-drafting block

Do not draft reader-facing theorem prose yet. The next useful durable step is
to review `formal/flow-graph-real-algorithm.tex` as a proof-development
surface, decide which theorem route survives Jörn review, and then translate
only that route into thesis prose.

### Exact branch proof-coverage checkpoint

This checkpoint is the repo-specific detector for the next Rust-versus-math
decision. It is based on `exact_search.rs::search_closed_orbits_exact`,
`exact_tube.rs::classify_closed_tube`, and
`formal/flow-graph-real-algorithm.tex`.

| Rust branch | Search meaning | Current proof coverage | Consequence |
| --- | --- | --- | --- |
| `build_tube` returns `Empty` | word contributes no retained orbit | covered by the active tube-domain route if `lem:fg-primitive-tubes-affine` and `lem:fg-tube-gluing` survive review | review math/code correspondence; no Rust change indicated |
| nonsingular fixed point outside `start_polygon` | word contributes no orbit | covered by `lem:fg-closed-tube-fixed-points` if the polygon representation is correct | review polygon/halfspace correspondence; no Rust change indicated |
| nonsingular fixed point with nonpositive action | `ZeroActionNoOrbit` | covered by capacity/action positivity conventions once `lem:fg-action-normalization` survives review | review action normalization; no Rust change indicated |
| nonsingular fixed point with positive action but some reconstructed segment time nonpositive | `NonStrictNoOrbit` | covered by the strict-time definition in `def:fg-closed-tube-search-data`; code test `exact_segment_time_filter_rejects_zero_time_boundary_point` covers the repaired branch | no Rust change indicated unless review rejects strict-time filter route |
| nonsingular fixed point with positive action and all reconstructed segment times positive | retained `PositiveOrbit` | covered by `lem:fg-closed-tube-fixed-points`, strict-time definition, and `lem:fg-action-normalization` if the primitive/gluing lemmas survive review | this is the intended exact positive-output path |
| singular fixed equation with inconsistent affine constraints | `EmptyTube` | covered by the affine fixed-point equation statement in `lem:fg-closed-tube-fixed-points` if that lemma survives review | can remain part of the fixed-point lemma rather than a separate theorem branch |
| singular fixed set whose fixed-polygon vertex actions are all nonpositive | `ZeroActionNoOrbit` | not covered by the current finite-orbit-regular theorem route, because that route excludes relevant long-word singular fixed equations | main fork: prove this refined singular classifier, make theorem-mode Rust reject it, or keep a two-layer claim |
| singular fixed set with any positive-action vertex | `UnsupportedPositiveSingular` | explicit typed non-success, not an accepted capacity output | document as unsupported; no proof of capacity output needed |
| no `PositiveOrbit` found after exhaustive search | panic at final `expect` | theorem route predicts this cannot happen on supported inputs because a simple capacity minimizer is represented | if a supported-input claim does not prove represented positive output, change Rust to typed unsupported instead of panic |

The singular classifier was already present in the initial flow-graph
implementation commit `69b3a50a`, and the same commit's README listed
positive-action singular fixed sets as explicit rejection cases. The commit-era
diagnostic tests and `experiments/dev-flow-graph/unresolved-diagnostic` also
use `resolve_closed_word_exact` to classify f64 closed-word failures,
separating exact empty tubes, exact zero-action no-orbits, exact positive
orbits, and exact unsupported positive singular outcomes. Session-log
archaeology found the pre-code design constraint that singular fixed-point
equations were unsupported when they "cannot be reduced or validated", not
categorically unsupported.

The actual decision surface is not "delete or keep". The current options are:

1. Keep the singular classifier for diagnostics and f64 exact-resolution
   behavior, and add a theorem-facing exact wrapper that rejects singular fixed
   maps.
2. Prove the singular fixed-polygon `ZeroActionNoOrbit` branch and include that
   branch in the implementation theorem.
3. Change the exact resolver itself to reject all singular fixed maps, after
   replacing the diagnostic/f64 behavior that currently depends on the
   classifier.

Option 1 is the lowest-risk next implementation boundary because it preserves
existing diagnostic behavior while giving the theorem a clean finite-orbit
regular boundary. Option 2 is the proof-improvement path. Option 3 is a
behavior change, not cleanup.

Minimal next proof packet:

1. Review `formal/flow-graph-real-algorithm.tex` for false statements, missing
   hypotheses, and sign/orientation mistakes.
2. Decide whether the thesis theorem uses finite-orbit regularity, a
   determinant-generic denominator-domain corollary, or an implementation
   boundary based on exact singular rejection.
3. Add a separate implementation-correspondence paragraph for the exact Rust
   path, including stricter or different code predicates.
4. Only then draft reader-facing thesis prose.
