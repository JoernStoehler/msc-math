# Ridge-Concentration Validation Review

Review date: 2026-07-11.

Verdict: technically and interpretively sufficient to trigger the repository's
credible independently validated proposer escalation. The validated object is
a frozen sub-threshold enrichment rule for one random-product generator, not a
rule demonstrated to find `sys > 1`.

## Technical And Provenance Review

- The config and README froze the two-stage rule, new seed, sample size,
  comparator, and descriptive pass criterion before target evaluation.
- Geometry, feature, and selection stages ran before `sys`. The target-field
  audit passed on all three pre-target JSONL files; their identities are in
  `pre-target-artifact-sha256.txt` and the generated evaluation report.
- The cascade is a proper subset of stage 1. Each bucket contains the expected
  stage-1, cascade, complement, and matched-baseline counts. The cascade and
  stage-1 baselines are disjoint and bucket-matched.
- Selection and evaluation artifacts have the same 2,490 unique candidate IDs;
  identity, bucket, membership, and copied pre-target fields agree.
- An independent aggregation reproduced the generated verdict. The analyzer
  was repaired after review to reject duplicate IDs, restrict reusable caches
  to the current selection, and check identity/membership agreement; rerunning
  it left the result unchanged.
- Six focused Rust tests pass. The skill validator and `git diff --check` also
  pass in the integration worktree.

The large geometry and feature caches are deterministic regeneration inputs,
not required to interpret the compact result. They are omitted from the final
tracked packet after recording hashes and the audit result.

## Interpretation Review

The frozen descriptive criterion passed. The allowed conclusion is that, after
selecting the lowest 1% per product bucket by normalized total ridge area, the
lower half by ridge-area maximum share provided incremental pre-`sys`
enrichment over the complementary half on a fresh generated sample. Detailed
effects and bucket heterogeneity remain in `incremental-validation.tsv` and
`validation-verdict.json`.

No evaluated row exceeded 1. The packet does not establish a counterexample,
near-counterexample, geometric mechanism, statistical significance, calibrated
hit rate, monotone extrapolation, arbitrary-generator transfer, or thesis-level
method/data closure.

The result creates a terminology crux. It supersedes “no
validated proposer” if that phrase includes validated sub-threshold enrichment.
It does not supersede “no proposer for finding `sys > 1`” if that phrase is
reserved for evidence bearing on reaching or credibly extrapolating to the
threshold. Jörn should choose that vocabulary, but the separate additional-data
and research-routing questions are assessed in
`../../../../coordination/research-direction-review-2026-07-11.md`.

## Review Architecture

Technical/provenance and domain interpretation were reviewed separately because
either could have failed independently. The parent repaired the one medium
analyzer robustness finding and preserved the lower-severity hash-binding
evidence. No further reviewer was added after those checks converged.
