# Sys-Landscape Method Ledger

## Purpose

- Cache a normalized index of methods that appear in the repo for the hostile-landscape result surface.
- Point from repo-level method names to frozen taxonomy IDs and concrete repo evidence.
- Make it faster to answer "did we do this?" without pretending the ledger is itself authoritative.

## Authority

- This file is a cache, not a source of truth.
- The repo contents win on disagreement:
  - code
  - committed artifacts
  - analysis scripts
  - adjacent notes when artifacts alone are ambiguous
- Do not make thesis claims or other high-risk decisions from this ledger without checking the cited repo evidence.

## Status Fields

- `repo_state`
  - `attempted`
  - `skipped`
  - `inapplicable`
  - `undecided`
- `thesis_use`
  - `main-evidence`
  - `supporting-only`
  - `spike-only`
  - `redo-before-thesis`
  - `undecided`

## Columns

| Ledger ID | Method | Concrete variant | Taxonomy refs | Search surface / data | Repo evidence | repo_state | thesis_use | Notes |
|-----------|--------|------------------|---------------|------------------------|---------------|------------|------------|-------|

Interpretation rules:

- `Ledger ID`: stable local handle for discussion and future audit links.
- `Method`: human-readable family or experiment line.
- `Concrete variant`: the actual repo attempt or tightly grouped attempt family.
- `Taxonomy refs`: stable IDs from `taxonomy-*.md`; multiple refs are allowed.
- `Taxonomy refs`: stable IDs from frozen external taxonomy files; leave blank or use `—` when a repo method has not yet been mapped to an external taxonomy.
- `Search surface / data`: random regime, endpoint regime, structured family, HKO-local packet, trace/event logs, or similar.
- `Repo evidence`: paths that a later check should inspect first.
- `repo_state`: current cached repo view only.
- `thesis_use`: current cached judgment only.

## Current Attempted-Method Cache

