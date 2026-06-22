# Flow-Graph Thesis Support Ledger

Status: section-local support ledger for
`thesis/flow-graph-algorithm-ch2021.tex`.

This file is not thesis prose, not a proof file, and not a Rust design log. Its
job is to keep the thesis-writing boundary clear until the flow-graph section is
ready to draft.

The active `.tex` file currently contains only the scaffold. Do not draft
reader-facing correctness prose from this note alone. The next thesis-facing
step is to review the proof-development surface and then translate only the
surviving theorem/caveat boundary into thesis prose.

## Source Truth

Use these sources directly when drafting or reviewing the section.

- Active thesis background:
  `thesis/generalized-reeb-orbits-polytopes.tex`.
  Checked labels:
  `thm:generalized-reeb-simple-minimizer` and
  `subsec:generalized-reeb-orbits-polytopes-ch2021`.
- Flow-graph proof development:
  `formal/flow-graph-real-algorithm.tex`.
  Checked labels:
  `def:fg-nondegenerate-facet-presentation`,
  `lem:fg-local-transition-regularity-positive-sign`,
  `def:fg-closed-tube-search-data`,
  `lem:fg-primitive-tubes-affine`,
  `lem:fg-tube-gluing`,
  `lem:fg-closed-tube-fixed-points`,
  `lem:fg-nonpositive-fixed-set-no-strict-orbit`, and
  `thm:fg-real-capacity-correctness`.
- Flow-graph implementation/control surface:
  `crates/symplectic/src/algorithms/flow_graph/README.md`.
- Exact Rust path:
  `crates/symplectic/src/algorithms/flow_graph/exact_search.rs` and
  `crates/symplectic/src/algorithms/flow_graph/exact_tube.rs`.
- f64 Rust path:
  `crates/symplectic/src/algorithms/flow_graph/f64_tube_search.rs`.
- Experiment/evidence routing:
  `experiments/dev-flow-graph/README.md` and `experiments/MAP.md`.
- Raw/recovered tube sources:
  `research/tube-algorithm-raw-jorn-2026-05-04.md`,
  `research/tube-algorithm.md`, and historical formal source
  `git show 25dd8b9acb8aeaaa6aa3abd80fc6d95db00c4747:formal/tube-algorithm.tex`.

`formal/flow-graph-real-algorithm.tex` and the historical formal files are
agent-written proof-development surfaces. They are useful for reconstruction,
but they are not accepted thesis proof until reviewed.

## Thesis Role

The FG section should explain a second serious capacity algorithm developed in
the project. Its thesis value is:

- completeness: the thesis records the implemented flow-graph/tube approach;
- verification: FG gives an independent algorithmic comparison against the
  HK/QP scalar capacity route on eligible examples;
- motivation: the section explains why QP remains the practical workhorse
  despite the FG implementation.

Do not frame FG as a universal certified solver, as support for HKO or
Lagrangian-product degeneracies, or as a performance replacement for QP.

## Draft-Planning Claim Boundary

The following claims are the current draft-planning boundary, subject to the
review gates below.

- The exact FG path is a rational implementation for selected four-dimensional
  rational polytope inputs.
- The exact path enumerates transition-pruned simple cyclic facet words,
  constructs affine tube data, solves closed-word fixed-point equations using
  rational arithmetic, filters positive outputs by strict segment times, and
  reports retained FG words up to the exact action threshold.
- Exact rejection/caveat behavior includes nonempty facet-pair
  zero-\(\omega_0\) candidates and positive-action singular fixed sets.
- HKO and Lagrangian-product zero-\(\omega_0\) examples are outside this
  implementation scope.
- f64 output is approximate output, not an exact certificate. A theorem-level
  f64 claim would need sound-predicate or numerical-error analysis that is not
  currently present.
- HK/QP is scalar comparison for eligible examples. It is not a retained-word
  oracle for FG words.

The exact theorem wording is not settled. Do not write that FG computes
\(c_{\mathrm{EHZ}}\) for the Rust runtime boundary until the formal/Rust
correspondence below is reviewed.

## Mathematical Support Ledger

