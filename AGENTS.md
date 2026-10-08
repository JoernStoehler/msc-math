# Project guidance

## Purpose

This is Jörn Stöhler's MSc thesis repository on probing Viterbo's conjecture through
computational symplectic geometry. It combines mathematical arguments,
proof-by-computation, reusable Rust code, experiments, data and a reader-facing thesis.
The project has several related results rather than one forced narrative.

A finished thesis needs claims and caveats that match their mathematical and computational
support, resolvable code/data/figure/certificate references, and truthful reproduction
promises. Compilation and tests cover only parts of that outcome.

## Workstation locations

Work directly on the workstation for now. Use the checkout assigned to the task;
`git rev-parse --show-toplevel` and `git worktree list` identify its root and sibling
checkouts. These are workstation locations, not required paths on another machine.

| Location | Use |
| --- | --- |
| `/workspaces/msc-math/` | Main checkout. Other worktrees have separate files and changes. |
| `~/.codex/worktrees/` | Codex-created worktrees; locate this project's checkout through Git. |
| `/tmp/msc-math/<task>/` | Disposable notes, drafts and exploratory outputs. |
| `~/.codex/artifacts/` | Files prepared for Jörn to view, review or download; use a project/task subdirectory. |
| `.agents/skills/` in the checkout | Project-owned skills and their helpers. Installed plugin skills are separate. |
| `~/.cache/msc-math/artifacts/` | Downloaded research snapshots; [docs/artifacts.md](docs/artifacts.md) owns cache overrides and data links. |
| `/data/msc-math/` | Candidate future home for a project bundle to copy or mount; no relocation is selected yet. |

Keep lasting project results with their source/evidence owner. Scratch notes and
review files are not their only retained copy. See [environment docs](docs/development-environments.md)
for host/sandbox setup and [docs/codex.md](docs/codex.md) for Codex state and recovery.

File links for Jörn: `/absolute/path` → `https://joern-pc.tailc5e761.ts.net/files/absolute/path` (expand `~`; URL-encode each path segment).

For dashboard links, resolve `docs/dashboard/index.html` (coordination) or
`docs/dashboard/thesis.html` (thesis) against the intended checkout's absolute
root, then apply that URL rule. The page's data and source
links must resolve within the same checkout.

## Find the relevant source

Start from the task, then retrieve only the context it needs:

- `README.md` is the entry point; `ARCHITECTURE.md` maps domain ownership.
- `thesis/README.md` identifies the active reader-facing source and build. `formal/`
  contains proof development, `crates/` reusable Rust, and `experiments/` producers,
  retained evidence and interpretation.
- `docs/coordination/README.md` explains the agent-facing coordination store:
  `current.json` owns selected assignments/resources, `thesis-work.md` owns the
  detailed thesis backlog, and `handoff.md` owns continuation constraints.
  Read that README when coordinating or handing off work. `docs/project-facts.md`
  owns attributed Jörn-confirmed scope; `docs/knowledge/README.md` routes reusable
  interpretation. `docs/history/` owns historical evidence, not current assignments.
- `INSTALL.md` owns tool setup. Topic READMEs own reproduction commands and local caveats;
  `papers/` owns source literature and `submit/` release/admin.
- `docs/codex.md` routes Codex configuration, daemon restart/thread recovery,
  subagent persistence and official documentation or release-matched source lookup.

Search claims, symbols, artifact paths and likely synonyms with `rg` before concluding that
work is absent. Follow summaries to their sources. Legacy and generated trees are poor
broad orientation surfaces unless a task specifically needs them.

When assignments, owners, dependencies, decisions or resource constraints materially
change, update `docs/coordination/current.json`. Jörn's browser view at
`docs/dashboard/index.html` renders that state. Timestamp observations and show unknown
quota/spending as unknown; old panes, deadlines and balances are not live state.
The coordinating agent integrates updates from its workers; preserve other sessions' work.

When thesis-source or planning work materially changes selected claims, scope decisions,
uncertainty or route, reconcile `docs/dashboard/thesis.html` against its sources. If a
discrepancy cannot be reconciled promptly, mark the affected claim visibly stale and hand
off its status. The agent making the material change owns reconciliation; when several
agents contribute, the integrating agent owns it. Keep the synthesis concise, not an
activity log, and do not refresh it for unrelated changes.

Before calling that view current, cross-check Jörn's required thesis areas and PASS threshold,
the selected manuscript's promoted claims, current work and evidence owners. A promoted claim
needs a thesis location, caveat and PDF/evidence check; show any unresolved coverage gap.
Refresh `docs/dashboard/thesis-sources.json` only after that source reconciliation.

