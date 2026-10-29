# Recorded cost evidence for the 28–29 October HKO research run

This note isolates execution and resource observations for the research campaign whose user launch message is in the native root rollout at **2026-10-28 22:35:36.893 UTC** (line 13). The task’s user-visible final delivery was **2026-10-29 01:22:51.305 UTC**, for **2 h 47 min 14.412 s** elapsed. The root rollout itself was created at 22:34:39.887 UTC, 57 seconds before the user launch; that earlier file/session birth is not used as the task start. Its delivery message says the paper was committed locally as `4e40a92f` and summarizes the checks; artifact-specific verification is retained in the project paper packet.

## Window and accounting method

The retained projection uses the half-open task window **[2026-10-28T22:35:36.893Z, 2026-10-29T01:25:00Z)**. The end leaves a small margin after the 01:22:51.305 delivery. It selects the root and native descendants found from actual rollout parent metadata: **39 threads with usage**, comprising root plus 38 workers/descendants. The earliest task usage was 22:35:41.133 and the latest was 01:22:51.311. The latter is within milliseconds of the delivery turn.

The sanitized per-thread projection is [usage-projection.json](usage-projection.json). It retains thread UUID, parent UUID, agent-path label, model ID, first/last observed usage times, response-record count, and token counters; it omits prompts, tool arguments, reasoning text, and outputs. The source was the locally selected Codex rollouts. I imported the installed `subtree-usage.py` functions from `/workspaces/agent-fleet-tools/skills/codex-resources/scripts/subtree-usage.py` and used its own inventory, parent-descendant, response ownership, response-ID deduplication, and counter validation. **No malformed/missing accounting warnings were returned.** This is response accounting, not account quota, a bill, per-tool cost, or proof of complete causal attribution.

The root rollout contains 716 cumulative `token_count` mirrors and 710 native `token_usage_record` envelopes over its full currently extended history; the bounded analysis counted 636 root-owned records in the selected window. The helper prefers owned native records, deduplicates repeated response IDs, and does not add their cumulative mirrors. Likewise, root-level worker sums below use each descendant’s owned records, not inherited fork prefixes. This is why the window total differs from raw line counts. No unexplained accounting error surfaced in the selected window.

## Whole-run native usage

| Scope | Model | Responses | Input tokens (cached within input) | Output tokens | API-equivalent subtotal |
|---|---|---:|---:|---:|---:|
| Coordinator root | gpt-6.1-sol | 636 | 129,961,959 (128,849,920 cached) | 279,101 | Unpriced in the dated table |
| Descendants | gpt-6-astra | 2,027 | 254,263,759 (250,101,248 cached) | 1,070,128 | **$345.23** partial shadow estimate |
| Descendants | gpt-6-sol | 1,223 | 138,215,858 (136,032,128 cached) | 374,776 | Unpriced in the dated table |
| Descendants | gpt-6-luna | 1,869 | 239,991,415 (234,909,056 cached) | 1,855,673 | Unpriced in the dated table |
| **All** | **four model IDs** | **5,755** | **762,432,991 (749,892,352 cached)** | **3,579,678** | **At least the priced Astra component; full equivalent unavailable** |

These large input totals include repeated cached context across many completions. They are model-reported request accounting; they are not token-quota points. The 5,755 count is completions, not independent scientific steps. No per-tool allocation exists.

The API-equivalent estimate is computed using the local resource helper’s fixed **2026-09-16** standard-API shadow table: gpt-6-astra at $10 / million uncached input tokens, $1 / million cached input tokens and $50 / million output tokens (with that helper’s documented per-request long-context rule). It is a hypothetical comparison only, not an invoice, subscription value, or current quote. The other three IDs had no rates in that dated table, and the root alone is entirely unpriced. Thus $345.23 is only the Astra worker subtotal; it must not be presented as the campaign’s full cost.

