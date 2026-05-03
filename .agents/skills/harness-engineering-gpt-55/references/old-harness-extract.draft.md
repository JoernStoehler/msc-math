# Old Harness Extraction Draft

<!--
Working extraction from the pre-migration active harness surface. This is not
policy. Exact old wording is recoverable from git history; this file is the
compact review packet Jörn can mark up without reading every deleted skill.
-->

## How To Read

- Each bullet is candidate material extracted from the old harness, not a
  decision to keep it.
- Use labels informally:
  - keep: likely local commitment worth preserving;
  - rewrite: content may survive but needs a fresh surface or wording;
  - starter: useful example/checklist, not binding policy;
  - delete: old scaffolding, historical note, stale path, or over-specific
    process;
  - open: Jörn design judgment needed.
- Git history is the exact archive. This file is the usable compression.

## Cross-Cutting Candidate Commitments

- keep: `AGENTS.md` is the always-loaded exploration map. It should stay thin
  and point agents to maps, task files, and future skills.
- keep: `/tmp/` is for ephemeral prompts, one-off reports, and inspection
  artifacts; durable state belongs in repo surfaces.
- keep: Jörn owns thesis-scope decisions, mathematical judgment,
  advisor-facing framing, taste, and external-world actions.
- keep: agents should inspect local repo evidence before asking Jörn to do
  agent labor.
- keep: delegate and reviewer output is evidence, not authority; top-level
  sessions own integration and final claims.
- keep: prompt/harness material is instrumental. It is useful only insofar as
  it improves thesis-project work.
- keep: binding constraints should come from the objective, damage model, or
  local design choice, not from old process habit.
- keep: old campaign notes, calibration examples, and provider surveys are
  historical inputs, not active policy.
- rewrite: future skills should expose objectives, authority, evidence, and
  validation before process. Process should be a starter pattern unless the
  path itself is part of success.
- rewrite: domain convention surfaces should mostly help exploration and local
  edit judgment; avoid encoding exact layouts where the repo is actively
  moving.
- open: which future surfaces deserve durable skills versus task-local prompts
  or `/tmp/` prompt packets.

## Always-Loaded Map

### `AGENTS.md`

- keep: project goal and quality objectives for long-running multi-agent work.
- keep: domain map with `thesis/`, `crates/`, `formal/`, `experiments/`,
  `research/`, `papers/`, and `/tmp/`.
- keep: harness map in spirit, but after deletion it should say the old
  `.agents/skills/**`, `.codex/reference/harness/**`, and `.codex/agents/**`
  surfaces are under GPT-5.5 migration, not active stable policy.
- keep: quick commands for diff checks, crate tests, workspace builds, thesis
  build, formal build, and label lookup.
- rewrite: any references to deleted active skills or `.codex/reference/harness`
  files after the bulk deletion.

## Skill Surface

### `cached-map-maintenance`

- keep: cached maps answer "where should I look next?" and are not source of
  truth unless they explicitly own a convention.
- keep: maps should preserve entrypoints, ownership boundaries, navigation
  shortcuts, source surfaces, refresh triggers, and known open edges.
- keep: displaced content routing: active work to `ROADMAP.md` / `tasks/*.md`;
  research interpretation to `research/*.md`; repeated checks to future
  verification surfaces; implementation detail to local files.
- rewrite: table-based map taxonomy into simpler bullets if recreated.
- starter: checks included `git diff --check`, `scripts/toc.sh`, targeted stale
  searches, and skill validation when a skill changed.

### `data-science-subexperiment`

- keep: for delegated data-science experiments, the lead owns objective,
  review, and integration; the worker owns execution inside the objective.
- keep: workers should not decide thesis blockers from scratch.
- keep: source truth should be human-readable and repo-owned: code/script,
  command, dataset snapshot, filters/subsets, outputs when relevant, report,
  and post-review ledger/task update.
- keep: record observations separately from inference, checks, caveats, and
  whether the result gives an actionable search method.
- delete: semantic ledger slug workflow, v1 subagent smoke tests, exact report
  header, and generic worker prompt skeleton unless a future data-science wave
  actually needs them.
- open: whether a future data-science skill exists at all or whether packets in
  `/tmp/` are enough.

### `dataset-conventions`

- keep: generated `.jsonl` / `.csv` artifacts are tracked datasets; identify
  producer, consumer, freshness, and provenance before changing them.
