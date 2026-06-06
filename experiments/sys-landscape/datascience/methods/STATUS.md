# Datascience Method Status

This file is the orchestrator-owned status ledger for
`experiments/sys-landscape/datascience/methods/`.

Method reports own evidence. Reviewers own findings. This file owns
orchestrator-approved method-row status. Worker-written report summaries and
reviewer verdicts are not authoritative status unless this file cites them and
records the approval scope.

## Status Rules

1. Executors may write evidence, observations, proposed interpretation,
   limitations, and proposed next actions in method folders.
2. Reviewers may write findings and review traces.
3. Executors and reviewers do not approve method-row status.
4. The orchestrator updates this file after inspecting the evidence and the
   review trace. Jörn approval is required for status that changes thesis
   wording, records a candidate-proposer, records a validated new row, or is
   otherwise ambiguous/high-impact.
5. A green review is evidence only for the checks named in its prompt and trace.
   It is not global proof that a row is finished, useful, or safe to merge.
6. If this file has no approved status for a method row, future agents must read
   the report and reason from evidence instead of inferring status from a
   report header, README summary, or reviewer verdict.

## Approved Status Ledger

| Method | Approved status | Authority | Evidence | Review trace | Scope limits / reopen trigger |
| --- | --- | --- | --- | --- | --- |
| `pca-projection` | Partial/status evidence. Not finished current method-table evidence. | Orchestrator-approved downgrade after charter-specific review. | `pca-projection/report.md`; `pca-projection/pca-summary.json`; `pca-projection/pc2-high-audit.json` | `reviews/pca-downgrade-charter-review-2026-06-06.md` | The PCA method-level interpretation remains open. Reopen or continue if PCA interpretation becomes thesis-relevant, if a pre-registered PCA-score proposal rule is specified, or if the retained dataset changes materially. |
| `table-column-regression` | No approved current retained-dataset evidence status. | Orchestrator placeholder. | `table-column-regression/report.md` | None. | Requires fresh evidence, explicit deferral, or abandonment before method-table use. |
| `regime-classification` | No approved current retained-dataset evidence status. | Orchestrator placeholder. | `regime-classification/report.md` | None. | Requires fresh evidence, explicit deferral, or abandonment before method-table use. |
| `endpoint-residualized-regression` | No approved current retained-dataset evidence status. | Orchestrator placeholder. | `endpoint-residualized-regression/report.md` | None. | Requires fresh evidence, explicit deferral, or abandonment before method-table use. |
| `pca-cluster-anomaly` / clustering row | No approved current retained-dataset evidence status. | Orchestrator placeholder. | `pca-cluster-anomaly/report.md` | None. | Stale source material only unless fresh clustering work replaces or abandons it. |
| `pca-cluster-anomaly` / anomaly row | No approved current retained-dataset evidence status. | Orchestrator placeholder. | `pca-cluster-anomaly/report.md` | None. | Stale source material only unless fresh anomaly work replaces or abandons it. |
| `supervised-alternatives` | No approved current retained-dataset evidence status. | Orchestrator placeholder. | `supervised-alternatives/report.md` | None. | Requires rerun, explicit deferral, or abandonment before method-table use. |
| `exact-f64-spot-check` | No approved current retained-dataset evidence status. | Orchestrator placeholder. | `exact-f64-spot-check/report.md` | None. | Requires rerun, explicit deferral, or abandonment before method-table use. |
