# Project-efficiency pilot

Status: exploratory process diagnostic over three integrated `main` snapshots.
This is not a thesis-quality measure of research productivity and does not
claim that message timestamps recover Jörn's working hours.

## Question

Can the repository support a useful comparison of resource use and thesis value
without pretending that tokens, commits, or changed lines are value?

The pilot uses three dates at the end of the recent high-use period. For each
date, the Git snapshot is the last `main` commit recorded that day. Codex
resource data comes from the full visible token-analysis run. The completion
state is read from the versioned `PROJECT_COMPLETION.md` at that snapshot.

Reproduction:

```bash
uv run --script experiments/ai-use/scripts/analyze_token_usage.py \
  --start 2025-09-01 --end 2026-07-12 \
  --cutoff 2026-07-12T20:34:39Z \
  --exclude-thread-id 019f57d7-2e11-7181-aaab-685f65245ca8 \
  --root /home/vscode/.codex/sessions \
  --root /home/vscode/.codex/archived_sessions \
  --root /home/vscode/.codex/imported_session_logs \
  --out-dir /tmp/codex-token-usage-lifetime2 \
  --plot --plot-bucket month

uv run --script experiments/ai-use/scripts/analyze_project_efficiency.py \
  --date 2026-07-10 --date 2026-07-11 --date 2026-07-12 \
  --token-dir /tmp/codex-token-usage-lifetime2 \
  --git-ref main \
  --out-dir /tmp/codex-project-efficiency-pilot
```

The producer writes the raw combined table to
`/tmp/codex-project-efficiency-pilot/snapshots.csv` and the structured packet
to `summary.json`. The report is the interpretation layer; it intentionally
does not duplicate the full generated table.

## Resource observations

| Snapshot | Main snapshot | Tokens | Long-context shadow cost | Rollouts | Log span | User-message events | Tool-call events | Compactions |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| 2026-07-10 | `c457c78e` | 233.8M | $179.72 | 35 | 8.84 h | 109 | 1,355 | 6 |
| 2026-07-11 | `c15cbbe9` | 3.444B | $2,572.16 | 165 | 14.86 h | 1,144 | 6,538 | 74 |
| 2026-07-12 | `1503caef` | 1.846B | $1,113.60 | 175 | 10.53 h | 957 | 6,525 | 13 |

The log span is the first-to-last observed event in the selected rollouts. It
is not Jörn time, active CPU time, or uninterrupted work time. User-message
events include all visible rollout messages and are therefore only an
interaction-volume proxy. No LICCA-like command events were detected in these
three samples; this is not Slurm accounting and should not be read as proof of
zero LICCA compute.

## Integrated value observations

The coarse completion-state counts are identical at all three snapshots: all
20 required surfaces have the same labels in the versioned completion ledger.
This is the absolute-progress result at the current resolution:

| Snapshot | Coarse state vector | Coarse readiness change |
|---|---|---:|
| 2026-07-10 | 20 surfaces, labels as recorded in `PROJECT_COMPLETION.md` | baseline |
| 2026-07-11 | same 20 labels | 0 |
| 2026-07-12 | same 20 labels | 0 |

This does not mean that the work had zero value. It means that the existing
ledger records only large completion transitions. Substantial support work can
strengthen a gate without changing its coarse label.

The following is the actual value assessment performed in this pilot. It is a
qualitative, evidence-backed ledger, not a numeric score:

| Date | Evidence-backed change | Downstream gate | Value type | Confidence |
|---|---|---|---|---|
| Jul 10 | Completion anchor and data-science recovery audit clarify that exploration is complete but demonstration remains | Data-science; project control | Risk reduction and state clarification | High |
| Jul 11 | Generic flow-graph correctness theorem and major CH2021 chapter revision | Flow graph/CH2021 | Thesis-support strengthening | High |
| Jul 11 | Exact/f64/indeterminate/Sage trust boundary added and stale appendix route retired | Numerics; published code/data | Claim-boundary strengthening | High |
| Jul 11 | AI disclosure separated from research-process reflection | Use of AI | Scope and provenance clarification | High |
| Jul 11 | Visualization side-result packet completed | 3D visualization | Evidence/legibility support | Medium |
| Jul 12 | KKT witness and capacity contracts clarified in code, formal notes, and QP chapter | Haim--Kislev QP; numerics | Mathematical and implementation-contract strengthening | High |
| Jul 12 | Flow-graph tube construction and explanatory figures improved | Flow graph/CH2021 | Legibility and evidence support | High |
| Jul 12 | AI-use artifact distinction clarified in project facts | Use of AI | Scope clarification | Medium |

