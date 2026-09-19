# Recognition/search screen: focus cues did not recover the main reader disagreements

Six fresh no-history reviewers examined immutable HKO and DS inputs under open, issue-class, and exact-locus conditions. **All three DS reviewers called the narrative coherent and predicted PASS, contrary to the recorded human narrative rejection.** The locus prompt explicitly asked about its research/discovery ordering. This is evidence against simply multiplying these review prompts as the next intervention; it is not evidence that a useful detector cannot be built.

## What ran

Each reviewer received only its frozen prompt and a complete input-only chapter/sample export, with permission to consult the paired PDF for extraction problems. No reviewer received historical judgments, the surrounding conversation, or other reviews. All used the inherited default model, reported by usage accounting as GPT-6 Astra. Six independent jobs ran concurrently; jobs used tool calls and are not six single API requests.

The output contract separated sentence readability, narrative readiness, overall acceptance prediction, anchored issues, and two acceptable spans. No minimum issue count was required. Class cues named purpose, reference introduction, inference burden and order; locus cues pointed to candidate passages while explicitly allowing them to be appropriate. The locus condition also supplied comparison loci: HKO opening orientation and DS scale explanation. These are qualified controls, not separately human-certified defect-free passages.

[Frozen prompts, exact input hashes, reports and accounting](../../experiments/writing-quality/runs/20260918-recognition/manifest.json) are retained. Canonical inputs remain in the dataset owner; the disposable export was removed after use. This was a single-run screen per condition, not a randomized estimate of prompt effects. The coordinator had read gold feedback before selecting loci; the review agents had not.

## Observed judgments

| Input | Open review | Issue-class cue | Exact-locus cue | Recorded human judgment |
|---|---|---|---|---|
| HKO | FAIL, mainly missing external computation/citation anchors | FAIL, narrowly, same main concern | Qualified PASS | Completed chapter PASS with seven local objections |
| DS | PASS; coherent trajectory | PASS; coherent progression | PASS; targeted ordering appropriate | Comfortable/passing sentence quality, narrative too chaotic, restructuring requested |

HKO's acceptance inversions are confounded by the deliberately standalone task: open/class reviewers explicitly say surrounding thesis references could change the verdict. Their reports identify an evidence-access problem and make it acceptance-critical, whereas Jörn accepted the chapter with local objections. This is not a clean measurement of their prediction of his actual reading context.

DS supplies the clearer disagreement: all three correctly identified comfortable prose, but also positively endorsed the sequence Jörn rejected. Even the locus reviewer wrote, “The research logic is available before the technical details,” and judged the targeted ordering appropriate. The prompt did not supply Jörn's desired discovery-to-pattern-to-conjecture narrative. The result therefore leaves wrong audience/purpose model versus insufficient discrimination unresolved; it does not establish missing general writing knowledge.

## Specific objections and controls

Conservative coordinator matching to active labels, not an exhaustive accuracy calculation:

| Human HKO observation | Open | Class | Locus |
|---|---|---|---|
| l001: clarify computer-algebra attribution | Broader verification traceability complaint; not exact recovery | Same | Same |
| l002: unintroduced certificate reference | Related unintroduced-verifier complaint | Directly recovered reference issue | Directly recovered, called minor |
| l003: worked example needs purpose before data | Not recovered; example praised | Not recovered; example praised | Not recovered; purpose judged already supplied |
| l004: unsolicited “not the reverse bound” distinction | Not reported | Not reported | Explicitly praised as clarification |
| l005: “forty-coordinate ... stronger” skips inference | Not reported | Not reported | Partial recovery: distinction left to reader, mitigated by earlier context |
| l006: explain geometric direction before coordinate prescription | Not reported | Not reported | Explicitly judged appropriate |
| l007: “without floating hints...” advertises code | “Floating hints” called implementation vocabulary; not exact recovery | Not reported | Not reported |

The cue ladder did not simply make agents agree with every proposed suspicion: the HKO locus agent rejected two historical objections and all DS agents accepted the structure. That rules out indiscriminate cue-following as a complete description of this run. It also means exact localization alone did not rescue agreement with this reader.

Both locus reviewers accepted the comparison material; the DS reviewers praised numerical qualifications and explicit comparisons. These are useful observations about discrimination, but not measured specificity. Passing HKO legitimately contains imperfections, and unannotated spans have unknown human status.

## Novel findings and potential harm

The reports consistently raise missing verifier/citation locators in HKO. The text supports the literal absence of usable locators in the standalone packet; whether that warrants FAIL is not established. Treating that requirement as decisive risks rewriting or rejecting accepted exposition for the wrong reason.

DS open/locus note that the low-ridge selection claim lacks concrete control/sample/effect details; open/class note the newly introduced term “Lagrangian product.” These are plausible local observations. No new human labels establish their importance, and adding detail could worsen the very narrative overload Jörn rejected. They must not be scored as verified novel successes or implemented automatically.

No repair occurred, so destructive-edit harm was not measured. There is no valid full precision or recall estimate: labels are selective, issue matching is interpretive, cases share project lineage, and there is only one job per cell. All six reviews share model and background knowledge; concurrency is not statistical independence of errors.

## Next action changed by this result

Do **not** immediately scale ten similar aspect reviewers or invest first in long-text partitioning. On a two-page sample, all three review variants missed the central disagreement despite a locus cue. Try an intervention that changes the reader/purpose representation:

1. Supply the intended research narrative as explicit task context, while withholding the historical verdict; test whether a reviewer can compare the actual paragraph sequence with that purpose without merely demanding verbatim conformity.
2. Contrast reader reconstruction or frozen prefix expectations with ordinary critique. Ask what research question each paragraph advances before asking whether the prose is good.
3. Transfer the candidate criterion to different natural material, retaining accepted imperfect text and a contrast that would make needless restructuring visible.

These investigate purpose-context and criterion mismatch before buying more search coverage. A reviewer that helps with reference defects can still be retained as a narrow specialist; this run supplies no basis for making it the overall acceptance gate.

## Cost and latency

The six jobs completed within the 108-second window from frozen manifest to collected reports. Metadata-linked worker usage was **$1.34 shadow API equivalent**; coordinator plus workers consumed **$2.50** during that window, excluding earlier preparation and this final writeup. Frozen before/after accounting reports retain exact totals and token counters. The earlier-cutoff report warns that the six later-created child threads had no records at that cutoff, as expected; the completed report has no accounting warning. In-flight/unflushed final work is excluded from those snapshots. These observations show that this diagnostic is inexpensive, not that a reliable writing detector has been obtained.
