# Generalized Reeb Orbits Content Notes

Status: section-local content companion for
`thesis/generalized-reeb-orbits-polytopes.tex`. Not source truth.

Purpose: gather the writing inventory for explaining why finite polytope
computations compute the relevant symplectic quantity.

Overruled by: `formal/`, source papers, revalidated legacy thesis material,
active algorithm sections, and Jörn/Kai review.

Lifecycle: keep while the generalized Reeb orbit section is being assembled.
After the section is stable, delete this file or reduce it to a short
maintenance index.

Update rule: add or change a claim only with a source pointer or an explicit
`needs source` marker.

## Session Scope

- In scope: generalized Reeb orbits as the polytope object that connects
  EHZ/minimum action to the finite HK quadratic-program statement.
- In scope: literature-level limit or smoothing statements needed to explain
  why polytope orbits are the correct nonsmooth analogues of smooth Reeb
  orbits.
- Out of scope: the flow-graph algorithm based on CH2021. CH2021 may still be
  used here as a source for definitions, smoothing limits, and combinatorial
  Reeb-orbit statements if those claims are the right source truth.
- Out of scope: project-original algorithmic improvements on top of HK or
  CH2021, except for small notation choices needed to state the literature in
  thesis notation.

## Definition

- Define generalized Reeb orbits on polytopes in the notation used by the
  algorithms.
- Ingredients to include: piecewise-linear curve, facets or dual vertices
  touched, beta/dwell times, closing condition, action, and what counts as an
  orbit word.

## Limit Of Smooth Bodies

- Explain generalized Reeb orbits as limits of smooth convex bodies.
- Needs source: exact limit statement, including what converges, which topology
  is used, and what conclusion is needed later.
- Check the exact CH2021 statement and topology before final prose. Do not
  import the CH2021 flow-graph algorithm merely because CH2021 is useful for
  this limit/smoothing question.

## Simple Minimizers

- State and prove, or cite and supplement, the existence of simple
  minimum-action generalized Reeb orbits on polytopes.
- Current decision: prove this in the thesis if retained.
- Existing thesis text defines "simple" as a generalized Reeb orbit whose
  derivative is piecewise constant, each constant value is a pure facet Reeb
  vector, and each facet velocity occurs on a single interval, possibly empty.
- This option supports the finite `(sigma, tau, b)` representation.
- The main text must state the result and why it applies. Long proof details
  can move to an appendix if they interrupt the reading path.
