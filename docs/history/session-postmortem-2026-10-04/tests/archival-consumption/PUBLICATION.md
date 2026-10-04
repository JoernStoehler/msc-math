# Retained archival-consumption study

[Results](analysis.md) · [preregistered design](design.md) · [protocol deviations and limits](protocol-notes.md) · [cases](cases.json) · [criteria](criteria.json) · [locked scores](locked-scoring-results.json).

The study did not demonstrate a net reconstruction-cost saving. All eight answers were adequate, all four packet consumers took longer, and aggregate source reading rose 7.3%. Small qualification gains do not establish a general benefit; the task 3 difference includes an additional historical fact unavailable to its archive-only consumer. The baseline itself had already been curated, so these results do not measure finding and reconstructing the incidents from unselected raw logs.

The study owner's [publication manifest](PUBLICATION-MANIFEST.json) hashes the exact retained protocol, source inventory, input configurations, canonical scoring exhibits, anonymous answers, initial/corrected scores and projected read/usage provenance. Complete private source bundles, native logs and full read receipts remain private. No rerun or new model call is needed to inspect these records.

The immutable [candidate manifest](packet/MANIFEST.json), [report](packet/REPORT.md), [knowledge note](packet/knowledge.md), [evidence](packet/evidence.json) and [output map](packet/OUTPUTS.md) preserve the tested 00:34 UTC packet. Its original HTML is stored as `packet/report.html.gz`; decompression reproduces the original hash. Relative links inside these exact archival files reflect their original location and are not rewritten to pretend they are the current package. Use the [final report](../../report.html) for current navigation.

Final production incorporates the study results, the later runtime delivery audit and historical receipt checks. It also corrects a knowledge route, adds the pre-freeze Fix probe receipt that exposed unequal information in task 3, trims two dangling excerpt tails, and improves mobile navigation and citation scrolling. A later path-only integration makes historical output routes explicit while copying this completed packet to canonical main; the underlying historical artifacts and frozen study inputs are unchanged. The [final-difference manifest](final-packet-differences.json) records tested and final hashes. Those revisions are not a tested improvement. The frozen packet was never replaced with final production bytes.

Run from the repository root:

```bash
python3 docs/history/session-postmortem-2026-10-04/tests/archival-consumption/verify.py
```

The verifier reads files only. It checks retained hashes, the compressed candidate HTML, original frozen inputs present in this public subset, answer/score identities, source-word accounting and descriptive aggregation. It does not independently inspect the omitted private sources, validate the rubric, establish the recipient's complete effective input or prove future savings. Runtime-source and byte-budget evidence address a narrower delivery question separately.
