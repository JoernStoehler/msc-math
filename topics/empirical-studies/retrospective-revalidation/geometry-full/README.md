# Full retrospective geometry/schema migration

**Completed 2026-09-18: all 14,336 rows rebuilt in 586.33 seconds on four threads. All 645,120 overlapping cells agree exactly with the historical table; six feature columns added, none removed.** Exact population ID equality and uniqueness passed. This closes full-population geometry/schema restoration, with historical volume normalization retained.

This run rebuilds geometric descriptors for the 14,336 recovered historical bodies, independently of current-capacity revalidation. The existing producer preserves historical sys and uses historical cached volume for normalization. It does not evaluate capacity or recompute volume, and no execution/certification metadata is invented.

`run.py` verifies the exact pilot executable/source identity and both registered source hashes, then executes with four Rayon threads and a 25-minute cap. Status, input/output hashes and actual timings are in `run-receipt.json`; producer diagnostics in `run.stderr`. The producer writes tables only after constructing every row: interruption requires a full restart into a fresh directory, not a resumed claim.

`compare.py /path/to/historical/polytope-table.jsonl` verifies the pinned historical table hash, complete unique 14,336-ID equality, and every overlapping cell. Discrepancies are retained, never silently overwritten or treated as assertions that abort the audit. No claim extends to new sampled bodies or current target certification.

Full output bytes are retained as reproducible gzip streams (mtime zero); raw run outputs remain in ignored `output/`. Restore with `gzip -dc polytope-table.jsonl.gz > /fresh/path/polytope-table.jsonl` and the analogous provenance command. `archive-receipt.json` records compressed and uncompressed hashes. `comparison.json` records schema differences and all observed mismatches.
