# Monthly project-efficiency ledger

Status: reusable monthly resource series with a qualitative value ledger. Each
row is one calendar month; the Git state is the last `main` snapshot available
at the end of that month. The resource totals are generated. The value column
is an interpretation record and is deliberately not a numeric progress score.

## Reproduction

First produce the full visible token packet:

```bash
uv run --script experiments/ai-use/scripts/analyze_token_usage.py \
  --start 2025-09-01 --end 2026-07-12 \
  --cutoff 2026-07-12T23:59:59Z \
  --exclude-thread-id 019f57d7-2e11-7181-aaab-685f65245ca8 \
  --root /home/vscode/.codex/sessions \
  --root /home/vscode/.codex/archived_sessions \
  --root /home/vscode/.codex/imported_session_logs \
  --out-dir /tmp/codex-token-usage-lifetime-refresh

uv run --script experiments/ai-use/scripts/summarize_project_efficiency_monthly.py \
  --start 2025-09-01 --end 2026-07-12 \
  --token-dir /tmp/codex-token-usage-lifetime-refresh \
  --git-ref main \
  --out-dir /tmp/codex-project-efficiency-monthly
```

The generated machine-readable table is
`/tmp/codex-project-efficiency-monthly/monthly.csv`. Months before the first
repository snapshot remain resource-only rather than receiving fabricated
thesis states.

## Monthly resources

| Month | Active days | Tokens | Cache hit | Mapped shadow cost* | Model mixture summary | Git snapshot |
|---|---:|---:|---:|---:|---|---|
| 2025-09 | 6 | 37.8M | 95.07% | not mapped | GPT-5/GPT-5-Codex | none |
| 2025-10 | 14 | 5.656B | 89.26% | not mapped | GPT-5/GPT-5-Codex | none |
| 2025-11 | 19 | 1.499B | 94.25% | not mapped | GPT-5, 5.1 variants | none |
| 2025-12 | 19 | 498.2M | 94.90% | not mapped | 5.1/5.2 variants | none |
| 2026-01 | 6 | 194.8M | 96.69% | not mapped | 5.2 variants | none |
| 2026-02 | 0 | 0 | — | — | — | `d7232894` |
| 2026-03 | 0 | 0 | — | — | — | `e37fc128` |
| 2026-04 | 17 | 4.719B | 95.40% | $2,068.66 | 5.4 dominant; 5.5 begins | `0ba62c6a` |
| 2026-05 | 25 | 3.958B | 96.10% | $3,033.55 | 5.5 dominant | `b6a6b752` |
| 2026-06 | 30 | 11.941B | 95.88% | $9,399.77 | 5.5: 99.97% | `57bb7730` |
| 2026-07 through 12 | 11 | 7.158B | 97.23% | $5,105.32 | Sol dominant; Terra/Luna material | `45bf895d` |

\* “Mapped shadow cost” excludes historical model labels without a pinned rate
map and therefore must not be read as zero cost. April also contains a small
unmapped Spark component. The actual subscription cost remains zero.

The monthly producer makes a one-point-per-month refresh cheap after the token
packet exists: it reads the daily/model/lineage/shadow-cost CSVs and resolves
month-end Git snapshots. A full raw-log refresh is the expensive local step,
but it is still a short batch operation in the current environment rather than
another long model investigation.

## Value ledger

The following records are intentionally coarse and source-backed. “Support
unit” means a material improvement to a thesis/project surface; it is not one
percent of completion, and units are not assumed equal.

| Month | Value assessment | Evidence / boundary |
|---|---|---|
| 2025-09–2026-01 | Resource-only historical baseline; no repository snapshot is available for a defensible thesis-state comparison. | Do not infer project progress from token use. |
| 2026-02 | Repository activity exists, but this ledger has no retrospective gate assessment and no visible token activity in the local Codex packet. | Month-end snapshot `d7232894`; value unassessed. |
| 2026-03 | Repository and harness work continued, but no standardized value ledger existed and no visible token activity is recorded. | Month-end snapshot `e37fc128`; value unassessed. |
| 2026-04 | Process and data-science pilot infrastructure became more explicit; the month also contains substantial harness work. | `0ba62c6a` and April commit history; support value plausible but not retrospectively unit-scored. |
| 2026-05 | Repository-status, reproducibility, and task-persistence controls were strengthened; direct thesis value is mixed with infrastructure work. | `b6a6b752` and May commit history; no final-gate closure claimed. |
| 2026-06 | Six material thesis-surface support units are defensible across HKO, flow graph, QP/numerics, data science, rotated products, and reproducibility/AI-use. | Detailed interval ledger in [June-to-July report](project-efficiency-june-to-july-2026-07-12.md). Final thesis gates remain open. |
| 2026-07 through 12 | At least five further support themes are visible: flow-graph theorem/figures, KKT and numerics contracts, data-science exploration closure, AI-use artifacts, and reproducibility/experiment evidence. | `43140cbb`, `dc1ca148`, `5e972118`, `c1d7a23e`, `45bf895d`, `1503caef`; partial month and mixed direct/enabling work. |

## Interpretation

The useful monthly question is now answerable in two layers:

1. What resources were consumed? The generated table gives tokens, cache,
   model mixture, mapped opportunity cost, active days, and Git snapshot.
2. What did the month buy? The value ledger names material thesis-surface
   support and says whether it closed a gate, strengthened evidence, reduced
   risk, or merely created enabling infrastructure.

The data does not support an honest scalar “research productivity” curve yet.
It does support identifying likely efficient or waste-prone months: June is a
high-cost month with broad thesis-surface support; July is an even higher
active-day-intensity month with a changing model and subagent mixture; early
historical months cannot be judged for thesis value because the repository
state is unavailable.

Future agents should update the monthly table and append value records rather
than redoing the full Git interpretation from scratch. The value ledger should
be revised when a support unit later closes, remains dormant, or is removed
from the retained thesis.

This is experiment-support material. A thesis use would require presenting it
as a caveated reconstruction of the AI-assisted research process, not as an
objective measurement of mathematical research value.
