# Planning Notes

Read this file as a short decision aid, not as a source of truth or a task
queue. In a normal short session, do not analyze the whole file. Read the top
rules, then only the section for the work area you are actually touching.

Use this file to answer:

- why a kind of work is currently prioritized or deferred;
- what source files to reread before acting;
- what stop condition prevents open-ended work;
- where Jörn/Kai review is required.

Do not use this file to:

- prove a mathematical, experimental, code, thesis, or admin claim;
- decide that a task is correct without rereading the named source surfaces;
- launch a broad cleanup/proof/compute program just because it is mentioned;
- treat examples or best-guess intermediate steps as required deliverables.

If this file and source truth disagree, source truth wins. If this file lacks
the reasoning needed to act, do not invent authority from it; either inspect the
source surfaces and update this file, or ask a focused question.

Planning hygiene:

- For high-level roadmap or next-packet planning, also read
  `tasks/references/planning-agent-memory-2026-06-01.md`. That dated reference
  records failure modes and review expectations for planning agents; this file
  keeps only the short operational rules.
- Treat unreviewed planning output as likely low quality. Common failure modes:
  missing reasoning breadcrumbs, false epistemic signals such as overconfidence,
  action lists that merely look like progress while ignoring opportunity cost,
  hazardous guesses filling missing information, motivated convenient guesses,
  and success-metric gaming that embellishes how close the thesis is to done.
- For roadmap or next-packet recommendations, first establish the evaluation
  contract in notes: done for the planning session, success measure, candidate
  list, source-backed facts versus judgment calls, visible risks, and question
  classification. Otherwise label the output a provisional sketch, not a
  recommendation.
- When reasoning about whether or when to do work, compare it against
  alternatives. Positive value is not enough. Include cost, opportunity cost,
  risks, value, likely outcome distribution, and relevant reference classes
  from similar past tasks.
- Planning or working artifacts need plain file headers. State what the file
  is, what it is not, whether it is source truth or a draft/quarantine, how
  future agents may use it, and what source truth overrules it.
- Separate final necessities, conditional necessities, best-guess
  intermediates, implementation details, and optional/future work.
- Do not call a step necessary unless the because-clause is written down. If it
  is necessary only while a story or claim remains retained, say that.
- For a best-guess intermediate, state which necessary deliverable it serves
  and what would make the intermediate obsolete.
- When asking Jörn a planning question, distinguish domain/context facts or
  review/taste judgments from planner-owned choices. Do not ask Jörn to supply
  arbitrary planning constants that should be derived from cost, value,
  dependencies, and failure modes.
- Do not ask abstract deadline/review questions such as "what is the actual
  target?" or "are there constraints?" when repo/admin sources can be checked
  first. Jörn confirmed that 9.6.2026 is a good deadline for sending the
  finished PDF to Kai. Official submission facts still need a current-source
  refresh before final handin; surface any missing concrete facts as risks or
  focused questions.
- Prefer deliverables, purposes, and milestones over process descriptions.
- Surface possible nonsense explicitly. If a recommendation depends on a weak
  assumption, a guessed cost/value tradeoff, missing source check, unsupported
  claim, or agent judgment that Jörn may reasonably reject, put that uncertainty
  near the recommendation. Do not bury it in prose, omit it to sound decisive,
  or let Jörn discover it only by adversarial reading.
- Prefer a short visible risk list over a polished but overconfident summary.
  Hidden bad assumptions can waste more thesis time than an extra review
  question.
- For high-level planning, run review before treating output as usable:
  independent sanity review for obvious bad assumptions, hidden gaps, omissions,
  and gloss; independent reasoning/completeness review checking that reasoning
  chains are reproducible from written breadcrumbs; and style/epistemics review
  for unclear language, overconfidence, imprecision, and embellishment.
- Ask Jörn questions in plain language about the specific missing fact or
  expert judgment needed. Do not design clever yes/no questions that appear to
  settle a decision while skipping the decision-relevant reasoning. If Jörn's
  reasoning cannot be reproduced from the current file, expose the missing
  reasoning first.
