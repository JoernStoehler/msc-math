# Flow-Graph Algorithm Thesis Content Notes

Status: section-local content companion for
`thesis/flow-graph-algorithm-ch2021.tex`. Not source truth.

Live algorithm/control source:
`crates/symplectic/src/algorithms/flow_graph/README.md`.

Use this file only to gather thesis-facing wording decisions, paragraph order,
and source pointers for the flow-graph section. Do not define the algorithm
here first. If this file and the live flow-graph README disagree, refresh this
file from the README and source truth.

## Current Thesis Use

- This file does not decide whether the section is retained.
- If retained, the section should use the CH2021/flow-graph/tube algorithm
  story only at the support strength recorded in the live algorithm README.
- The current implementation surface is f64 development evidence, not an exact
  certificate for `c_EHZ`.
- The thesis section must match the support strength recorded in the live
  algorithm README.
- Scoping decision from Jörn, 2026-06-15: leave the flow-graph algorithm to
  another session. Do not include this section in the current established-HK
  content milestone.
- CH2021 may still be relevant outside this section as source truth for
  smoothing/limit statements relating smooth convex bodies, polytopes, and
  combinatorial or generalized Reeb orbits. Route those uses through
  `thesis/generalized-reeb-orbits-polytopes-content.md`, not through this
  algorithm section.

## Writing Inventory

- Deferred. A later session must decide whether and how to present this as a
  retained thesis algorithm, and must match all correctness/performance claims
  to the live flow-graph README.
- First-use name: "Flow-Graph Algorithm Based On CH2021". This is the most
  correct and precise thesis-facing name.
- Short forms after first use: "flow graph algorithm"; "algorithm", "capacity
  algorithm", or "it" when clear in context.
- The separate tube picture may be useful in the definition because it
  describes the operated-on objects, but it does not by itself say that the
  algorithm intersects tubes or chooses which tubes to build.
- Open decision after the flow-graph/tube branch lands: which tube objects
  appear in the exposition, and the exact strength of final result claims.

## Section Notes

- Deferred with the section. When reopened, likely topics include: input
  polytope, face graph, tube, primitive tube, tube intersection, action
  restriction, closed-loop fixed points, output orbit/capacity, correctness
  assumptions, and empirical comparison with HK. These are not part of the
  current established-HK content milestone.

## Source Pointers

- Algorithm/control: `crates/symplectic/src/algorithms/flow_graph/README.md`
- Implementation: `crates/symplectic/src/algorithms/flow_graph/`
- Experiments: `experiments/dev-flow-graph/README.md`
- Legacy import material: `research/tube-algorithm.md`,
  `research/tube-algorithm-raw-jorn-2026-05-04.md`
- Paper source: `papers/ch2021/`
