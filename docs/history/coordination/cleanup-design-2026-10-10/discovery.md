# Evidence gathered in the first 15 minutes

Read-only bounded discovery; current observations on 10 October 2026.
Sources below distinguish observed facts from candidate interpretation.

## Existing failures and already-tried remedies

- `docs/history/process-audit/REPORT.md`, findings 1, 3–6: poor approach
  selection, local reviews not covering assembled outcomes, reviewer blind
  spots. Proposed forward tests are not evidence of effective prevention.
- `docs/knowledge/session-failures-2026-10-03.md`: remembering parent goals and
  observing completion did not reliably yield the right next action. Extra
  explanatory packets did not establish a benefit in short tests.
- `docs/history/session-postmortem-2026-10-04/tests/transfer/REPORT.md`:
  six hypothetical cases had rich information and near-ceiling baseline;
  cannot answer whether scattered-repo cleanup helps sustained Sol execution.
- `docs/history/session-postmortem-2026-10-04/tests/archival-consumption/analysis.md`:
  four paired historical reconstruction tasks, all eight answers adequate;
  additional packet increased reading by 7.3%, calls 63 to 83, and time in every
  pair. Both arms already received curated archives. Another summary is not
  automatically a usability improvement.
- `experiments/writing-quality/runs/20260918-human-feedback-revision/README.md`:
  supplied feedback, rewrite, fresh audience review, separate meaning review,
  repair and rereview already tried. Real fixes, no established human PASS or
  transferable acceptance predictor.
- `experiments/writing-quality/README.md`: seven immutable cases and 37 bounded
  human observations. Shared lineage prevents claiming independent validation
  merely by splitting local examples. Broad historical reviewers missed the
  main DS narrative objection and most HKO objections; narrower flaggers also
  produced false positives (`docs/history/reviewer-trial/README.md`).

## Representative concrete problems

1. `experiments/regular-products/pentagon-rotation-empirics/README.md` calls the
   retained certificate theorem evidence, while the sibling proof README and
   current thesis Chapter 9 explicitly give an analytic proof independent of
   it. A link checker misses this semantic mismatch. Useful test: a researcher
   entering through the empirical packet explains the current result/proof,
   necessary computations, retained historical route and existing extensions.
2. `docs/capacity-calculation-map.md` and producer contracts distinguish scalar
   capacity bounds, orbit payloads and minimizer sets. Deduplicating apparently
   similar implementations can break their different purposes. Root Cargo
   checks do not cover standalone workspaces, e.g. paired-tangentialization.
3. All 39 registered consumer paths for 13 external snapshots are absent in
   this new worktree; all 13 cache snapshot directories/manifests exist under
   `~/.cache/msc-math/artifacts/`. Presence only, hashes not checked. Missing
   worktree links do not establish lost data or justify expensive reruns.
4. CLI and running daemon report 0.162.1; `docs/codex.md` assessment is 0.160.0.
   `scripts/codex-feature-inventory.py --check` correctly fails because cached
   source lacks `rust-v0.162.1^{commit}`. A refresh mechanism exists but advice
   is not automatically current. No source refresh, config change or restart
   performed. Historical async-question cause remains unresolved in local
   sources; user's current reported failure must not be treated as a live
   reproduction performed by this session.

## Tests that would discriminate rather than just demonstrate activity

- Current tree versus candidate synthesis: fresh intended-model consumers must
  reconstruct a result and answer a new consequence question. Check source
  support, omitted conditions, repeated work, time and source-reading burden;
  include synthesis-authoring cost. Faster wrong answers fail.
- Adequate mathematical reconstruction followed by human exposition separates
  an information bottleneck from composition/evaluation failure. Freeze both
  writer output and review prediction before human judgment, if available.
- Challenge evaluators with plausible wrong-goal/incomplete candidates AND
  valid conservative candidates. Rejecting everything is not a good detector.
- Cold-start task with a required decision, independent available work and
  uncertain runtime behavior; inject steering. Check actual preservation,
  routing and continuation, not self-reported checklist completion. Do not
  invoke the prohibited async question tool to create this test.

These are candidate design-hour experiments. One successful narrow pilot can
support a restricted experiment; it cannot establish week-long reliability or
guarantee human thesis acceptance. Compare against the human-led baseline and
preserve useful failure outcomes without renaming them goal completion.

## Fresh Sol 6.1 medium baseline (completed about 15:57 UTC)

Exact research request (fresh context, read-only, <=4 minutes):

> I want to build on the current rotated-regular-pentagon capacity result.
> Explain what result is currently established, the main proof idea, which
> computational artifacts are necessary to justify it, and which source I
> should read to extend it. Distinguish related extensions so I do not repeat
> an existing result.

The worker was directed to normal discovery surfaces, given no answer key or
suspected stale path, and asked to report inspected files/missing inputs. It
returned the analytic profile and interpolation argument, correctly separated
the retained certificate, found affine and 7x7/5x7 developments, and named the
still-open general odd-by-odd route. It reported about 16 files read and no
unavailable necessary input. It also reported a retained-certificate source
hash discrepancy; the cause was not investigated and it does not affect the
analytic proof. No quantitative latency/token/cost comparison was performed.

It omitted the fixed-pentagon/arbitrary centrally symmetric partner branch and
its odd-regular calibration generalization. Independent targeted source
assessment by repo_probe confirmed this is material to the explicit request
to avoid rediscovering existing extensions. Sources:

- `docs/empirical-viterbo-design/desk/pentagon-arbitrary-partner/symmetric-partner-lemma.md`
- `docs/empirical-viterbo-design/desk/pentagon-arbitrary-partner/odd-regular-symmetric-partner.md`
- `docs/empirical-viterbo-design/desk/pentagon-arbitrary-partner/local-calibration-stability.md`

Assessment: useful but materially incomplete discovery, not a generally wrong
answer. The omitted odd-regular/symmetric-partner theorem does NOT resolve the
stated odd-by-odd nonsymmetric conjecture; the source explicitly separates
them. No full proof audit was performed. This supplies a concrete baseline
coverage failure that a candidate agent-facing synthesis must improve without
damaging the parts the current repository already makes usable.
