---
name: message-review
description: "Review the exact text of prepared user-visible msc-math messages before delivery. Use for reports, recommendations, handoffs, execution summaries and progress updates. Direct synchronous discussion and private native subagent returns are exempt."
---

# Exact-text message review

Prevent Jörn having to reconstruct the answer, manage agent follow-through, or discover unsupported claims. A named separate reviewer approves the actual outgoing message, not merely its source artifact. This is a selected project gate, not a claim of proven behavioral repair.

## Coordinator: prepare the review

1. Write the complete candidate to a UTF-8 file, including `[review: REVIEWER]`. Use the actual canonical native subagent name or UUID returned by the runtime. Nothing user-visible is appended after approval. Only whitespace and Unicode normalization that leave the normalized text identical are permitted. Markdown punctuation and emphasis changes require renewed review.
2. Prepare a context file containing the actual user request and relevant subsequent steering, the selected objective, governing decisions/authority, unresolved objections and necessary evidence. Preserve the user's wording. State factual uncertainties; do not instruct the reviewer to endorse the preferred framing. For corrections, include what the user rejected and whether they accepted any earlier artifact. A change in relevant user steering invalidates prior approval.
3. Give a separate native subagent this workflow and the two files. Prefer a fresh context for a consequential disputed framing. Reuse a reviewer for routine updates when it has the necessary context. Assign review only; the coordinator owns implementation and integration. The reviewer may make bounded read-only checks but does not restart the underlying work or spawn recursive message reviewers.
4. A rejection means revise and resubmit the complete message, or do the missing work before drafting. Do not send the rejected text with disclaimers, a review promise or amendments that make Jörn assemble a replacement. If approval would require a real user decision, the reviewer identifies the exact decision and why; it does not invent permission requirements.

## Reviewer: check the message in its actual context

Read the context and complete candidate before forming a verdict. Judge usefulness and consequential defects, not compliance phrases, a preferred prose style, minimum length or whether every checklist topic appears in the answer. Obtain enough necessary evidence to judge claims; reject unsupported claims instead of certifying unavailable evidence.

For each of these five checks, record `PASS` or `FAIL` with a short reason specific to the candidate:

- **answer_relevance:** Does it answer the actual request at the right scope, with the main point apparent early? Are activities/component checks being substituted for the requested assessment or outcome? Is a rejected report being treated as accepted? Does a correction provide a coherent replacement when needed?
- **user_effort:** Can Jörn understand and use it without reconstructing scattered context, navigating an unnecessary report, supervising remaining work or diagnosing an agent-resolvable uncertainty? Does any question have the concrete object, context and purpose needed, and require his knowledge/judgment/authority? Are timing and available attention respected?
- **evidence_scope:** Are facts, judgments, proposals and uncertainty distinguished at the strength their evidence supports? Does a compile, source review, synthetic test or approval get promoted into a broader readiness claim? Are self-explanations or inferred causes presented as established facts?
- **continuation:** Does a final/end-of-work message legitimately complete the selected objective or present a genuine required input? If work remains authorized and feasible, return the unfinished obligation and concrete next operation instead of approving a component stop. For progress messages, check that updates do not imply false closure. Criticism, corrections and acknowledgments do not by themselves authorize stopping.
- **communication:** Is the message organized around what Jörn needs to know or decide? Remove activity narration, unsupported assurances, repetition, unnecessary apology/self-commentary and avoidable retrieval burden. Respect his requested detail and format; useful context is allowed and brevity alone is not success.

Reject on any consequential failure. Explain the defect and an actionable correction, rather than rewrite only the latest complained-about dimension. When a check is not relevant, record `PASS` and why. Do not require irrelevant content to be added to satisfy the checklist.

## Reviewer: return and retain an approval

Only the reviewer writes the receipt. It contains:

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

Use the repository helper's `normalize_text` and `text_sha256` functions to compute hashes. Receipts and associated text/context files belong in `docs/coordination/message-reviews/<date>/<packet>/`, with each revision named separately; do not overwrite earlier rejected/approved versions. These files are review evidence, not a competing assignment store. Keep credentials and irrelevant private history out of packets.

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
