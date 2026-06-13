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

## Writing Inventory

- Present this as a retained thesis algorithm with correctness and performance
  claims. The main remaining uncertainty is how good the final results are, not
  whether the section is merely exploratory.
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

- Definition: define the algorithm in current thesis notation from the current
  mathematical source, not stale old thesis text.
- Definition ingredients to check: input polytope, face graph, tube, primitive
  tube, tube intersection, action restriction, closed-loop fixed points, and
  output orbit/capacity.
- Correctness: prove the algorithm computes the same target as the HK2019
  formulation under the stated assumptions, if the retained support strength
  licenses that claim.
- Correctness assumptions/proof steps to name if retained: exhaustive
  simple-word search, pruning claims used or not used, fixed-point solving, and
  comparison with the generalized Reeb orbit definition.
- Performance optimization: explain the concrete improvements that make the
  algorithm useful in thesis computations once the implementation is finished.
- Empirical tests: state comparison tests against HK2019 and targeted tests for
  the algorithm's own objects, such as primitive maps, empty intersections,
  action restriction, fixed points, small examples against HK2019, and retained
  HKO/regular-polygon cases.

## Source Pointers

- Algorithm/control: `crates/symplectic/src/algorithms/flow_graph/README.md`
- Implementation: `crates/symplectic/src/algorithms/flow_graph/`
- Experiments: `experiments/dev-flow-graph/README.md`
- Legacy import material: `research/tube-algorithm.md`,
  `research/tube-algorithm-raw-jorn-2026-05-04.md`
- Paper source: `papers/ch2021/`
