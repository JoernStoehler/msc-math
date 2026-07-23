# Topic Maps

Each file in this folder is a cross-packet map for one topic. A map may record
an active seed, a closed question, or reopen conditions; presence here is not a
launch recommendation. Agents use these files to recover current synthesis,
find source packets, and avoid repeating answered questions.

Each topic owner may choose the file layout that serves its own use case. The
file should say what use it is optimized for and how it is maintained. For
example, an append-only learning log is fine if the topic owner wants low
maintenance cost and accepts higher later reading/reduction cost; a compact
current-belief summary is better if many other sessions need quick orientation.

Topic maps linked from `../next-session-candidates.md` should include a compact
status block. Other topic maps should use this structure when useful:

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
  stale-if:
  allowed-downstream-use:
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
grep. If a linked topic omits the block, the next topic-owner or steward pass
should add it before treating the file as launch-ready. For ready prompts, state
whether the prompt is `executor-ready`, `reviewer-ready`, `topic-owner-ready`,
`audit-ready`, or `workflow-test-only`.

When a topic matures into its own experiment folder or method packet, keep this
map as the routing surface and link the new owner-local files.

The current seed files reflect the current random/product-heavy evidence base.
Surface scouts and topic owners may add topics outside that distributional
frame when they are relevant to the broader thesis datascience slice.
