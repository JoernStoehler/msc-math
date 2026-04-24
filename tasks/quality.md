<!--
Purpose: repeatable quality-baseline roadmap for thesis closeout.
Context: this is a task bundle, not the final done authority. It points agents
at checks that can be rerun and then routed into topic-specific work.
-->

# Quality Baseline Roadmap

## Status

- State: active repeatable baseline.
- Last updated: 2026-04-24.
- Source surfaces: `FINAL-VERIFICATION.md`, `.agents/skills/verification/`,
  `RESULTS.md`, `ROADMAP.md`, `tasks/*.md`, thesis/build surfaces when present.
- Refresh when: a retained thesis claim, promised repo command, figure/table,
  or submission artifact changes.

## Steering Cache

- Quality passes are repeatable gates, not a license to expand scope.
  Why it matters: a failed check should create a concrete thesis-path task or a
  cut/weaken decision, not automatic polish work.
- Final authority remains `FINAL-VERIFICATION.md`.
  Why it matters: this bundle can schedule and cache checks, but it must not
  silently redefine the archive-ready state.
- The verification skill owns reusable check packets.
  Why it matters: agents should start from `.agents/skills/verification/` and
  its `references/` files instead of inventing fresh checklists each time.
- Code-quality and test-coverage work applies to promised or thesis-cited repo
  surfaces first.
  Why it matters: broad code cleanup is future/follow-up unless a thesis claim,
  reproducibility promise, or reader-facing repo promise depends on it.

## Work Map

| item | state | value class | owner/gate | next action | source |
| --- | --- | --- | --- | --- | --- |
| Repeatable quality pass protocol | `[active]` | mainline thesis | agent | Run the protocol below before Jorn review, final assembly, and archive; split failures into topic bundles. | this file |
| Thesis claim support pass | `[active]` | mainline thesis | agent then Jorn | Check retained `RESULTS.md` claims against proof/data/source support and mark missing support, caveats, or Jorn-only judgments. | `.agents/skills/verification/references/thesis-claims-are-supported.md`, `FINAL-VERIFICATION.md:T1,T2,T6` |
| Repo promise truth pass | `[active]` | mainline thesis | agent | Check thesis-facing repo promises, command promises, data/figure provenance, and artifact locations against current files. | `.agents/skills/verification/references/repo-promises-are-truthful.md`, `FINAL-VERIFICATION.md:T4,T5` |
| Code-quality baseline | `[active]` | contingent during writing | agent | Run on crates/experiments that the thesis cites or promises; route failures to `tasks/infrastructure.md`, `tasks/numerics.md`, `tasks/hko.md`, or `tasks/landscape.md`. | `.agents/skills/verification/references/code-is-high-quality.md` |
| Test-coverage baseline | `[active]` | contingent during writing | agent | Run on theorem-critical and repo-promised surfaces; distinguish missing regression tests from slow validation experiments. | `.agents/skills/verification/references/test-coverage-is-high.md` |
| Falsification experiment baseline | `[active]` | contingent during writing | agent then Jorn | Check that thesis-used verification experiments actually try to falsify the story stated in the thesis. | `.agents/skills/verification/references/verification-experiments-try-to-falsify-the-story.md` |
| Thesis readability pass | `[Jorn]` | mainline thesis | agent prep then Jorn | Agents precheck structure, references, figure/text alignment, and TODOs; Jorn judges mathematical readability and taste. | `FINAL-VERIFICATION.md:T3,T8` |
| Bibliography and cross-reference pass | `[blocked]` | mainline thesis | thesis draft | Run once thesis text is less placeholder-heavy; create missing packet if this becomes repeated labor. | `FINAL-VERIFICATION.md:T4.1,T4.2` |
| Submission artifact pass | `[blocked]` | external clock | agent prep then Jorn | Run near final handin against university requirements, filled forms, printed copies, upload/preservation steps, and archive action. | `FINAL-VERIFICATION.md:T7,T9`, `tasks/submission.md` |

## Agent Cache

- [fresh 2026-04-24] Minimal repeatable pass protocol:
  1. Name the retained surface being checked.
  2. Load the nearest verification packet and the relevant final gates.
  3. Report findings as pass, caveat needed, missing support/stale evidence, or
     Jorn-only judgment.
  4. Patch the relevant `tasks/*.md` bundle for each retained failure, or patch
     `RESULTS.md` / thesis prose when the correct action is cut/weaken.
  5. Re-run only checks touched by the patch.
  Refresh by: reading `.agents/skills/verification/SKILL.md`.
- [fresh 2026-04-24] Missing reusable packets in the verification skill:
  bibliography/cross-reference resolution, data-and-figures reproducibility,
  and submission artifacts.
  Refresh by: `ls .agents/skills/verification/references` and the coverage
  table in `.agents/skills/verification/SKILL.md`.

## Pruned / Stale

- Do not keep a standing "polish all code" task here. Route promised-surface
  failures to concrete topic bundles; leave broad maintainability work in
  `tasks/infrastructure.md` as future/follow-up unless it blocks thesis truth.
