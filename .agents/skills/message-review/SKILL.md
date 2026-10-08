---
name: message-review
description: "Review the exact text of prepared user-visible msc-math messages before delivery. Use for reports, recommendations, handoffs, execution summaries and progress updates. Direct synchronous discussion and private native subagent returns are exempt."
---

# Exact-text message review

Prevent Jörn having to reconstruct the answer, manage agent follow-through, or discover unsupported claims. A named separate reviewer approves the actual outgoing message, not merely its source artifact. This is a selected project gate, not a claim of proven behavioral repair.

## Coordinator: prepare the review

1. Write the complete candidate to a UTF-8 file, including `[review: REVIEWER]`. Use the actual canonical native subagent name or UUID returned by the runtime. Nothing user-visible is appended after approval. Only whitespace and Unicode normalization that leave the normalized text identical are permitted. Markdown punctuation and emphasis changes require renewed review.
2. Prepare a context file containing the relevant conversation in the user's wording, governing instructions/decisions, uncertainties or disagreements and necessary evidence. Include goals, requests, authority or acceptance only insofar as they actually exist; distinguish them from the sender's interpretation. Do not replace the conversation with a summary that assumes the sender's framing. For corrections, include what the user rejected and whether they accepted any earlier artifact. A change in relevant user steering invalidates prior approval.
3. Default to a separate native subagent with no inherited conversation (`fork_turns="none"`). Supply this prompt, the exact candidate, relevant verbatim conversation excerpts and necessary instructions/evidence inline, including already-computed text/context hashes. Request one response with no tools when that packet is sufficient. Reuse a reviewer only when retained context is useful and up to date; do not inherit the whole root merely for convenience. Assign review only; the coordinator owns implementation and integration. If missing context could change the judgment, obtain the particular additional context before approval; bounded reviewer retrieval is allowed when useful. Do not restart the underlying work or spawn recursive message reviewers.
4. A rejection means revise and resubmit the complete message, or do the missing work before drafting. Do not send the rejected text with disclaimers, a review promise or amendments that make Jörn assemble a replacement. If approval would require a real user decision, the reviewer identifies the exact decision and why; it does not invent permission requirements.

## Reviewer prompt template

Pass the following prompt with paths to the actual conversation/context, candidate message and intended receipt. This is the substantive review workflow; the receipt mechanics below retain exact-text accountability.

> You are reviewing a message before it is sent to Jörn. Decide whether this exact text is fit to send at this point in the actual conversation.
>
> Read the supplied conversation, relevant instructions and candidate. Establish what is understood, uncertain or disputed, and what, if anything, has been requested, agreed or authorized. Do not assume a settled task, goal, deliverable or desired conclusion. Examine the sender's interpretation rather than adopting it.
>
> Examine the message's claims, assumptions, questions, proposals and implications for subsequent interaction or work. Use relevance, reasoning, evidence, clarity, user effort and follow-through as lenses where useful; derive specific criteria from this situation. Exploration need not complete a task, and brevity alone is not success.
>
> For each consequential concern, identify the wording and relevant context, explain the likely consequence, and give an actionable correction. Review the whole message. If more agent work or context is needed, say specifically what; do not offload that work to Jörn. Do not invent preferences, acceptance, authority or psychological causes.
>
> Return APPROVE or REJECT for this exact candidate, with your consequential findings, required next action and any important uncertainty in your interpretation. Approval requires no unresolved consequential objection, not perfection or your preferred style. Replacement wording needs its own review.
>
> You own the review; the sender owns revision, action and delivery. Return your authored receipt under the supplied exact-text/context protocol. Review the outgoing message, not merely its source artifact.

The receipt retains five legacy identifiers: `answer_relevance`, `user_effort`, `evidence_scope`, `continuation`, `communication`. They are accountability fields, not definitions of ordinary words or a requirement that the conversation contain a request or task. For each, record `PASS` or `FAIL` and a candidate-specific reason; record non-applicability explicitly as `PASS` with its reason. Any consequential finding must affect the verdict, including one that does not fit those identifiers. A `PASS` on an inapplicable field does not establish completion, acceptance or authority. Do not add irrelevant content merely to populate the receipt.