The commit-to-gate links behind these rows are `c457c78e`, `43140cbb`,
`14af3e18`, `c15cbbe9`, `4f364cb8`, `dc1ca148`, `c1d7a23e`, `5e972118`, and
`1503caef`. The changed-file counts and subjects are generated in
`snapshots.csv`; the table above is the interpretation layer.

### 2026-07-10: baseline and state clarification

The snapshot records the project-completion control anchor and the July
data-science recovery audit (`16a8b5f5`, `c457c78e`). Its main value is risk
reduction and state clarification: the data-science surface is honestly
recorded as exploration complete but still requiring a demonstration, while
existing thesis packets retain their review boundaries. This is real project
value even though no required-surface label visibly advances.

### 2026-07-11: high-cost mixed production

The day produced several potentially high-value thesis changes:

- a generic flow-graph correctness theorem and major CH2021 chapter revision
  (`43140cbb`);
- a numerics/exactness trust boundary in the thesis (`14af3e18`);
- separation of AI disclosure from research-process reflection (`c15cbbe9`);
- a completed visualization side-result packet (`4f364cb8`);
- QP verification-label and retired-artifact cleanup (`721edb12`, `42bbd011`).

It also included a large GPT-5.6 harness migration and related infrastructure
work. The resource cost was roughly fifteen times July 10's tokens, while the
coarse completion-state counts stayed unchanged. This is a high-value-looking
but mixed day, not evidence of fifteen times the thesis progress.

### 2026-07-12: expensive direct thesis support

The day included direct KKT-contract and thesis-chapter changes (`dc1ca148`),
flow-graph tube figures and exposition (`c1d7a23e`, `5e972118`), exact-route
corrections, and AI-use clarification (`1503caef`).
It used about 54% of July 11's tokens and 43% of its adjusted shadow cost, with
roughly the same tool-call volume. Relative to July 11, it is a plausible
candidate for better resource efficiency, but the value comparison remains
qualitative until the individual changes are mapped to evidence and gate
strength.

## What the pilot establishes

The resource side is feasible and largely automatable. The value side should
not start with a scalar 0--100 score. The useful unit is an evidence-backed
value record:

| Field | Meaning |
|---|---|
| Downstream gate | Which thesis, reproducibility, crate, or submission gate is affected |
| Value type | Closure, support strengthening, risk reduction, legibility, enabling work, or regression |
| Evidence level | Committed, built/tested, independently reviewed, or stakeholder-accepted |
| State effect | What changed in the absolute snapshot state, if anything |
| Confidence | How strongly the value interpretation is supported |
| Deferred value | Enabling work whose payoff is not yet realized |

Only after these records exist should we derive efficiency summaries such as
value per token, value per observed interaction span, or value per LICCA
core-hour. The denominators remain separate because they measure different
resources. A single weighted score would conceal whether a day was expensive
because of Codex, Jörn attention, or external computation.

## Disposition

The pilot supports building a recurring absolute-state/value ledger. It does
not support a progress curve from the current coarse labels. The next useful
extension is to refine each gate into evidence states—scaffold, drafted,
source-supported, built/tested, independently reviewed, and accepted—then
classify the three snapshot packets against that finer ledger with commit/file
citation. A future curve can be derived from those reviewed states without
assigning arbitrary credit to every commit.

This artifact is experiment-support material. It is not currently thesis-ready:
the log-derived resource measures are proxies, LICCA accounting is missing,
and the value classifications are intentionally qualitative.
