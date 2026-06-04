# Thesis Map

Status: navigation cache for the active thesis scaffold.

## Active Thesis

- `main.tex`: structural entry point. It inputs the active scaffold files and
  no legacy thesis prose.
- `abstract.tex`: abstract scaffold.
- `introduction.tex`: introduction scaffold.
- `preliminaries.tex`: preliminaries scaffold.
- `generalized-reeb-orbits-polytopes.tex`: generalized Reeb orbit scaffold.
- `quadratic-program-algorithm-hk2019.tex`: HK2019 quadratic-program algorithm
  scaffold.
- `flow-graph-algorithm-ch2021.tex`: CH2021 flow-graph algorithm scaffold.
- `first-order-perturbations.tex`: first-order perturbation scaffold.
- `hko-local-maximum.tex`: HKO local-maximum main-result scaffold.
- `hko-local-maximum-content.md`: section-local content-gathering notes for
  the HKO result packet. Not source truth; use its source pointers before
  relying on claims.
- `black-box-datascience.tex`: data-science search-result scaffold.
- `rotated-regular-polygons.tex`: rotated regular polygon side-result
  scaffold.
- `rotated-regular-polygons-content.md`: section-local content-gathering notes
  for the rotated regular polygon side result, especially the pentagon
  executable proof packet. Not source truth; use its source pointers before
  relying on claims.
- `visualization-3d.tex`: visualization side-result scaffold.
- `numerics.tex`: high-level numerics scaffold.
- `published-code-data.tex`: published code and data scaffold.
- `use-of-ai.tex`: AI-use scaffold.
- `conclusion.tex`: conclusion scaffold.
- `appendix-datascience-results.tex`: data-science appendix scaffold.
- `appendix-numerics-proofs.tex`: numerics appendix scaffold.
- `appendix-sagemath-computations.tex`: SageMath computation appendix
  scaffold.
- `preamble.tex`: additional LaTeX preamble definitions.
- `bibliography.bib`: thesis bibliography.

## Thesis Development Notes

- `DEVELOPMENT.md`: maintainer-facing notes about the scaffold conversion and
  process discussion that produced it.
- Section-local `*-content.md` files, when present, are writeup-gathering
  companions for nearby `.tex` files. They are not source truth.

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

- `planned-toc.md` was converted into the active scaffold and removed. Its
  useful section descriptions now live as local comments in the scaffold files.
