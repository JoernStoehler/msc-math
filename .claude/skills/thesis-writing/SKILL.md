---
name: thesis-writing
description: Workflow for iterating thesis content to publication quality. Load when writing or substantially revising thesis chapters, not for small edits or formatting fixes.
---

# Thesis Writing Workflow

## Before writing

1. Read the target section's current state and surrounding context
2. Read the relevant math.tex files and experiment logbooks that feed into this section
3. Check TASKS.md for open TODOs affecting this section
4. Identify what Jörn has approved vs what's unreviewed (look for `% Jörn:` markers)

## Writing

5. Write content in proper environments (`\lemma`, `\theorem`, `\proof`, `\remark`, `\definition`)
6. Wrap new agent-written math in `\begin{unverified}...\end{unverified}`
7. Every factual claim verified against source: data from JSONL, code via grep, citations via .bib
8. Cross-references use `\ref{label}`, never hardcoded numbers
9. Notation matches `appendix-notation.tex`

## Self-review iteration

10. Re-read what you wrote as if encountering it for the first time
11. Check: is each lemma self-contained? Can a reader reach it via `\ref` and understand it without reading surrounding prose?
12. Check: are there adjective clusters, dramatic language, or `\texttt{}` in theorem environments?
13. Check: does every proof state its assumptions, claim, and conclusion explicitly?
14. Build: `cd thesis/ && latexmk && ./check-build.sh` — fix any errors or warnings

## Before presenting to Jörn

15. Spawn review subagents as appropriate (math-review, claim-verify, figure-review) [TODO: specify orchestration]
16. Present: what section was written/revised, which theorem numbers (from main.aux) to look at, what's unverified