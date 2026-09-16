# Recovered thesis candidate

`main.tex` is the canonical entry for the recovered working candidate. This
is the frozen 86-page review baseline from September 14, 2026, with dependency
paths relocated into ordinary project storage. Scientific content is unchanged.
It is not an approved thesis or an integration of later revisions.

Build from the repository root:

```sh
sh thesis/candidate/build.sh
```

The output is `thesis/candidate/build/main.pdf`; the entire `build/` directory
is ignored. The candidate directory is self-contained for the PDF build given
an installed TeX toolchain: it needs neither `.git/codex/` nor the original
`thesis/` sources. The build uses pdfLaTeX, latexmk, Biber, and the packages
loaded by `main.tex` and `legacy/preamble.tex`. Tested versions and an isolated
build comparison are recorded in [the recovery report](../../docs/source-recovery/README.md).

`recovered/` contains inputs formerly under `.git/codex/`. `legacy/` contains
the recorded inputs from the earlier thesis tree, copied to avoid overwriting
that version. `support/` holds four small historical generator/provenance files
that are not PDF build dependencies. Their historical paths and commands are
preserved verbatim; they are not portable build commands. The dependency
manifest records each origin, checksum, and path rewrite.

The pentagon rewrite under `docs/pentagon-chapter-v2/` and the literature
corrections from the subsequent review remain separate pending work. This
baseline deliberately retains the old pentagon proof and existing claims.
Future integration should edit this candidate, not the hidden recovery source.

The recovered files have not been committed. Until the new source and assets
are added to Git, a fresh checkout will not contain this candidate. Building
the PDF does not reproduce the experiments or certify the mathematical claims.