## Collaboration friction

Detailed workflow friction is owned by `docs/friction/README.md` and its individual
records; the linked Pages overview is the concise human view. When relevant authorized
work changes material friction or its disposition, update the record and reconcile that
Page without waiting for Jörn to request it. Keep diagnostics and handoffs in the repository.
If a surface cannot be updated, report its stale or unsaved state in the handoff. This does
not restart paused thesis work or establish ongoing monitoring.
## Collaboration and planning

Before sending a prepared user-visible message, use the
[message-review workflow](.agents/skills/message-review/SKILL.md) to obtain a separate
subagent's approval of the exact complete text, including `[review: REVIEWER]`.
Substantive changes require renewed review. Direct synchronous conversation and
private native agent messages are exempt; the skill owns the boundaries and procedure.
If required review is unavailable, keep prepared delivery pending.

Keep the Codex+user workflow moving: during unfinished authorized work, treat
brief acknowledgments such as "thx" as acknowledgment, not a stop, pause or scope
change. Continue the work rather than ending with a courtesy-only reply and
requiring Jörn to restart it. Finish when the requested work is complete, Jörn
explicitly stops it, or a required decision/input is clearly presented as pending;
do not invent additional work after completion.

Within selected work, use bounded subagent assignments and independent sessions
when they help discovery, planning or execution. Respect the actual tool's
authorization requirements. A delegation names its owned output, scope,
resource allowance, stopping/review point and receiving owner. Review ownership
does not imply implementing fixes; agree who integrates, commits and cleans up.

The coordinator owns workload supervision: check pending messages/reviews,
ownership and dependencies alongside available usage telemetry. During ongoing
multi-agent work, delegate a bounded periodic watch when useful; do not leave
Jörn to discover overload or abandoned work. The coordination README documents
the local checker. Missing telemetry and quiet threads do not establish idle
capacity; token counters alone do not measure coordination burden.

Obtain Jörn's explicit approval before large budget expenditure. Fund useful
preliminary exploration and comparison of approaches within the authorized
scope; present a concrete plan, alternatives, expected outcomes, uncertainty and
resource expectations before committing to substantial execution. Thresholds
are not yet agreed: do not invent a dollar/token allowance or a gate for every
small task. Splitting work across agents, sessions or stages does not enlarge
the approved scope or budget.

The async question tool is disabled by project instruction pending verified
delivery: on 30 September it reported accepted submissions that Jörn says he
never received. Put questions directly in chat. A submission acknowledgment is
not evidence of user receipt; do not silently wait behind an undelivered
question. For a required approval, keep the dependent package waiting through
ownership changes and record the decision in coordination. Silence, automatic
action approval and elapsed time are not Jörn's plan approval. An optional
clarification does not create a new approval gate; preserve its stated fallback.

Read [the collaboration commentary](AGENTS.md.commentary.md) when choosing an
experiment/research programme, preparing a spending proposal, or designing
delegation and human review. It owns curated rationale and open choices, not a
second task list. [The knowledge index](docs/knowledge/README.md) routes further
process and domain knowledge by the decision being made.

## Respect evidence boundaries

```text
paper -> formal statement/proof -> implementation/experiment -> thesis prose
```

Check every relevant arrow. A paper note is not a project result; a formal argument is not
automatically in the thesis; a test or finite experiment does not prove an unrestricted
theorem; exact arithmetic does not supply a missing implication; and a LaTeX build does not
validate mathematics or prose. State conclusions at the strength their sources support.

Generated evidence is owned by its producer. Commands using `--full`, many `cargo run`
producers and certificate workflows can be expensive or replace retained evidence; inspect
their inputs and write effects before rerunning them.

## Commands

Run from the repository root unless a local README says otherwise. Choose the relevant entry
point rather than treating this as one test suite:

```bash
scripts/repo-status-summary.sh  # whether dated checks still apply
cargo fmt --all -- --check      # Rust formatting
cargo check --workspace         # root-workspace compile smoke
uv run path/to/script.py        # ordinary Python with inline dependencies
sh thesis/build.sh               # selected thesis
(cd formal && latexmk)           # proof-development document
```

`docs/algorithm-testing.md` maps focused Rust checks to stronger confidence layers. Local
READMEs own theorem certificates, Sage commands, full producers and comparison contracts.
Sage uses its separate environment.
