# Preliminaries Content Notes

Status: section-local content companion for `thesis/preliminaries.tex`. Not
source truth.

Purpose: gather the definitions, source pointers, and proof-placement notes
needed before writing the preliminaries.

Overruled by: `FACTSHEET.md`, `formal/`, source papers, active proof notes,
legacy thesis source where revalidated, and Jörn/Kai review.

Lifecycle: keep while the preliminaries are being assembled. After the section
is stable, delete this file or reduce it to a short maintenance index.

Update rule: add or change a claim only with a source pointer or an explicit
`needs source` marker.

## Purpose

- Define the mathematical objects needed by later chapters.
- Present the simple, robust domain model rather than implementation shortcuts.
- Collapse notation and conventions where the literature is inconsistent.
- Keep this section educational and preparatory. It should explain standard
  Euclidean and symplectic objects, plus the natural thesis notation needed to
  use the standard literature. It should not present project-original
  algorithmic improvements or new theorem-strength insights.
- Distinguish three kinds of material when drafting:
  - standard background and standard literature results;
  - thesis explanations, notation choices, and harmless organizing machinery
    used to make that literature readable here;
  - project-original extensions, which belong in later result or method
    sections unless a small setup convention is needed for readability.

## Polytopes

- Polytopes in `R^4`.
- Duality between convex hulls of finite sets and bounded intersections of
  half-spaces.
- Polytopes containing zero and the dual polytope with dual vertices.
- Validity checks for a proposed finite dual-row set: `{a_k}` must be exactly
  the extremal points of `conv {a_k}`, and `{x : <a_k,x> <= 1}` is bounded
  exactly when `0 in int conv {a_k}`.
- Algorithmic construction of the primal vertex set from valid dual vertices.
- Incidence of a primal vertex with a facet is the defining equality
  `<a_i,x> = 1`; precomputing incidence is useful later but is not a separate
  mathematical assumption.
- Support and gauge functions.
- Face lattice and naming conventions.
- Convex bodies, smooth convex bodies, and Hausdorff distance.
- The topological space of polytopes with a fixed number of facets.
- The dual-vertex parametrization of a fixed-facet-count polytope family, with
  the caveat that facet permutations identify presentations. This is natural
  setup machinery for local variation and search sections, not a thesis result
  to advertise as a new contribution.
- HKO2024 as a ten-facet polytope example.
- Volume of a polytope.

## Smooth Symplectic Geometry

- Standard symplectic structure and notation on `R^4`.
- Reeb vector fields for smooth convex bodies.
- Reeb trajectories and Reeb orbits.
- Action of a curve.
- Minimum action.
- Existence of a Reeb orbit and of a minimum action, cited without proof unless
  a retained claim needs more detail.
- Symplectic capacities.
- Minimum action as a symplectic capacity, cited without proof unless a
  retained claim needs more detail.
- Viterbo's conjecture for smooth convex bodies.
- Continuity argument via scaling and Hausdorff distance.

## Clarke Dual Action Principle

- State the principle at the level needed for the thesis.
- Do not decide proof depth by default. Current open choice: cite/state only,
  give a proof sketch, or import a polished proof from legacy material after
  revalidation.
- Name the function space, action functional, minimizer/existence statement,
  and the conclusion used later for polytopes.
- Use it only for the part of the finite computation story it actually
  justifies. Open decision: state exactly which later step it justifies:
  existence of a minimum-action orbit, reduction to a closed
  curve/action-minimization problem, the simple-minimizer route, or the
  HK finite optimization theorem.
- Keep functional-analytic details only where needed for correctness and reader
  pacing.

## Out Of Scope For This Session

- The flow-graph algorithm based on CH2021 is left to another session.
- Our algorithms for solving the HK quadratic programs are left to another
  session. This section may prepare notation used by those algorithms, but
  should not explain pruning, KKT solving, exact/f64 behavior, or performance
  engineering.
