# Project-efficiency analysis

Status: canonical interpretation report for the monthly project-efficiency
experiment. It joins Codex resource accounting with month-end `main` snapshots
and a qualitative, source-backed value ledger. It does not claim to measure
mathematical research value objectively.

## Reproduction

Produce the full token packet first:

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
`/tmp/codex-project-efficiency-monthly/monthly.csv`.

## Monthly resource series

| Month | Active days | Tokens | Cache hit | Mapped shadow cost* | Model mixture | Month-end `main` |
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

\* Mapped shadow cost excludes historical labels without a pinned pricing map;
“not mapped” is not zero cost. The actual subscription cost is zero.

## Durable value ledger

One qualitative support unit means a material, source-backed improvement to a
thesis/project surface. Units are not percentages and are not assumed equal.

| Period | Value assessment | Evidence and boundary |
|---|---|---|
| 2025-09–2026-01 | Resource-only historical baseline; no repository snapshot exists for a defensible thesis-state comparison. | Do not infer progress from tokens. |
| 2026-02 | Repository activity exists, but value was not retrospectively scored; no visible Codex token activity is recorded. | `d7232894` |
| 2026-03 | Repository and harness work continued, but no standardized value ledger existed and no visible token activity is recorded. | `e37fc128` |
| 2026-04 | Data-science pilot/process infrastructure and thesis planning became more explicit, mixed with harness work. | `0ba62c6a`; not retrospectively unit-scored. |
| 2026-05 | Reproducibility, repository-status, and task-persistence controls strengthened; direct thesis value is mixed with infrastructure. | `b6a6b752`; no final-gate closure claimed. |
| 2026-06 | Six material thesis-surface support units: HKO, flow graph, QP/numerics, data science, rotated products, and reproducibility/AI-use. | Detailed evidence below; final thesis gates remained open. |
| 2026-07 through 12 | At least five further support themes: flow-graph theorem/figures, KKT/numerics contracts, data-science exploration closure, AI-use artifacts, and reproducibility/experiment evidence. | `43140cbb`, `dc1ca148`, `5e972118`, `c1d7a23e`, `45bf895d`, `1503caef`; partial month. |

## June 1 → July 1 example

The last `main` snapshots were `fc7f1b99` on June 1 and `fcd8545a` on July 1.
Between those end-of-day snapshots—June 2 through July 1—the project consumed
11.858B tokens and $9,365.79 mapped API-equivalent shadow cost. The mixture was
99.966% GPT-5.5 and 0.034% Spark, with 975 rollouts and 92,242 usage events.

The six support units are:

| Surface | Evidence | Boundary |
|---|---|---|
| HKO local maximum | `eaf31f0d`, `4aa37ff0`, `2e2f1579`, `4b062c09`, `8db8dcd2` | Certificate/verifier and trust-boundary route advanced; final theorem/advisor gate remained open. |
| Flow graph / CH2021 | `70fedca8`, `bfea2eab`, `406381cb`, `e87ea869`, `fcd8545a` | Algorithm, formal scaffolds, semantic tests, and theorem-scope exposition strengthened; final role remained conditional. |
| QP and numerics | `0ab23599`, `682ba32d`, `31cefd7e`, `e18c38f8`, `9a4148cf` | Finite-computation wording, f64 boundaries, failure demonstrations, and provenance clarified; numerical thesis surface remained incomplete. |
| Data-science search | `b0033904`, `c7e7a74d`, `8a8cb06d`, `94d6b2e6`, `d19821c9`, `b1bf6db8` | Baseline closure, LICCA producer/provenance infrastructure, run statistics, and retained evidence consolidated; demonstration remained open. |
| Rotated products | `c25aa2ae`, `bd7a9317`, `d8f4e9a9` | Thesis draft and proof-status framing advanced; finite-enumeration and exposition gates remained open. |
| Reproducibility and AI-use | `5ccde836`, `e8321161`, `f05d1e75`, `d1f1771d` | Reproduction promises, provenance inventory, and AI-use process evidence became explicit artifacts; final integration remained open. |

This is the defensible meaning of “value went up” for this interval: six
material support transitions, not six percent completion. No final thesis,
advisor, or submission gate can be claimed as closed from this accounting.

## Interpretation boundary

The resource series supports monthly comparisons of tokens, cache, model mix,
and mapped API-equivalent opportunity cost. The value ledger supports a
reviewable explanation of what those resources produced. It does not recover
Jörn hours or LICCA core-hours, and it does not justify a scalar productivity
curve without a separately accepted weighting scheme.

This is experiment-support material. A thesis use would be a caveated
reconstruction of the AI-assisted research process, not an objective measure
of mathematical research value.
