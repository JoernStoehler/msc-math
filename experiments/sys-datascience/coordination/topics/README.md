# Topic Maps

Each file in this folder is an active research ledger for one topic seed or
topic-owner session. Agents read these files to recover a topic owner's current
best guesses, understand the evidence and methodology behind those guesses,
borrow ideas for other topics, and write or review packet prompts.

Each topic owner may choose the file layout that serves its own use case. The
file should say what use it is optimized for and how it is maintained. For
example, an append-only learning log is fine if the topic owner wants low
maintenance cost and accepts higher later reading/reduction cost; a compact
current-belief summary is better if many other sessions need quick orientation.

Topic maps should use this structure when useful:

```text
Use / maintenance model:
Scope:
Status block:
  topic-status:
  spawn-status:
  next-role:
  next-action:
  review-gate:
  belief-update-owner:
  last-reviewed:
  source-of-truth:
Current belief:
Evidence sources:
Adjacent topics to read:
Candidate hypotheses:
Cheap discriminators:
Packet ideas:
Ready packet prompts:
Needs topic-owner sharpening:
Opportunity-cost notes:
Owner-readiness/status:
```

Use the status block when a topic may be read by surface scouts or fresh topic
owners. Keep prose flexible, but keep status values short enough to compare and
grep. For ready prompts, state whether the prompt is `executor-ready`,
`reviewer-ready`, `topic-owner-ready`, `audit-ready`, or `workflow-test-only`.

When a topic matures into its own experiment folder or method packet, keep this
map as the routing surface and link the new owner-local files.

The current seed files reflect the current random/product-heavy evidence base.
Surface scouts and topic owners may add topics outside that distributional
frame when they are relevant to the broader thesis datascience slice.
