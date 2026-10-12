# Design notes

Combines the project cross-session messaging invariants and complete ChatGPT Pro round trip. Preserves recipient/action/reply semantics, incoming-message interpretation and requested receipts, transfer authority, exact successful human prompt, Taildrop discovery and fresh receive directory, unrelated arrivals and independent integration. Adds route comparison and sufficient assignment context. Native subagent transport is excluded.

See [analysis and evidence](../../../../docs/knowledge/collaboration-guidance-design-2026-10-05.md). Current runtime content lives in one sectioned `SKILL.md`: technical usage/contracts first, followed by conditional work-choice/assignment guidance and integration. Design memory alone stays separate because its consumer is the maintainer. No useful-behavior or near-optimality claim has been established.

## Trigger rationale and review

The abstract condition is exchange across independent agent contexts; Codex, Claude Code, Pro and human relay are recognition examples. Assignment quality can matter inside a native subagent tree without requiring another transport protocol.

Open review question: is this boundary clear enough to cover real independent exchanges without duplicating native transport?

## Usage documentation and decision scope, 5 October 2026

Jörn identified two design problems. An unscoped instruction to classify an exchange has no demonstrated use, can misrepresent multipart messages and might produce a provisional label later reused as settled context. A technical channel section should document current tools/scripts from a usage perspective, foregrounding the path that works best in its applicable environment rather than presenting equivalent competing routes. These are accepted design corrections; the predicted classification failure is not an observed behavioral test.

The body now presents technical usage/contracts first, then conditional work-choice/assignment guidance and integration. The inline technical section gives one preferred route per operation/environment and the addressing, formatting, return and delivery contracts. Shell queueing is a fallback when direct thread tools are unavailable. A file is material storage, not notification; predictable reply channel/destination is part of the technical contract. Native metadata can already supply it. No fixed message taxonomy or mandatory message header was introduced.

Checked exposed tool schemas and installed CLI help without sending messages or starting work. Observed `codex_tui.create_thread` rather than `spawn_session`, its explicit-new-task requirement, a 1,000-byte prompt limit, `codex queue` syntax and native `collaboration.*` semantics. `codex_app.*` was not exposed and a dedicated Claude↔Codex bridge was not verified; their recipes are not invented. The complete Pro recipe is inline with the other technical usage subsections. Technical coverage of native subagents supplies neighboring usage knowledge without broadening the trigger or adding another tree protocol. No forward trial establishes better selection or behavior.

## Current layout rationale, 6 October 2026

Jörn challenged the runtime-reference split: its review had checked technical correctness, not the cost of extra reads, missed hops or coordinated maintenance. He advised a complete related skill first, with section decomposition independently useful; establish useful behavior before cost-driven workflow/file splitting. Runtime channel and Pro references were therefore folded inline without dropping their content. Workflow-specific skill splitting remains an alternative for later comparison against a useful baseline. This choice accepts fuller initial loading; it is not evidence that extra context is free or that one file is optimal.
