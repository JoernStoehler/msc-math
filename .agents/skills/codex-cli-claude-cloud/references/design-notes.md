# Design notes

## Cloud scope and preservation, 6 October 2026

This package was preserved from the September Claude-cloud workflow, not validated as a fresh-clone setup for every environment. Bootstrap installation and session-output linking remain explicit setup, with read-only status commands available separately. The pre-sweep repair preserves unexpected session-path objects, rejects every zero timeout spelling, and cleans temporary prompt/output files on early exit. Mock checks establish those mechanics, not current cloud login or model execution.

Jörn explicitly declined standing authority for future device grants in the [interop source](https://github.com/JoernStoehler/agent-skills/blob/main/codex-claude-interop.md). Per-grant approval remains; the classifier and nine-box UI are dated observations. Sandbox default remains workspace-write. The source checkout has separate ownership; this project does not promise its availability or synchronized deployment.

Open choices: whether setup should become separate opt-in install/link commands; what current Claude-cloud support should be promised. Changing those is a workflow design choice rather than a consequence of structural validation. See the [pre-sweep assessment](../../../../docs/history/skill-presweep-20261006/README.md).

## Naming and account-resource boundary, 6 October evening

The independent reset investigation [handoff](../../../../docs/history/reset-investigation-20261005/skill-maintenance-handoff.txt) reports Jörn's objection that `codex-usage` is a bad name. This skill means using the CLI from Claude cloud, not tracking account quota or executing/recovering resets. Preserve its useful workflow while assessing a discriminating name (candidate: `codex-cli-from-claude`); no rename or account-management expansion was activated at intake.

The [maintained comparison](../../../../docs/history/skill-presweep-20261006/README.md#resource-reset-investigation-handoff-6-october-evening) retains actual resource owners, the unimplemented skill/discovery gap and alternative complete versus split resource/reset workflows. It also preserves the withdrawn low-usage advice, unknown production refusal cause, mock-tested CLI evidence and recovery-authority boundaries. No new production action or automatic-hook revival follows from these notes.

The later independent review preferred `codex-cli-claude-cloud`: preserved setup evidence is cloud-specific. That rename is now integrated locally, with the script contents and authorization contract retained; its discovery link and runtime caller migrated. `codex-resources` is the separate local guidance owner. Neither change establishes current cloud execution or production recovery capability.
