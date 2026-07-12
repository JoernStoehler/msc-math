# Response to independent review

Date: 2026-07-12

All required repairs in `REVIEW.md` were applied to this scratch packet only.

1. `REPORT.md` now identifies the bootstrap target separately for retained
   re-ranking, the 1M equal-bucket aggregate, the pooled cross-run aggregate,
   and cross-run per-bucket target-value resampling.  It no longer presents the
   aggregate bucket bootstraps as target-row sampling intervals.
2. The q=.01 versus q=.0001 discussion is narrowed to a run-specific,
   hypothesis-generating sign pattern.  It makes no claim to discriminate
   against universal monotonicity or establish reversal/saturation.
3. `metadata.json` now records frozen 1M and 100k stage-1 selection-plan paths,
   SHA-256 hashes, and concise pre-target selection semantics.  It also records
   the 1M selected-before-target cache hash.
4. `python3 analyze.py --out-dir /tmp/ridge-empirics-review-repair-rerun` was
   rerun after the code/provenance repair.  All generated TSVs were
   byte-identical to this packet's TSVs.  `metadata.json` is intentionally not
   byte-identical to the prior reviewed version because selection-plan and
   selected-before-target provenance fields were added; its content is
   byte-identical across repaired reruns.

No repository-tracked file was changed.
