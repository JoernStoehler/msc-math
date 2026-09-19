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
- `docs/project-facts.md` records Jörn-confirmed facts and scope decisions;
  `docs/WORK_REMAINING.md` records current work. `RESUME.md` records the current
  handoff; `docs/history/` contains superseded plans. Dated plans and old deadlines
  are not current assignments.
- `INSTALL.md` owns tool setup. Topic READMEs own reproduction commands and local caveats;
  `papers/` owns source literature and `submit/` release/admin.

Search claims, symbols, artifact paths and likely synonyms with `rg` before concluding that
work is absent. Follow summaries to their sources. Legacy and generated trees are poor
broad orientation surfaces unless a task specifically needs them.

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
