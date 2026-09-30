# Project guidance

## Purpose

This is Jörn Stöhler's MSc thesis repository on probing Viterbo's conjecture through
computational symplectic geometry. It combines mathematical arguments,
proof-by-computation, reusable Rust code, experiments, data and a reader-facing thesis.
The project has several related results rather than one forced narrative.

A finished thesis needs claims and caveats that match their mathematical and computational
support, resolvable code/data/figure/certificate references, and truthful reproduction
promises. Compilation and tests cover only parts of that outcome.

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

## Collaboration and planning

Within selected work, use bounded subagent assignments and independent sessions
when they help discovery, planning or execution. Respect the actual tool's
authorization requirements. A delegation names its owned output, scope,
resource allowance, stopping/review point and receiving owner. Review ownership
does not imply implementing fixes; agree who integrates, commits and cleans up.

Obtain Jörn's explicit approval before large budget expenditure. Fund useful
preliminary exploration and comparison of approaches within the authorized
scope; present a concrete plan, alternatives, expected outcomes, uncertainty and
resource expectations before committing to substantial execution. Thresholds
are not yet agreed: do not invent a dollar/token allowance or a gate for every
small task. Splitting work across agents, sessions or stages does not enlarge
the approved scope or budget.

Use an async question when useful independent authorized work remains. For a
required approval, keep the dependent package waiting even if its owner changes
or the question disappears from the UI. Record the pending decision in
coordination; silence, automatic action approval and elapsed time are not
Jörn's plan approval. An optional clarification does not create a new approval
gate; preserve any stated fallback.

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