- Distinguish Jörn expertise from Jörn-only access. Jörn expertise includes
  mathematical judgment, research taste, advisor history, and tacit project
  context. Jörn-only access includes external conversations or actions, such as
  talking to Kai. Ask for the former as focused expert input; do not ask Jörn to
  perform source extraction or planning work that agents can do.
- Keep planning notes in planning-note style: compact decision guidance with
  source surfaces, reasons, stop conditions, risks, and review gates. Do not
  turn them into polished prose, a narrative apology, a prompt queue, or an
  activity log.
- Track epistemics explicitly. Mark source-backed facts, inferences, guesses,
  external/Jörn judgments, stale checks, and confidence or evidence strength
  when they affect work ordering or thesis claims.

<!--
Review status:
This top planning guidance and
`tasks/references/planning-agent-memory-2026-06-01.md` were committed once
before their own review rule was applied. After commit `22b14128`, three
independent read-only reviews checked sanity, reasoning/completeness, and
style/epistemics. This note records the amend that addresses their
needs-amend findings; it is not an endorsement of every downstream work-ordering
judgment as source truth.

Migration-review note: live-test this file by asking a fresh agent what
"working hypothesis" means here, what next object-level thesis task it would
choose, what work it would defer, where it would stop for Jörn/Kai, and whether
the file supports cost/value reasoning. The 2026-05-31 test passed for the old
"route" wording: the fresh agent understood the file as decision guidance
rather than source truth or an executable queue, picked a
thesis-success-changing next task, deferred broad solver polish, and identified
Jörn/Kai stop points. Keep this file healthy by checking that future agents
still infer concrete source surfaces, stop conditions, and anti-busywork guards
from it.
-->
Current working hypotheses for ordering and constraining work. Not source truth.
Not an executable queue.

Use these notes to see the reasoning that should be re-checked, not as
authority. A bullet here may be a priority, guard, sequencing constraint, task
candidate, or stop condition. Before turning a bullet into work, name the
thesis/source surface it can change and the stop condition that prevents
open-ended work.

Before using a note, reread its source surfaces and the relevant
`current-state.md` row.

Keep packet reasoning here while packets are being discussed, ordered, cut,
deferred, or scoped. Write `/tmp` material only for targeted one-use
consumption: an executable fresh-session prompt, or a scratch report from that
fresh agent before durable consequences are copied back to source files.

Planning-note statuses:

- `active`: currently justified by `definition-of-success.md` and
  `current-state.md`.
- `deferred`: plausible but not current.
- `rejected`: do not retry without new evidence.
- `stale-check-needed`: refresh before use.

## Global Work Ordering

Status: active.
Evidence: `tasks/definition-of-success.md`, `tasks/current-state.md`,
`thesis/MAP.md`, active `thesis/*.tex` scaffold state, `thesis/DEVELOPMENT.md`
questionnaire notes, `research/INDEX.md`, and the relevant HKO/hostile
research notes named below.

Deliverable ordering:

- Necessary final deliverables are: a defensible thesis PDF; central claims
  stated at supportable strength; evidence and caveats for retained claims;
  final Jörn acceptance; final build/readability/provenance checks; and
  submission/admin/archive completion.
- Current planning inference: the highest-level bottleneck appears to be that
  active `thesis/*.tex` is still mostly scaffold while source knowledge is
  scattered across thesis, research, experiments, formal notes, and tasks.
- Rank work by thesis-success gain per calendar day and Jörn-hour, penalized by
  open-endedness. Prefer work that makes the thesis writeable, settles central
  claim strength, or completes evidence needed by retained text.
- Distinguish necessary deliverables from best-guess intermediate steps.
  Side-by-side working files, subclaim maps, worker briefs, and staged audits
  are useful only when they reduce source-search/review burden or prevent
  open-ended work.

Necessity arguments:

- A defensible thesis PDF is necessary by the degree requirement and the
  project definition of success.
- Supportable central claim wording is necessary because unsupported theorem,
  experiment, computation, or scope claims would make the thesis indefensible
  or force late weakening after prose has already been built around them.
- Evidence and caveats for retained claims are necessary because the thesis
  story depends on proof-by-computation, bounded experiments, and negative
  search interpretation. The evidence standard may be weaker than a theorem in
  some places, but the support/caveat match is not optional.
