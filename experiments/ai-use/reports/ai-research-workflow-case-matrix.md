# AI-Assisted Mathematical-Research Workflow Cases

Status: reviewed aggregate evidence for thesis section 13. This is a selected
case matrix, not a prevalence estimate, productivity study, or ranking of labor
types or model generations.

Cases were selected for explanatory and outcome diversity after observing the
project history; they are not a prospective or representative sample.

## Reading the matrix

Each case separates six stages that often occurred in different turns or
sessions:

1. **contract**: the research or implementation question supplied;
2. **produced**: output attributable to the inspected episode;
3. **verified**: checks actually performed and what they discriminate;
4. **interpreted**: conclusion drawn inside the episode;
5. **selected**: material Jörn correction, acceptance, rejection, or stopping;
6. **integrated**: later repository or thesis disposition.

Later artifacts are not attributed to the earlier episode unless recorded
lineage establishes the connection. Git establishes retained artifact state,
not idea origin or mathematical correctness. Session timestamps are wall-clock
evidence, not active human labor.

The private complete-case audits and coverage manifests are retained locally in
`experiments/ai-use/artifacts/ragged-frontier-2026-07-12/`. Raw session logs
remain source evidence and are not committed.

## Case matrix

| Case | Contract | Produced | Verified | Interpreted | Selected | Integrated and evidence boundary |
| --- | --- | --- | --- | --- | --- | --- |
| Early HK2017 implementation | Translate the public MATLAB HK2017 algorithm into a faster executable implementation. | An early Rust/f64 program that ran and enabled experiments while Jörn was still learning the detailed algorithm. | Early execution established that it ran, not that it was mathematically correct. | It looked promising as an experiment path. | Jörn later identified wrong results, poor code quality, missing pruning, and the need for substantial repair. | Extensive later implementation work produced the code used by the thesis. Timing and speed comparisons are Jörn's retrospective account; no human-only counterfactual was measured. |
| Crosspolytope search | Implement a specified search using the existing HK2017 infrastructure. | A running Rust binary and correct volume smoke after about eight minutes; initial commit `d0985fcd` after about nine. | The volume smoke checked the initial geometry. Later work added backtracking, symmetry handling, checkpointing, and capacity computation. | The early artifact made the proposal inspectable, not complete. | Search design and verification were expanded before accepting the result. | Search machinery `9c592c47`; same-day retained result `c=4`, `sys=3/4` in `da5049f2`; later phase `b567adca`. This supports time-to-inspectable-artifact under mature infrastructure, not total labor saving. |
| Projection-sign replay | Repair the known reduced-gradient sign defect from frozen base `f3d36cc9`, without receiving the historical fixture or answer. | All four agents repaired the production sign. Three produced discriminating regressions; the minimal GPT-5.6-sol run strengthened a symmetric fixture that remained sign-insensitive. | Independent mutation restored the bad sign. It failed the two verifier-first regressions and the GPT-5.5 minimal regression, but still passed the GPT-5.6-sol minimal test. | A plausible passing test was not evidence for the targeted repair when it could not detect the mutation. | The verifier-first contracts required fail-before/pass-after evidence. | Historical repair commits `94fee3e8` and `e56cf161`; replay threads and the observed mutation matrix are recorded in the companion replay packet. One bug and four runs do not rank model generations or estimate a causal effect size. |
| FG/CH2021 theorem and implementation review | Triage gaps between CH2021, a proposed generic flow-graph theorem, formal development, Rust behavior, and thesis claims. | A source-anchored review found the Type-2 capacity boundary, short-word and singular-boundary mismatches, overstated algebraic support, and underspecified hypotheses. | Paper-source inspection, call-site inspection, focused Rust checks/tests, proof-risk checks, thesis build, and diff review. These checks did not prove the theorem. | The review narrowed the theorem and implementation boundary; the formal theorem remained explicitly unverified. | Jörn selected the generic theorem route, rational-only implementation, and separation of singular diagnostics from theorem claims. | Durable corrections merged in `fcd8545a`. This is a positive review/error-detection case, not proof completion. Source rollout thread `019f1e49-d11e-7703-8a42-d8eba0a54837`. |
| Triangle--hexagon interval proof | Determine whether the regular triangle--hexagon rotation path could switch branches and exceed `sys=1`, and repair an underspecified empirical handoff. | An agent derived an explicit generalized three-bounce billiard proving `sys <= (3/4)sec(delta)^2 <= 1` on the fundamental interval, plus a separate range-norm lower bound proving endpoint equality. | A separate agent checked the coordinates, facet membership, normal-cone signs/indexing, endpoint nonsmoothness, action/volume normalization, non-translatability criterion, cyclic-maxima argument, and theorem boundary against repository sources. | The retained agent-derived argument establishes Viterbo's inequality on this path, conditional on the cited capacity--billiard interface; a fresh agent review accepted the proof, but Jörn has not yet reviewed or approved it. Equality with the secant branch at interior angles remains unproved and is unnecessary for the inequality. | The parent agent selected the proof for promotion and independent review. No inspected evidence establishes Jörn's mathematical review or approval. | Proof `a0d4414e` and review `dc966b07` were reconciled into Main by `43ac70b4`. The proof file explicitly identifies its core mathematics as agent-derived. Producing thread `019f5824-cb91-73b0-92f6-527a122944f7`; review thread `019f5829-e29c-7721-b459-08644fa7b0f6`. |
| Local systolic-behavior proof search | Determine what data at a base polytope can establish about nearby systolic behavior, including finite-distance completeness or bounds. | The route became an executable diagnostic comparing first-order prediction with fresh HK2017 recomputation; one tested failure mechanism involved a previously absent sigma becoming available after a transition-status change. | Rust checks/tests/clippy, release smoke runs, JSONL inspection, active-set diagnostics, and transition comparison at the base and perturbed point. | Useful bounded diagnostic progress; no finite-neighborhood theorem, universal transition account, or certified bound. | Jörn repeatedly corrected stopping, goal scope, vague review targets, and an initially poor KISS/YAGNI review. | Episode commits `a3ae1ee1` through `e1707049`; package later removed/migrated by `0c905a96`. Durable value was methodological residue, not a retained proof artifact. Thread `019eabeb-1fba-7792-b82d-be972b52adf8`. |
| Gradient-ascent convergence diagnostic | Test whether selected ascent endpoints supported convergence or local-maximum claims. | A development spike operationalized polishing and residual-step diagnostics and found selected top endpoints remained improvable. | Smoke/release runs and implementation checks supported the sampled endpoint result; compile/clippy did not establish mathematical adequacy. | The result undermined an interpretation of the endpoints as converged local maxima or attractors. No replacement theorem or proof-ready formalization was produced. | The agent later recognized that it had over-trusted a redundant task prompt and blurred evidence completion with branch readiness. | No durable integration was independently confirmed; the worktree disappeared and the result largely duplicated an existing diagnostic. Thread `019eb8bd-3552-7423-ae1f-424c29a52746`. |
| Candidate-polytope proposer | Turn retained scalar correlations into pre-capacity candidate rules and prospective candidate rows. | Low normalized two-face-area rules, exported polytope candidates, and a reusable selection interface. | Held-out proxy evaluation showed upper-tail enrichment; review found and corrected a pooled-control bug, stratified controls, and documented leakage/product-subtype limits. | The rules improved a proxy distribution but produced no `sys>1` row and no new mathematical conjecture or object. | Jörn and review separated proxy enrichment from the actual target and corrected the control contract. | Episode branch was abandoned; commits include `04c7bf12` and `6d7f5817`. The later maintained scalar/ridge proposer is separate evidence and must not be attributed to this episode. Thread `019f0f7d-c640-76c3-ae89-e21bbf4fad8e`. |
| Sys-data-science breadth | Pursue several research and workflow hypotheses in parallel, including an equal-budget forward implementation. | Roughly nine object-level lines and thirty structurally linked descendant sessions; the S0 implementation alone reached 7,312 inserted lines with several repair/no-go rounds. | Reviews and corrections were performed per line, but the principal real-target forward run did not occur. | The portfolio created bounded candidate and negative research state, not a completed empirical conclusion. | Jörn redirected premature narrow focus, enlarged or stopped tasks, paused the equal-budget run, and supplied mathematical prioritization cruxes. | Initially branch-local commits were later reconciled into Main as bounded research state by `43ac70b4`. Integration of state is distinct from execution of the target experiment. Root thread `019f50cf-2d2a-7521-a088-eb5bc180d01b`. |
| Profiling and handoff interpretation | Profile the HK2017 implementation and turn measurements into a research recommendation and handoff. | Useful profiling measurements and optimization artifacts, accompanied by an overconfident recommendation and contaminated handoff framing. | A later factual rewrite and fresh audit separated measurements from unsupported interpretation and provenance claims. | Initial interpretation exceeded what the measurements established. | Jörn chose the accepted provenance boundary and required the reassessment. | Retained commits `054c689b` and `95aa3506`. This supports a distinction between producing measurements and deciding what they imply. |

