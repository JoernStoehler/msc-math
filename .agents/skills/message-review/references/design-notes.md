# Message review design notes

On 8 October 2026 Jörn proposed making the communication guidance actionable by requiring prepared, non-synchronous messages to identify a separate subagent that approved their exact text, with a prepared checklist/workflow for non-mechanical violations. He authorized making the guidance actionable; this package implements that local request without changing separately governed Codex configuration.

The immediate incident: the audit root reviewed an artifact's factual claims but delivered an activity-oriented chat answer, then offered corrections to an assessment Jörn had not accepted. The gate reviews the outgoing text in the actual request/steering context, not its report alone. The rejected audit remains evidence, not user acceptance. Its private explanatory claims should not be promoted into proven causal diagnoses.

Exact text includes the reviewer attribution. Unicode NFC, CRLF and whitespace variation are accepted; punctuation/word changes require renewed review. Context hashes and relevant-steering invalidation prevent reuse after the decision changes. Coordinator cross-check against native tool identity is required because a JSON receipt or Python script cannot authenticate its author.

Synchronous conversation is exempt to preserve useful direct interaction. It is defined by the interaction and deliverable, not by the channel or presence of a latest user question. Reports/progress messages remain covered. Misclassification that exempts reports is a known risk; so is unnecessary review latency for actual dialogue. Native private reviewer returns are exempt to avoid infinite recursion.

The initial implementation is an instruction gate and portable verifier. Pre-delivery runtime interception, reviewer quality, reliable selection/application and walltime benefit need separate evidence. Stop hooks after streaming cannot establish pre-delivery enforcement. No host/project config edit or global rollout is implied. Evaluate exact-text mechanics separately from whether reviewers catch relevance, attention, scope and continuation defects.
