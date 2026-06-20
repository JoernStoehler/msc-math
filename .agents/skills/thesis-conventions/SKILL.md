---
name: thesis-conventions
description: Use when Codex writes, edits, reviews, or delegates thesis-facing LaTeX/prose work in `thesis/`, especially publication-facing mathematical writing, experiment exposition, figures, or self-contained thesis assets.
---

# Thesis Conventions

Use this skill for thesis-facing text and assets. The audience is Kai,
Elizabeth, and future master students who build on this thesis. Software
engineers are not the audience, so thesis prose should explain mathematics,
experiments, and evidence rather than code mechanics.

## Source And File Boundaries

- Active `thesis/*.tex` files are reader-facing thesis text: prose, theorem
  statements, proofs, figures/tables, labels, citations, and short local
  comments that prevent misrepresenting proof or evidence status.
- Section-local `*-content.md` companions are for what-to-say notes, source
  pointers, caveats, fallback branches, review gates, and open writing
  decisions. They can point to source truth, but they are not themselves
  evidence and should not be cited as evidence in thesis prose.
- `thesis/legacy/` is source material only. Revalidate legacy claims before
  importing them, and prefer extracting facts/conventions/hazards before
  writing fresh active prose.
- `thesis/` is self-contained. Do not `\input` from `formal/`, `experiments/`,
  or `crates/`; deliberately copy thesis-facing assets into `thesis/` when
  needed.
- Figure formatting, including fonts, sizes, and colors, is owned by the
  figure-producing code. LaTeX includes the final asset.

## Thesis Prose Review

When writing, reviewing, or delegating thesis prose, first ask what could make
the changed text fail as thesis text in its current section. Use the bullets
below as common high-value review lenses, not as a complete taxonomy. They are
examples of recurring concerns that often catch most of the risk; add
task-specific concerns from the changed text, the source material, and the
current thesis role of the section. Scale the review to the risk and size of
the edit; do not turn tiny wording fixes into ritual checklist work.
Do not suppress a concern merely because it is not named below.

- **Correctness:** mathematical, experimental, and contextual claims are true
  under the stated hypotheses and current thesis conventions.
- **Source support:** claims are backed by the right source: proof, citation,
  experiment, data artifact, checked `formal/` source/status, or Jörn/Kai review
  status. Unsupported claims are removed, caveated, or marked outside
  reader-facing prose.
- **Claim strength:** wording does not overstate what the source supports.
  Watch especially for theorem-strength wording, empirical evidence presented
  as proof, and literature statements strengthened beyond the citation.
- **Thesis fit:** the material belongs in this section and helps the surrounding
  thesis argument, exposition, or evidence chain.
- **Reader understanding:** the intended reader can form the intended
  understanding without repo archaeology, stale legacy files, or guessing why an
  object was introduced.
- **Writing quality:** prose should help the intended reader form the intended
  understanding with minimal unnecessary effort and minimal risk of
  misunderstanding. Common failure modes, not a checklist or definition, include
  unclear purpose, order that hides dependencies, weak coherence between
  sentences or paragraphs, imprecise wording, unnecessary verbosity,
  hard-to-parse sentences, misplaced emphasis that changes perceived importance,
  and prose that still reads like notes, comments, chat, or legacy scaffolding.
- **Scope and consolidation:** the edit absorbs or intentionally excludes the
  source material needed for the current task, or explicitly leaves out material
  that is out of scope, without starting unplanned proof, experiment, or cleanup
  work.
- **Presentation and mechanics:** notation, labels, displayed equations,
  citations, cross-references, figures/tables, build output, and diff hygiene are
  appropriate for the review surface.

Writing quality, thesis fit, and reader understanding are judgment-based. When
reviewing those lenses, cite concrete locations and explain the issue
instead of giving bare taste judgments. If a real concern does not fit the
common lenses above, report it anyway and name the concern it suggests for this
task.

## Review And Delegation

- Agents own the ordinary review of their thesis-facing work. Escalate to
  Jörn/Kai only for gates where expert or advisor judgment is required, such as
  theorem-strength wording, advisor framing, thesis scope, or final thesis
  readiness.
- For mathematical thesis prose, use `formal-math` expectations as well:
  prioritize false claims, missing hypotheses, proof gaps, convention errors,
  and unmarked verification status.
- Review subagents are useful for bounded surfaces. Ask them for findings and
  spots of interest, not decisions. Give them the changed files, source files,
  and the review dimensions that matter for the task.
- Final reports after nontrivial thesis prose work should name the review passes
  performed, including source comparisons, build checks, and review subagents
  used or intentionally not used.
