# Endpoint Residualized Regression Disposition

Date: 2026-05-31.

Purpose: record the review result for the `endpoint-residualized-regression`
row. This is a disposition note, not a repaired experiment report.

## Disposition

Recommendation: repair narrowly before thesis use, or cut the row from
thesis-facing evidence.

Current thesis role:

- not thesis-bearing;
- not a main hostile-landscape claim;
- not a conjectured-positive search lead;
- at most a future/caveat row until repaired or cut.

## Source-Backed Findings

- `research/sys-landscape-toolbox-audit.md` already marked the row unresolved:
  no current-contract report had been reviewed, and current artifacts were not
  enough for a terminal negative or thesis-facing claim.
- `research/sys-landscape-datascience/idea-ledger.md` marked the row
  `current-review`.
- `research/sys-landscape-datascience/method-ledger.md` marked thesis use
  `undecided`.
- `experiments/sys-landscape/datascience/methods/feature-pattern-search/analyze_residual.py`
  documents an endpoint-only residual packet, but its current `main()` calls
  `load_joined_rows(dataset_dir)` instead of
  `load_joined_rows(dataset_dir, endpoint_only=True)`.
- `experiments/sys-landscape/datascience/methods/feature-pattern-search/common.py`
  supports endpoint filtering via the `endpoint_only` argument.
- Therefore the committed residual plot is not reliable source truth for the
  stated endpoint-only question.
- `analyze_residual.py` defines `write_summary(...)`, but current `main()` does
  not call it, and no durable residual markdown report is committed.

## Scratch-Only Check

A scratch diagnostic on an existing `/tmp` dataset snapshot suggested that some
endpoint feature blocks may add grouped-CV signal beyond metadata. This is not
durable source truth and does not by itself give a label-free search rule, a
new `sys > 1` row, or a conjectured-positive follow-up.

Use this only as motivation for the repair-or-cut decision, not as thesis
evidence.

## Repair Scope

If Jörn keeps the row, the narrow repair is:

- make `analyze_residual.py` enforce endpoint-only loading;
- write a durable report beside the analyzer;
- record input data provenance, row counts, grouped split policy, metrics,
  caveats, verdict, thesis-use proposal, and reopen trigger;
- regenerate `feature_pattern_search_residual.png` after the script is fixed;
- update the idea ledger, method ledger, and toolbox audit with the terminal
  verdict.

If Jörn cuts the row, do not cite this packet in thesis-facing evidence. Mark it
future or omitted with the reason: current analyzer contract bug and no
actionable search rule.

## Open Decision

Jörn decision needed: repair this as a caveat-style endpoint diagnostic, or cut
it from thesis-facing evidence and leave it as future work.
