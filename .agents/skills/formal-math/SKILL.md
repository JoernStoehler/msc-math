---
name: formal-math
description: Use when Codex writes, edits, reviews, or delegates review of mathematical writing in this repo, especially `formal/*.tex`, research proof notes, theorem statements, proof sketches, verification-status comments, or code comments that claim correspondence with formal mathematics.
---

# Formal Math Conventions

## Instrumental Objectives
- `formal/` is developer-facing mathematics. Jörn reviews the pdf, but the primary readers are future agents.
- clarity: all agents and Jörn understand the proofs
- correctness: no false claims are made, gaps and untrusted steps are marked, the math is rigorously formalized
- verifiability: future agents can easily follow argumentation, reasoning, proofs, applications to check correctness
- usefulness: the definitions and theorems, and to a lesser degree the proofs, are useful for the thesis and for empirical work
- navigation: related content is grouped together, unrelated content is separated as files. labels are grep-able.

## Conventions
- write plainly, be specific, neither over- nor under-inclusive, break down sentences that entangle multiple concerns, avoid vague terms, avoid analogies and metaphors
- do not invent new terminology, disambiguate terminology with extra adjectives and long names. the agent readers are familiar with most mathematical literature and naming/notation conventions.
- the focus is rigorously formalized mathematics that allows us to catch any wrong statements and edge cases.
- clearly track the verification status of mathematical writing, such as whether Jörn reviewed a formalization or proof, what gaps remain and why those look closeable, whether the proof idea is trusted and notation troubles are the obstacle, whether generic/main cases are trusted and what edge cases cause trouble, and so on
- use comments to track the "why" behind the current definitions/statements/proof methods, don't discuss historical attempts beyond what matters for the current state and for anticipated future work.
- use grep-able LaTeX labels and reference them
- be fully rigorous in what conditions and guarantees lemmas claim, and in what inputs and outputs algorithms provide.
- new agent-written mathematics is unapproved unless it is mechanical or
  explicitly approved by Jörn
- don't hardcode theorem numbers; use labels and check references

## Feedback from Jörn
- rebuild `formal/main.pdf`
- look up the numbers of the items you want feedback on, and the ones that Jörn needs to read for context
- tell Jörn what questions/aspects/writing you want feedback on, ordered from most important to least, to prevent nitpicking

## Feedback from Subagents
- Subagents are cheaper, faster, but less reliable than Jörn. 
- They have a high false-negative and false-positive rate, but the true-positives and true-negatives are still useful signals / Bayesian evidence / elevate-to-attention flags. So check the positives they mention to exclude false-positive complaints.
- Tell the review subagent the review surface, the relevant latex sources, potentially relevant context like rust files or research notes when usefulness is being reviewed, and a prioritization of what aspects you care about most to least. Don't reexplain quality standards, just reference this skill.
- Ask the subagent to indicate confidence, encourage it to include more positives since we take care of false-positives in a second pass.
- Review-prompt lessons are recorded in `references/review-prompt-learnings.md`.

## Prompt Example

```text
Required cwd: /workspaces/msc-math

Use $formal-math to review the pruning-correctness writeup.

Review surface:
- formal/main.tex
- formal/search-pruning-correctness.tex
- labels `lem:transition-feasibility`, `cor:ridge-sufficiency`, and `ex:a3-prunes`

Context that may matter:
- formal/capacity-algorithms.tex, especially `cor:adjacency-pruning`
- experiments/verification/algorithm-comparison/ablation/ablation.jsonl
- experiments/verification/algorithm-comparison/ablation/analyze.py

Please review against the ordinary `formal/` quality baseline from the skill.
Prioritize, in this order:
1. false claims, missing hypotheses, proof gaps, and edge cases;
2. whether the definitions/statements are useful for later thesis or empirical work;
3. unclear exposition, overloaded terminology, or missing verification-status markers;
4. navigation issues such as bad labels, missing references, or content placed in the wrong file.

Do not create a new mathematical objective. Do not decide whether new mathematics is approved; reserve that for Jörn.

Output:
- Findings first, ordered by severity.
- For each finding, cite the file and label or nearby text anchor, explain the issue, and say whether it is a definite error/gap or a plausible concern.
- Prefer recall over precision: include plausible issues even when unsure, mark your confidence for each finding, and do not suppress concerns merely because they may be false positives.
- Include a short "Checked but found no issue" section for the main things you inspected.
- Include a short "Not checked" section for relevant surfaces you did not inspect.

Stop if the required cwd is wrong, the review surface is missing, or the task would require broad reconstruction beyond the listed context.
```
