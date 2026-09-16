# Initial independent findings — before prior-audit comparison

Provisional diagnosis frozen before reading the prior audit. Companion DS/outcome files provide their separately scoped evidence. This file will not be rewritten after comparison; qualifications belong in reconciliation.md.

The largest likely gains come from testing whether the manuscript explains useful mathematics and whether the current approach deserves continued investment, before spending the scarce reading window on sequential local repair. This is a hypothesis about expected utility, not proof that a different process would have achieved the first deadline. Preserve the effective exact checks, ownership coordination, immutable review snapshots, candid uncertainty and carefully scoped PASS judgments.

## 1. Review calibration produced warnings without promptly changing the delivery decision

**Observed.** Coordinator raw rollout `01a0a021-b283-7fa3-9e16-aec87ce950df` at `/home/agent/.codex/sessions/2026/09/14/rollout-2026-09-14T13-35-58-01a0a021-b283-7fa3-9e16-aec87ce950df.jsonl` (exact path also in corpus.json): L496 acknowledged a material narrative miss at 13:58 UTC; L1020 acknowledged sentence-level reviewer unreliability at 14:22. At 15:23, L2620 reported no material followability issue and left preliminaries unchanged. At 15:33, Jörn immediately challenged their prose readiness (L2962); the coordinator admitted the gate checked prerequisites/inference gaps rather than prose (L2967). Review child `01a0a028-8c8c-7b10-9f75-d0d06482fbb5`, L703/L768, explicitly framed its scope around undefined prerequisites, inference jumps and false claims. The evidence supports a scope mismatch; it does not establish that the child disobeyed its unreadable assignment.

**Earlier alternative.** After the first calibration miss, separate mathematical integrity, sequential reading quality, and contribution/narrative coverage in delivery decisions. Before sending another substantial section, have a reviewer read a representative passage with the audience and existing feedback available, then state what remains untested. Do not relabel absence of mathematical findings as prose readiness. A small cross-section sample could reveal that opening fixes did not transfer to later chapters.

**Expected value / uncertainty.** High confidence in the mismatch; moderate confidence that a changed gate would save human attention. No observed recall estimate for a new reviewer protocol. Requiring exhaustive review before every delivery could itself miss the deadline; sample based on uncertainty and expand when failures recur.

## 2. The human feedback schedule discovered familiar local defects before mapping whole-thesis risk

**Observed.** Initial request explicitly targeted whole-thesis PASS and cheaper feedback loops (root L16); Jörn offered nearly the entire remaining reading window (L101). HKO request asked for the first material objection and 10–15 minutes of reading (L824, L1443). HKO then received successive local annotations and fixes. At 15:25 the introduction was still missing literature context (L2697), and at 15:35 Jörn himself skipped preliminaries to seek new error types in Chapter 4 (L3014). This last choice supplies a contemporaneous alternative, not hindsight invented from the later FAIL.

**Diagnosis.** First-objection reading is efficient for rejecting a passage but weak for estimating heterogeneous whole-document readiness. The successful HKO sample and many correct local repairs did not answer whether the empirical narrative, scholarship, prerequisites and contribution structure were all viable. This is distinct from falsely claiming whole-thesis PASS; the coordinator repeatedly avoided that claim.

**Earlier alternative.** Once prose feasibility is established, use a short, diverse acceptance probe: introduction/contribution map, one central proof passage, empirical results/figure, and one prerequisite transition. Ask for the most consequential obstacle in each relevant dimension, preserving Jörn’s right to read sequentially. Delegate known recurring repairs while using the next human request to reduce a different consequential uncertainty. Reopen a judged passage only when its substance changed or a residual risk warrants it.

**Expected value / uncertainty.** Likely high value for large heterogeneous research deliverables; not a universal review order. We do not know how much of Jörn’s observed reading would actually be displaced or whether his whole-thesis FAIL criteria could all have been discovered from samples. HKO reading also gave valuable positive calibration and must not be counted wholesale as waste.

## 3. Source checking answered a dated question while the task required current scholarship

**Observed.** Jörn recalled a recent capacity equality result (L2697). Coordinator L2749 and review child L835 used an HKO open-problem statement to contradict that recollection. Coordinator L2819 said the literature addition passed source-scope review. The next-day correction appears at L3917/L3975, attributed to Pro and subsequent verification. The raw contemporary exchange establishes the mistaken evidential inference even without assuming all later mathematical analysis is correct.

**Earlier alternative.** For a claim that something remains open, especially after a user recalls a newer result, verify the latest version/date and search forward from the cited open statement. Until resolved, preserve the uncertainty or omit the status claim. Agreement between two agents consulting the same dated source is not independent corroboration.

**Expected value / uncertainty.** High confidence in the need for date-sensitive checking; unknown probability of finding the relevant result in the available minutes. Scope-only citation checks should remain for hypotheses, but must not masquerade as current-status checks.

## 4. Durable work crossed from isolated draft to hidden build dependency

**Observed.** Current `.git/codex/thesis-candidate/main.tex` directly imports rewritten chapters from `.git/codex` (lines 47–80) and an absolute bibliography there (L22). Its README build command requires these paths. These are direct artifact observations, not acceptance of the coordinator’s later apology. Raw L2583/L2590/L2602 show the coordinator inspecting and maintaining such files during the sprint.

