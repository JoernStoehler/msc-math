<!--
Purpose: once-run final gate for declaring the thesis done.
Context: this file is optimized for repeated reading and one final execution.
Reusable checks and operationalization details live in the verification skill.
-->

# Verify Thesis Done Roadmap

## Status

- State: blocked until thesis assembly and topic obligations close.
- Last updated: 2026-04-25.
- Source surfaces: `research/INDEX.md`, `tasks/*.md`, `thesis/`,
  old harness extraction: verification packet candidates, `thesis/submission/README.md`.
- Refresh when: a thesis story, proof obligation, thesis prose, repo promise,
  submission requirement, or archive requirement changes.

## Steering Cache

- [accepted 2026-04-25] This file owns the once-run final "the thesis is done"
  declaration. It is intentionally redundant with reusable verification packets
  because it optimizes for final reading and signoff, not repeated execution.
  Source: Jorn.
  Why it matters: final gates are read often and run once; repeatable checks are
  read and run many times.
- [accepted 2026-04-25] Reusable quality measurement, cached operational
  definitions, and check procedures live in the `verification` skill and its
  packet files, not in this task bundle.
  Source: Jorn.
  Why it matters: verification packets can evolve independently while this file
  stays a stable final decision surface.
- [accepted 2026-04-25] Submission is downstream of this file. `tasks/submit-thesis.md`
  starts once the final thesis-done gate passes, except for external-clock
  actions that can be prepared earlier.
  Source: Jorn.
  Why it matters: "done" and "submitted/archived" stay separate.

## Final Thesis-Done Gate

Jorn can declare the thesis done when all gates below pass or Jorn explicitly
accepts a named caveat as non-blocking for submission.

| gate | required condition | evidence surface |
| --- | --- | --- |
| Story obligations closed | Every retained thesis story in `research/INDEX.md` has its proof, interpretation, writeup, verification, and cut/weaken obligations closed or explicitly moved to future/cut in the relevant topic bundle. | `research/INDEX.md`, `tasks/hko.md`, `tasks/landscape.md`, `tasks/numerics.md`, `tasks/writing.md` |
| Thesis artifact ready | The final thesis PDF builds, has no silent placeholders, and has passed the intended readability/proofread review level. | `tasks/writing.md`, `thesis/` |
| Claim support checked | The reusable thesis-story support pass reports no blocking missing proof, missing evidence, stale interpretation, or uncaveated overclaim. | old harness extraction: verification packet candidate `thesis-stories-are-supported.md` |
| References and provenance checked | Bibliography, cross-references, theorem/proof references, figures, tables, experiment artifacts, datasets, and code references resolve at the level the thesis uses them. | old harness extraction: verification packet candidate `references-resolve.md`, old harness extraction: verification packet candidate `data-and-figures-are-traceable.md` |
| Repo promises checked | Every thesis-facing repo, code, command, reproducibility, and archive promise is true or caveated. | old harness extraction: verification packet candidate `repo-promises-are-truthful.md`, `tasks/reproducibility.md`, `tasks/infrastructure.md` |
| External submission prerequisites known | Submission requirements are known enough that no thesis-content work remains hidden behind an administrative unknown. | `tasks/submit-thesis.md`, `thesis/submission/README.md` |
| Open work classified | Every remaining open row in `tasks/*.md` is either non-thesis future/follow-up, external-clock submission mechanics, or explicitly accepted as non-blocking by Jorn. | `ROADMAP.md`, `tasks/*.md` |
| Jorn final acceptance | Jorn says the thesis is ready to submit and no remaining thesis-scope work should block submission. | explicit Jorn decision |

## Work Map

| item | state | value class | owner/gate | next action | source |
| --- | --- | --- | --- | --- | --- |
| Final thesis-done declaration | `[blocked]` | mainline thesis | Jorn after agent prep | Run this gate only after writing, topic, reproducibility, and verification passes have no blocking findings. | this file |
| Reusable verification packet coverage | `[active]` | mainline thesis | agents | Rebuild repeated check definitions from the old verification packet candidates before running this final gate. | old harness extraction: verification packet candidates |
| Submission handoff | `[blocked]` | external clock | Jorn / agents | Once thesis-done passes, run mechanical submission and archive tasks. | `tasks/submit-thesis.md` |

## Agent Cache

- [fresh 2026-04-25] The old `FINAL-VERIFICATION.md` was intentionally
  replaced by this compact final gate plus reusable verification packets.
  Refresh by: checking the rebuilt verification surface, `research/INDEX.md`,
  and topic bundles.

## Pruned / Stale

- [stale 2026-04-25] Do not recreate the large final truth tree here. If a
  repeated check needs more operational detail, add or edit a verification
  packet in the future rebuilt verification surface.