- keep: generated data should stay with the producer; avoid multiple maintained
  producers writing the same tracked output.
- keep: do not patch-edit `.jsonl`; regenerate or report the needed refresh.
- keep: smoke runs should avoid mutating canonical tracked outputs unless the
  task explicitly requests a canonical refresh.
- keep: if tracked data changes unexpectedly, stop and report file and command.
- keep: trace provenance with targeted searches over artifact filenames,
  `Input Artifacts:`, `Output Artifacts:`, thesis sources, and research notes.
- rewrite: artifact declaration rules after experiment layout conventions
  settle.

### `experiment-conventions`

- keep: experiments answer research questions and produce data, figures, and
  evidence; interpretation belongs in `research/`.
- keep: experiment layout is mixed Rust/Python and sometimes non-standard; maps
  and local manifests should guide exploration.
- keep: formal math lives under `formal/*.tex`, named by math/proof object, not
  experiment path correspondence.
- keep: validation experiments own slow/broad mathematical evidence; crate
  tests own cheap live checks.
- keep: use semantic experiment paths; do not reorganize solely for visual
  balance.
- keep: helper `.rs` files may live beside experiment binaries; `src/` is for
  Rust-heavy packages or crate-incubator surfaces.
- rewrite: exact `main.rs` / `analyze.py` expectations are too strong and
  should become exploration guidance or current-pattern notes.
- open: standard smoke/default command conventions for experiment binaries.

### `formal-math-conventions`

- keep: `formal/` is developer-facing math for code and experiments; thesis
  prose follows a separate surface.
- keep: root-level formal files are named by formal object, theorem cluster, or
  proof cluster.
- keep: labels should be stable, unique, and cited by code with bracketed
  references such as `[lem:...]`, not rendered theorem numbers.
- keep: Rust comments should cite formal labels instead of duplicating proofs.
- keep: run the formal build or targeted label searches after edits.
- rewrite: any remaining file-layout guidance after the flattened formal
  layout settles.

### `harness-engineering`

- keep: see `harness-engineering-extract.draft.md`.
- delete: active skill deleted during this migration.

### `maintainability-improvement`

- keep: start from observed friction, not aesthetic complaints.
- keep: improvement packets should preserve behavior and mathematical meaning,
  and review should check behavior drift, loss of math-code correspondence,
  abstraction widening, and output compatibility.
- keep: split work only when write scopes are disjoint; top-level owns
  integration.
- rewrite: turn packet/worktree choreography into optional starter patterns.
- delete: mandatory `/tmp/improvement-*-packets.md` protocol unless repeated
  use proves it worthwhile.
- open: whether this remains a skill or becomes a small section under review /
  subagent delegation / code quality verification.

### `paper-download`

- keep: source acquisition is distinct from paper reading or citation review.
- keep: prefer arXiv source when grep-able LaTeX helps; local-only PDFs should
  not be committed unless explicitly intended.
- starter: naming pattern `papers/<abbreviationYear>/`.

### `post-mortem`

- keep: post-mortems are top-level, Jörn-invoked, advisory, and should not edit
  files directly.
- keep: focus on concrete operational failures and useful preserved behavior.
- keep: suggested changes should name the concrete failure they address.
- delete: any temptation to turn reflection into governance prose.
- open: whether this needs a durable skill or can live in current chat plus
  future `/tmp/` notes.

### `pre-merge`

- keep: pre-merge is readiness work before asking Jörn about integration, not
  merge approval.
- keep: scope verification to touched surfaces for docs/harness-only work.
- keep: prompt/harness validation includes diff checks, skill validation, TOML
  parsing for `.toml`, and stale-reference searches when names/paths/authority
  changed.
- keep: reviewer/subagent findings must be cross-checked before reporting.
- keep: merge conflicts should be resolved by semantic truth/current repo
  state, not timestamp, branch side, author, or task ownership.
- rewrite: rigid phase list into a touched-surface verification matrix.
- rewrite: experiment smoke/data freshness guidance after experiment
  conventions settle.

### `python-conventions`

- keep: experiment Python scripts are self-contained analysis/figure scripts.
- keep: use `Path(__file__).resolve().parent`-relative paths.
- keep: figure generation should use shared `experiments/figure_config.py`.
- keep: captions state observations, not interpretations.
- rewrite: exact `analyze.py` import snippet after experiment layout settles.

### `research-direction`

- keep: build the research surface before proposing an experiment, proof task,
  literature check, or implementation.
