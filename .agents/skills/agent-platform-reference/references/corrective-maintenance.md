# Diagnose once, correct the maintained owner

This reference is for a failure review or maintenance task. It does not require every agent task to perform a platform audit. The desired improvement is cheaper successful future work after a failure exposed a missing fact or faulty harness choice.

## Establish the mechanism

Recover the trigger, agent action, failing command/output or interaction, effective client version/configuration and relevant user cost. Preserve the difference between reported observation, source explanation and runtime reproduction. Read the nearest product reference and inspect the effective source/configuration when behavior is contested. A plausible diagnosis is a hypothesis until the implicated mechanism is established; do not attribute every failure to the model, instructions or tools without distinguishing them.

## Choose the owner by what must change

| Established problem | Candidate maintained owner | Why / limitation |
| --- | --- | --- |
| Repository command or cache path assumes the old workstation | Script/build config plus repository contract | A warning in a global prompt does not repair an executable absolute path |
| Destination doesn't receive repo rules or personal skills | Repo-contained resources, explicit handoff prompt; client-specific discovery bridge where supported | Cowork is not Claude Code; don't assume the same auto-loading |
| Missing platform fact needed only in rare tasks | This reference or a conditional specialized skill | Avoid broad unconditional guidance when the fact has a narrow consumer |
| Agent routinely invokes a rare feature without knowing its constraints | Short AGENTS.md pointer to relevant feature guidance | Changes discovery; doesn't change feature mechanics or higher-priority tool policy |
| Misleading or incomplete tool description | Supported tool/catalog description configuration, after inspecting effective schema/merge | Verify the exact tool is overridable; restart/reload as documented; don't assume this changes code enforcement |
| Notification/queue lifecycle loses required questions | Agent question-state procedure and, where necessary, harness pause/notification fix | A behavior instruction cannot guarantee a UI event or change runtime continuation |

Compare materially different fixes on user effort, invocation/context cost, reliability, maintenance and scope. Prefer the closest maintained owner that can actually affect the failure. Generic process wording is insufficient for a broken script or runtime gate. A source-level limitation may require a product/harness change rather than a larger prompt.

## Produce a reviewable correction

Provide the exact file/configuration change or specialized-skill draft, its trigger and the regression it is intended to prevent. Preserve unrelated guidance and authority. Activate/push only within authorization. Test both the observed failure and an adjacent case that prevents overcorrection: e.g. an optional question should not stop unrelated work, and a plain “Goal:” heading should not activate the goal feature.

## Evidence needed to claim improvement

Show a representative downstream task, the point at which the skill supplied information, the resulting changed action/artifact and whether the task could continue. Separate:

- consultation/diagnosis worked;
- the maintained change is present and reaches the next consumer;
- fixture execution passed;
- live client behavior was independently verified;
- user interventions/time were actually measured.

A disclosed-case repair is not a held-out result. Keep negative evidence and unsupported activation claims in the review. This package's `REVIEW.md` records demonstrations and their limits; withheld user cases remain independent acceptance tests.
