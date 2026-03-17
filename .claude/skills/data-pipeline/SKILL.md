---
name: data-pipeline
description: Test data pipeline conventions — caching strategies, fixture generation, staleness detection, timing, and LICCA cluster offloading. Load when writing tests that consume expensive data, or when test runtime needs optimization.
---

# Data Pipeline Conventions

## Design principle

Balance marginal value of more compute against marginal friction to future agents. Profile before optimizing. One-time runs with large parameters are fine — the friction source is tests that run on every `cargo test --lib` and take too long.

## Caching strategies

Choose the cheapest strategy that avoids fatal staleness.

| Strategy | When to use | Example |
|----------|-------------|---------|
| **No caching** | Data is fast (<10ms) or always stale when tests change | `Polytope4D::new()` from literal normals/heights |
| **In-memory** | Same data used by multiple tests in one binary, construction is moderate (10ms–1s) | `known_polytopes::simplex()` via `LazyLock` |
| **On-disk** | Generation is expensive (>10s), consumed more often than regenerated | Capacity fixtures (33 polytopes × full enumeration) |

### No caching

Inline construction. No special infrastructure needed.

```rust
let polytope = Polytope4D::new(normals, heights).unwrap();
```

### In-memory caching

Use `std::sync::LazyLock` for expensive-to-construct test fixtures shared across tests:

```rust
static SIMPLEX: LazyLock<KnownPolytope> = LazyLock::new(|| {
    KnownPolytope { polytope: Polytope4D::from_dual_vertices(...).unwrap(), ... }
});
```

**When to use:** The same polytope constructor appears in 5+ tests and takes >10ms. Current candidates: `known_polytopes::simplex()`, `hypercube()`, `hko_pentagon()`, `lagrangian_triangle_product()`.

### On-disk caching

Expensive computations write results to a checked-in data file. Tests load the file instead of recomputing.

**Structure:**
```
crates/src/algorithms/hk2017/
├── generate_capacity_fixtures.rs   # generation stage (#[ignore] test)
├── fixtures/                        # generated data (checked into git)
│   └── capacity_fixtures.json
├── literature_test.rs               # consumes fixtures
├── conformality_test.rs             # consumes fixtures
└── ...
```

**Generation stages are independent.** Each generates exactly one dataset. Never bundle them — an agent should only regenerate what's actually stale, not blow 30 minutes on all pipelines indiscriminately.

**Generation stage:**
- Single concern: generates exactly one dataset
- Runnable via `cargo test --lib -- generate_capacity_fixtures --ignored`
- Prints timing: "Generated 33 polytopes in 4m32s"
- Writes to a deterministic path under `fixtures/`

**Consumer tests:**
- Load fixtures at test start
- Check staleness (see below)
- Run fast assertions against pre-computed data

## Staleness detection

Two layers, complementary:

### Semantic staleness (primary)

The cached data includes expected values. If a code change produces different values, the consumer test fails. This is both regression detection AND staleness detection:

- If the old values were correct → the code change is a regression (bug)
- If the old values were wrong → the data was already stale (regenerate)

The consumer test can't distinguish these. It reports the mismatch with an actionable message:
```
Capacity mismatch for simplex: cached=0.125, computed=0.126.
If this is a legitimate code change, regenerate:
  cargo test --lib -- generate_capacity_fixtures --ignored  (~5 min)
If this is unexpected, investigate the regression.
```

### Generator-content staleness (secondary)

The fixture file includes a content hash of the generator source. On load, compare against current file. If the generator changed but data wasn't regenerated, warn:
```
WARNING: generate_capacity_fixtures.rs changed since last generation.
Consider regenerating: cargo test --lib -- generate_capacity_fixtures --ignored  (~5 min)
```

This is a WARNING, not an error — the generator might have changed in ways that don't affect output (comments, refactoring). The semantic check above catches actual mismatches.

**Why not git commit hash?** Too many false positives. Every commit marks everything stale, even unrelated changes.

## Cached data as regression detection

On-disk cached values serve double duty: they're both test fixtures AND regression baselines. When capacity computation code changes:

1. Consumer test loads cached `simplex_capacity = 0.125`
2. Test recomputes capacity and compares
3. Mismatch → either regression or legitimate change requiring re-validation

This is the cheapest regression test: load JSON + one assertion. Expensive recomputation only happens when values actually change.

## Timing collection

Always collect timing by default in expensive tests. A few milliseconds of overhead prevents expensive re-runs just to measure.

```rust
let start = std::time::Instant::now();
let result = expensive_computation();
let elapsed = start.elapsed();
eprintln!("[TIMING] {test_name}: {elapsed:.1?}");
```

**Why:** When a test takes 30s, you need to know whether it's the fixture load (0.1s), the capacity computation (29s), or the assertion (0.9s). Without timing, you re-run the whole thing to find out.

## Test performance budget

| Category | Target | Current |
|----------|--------|---------|
| Fast tests (default suite) | < 2 min | ~7 min (needs work) |
| Full suite (with `--ignored`) | < 10 min | unknown |
| Fixture generation | < 10 min | ~5 min |

Tests exceeding the fast budget should be `#[ignore]` with a comment:
```rust
#[ignore] // ~30s: full permutation enumeration on 33 polytopes. Run with --ignored.
```

**When to regenerate vs not:** Only regenerate when the generator's *semantic output* would change. A rounding change in display code doesn't warrant 5 minutes of regeneration. A KKT solver bugfix does. When in doubt, run the consumer tests first — if they pass, the cached data is still correct.

## Discovering data pipelines

To find all data generation stages in the codebase:
```bash
grep -rn '#\[ignore\]' crates/src/ | grep -i 'generat\|fixture\|dataset'
```

Each generation stage's doc comment should name its consumers:
```rust
//! Generates capacity fixtures for 33 polytopes.
//! Consumers: literature_test.rs, conformality_test.rs, symplectic_invariance_test.rs
```

And each consumer should name its data source:
```rust
//! Consumes: fixtures/capacity_fixtures.json
//! Regenerate: cargo test --lib -- generate_capacity_fixtures --ignored  (~5 min)
```

## LICCA cluster offloading

For computations exceeding 10 minutes (large sweeps, full dataset generation):

1. Agent writes the Rust binary and a SLURM job script (from template)
2. Agent presents both to Jörn with: what it computes, expected runtime, output path
3. Jörn opens a terminal, builds, and submits: `ssh licca && sbatch job.sh`
4. Results are copied back to the repo and committed

**Template location:** `experiments/slurm/template.sh` (TODO: create)

**When to offload vs optimize locally:**
- If the computation can be made 10x faster with algorithmic changes → optimize locally
- If it's inherently O(n!) or O(2^n) on large inputs → offload
- If it runs once to generate a dataset that's consumed many times → offload

## Anti-patterns

- **Blanket regeneration:** `just generate-all-fixtures` runs all pipelines indiscriminately, wasting 30 minutes when only one dataset is stale. Each pipeline is independent — regenerate only what changed.
- **Magic regeneration:** Test silently regenerates stale fixtures. Future agent doesn't know it's spending 5 minutes on regeneration vs 5 seconds on the actual test.
- **Fixture in test body:** Large JSON literal in a test function. Hard to update, bloats the test file.
- **Staleness via git hash:** `assert_eq!(git_hash, "abc123")` — every commit triggers regeneration even when the generator didn't change.
- **No timing:** Test is "slow" but nobody knows which part. Agents add `#[ignore]` instead of optimizing the bottleneck.
- **Regenerating for cosmetic changes:** Capacity values rounded differently doesn't justify 15 core-hours of regeneration. Do this once before publication to update all figures.
