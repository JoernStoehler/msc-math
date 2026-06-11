# Quadratic Program Algorithm Content Notes

Status: section-local content companion for
`thesis/quadratic-program-algorithm-hk2019.tex`. Not source truth.

Purpose: gather the writing inventory for the HK2019 finite optimization
algorithm section.

Overruled by: source papers, `formal/`, `crates/symplectic/`, revalidated
legacy thesis material, experiment artifacts, and Jörn/Kai review.

Lifecycle: keep while the section is being assembled. After the section is
stable, delete this file or reduce it to a short maintenance index.

Update rule: add or change a claim only with a source pointer or an explicit
`needs source` marker.

## Purpose And Naming

- Present the first finite algorithmic formulation for the minimizer search.
- First use can call this "Quadratic Program Algorithm Based On HK2019"; after
  first use, use the shorter algorithmic name.
- The bibliography currently also has an `HK2017` entry for the same title; the
  journal-version GAFA citation is 2019 and should be preferred in
  reader-facing naming unless a writer chooses otherwise.

## Definition

- Define the finite optimization problem, variables, constraints, objective,
  and reconstruction of the orbit.
- Include sigma/orbit word, beta constraints, quadratic objective including
  normalization/factor convention, admissibility conditions, and orbit
  reconstruction formula.
- State the input contract explicitly: the dual rows have already been checked
  to be extremal and bounded in the sense of the polytope preliminaries.
- The capacity algorithm may use the computed primal vertex set and
  vertex-facet incidence for feasibility and pruning, but mathematical
  correctness is the HK2019 finite-search statement plus the preceding validity
  checks.

## Correctness

- Prove that the finite problem computes the intended minimum action/capacity
  in the polytope setting.
- Likely named claims: finite candidate reduction, equivalence between beta
  problem and generalized orbit action, existence of an optimum, and recovery
  of capacity/sys.

## Performance Optimization

- Explain only optimizations that matter for feasibility or for understanding
  later experiments.
- Candidate topics: partial-word enumeration, pruning/admissibility checks, KKT
  solve, repeated/tied minimizer handling, and accumulator/certainty status.
- Separate correctness from implementation speed. Incidence tables, cached
  symplectic products, ordered candidate generation, tracing, profiling, and
  benchmarking explain why experiments can be run, not why the theorem is true.

## Empirical Tests

- State tests supporting the implementation and put detailed outputs/knobs
  outside the main reading path.
- Candidate families: known polytopes, HK2019-vs-CH2021 comparison,
  exact/f64 spot checks, orbit recovery, and regression tests for past bugs.
- Testing philosophy: whenever a believed statement is precise enough to
  formalize and cheap enough to exercise, write it as an automated test.
