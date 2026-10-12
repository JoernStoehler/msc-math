# Retained transfer-study material

[Results](REPORT.md) · [preregistered protocol](PROTOCOL.md) · [exact cases](cases.json) · [criteria](criteria.json) · [frozen input manifest](manifest.json) · [locked scores](locked-scoring-results.json).

The six-case study used the earlier frozen knowledge note, not the revised report. It found no robust incremental decision benefit. Exact participant packets, anonymous answer files and scorer packets are retained. Initial scorer judgments remain alongside corrected judgments; the input-delivery deviation is documented in the report and read-completeness records.

**Later qualification of the delivery checks:** the file named `model-facing-read-checks.json` records matches against serialized native tool output. A subsequent [runtime audit](../../report.html#runtime-live-history-boundary) established that the client can retain full rollout output while truncating the separate history sent to the model. Those byte matches alone therefore do not prove complete effective model input. The original filenames, protocol and result records are preserved; scorer reports and the sizes of corrected reads are separate evidence. This qualification does not by itself invalidate the corrected scores or show that every consumer input was truncated.

Participant and assessor provenance files are explicitly projected: private native-log paths, duplicated answer bodies and complete tool calls are omitted. IDs, source hashes, model/effort, timestamps, counters, call counts, command-read status/hashes and completeness observations remain. Original and published hashes are recorded in [publication-manifest.json](publication-manifest.json). No private reasoning, native logs, encrypted payloads or account data are included.

Run `python3 verify.py` from this directory to check all frozen inputs, response hashes and score aggregation without making model calls or changing files. This validates the retained study records, not behavioral generalization.