| Ledger ID | Method | Concrete variant | Taxonomy refs | Search surface / data | Repo evidence | repo_state | thesis_use | Notes |
|-----------|--------|------------------|---------------|------------------------|---------------|------------|------------|-------|
| `M001` | random search baseline | random generic polytope sweep | `DFO-BASE-RANDOM` | random generic polytopes | `experiments/sys-landscape/random-sample/`, `research/sys-landscape.md` | `attempted` | `main-evidence` | Negative baseline for new `sys > 1` discovery so far. |
| `M002` | random search baseline | random Lagrangian-product sweep | `DFO-BASE-RANDOM` | random structured product family | `experiments/sys-landscape/random-product-sample/`, `research/sys-landscape.md` | `attempted` | `main-evidence` | Structured-family random baseline, not a uniform model of all convex bodies. |
| `M003` | random-search calibration | rejection / acceptance calibration | `DFO-BASE-RANDOM` | random-sampling baseline support | `experiments/sys-landscape/rejection-calibration/` | `attempted` | `supporting-only` | Calibration/support packet, not a standalone hostile-landscape claim. |
| `M004` | structured family search | rotated regular-product sweep | `DFO-BASE-RANDOM` | explicit low-dimensional family | `experiments/sys-landscape/rotated-regular-products/`, `research/sys-landscape.md` | `attempted` | `main-evidence` | Shows the known pentagon-pentagon family is special among tested regular families. |
| `M005` | local optimization | fixed-`F` gradient ascent from random starts (general) | `NUMOPT-GRADIENT`, `NUMOPT-LINESEARCH`, `NUMOPT-INIT-MULTISTART` | endpoint search over generic fixed-`F` polytopes | `experiments/sys-landscape/gradient-ascent-general/`, `research/sys-landscape.md` | `attempted` | `main-evidence` | Local optimization improves samples but does not yield a new `sys > 1` example. |
| `M006` | local optimization | fixed-`F` gradient ascent from random starts (products) | `NUMOPT-GRADIENT`, `NUMOPT-LINESEARCH`, `NUMOPT-INIT-MULTISTART` | endpoint search over product family at fixed `F` | `experiments/sys-landscape/gradient-ascent-products/`, `research/sys-landscape.md` | `attempted` | `main-evidence` | Product-side analogue of `M005`. |
| `M007` | continuation / local search | variable-`F` continuation | `NUMOPT-INIT-CONTINUATION`, `CONT-PARAM-PREDCORR`, `CONT-HOMOTOPY-SOLUTION` | continuation from fixed-`F` endpoints into `F+1` states | `experiments/sys-landscape/variable-f-ascent/`, `research/sys-landscape.md` | `attempted` | `main-evidence` | Improves some local maxima but still stays below `sys = 1` in current runs. |
| `M008` | HKO-local search | perturbation neighborhood around HKO2024 |  | HKO-local packet, not generic global search | `experiments/hko-local-maximum/perturbation-neighborhood/` | `attempted` | `supporting-only` | Supports local/HKO-neighborhood interpretation, not the generic-search headline by itself. |
| `M009` | scalar hypothesis testing | omega hypothesis correlation check | `STAT-CORR-SPEARMAN` | scalar geometry heuristic on landscape packets | `experiments/combinatorial-cells/omega-hypothesis/`, `tasks/landscape.md` | `attempted` | `supporting-only` | Negative heuristic check rather than a main search method. |
| `M010` | visual exploration | 3D projection / picture inspection | `EDA-VIS-PROJECTION`, `EDA-VIS-SCATTER` | exploratory geometry and dynamics views | `research/visualization.md`, `tasks/landscape.md` | `attempted` | `supporting-only` | Negative exploratory result; useful as communication and failed-pattern evidence. |
| `M011` | supervised regression | feature-block regression with ridge and random forest | `ISLR-REG-RIDGE`, `ISLR-TREE-RF`, `ESL-SEL-RIDGE`, `ESL-TREE-RF` | tabular feature blocks over random and endpoint regimes | `experiments/sys-landscape/datascience/methods/feature-pattern-search/analyze.py`, `research/sys-landscape.md`, `tasks/landscape.md` | `attempted` | `main-evidence` | Current repo summary treats the negative transfer result as claim-bearing. |
| `M012` | supervised classification | regime classification with logistic regression and random forest | `ISLR-CLS-LOGIT`, `ISLR-TREE-RF`, `ESL-LIN-CLS`, `ESL-TREE-RF` | grouped tabular regime-separation task | `experiments/sys-landscape/datascience/methods/feature-pattern-search/analyze_regime_classification.py`; `experiments/sys-landscape/datascience/methods/feature-pattern-search/regime-classification-report.md`; `experiments/sys-landscape/datascience/methods/feature-pattern-search/regime_classification_summary.md` | `attempted` | `supporting-only` | Reset-contract report marks this negative as a caveated diagnostic: regimes separate under grouped CV, but the result is not a label-free search rule. |
| `M013` | residualized regression check | endpoint residual analysis beyond metadata | `ISLR-REG-RIDGE`, `ISLR-TREE-RF`, `ESL-SEL-RIDGE`, `ESL-TREE-RF`, `EDA-VIS-RESIDUAL` | endpoint-only tabular packet after metadata baseline subtraction | `experiments/sys-landscape/datascience/methods/feature-pattern-search/analyze_residual.py` | `attempted` | `undecided` | Present in repo; whether it is claim-bearing, supporting, or spike-only still needs judgment. |

## Planned Use In Phase 2

- Add skipped and inapplicable rows by mapping frozen taxonomy IDs into this file.
- Add more explicit `redo-before-thesis` uses where a repo method exists but should not yet be cited.
- Keep row additions cheap; prefer adding rows here over caching state back into taxonomy files.
- Add additional frozen external taxonomies for exploratory data analysis and trajectory-analysis families if those method families should be represented outside the learning/optimization taxonomies.