- Final Jörn acceptance and build/readability/provenance checks are necessary
  because a thesis can
  be mathematically right but still fail submission or review through broken
  references, missing figures, unreadable organization, false repo promises, or
  uncaught overclaims.
- Submission/admin/archive completion is necessary because thesis-content
  readiness alone does not submit the degree artifact.
- A minimal section-control artifact is not logically necessary if Jörn writes
  directly from scattered sources. It is the current best-guess intermediate
  because active TeX is scaffold-heavy, source knowledge is scattered, and
  combining source discovery, claim-strength decisions, educational ordering,
  and polished prose in one pass creates avoidable attention splitting. Keep it
  small enough to reduce source-search and review cost for the next writing
  packet; do not let it become a separate planning project.
- HKO claim settlement is necessary while HKO remains a central result. It may
  settle as theorem-strength proof, proof-by-computation with a review gate,
  weakened computational evidence, or future/certificate-in-progress wording;
  what is necessary is eliminating hidden overclaim.
- Hostile-landscape method-table completion is necessary while the thesis keeps
  the hostile-landscape story. It may end with negative, failed, inapplicable,
  or positive/conjectured-positive rows; what is necessary is that rows used by
  the thesis have defensible terminal states instead of forgotten or
  half-interpreted gaps.

Reasoning:

- Planning inference from current scaffold/source-scatter evidence: the first
  bottleneck is probably not sentence polish. It is making the next thesis
  sections writeable by deciding what belongs where, which source supports it,
  what can be caveated or cut, and what needs Jörn/Kai review.
- HKO claim routing belongs inside the minimal section-control artifact, not
  after it, because HKO claim strength determines the abstract, introduction,
  conclusion, HKO section, and supporting algorithm/computation sections. A
  separate HKO proof sprint before that artifact risks doing expensive work
  without knowing the exact thesis wording it must support.
- Hostile landscape work should be organized around method-table completion
  because Jörn's target is a defensible account of the standard-toolbox search,
  including source-owned evidence, caveats, and thesis-use verdicts for
  failures or inapplicability. A single-row cleanup can be locally valid while
  still leaving the table incomplete and the thesis story weak.
- Final claim-support and repo-promise checks need the retained text. Running
  them before central prose exists mostly verifies known scaffold state and
  does not decide thesis wording.

Visible risks / uncertainty:

- The minimal section-control artifact is a best-guess intermediate. It is
  overhead if Jörn can write a section directly from existing sources faster
  than an agent can prepare useful working material.
- HKO and hostile landscape are prioritized because current task/research notes
  treat them as retained central stories. Advisor feedback or Jörn's scope
  decision can change that.
- The hostile method-table shape needs source refresh from the current ledgers
  before row workers start; stale rows must not be treated as already settled.
- Drafting may reveal hidden proof, evidence, exposition, or reproducibility
  blockers that change this ordering.
- Jörn/Kai may reject a proposed claim-strength branch even if it is internally
  well organized.

Current milestone order:

This is a dependency order, not a strictly serial schedule. HKO settlement,
hostile table work, and TeX drafting may overlap once the minimal
section-control artifact has enough source-backed material for the affected
section.

Current central-section control state, 2026-06-01:

- Durable control packet:
  `tasks/references/central-claim-control-packet-2026-06-01.md`.
- Scope: `thesis/abstract.tex`, `thesis/introduction.tex`,
  `thesis/hko-local-maximum.tex`, `thesis/black-box-datascience.tex`, and
  `thesis/conclusion.tex`.
- Use: start HKO blocker, hostile method-table closeout, central TeX drafting,
  rotated-regular-polygons wording, or supporting-section status packets from
  that durable file. Do not use the older `/tmp` copy as project state.
- Status: reviewed for obvious gaps by subagent `Beauvoir`; review fixes were
  applied. It is still a dated control packet, not source truth. Refresh
  affected rows from source files before treating them as current claim state.

1. Minimal section-control artifact: durable side-by-side working material for
   the next active thesis section or tightly coupled section group. Include only
   the central story outline, section claims, support sources, caveats,
   paragraph order, review gates, cut/defer options, relevant HKO
   claim-strength branches, relevant hostile-landscape table/story branches,
   and claims requiring Jörn/Kai review.
