# Published Code And Data Content Notes

Status: section-local content companion for `thesis/12-published-code-data.tex`.
Not source truth.

Purpose: gather the publication and reproducibility claims before final prose is
written.

Overruled by: final repository state, `submit/`, `FACTSHEET.md`, and Jörn/Kai
review.

Lifecycle: keep while publication mechanics are unsettled. After the section is
stable, delete this file or reduce it to a short maintenance note.

Update rule: add or change a claim only with a source pointer or an explicit
`needs source` marker.

## Accepted chapter boundary

- Explain what published code and retained data support the thesis; do not turn
  the chapter into a repository run manual. Detailed commands live in owner
  READMEs and the root `REPRODUCIBILITY.md`.
- GitHub (`https://github.com/JoernStoehler/msc-math`) is the living,
  agent-oriented repository. One Zenodo record is the frozen thesis-support
  plus continuation archive. Software Heritage is not a
  promised publication surface; mention it only if a clean origin is actually
  published.
- The final archive is a useful near-mirror of the closure-time repository plus
  the checked thesis PDF. It excludes raw sessions, credentials, genuinely
  private correspondence, disposable state, and third-party material without
  clear redistribution rights. Ordinary coordination or administrative context
  is not presumed sensitive.
- Retained data are selected for immediate interpretation, validation, or
  continuation, especially when regeneration is expensive. Small data use Git;
  large or poorly diffing data use Git LFS and enter Zenodo as hydrated bytes.
- Do not promise that every repository executable is rerun by one command.
  Reproduction routes stay with their owning experiments.

## Sources the writer should use

- `REPRODUCIBILITY.md`: concise cross-project publication and data policy.
- `submit/archive-closure-checklist.md`: final cleanup/publication gates; it is
  not thesis prose.
- `LICENSE.md`: Apache-2.0/CC BY 4.0 material boundary.
- `experiments/hko-local-maximum/theorem/README.md`: HKO exact certificate.
- `experiments/regular-products/pentagon-rotation-formula-proof/README.md`:
  rotated-pentagon exact certificate.
- `experiments/sys-datascience/README.md`: retained 14,336-row bounded negative
  search result and frozen generated-candidate evidence.
- `experiments/sys-landscape/gradient-ascent-observed-general/README.md`:
  twelve-start finite first-order experiment.

The current TeX already records the two certificate packets and their checking
boundaries. The writer should add a compact repository/archive account and the
retained empirical-data routes above, calibrated to the claims in their owning
chapters.

## Final local placeholders

The chapter may be drafted completely while leaving conspicuous local TODOs
only for values that genuinely arise at closure:

- Zenodo DOI and URL;
- final archive filename and byte size;
- final release commit SHA and archive SHA-256;
- publication/upload date;
- exact final environment versions, but only if the prose chooses to name them.

The GitHub URL is already known. Do not leave policy placeholders for archive
contents, Software Heritage, licensing, or what “reproducible” means.

Omit repository-wide core-hour estimates, history-age claims, maintenance
philosophy, and a global rerun promise unless a later source-backed reader need
appears. They are not required to write this chapter.
