# Taxonomy: Numerical Continuation / Homotopy Methods

Source intent:

- Frozen taxonomy snapshot for continuation and homotopy-style methods adapted from Allgower--Georg, *Numerical Continuation Methods*.
- This file is deliberately narrow: it exists because continuation is a real external family relevant to the repo, not because it should dominate the search taxonomy.

## Parameter Continuation

- `CONT-PARAM-SIMPLE` Simple parameter continuation
- `CONT-PARAM-PREDCORR` Predictor--corrector continuation

## Path Following

- `CONT-PATH-ARC` Arc-length continuation
- `CONT-PATH-BRANCH` Branch following and branch switching

## Homotopy And Deformation Methods

- `CONT-HOMOTOPY-ROOT` Homotopy continuation for root/path tracking
- `CONT-HOMOTOPY-SOLUTION` Solution continuation under changing problem data

## Likely Use In This Project

- Strongest overlap: continuation from one local search surface to a nearby changed surface, for example changing facet count or other problem structure.
- Weak overlap: full theorem-grade path-following machinery or certified bifurcation analysis.
