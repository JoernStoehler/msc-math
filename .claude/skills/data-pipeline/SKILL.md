---
name: data-pipeline
description: How to handle expensive test data — caching, fixture generation, staleness detection, and offloading slow computation to the LICCA cluster. Load when writing tests that need precomputed data (e.g., polytope datasets) or when tests are too slow because they regenerate data on every run.
---

# Data Pipeline Conventions

## Caching strategies

| Strategy | When to use | Example |
|----------|-------------|---------|
| **No caching** | Fast (<10ms) or always stale | `Polytope4D::new()` from literals |
| **In-memory** | Shared across tests, 10ms–1s | `LazyLock<KnownPolytope>` |
| **On-disk** | Expensive (>10s), consumed often | Capacity fixtures (33 polytopes) |

### On-disk caching

```
crates/tests/fixtures/capacity_dataset.json   # checked into git
crates/src/algorithms/hk2017/generate_capacity_fixtures.rs  # generator
```

Two loading tiers:
- `load_dataset_entries()` → scalar fields only, ~1ms
- `load_test_dataset()` → full `Polytope4D` reconstruction, ~8s

Generation stages are independent. Only regenerate what's stale.

## Staleness detection

Cached data includes expected values. Consumer tests recompute and compare:
```
Capacity mismatch for simplex: cached=0.125, computed=0.126.
Regenerate: cargo test --release --lib -- regenerate_test_dataset --ignored
```

Only regenerate when the generator's *semantic output* would change. Run consumer tests first — if they pass, cached data is still correct.

## Test performance budget

| Category | Target |
|----------|--------|
| `cargo test --release --lib` | < 5s wall |
| `cargo test --lib` (debug) | < 20s wall |
| Full suite (`--ignored`) | < 10 min |

Tests exceeding budget: `#[ignore]` with comment explaining why and runtime.

Default is `cargo test --release --lib`. All `debug_assert!` promoted to `assert!` so release loses no safety.

## LICCA cluster offloading

For computations >10 min: agent writes binary + SLURM script, Jörn submits via `ssh licca && sbatch job.sh`. Results committed back.

Offload when inherently O(n!) / O(2^n). Optimize locally when algorithmic speedup is feasible.

## Discovering pipelines

```bash
grep -rn '#\[ignore\]' crates/src/ | grep -i 'generat\|fixture\|dataset'
```

Generators name consumers, consumers name data source.
