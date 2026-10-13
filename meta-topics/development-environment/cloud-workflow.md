# Cloud work and handoff

This is the shared msc-math contract for continuing work in a cloud session or
another machine. It connects existing source owners; it does not select a
research programme, authorize expenditure or establish that every destination
has been tested. [Environment requirements](development-environments.md)
record the intended surfaces. Product-specific mechanics and their evidence
belong to the [agent-platform reference](../../.agents/skills/agent-platform-reference/SKILL.md).
The [surface runbook](cloud-surface-runbook.md) provides a bounded branch/tool/
return check when effective account or harness behavior needs observation.

## Deliver the intended source

Use a Git revision the recipient can actually retrieve, or an inspected source
archive. A local commit is not a remote commit. An archive of a working tree
has its own file identity; its base commit does not identify its changed bytes.
Do not push accumulated local history merely to make a handoff possible.

[scripts/cloud-handoff.py](../../scripts/cloud-handoff.py) creates selected
source archives and supplies an independent verifier. For example, from the
repository root, package the current documentation and repository skills:

```bash
python3 scripts/cloud-handoff.py create --working-tree \
  --path README.md --path AGENTS.md --path CLAUDE.md \
  --path .agents/skills --path .claude \
  --path meta-topics/development-environment \
  --path meta-topics/project-management --path scripts \
  --output /tmp/msc-math/cloud-handoff/run/source.tar.gz
```

Use `--revision COMMIT` instead for committed bytes. Select additional topic,
proof, code and input paths for the actual task. This example is a documentation
packet, **not a complete checkout or a Rust/PDF reproduction bundle**. Inspect
the manifest, selected paths and link-target warnings before transmitting it.
The helper does not classify private material or distinguish code from datasets.
Ignored credentials, build outputs and machine caches are not selected by its
working-tree inventory; tracked private material still needs deliberate omission.

Extract with `tar -xzf source.tar.gz` into a fresh directory. Follow the packaged
`FIRST-STEPS.txt` and standalone verifier; tar preserves the relative skill links
that an ordinary Python ZIP extraction would flatten. Verify after the archive
reaches the destination. Receiving a file, loading it as chat context and
extracting it into an execution workspace are different states.

## Make the assignment and instructions explicit

The receiving prompt identifies the source/manifest, actual task, authority,
resources and stop/check-in point, pending decisions, expected return and
receiving owner. A narrow packet states its omissions. A shell agent can locate
its checkout with Git; an extracted packet may have no `.git` directory.
Start at `README.md` and read `AGENTS.md` explicitly when the destination does
not demonstrate automatic instruction loading.

Repository skills live in `.agents/skills`. `.claude/skills` contains relative
discovery links and `CLAUDE.md` imports the project instructions for Claude Code.
Those are source delivery mechanisms, not evidence of automatic loading in
ChatGPT Chat/Work or Claude chat/Cowork. Their tools, installed Skills/plugins
and available file access determine how to read the supplied resources. Follow
the platform reference's [deployment notes](../../.agents/skills/agent-platform-reference/references/skill-deployment.md)
for those distinctions; copying source does not apply account settings.

Use the task's scientific owner, not an exhaustive repository tour.
[Research guide](../../docs/research-guide.md) routes mathematical questions;
topic READMEs identify the current proof, implementation, producer and evidence.
[Thesis README](../../thesis/README.md) owns the selected manuscript and build.
An old handoff or retained producer prompt is provenance unless it is selected
as the current assignment.

## Set up only the selected workload

[Installation workflow](installation-workflow.md) owns the actual dependency
commands. Rust uses the pinned toolchain and lockfile; ordinary Python uses uv
and declared script dependencies; TeX builds the selected manuscript. Sage and
authenticated R2 inputs are conditional on the selected verifier or producer.
A source-reading or writing task can proceed without those installations.

`scripts/bootstrap-cloud.sh` remains the configured Codex universal-image
adapter. It requires that image's x86_64 preinstalled runtimes and privileged
package installation. `--without-r2` removes the private-data setup requirement;
it does not make the script a generic Chat/Work/Claude microVM installer.
For another environment, use the installation workflow and the permitted
runtime-specific installation route. Do not transplant workstation homes,
shell configuration, auth, telemetry or browser state.

[Artifact contracts](../../docs/artifacts.md) own registered inputs, hashes,
cache location and materialization. A cache link created on another machine
does not supply the bytes. Recover the needed input and create links within the
current environment; write new results to a distinct task/run directory.
Inspect expensive or replacing producers before executing them.

## Keep work and decisions recoverable

Record the actual source identity, commands, input identities, results, remaining
work and pending required questions with their owner. A steer or acknowledgment
does not answer an unresolved dependency. Use the effective interaction route.
The project's async-question tool is disabled pending verified delivery; use
direct chat questions. Goal activation, continuation and attention controls
are separate decisions: consult the platform reference's
[goal guidance](../../.agents/skills/agent-platform-reference/references/goal-loops.md)
when relevant. The current cloud-enablement assignment uses ordinary chat
without `/goal`; that does not disable the feature for every future task.

Each session carries its assigned scope, model/effort choice, spending allowance
and check-in point. Shared quota readings, session token counts and billed cost
are different observations. Unknown data remains unknown. For a new cloud
session, decide what state must be returned before its workspace can disappear;
account-saved files or chats do not imply persistent installed dependencies.

[Coordination](../project-management/coordination.md) owns selected assignments
and continuation. A cloud return supplies its own task update for integration;
it must not replace a newer workstation `current.json` wholesale. The integrating
owner preserves other sessions' changes and reconciles the actual handoff.

## Return, assess and clean up

Return changed files or a patch, source/input identity, commands and results,
remaining caveats and obligations, and measured cost where available. Provide
a downloadable artifact or another demonstrated destination-supported route.
A cloud `localhost` link or `/tmp/...` path alone is not a workstation deliverable.
[External-agent communication](../../.agents/skills/external-agent-communication/SKILL.md)
owns channel choice and actual return mechanics; the coordinator verifies receipt,
reviews consequential findings and retains selected results with their source owner.

Prepared messages still follow [message review](../../.agents/skills/message-review/SKILL.md).
Establish the independent review route available to the recipient; if required
review is unavailable, preserve the draft and its pending delivery instead of
inventing an approval. A setup check does not accept a mathematical result or
establish the broader efficiency of the cloud workflow.

Archive disposable trial conversations after receiving their artifacts. Close
agent-created browser tabs and remove unselected scratch outputs when safe;
retain the input/return identity and consequential evidence. If human inspection
is needed, identify the exact chat/artifact and clicks before cleanup removes
the convenient view. No scheduled monitoring or standing cloud workload follows
from one successful handoff.
