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

## Source Pointers

- Algorithm/control: `crates/symplectic/src/algorithms/flow_graph/README.md`
- Implementation: `crates/symplectic/src/algorithms/flow_graph/`
- Experiments: `experiments/flow-graph/README.md`
- Legacy import material: `research/tube-algorithm.md`,
  `research/tube-algorithm-raw-jorn-2026-05-04.md`
- Paper source: `papers/ch2021/`
