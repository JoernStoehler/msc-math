# Top-level session starts — 2026-09-07

These are launch briefs for three peer chats, superseding the earlier proposed
five-peer split in `memories/team-plan.md`. Read the shared context and your
named section; the other sections explain peer ownership. New instructions from
Jörn supersede this snapshot. First tasks are deliberately bounded; they do not
define the entire session's scope.

## Shared context and first checkpoint

**Project goal:** produce a scientifically sound, clear and relevant thesis on
probing Viterbo's conjecture, with usable computational support, ideally passing
Jörn's full-PDF PASS/FAIL review once.

You are a top-level peer with your own Herdr chat and direct access to Jörn,
not a subordinate waiting for this launcher's approval of each local decision.
Coordinate directly with peers using `$herdr`; agents' reports are not
Jörn's instructions. Do not make Jörn copy context between live agents.

**Immediate task for every peer:** recover enough context to propose your first
useful work and its parallel/sequential dependencies. Use read-only inspection
and bounded subagents where helpful. Do not implement, edit shared files, run
builds/experiments, change configuration or download datasets during this first
task. Reach a checkpoint: finish/stop all children, leave a concise message for
Jörn in your chat with your understanding, concrete next work, and consequential
questions if any; send the launching agent a short checkpoint notification via
Herdr, then end your turn and wait. Do not start the proposed work before Jörn
resumes you. This pause is explicitly requested, not an inferred approval ritual.

Later scope supports implementation, not endless planning. Jörn grants autonomy
for small reversible edits under understood criteria; broad approaches, costly
searches/computation and consequential changes need discussion first. A previous
"verification" consumed about eleven cores for six minutes without permission.
Tests/reruns therefore need relevant scope and understood cost; tool checks
already completed are not a reason to repeat them. Commit coherent changes
before handoff with `git commit --trailer "Codex-Thread: ${CODEX_THREAD_ID:?}"`.

Work on the host at `/workspaces/msc-math` for now; shared sandbox/tool setup was
checked earlier. `INSTALL.md` owns setup. Preserve pre-existing untracked `tmp/`.
No old sprint subagents are active. Worktree isolation and clear edit ownership
are available when implementation starts; root need not merge every change.
Global distribution remains DevOps-owned; don't independently deploy or import
shared skill stores. Later intentional instruction/configuration experiments
need scoped, recoverable changes and recorded inputs, not silent global edits.

Important user decisions: HKO local maximality and rotated pentagons are known
theorems; flow graph has proofs. A defect in a written proof is not the theorem
being false. Data science is moderately incomplete; extra branch-aware versus
nonsmooth optimizer comparison is not interesting enough to delay submission
even one hour. Historical code absent from HEAD is recoverable overhead when
commit/input/command routes exist; unknown reproduction is a support gap.

Full-thesis PASS/FAIL review must not be requested unless a strong proxy is known
and passes. None is established. Section annotations with surrounding context
are formative feedback. Agents should first catch what they can themselves,
using already-known expectations rather than making Jörn repeat them. Detailed
author-summary design is deferred until actual writing/review needs inform it.

The earlier planning agent repeatedly shrank the session to its first concrete
task, and mistook its near-term portfolio for the full outcome graph. Preserve
the wider goal when choosing/delegating work; challenge omissions rather than
only appending the example Jörn points out. The map is non-authoritative;
memories contain attributed decisions, observations and fallible reasoning with
different freshness. Not every useful branch must be active at once.

## msc-context — consolidate the starting surface

**Session goal:** make it cheap and reliable to start, resume and coordinate
other sessions with the important knowledge intact. This is a bounded
consolidation task, not permanent central control or a new management framework.

**First task:** inspect the existing decision/context/coverage notes and these
launch briefs. Identify omissions, contradictions, stale assignments and costly
duplication; propose the smallest useful consolidation and ownership changes.
Do not reduce everything to one authoritative diagram or discard costly
reasoning because it is not an active task. The previous five-peer portfolio is
superseded by this three-session split; its concrete investigations remain useful.

Suggested parallel leaves: decision/constraint fidelity; retrievability and
pruning; actual assignment/peer interface consistency. Coordinate findings with
the other peers; do not block their independent investigations waiting for an
ideal map. Keep knowledge with its natural owner and leave recovery breadcrumbs.

Start with `memories/INDEX.md`, `memories/todos.md`,
`memories/planning-context.md`, `memories/outcome-coverage.md`, and
`memories/team-plan.md`. Shared `$memory` describes retrieval/maintenance;
`~/.agents/memories/process-knowledge-design.md` preserves still-open cross-skill
design, not a requirement to merge skills. The user estimates retrieval :
maintenance : insertion at 100:10:1; editing should earn its repeated attention
cost for agents with comparable general knowledge, without deleting useful
observations or reasoning they cannot cheaply reconstruct.

