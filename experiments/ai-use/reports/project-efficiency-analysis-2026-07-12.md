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

The month-end Git/resource/value records are in
[`reports/project-efficiency-checkpoints/`](project-efficiency-checkpoints/).

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

## Why July 11–12 became expensive

The immediate incident is concentrated in the last two days: July 11–12
contain 5.768B tokens, or 80.6% of July's 7.158B tokens so far. The relevant
comparison is therefore July 1–10 versus July 11–12; June is only a broader
baseline.

| Metric | July 1–10 | July 11–12 | Change |
|---|---:|---:|---:|
| Tokens per active day | 154.5M | 2.884B | 18.7× |
| Rollouts per active day | 33.6 | 183.0 | 5.4× |
| Tokens per rollout | 4.60M | 15.76M | 3.4× |
| Subagent token share | 48.2% | 84.6% | +36.4 percentage points |
| High or xhigh effort share | 13.2% | 66.7% | +53.5 percentage points |
| Cache-hit share | 94.84% | 97.81% | +2.97 percentage points |
| Long-context requests | 70 | 2,252 | 32.2× |
| Mapped shadow cost | $1,201.89 | $3,903.43 | 3.2× |
| Shadow cost per million total tokens | $0.865 | $0.677 | lower during burst |

This gives an accounting identity for the incident rather than a correlation:

```text
tokens per active day = rollouts per active day × tokens per rollout
                    ≈ 5.4 × 3.4 = 18.4×
```

Rounding accounts for the small difference from the measured 18.7×. Thus both
more parallel calls and larger calls are required to explain the burst. The
model/workflow switch is visible too: July 1–10 was 83.1% GPT-5.5 and 16.8%
Sol, while July 11–12 was 86.8% Sol, 9.4% Terra, and 3.8% Luna. That switch did
not make tokens more expensive; the burst's per-token mapped cost was lower.

For broader context only, the June-to-July comparison remains:

| Metric | June | July through 12 | Change |
|---|---:|---:|---:|
| Tokens per active day | 398.0M | 650.7M | +63% |
| Rollouts per active day | 32.2 | 60.7 | +89% |
| Tokens per rollout | 12.36M | 10.72M | −13% |
| Subagent token share | 43.9% | 77.5% | +33.6 percentage points |
| High or xhigh effort share | 17.7% | 56.3% | +38.6 percentage points |
| Cache-hit share | 95.88% | 97.23% | +1.35 percentage points |
| Mapped shadow cost per million total tokens | $0.787 | $0.658 baseline / $0.713 long-context adjusted | lower in July |

This points primarily to more parallel calls and a much more high-effort,
subagent-heavy workflow. The average rollout actually became smaller, while
the number of rollouts grew sharply. July was not made expensive by a collapse
in cache reuse: absolute uncached input fell from 490.0M tokens in June to
197.5M in July despite the high July total. The model mixture also became
cheaper on average because Terra and Luna appeared; Sol and GPT-5.5 have the
same mapped rates in the current shadow model.

A repository-read proxy needs the same incident-local comparison; a whole-month
denominator hides the July 11–12 event. In raw tool-call arguments, July 1–10
versus July 11–12 had these occurrences:

| Command text | July 1–10 total | July 11–12 total | Per active day | Per million tokens |
|---|---:|---:|---:|---:|
| `sed` | 5,923 | 3,565 | 592 → 1,783 | 4.26 → 0.62 |
| `git` | 2,872 | 2,422 | 287 → 1,211 | 2.07 → 0.42 |
| `rg` | 1,263 | 2,041 | 126 → 1,021 | 0.91 → 0.35 |
| `find` | 747 | 637 | 75 → 319 | 0.54 → 0.11 |
| `cargo` | 303 | 679 | 30 → 340 | 0.22 → 0.12 |

So visible repository/tool activity did increase during the burst, especially
search and compilation, but it increased sublinearly relative to token volume.
The most frequently referenced individual paths in the burst occurred only in
the low hundreds (the largest were
`crates/symplectic/src/algorithms/orbit_search.rs` at 356 and
`experiments/dev-gradient-ascent/local-geometry-probe/main.rs` at 346 across
all rollout arguments). There is no evidence for a single file being read
thousands of times. This rules out a simple repository-read storm as the
primary explanation, but not the possibility that a few large tool outputs
were repeatedly included: the local logs do not expose tool-output token
counts or cache keys.

The proxy can be regenerated with:

```bash
uv run --script experiments/ai-use/scripts/analyze_rollout_tool_patterns.py \
  --rollout-csv /tmp/codex-token-usage-lifetime-refresh/rollout-daily.csv \
  --start 2026-07-01 --end 2026-07-12 \
  --period pre=2026-07-01:2026-07-10 \
  --period burst=2026-07-11:2026-07-12 \
  --out-dir /tmp/codex-rollout-tool-patterns-split
```

The optional `--period` arguments are important here: grouping the whole
month would hide the incident inside a larger denominator.

## Causal hypothesis audit