2. HKO claim settlement and hostile-landscape method-table completion, both
   using the minimal section-control artifact and run in parallel where useful.
3. Central TeX draft, starting as soon as the section-control artifact gives
   enough material; it need not wait for every HKO or hostile-landscape detail.
4. Claim-support audit against the actual retained TeX, not against imagined
   future wording.
5. Final Jörn acceptance, PDF build/readability checks, and submission/archive
   closure.

Guidance:

- Preserve writeup-first closeout.
- Use active thesis scaffold files as the root surface for writing sessions.
- Separate writing-control work from reader-facing prose polish. The near-term
  bottleneck is deciding what to say, where it belongs, which source supports
  it, and what review gate it needs. Polished prose for readers is a later
  pass unless Jörn explicitly asks for a draft to rewrite.
- Prefer durable side-by-side thesis working material for multi-day content
  organization, rather than mixing source-transfer notes into publication TeX
  or leaving them only in `/tmp`.
- Cost/value example: a one-section control artifact is worth doing when it
  lets a worker draft retained TeX with source links and review gates in less
  total Jörn time than direct source search during prose writing. A whole-thesis
  artifact is likely not worth doing if it delays the first retained section or
  turns weak guesses into apparent authority.
- Settle retained claim wording while drafting.
- Direct code, proof, experiment, and reproducibility maintenance from settled
  thesis wording.
- Do not reopen broad code/proof/compute programs unless retained thesis wording
  or final repo promises need them.
- For HKO, close exact Packet 3 to the strength needed for
  `thesis/hko-local-maximum.tex` theorem wording or weaken that wording
  honestly.
- For hostile landscape, complete the method table to row states with
  source-owned evidence, caveats, and thesis-use verdicts: inapplicable to our
  setting, failure on our side and not worth further work, no useful pattern
  found, meaningful pattern found, or positive/conjectured-positive requiring
  follow-up.
- For numerics, state the exact/f64/indeterminate contract needed for retained
  experiments and prose; do not create a public-solver certification program
  unless the thesis requires it.
- After writing and topic blockers stop surfacing, run final claim-support,
  provenance, repo-promise, build, and readability checks.
- Submission/archive follows thesis done, with external-clock prep allowed
  earlier.

Invalidate if:

- advisor feedback changes retained story blocks;
- chapter drafting reveals a hidden proof, evidence, or reproducibility blocker;
- HKO exact work closes or fails in a way that changes claim strength;
- thesis text promotes numerical/code promises not supported by current
  evidence;
- administrative facts reveal a hidden thesis-content prerequisite.

Near-term independent starts:

- Minimal section-control artifact: create side-by-side working material for
  the next active thesis section or tightly coupled section group with section
  claims, support sources, caveats, paragraph order, review gates, and cut/defer
  options. Include relevant HKO claim-strength and hostile-landscape table/story
  branches when they determine the abstract, introduction, conclusion, or
  supporting sections.
- HKO claim settlement: make the theorem target, current support, missing
  subclaims, artifact-to-subclaim map, review gates, and fallback wording
  explicit before launching an open-ended proof sprint. Any sprint cap should be
  derived from that check, not asked of Jörn as an arbitrary planning
  constant.
- Hostile landscape: turn the method universe into row packets and drive
  unresolved rows to terminal states with source-owned evidence and
  thesis-use verdicts that can survive focused review.
  `endpoint-residualized-regression` and `stat-sanity` are important unresolved
  rows, not the whole ordering.
- Thesis writing: `thesis/first-order-perturbations.tex` is agent-suitable for
  a generic row-chart draft with explicit non-generic caveats. Do not accept
  theorem-strength arbitrary-polytope or HKO wording there.
- Thesis scope audit: `thesis/flow-graph-algorithm-ch2021.tex` is audit-only
  until Jörn decides whether a theory/status section is worth retaining without
  implementation or empirical validation.

Defer:

- HKO claim-wording audit until theorem-strength wording is ready for focused
  review or HKO text starts changing.
- Numerics retained-claim audit until numerics text is retained enough to
  decide exact/f64 caveats.
- Repo promise/provenance audit until enough thesis text exists to know which
  code/data/command promises are retained.