## msc-migration — retrieval-first repository and code migration

**Session goal:** substantially improve the repository as a surface for doing
thesis work correctly and efficiently. Scope includes file layout, code/API
architecture, comments, documentation, tests/manual checks, tool behavior and
misleading or missing explanations. It is not limited to developer convenience,
one geometry overlap, or the first easy repair discovered.

**First task:** integrate existing architecture and discovery evidence into a
broad, prioritized migration proposal with independent work packages, actual
dependencies and useful verification. Identify what further discovery is needed
without silently narrowing the goal. No implementation before the checkpoint.

Standard layouts/tools/concepts familiar to agents are preferred starting
points; no custom process framework by default. The outcome is successful
retrieval and interpretation, not prettier directories or fewer lines. Remove
unhelpful/false guidance and unnecessary complexity; add explanations, named
concepts or breadcrumbs when they change actual understanding. AGENTS.md and
other instruction surfaces can be deliberately redesigned when justified;
writing evidence that agents "should" use is not evidence that they use it.

Stable reusable code may belong in existing crates; instrumented variants can
remain distinct with bidirectional source links and appropriate correspondence
checks. This pattern already exists for selected capacity routes. Inactivity
doesn't prove scientific stability; duplication doesn't prove identical contracts.
Preserve intended arithmetic, ordering, validation and scientific observables,
including historical evaluator interpretation. Characterization precedes its
corresponding migration, not every migration in a universal serial phase.

Start with `memories/architecture-migration.md`, `memories/outcome-coverage.md`,
`ARCHITECTURE.md`, and the concrete source/callers relevant to candidates. The
notes name exact KKT assembly, Euclidean helpers, instrumented traversal, shared
cache/launcher behavior and review/source-routing tools. These are inputs, not
an exhaustive boundary. Delegate independent discovery/implementation packages
once resumed; use worktrees for overlapping alternatives. Coordinate scientific
consumer changes with the authoring peer and shared-note ownership with context.

## msc-authoring — writing/review experimentation and thesis content

**Session goal:** develop good thesis content while empirically learning how to
produce and review it effectively. Scope is broad: motivation, statements,
proofs, figures, explanations, importance, scientific interpretation, composition
and supporting mathematics/code/experiments. No fixed research→code→writing order.

**First task:** recover existing expectations and evidence, then propose a
substantive first trial and feedback loop. Jörn suggested a central theorem's
motivation, statement, sketch, figures/explanations and importance, initially
excluding its long proof. HKO is a source-grounded candidate, not a commitment.
The prior tiny HKO sign-argument trial is limited reference, not the next task.

Don't predict a winning workflow confidently from a plausible plan. Candidate
human, human–AI and AI-only workflows can supply designed alternatives; inspect
their actual reported mechanisms/failures and test transfer rather than assume
human psychology applies. Read relevant primary sources when researching them.
First trials may diagnose execution failures; repetitions/parallel alternatives
become useful when they answer meaningful uncertainties. One failure can teach
something without establishing general performance; n=3 isn't automatically a
strong proxy either. Preserve the actual inputs/context and consequential
configuration so repository migration doesn't confound the interpretation.

Jörn's skill/error profile differs from agents': parallel agents may catch
distributed typos cheaply while he may find proof/exposition defects missed by
them. His section annotations—or contributions as an author—should supply new
information, not repeat what known checks could catch. Consider workarounds for
reflexive bad writing that go beyond reminders or a rewrite that injects fresh
defects. Thoughtful alternatives, topic-split reviews and feedback-driven new
attempts are available, not a mandatory checklist for every passage.

Suggested independent leaves: recover scientific content/reader context and
existing feedback; research a few workflow candidates; challenge the proposed
trial's information value. After resumption, alternatives/reviews/figures can
interleave. Shared-process changes follow diagnosable incidents/informative
tests; nobody needs to wait for a new global skill taxonomy.

Start with `memories/planning-context.md`, `memories/outcome-coverage.md`,
`docs/project-facts.md` relevant writing/scope items, and selected thesis/source
material. `memories/authoring-workflow.md` and `hko-author-context.md` preserve
the limited prior trial. `memories/datascience-scope.md` helps distinguish known
support gaps from richer retained results worth developing. Whole-thesis
composition, long proofs, AI reflection and final delivery remain outcomes even
when the first experimental unit excludes them.