The following is the strongest conclusion supported by the logs, with the
remaining uncertainty stated explicitly.

| Hypothesis | Evidence in the July 1–10 → July 11–12 comparison | Status |
|---|---|---|
| More work or parallelism | Rollouts per active day rose 5.4×; subagent share rose from 48.2% to 84.6%. The top three parent tasks account for about 64.9% of burst tokens. | Strongly supported; primary factor. |
| Harder/larger reasoning calls | Tokens per rollout rose 3.4×; high/xhigh effort rose from 13.2% to 66.7%; long-context requests rose 32.2×. | Strongly supported; co-primary factor. |
| A more expensive 5.6/model mixture | Burst cost per total token was $0.677/M versus $0.865/M before it. Terra/Luna appeared, and the mapped Sol rate equals GPT-5.5. | Ruled out as the cause of the increase. |
| Cache collapse or cache misses | Hit rate improved from 94.84% to 97.81%. Uncached input did rise in absolute terms (71.4M → 126.1M), but its share fell (5.13% → 2.19%). | Not the primary cause; cache-write/cache-key details remain unobservable. |
| Pathological repeated repository reads | Visible command counts rose per day but fell per million tokens; individual path references stay in the low hundreds, not thousands. | Not supported as the primary cause; large repeated tool outputs cannot be fully excluded. |
| More productive thesis work | The top roots correspond to real numerics, experiment, harness, and AI-use work, but logs do not score accepted mathematical value. | Plausible but unmeasured; cannot justify the spend. |
| Model inefficiency or rework | Several top roots contain explicit user corrections about scope, acceptance gates, and reports, but this is observational and not a token-level counterfactual. | Possible contributor; not identifiable from logs alone. |

The burst is therefore not explained by one hidden price or cache bug. The
remaining explanation is a measurable workload multiplication: a small number
of broad parent tasks spawned many subagents, and those calls ran at higher
effort with much longer contexts. Whether that multiplication was worth its
mathematical output is a separate value question; the current logs cannot
collapse it into a truthful productivity number.

## Was GPT-5.6 itself wasteful?

The answer supported by this data is: **not established; the deployment was
wasteful-looking, but the model was not isolated as the cause**.

The model change and the workflow change are confounded. During July 11–12,
5.6 models account for essentially all recorded tokens, but the same interval
also introduced the 5.4× rollout multiplier, a 5.1× increase in high/xhigh
effort share, and 32.2× more long-context requests. The migration task's root
prompt explicitly prioritized correctness over token efficiency. That is
evidence of an intentional expensive policy, not evidence that 5.6 needed the
expense for equal work.

The token fields also show that this was not mainly verbose final answers:
99.8% of burst tokens are input/context tokens. The dominant cost is therefore
replayed parent context, tool output, and delegated-session context, not a
large amount of 5.6-generated answer text. Cache reuse improved, so even this
context-heavy behavior is not evidence of a cache failure.

There is a weak descriptive signal that pre-burst Sol rollouts were larger
than pre-burst GPT-5.5 rollouts (6.68M versus 4.31M tokens per rollout), but
those are different tasks and effort mixtures. It cannot identify a model
effect. No current log field says whether a Sol result was accepted, corrected,
or superior to a 5.5 result.

There is nevertheless a real model-conditioned fan-out signal, not just a
vague workflow label. Across June 1–July 12, restricting to parent lineages
with at least 1M recorded tokens, Sol roots had a median of 4.5 rollout files
and a mean of 12.5, while GPT-5.5 roots had a median of 1 and a mean of 5.4.
This comparison is still confounded by task selection and harness changes, but
it establishes that the observed 5.6-era agents actually spawned larger trees.
It does not establish that the trees were low-value.

Therefore the actionable conclusion is to restrict the expensive **policy**
now—fan-out, high effort, and inherited context—not to conclude that 5.6 is
intrinsically wasteful. A model-level conclusion requires matched trials: the
same bounded task and source packet, fixed delegation depth, 5.5 versus 5.6,
and an acceptance/value record for each result.

The dominant parent-task concentration is:

| Parent thread | Short task description from its root prompt | Burst tokens | Share |
|---|---|---:|---:|
| `019f50cf…` | Forward-test the experiment workflow and run the sys-datascience exploration. | 1.499B | 26.0% |
| `019f5306…` | Numerics session: mathematical/code closure and parallel verification. | 1.383B | 24.0% |
| `019f4adc…` | GPT-5.6 Sol/Terra/Luna migration and harness/delegation exploration. | 859.1M | 14.9% |

These are not independent random samples: the first two alone consumed about
half of the burst, and both explicitly involved broad parallel work. This is
why a month-level model or cache comparison was insufficient.

The practical compensation order is therefore:

1. cap or stage subagent fan-out;
2. reserve high/xhigh effort for questions where it produces a measured gain;
3. keep Sol/5.5 and long-context use selective;
4. investigate exact repeated tool-output content only if the first three
   controls do not restore the budget.

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
