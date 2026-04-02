# Task: Migrate experiments to use the polytope database

## Context

The `crates/database/` crate (branch `database-implementation`) provides shared caching of
(polytope, capacity, sigmas) across experiments. The main payoff: when the capacity algorithm
changes, one pass recomputes capacity/sigmas for all polytopes in `data/polytopes.jsonl`, and
every experiment picks up the new values without redundantly rerunning vertex enumeration or
polytope generation. Without this, 10+ heterogeneous experiment binaries each call the capacity
algorithm their own way, and any agent trying to understand what the experiments conclude after
an algorithm migration will get confused and fail.

## Scope

Each experiment binary should be restructured into three phases:

1. **Declare polytopes** — the experiment specifies which polytopes it needs (static set or
   dynamic loop). This is just a HashMap lookup: `db.get(&key)` or `db.entry(key)`.

2. **Read cached data** — capacity, volume, sigmas come from the database record. If missing,
   compute and insert. This is where capacity algorithm calls get centralized — ideally in
   a shared population step, not scattered across experiment binaries.

3. **Compute experiment-specific quantities** — from the cached (polytope, capacity, sigmas),
   derive whatever the experiment needs: orbit geometry, gradients, ω₀ features, convergence
   slopes, etc. This should be much cheaper than rerunning the full EHZ algorithm.

### Migration candidates, ordered by impact

**Tier 1 — reconstruct polytopes from other experiments' JSONL** (vertex enumeration redo):
- `exp-sys-optimization/combinatorial-structure/run.rs` — loads 170 polytopes from
  random-sweep + random-product-sweep JSONL, calls `from_f64()` on each. Lines 1106-1127.
- `exp-sys-optimization/boundary-crossing-search/run.rs` — loads warm starts from
  gradient-descent.jsonl, calls `from_f64()`. Lines 741-750.
- `exp-sys-optimization/gradient-search/run.rs` — loads seeds.jsonl, calls `from_f64()`
  per seed. Line 116. (Note: this experiment is superseded by sys-search, may be deleted.)

**Tier 2 — recompute capacity on shared polytopes:**
- `exp-hko-local-maximum/omega-hypothesis/run.rs` — HKO pentagon + `all_known()` + random
  (SEED=42, F=5..10). Needs full `EhzResult` for orbit extraction.
- `exp-capacity-axioms/orbit-recovery/run.rs` — `all_known()` + random (SEED=42, F=5..10).
  Needs full `EhzResult` for orbit recovery validation.
- `exp-numerical-analysis/q-error/run.rs` — `all_known()`. Runs `ehz_capacity()` on each.
- 5 experiments independently compute capacity on the HKO pentagon.

**Tier 3 — same-seed random polytopes overlap:**
- random-sweep, omega-hypothesis, orbit-recovery all use SEED=42 with overlapping F ranges.

### Dynamic-loop experiments

Gradient descent experiments (gradient-search, large-scale-descent, boundary-crossing-search,
sys-search) modify dual vertices each iteration, producing new polytopes dynamically. The
database supports this: `db.get(&key)` returns `None` for a new polytope, the experiment
computes capacity and inserts it. If the optimizer revisits a polytope (backtracking), the
cached result is there. The "declare + read" pattern is just HashMap lookup, same API as
static-set experiments.

## Out of scope

- **Do NOT change the database crate API** unless a concrete experiment needs it. The current
  API (load/save/from_polytope/to_polytope/with_computed_fields/with_sigmas) covers the
  known use cases.
- **Do NOT create a separate "population binary"** upfront. Let the first migrated experiment
  drive what's needed. If a shared tool emerges naturally, factor it out then.
- **Do NOT migrate all experiments at once.** Pick one Tier 1 experiment, migrate it, verify
  output is unchanged, then proceed.

## Key files

- `crates/database/src/lib.rs` — API docs at top, implementation below
- `crates/library/src/geom/polytope.rs` — `from_rational_parts` constructor (line ~240)
- `crates/library/src/random.rs` — `generate_polytope` with blake3 seeding (line ~66)
- `data/polytopes.jsonl` — database file (currently empty)
- `handoffs/database-implementation.md` — original design spec (types, API, architecture)

## Prior findings

- **PolytopeRecord stores scalar capacity, not full EhzResult.** Experiments like
  omega-hypothesis need `best_beta`, `best_perm`, `best_q` for orbit geometry. The
  `SigmaAction` stores `perm + action` but not `beta`. For these experiments, the
  migration path is: load polytope from database (skip vertex enumeration), then
  solve the KKT for just the cached sigma permutation (cheap, single-perm solve)
  to recover beta. This is much cheaper than full EHZ which enumerates all permutations.

- **BigRational serialization uses "numer/denom" strings**, not the opaque default.
  Human-readable, `jq`/Python compatible.

- **JSONL + git merge works.** One-record-per-line means git handles the common case
  (two branches appending different records) automatically. Conflicts only arise if
  both modify the same record, which append-only usage avoids.

- **No existing experiment stores BigRational.** All 49 data files use f64 exclusively.
  The database is new infrastructure — it doesn't retrofit existing data files.

- **14 pre-existing test failures** in `algorithms::billiard` and `algorithms::hk2017`
  on main. Unrelated to database work.

## Branch state

Branch `database-implementation` (worktree at `.claude/worktrees/database-implementation`).
Rebased onto current main. Three commits:
1. `Implement polytope database crate` — all three pieces (from_rational_parts, database
   crate, generate_polytope)
2. `Use human-readable 'numer/denom' string format for BigRational serde`
3. `Expand database module docs: design rationale, API overview, usage pattern`

Merge this branch to main first, then start experiment migration on a new branch.

## Success criteria

For the database branch (ready to merge now):
- `cargo build --workspace --release` — passes
- `cargo test --release --lib -p symplectic` — 315 pass, 14 pre-existing failures, 30 ignored
- `cargo test -p database --release` — 7 pass
- `cargo clippy --lib -- -D warnings` — clean
- `cargo clippy -p database -- -D warnings` — clean

For experiment migration (future work):
- Each migrated experiment produces identical output JSONL/CSV
- `data/polytopes.jsonl` populated with all polytopes used across experiments
- No experiment binary calls `ehz_capacity()` directly — capacity comes from database
  or from a shared population step
- Capacity algorithm can be swapped by changing one place + rerunning population