| Thesis ingredient | Source/status | Thesis consequence |
| --- | --- | --- |
| A capacity-realizing generalized Reeb orbit can be chosen simple. | Active theorem `thm:generalized-reeb-simple-minimizer`; source notes cite HK2017 Theorem 1.5. | Do not assume simple minimizers as an FG hypothesis; cite the active thesis theorem. |
| CH2021 flow-graph/tube background. | Active background at `subsec:generalized-reeb-orbits-polytopes-ch2021`, plus `papers/ch2021/`. | This motivates the construction. It is not by itself a proof that the Rust exact search computes capacity. |
| Adjacent facet-pair pruning. | `alg:fg-real-exhaustive-search`; Rust `exact_search.rs` uses `build_transition_matrix_from_facet_intersections_and_omega`. | This is a necessary pruning condition. Do not describe the transition matrix as an exact physical-transition oracle. |
| Local transition signs from nonzero \(\omega_0\). | Active proof-development labels `def:fg-nondegenerate-facet-presentation` and `lem:fg-local-transition-regularity-positive-sign`. | The current formal/Rust route uses the stronger nonempty facet-pair nonzero-\(\omega_0\) condition. Do not silently replace it by the weaker local trajectory condition. |
| Closed tube search data and strict-time output. | `def:fg-closed-tube-search-data`; Rust `exact_tube.rs` has `NonStrictNoOrbit` and strict segment-time filtering. | Reader-facing prose must distinguish closed search domains \(\tau_r\ge0\) from returned orbits for a displayed word, which require \(\tau_r>0\). |
| Primitive affine tubes, gluing, and fixed points. | `lem:fg-primitive-tubes-affine`, `lem:fg-tube-gluing`, `lem:fg-closed-tube-fixed-points`; unapproved proof-development text. | These are the main proof pieces to review before theorem-strength thesis prose. |
| Singular fixed-point equations. | Rust classifies exact singular fixed sets; `lem:fg-nonpositive-fixed-set-no-strict-orbit` covers the no-orbit side for nonpositive-action fixed sets; the finite-orbit-regular theorem route still excludes relevant singular fixed maps from the capacity theorem. | Do not identify the finite-orbit-regular theorem route with the full Rust singular-classifier runtime boundary until the singular-classifier correspondence is reviewed. |
| Exact implementation boundary. | `exact_search.rs`, `exact_tube.rs`, and flow-graph README. | State implementation behavior separately from mathematical theorem hypotheses. |
| f64 behavior. | `f64_tube_search.rs` and flow-graph README. | State as approximate/numerical implementation behavior unless a later numerical-analysis task proves sound predicates. |
| HK/QP comparison. | Exact accepted examples and verification code. | Use only for scalar capacity comparison on eligible examples; do not use as a word-level oracle. |

## Implementation Facts That Matter For Thesis Wording

These are code facts, not theorem hypotheses.

- `exact_search.rs::search_closed_orbits_exact` validates nonnegative action
  threshold, rejects nonempty facet-pair zero-\(\omega_0\) candidates, enumerates
  transition-pruned simple words, and treats
  `EmptyTube`, `ZeroActionNoOrbit`, and `NonStrictNoOrbit` as no-orbit outcomes.
  If exhaustive search finds no positive orbit, it returns typed non-success
  rather than panicking.
- `exact_tube.rs` reconstructs segment times before returning `PositiveOrbit`.
  Positive total action alone is not enough.
- `exact_tube.rs::solve_singular_fixed_tube` and
  `exact_tube.rs::singular_fixed_polygon_result` classify singular fixed sets
  exactly on the uncut tube or on the tube remaining after an action cutoff.
  Positive-action singular fixed sets in that searched domain become
  `UnsupportedPositiveSingular`; nonpositive-action singular fixed sets become
  `ZeroActionNoOrbit`.
- `f64_tube_search.rs::capacity_f64` resolves f64 closed-word errors with exact
  closed-word arithmetic, but direct f64 positive words remain f64 outputs. If
  no positive orbit remains, it returns typed non-success rather than panicking.

The code-design comparison for singular fixed maps belongs in the flow-graph
README, not in this thesis ledger.

## Drafting Gates

Before drafting `thesis/flow-graph-algorithm-ch2021.tex`, complete these gates.

1. Review `formal/flow-graph-real-algorithm.tex` for false statements, missing
   hypotheses, sign/orientation mistakes, and theorem/code mismatch.
2. Decide the theorem route for reader-facing prose:
   finite-orbit regularity, a determinant-generic corollary, or a narrower
   implementation-status statement.
3. Write a theorem/input paragraph in mathematical language:
   start from \(K\subset\mathbb R^4\) a bounded convex polytope with
   \(0\in\operatorname{int}K\). Keep rational data, matrices, validators, and
   f64 behavior in implementation paragraphs.
4. State exact and f64 behavior separately.
5. State HK/QP comparison as scalar comparison only.
6. Check that no reader-facing sentence depends on hidden repo state through
   words such as "current", "supported", "route", or "style".

## What Not To Import

- Do not import CH2021 rotation pruning as implemented behavior.
- Do not claim support for HKO, Lagrangian products, or Type 2/Type 3
  degeneracies.
- Do not present f64 output as an exact certificate.
- Do not use HK/QP as a retained-word oracle.
- Do not move code-design option comparisons, commit archaeology, or session
  repair notes into this thesis-side file.