- keep: separate observation, inference, and speculation.
- keep: prefer checks that can falsify a hypothesis over work that only adds
  detail.
- keep: failed methods should remain in research record with reason.
- keep: route task-graph work to roadmap/task surfaces.

### `review`

- keep: review reports findings first, ordered by severity, with file/line,
  evidence, and action.
- keep: reviewers must name the review surface: diff, commit, uncommitted diff,
  exact files, or explicitly partial surface.
- keep: report-only reviewers do not edit.
- keep: factual findings should be verified against cited files/commands.
- keep: review references/checklists may be useful starter packets, not proof
  that no other issue exists.
- rewrite: skill should become a small core plus targeted review packets only
  if progressive disclosure remains useful.

### `roadmap-maintenance`

- keep: `ROADMAP.md` and `tasks/*.md` route work, preserve steering decisions,
  and cache resume points; they are not proof databases or final done
  authority.
- keep: PM work should make project state legible by reading, comparing,
  compressing, and drafting concrete choices.
- keep: ask Jörn only for priority, ownership, thesis/depth classification,
  decomposition, advisor framing, or decisions agents cannot make.
- keep: preserve Jörn/Kai/external decisions and why tasks are blocked,
  deferred, stale, or Jörn-owned.
- delete: historical delegation calibration appendix as active policy; extract
  only the cwd/worktree lesson.
- rewrite: exact task-bundle section rules after task surface is revisited.

### `rust-conventions`

- keep: default to standard Rust, explicit control flow, plain data structs,
  local one-off helpers, and moderate duplication.
- keep: avoid routine trait/generic/builder/framework layers unless they remove
  real complexity.
- keep: split files by real boundary: multiple callers, stable format/schema,
  test/verification surface, or immediate sharing.
- keep: distinguish mathematical code from orchestration code.
- keep: cite formal labels for non-trivial mathematical correctness.
- keep: crate tests are cheap live checks; broad validation belongs in
  experiments.
- keep: coordinate order is `(q1, q2, p1, p2)`.
- rewrite: experiment helper placement after experiment conventions settle.

### `slurm`

- keep: agents do not have LICCA SSH access; agents prepare scripts/binaries,
  Jörn submits and retrieves results.
- keep: resource choices need a short justification.
- keep: result retrieval/commit is Jörn-owned unless local files are already
  present.
- rewrite: exact template workflow if future cluster job orchestration changes.

### `subagent-delegation`

- keep: use subagents for bounded first-pass labor; top-level owns integration
  and correctness.
- keep: delegate output is evidence and must be verified before presentation.
- keep: delegate when result matters, output contract is bounded, and the
  result can be checked cheaply.
- keep: keep work local when the path matters, the task is tightly coupled, or
  the delegate would have to choose the boundary.
- keep: every subagent prompt names required cwd, scope, ownership, success
  check, output format, reserved decisions, and stop condition.
- keep: `spawn_agent` cannot set cwd; prompts must anchor cwd.
- keep: parallelize only independent read-only questions, review surfaces,
  disjoint write scopes, or independent checks.
- rewrite: packet templates as examples, not durable required forms.

### `thesis-tex-conventions`

- keep: thesis text is self-contained and publishable; do not rely on readers
  opening `formal/`, `experiments/`, or `crates/`.
- keep: agent edits inside `% Jörn:` approved scopes remove the approval
  marker.
- keep: new agent-written math is unapproved unless mechanical or Jörn-approved.
- keep: labels and references resolve; do not hardcode theorem numbers.
- keep: figure formatting belongs in Python; LaTeX inclusion is pass-through.
- keep: captions state observations, not interpretations.
- rewrite: marker syntax/details after thesis-writing workflow is revisited.

### `verification`

- keep: distinguish final thesis-done gates, reusable readiness passes, and
  current status checks.
- keep: route thesis-story interpretation to `research/`, milestones and
  ownership to `ROADMAP.md` / `tasks/*.md`, and reusable check definitions to
  future verification packets.
- keep: verification output separates supported/pass, caveat needed, missing
  support/stale evidence, and Jörn-only judgment.
- keep: packet checklists are starter workflows, not sufficiency proofs.
- rewrite: coverage list and packet set after thesis closeout surfaces are
  revisited.
- starter: useful packet properties include code quality, test coverage,
  reference resolution, data/figure traceability, repo-promise truthfulness,
  thesis-story support, submission artifact completeness, and falsification
  strength of verification experiments.

