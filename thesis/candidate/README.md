# Integrated candidate for review

This worktree contains a proposed integration of the interrupted thesis sources
and reviewed technical patches, including the analytic rotation proof and the
independent-linear-deformation extension. It is **not an accepted thesis**, a
selected permanent release checkout, or a claim of prose PASS. The original
86-page recovered baseline and interrupted checkout remain unchanged elsewhere.

The [integration ledger](../../docs/candidate-assembly-integration/README.md)
records adopted and deferred changes, exact input identities, and checks.

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

The selected recovered preliminaries and complete DS chapter remain in place.
The standalone revised DS subsection and optional further polygon results have
not been silently substituted. Reader-facing summaries have only received
bounded consistency edits and still require a substantive final synthesis.