## HKO Work Ordering

Status: active while HKO remains retained thesis spine.

Reread before use: `research/hko-local-maximum*.md`, exact-Clarke artifacts,
`tasks/current-state.md`, `thesis/hko-local-maximum.tex`, and any HKO claim in
`thesis/abstract.tex`, `thesis/introduction.tex`, or `thesis/conclusion.tex`.

Guidance:

- Prefer exact first-order certificate if it becomes trusted.
- If exact certificate does not close, weaken thesis wording honestly.
- Do not claim strict local maximality in raw `R^40`.
- Do not use smooth-branch/Danskin arguments as a substitute for the
  arbitrary-polytope first-order gap.
- Do not schedule LICCA or higher-F perturbation by default unless cheap results
  already exist or Jörn chooses the external action.

## Sys First-Order Work Ordering

Status: active for generic thesis exposition; stale-check-needed before any
claim to solve the full arbitrary-polytope theorem.

Reread before use: `research/sys-first-order-local-behavior.md`,
`thesis/first-order-perturbations.tex`, relevant formal notes.

Guidance:

- Write the generic row-chart case first.
- Treat `thesis/first-order-perturbations.tex` as the current exposition target.
- State concrete open dense assumptions only when used.
- Keep generic smooth-branch theorem separate from the full non-generic
  compute-once evaluator.
- Discuss boundary/non-generic behavior later.
- Treat full semialgebraic evaluator as heavy fallback, not first exposition.

Acceptance guard:

- Do not call a proof path `PROVED` unless it includes compute-once `D(a)`,
  `Eval(D(a), h)` for arbitrary directions, degeneracy coverage, discharged
  proof obligations, and an algorithm contract.
- Check or explicitly exclude `beta_i=0`, limiting positive beta to zero, ray
  feasibility versus linearized feasibility, singular KKT or active continua,
  repeated/redundant listed rows, volume combinatorics, and exact-real versus
  `f64` behavior.
- Before treating a proof as theorem-ready, run a review whose goal is to
  downgrade it by finding hidden smooth-branch, Hadamard-only, ray-limit, or
  per-direction optimization substitutes.

## Hostile Landscape Work Ordering

Status: active while hostile landscape remains retained thesis spine.

Reread before use: `research/sys-landscape-toolbox-audit.md`,
`research/sys-landscape-datascience/idea-ledger.md`, current experiment report
paths, and reusable procedure under `research/sys-landscape-datascience/` if it
exists.

Guidance:

- Use bounded idea exhaustion, not open-ended method invention.
- Every thesis-affecting tried result needs repo-owned evidence and caveats.
- Treat the method table as the deliverable, not any single row. Drive rows
  that the thesis relies on to a terminal state with evidence and adversarially
  defensible interpretation.
- `endpoint-residualized-regression` has a 2026-05-31 disposition note:
  current artifacts are not thesis-bearing because the analyzer does not enforce
  endpoint-only loading and no durable report exists. It is an important
  unresolved row, but not the whole work ordering.
- `stat-sanity` is likewise an important unresolved support row; repair,
  downgrade, or classify it as part of the full-table closeout instead of
  treating it as a serial gate after one other row.
- Candidate omitted/unfinished rows such as `svm-supervised-baseline` and
  `interpretable-tail-rules` should be evaluated by the same value/cost and
  thesis-table standard: inapplicable, our-side failure and not worth further
  work, no useful pattern, meaningful pattern, or positive/conjectured-positive
  follow-up.
- If any row gives a meaningful pattern, conjectured-positive, or actual
  positive result, stop unrelated method churn and write the follow-up or
  escalation packet.
- Stop for Jörn if a method needs new polytopes, cluster-scale compute, or a new
  feature definition.

Stop condition:

- Stop the current data-science closeout when the method table rows needed by
  the thesis have terminal states, no conjectured-positive lead is unresolved,
  and the toolbox audit states thesis-use/caveats for methods the thesis still
  mentions. Rows may end as inapplicable, our-side failure and not worth further
  work, no useful pattern, meaningful pattern, or positive/conjectured-positive
  follow-up.

Closure summary:

- Closure blockers: coverage, verdict, positive-follow-up, evidence,
  experiment-validity, caveat, and thesis-use.
