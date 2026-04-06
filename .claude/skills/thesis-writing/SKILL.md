---
name: thesis-writing
description: Workflow for iterating thesis content to publication quality. Load when writing or substantially revising thesis chapters, not for small edits or formatting fixes.
---

# Thesis Writing

## Before writing

- Read target section + surrounding context
- Read relevant math.tex files and experiment logbooks
- Check TASKS.md for open TODOs affecting this section
- Identify what Jörn has approved vs unreviewed (`% Jörn:` markers)

## Writing

- Use proper environments: `\lemma`, `\theorem`, `\proof`, `\remark`, `\definition`
- Wrap new agent-written math in `\begin{unverified}...\end{unverified}`
- Cross-references: `\ref{label}`, never hardcoded numbers
- Notation: match `appendix-notation.tex`

## Before presenting to Jörn

- Build: `cd thesis/ && latexmk && ./check-build.sh`
- Launch review subagents as appropriate (review-proof, review-claims, review-thesis, review-figures)
- Present: section written, theorem numbers (from main.aux), what's unverified
