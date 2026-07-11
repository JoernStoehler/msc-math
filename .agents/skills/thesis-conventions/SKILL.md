---
name: thesis-conventions
description: Use when Codex writes, edits, reviews, or delegates thesis-facing LaTeX or prose work in `thesis/`, especially publication-facing mathematical writing, experiment exposition, figures, or self-contained thesis assets.
---

# Thesis Conventions

The audience is Kai, Elizabeth, and future master's students who build on this
thesis. Explain mathematics, experiments, and evidence rather than software
mechanics.

## Source And File Boundaries

- Active `thesis/*.tex` files contain reader-facing thesis text and short local
  comments needed to preserve proof or evidence status.
- Section-local `*-content.md` companions hold source pointers, caveats,
  fallback branches, review gates, and open writing decisions. They are
  navigation, not evidence to cite in thesis prose.
- `thesis/legacy/` is source material. Revalidate claims before importing them.
- Keep `thesis/` self-contained. Deliberately copy publication assets rather
  than depending on `formal/`, `experiments/`, or `crates/` at build time.
- Figure-producing code owns figure formatting; LaTeX includes the result.

## Review Lenses

Choose lenses according to the changed text and its role. The list is not a
complete taxonomy.

- **Correctness:** Are mathematical, experimental, and contextual claims true
  under their stated hypotheses?
- **Source support:** Does each claim have the right proof, citation, artifact,
  formal status, or accepted review status?
- **Claim strength:** Does the wording distinguish theorem, empirical evidence,
  conjecture, and cited literature without strengthening the source?
- **Thesis fit:** Does the material help the surrounding thesis argument,
  exposition, or evidence chain?
- **Reader understanding:** Can the intended reader form the intended
  understanding without repo archaeology or guessing why an object appears?
- **Writing quality:** Do purpose, dependency order, coherence, precision,
  sentence structure, emphasis, and length minimize effort and
  misunderstanding? Does the text read as thesis prose rather than notes,
  comments, chat, or scaffolding?
- **Scope and consolidation:** Did the edit absorb or deliberately exclude the
  source material needed for its task without starting unrelated work?
- **Presentation:** Are notation, equations, labels, citations, references,
  assets, build output, and diff hygiene appropriate?

These lenses support partial feedback passes rather than requiring every pass
to be a full thesis review. For judgment-based findings, cite a concrete
location and explain the effect instead of giving a bare taste judgment. Report
important concerns even when they do not fit a named lens.

Agents own ordinary review. Escalate theorem-strength wording, advisor framing,
thesis scope, and final-readiness decisions when expert or advisor judgment is
actually required. Apply `formal-math` status and rigor conventions to
mathematical thesis prose.
