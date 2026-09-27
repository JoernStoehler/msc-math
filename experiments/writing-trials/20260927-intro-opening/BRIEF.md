# Writing trial 2026-09-27: introduction opening

Question this trial answers: which authoring workflow produces an introduction
opening Jörn judges closest to PASS? Jörn compares the variants blind, as
rendered pages. The winning workflow gets scaled; losers are dropped.

## Deliverable (all variants)

`<variant>/opening.tex`: replacement for the part of
`thesis/chapters/01-introduction.tex` from `\section{Introduction}` up to (not
including) `\subsection{Capacities, dynamics and convex geometry}`. Keep
`\section{Introduction}\label{sec:introduction}`. Target length: 1 to 1.5
rendered pages. It must lead naturally into §1.1–1.5 (see `toc.txt`); you may
say what those subsections cover but they are not rewritten here.

Build check: `sh experiments/writing-trials/20260927-intro-opening/build.sh <variant>`
must succeed. Only cite keys that exist in `thesis/references/*.bib`.

Also write `<variant>/notes.md` (≤150 words): what you did, anything you were
unsure about. Do not edit any other file.

## Hard requirements

- Framing: `docs/project-facts.md` items 90–97 (confirmed by Jörn today) plus
  items 8–11. These override the current introduction and abstract.
- Every mathematical or empirical claim must match the thesis chapters
  (`thesis/chapters/*.tex`) at the strength they support. Check before stating.
  Never write the raw-false "no bodies with sys > 1 were found".
- Readers: real readers Kai (supervisor) and Elizabeth; imagined reader a
  master student who has taken a symplectic geometry lecture and basic
  optimization.

## Evidence about Jörn's taste (read before writing)

- `docs/project-facts.md` item 97 (his notes on the current p. 5).
- `experiments/writing-quality/human-review/responses/20260918-root-contemporary-ds-reader.md`
  (12 annotations on a DS draft).
- `docs/history/reviewer-trial/human/microbatch-1.md` … `microbatch-4.md`.
- Short version: say exactly what was obtained; use one term per concept
  (e.g. always "systolic ratio"); no filler topic sentences; no hedging or
  provenance clauses in reader prose; skip what is trivial for an MSc student,
  define what they need; first sentences of paragraphs should carry the
  results.

## Result material

`thesis/chapters/00-abstract.tex` lists the results (but its framing is the old
one), `toc.txt` the structure, `thesis/chapters/15-ai-reflection.tex` and
`thesis/ai-use-disclosure.tex` the AI facts, `thesis/chapters/08-data-science.tex`
the data-science outcome, `thesis/chapters/07-hko.tex` the local theorems
(including the 24 Sep non-HKO local maximum at ratio one).