**Earlier alternative.** Keep parallel drafts and generated outputs in ordinary named project directories, with a declared source boundary and build dependencies. At first integration, check that a clean checkout plus declared external data can build the candidate. Freezing the review PDF was useful; it did not require hiding editable sources inside Git metadata.

**Expected value / uncertainty.** High confidence in recoverability failure; potentially high future project value. No evidence of actual data loss or that this materially caused the missed first deadline. One repository invariant plus a source-closure/build test is preferable to repeated prose warnings about hygiene.

## 5. Local correctness and documentation repair need a competing search for better mathematics

**Observed / evidence status.** Coordinator next-day report L3975 attributes a short pentagon proof and restricted-family ridge–capacity law to Pro, beyond its own presentation-focused diagnosis. The independent outcome audit examines the underlying frozen package. This is evidence that a better approach existed and was found later, not that it was readily discoverable at a specific earlier moment. Earlier root L812 describes reorganizing the existing pentagon proof and moving plots/code to appendices: a legitimate writing improvement within the incumbent approach.

**Earlier alternative.** At a costly proof or an empirical chapter with no clear mathematical conclusion, briefly compare repair, simplification, restriction, replacement and removal before commissioning more local polish or experiments. Give a bounded independent researcher the theorem/question and evidence, with permission to find a different route; do not merely ask them to audit the incumbent code. Stop the alternative search when no promising lead emerges within the chosen budget; do not make novelty a mandatory gate on a valid proof.

**Expected value / uncertainty.** Potentially the largest quality gain, but lower causal confidence than the review-scope and repository findings. No model/mode attribution or guaranteed time saving follows from the unequal-budget Pro comparison. An analytic attempt can fail; a computational proof can remain the right method. Evaluate the resulting math and explanation, not whether it resembles Pro’s answer.

## Successes to preserve and claims to avoid

- The initial 13:44 warning (L104) candidly said the deadline could not be forecast responsibly; do not claim the coordinator promised success until late. The explicit unrealistic assessment came after preliminaries feedback at L3081. Better milestone forecasting may help, but a precise alternative completion time is unsupported.
- Frozen PDFs and predictions prevented feedback from silently changing its target (L1406/L1436 and multiple hash checks). Keep version identity; avoid making every minor text edit a separate human review cycle.
- Mathematical checks found concrete defects and enforced claim bounds. The empirical appendix coordinate-space contradiction was repaired (L1282); provenance and exact checks have real value even though whole-thesis PASS did not follow.
- Human delivery was imperfect: L1212/L1220 show a missing pending review request, while L824 records a failed link. These are real friction. Their wall-clock spans are not recoverable idle-time totals: useful work and other reading overlapped. A single review-request owner and end-to-end delivery check are plausible small fixes.
- No evidence here supports changing the selected model, reasoning effort, sandbox policy or adding a general multi-agent framework as the first remedy. Parallel bounded reviews already helped; additional agents also require integration and can share the same blind spot.

## Small prioritized intervention set (proposal, not activated)

| Priority / owner surface | Concrete change | Feasible behavioral test | Overhead / failure guard |
|---|---|---|---|
| 1 — project kickoff/review workflow | Maintain a short acceptance map linking whole-thesis criteria to actual reviewed evidence and uncertainties. After a miss, alter the next delivery/review decision; use diverse probes before serial polish when overall readiness is unknown. | Give a fresh reviewer unseen passages spanning prose, mathematical prerequisites and empirical synthesis. Withhold human labels; compare severe misses and false alarms, then test on a fresh passage. Include a mathematically correct but badly motivated passage and a clean passage. Check whether narrow review findings are kept narrow in the handoff. | A few selected passages and one compact record; no review dataset construction prerequisite. Retire redundant maps/status files. Measure Jörn’s reading effort, not issue count alone. |
| 2 — research-task prompt / project research workflow | Before expensive incumbent repair, compare the best plausible alternative route against continuing; commission a bounded challenge only when it could materially change the result. | Replay a pre-repair decision packet without later solution. Score valid simpler approaches, informative early failures, and justified retention of incumbent; include a case where computational proof is appropriate. Use fixed time/compute budgets only in a separately authorized test. | One short decision at consequential branch points, not every edit. Do not reward novelty without correctness or unlimited exploratory branches. |
| 3 — repository build surface plus scoped guidance | Declare source/build locations; forbid deliverable dependencies inside Git metadata. Integrate via ordinary versioned source and test source closure. | Build exported tracked source plus declared dependencies in a temporary clean directory with no original `.git/codex`. Negative fixture includes hidden source import. | One deterministic pre-handoff check; migration is separate owner work. No migration performed here. |
| 4 — literature review/check contract | Distinguish theorem-scope checking from current-status checking; investigate later evidence before asserting a result remains open. | Supply an old open-problem statement and a discoverable newer theorem; separately a genuinely still-open case. Check precise theorem scope and uncertainty, not automatic agreement with the user. | Targeted searches for consequential status/novelty claims; no exhaustive literature hunt for every routine citation. |

Optional low-cost delivery fix: one owner tracks the currently pending request and immutable artifact; verify the actual offered URL and request acceptance. Repeated missing requests should trigger tool-state investigation rather than making Jörn report them again. Do not add a second task graph just for this.

Proposed tests are not run. Historical examples demonstrate failure modes, not measured effectiveness of these changes. Choose activation after integrating this audit with the prior one and Jörn’s priorities.
