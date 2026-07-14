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

## Content Inventory

- Current text records the two exact SageMath packet locations, the differing
  explicit-check/Python-assertion boundaries, their owner-local rerun READMEs,
  and the lack of embedded source/environment identity in retained generated
  records.  This material was moved from the rejected standalone Sage appendix.
- State the repository structure at a high level.
- State the live GitHub repository, with the caveat that it may retire in a few
  years, and the permanent uploads on chosen archive sites once known.
- Accepted archive direction, 2026-07-14: GitHub remains the living repository;
  one manually curated Zenodo record holds the frozen thesis-support plus
  continuation bundle; Software Heritage complements it for public Git source
  history. Fill exact URLs and identifiers near submission.
- State which experiment artifacts support thesis claims.
- State that data is committed, the git history is not pruned and covers
  roughly half the thesis lifetime, the thesis PDF is rebuildable, and
  documentation explains how to read and run the repo.
- State which commands or archived outputs are promised.
- Do not turn the thesis into the run manual. "How to read this" and "how to
  run this" live in the repo.
- The repo promises reproducibility via the devcontainer/Docker definition.
  Open TODO: pin versions if needed before submission.
- The repo root `README.md` should be the detailed reproducibility surface. The
  thesis can copy final numbers or summaries from it.
- Include rough total core-hour estimates for reproducing different parts.
  These estimates help readers distinguish cheap sanity checks from expensive
  full reproduction and are evidence that the reproducibility promise has been
  thought through.
- Caveat: the repo documentation is optimized mostly for capable current
  GPT-5.6 agents as readers/operators, not primarily for human readers.
- Maintenance philosophy: code clarity wins by default; optimize only when
  tracing, profiling, benchmarking, or final consumers show that performance is
  material for a retained thesis computation.
- Maintenance after writeup should repair thesis/code mismatches, missing
  tests, reproducibility gaps, and profiling evidence that matters for final
  experiments.

## Open Decisions

- Fill exact GitHub, Zenodo, and Software Heritage identifiers and the exact
  reproducibility promise when the closure payload and mechanics are known.
