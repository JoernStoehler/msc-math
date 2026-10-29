# Exact replay: the sharp six-simplex inequality

This package checks the finite inputs to the theorem

`max c_EHZ(K)^3 / (6 volume_6(K)) = (945 + 75 sqrt(5))/1922`

for six-dimensional simplices, together with the algebra used for its equality
classification. Inputs are generated from definitions inside the four scripts;
no external arrays, downloads, repository checkout, or unpublished data are
required.

## Run

Use the Python interpreter of a SageMath environment that includes the exact
`cdd` and `ppl` polyhedron backends. Tested: SageMath 10.9 with Python 3.13.15.
`algebra.py` and `local_maximum.py` require only Python's standard library;
the two coverage scripts additionally import `sage.all`.

Copy these files to a fresh working directory, enter it, and run:

```sh
SAGE_PYTHON=/path/to/sage-environment/bin/python
"$SAGE_PYTHON" algebra.py > algebra.out
"$SAGE_PYTHON" local_maximum.py > local_maximum.out
"$SAGE_PYTHON" dual_coverage.py > coverage.out
```

`dual_coverage.py` imports and runs `full_classification.py`; running the latter
separately is unnecessary. It writes `full_classification.json` and
`dual_coverage.json` in the current directory. All three commands must exit
successfully. Do not use Python's `-O` option: the exact checks use assertions.

The tested conda Sage launcher does not accept `sage -python`; using its Python
interpreter directly is the command actually verified here. In an activated
Sage environment where `python -c 'import sage.all'` succeeds, `SAGE_PYTHON=python`
is sufficient. No additional pip packages are needed.

## Checks and expected outcomes

- `algebra.py`: an independent determinant construction of all 6,545 triple
  coefficients agrees with the signed-perfect-matching Pfaffian expansion;
  735 coefficients are nonzero. An independent clique recursion finds 420
  maximal five-triple packings and 30 Fano packings. It checks the F32 support
  pattern, the fence-pyramid quadratic matrix and its six-square decomposition,
  and the two permutations generating all four equality sign patterns.
- `local_maximum.py`: arithmetic in `Q(sqrt(5))` checks all 720 order scores,
  the 21 active normals of rank 11, five isolating inequalities, the exact
  golden value, a positive normal-cone gradient identity, and negative
  definiteness of the four-dimensional face Hessian by exact principal minors.
  This supplies the strict-local isolation used in the global equality proof.
- `full_classification.py`: six permutation generators preserve all 70
  relaxation inequalities; two disjoint vertex orbits have sizes 720 and 840.
  Exact cdd and PPL backends agree on every tangent ray at both representatives.
  The order representative has 186 rays, ending at 165 order and 21 fractional
  vertices. The fractional representative has 18 rays, all ending at orders.
  Neighbor closure plus connectedness of the polytope graph establishes the
  complete 1,560-vertex list.
- `dual_coverage.py`: checks the 840 fence forms, the unique score `6/5` and
  all other scores at most `4/5`, the 18-vertex base facet of affine rank 14,
  and the 18 integral neighbors lying on the associated cutting hyperplane.
  These are the finite predicates for the sequential-cut and pyramid-cover
  arguments in the proof.

The checks use exact integers/rationals or explicit `Q(sqrt(5))` arithmetic.
The polyhedral ray enumeration relies on Sage's exact cdd and PPL libraries;
this is not a proof-assistant formalization. The human proof supplies the
mass-merging argument, packing classification, elementary Fano bound,
sequential-cut induction, midpoint cancellation, cubic comparison, and
strict-local-to-global equality argument.

## Successful replay receipt

`receipt.json` records an independent run of a copy of these four scripts in a
new empty scratch directory, including exact script hashes, interpreter and
library versions, commands, exit codes, and timings. The adjacent `.out` and
`.json` files are the output of that copied-package run. The runner used no
source files outside its copied package. CPU timing fields naturally vary.

The scripts originate from the independent algebra/local and polyhedral audits.
Portability edits only replace absolute JSON output paths with current-directory
filenames; the mathematical checks are unchanged.
