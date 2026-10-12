# Proposed cleanup execution graph

`review.html` is the human review surface; `work.dot` and `team.dot` own the two
manual component graphs. `packages.json` retains source-backed proposed work
seeds; the HTML's E1–3/P/A account and writing-plan blocks incorporate later
Q13 steering. These are proposals, not live worker assignments. Selected work
and authorization remain in [current coordination](../../../../coordination/current.json).

Render DOT with Graphviz and replace the corresponding embedded SVG in the
standalone review HTML after edits. The HTML embeds SVG to work through the
narrow review-file server. Root owns synchronization; check the task/delegation
meaning as well as successful rendering. No new server or automation is needed.

The diagrams have been visually inspected as Graphviz PNGs. Full browser layout
is unverified because no browser connection was available in this session.
The final Astra review was in a disposable message packet removed during the
12 October retention migration; its recovery manifest is described in
[message-review retention](../../../../../meta-topics/agent-workflow/message-review-retention.md).

On 11 October Jörn considered the package worth running and requested early
crux checks. `early-checks.md` and the EP branch in `work.dot` move a complete
writing-value slice ahead of bulk account authoring and expose local migration
and integration gates. These are proposed probes; only the two named cache
verifications were performed. The original Astra planning review predates this
addition; the new user-facing explanation has separate Sol message review.