## Review Reference Packets

### `review/references/*`

- keep: claim review verifies numbers, counts, extremes, code behavior,
  bibliography claims, cross-references, and figure descriptions against
  sources; result statuses were `VERIFIED`, `WRONG`, `UNVERIFIABLE`, and
  `NO SOURCE CITED`.
- keep: figure review checks data source, script freshness, shared figure
  config, labels, legends, rendered clipping/overlap, LaTeX inclusion, and
  captions.
- keep: formal review checks label uniqueness/resolution, statements/proofs,
  Rust-label correspondence, assumptions, notation, and does not claim proof
  correctness.
- keep: Python review checks self-contained scripts, local paths, `uv run`,
  PEP 723 when needed, shared figure config, and observational captions.
- keep: Rust review checks formal labels, invariants, interface honesty,
  split justification, error handling, performance claims, and tests.
- keep: thesis review checks headers, approval markers, unverified wrappers,
  labels, self-contained prose, figure inclusion, captions, and bibliography.
- rewrite: decide later whether these remain separate packets or become one
  compact review checklist.

## Verification Reference Packets

### `verification/references/*`

- keep: code quality = explorable, predictable, maintainable, simple, and
  mathematically honest; check entry/read path, helper indirection, duplicated
  policy, boundary honesty, and math-code correspondence.
- keep: data/figure traceability = every thesis-used artifact has named source,
  provenance path, and matching interpretation.
- keep: reference resolution = reader-facing citations and internal references
  resolve to objects that support the cited sentence.
- keep: repo-promise truthfulness = repo artifacts and commands match
  deliverable/promise wording; weaken wording or name concrete fix on mismatch.
- keep: submission completeness = mechanical university/admin/archive package
  gaps are classified by owner.
- keep: test coverage = cheap bugs are caught by unit/regression/smoke checks;
  expensive validation is not a substitute.
- keep: thesis-story support = claims are supported, caveated, missing support,
  future/cut, or Jörn-only.
- keep: falsification strength = verification experiments should be able to
  expose the named failure mode and preserve negative/mismatching outcomes.
- rewrite: all packets should be rebuilt from current thesis surfaces if kept.

## Cluster And External Execution

### `slurm/references/*`

- keep: LICCA setup note records that Jörn handles SSH/submission/retrieval.
- keep: job template had resource TODOs and direct binary execution.
- rewrite: exact job template after future cluster workflow settles.

## Cross-Cutting Harness References

### `.codex/reference/domain/conventions.md`

- keep: module-level source files should have enough local purpose/context for
  agents to place them quickly; small leaf files can rely on clear names.
- keep: cross-file references should name explicit neighboring surfaces such as
  TeX labels, Rust symbols, or Sage symbols.
- keep: exploratory code starts in `experiments/`; stable approved algorithms
  migrate into `crates/`.
- keep: crate tests cover cheap regressions; slow broad validation, edge-case
  searches, random sweeps, and generated evidence live in `experiments/`.
- keep: code cites formal math with labels when correctness depends on formal
  results.
- keep: research-state notes, interpreted analysis, decision history, and
  next-step planning belong in `research/`.
- keep: generated data stays with its producer; `.jsonl` files are LFS-tracked
  generated artifacts; provenance is traced with targeted searches, not a
  maintained global dataflow map.
- rewrite: this content probably belongs in future domain convention skills or
  `AGENTS.md` map notes, not a separate hidden reference file.

### `.codex/reference/harness/session-rules.md`

- keep: work only in assigned cwd; treat tool default cwd as untrusted.
- keep: decide what result would prove the task done; tool success is not task
  success.
- keep: before replying, take the next useful step, ask one Jörn-only question,
  or report a concrete blocker.
- keep: remove only generated temporary artifacts clearly from current commands
  and not intended deliverables; leave ambiguous/unrelated work alone.
- keep: ask Jörn for mathematical judgment, thesis scope, advisor framing,
  taste, and external-world actions.

### `.codex/reference/harness/worktrees-and-git.md`

- keep: use local `main` as base unless Jörn names another base.
- keep: use root checkout on `main` only when the task targets it or Jörn
  grants main-checkout work.
- keep: create worktrees for isolated edits or parallel overlapping sessions.
- keep: every subagent prompt names required cwd; `spawn_agent` cannot set cwd.
- keep: agents may commit; ask Jörn about merge approval, not commit
  permission.
