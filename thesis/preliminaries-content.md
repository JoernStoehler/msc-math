# Preliminaries Content Notes

Status: section-local authoring companion for `thesis/02-preliminaries.tex`.
It is not mathematical source truth.

Purpose: record only the common reader state that the preliminaries should
establish. Use `theory-authoring-map.md` for the cross-chapter dependency and
ownership reasoning.

Overruled by: `FACTSHEET.md`, accepted Jörn/Kai decisions, active theorem
statements, mathematical owners, source papers, and integrated reader review.

## Intended Reader Change

The introduction motivates the remaining structural questions around
Viterbo's conjecture and the HKO counterexample. The preliminaries should give
the reader the common language needed to enter the generalized-orbit and later
result stories without first learning any one algorithm.

On leaving the preliminaries, the reader should understand:

- the thesis conventions for coordinates, \(J_0\), \(\omega_0\), \(\lambda_0\),
  action, smooth closed characteristics, and contact-normalized Reeb orbits;
- why the chosen normalization gives \(A=T\);
- convex bodies containing the origin, support and gauge functions, normalized
  facet rows and polar vertices, boundedness, irredundancy, and labelled versus
  unlabelled presentations;
- the EHZ capacity, the four-dimensional systolic ratio, Viterbo's threshold,
  volume, and a compact account of the capacity axioms that makes the thesis's
  symplectic comparisons meaningful;
- the geometric meaning of a Lagrangian \(q/p\)-product and why it is not a
  symplectic product;
- Clarke's general dual action principle for smooth and nonsmooth convex
  bodies, in the detailed free-period and uncentered convention already
  developed for the thesis.

Use brief motivation where it helps future master's students form these
objects. Do not expand standard background merely to be encyclopedic. Give
extra detail to project-wide conventions and literature translations because
later proofs depend on their exact signs, factors, and quantifiers.

## Retained Source Routes

- Coordinate/action conventions and smooth Reeb normalization:
  `02-preliminaries-convex-symplectic-notation.tex`, with
  `thesis/legacy/basic-definitions.tex` only as fallible explanatory source.
- Polytope input language: the conceptual parts of
  `02-preliminaries-polytope-representations-finite-geometry.tex`, standard
  convex duality, and the accepted boundedness material in
  `formal/random-polytope-boundedness.tex` where needed.
- Capacity and systolic ratio: `02-preliminaries-ehz-capacity.tex`, with final
  citation placement checked against authoritative capacity sources and the
  nonsmooth AAO/HK route.
- Clarke duality: retain `02-preliminaries-clarke-dual-action-principle.tex`.
  `PROJECT_COMPLETION.md` owns its settled proof boundary and review status;
  AAO2014 and HK2017 own the external analytic inputs and correspondence.
- Lagrangian products: the active definition, HKO2024 for the central example,
  and the later QP/polygon owners for specialized enumeration or symmetry.

## Material With A Later Owner

Move rather than delete material whose reader purpose is narrow:

- The symplectic shoelace identity moves to the generalized-orbit proof spine
  near its first substantive use.
- Generalized facet directions, words, dwell times, closure, base-point
  recovery, and simple minimizers remain with generalized orbits.
- Exact vertex enumeration, incidence extraction/rules, boundedness kernels,
  local fixed-facet charts, volume computations, and label order move to the
  algorithms or results that consume them.
- Systolic continuity and product-volume constancy move to the limiting or
  polygon arguments that use them unless another common use emerges.
- The unused symplectic-product capacity formula should be cut unless a
  retained reader-facing argument creates a use.

The three recovered legacy candidates are dispositioned in
`theory-authoring-map.md`: retain compact orbit existence, use the longer
characteristic/reparametrization explanation only if reader testing demands
it, and omit Clarke symmetry-class uniqueness absent a consumer.

Do not repair the current incidence-only edge/two-face rule in isolation. Its
correct statement depends on whether it becomes a restricted pipeline
contract, a general face-lattice statement, or no publication claim at all.

## Writing And Review Boundary

This companion and `theory-authoring-map.md` prepare an author; they do not yet
authorize mechanical movement of TeX. Before drafting, test the proposed
reader-state boundary against the introduction and the opening of generalized
orbits. Repair Clarke's current forward reference by introducing the general
nonsmooth characteristic/Hamiltonian-inclusion notion before the dual theorem;
the later generalized-Reeb material then specializes it to polytope facets.
After moving material, review the full transition and rebuild the PDF;
the current local coherence of each source file is not evidence that the new
cross-file reading path works.

After the active Clarke passage has passed Jörn's integrated review and all
useful source decisions have been absorbed, delete
`legacy/clarkedual-action-principle.tex`. Until that gate, it remains fallible
source material rather than a parallel draft to preserve.
