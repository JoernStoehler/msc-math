---
name: formal-math
description: Use when Codex writes, edits, reviews, or delegates review of mathematical writing in this repo, especially `formal/*.tex`, owner-local proof notes, theorem statements, proof sketches, verification-status comments, or code comments that claim correspondence with formal mathematics.
---

# Formal Math Conventions

`formal/` is developer-facing mathematics. Jörn reviews the PDF, while future
agents are its primary readers. Optimize it for correctness, verifiability,
and usefulness to the thesis and empirical work.

- Formalize statements rigorously enough to expose wrong claims and edge cases.
  State every condition and guarantee of lemmas and mathematical algorithms.
- Track verification status precisely: distinguish proof idea from
  formalization, trusted main case from unchecked edge cases, notation trouble
  from a mathematical gap, and agent review from Jörn approval.
- Agent-written mathematics may be expert work, but writing or committing it
  does not make it Jörn-reviewed or accepted. Record that epistemic status
  where it matters.
- For theorem-critical new arguments, independent review is additional evidence,
  not authority. When testing whether the proof stands on its own, give the
  reviewer the statement and necessary sources without coaching it toward the
  intended derivation; verify any reported gap or repair.
- Preserve the current reason for definitions, statements, and proof methods;
  leave irrelevant attempt history in Git.
- Use grep-able LaTeX labels and references. Do not hardcode theorem numbers.

When requesting Jörn's review:

- rebuild `formal/main.pdf`;
- identify the numbered items Jörn should review and the numbered context they
  depend on;
- state the requested questions or review aspects in priority order.

`references/review-prompt-learnings.md` records one observed exposition
preference. Treat it as evidence from that case, not a universal writing rule.
