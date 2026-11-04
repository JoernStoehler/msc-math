# msc-math: agent entry point

Jörn Stöhler's MSc thesis probes Viterbo's conjecture through computational
symplectic geometry: proofs, Rust code, experiments, data and a reader-facing
manuscript. Several related results need their own claims and caveats.

## Start from the task

Use the assigned checkout; sibling worktrees have separate changes. The
workstation is the default, with cloud work only when explicitly assigned.
Start at `README.md`, retrieve task-relevant context, and follow summaries to
sources. Search claims, symbols, paths and synonyms with `rg`, or the available
source/search tools when shell access is absent. State the inspected source and
access limits; excerpts do not establish a checkout or exhaustive search. Avoid
broad legacy/generated-tree tours.

[PLAN.md](PLAN.md) is the readable project work document; [STATUS.md](STATUS.md)
is its short entry view. Use the shared
`$work-document`
to initialize, read, search or maintain the plan. Linked domain sources own exact
claims; recorded legacy registry/dashboard state does not establish live work.

| Work | Entry |
| --- | --- |
| Research, proofs, code or experiments | `docs/research-guide.md` → `topics/` READMEs; shared interfaces remain in `crates/`, `formal/`, `experiments/` |
| Manuscript and selected scope | `thesis/README.md`; `docs/project-facts.md` owns attributed Jörn-confirmed scope |
| Setup or cloud work | `meta-topics/development-environment/installation-workflow.md` and `cloud-workflow.md` in that directory |
| Coordination or continuation | `PLAN.md` → linked evidence and `meta-topics/project-management/thesis-work.md`; legacy registry/dashboard contract: `meta-topics/project-management/coordination.md` |
| Placement, retention or publishing evidence | `meta-topics/repository-organization/README.md` and `docs/artifacts.md` |
| Interpretation or historical evidence | `docs/knowledge/README.md`; `docs/history/` is provenance, not current assignments |

Project-specific skills, when present, live in `.agents/skills/`; shared skills are invoked by `$name` from the active catalog or plugin. Select skills from their discovered descriptions. If the catalog is unavailable, identify the missing guidance and continue independent work instead of assuming checkout-local symlinks. Source delivery does not establish automatic instruction loading.

## Quick commands

When a checkout and shell are available, run from the repository root unless a
local README says otherwise. Choose relevant checks; this is not one mandatory
test suite.

```bash
git rev-parse --show-toplevel    # this checkout
git worktree list               # sibling checkouts
rg --files topics crates thesis # source entry points
scripts/repo-status-summary.sh  # applicability of dated checks
cargo fmt --all -- --check
cargo check --workspace --locked
uv run path/to/script.py        # ordinary Python + inline dependencies
sh thesis/build.sh              # selected thesis → thesis/build/main.pdf
(cd formal && latexmk)          # proof-development build
```

`docs/algorithm-testing.md` owns focused Rust check scope. Topic/local READMEs
own reproduction, Sage, certificates and producer comparisons. Sage has a
separate environment; setup installs only the selected workload's tools.

## Repository-wide rules

- Match conclusions to support across `paper → proof → implementation/experiment
  → thesis prose`. Tests and finite experiments do not prove unrestricted
  theorems; exact arithmetic does not fill missing implications; LaTeX builds
  do not validate mathematics or prose. Finished thesis claims need matching
  caveats, resolvable evidence references and truthful reproduction promises.
- New inputs, outputs and metadata start in `/tmp/msc-math/<task>/<run>/` until
  deliberately retained. Keep lasting results with their source/evidence owner.
  Inspect inputs and write effects before expensive/replacing producers
  (`--full`, many `cargo run` commands, certificate workflows).
- Before prepared user-visible delivery, use `$message-review`
  for a separate subagent's approval of the exact complete text, including
  `[review: REVIEWER]`. Substantive changes need renewed review. Direct synchronous
  conversation and private native messages are exempt; the skill owns the boundary.
  If review is unavailable, keep delivery pending. Packets go in
  `/tmp/msc-math/message-reviews/`; lasting findings stay with their domain owner.
- During unfinished authorized work, “thx” and similar acknowledgments do not
  stop, pause or change scope. Continue until the intended work is complete,
  Jörn stops it, or required input is explicitly pending. Component completion
  does not close the parent objective; do not invent work after completion.
- Before large expenditure, present the concrete plan, alternatives, outcomes,
  uncertainty and resources, and get Jörn's explicit approval. Fund useful bounded
  preliminary exploration within scope. Do not invent thresholds or gates for
  small authorized tasks. Splitting work never enlarges scope or budget.
- The async question tool delivered a user reply on 2026-11-04; the previous
  prohibition is superseded. Use the async tool for required decisions when it
  fits the interaction. Include enough context and consequences for the choice
  to be actionable. An accepted submission is not an answer; silence and elapsed
  time are not answers either. Continue work that does not depend on the reply,
  and do not end a turn with an unresolved decision but no explicit request.
  Preserve pending approvals across ownership changes; optional clarification
  does not create a gate and keeps its stated fallback.

## Conditional obligations

Read the named owner when its trigger applies:

| Trigger | Obligation and owner |
| --- | --- |
| Delegation, human-review design, spending proposal or research/experiment selection | Read `AGENTS.md.commentary.md`. Delegate when useful within selected work. When delegating, name owned output, scope, resources, stop/review point and receiving owner; agree integration, commit and cleanup ownership. The coordinator supervises messages/reviews, dependencies and available telemetry. Details: `meta-topics/project-management/coordination.md`. |
| Material assignment, owner, dependency, decision or resource change | Update `PLAN.md` and its short `STATUS.md` view. If the selected work explicitly maintains the legacy registry/dashboard route, also follow its `meta-topics/project-management/coordination.md` contract. Unknown quota/spending stays unknown; old balances/deadlines are not live state. |
| Material thesis-source/planning change to selected claims, scope, uncertainty or route | Reconcile `docs/dashboard/thesis.html` under `meta-topics/project-management/coordination.md#thesis-synthesis`. Mark unresolved discrepancies visibly stale and hand them off; refresh `thesis-sources.json` only after source/PDF/PASS reconciliation. Avoid unrelated refreshes. |
| Material collaboration-friction change | Follow `meta-topics/project-management/friction/README.md`: update the record and linked Pages overview; report stale/unsaved status in handoff. This selects no monitoring or resumed thesis work. |
| Host paths or client recovery | `meta-topics/development-environment/development-environments.md#workstation-locations`, `meta-topics/development-environment/codex-reference.md`. The custom viewer is retired; `docs/file-viewer.md` is historical, not a launch procedure. Workstation paths are not prerequisites elsewhere. |
| Product mechanics affecting harness design, instruction placement, skills or subagent receipt | Use `$agent-platform-reference`; ask it for instruction/routing evidence and attributed bootstrap/retrieval advice. Shared harness-engineering owns intervention design and evaluation. |
| Independent-context exchange and cloud/VM clients | Use `$external-agent-communication` with project `cloud-workflow.md` and `docs/artifacts.md` for source delivery and retention. Where selected, `$codex-app-server` owns the VM-local client/setup/auth route; it does not connect workstation threads or credentials. |
| Literature collection or release/admin | `topics/literature/README.md` or `meta-topics/thesis-submission/README.md`. |