## Cross-case conclusions supported by this matrix

- In several observed implementation cases, an explicit executable contract and
  existing infrastructure yielded an inspectable artifact before trustworthy
  completion.
- Production repair and discriminating verification are separate tasks. A
  passing test is evidence only for failures it could detect.
- Several selected proof-development cases produced obligations, counterchecks,
  or empirical diagnostics without completing their target theorem. A later
  artifact-first case produced a retained agent-derived proof and independent
  agent review. These selected cases do not estimate proof-success frequency.
- One source-anchored theorem/code review found consequential durable defects;
  this establishes feasibility in that case, not comparative reliability.
- Candidate generation and proxy enrichment did not by themselves establish a
  new mathematical object, correct conjecture, or thesis-valued conclusion.
- Increased parallel production coexisted with substantial prioritization,
  verification, stopping, correction, and integration work. The matrix does not
  identify a causal bottleneck coefficient or net productivity effect.

## Deliberately unsupported uses

Do not use this matrix to infer:

- a general GPT-5.5/GPT-5.6 ranking;
- a ranked frontier of mathematical labor types;
- agent or human success rates;
- human-only counterfactual time or cost;
- idea origin from Git or prompt authorship;
- mathematical correctness from assistant claims, code volume, commits, or
  nondiscriminating tests.
