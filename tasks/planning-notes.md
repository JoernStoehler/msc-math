# Planning Notes

<!--
Migration-review note: live-test this file by asking a fresh agent what "Route"
means, what next object-level thesis task it would choose, what work it would
defer, where it would stop for Jörn/Kai, and whether the file supports
cost/value reasoning. The 2026-05-31 test passed: the fresh agent understood
Route as decision guidance rather than source truth or an executable queue,
picked a thesis-success-changing next task, deferred broad solver polish, and
identified Jörn/Kai stop points. Keep this file healthy by checking that future
agents still infer concrete source surfaces, stop conditions, and anti-busywork
guards from it.
-->
Route reasoning only. Not source truth. Not an executable queue.

Here, a route is decision guidance for choosing and constraining work. Route
bullets can be priorities, guards, sequencing constraints, task candidates, or
stop conditions. Before turning a route bullet into work, name the thesis/source
surface it can change and the stop condition that prevents open-ended work.

Before using a note, reread its source surfaces and the relevant
`current-state.md` row. Write executable fresh-session prompts in `/tmp`.

Route-note statuses:

- `active`: currently justified by `definition-of-success.md` and
  `current-state.md`.
- `deferred`: plausible but not current.
- `rejected`: do not retry without new evidence.
- `stale-check-needed`: refresh before use.

## Global Route

Status: active.
Evidence: migrated task-state synthesis.

Route:

- Preserve writeup-first closeout.
- Use active thesis scaffold files as the root surface for writing sessions.
- Settle retained claim wording while drafting.
- Route code, proof, experiment, and reproducibility maintenance from settled
  thesis wording.
- Do not reopen broad code/proof/compute programs unless retained thesis wording
  or final repo promises need them.
- For HKO, close exact Packet 3 to the strength needed for
  `thesis/hko-local-maximum.tex` theorem wording or weaken that wording
  honestly.
- For hostile landscape, resolve the current closure blockers in the hostile
  landscape route to support `thesis/black-box-datascience.tex` with caveats.
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
- HKO exact route closes or fails in a way that changes claim strength;
- thesis text promotes numerical/code promises not supported by current
  evidence;
- administrative facts reveal a hidden thesis-content prerequisite.

## HKO Route

Status: active while HKO remains retained thesis spine.

Reread before use: `research/hko-local-maximum*.md`, exact-Clarke artifacts,
`tasks/current-state.md`, `thesis/hko-local-maximum.tex`, and any HKO claim in
`thesis/abstract.tex`, `thesis/introduction.tex`, or `thesis/conclusion.tex`.

Route:

- Prefer exact first-order certificate if it becomes trusted.
- If exact certificate does not close, weaken thesis wording honestly.
- Do not claim strict local maximality in raw `R^40`.
- Do not use smooth-branch/Danskin arguments as a substitute for the
  arbitrary-polytope first-order gap.
- Do not schedule LICCA or higher-F perturbation by default unless cheap results
  already exist or Jörn chooses the external action.

## Sys First-Order Route

Status: active for generic thesis exposition; stale-check-needed before any
claim to solve the full arbitrary-polytope theorem.

Reread before use: `research/sys-first-order-local-behavior.md`,
`thesis/first-order-perturbations.tex`, relevant formal notes.

Route:

- Write the generic row-chart case first.
- Treat `thesis/first-order-perturbations.tex` as the current exposition target.
- State concrete open dense assumptions only when used.
- Keep generic smooth-branch theorem separate from the full non-generic
  compute-once evaluator.
- Discuss boundary/non-generic behavior later.
- Treat full semialgebraic evaluator as heavy fallback, not first exposition.

Acceptance guard:

- Do not call a route `PROVED` unless it includes compute-once `D(a)`,
  `Eval(D(a), h)` for arbitrary directions, degeneracy coverage, discharged
  proof obligations, and an algorithm contract.
- Check or explicitly exclude `beta_i=0`, limiting positive beta to zero, ray
  feasibility versus linearized feasibility, singular KKT or active continua,
  repeated/redundant listed rows, volume combinatorics, and exact-real versus
  `f64` behavior.
- Before treating a proof as theorem-ready, run a review whose goal is to
  downgrade it by finding hidden smooth-branch, Hadamard-only, ray-limit, or
  per-direction optimization substitutes.

## Hostile Landscape Route

Status: active while hostile landscape remains retained thesis spine.

Reread before use: `research/sys-landscape-toolbox-audit.md`,
`research/sys-landscape-datascience/idea-ledger.md`, current experiment report
paths, and reusable procedure under `research/sys-landscape-datascience/` if it
exists.

Route:

- Use bounded idea exhaustion, not open-ended method invention.
- Every thesis-affecting tried result needs repo-owned evidence and caveats.
- Run or resolve `endpoint-residualized-regression` first. This overrides the
  old ledger row's `future` status for the narrow purpose of resolving the
  existing artifact; thesis use still depends on the reviewed report and
  ledger/audit disposition.
- If it gives a conjectured-positive, stop unrelated method work and write a
  falsification/search packet.
- If negative or future-only, repair/downgrade `stat-sanity`.
- Optional small parallel probe: at most `svm-supervised-baseline` and
  `interpretable-tail-rules`.
- Stop for Jörn if a method needs new polytopes, cluster-scale compute, or a new
  feature definition.

Stop condition:

- Stop the current data-science closeout when `endpoint-residualized-regression`
  has a reviewed disposition, `stat-sanity` is either repaired or downgraded,
  no conjectured-positive lead is unresolved, and the toolbox audit states
  thesis-use/caveats for methods the thesis still mentions.

Closure summary:

- Closure blockers: coverage, verdict, positive-follow-up, evidence,
  experiment-validity, caveat, and thesis-use.
- Tried results affecting thesis wording need repo-owned evidence, verdict-fit
  checks, caveats, and thesis-use disposition.
- Positive-escalate or conjectured-positive results stop unrelated method work
  until Jörn or a falsification/search packet resolves the lead.
- Concrete worker prompts go in `/tmp`.
- Reusable worker launch/review procedure belongs under
  `research/sys-landscape-datascience/`, not in `tasks/`.

## Numerics Route

Status: active for retained numerics claims; deferred for broad solver
formalization unless thesis wording needs it.

Reread before use: `research/numerics*.md`, `formal/hk2017-qp-*.tex`,
`thesis/numerics.tex`, `thesis/appendix-numerics-proofs.tex`.

Route:

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

## Rust And Repo Maintenance Route

Status: active for cleanup that protects thesis closeout; deferred for broad
SWE polish.

Route:

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

## Submission And Archive Route

Status: active external-clock route; not a substitute for thesis-content
readiness.

Reread before use: `tasks/submit-thesis/`, current official university pages,
preservation target docs.

Route:

- Prepare external-clock actions when cheap.
- Do not use submission work as evidence for thesis-content readiness.
- Use `tasks/submit-thesis/README.md` for downloaded forms, local markdown
  conversions, source URLs, and checked preservation links.
- Verify official handin facts close to final submission.
- Choose preservation target before final archive.
- Keep arXiv/outreach post-Kai-review unless Jörn/Kai promote them.
