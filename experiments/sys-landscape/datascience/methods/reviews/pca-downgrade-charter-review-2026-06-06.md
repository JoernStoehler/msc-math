# PCA Downgrade Charter Review

Date: 2026-06-06

Reviewed commit: `0fc3e4c9`

Review scope: check whether `pca-projection` satisfies the "Honest Downgrade"
terminal state from `/tmp/ds_pca_goal_charter_2026-06-06.md`.

## Findings

No blocking issues were found for the honest-downgrade state.

The PCA row is not repaired current method-table evidence. It is partial/status
evidence with reproducible artifacts, an open method-level interpretation, no
current candidate-proposer, and no validated new row.

## Trace

The review checked that the report states which partial evidence exists:

- retained-dataset PCA run;
- PC2-high audit;
- PC2-high captures `20/85` top-1% rows versus `4.26` expected;
- no current candidate-proposer;
- no validated new row.

The review checked that the report marks the method-level PCA interpretation
open:

- the report says it does not yet give a thesis-usable answer to what PCA
  applied to this project tells us;
- the report says the method-level interpretation remains open.

The review checked that future agents are not invited to treat PCA as finished
merely because the artifacts reproduce or because PC2-high has enrichment:

- `methods/README.md` says PCA is partial/status evidence;
- `methods/README.md` says PCA is not finished current method-table evidence;
- `pca-projection/report.md` says thesis writers may use it only as
  partial/status evidence.

The review checked reproduction:

- `pca-summary.json` reproduced byte-for-byte;
- `pc2-high-audit.json` reproduced byte-for-byte when using the committed
  summary path.

## Commands Recorded By Reviewer

- `git status --short`
- `git rev-parse HEAD`
- `git branch --show-current`
- `git show --stat --oneline --decorate --no-renames 0fc3e4c9`
- `sed`
- `nl`
- `rg`
- `jq`
- `uv run --script experiments/sys-landscape/datascience/methods/pca-projection/analyze.py ...`
- `uv run --script experiments/sys-landscape/datascience/methods/pca-projection/interpret_pc2_high.py ...`
- `cmp`
