# First-Order Perturbations Content Notes

Status: section-local content companion for
`thesis/06-first-order-perturbations.tex`. Not source truth.

Purpose: gather the writing inventory for the general first-order perturbation
machinery used in gradient-like search, concept formation, and later proof
work.

Overruled by: `research/`, `formal/`, `experiments/hko-local-maximum/`,
`crates/`, and Jörn/Kai review.

Lifecycle: keep while the section is being assembled. After the section is
stable, delete this file or reduce it to a short maintenance index.

Update rule: add or change a claim only with a source pointer or an explicit
`needs source` marker.

## Definition

- Define the first-order objects in the notation used by the rest of the
  thesis.
- Include row-coordinate perturbation `h`, active orbit/word set, branch
  action, beta derivative or subgradient, volume derivative, and derivative of
  `sys`.
- Write this as a general machinery chapter, not only as an HKO-driven tool
  chapter.
- Introduce the generic case first because much of the machinery is clean under
  an open dense generic condition.
- Then explain limits to non-generic polytopes, including HKO2024. These limits
  are annoying but important for the later proof-work and for how the concepts
  were found, even though they are not the SageMath verification route for HKO
  local maximality.
- Open decision: exact generic hypotheses to list, possibly positive dwell
  times, full-rank constraints, negative-definite reduced Hessian, unique
  active minimizer or finite active set, and fixed face combinatorics for
  volume.
- Open decision: which non-generic cases should be explained in the thesis
  because they illuminate the proof-work and HKO2024 limit behavior. Do not try
  to catalogue every possible degeneracy unless it serves that goal.

## Correctness

- Prove the generic case cleanly.
- Add non-generic cases afterward only as needed for HKO2024 or honest caveats.
- Likely theorem statements: differentiability of one branch, derivative
  formula for capacity/action, derivative formula for volume,
  derivative/subgradient statement for `sys`, and a separate caveat for ties or
  semidefinite cases.

## Empirical Tests

- State the numerical checks that the implementation behaves as expected on the
  examples used in the thesis.
- Candidate checks: finite-difference agreement, active-set stability,
  comparison to ascent behavior, and HKO-specific row/rank bookkeeping.
