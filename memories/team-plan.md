# Proposed user-facing sessions

Source investigations 2026-09-07; planning, not active assignments. The user
prefers separate Herdr chats for work involving him, with bounded subagents
underneath. This plan separates concrete outcomes rather than assigning an
unbounded subtree to an authoring or code lead. Overall purpose and feedback
constraints are in [planning context](planning-context.md).

This is a candidate session portfolio, not the full outcome graph.
[Broader outcome discovery](outcome-coverage.md) preserves composition, long
proofs and delivery obligations plus new concrete opportunities; their absence
as separate peers does not defer or erase them.

## HKO presentation and authoring feedback

First outcome: one substantive central-result presentation, self-reviewed and
ready for informative section annotations, not the final PASS/FAIL review.
`thesis/07-hko-local-maximum.tex` already separates the statement/motivation and
proof spine from long proof inputs. HKO is central in project facts; pentagon
is a side result and flow-graph has a larger algorithm/regularity surface.

Delegate source/prerequisite recovery, genuinely different presentation
alternatives, semantic review and reader reconstruction. Alternatives use the
recovered context; review follows candidates. Figure ideas can overlap with
drafting; implement them when they have an explanatory purpose. Local supporting
code/research belongs to this outcome, not an artificial separate phase.
Source-study and contextual annotation support can develop through actual use;
the citation index and review locator have concrete limitations recorded in the
broader discovery note. Whole-thesis prerequisite/coherence needs and later long
proof exposition remain outside this initial unit, not outside the goal.

## Predictable execution and artifacts

First outcome: a small named set of launch/materialization paths behaves
predictably, with synthetic regression tests rather than scientific reruns.
Independent initial tasks:

- `experiments/polytope-invariant-table/build-retained-table.sh` computes ROOT
  but invokes Cargo from caller cwd; fixed output names are opened with
  `File::create` in `write_database.rs`. Inspect intended overwrite/resource
  semantics before changing them. `experiments/sys-datascience/smoke-pipeline.sh`
  also mixes caller-relative Cargo with absolute Python paths.
- `scripts/artifacts.py:materialize` checks then installs a shared destination;
  source inspection suggests concurrent cold consumers can race at rename.
  Reproduce with fake rclone before patching; retain corruption/hash safeguards.
- `experiments/visualization/viewer/serve.sh` rebuilds/regenerates before
  serving; a cheap serve-existing route already appears in screenshot-script
  comments. Expose that distinction without inventing a new pipeline.

These are source findings, not reproduced bugs or permission for costly runs.
Shell tests, cache-race fixtures and command-contract review can run in parallel.

## Codebase architecture and migration

Session outcome: predictable stable reusable implementations, appropriate
experimental/instrumented variants, understandable relationships and effective
tooling, with justified migrations carried out. The earlier geometry overlap
is one input, not the scope boundary. The existing three crates and selected
capacity/instrumented-copy arrangement already realize part of this design.

[Architecture migration findings](architecture-migration.md) preserve concrete
consumers, semantic differences and four proposed work packages: ordinary
geometry; exact theorem-independent support; production/control/instrumentation
boundaries; deformation and flow-graph contracts.

First parallel work can characterize Euclidean components, extract shared exact
KKT assembly, and characterize instrumented traversal/correspondence. Each
migration depends on its own contract checks, not the entire inventory. These
are proposals, not active assignments; no test execution occurred in the
planning investigation. Historical evaluator semantics and meaningful variant
differences must survive import/layout changes.
Scientific change impact includes rounding/underflow assumptions, operation and
candidate ordering, exclusion/completeness and fallback contracts described in
`thesis/11-numerics.tex`; API compatibility alone does not cover these.

## Data-science contribution development

First outcome: a defensible account of what the retained results establish and
which substantive content remains to develop, not an abstract scope verdict.
Independent leaves: frozen 14336-row evaluator provenance; appendix diagnostic
support; the scientific questions answered by the retained results. Sources and
known exclusions are in [data-science scope](datascience-scope.md). Integrate
their findings before proposing further computations or asking value questions.
This can proceed alongside HKO; it is not a simultaneous full-chapter rewrite.

## Agent-process feedback

First outcome: determine whether an existing guidance patch earns its reading
cost using recoverable incident evidence and downstream use. Current shared
patches have source/structural review, not demonstrated behavioral benefit.
Global `~/.agents/memories/process-knowledge-design.md` owns the design context.

Incident reconstruction and a challenge of the proposed comparison can overlap.
Actual variant/retrieval trials depend on adequate inputs. If reconstruction
cannot support a useful test, use an upcoming real task or wait for an incident;
do not fabricate a replay or keep reorganizing skills. This owner collects
needed context from peers; peers need not write periodic postmortems.

## Dependencies and coordination

All five first investigations can start independently; that does not establish
five sustained workloads. Process-feedback work may finish or wait after its
initial evidence check while other sessions continue. Numerical API changes
require coordination with concrete callers; artifact tools own byte recovery,
not scientific interpretation. Authoring decides a figure's purpose; engineering
can help make its execution reliable. No session waits for a global skill merge.

This main chat keeps the overall map and cross-owner decisions, not every leaf
task. User-facing owners communicate directly via provenance-wrapped Herdr
messages and keep consequential decisions discoverable. Overlapping edits need
an agreed owner or isolated worktrees; root need not cherry-pick every change.
Pentagon/flow-graph presentation and final archive/submission remain later
outcomes, not erased or automatically launched to fill agent capacity.
