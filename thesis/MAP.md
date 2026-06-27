# Thesis Map

Status: navigation cache for the active thesis surface.

## Active Thesis

- `main.tex`: structural entry point. It inputs the active thesis files and
  no legacy thesis prose.
- `00-abstract.tex`: abstract thesis surface.
- `abstract-content.md`: section-local abstract content notes.
- `01-introduction.tex`: introduction thesis surface.
- `introduction-content.md`: section-local introduction content notes.
- `02-preliminaries.tex`: contains `\section{Preliminaries}`, the section
  opening, and inputs the
  `02-preliminaries-*` semantic subfiles.
- `preliminaries-content.md`: section-local preliminaries content notes.
- `03-generalized-reeb-orbits-polytopes.tex`: contains the generalized Reeb
  orbit `\section`, the section opening, and inputs the
  `03-generalized-reeb-orbits-*` semantic subfiles.
- `generalized-reeb-orbits-polytopes-content.md`: section-local generalized
  Reeb orbit content notes.
- `04-haim-kislev-quadratic-program.tex`: Haim--Kislev quadratic-program
  thesis surface.
- `quadratic-program-algorithm-hk2019-content.md`: section-local
  quadratic-program content notes.
- `05-flow-graph-algorithm-ch2021.tex`: CH2021 flow-graph algorithm thesis
  surface.
- `flow-graph-algorithm-ch2021-content.md`: section-local content companion
  for the flow-graph algorithm section. Not source truth; use
  `crates/symplectic/src/algorithms/flow_graph/README.md` before relying on
  claims.
- `06-first-order-perturbations.tex`: first-order perturbation thesis surface.
- `first-order-perturbations-content.md`: section-local first-order
  perturbation content notes.
- `07-hko-local-maximum.tex`: contains the HKO `\section`, opening, theorem
  statement, and inputs the `07-hko-local-maximum-*` semantic subfiles.
- `hko-local-maximum-content.md`: section-local content-gathering notes for
  the HKO result packet. Not source truth; use its source pointers before
  relying on claims.
- `08-black-box-datascience.tex`: data-science search-result thesis surface.
- `black-box-datascience-content.md`: section-local data-science search-result
  content notes.
- `09-rotated-regular-polygons.tex`: contains the rotated regular polygon
  `\section`, family overview, and inputs the
  `09-rotated-regular-polygons-*` semantic subfiles.
- `rotated-regular-polygons-content.md`: section-local content-gathering notes
  for the rotated regular polygon side result, especially the pentagon
  executable proof packet. Not source truth; use its source pointers before
  relying on claims.
- `10-visualization-3d.tex`: visualization side-result thesis surface.
- `visualization-3d-content.md`: section-local visualization content notes.
- `11-numerics.tex`: high-level numerics thesis surface.
- `numerics-content.md`: section-local numerics content notes.
- `12-published-code-data.tex`: published code and data thesis surface.
- `published-code-data-content.md`: section-local publication/reproducibility
  content notes.
- `13-use-of-ai.tex`: AI-use thesis surface.
- `use-of-ai-content.md`: section-local AI-use content notes.
- `14-conclusion.tex`: conclusion thesis surface.
- `conclusion-content.md`: section-local conclusion content notes.
- `a-datascience-results.tex`: data-science appendix thesis surface.
- `appendix-datascience-results-content.md`: section-local data-science appendix
  content notes.
- `b-numerics-proofs.tex`: numerics appendix thesis surface.
- `appendix-numerics-proofs-content.md`: section-local numerics appendix
  content notes.
- `c-sagemath-computations.tex`: SageMath computation appendix thesis surface.
- `appendix-sagemath-computations-content.md`: section-local SageMath appendix
  content notes.
- `central-claim-control.md`: thesis-wide content companion for central claims,
  support sources, caveats, paragraph placement, and review gates. Not source
  truth; use the named source files before relying on claims.
- `preamble.tex`: additional LaTeX preamble definitions.
- `bibliography.bib`: thesis bibliography.

## Thesis Development Notes

- `DEVELOPMENT.md`: maintainer-facing notes about the thesis surface
  convention, scaffold conversion history, and process discussion that produced
  it.
- Section-local `*-content.md` files are writeup-gathering companions for
  nearby `.tex` files. They are not source truth.

## Legacy Source Material

- `legacy/README.md`: explains the status of legacy thesis files.
- `legacy/*`: old thesis prose and thesis-local notes. These files are source
  material only and are not input by `main.tex`.

## Legacy Source Hints

- `legacy/algorithms.tex`, `legacy/general-case-algorithm.tex`,
  `legacy/pruned-general-case-algorithm.tex`, and
  `legacy/lagrangian-product-algorithm.tex`: likely source material for
  `04-haim-kislev-quadratic-program.tex` and related algorithm TODOs.
- `legacy/proofs.tex`, `legacy/basic-definitions.tex`,
  `legacy/clarkedual-action-principle.tex`,
  `legacy/simple-minimizer-existence.tex`,
  `legacy/general-case-algorithm-proof.tex`, and
  `legacy/pruned-general-case-algorithm.tex`: likely source material for
  `02-preliminaries.tex`, `03-generalized-reeb-orbits-polytopes.tex`,
  `04-haim-kislev-quadratic-program.tex`, and `b-numerics-proofs.tex`.
- `legacy/appendix-numerical.tex` and `legacy/numerical-story.md`: likely
  source material for `11-numerics.tex` and `b-numerics-proofs.tex`.
- `legacy/experiments.tex`: likely source material for
  `08-black-box-datascience.tex`, `09-rotated-regular-polygons.tex`, and
  `10-visualization-3d.tex`.
- `legacy/sys-first-order-regular-case.tex`: likely source material for
  `06-first-order-perturbations.tex`.
- `legacy/migration-findings.md`: legacy-era thesis/code mismatch inventory.
  Revalidate relevant rows before relying on affected old algorithm, numerics,
  or tube prose.

## Removed Source

- `planned-toc.md` was converted into the active thesis surface and removed. Its
  useful section descriptions now live in section-local `*-content.md`
  companions.
