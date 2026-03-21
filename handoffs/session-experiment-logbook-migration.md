# Session: Experiment Logbook Migration

**Goal:** Migrate 15 remaining experiments to the logbook format established by hko-neighborhood.

**Worktree:** Yes. Branch from local `main`.

## Context

hko-neighborhood is done (logbook.md + math.tex + role-based filenames). The remaining 15 experiments need the same treatment. This is mechanical work, parallelizable via subagents.

## Experiments to migrate

1. ablation
2. benchmark
3. correctness
4. crosspolytope
5. gradient-descent
6. kkt-inertia
7. lagrangian-products
8. omega-obstacle
9. orbit-recovery
10. pentagon-perturb
11. q-error
12. random-product-sweep
13. random-sweep
14. rejection-sampling
15. sys-optimization
16. unknown-predicates
17. visualization

(Check which actually exist — some may be stale or empty.)

## Per-experiment checklist

1. Read existing README.md, .tex, code headers, git history for context
2. Create `logbook.md` from README.md content + .tex prose + code header comments
3. Rename `<name>.rs` → `run.rs`
4. Rename `<name>.py` → `analyze.py` (if exists)
5. Rename `<name>.tex` → `math.tex` (if exists)
6. Update `experiments/Cargo.toml` `[[bin]]` paths for renamed .rs files
7. Update any `thesis/*.tex` `\input` paths that reference renamed .tex files
8. Delete `README.md`
9. Verify `cd experiments && cargo build` succeeds after renames

## Skills to load

- `experiment-conventions` — directory structure, logbook format, role-based names
- `math-tex` — math.tex conventions (for renamed .tex files)

## Parallelization

Dispatch one subagent per 3-4 experiments. Each reads the experiment-conventions skill. Each surfaces questions it can't resolve (e.g. conflicting README vs data content).

## Known pitfall

The hko-neighborhood logbook agent found stale README content by cross-checking against JSONL data. Other experiments likely have similar staleness. When sources conflict, flag the conflict explicitly and check the data (JSONL/output files) as ground truth.

## Deliverable

- All changes committed on the worktree branch (one commit per experiment or per batch)
- Report: which experiments migrated cleanly, which had conflicts/questions
- `cargo build` from `experiments/` must succeed
