# Handoff: Panics Convention (incomplete)

## Status

A draft exists in `.claude/rules/rust.md` under "## Panics" on the `gradient-correctness` branch. It has NOT gone through the full agent-design workflow (steps 1-3 skipped). Jörn dictated the core content but the design process was not followed.

## What Jörn said (verbatim summary)

- We use types with semantic meaning
- We write correct code
- We prove correctness by code-math correspondence, e.g. via types
- We use standard Rust patterns (error types, asserts) to return errors to be handled
- We panic when something is wrong in a bigger context: a lemma proven wrong by code, deferred work that needs doing now
- The panic indicates "this is not about the local code but about the external context"
- Panics bubble up all the way (to Jörn)
- Code comments need to be very informative about the context and why behind a panic

## What needs doing

1. Run agent-design steps 1-3 (gather situations, supply info, Jörn decides approach)
2. Revise the draft based on the proper design process
3. Run verification (step 5) — one naive subagent test was done, passed
4. Fix all convention violations (tracked in TASKS.md `convention-violations`)

## Known violations

- `experiments/gradient-correctness/run.rs`: 2× catch_unwind
- `experiments/hko-neighborhood/run.rs`: 4× catch_unwind
- Panic comments in saddle_point_solver.rs and capacity_accumulator.rs need improvement
