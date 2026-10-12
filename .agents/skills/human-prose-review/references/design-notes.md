# Design notes

## Human review evidence, 6 October 2026

Frozen artifacts and raw annotations belong with the manuscript/experiment owner. The skill deliberately separates Jörn's judgments from agent interpretation, withholds agent reviews for independent comparisons, and does not treat unmarked text as approval. A hash records identity; it does not prevent a later edit.

The [September trial](../../../../docs/history/reviewer-trial/inputs/development-labels.md#api-retrieval-trial) matched all twelve Markdown annotations against the pasted export and retrieved one selected PDF comment after Jörn confirmed annotation worked. That is meaningful route evidence on the tested device, not all-device coverage or a current end-to-end check.

Offline pre-sweep probes cover pagination boundaries, scoped queries and duplicate-ID refusal. They do not test today's embedded viewer, authentication, selection quality or export menu. Host coordination now uses an available channel rather than assuming Herdr. See the [pre-sweep assessment](../../../../docs/history/skill-presweep-20261006/README.md).