## Reviewer: return and retain an approval

Only the reviewer authors the receipt. The sender may store a reviewer-returned JSON receipt verbatim, after checking the native return's identity, or the reviewer may write it through a tool when needed. It contains:

```json
{
  "schema_version": 1,
  "verdict": "APPROVE",
  "reviewer": "/root/message_review",
  "reviewed_at": "2026-10-08T12:00:00Z",
  "message_sha256": "SHA256_OF_NORMALIZED_TEXT",
  "context_sha256": "SHA256_OF_NORMALIZED_CONTEXT",
  "approved_text": "COMPLETE_MESSAGE_INCLUDING_[review: /root/message_review]",
  "checks": {
    "answer_relevance": {"status": "PASS", "reason": "Candidate-specific reason"},
    "user_effort": {"status": "PASS", "reason": "Candidate-specific reason"},
    "evidence_scope": {"status": "PASS", "reason": "Candidate-specific reason"},
    "continuation": {"status": "PASS", "reason": "Candidate-specific reason"},
    "communication": {"status": "PASS", "reason": "Candidate-specific reason"}
  }
}
```

The sender can supply hashes computed with the repository helper's `normalize_text` and `text_sha256` functions; the pre-send checker independently verifies them against the returned complete text and context. Do not ask an LLM to calculate a cryptographic hash by hand. Receipts and associated text/context files belong in `docs/coordination/message-reviews/<date>/<packet>/`, with each revision named separately; do not overwrite earlier rejected/approved versions. These files are review evidence, not a competing assignment store. Keep credentials and irrelevant private history out of packets.

## Context and review cost

A sufficient inline packet permits one response without tools; native runtime internals and cost are not guaranteed by this instruction. More context increases input cost; retrieval adds tool/inference rounds. Forking a fixed number of messages can omit decisive earlier steering and can include irrelevant material. A summary can help orientation, but retain the pertinent verbatim turns so the reviewer can question the sender's interpretation. Session-log pointers require retrieval and are recovery routes, not the default input. Include repo evidence only when needed to assess a consequential claim; do not rerun passed tests or re-prove mathematics merely because a message mentions them. Report packet size, reviewer tool rounds and observed latency when assessing cost; do not infer dollars from token counts or advertise measured savings without a comparison.

A `REJECT` receipt uses the same fields with failing checks and the inspected candidate in `approved_text`; the verdict makes it ineligible for delivery. Return the verdict, identity, receipt path and concrete corrections to the coordinator through native transport. Native returns do not themselves require another reviewer.

## Coordinator: verify and send

Cross-check the receipt's reviewer against the actual subagent tool identity and its returned verdict; JSON alone cannot authenticate an agent. Then run:

```bash
python3 scripts/check-message-review.py MESSAGE_FILE RECEIPT_FILE \
  --sender ACTUAL_SENDER_NAME --context-file CONTEXT_FILE
```

Send the approved text only after that check passes. Re-review after substantive edits or relevant new user input. The visible attribution points to the separate reviewer; the retained receipt binds it to the exact message and context. A successful check does not authorize unfinished work to stop or override Jörn's authority.

If a platform cannot supply native subagents, prepared delivery remains pending. State the capability limitation in direct chat without exposing the rejected report. Do not invent a substitute self-approval or quietly remove attribution.

## Enforcement boundary

This package supplies an active instruction gate, a named independent decision, and deterministic receipt/text checks. It does not install a platform send interceptor. A sender could still violate instructions and bypass the checker. A reviewer can also share blind spots or approve poor text. Assess those failures separately from exact-text binding and discovery; do not label the workflow reliable merely because tests pass.

For maintenance, read [design notes](references/design-notes.md).
