# PCA Interpretation Technical Review

Date: 2026-06-06

Reviewed branch/worktree: `/workspaces/msc-math/.worktrees/ds-pca-interpretation`

Reviewed commit: `c5f9fcef`

## Findings

No blocker, high, or medium findings were reported.

The reviewer assessed the committed PCA interpretation work as technically
reliable enough to support the integration decision:

- current retained-dataset descriptive evidence;
- no current candidate-proposer;
- no validated new row.

Confidence reported by reviewer: about `0.85`.

## Low Hardening Issue

The reviewer found one low issue: `interpret_components.py` recomputed PCA
scores and mixed recomputed metrics with `pca-summary.json` top loadings
without explicitly checking that recomputed component count, explained
variance, and top loadings matched the summary.

Resolution: patched after review by adding
`check_recomputed_pca_matches_summary`.

## Checks Reported

The reviewer checked:

- committed diff from `36d41d9f` to `c5f9fcef`;
- data-science README/status/report surfaces;
- `analyze.py`, `interpret_pc2_high.py`, and `interpret_components.py`;
- leakage controls: PCA inputs exclude `sys`, capacity/raw/orbit witness
  columns, ids, and observation-table provenance before fitting;
- source labels and `sys` are used only after PCA fitting for audit and
  interpretation;
- dataset identity against report, dataset README, `pca-summary.json`, and
  `fingerprint-dataset.py`;
- `poly_id` consistency between retained tables;
- reproduction of all three PCA artifacts to `/tmp` with byte-for-byte
  comparison.

## Commands Recorded By Reviewer

- `git status --short --branch`
- `git log --oneline --decorate -n 12`
- `git merge-base HEAD main`
- `git diff --stat`
- `git diff --name-status`
- `git diff --find-renames`
- `sed`, `nl -ba`, `jq`
- `time uv run --script .../analyze.py --dataset ... --output /tmp/review-pca-summary.json`
- `time uv run --script .../interpret_pc2_high.py --dataset ... --summary ... --output /tmp/review-pc2-high-audit.json`
- `time uv run --script .../interpret_components.py --dataset ... --summary ... --output /tmp/review-component-interpretation.json`
- `cmp -s` for all three reproduced artifacts
- `uv run --script experiments/sys-landscape/datascience/fingerprint-dataset.py experiments/sys-landscape/datascience/dataset`
- a short `python3` JSONL check for `poly_id` uniqueness and table id-set equality

## Residual Risk

The main residual risk reported was later thesis wording drift: the result
could be overused as a search method instead of descriptive retained-dataset
evidence. The reviewer estimated this risk at about `10-15%` if merged without
careful status wording.
