# All-Minimum Validation: Logbook

## Motivation

This packet isolates the sigma-side question that the result-layer refactor was
meant to unlock:

- for a diverse local-first pool of polytopes, which minimum-action simple
  orbits does the shared HK2017 result layer return?
- do the returned minimum-action values agree with the scalar
  `ehz_capacity(...)` route?

This packet does **not** test geometric orbit recovery. It produces the trusted
minimum-sigma rows that the separate orbit-recovery packet consumes.

## Status

**Split from orbit-recovery on 2026-04-17.** The experiment now owns the
minimum-set computation and trusted sigma dataset.

The binary has two run modes:

- default invocation writes untracked smoke outputs for infrastructure checks;
- `--full` refreshes the canonical local-first dataset.

Output surface:

- `all-minimum.jsonl`: one summary row per polytope
- `all-minimum-orbits.jsonl`: one detail row per trusted minimum orbit
- smoke variants mirror those names with `smoke-` prefixes

## How to run

```bash
cargo run -p dev-capacity-validation --release --bin axioms-all-minimum
cargo run -p dev-capacity-validation --release --bin axioms-all-minimum --full
uv run analyze.py
uv run analyze.py --smoke
```

### Files

| File | Role |
|------|------|
| `main.rs` | Rust binary: minimum-set computation and scalar cross-check |
| `analyze.py` | Python: summary statistics from polytope-level and per-orbit outputs |
| `all-minimum.jsonl` | Canonical polytope-level summary dataset |
| `all-minimum-orbits.jsonl` | Canonical trusted minimum-orbit dataset |
| `smoke-all-minimum.jsonl` | Untracked smoke summary dataset |
| `smoke-all-minimum-orbits.jsonl` | Untracked smoke per-orbit dataset |

## Design

### Dataset policy

The packet optimizes for **diversity first, size second**.

Canonical `--full` pool:

- known polytopes from `known_polytopes::all_known()`, excluding the
  crosspolytope;
- one random shared-cache representative per facet-count stratum;
- one lagrangian-product shared-cache representative per polygon-pair stratum;
- one correctness-derived row from each of the `scaled`, `transformed`, and
  `perturbed` groups.

Targets are deduplicated by polytope geometry so repeated local sources do not
inflate the dataset without adding diversity.

Smoke pool:

- `simplex`, `hypercube`, `lagrangian_triangle_product`,
  `random_F5_seeded`, and one transformed correctness row.

### Computation policy

For every selected polytope:

1. enumerate pruned HK2017 sigma candidates directly;
2. solve each sigma with `solve_orbit_sigma(...)`;
3. aggregate with `OrbitGuaranteeMode::MinimaSafe`;
4. filter the resulting candidate set to observed action ties
   `<= min_action + 1e-12`;
5. cross-check the minimum action against `ehz_capacity(...)`.

Important boundary:

- shared caches are **polytope sources only**;
- this packet never trusts cached minimum sigmas as ground truth;
- the trusted sigma rows written here are the intended inputs to
  `experiments/verification/orbit-recovery/`.

### Validation procedure

Polytope-level summary rows record:

- minimum-orbit count
- observed minimum-action spread across the retained tie set
- raw candidate interval width from the broader `MinimaSafe` collector
- admissible `f64` and exact-resolved minimum counts
- scalar agreement with `ehz_capacity(...)`
- timings for minimum-set and scalar-check stages

Known multiplicity checks currently asserted from local evidence:

- `simplex`: 6 minimum orbits
- `hypercube`: 2 minimum orbits

## Current local-first results

Canonical run on 2026-04-17:

- `28/28` selected polytopes pass
- `469` trusted minimum orbits total
- largest observed sigma-level minimum set:
  `hko_pentagon = 412`
- documented multiplicities hit:
  `simplex = 6`, `hypercube = 2`

## Current limitations

- The packet counts sigma-level minima as returned by the shared result layer;
  it does not quotient by symmetry classes or geometric-equivalence classes.
- The canonical local-first pool is intentionally diverse, not exhaustive over
  every local artifact.
- The scalar cross-check compares minimum values, not whole minimum sets across
  alternative algorithms.
