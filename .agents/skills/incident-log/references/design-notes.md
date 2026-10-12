# Incident-log design notes

## Provenance and intended boundary

The project package preserves the installed agent-skills operational guidance. Upstream commit [1ee3326](https://github.com/JoernStoehler/agent-skills/commit/1ee3326cfdebd7cd196b0b4ab09e9e03e76f1795), 30 September 2026, redesigned it for low-friction intake. Its useful boundary is retaining evidence cheaply while leaving diagnosis, taxonomy, evaluation and remediation to later work. Records preserve user explanations and unknowns rather than becoming retroactive proof of a theory.

The default cross-project owner is the agent-skills repository's `incidents/intake/`; an explicit project incident owner overrides it. `/workspaces/agent-skills` is the available owner on this workstation. Other environments may not have it. The 6 October independent review identified the missing runtime fallback; the body now uses an existing durable project evidence location, or explicitly unsaved compact chat intake when none is writable. This is a source repair, not a tested fresh-cloud filesystem contract. The instruction not to list existing files concerns filename allocation, not an authorized evidence/history audit.

## Observations and open choices, 6 October 2026

[One bounded check](../../../../docs/history/incident-log-evaluation-20261006/README.md) succeeded under explicit invocation and an explicit scratch owner: the agent captured a synthetic incident without diagnosis, kept uncertainty, and continued the task. A real intake was retained through the default owner. These observations do not establish reliable automatic selection, useful downstream learning or broad behavior.

Earlier in this session root did not invoke the skill when Jörn identified process failures. Its trigger is user-requested/user-identified capture, not all agent-discovered issues. Whether selective proactive capture would help is an open alternative; costs include duplicate/noisy records, interruption and recording tentative issues as failures. Do not broaden the trigger merely from one missed invocation.

The current mechanics are short and self-contained. Companion memory is maintenance-only and supplies no new intake schema or authority. Preserve a clearly separate evaluation/triage surface instead of adding diagnosis rituals to intake.
