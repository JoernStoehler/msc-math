# Ridge-Sum Tail Diagnostic

Status: exploration artifact, not thesis-ready.

Question: does low volume-normalized ridge symplectic-area sum track high `sys`, and what happens in the generated-candidate lower tail?

Inputs:
- retained table: `/workspaces/msc-math/.worktrees/thesis-datascience-integration/experiments/sys-datascience/prepare`
- 1M ridge-sum run: `/tmp/sys-ds-extreme-scalar-rejection-proposer-1m-ridge-sum`
- HKO reference prepare: `/tmp/ds-prepare-hko`

Generated plots:
- `sys_vs_ridge_by_bucket.png`: exact product-bucket panels; blue bins are old retained rows from that bucket, red/black points are 1M per-bucket top-10 selected/baseline rows.
- `sys_vs_ridge_cloud.png`: pooled diagnostic; blue density is the old retained random-product sys-evaluated table and red/black overlays are the separate 1M generated-product run.
- `ridge_marginal_1m.png`: 1M generated-candidate ridge-sum marginal with the pooled top-100 cutoff.
- `role-summary.tsv`: compact selected/baseline `sys` summary for the pooled top-100 and per-bucket top-10 selections.
- `source-selection-summary.tsv` and `source-evaluation-report.json`: copied compact reports from the source 1M run, when present.

Interpretation:
- Within product buckets, low ridge-area sum is strongly associated with high `sys` in the old retained table.
- The 1M per-bucket top-10 selected rows are enriched toward the high-`sys` side of their bucket, so the scalar is a real generated-candidate proposer signal.
- Extreme filtering does not extrapolate monotonically toward `sys > 1`; the selected lower-ridge tail is enriched but not near-counterexample-producing in this run.
- The pooled top-100 diagnostic is mainly a support check. Product buckets have different ridge/sys scales, so the per-bucket grid is the main comparison.
- Each per-bucket panel uses its own ridge-axis zoom; the old retained high-ridge tail is clipped so the selected/baseline comparison is readable.

Support check: old retained product rows at or below the pooled top-100 cutoff: 0.

HKO marker: ridge=8.944271909999161, sys=1.0472135954999569.

Global low-ridge cutoff:
- `global_low_sum_top_100`: ridge=8.917642700838966

Per-bucket low-ridge cutoffs:
- `3x3`: ridge=12.235881326478664
- `3x4`: ridge=11.420757225635963
- `3x5`: ridge=10.999225180004208
- `3x6`: ridge=10.714510249814834
- `4x4`: ridge=9.329640964460278
- `4x5`: ridge=9.143659508481042
- `4x6`: ridge=9.069654132586711
- `5x5`: ridge=8.882699960071957
- `5x6`: ridge=8.812908484793466
- `6x6`: ridge=8.781408639955293
