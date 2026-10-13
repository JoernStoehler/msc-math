# Corrective maintenance results

## Consultation record

Consulted five maintained reference pages from `agent-platform-reference`: `references/INDEX.md` (2,199 bytes), `references/corrective-maintenance.md` (3,783 bytes), `references/user-input-lifecycle.md` (3,862 bytes), `references/goal-loops.md` (4,636 bytes), and `references/corrections.md` (5,850 bytes). Total read: 20,330 bytes. The index routed to the right surface; maintenance guidance selected the repo instruction/conditional guide scope; lifecycle and goal pages supplied the mechanisms; corrections was checked before making platform claims. Source references recorded in `diagnosis.json` include the pinned goal-source review and model catalog review.

## Maintained change and decision updates

Added `.agents/skills/input-and-goal-recovery/SKILL.md` and a short `AGENTS.md` pointer. The guide now distinguishes required answers from optional preferences, treats async delivery as non-blocking, reconciles steering against unresolved input, and documents Codex goal activation, continuation and notification boundaries. This is intentionally a repo-level behavioral correction: it reaches future agents that read this repository's instructions, without asserting a runtime change.

For `questions-after-steer`, the concise-report steer does not resolve the required target. The target-specific implementation remains gated, and `next-user-message.txt` contains the actual question to surface again. For `goal-missing-input`, the ordinary heading does not justify `/goal`; the trace's `create_goal` call is the activation evidence to investigate. The guidance directs future work to use exposed controls and escalate a first-blocker notification defect to the runtime owner rather than guessing, inventing a catalog override, or claiming completion.

## Goal-case source correction

Reconsulted the updated `agent-platform-reference/references/goal-loops.md` (5,888 bytes) and `agent-platform-reference/evidence/human-input-source-review.md` (4,486 bytes), 10,374 bytes total. The evidence separates the ordinary end-of-turn completion alert from `AsyncQuestion` and `PlanModePrompt` notifications. In pinned source, `request_user_input` can be available during an active goal; it blocks and awaits only in Plan mode and only when its feature is enabled/tool exposed. Default-mode exposure has an additional flag and may auto-resolve. Therefore inspect the actual session's collaboration mode, tool exposure, and notification settings, then ask “Which deployment target should the pipeline migration use?” via a verified available path. Do not infer that input requires goal termination or assume a specific tool is exposed.

Supported intervention choices are a short repo `AGENTS.md` pointer (discovery), the repo-local conditional recovery guide (procedure), a supported catalog description override only if effective schema proves the relevant description is overridable and misleading, or a harness/runtime change if the verified available question route fails. The user maintains one technical skill; this correction is a repo-local conditional guide proposal, not installation or modification of that global skill. Actual runtime settings remain unknown.

## Validation and limits

Validated that the case IDs in `diagnosis.json` match `cases.json`, both required dependencies are represented as unresolved in `outstanding-inputs.json`, both follow-up messages ask for their respective targets, and the fixture guidance contains no target guess or fabricated override. This is post-failure diagnosis and maintenance, not a Codex TUI reproduction or held-out evaluation. No live client/account setting was changed. Reported UI disappearance and notification suppression were not independently replayed; source findings are scoped to `rust-v0.162.1`.