The projection pins the helper SHA-256, the rate table actually used and the
formula. For requests above 272,000 input tokens the helper doubles input/cache
rates and multiplies output rates by 1.5; cache-write input is priced at 1.25
times the ordinary input rate and remains within total input. These are the
historical helper's assumptions, not independently verified current vendor
prices. [cost-extract.py](cost-extract.py) regenerates the projection from the
local native sources; a different helper hash or changed source coverage must
be considered before comparing a future extraction.

## Bounded HKO and adjacent-result episodes

The table reports usage only for named worker threads during the indicated timestamps. These are **overlapping episode windows**, not exclusive cost allocations: they omit coordinator-root usage, include cached context, and do not establish that every token caused the result. The fixed-body intervals were selected from the root’s timed task/coordination messages and the workers’ recorded returns. The exact replay begins when its thread says the author froze the new certificate packet (00:38:53.079) and ends at the first PASS return (00:48:50.385). Multicut’s first window begins at that worker’s actual session creation (22:38:19.104); the approximate 00:04:10.888 split is where the extension phase follows the initial ascent and derivative reviews.

| Episode and owned worker path(s) | Bounded interval UTC | Model | Responses | Input (cached) | Output | Dated API shadow subtotal |
|---|---|---|---:|---:|---:|---:|
| Fixed-body development — geometry_frontiers | 00:14:26.973–00:51:10.249 | gpt-6-astra | 122 | 19,621,529 (19,293,056) | 51,301 | $25.14 |
| Fixed-body mathematical audit — hko_ascent_math_audit | 00:29–00:56 | gpt-6-astra | 51 | 7,957,355 (7,727,360) | 26,207 | $11.34 |
| Fixed-body fresh exact replay — affine_hko_exact_replay | 00:38:53.079–00:48:50.385 | gpt-6-sol | 59 | 4,914,542 (4,790,144) | 18,171 | Unpriced |
| Multicut initial discovery and ascent reviews — multicut_variation + children | 22:38:19.104–00:04:10.888 | gpt-6-astra | 218 | 29,600,274 (29,212,160) | 102,318 | $38.21 |
| ↳ same multicut worker group | same interval | gpt-6-luna | 513 | 73,975,101 (72,620,672) | 477,807 | Unpriced |
| Multicut extensions and later discriminators — still-active multicut workers | 00:04:10.888–01:09:02.224 | gpt-6-astra | 99 | 16,817,660 (16,524,160) | 80,859 | $23.50 |
| ↳ same later multicut worker group | same interval | gpt-6-luna | 12 | 2,035,745 (2,002,944) | 23,086 | Unpriced |

The first three rows are the bounded specialist work on the exact HKO-improving body: 232 responses, 32,493,426 input (31,810,560 cached), 95,679 output, and $36.48 from the priced Astra portions plus an unpriced gpt-6-sol portion. They overlap in elapsed time. This is useful episode accounting for development, audit, and independent exact replay; it omits the root’s interleaved work and cannot serve as the total cost of producing the body or its paper. Whole-paper editing, assembled reviews, and delivery remain visible in the whole-run totals and per-thread projection, but are not labeled as exclusive HKO costs.

The multicut branch is included as an adjacent result, not silently dropped from the path: its initial response-law/ascent work generated the twenty-facet local theorem; later work added the seventeen-facet refinement, quantitative effectivity and multi-hinge discriminators, including a scoped first-order obstruction. The split is approximate and causal; some thread context and work spans both phases. Astra’s worker subtotals use the dated shadow rates; the Luna/Sol components are not priced by that local table.

These episode windows overlap each other and other workers. Do not add their elapsed durations as serial labor. Per-thread spans and response counts do not measure attention, CPU runtime, or the causal share of the final theorem.

The 580,125-output initial multicut envelope combines early bank probes,
the quadratic/corner-family work, the general derivative theorem, complete
190-chart calculation and reviews. Its named children do not separate those
steps cleanly: `variation_duality` contributed to theory and failed general
shortcuts, `convex_ensemble` considered domination and radial candidates, and
`derivative_review` checked successive mathematics. No contemporaneous
per-approach token annotations were recovered. Therefore this note does not
price the 26-orbit probe, 6,040-orbit bank, quadratic certificate, corner-family
discriminator, or exact catalogue separately. An earlier timestamp split
would divide concurrent proof and exploration, not solve that attribution.

