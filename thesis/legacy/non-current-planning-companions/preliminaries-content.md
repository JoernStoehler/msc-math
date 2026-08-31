# Preliminaries Content Notes

Status: section-local authoring companion for `thesis/02-preliminaries.tex`.
It is not mathematical source truth.

Purpose: record only the common reader state that the preliminaries should
establish. Use `theory-authoring-map.md` for the cross-chapter dependency and
placement reasoning.

Overruled by: `docs/project-facts.md`, accepted Jörn/Kai decisions, active theorem
statements, mathematical sources, source papers, and integrated reader review.

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
- General convex Hamiltonian language and generalized characteristics:
  `02-preliminaries-convex-hamiltonian-language.tex`, with the Clarke passage
  explaining the dual correspondence rather than defining the primal
  object.
- Polytope input language: `02-preliminaries-polytope-input-language.tex`,
  standard convex duality, and the accepted boundedness material in
  `formal/random-polytope-boundedness.tex` where needed.
- Capacity and systolic ratio: `02-preliminaries-ehz-capacity.tex`, with final
  citation placement checked against authoritative capacity sources and the
  nonsmooth AAO/HK route.
- Clarke duality: retain `02-preliminaries-clarke-dual-action-principle.tex`.
  `docs/project-status.md` records its settled proof boundary and review status;
  AAO2014 and HK2017 supply the external analytic inputs and correspondence. The
  active proof uses the conjugate Hamiltonian notation \(H_K^*\) to keep the
  Fenchel and reconstruction steps visible without repeatedly expanding
  \(h_K^2/4\).
- Lagrangian products: `02-preliminaries-lagrangian-products.tex`, HKO2024 for
  the central example, and the later QP/polygon sections for specialized
  enumeration or symmetry.
- Explanatory figures: `figures/foundations/generate.py` is the thesis-native
  producer for `characteristic-normalization.pdf` and `facet-polarity.pdf`.
  The first is a dimension-reduced tangent-space schematic; the second is an
  exact planar polarity model. They explain conventions only and are not proof
  evidence. Run `uv run --script thesis/figures/foundations/generate.py` from
  the repository root to regenerate all foundation figures.

## Material With A Later Home

Move rather than delete material whose reader purpose is narrow:

- The symplectic shoelace identity now lives with finite generalized-orbit word
  data near its first substantive use.
- Generalized facet directions, words, dwell times, closure, base-point
  recovery, and simple minimizers remain with generalized orbits.
- Exact vertex enumeration, incidence extraction/rules, boundedness kernels,
  local fixed-facet charts, volume computations, and label order move to the
  code or result sections that consume them.
- Systolic continuity and product-volume constancy move to the limiting or
  polygon arguments that use them unless another common use emerges.
- The unused symplectic-product capacity formula has been cut. Recover it from
  Git only if a later reader-facing argument creates a use.

The three recovered legacy candidates are dispositioned in
`theory-authoring-map.md`: retain compact orbit existence, use the longer
characteristic/reparametrization explanation only if reader testing demands
it, and omit Clarke symmetry-class uniqueness absent a consumer.

Do not repair the current incidence-only edge/two-face rule in isolation. Its
correct statement depends on whether it becomes a restricted pipeline
contract, a general face-lattice statement, or no publication claim at all.

## Writing And Review Boundary

The active TeX now introduces the general nonsmooth
characteristic/Hamiltonian-inclusion notion before Clarke and specializes it to
polytope facets in the generalized-Reeb material. The mixed finite-geometry
source has been replaced by semantic convex-Hamiltonian, polytope-input, and
Lagrangian-product units; consumer-specific enumeration, incidence, chart, and
volume material is no longer part of the preliminary reading burden. The
Clarke proof now separates dual feasibility, weak Euler--Lagrange criticality,
and reconstruction; the converse obtains
\(\nu=I_K(z)/T=c^2\) directly from homogeneity. The downstream
simple-minimizer proof uses that identity rather than repeating the older
reconstruction algebra. The AAO2014 attainment citation is connected to the
free-period convention by the explicit centering, \(J_0\)-rotation, time
change, action normalization, and objective factor; AAO2014 Proposition 2.7 is
cited at the first nonsmooth least-action claim. A compact gate table blocks
the false inference that arbitrary dual-feasible curves already reconstruct.
This integrated candidate remains agent-reviewed and is not yet Jörn-reviewed.

After the active Clarke passage has passed Jörn's integrated review and all
useful source decisions have been absorbed, delete
`legacy/clarkedual-action-principle.tex`. Until that gate, it remains fallible
source material rather than a parallel draft to preserve.
