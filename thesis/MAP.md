# Thesis Map

Status: navigation cache for the active thesis surface.

## Active Thesis

- `main.tex`: structural entry point. It inputs the active thesis files and
  no legacy thesis prose.
- `abstract.tex`: abstract thesis surface.
- `abstract-content.md`: section-local abstract content notes.
- `introduction.tex`: introduction thesis surface.
- `introduction-content.md`: section-local introduction content notes.
- `preliminaries.tex`: preliminaries thesis surface.
- `preliminaries-content.md`: section-local preliminaries content notes.
- `generalized-reeb-orbits-polytopes.tex`: generalized Reeb orbit thesis
  surface.
- `generalized-reeb-orbits-polytopes-content.md`: section-local generalized
  Reeb orbit content notes.
- `quadratic-program-algorithm-hk2019.tex`: HK2019 quadratic-program algorithm
  thesis surface.
- `quadratic-program-algorithm-hk2019-content.md`: section-local HK2019
  algorithm content notes.
- `flow-graph-algorithm-ch2021.tex`: CH2021 flow-graph algorithm thesis
  surface.
- `flow-graph-algorithm-ch2021-content.md`: section-local content companion
  for the flow-graph algorithm section. Not source truth; use
  `crates/symplectic/src/algorithms/flow_graph/README.md` before relying on
  claims.
- `first-order-perturbations.tex`: first-order perturbation thesis surface.
- `first-order-perturbations-content.md`: section-local first-order
  perturbation content notes.
- `hko-local-maximum.tex`: HKO local-maximum main-result thesis surface.
- `hko-local-maximum-content.md`: section-local content-gathering notes for
  the HKO result packet. Not source truth; use its source pointers before
  relying on claims.
- `black-box-datascience.tex`: data-science search-result thesis surface.
- `black-box-datascience-content.md`: section-local data-science search-result
  content notes.
- `rotated-regular-polygons.tex`: rotated regular polygon side-result
  thesis surface.
- `rotated-regular-polygons-content.md`: section-local content-gathering notes
  for the rotated regular polygon side result, especially the pentagon
  executable proof packet. Not source truth; use its source pointers before
  relying on claims.
- `visualization-3d.tex`: visualization side-result thesis surface.
- `visualization-3d-content.md`: section-local visualization content notes.
- `numerics.tex`: high-level numerics thesis surface.
- `numerics-content.md`: section-local numerics content notes.
- `published-code-data.tex`: published code and data thesis surface.
- `published-code-data-content.md`: section-local publication/reproducibility
  content notes.
- `use-of-ai.tex`: AI-use thesis surface.
- `use-of-ai-content.md`: section-local AI-use content notes.
- `conclusion.tex`: conclusion thesis surface.
- `conclusion-content.md`: section-local conclusion content notes.
- `appendix-datascience-results.tex`: data-science appendix thesis surface.
- `appendix-datascience-results-content.md`: section-local data-science appendix
  content notes.
- `appendix-numerics-proofs.tex`: numerics appendix thesis surface.
- `appendix-numerics-proofs-content.md`: section-local numerics appendix
  content notes.
- `appendix-sagemath-computations.tex`: SageMath computation appendix
  thesis surface.
- `appendix-sagemath-computations-content.md`: section-local SageMath appendix
  content notes.
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
  `quadratic-program-algorithm-hk2019.tex` and related algorithm TODOs.
- `legacy/proofs.tex`, `legacy/basic-definitions.tex`,
  `legacy/clarkedual-action-principle.tex`,
  `legacy/simple-minimizer-existence.tex`,
  `legacy/general-case-algorithm-proof.tex`, and
  `legacy/pruned-general-case-algorithm.tex`: likely source material for
  `preliminaries.tex`, `generalized-reeb-orbits-polytopes.tex`,
  `quadratic-program-algorithm-hk2019.tex`, and
  `appendix-numerics-proofs.tex`.
- `legacy/appendix-numerical.tex` and `legacy/numerical-story.md`: likely
  source material for `numerics.tex` and `appendix-numerics-proofs.tex`.
- `legacy/experiments.tex`: likely source material for
  `black-box-datascience.tex`, `rotated-regular-polygons.tex`, and
  `visualization-3d.tex`.
- `legacy/sys-first-order-regular-case.tex`: likely source material for
  `first-order-perturbations.tex`.
- `legacy/migration-findings.md`: legacy-era thesis/code mismatch inventory.
  Revalidate relevant rows before relying on affected old algorithm, numerics,
  or tube prose.

## Removed Source

- `planned-toc.md` was converted into the active thesis surface and removed. Its
  useful section descriptions now live in section-local `*-content.md`
  companions.