- Tried results affecting thesis wording need repo-owned evidence, verdict-fit
  checks, caveats, and thesis-use disposition.
- Positive-escalate or conjectured-positive results stop unrelated method work
  until Jörn or a falsification/search packet resolves the lead.
- Concrete one-use worker prompts may go in `/tmp`; multi-day table/control
  material should live in a durable research or thesis-adjacent file.
- Reusable worker launch/review procedure belongs under
  `research/sys-landscape-datascience/`, not in `tasks/`.

## Numerics Work Ordering

Status: active for retained numerics claims; deferred for broad solver
formalization unless thesis wording needs it.

Reread before use: `research/numerics*.md`, `formal/hk2017-qp-*.tex`,
`thesis/numerics.tex`, `thesis/appendix-numerics-proofs.tex`.

Guidance:

- First state the exact/f64/indeterminate boundary needed for retained
  experiments and thesis prose.
- Use generic-case-first: explicit conditions, exact theorem/contract, f64
  diagnostics, then non-generic limit behavior.
- Candidate generic variables are full rank/condition of `C`, strict negative
  reduced Hessian on the retained tangent space, positive beta margin, positive
  `Q`/action gap from competitors, and adjacency/pruning assumptions.
- Fix or caveat only pieces the thesis cites.
- Treat broad solver formalization, beta-LP unification, and public certified
  solver polish as future unless retained wording needs them.
- Revalidate `thesis/legacy/migration-findings.md` rows 3-11 before relying on
  old algorithm boxes or numerical appendix prose.
- Tube algorithm work starts from Jörn's current raw source, not deleted stale
  thesis/formal/Rust surfaces. Before starting implementation, check whether
  `thesis/flow-graph-algorithm-ch2021.tex` or another active thesis file still
  retains tube content.

Tube import done state:

- current mathematical source states `Tube(k,s,Acut)`, breakpoint order and
  locations, finite polygon-affine representation, primitive tubes, tube
  intersection, action restriction, closed-loop fixed points, exhaustive
  simple-word capacity search, and current exclusions;
- thesis either includes a correct section matching that source or explicitly
  cuts/defers it;
- Rust implements primitive constructor, tube intersection, action restriction,
  closed-loop fixed-point solving, exhaustive simple-word search, and capacity
  plus simple Reeb-orbit output below `capacity + threshold`;
- evidence shows implementation matches the source for primitives, polygon
  emptiness, intersection, action restriction, fixed points, and comparison to
  HK2017 on small eligible examples;
- old thesis/formal/Rust tube files are absent from the active tree or rewritten
  from the current source.

## Rust And Repo Maintenance Work Ordering

Status: active for cleanup that protects thesis closeout; deferred for broad
SWE polish.

Guidance:

- Main must stay blocker-free for parallel agents.
- Use independent packets when possible.
- Ask Jörn for high-risk architecture/API/data-shape decisions.
- Do not ask Jörn for low-risk reversible mechanical cleanup where source
  evidence decides the choice.
- Broad architecture/API/SWE polish is future unless it protects retained
  claims, reproducibility, final repo promises, or current agent velocity.
- Treat scratch reports as non-durable unless their relevant result is
  summarized into tracked source.
- For exact/certified validation, do a code-first audit of exact/certified/
  ground-truth paths. Do not trust old weak audit coverage.
- `ehz_capacity_pruned_certified` is the exact rational output path for callers
  that need certified capacity/minimizers instead of scalar-style f64 result.
- Check euclidean-polytopes API decisions in crate README and DEVELOPMENT files
  before reopening them.

## Submission And Archive Work Ordering

Status: active external-clock work area; not a substitute for thesis-content
readiness.

Reread before use: `tasks/submit-thesis/`, current official university pages,
preservation target docs.

Guidance:

- Prepare external-clock actions when cheap.
- Do not use submission work as evidence for thesis-content readiness.
- Use `tasks/submit-thesis/README.md` for downloaded forms, local markdown
  conversions, source URLs, and checked preservation links.
- Verify official handin facts close to final submission.
- Choose preservation target before final archive.
- Keep arXiv/outreach post-Kai-review unless Jörn/Kai promote them.
