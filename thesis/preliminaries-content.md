# Preliminaries Content Notes

Status: section-local content companion for `thesis/02-preliminaries.tex`. Not
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
- 2026-06-20 correction: these are not optional implementation details.
  Active prose now includes
  `subsec:preliminaries-polytope-finite-geometry`, covering normalized
  \(H\)-representations, polar dual vertices, boundedness via positive
  spanning, irredundancy via extremality of dual rows, exact vertex enumeration
  from four active facets, vertex--facet incidence, face-incidence extraction,
  and Hausdorff/fixed-facet local charts. This material is non-novel but needed
  in multiple thesis sections and should remain thesis-facing, even if some
  details later move to an appendix.

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
- HK's `singular_capacity` citation is `AAO2014` in
  `papers/hk2017/EHZ-polytopes.bbl`. The relevant source trail is
  `AAO2014` Propositions 2.5 and 2.7 plus Lemmas 5.1 and 5.2: fixed-period
  dual minimum, generalized closed-characteristic action equality, weak
  critical-point correspondence, and minimizers weak critical. The active
  thesis should not merely cite this correspondence; it should prove the
  convention translation and Reeb correspondence in thesis notation.
- Preserve the recovered convention decisions: free period `T`, uncentered dual
  problem, no `\int z = 0` constraint, and no untranslated HK2017 fixed-`[0,1]`
  or `2 I_K` normalization. In the thesis convention, a contact-normalized Reeb
  curve has `I_K = T = A`.
- Name the function space, action functional, minimizer/existence statement,
  and the conclusion used later for polytopes.
- Use it only for the part of the finite computation story it actually
  justifies: the reduction from primal minimum-action closed characteristics to
  the dual curve problem in which the simple-minimizer argument is carried out.
- Keep only the general nonsmooth-analysis infrastructure outside the main
  proof: existence of a dual minimizer and the validity of Clarke's nonsmooth
  Lagrange multiplier rule. The active text should derive the free-period,
  uncentered Euler--Lagrange relation, scaling/translation, Reeb
  correspondence, and \(I_K=T=A\) normalization in thesis notation.
- 2026-06-20 draft review: one math-risk subagent flagged the missing
  minimizer-is-critical step and the Euler--Lagrange equivalence wording.
  The active draft now avoids the unsupported "critical point iff" formulation
  and cites the AAO/HK source trail explicitly. The remaining review boundary
  is whether the only external analytic input has been narrowed enough:
  existence of a dual minimizer plus Clarke's nonsmooth multiplier rule.
- The PDF review built from commit `69fdbd29` was abandoned after little review
  happened. The next review target should be a polished branch-head PDF for the
  combined Clarke/simple-Reeb surface, not the old baseline.
- Review boundary for branch-head PDF: whether the remaining cited analytic
  input is acceptable at this level of detail, and whether the free-period,
  uncentered derivation of weak criticality and the \(I_K=T=A\) normalization
  are correct.
- Legacy/PDF comparison on 2026-06-20 checked
  `thesis/legacy/basic-definitions.tex`, the old `/tmp/pdf-source-a872` thesis
  packet, and `/tmp/thesis_stoehler_20260216.pdf` against active preliminaries.
  Active text now absorbs the useful low-cost preliminaries omissions:
  standard \(J_0,\omega_0,\lambda_0\) identities, the standard capacity
  axioms/background, and the symplectic-product capacity formula for normalized
  symplectic capacities.
- Stale-branch mining on 2026-06-22 recovered the provenance detail behind
  that PDF comparison: `/tmp/thesis_stoehler_20260216.pdf` matched rebuild
  commit `a8724979698e1e2b98d5a0145184c65b08f809ed`, whose relevant source
  files were `thesis/clarkedual-action-principle.tex`,
  `thesis/simple-minimizer-existence.tex`, `thesis/basic-definitions.tex`,
  `thesis/proofs.tex`, and `thesis/experiments/experiments.tex`. The current
  descendants are in `thesis/legacy/`. This is source-routing information for
  future cleanup, not thesis evidence.
- Deliberately not ported from old preliminaries in this checkpoint:
  characteristic-line-field definitions and their reparametrization proof,
  Stokes/symplectic-area motivation for action, full smooth/polytope orbit
  existence theorem statements, and the Clarke symmetry-class uniqueness
  remark. These are lower-priority polish or background candidates for the
  preliminaries review surface; they should be reconsidered before deleting the
  corresponding legacy source material.
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
