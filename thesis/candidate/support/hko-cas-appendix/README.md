# Annotated HKO CAS appendix

`hko-cas-appendix.tex` is a publication-oriented appendix fragment.  It is
based on the runnable owner
`experiments/hko-local-maximum/theorem/verify.sage.py` and its
`witness.json`; it is not a replacement verifier.  The 26 explicit witness
assignments are copied from `.git/codex/hko-writing-input/exact-fixed-values.json`
and the tracked witness.  Facet and word positions in the table are one-based;
the runnable JSON uses zero-based indices.

Inspected source hashes:

```
verify.sage.py       58396d326094f2625d0d9e74fd21662fbcc33e08b71bfcae602f397af2a4aa81
witness.json         98362e00408d1bc6b3085b2ed6cc23174c3270c6b0ad6219a00e3a7732f5472c
verification-summary 8b407b20033d7acc774795a2326cf4c14e021d7e7793de7a560b8b5223cd15c0
```

The appendix displays the proof-facing construction and predicates, while
omitting candidate-ordering hints, diagnostics, JSON plumbing, and the full
1040 expanded derivative entries.  The latter are reconstructible from each
displayed `(sigma,I,J,u)`, the displayed formulas, and the exact field
definition.  The existing retained summary reports: 26 rows, row rank 25,
symmetry rank 15, and a strictly positive normalized left-kernel relation.
The self-contained core was syntax-checked and executed with
`sage -python .git/codex/hko-cas-appendix/hko_core.py`; it passed with
`rows=26 rank=25 symmetry-rank=15`.  This bounded check is separate from a
full rerun of the original JSON-driven verifier.

Coordinator integrated the actual source listings into the evolving whole
candidate and checked the rendered appendix. The rejected earlier pseudocode
is no longer included. Current core SHA-256:
`475196efdf33d1f281a724b90069ec08a2335552ddc4362332e2b975866f10f5`.
An independent bounded source comparison found matching geometry, assignments,
volume coefficients, derivative factors, and symmetry generators. Additional
explicit guards check invertible minors, word/position validity, differentiated
closure, row count, and kernel dimension. The core was rerun after those guards.
The TeX listings use source line ranges; changing core line structure requires
updating the ranges to keep every executable line included.

The CAS checks do not prove the geometric chart, smooth-volume lemma,
Haim--Kislev formula, or symmetry-slice/Taylor implications.  Those are
mathematical lemmas in the thesis.  The CAS block verifies the finite
instantiations required by `lem:hko-exact-bounds`.
