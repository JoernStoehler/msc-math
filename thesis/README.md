# Thesis

`main.tex` defines the active publication surface. Files under `legacy/` are
source material only and are not part of the thesis unless deliberately copied
into active text.

## Active structure

| Content | Active source | Writing companion |
| --- | --- | --- |
| Abstract | `00-abstract.tex` | `abstract-content.md` |
| Introduction | `01-introduction.tex` | `introduction-content.md` |
| Preliminaries | `02-preliminaries*.tex` | `preliminaries-content.md` |
| Generalized Reeb orbits | `03-generalized-reeb-orbits-polytopes.tex` and `03-generalized-reeb-orbits-*.tex` | `generalized-reeb-orbits-polytopes-content.md` |
| Haim--Kislev quadratic program | `04-haim-kislev-quadratic-program.tex` | `quadratic-program-algorithm-hk2019-content.md` |
| Flow graph/CH2021 | `05-flow-graph-algorithm-ch2021.tex` and `05-flow-graph-*.tex` | `flow-graph-algorithm-ch2021-content.md` |
| First-order perturbations | `06-first-order-perturbations.tex` | `first-order-perturbations-content.md` |
| HKO local maximum | `07-hko-local-maximum*.tex` | `hko-local-maximum-content.md` |
| Data-science search | `08-black-box-datascience*.tex` | `black-box-datascience-content.md` |
| Rotated regular polygons | `09-rotated-regular-polygons*.tex` | `rotated-regular-polygons-content.md` |
| Visualization | `10-visualization-3d.tex` | `visualization-3d-content.md` |
| Numerics | `11-numerics.tex` | `numerics-content.md` |
| Published code and data | `12-published-code-data.tex` | `published-code-data-content.md` |
| Use of AI | `13-use-of-ai.tex` and `ai-use-disclosure.tex` | matching `*-content.md` files |
| Conclusion | `14-conclusion.tex` | `conclusion-content.md` |
| Data-science appendix | `a-datascience-results.tex` | `appendix-datascience-results-content.md` |

Writing companions collect sources, caveats, decisions, and missing exposition.
They are not proof or evidence sources. Follow their pointers to `formal/`,
`experiments/`, `crates/`, papers, or accepted decisions.

In publication prose, use “exact” only when it distinguishes arithmetic or
representation from numerical approximation, or when it is part of a standard
term such as “exact form.” Mathematics is otherwise understood literally:
do not use “exact” as a synonym for proved, rigorous, complete, or fully
specified.

## Cross-cutting writing controls

- `central-claim-control.md`: claim placement, support, caveats, and review
  gates; navigation only.
- `theory-authoring-map.md`: reader questions and explanatory placement across
  the early theory.
- `DEVELOPMENT.md`: maintainer workflow.
- `preamble.tex`: thesis-local LaTeX setup and definitions.
- `bibliography.bib`: active bibliography.
- `figures/`: thesis-owned final assets and their producers.
- `working/`: thesis-owned candidate assets. Some are active TeX inputs; check
  the active TeX reference and writing companion rather than inferring use from
  the directory name.

## Build

```bash
cd thesis
latexmk
./check-build.sh
```

The build checks compilation and selected structural conditions. It does not
establish proof correctness, source adequacy, or Jörn/Kai acceptance.

## Legacy

`legacy/README.md` explains the retained historical source. Search `legacy/`
only when an active companion or concrete missing argument points there.
