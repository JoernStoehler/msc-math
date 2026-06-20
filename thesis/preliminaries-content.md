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

- Jörn clarified on 2026-06-20 that Clarke is a required thesis-facing
  proof/exposition component: the literature is too compressed for the thesis
  role this result plays.
- The 2026-06-16 source-archeology result says to use
  `thesis/legacy/clarkedual-action-principle.tex` and
  `thesis/legacy/simple-minimizer-existence.tex` as primary old writing, while
  treating older raw talk files as convention/pitfall archaeology only.
- Preserve the recovered convention decisions: free period `T`, uncentered dual
  problem, no `\int z = 0` constraint, and no untranslated HK2017 fixed-`[0,1]`
  or `2 I_K` normalization. In the thesis convention, a contact-normalized Reeb
  curve has `I_K = T = A`.
- Name the function space, action functional, minimizer/existence statement,
  and the conclusion used later for polytopes.
- Use it only for the part of the finite computation story it actually
  justifies: the bridge from primal minimum-action closed characteristics to
  the dual curve problem in which the simple-minimizer argument is carried out.
- Keep functional-analytic details only where needed for correctness and reader
  pacing. The Euler--Lagrange inclusion should cite Clarke rather than prove
  nonsmooth calculus from scratch.
- 2026-06-20 draft review: one math-risk subagent flagged the missing
  minimizer-is-critical bridge and the Euler--Lagrange equivalence wording;
  both were patched. Jörn review is still needed for the final free-period
  uncentered equivalence and normalization.
- The PDF review built from commit `69fdbd29` was abandoned after little review
  happened. The next review target should be a polished branch-head PDF for the
  combined Clarke/simple-Reeb surface, not the old baseline.
- Branch-head review should ask Jörn specifically whether the cited Clarke
  Euler--Lagrange equivalence is acceptable in the free-period, uncentered
  convention at the level of detail in the thesis proof.
- Once the active Clarke section has passed Jörn's review, delete
  `thesis/legacy/clarkedual-action-principle.tex`; it should not remain as a
  parallel source after its content and phrasing are absorbed into active
  thesis text.

## Out Of Scope For This Session

- The flow-graph algorithm based on CH2021 is left to another session.
- Our algorithms for solving the HK quadratic programs are left to another
  session. This section may prepare notation used by those algorithms, but
  should not explain pruning, KKT solving, exact/f64 behavior, or performance
  engineering.