- keep: destructive Git operations require explicit approval.
- keep: `.jsonl` is Git LFS tracked; pre-commit blocks non-LFS files over
  10 MB.

### `.codex/reference/harness/planning-and-verification.md`

- keep: plans for multi-step work should name objective, dependencies, owner,
  and verification/review check.
- keep: route planning surfaces explicitly: `research/` for interpretation,
  `ROADMAP.md` / `tasks/*.md` for task routing, `tasks/verify-thesis-done.md`
  for final thesis-done gate.
- keep: before asking Jörn for review, run agent-doable checks such as
  buildability, consistency, attribution, figure/text alignment,
  claim/data alignment, references, tests, and scope drift.
- rewrite: this likely belongs in future roadmap/verification/pre-merge
  surfaces, not a separate reference file.

### `.codex/reference/harness/text-for-agents.md`

- keep: future-agent text should be correct/corrigible, observable,
  unambiguous, complete enough for the task, actionable, simple, and concrete.
- keep: vague words are search triggers, not banned tokens.
- rewrite: likely fold into future harness-engineering guidance, if at all.

## Subagent Role Overrides

### `.codex/agents/reviewer.toml`

- keep: reviewer is report-only and must not edit, revert, clean outputs,
  create worktrees, or repair findings.
- keep: reviewer reads `AGENTS.md`, review skill, assigned surface, matching
  convention skills, and only relevant review references.
- keep: reviewer must use required cwd named by assignment and report missing
  cwd before reviewing.
- keep: reviewer prioritizes bugs, false claims, stale paths, broken builds,
  missing tests, proof gaps, figure/text mismatches, data/source mismatches,
  and convention violations.
- keep: command-heavy checks inspect git status before/after and stop if they
  would unexpectedly change tracked files.
- keep: output findings first with severity, location, evidence, and action.

### `.codex/agents/simplification-scout.toml`

- keep: read-only scout can inspect bounded code surface and return decision
  packet about accidental complexity.
- keep: preserve behavior, data semantics, operational correctness,
  mathematical invariants, and experiment-specific policy.
- keep: discoveries should name location, evidence, epistemic status, and
  confidence; proposals should be bounded and actionable.
- delete: durable role override may be unnecessary if future
  maintainability/delegation surfaces can generate task-local prompts.
- open: whether to keep this as a subagent definition or replace with prompt
  snippets.

## Repo-Maintainability Design Notes

### `.codex/reference/repo-maintainability/design/*`

- keep: these files were already marked historical snapshots, not active
  instruction or live architecture state.
- keep: useful candidate facts include: no top-level `ARCHITECTURE.md` existed
  at the time; repo orientation was split across maps, crate module headers,
  experiment helper headers, and task surfaces; experiments used deep library
  paths; repeated helper families existed in experiments; shared polytope cache
  policy was open.
- keep: possible future architecture questions include stable versus expert API
  surfaces, canonical versus mirrored JSONL cache policy, and topic helper
  extraction rules.
- keep: historical execution constraints overlap with extracted session/git/data
  safety points: assigned cwd, worktree isolation, JSONL/LFS caution, and
  verification before broad refactor.
- delete: do not keep these as active harness/reference files. Use git history
  or rebuild fresh facts against current repo state if maintainability design
  work resumes.

## Obvious Bulk Deletion Candidates

- delete: all pre-migration active `SKILL.md` files except the GPT-5.5 harness
  migration skill after this extraction is committed.
- delete: old review and verification reference packets as active policy after
  extracting the candidate content above.
- delete: old `.codex/reference/harness/*.md` files as active policy after
  extracting the candidate content above.
- delete: old `.codex/agents/*.toml` active role overrides until fresh target
  roles are designed.
- rewrite: `AGENTS.md` harness map so it no longer routes agents to deleted
  active files.

## Open Questions For Jörn

- Which old domain convention skills should be rebuilt first: Rust,
  experiment, formal math, thesis TeX, datasets, Python, or research direction?
- Should review and verification return as separate skills, as one combined
  quality skill, or as task-local prompt packets?
- Should subagent role overrides exist durably, or should main sessions write
  `/tmp/` prompts from current context?
- Should `slurm` remain a durable execution skill because LICCA access is
  unusual, or should it become a small reference in experiment conventions?
- Should `paper-download` remain a durable skill or be deferred until needed?
