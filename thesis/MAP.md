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
- `black-box-datascience.tex`: data-science search-result scaffold.
- `rotated-regular-polygons.tex`: rotated regular polygon side-result
  scaffold.
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

## Legacy Source Material

- `legacy/README.md`: explains the status of legacy thesis files.
- `legacy/*`: old thesis prose and thesis-local notes. These files are source
  material only and are not input by `main.tex`.

## Removed Source

- `planned-toc.md` was converted into the active scaffold and removed. Its
  useful section descriptions now live as local comments in the scaffold files.