## Writing, assembled review and delivery: shared paper costs

The valued outcome includes a written and checked report. The following
bounded worker windows were recovered rather than leaving all that work in
the campaign total. They serve **the whole research paper**, including its
other theorems; none is an exclusive HKO charge. Root's interleaved authoring,
scientific checking, visual inspection, archive/export work and coordination
remain outside these worker envelopes.

| Shared episode / worker | UTC window | Model | Responses | Input (cached) | Output | Dated shadow |
| --- | --- | --- | ---: | ---: | ---: | ---: |
| Writing/integration — paper_integration | 23:57:58–01:18:23 | gpt-6-sol | 471 | 62,354,902 (61,747,328) | 141,460 | Unpriced |
| Assembled mathematical review — assembled_math_review | 00:10:53–01:14:36 | gpt-6-astra | 130 | 13,512,165 (13,053,824) | 54,246 | $20.35 |
| Reader review — assembled_reader_review | 00:11:04–01:14:44 | gpt-6-sol | 80 | 11,984,151 (11,680,896) | 29,320 | Unpriced |
| Final archive/build replay — affine_hko_exact_replay | 01:15:05–01:19:14 | gpt-6-sol | 30 | 3,523,291 (3,495,424) | 11,122 | Unpriced |

The first three windows bracket their original-campaign native usage spans,
rounded outwards to whole seconds. The archive window begins with the
replay worker's readable 01:15:05.229 update preparing bundle/manifest checks
and ends after its 01:19:13.662 PASS (native usage at 01:19:13.677). Its
01:17:20.668 update says the archive is ready; work before that included
preparation, not four uninterrupted minutes of replay. These messages are in
thread `01a184bf-69db-7023-8b99-653e04edd2bc`, rollout response-item message
records at lines 696, 829, 896 and 913. The delivery receipt preserves its
actual checked outputs. No archive producer or certificate was rerun for this
postmortem.

These shared windows total 236,148 output tokens, but that number is neither
the entire delivery cost nor the price of writing the finite theorem. It
excludes source authors and root and includes other results. The complete
per-model rows and exact window predicates are in the projection.

## Account quota observations (separate from token usage)

These are account-wide primary weekly quota snapshots retained in the root continuation and delivery messages, not campaign attribution:

| Observation | Remaining | Source/meaning |
|---|---:|---|
| 22:35 UTC launch baseline | 89% | Root’s 22:37:03.414 commentary reports the fresh check; it is account-wide. |
| 22:50 UTC | 86% | Root continuation snapshot; account-wide. |
| 23:09 UTC | 79% | Root continuation snapshot; account-wide. |
| 23:42 UTC | 58% | Root continuation snapshot; account-wide. |
| 00:01:29 UTC | 53% | Root continuation snapshot; account-wide. |
| 00:38:17.994 UTC | 43% | Root continuation snapshot; account-wide. |
| 01:16 UTC | 37% | Root’s final delivery message; account-wide and the latest campaign-era snapshot located. |

The initial 89%-remaining value appears in the original rollout’s user-visible progress message (line 110); intermediate values are preserved in `/tmp/msc-math/restart/ROOT-CONTINUATION.md` (lines 134 and 70). The final 37% value appears in the root delivery message at 01:22:51.305. These snapshots combine this run and unrelated account activity. They do not imply that the campaign used 52 quota points; no conversion from quota percentage to response tokens or API value is supported. No reset was used during the original campaign according to the retained continuation.

## Timing/cost evidence still missing

The account helper does not record which campaign consumed each quota point. No exact per-tool token cost, billed dollar amount, or non-model compute cost was recovered. The final delivery is timestamped, but it does not establish exact per-agent wall-clock effort. Some native tool-call argument bodies in this rollout family are encrypted; this projection did not need them. A useful next attempt for per-work attribution would require contemporaneous explicit annotations or a complete work ledger, neither of which was located in the retained postmortem inputs. The raw rollouts are private and intentionally not duplicated in this folder.
