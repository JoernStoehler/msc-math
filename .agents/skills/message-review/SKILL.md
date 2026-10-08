---
name: message-review
description: "Review the exact text of prepared user-visible msc-math messages before delivery. Use for reports, recommendations, handoffs, execution summaries and progress updates. Direct synchronous discussion and private native subagent returns are exempt."
---

# Message review

Sender and reviewer use this same standard. Review whether the exact proposed message is fit to send at this point in the actual conversation. Do not assume a settled request, goal or deliverable; examine the sender's interpretation rather than adopting it.

## Standard

Use relevance, reasoning, evidence, clarity, user effort and follow-through as lenses where useful. Derive specific criteria from the conversation and applicable instructions. Consider what the message contributes, presupposes, asks of Jörn or implies for subsequent interaction/work. Distinguish established facts and choices from proposals, interpretations and uncertainty.

Judge the whole message. Exploration need not complete a task, brevity alone is not success, and a source artifact's correctness does not establish that the outgoing message is appropriate. Do not make Jörn reconstruct avoidable missing context or repair work the agents can do. Respect actual authority and accepted choices without inventing preferences, acceptance or causes.

## Use

- **Scope:** Prepared reports, recommendations, handoffs, execution summaries and progress updates require review. Immediate synchronous discussion is exempt when it is direct joint conversation rather than delivery of a prepared work result. A report does not become exempt merely by answering the latest question. Private native agent messages are exempt. On uncertainty, use review.
- **Sender:** Freeze a candidate version including `[review: REVIEWER]`. Give a separate reviewer this skill, the complete message (inline or file), and enough actual conversation, instructions and evidence to judge it. Retain pertinent verbatim turns; do not substitute a summary that assumes your framing. Prefer a fresh context over inheriting unrelated history. Supply sufficient inputs inline when useful to avoid retrieval rounds.
- **Reviewer:** Act as a read-only reviewer: inspect necessary inputs, make no project edits or execution changes, and return your judgment. If consequential context is missing, ask the sender for it or retrieve the specific needed material. Do not rerun already-passed tests without a concrete reason. Do not recursively review your private return.
- **Return:** Say `APPROVE <candidate-version>` or `REJECT <candidate-version>`. Include consequential findings, actionable corrections and important uncertainty. No schema, hashes, fixed checklist fields or repeated candidate text. Approve only with no unresolved consequential objection; perfection and your preferred style are not requirements. Proposed replacement text needs its own review.
- **Delivery:** The sender checks the actual native reviewer identity, verdict and candidate version, then sends that complete text unchanged. Substantive changes or relevant new user steering require renewed review. The sender owns revision, action and delivery; the reviewer cannot close unrelated unfinished obligations. If required review is unavailable, keep prepared delivery pending and state the limitation in direct conversation.

Keep frozen candidates, supplied context and native review returns together under `docs/coordination/message-reviews/<date>/<packet>/`, preserving revisions separately. Sender storage does not authorize fabricating the reviewer's approval.

## Limits

This is an instruction gate, not a platform delivery block or proof of reliable review. Read-only describes the reviewer's operations; sandbox confinement depends on the actual runtime. Sufficient inline context permits a no-tool response; file/log/repo retrieval adds rounds. Cost and judgment quality remain matters for observation, not guarantees from this text.

For maintenance rationale and retained evidence, see [design notes](references/design-notes.md).
