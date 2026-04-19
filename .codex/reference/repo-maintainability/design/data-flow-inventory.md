<!--
Purpose: durable data-flow and cache inventory for the repo maintainability / architecture program.
Context: discovery packet D3. This note records observed producers, consumers, mirror candidates, and transient outputs without changing any committed JSONL values. Canonical-dataset policy stays open.
-->

# Data-Flow And Cache Inventory

## Status

- Last updated: 2026-04-16.
- Phase: discovery only.
- Policy decision: open.
- Stop condition: if a cleanup would change committed data values, keep the note descriptive and do not propose regeneration as a fix.

## Method / Evidence

Commands used:

- `cd /workspaces/msc-math && pwd`
- `cd /workspaces/msc-math && sha256sum experiments/sys-landscape/cache.jsonl experiments/combinatorial-cells/polytopes.jsonl experiments/verification/orbit-recovery/polytopes.jsonl`
- `cd /workspaces/msc-math && rg -n "owned_db_path|save\\(&owned_db_path|load_many\\(&\\[owned_db_path.as_path\\(\\)\\]\\)" experiments/combinatorial-cells/*/main.rs experiments/verification/orbit-recovery/main.rs`
- `cd /workspaces/msc-math && rg -n "save\\(&family_cache_path|load_many\\(&\\[family_cache_path.as_path\\(\\)\\]\\)" experiments/sys-landscape/*/main.rs`
- `cd /workspaces/msc-math && rg -n "cache\\.jsonl|polytopes\\.jsonl|orbit-recovery\\.jsonl|gradient-ascent-general\\.jsonl|gradient-ascent-products\\.jsonl|variable-f-ascent\\.jsonl" experiments/*/*/analyze.py experiments/*/*/main.rs`
- `cd /workspaces/msc-math && nl -ba crates/symplectic/src/database.rs | sed -n '1,260p'`
- `cd /workspaces/msc-math && nl -ba experiments/verification/orbit-recovery/main.rs | sed -n '1,220p'`
- `cd /workspaces/msc-math && nl -ba experiments/combinatorial-cells/omega-hypothesis/main.rs | sed -n '1,220p'`
- `cd /workspaces/msc-math && nl -ba experiments/sys-landscape/random-sample/main.rs | sed -n '1,220p'`
- `cd /workspaces/msc-math && nl -ba experiments/sys-landscape/src/lib.rs | sed -n '470,620p'`
- `cd /workspaces/msc-math && nl -ba experiments/sys-landscape/variable-f-ascent/main.rs | sed -n '540,760p'`

Key evidence:

- `crates/symplectic/src/database.rs` says there is no canonical mutable shared cache path and that callers own path policy. See [database.rs](</workspaces/msc-math/crates/symplectic/src/database.rs:1>).
- The three candidate shared-cache files had identical SHA-256 hashes on 2026-04-16.
- `experiments/sys-landscape/src/lib.rs` documents the append/resume/canonicalization rules for local output files, not for the polytope cache itself. See [sys-landscape lib](</workspaces/msc-math/experiments/sys-landscape/src/lib.rs:485>).

## Dataset Inventory

### Shared Catalog

| path | role candidate | producer | consumer(s) | trusted fields | drift risk |
| --- | --- | --- | --- | --- | --- |
| `experiments/combinatorial-cells/polytopes.jsonl` | strongest shared-catalog candidate | `omega-hypothesis` can write back missing rows; other combinatorial-cells generators also load it | `cell-widths`, `boundary-characterization`, `multiple-crossings`, `convexity`, `omega-hypothesis` | `dual_vertices_rational`, `vertices_rational`, `source`, `volume`, `capacity`, `sigmas`, `sigma_gap_cutoff` | a schema or row-value change would affect every combinatorial-cells consumer; current code trusts cached `capacity`/`sigmas` when present |

Observed entry points:

- `omega-hypothesis` loads and may save the file. See [omega-hypothesis/main.rs](</workspaces/msc-math/experiments/combinatorial-cells/omega-hypothesis/main.rs:17>) and [save site](</workspaces/msc-math/experiments/combinatorial-cells/omega-hypothesis/main.rs:496>).
- Other combinatorial-cells binaries only load it. See [cell-widths/main.rs](</workspaces/msc-math/experiments/combinatorial-cells/cell-widths/main.rs:15>), [boundary-characterization/main.rs](</workspaces/msc-math/experiments/combinatorial-cells/boundary-characterization/main.rs:15>), and [multiple-crossings/main.rs](</workspaces/msc-math/experiments/combinatorial-cells/multiple-crossings/main.rs:14>).

### Mirror

| path | role candidate | producer | consumer(s) | trusted fields | drift risk |
| --- | --- | --- | --- | --- | --- |
| `experiments/sys-landscape/cache.jsonl` | mirror candidate of the shared polytope catalog | `random-sample`, `random-product-sample`, `gradient-ascent-general`, `gradient-ascent-products` all load and save it | those same four binaries, plus `variable-f-ascent` as a local cache source | `dual_vertices_rational`, `vertices_rational`, `source`, `volume`, `capacity`, `sigmas` | if any writer diverges from the shared catalog, future cache hits can silently change the accepted polytope set or cached orbit permutation |
| `experiments/verification/orbit-recovery/polytopes.jsonl` | mirror candidate of the shared polytope catalog | `axioms-orbit-recovery` loads and saves it | `orbit-recovery/main.rs` and `orbit-recovery/analyze.py` | `dual_vertices_rational`, `vertices_rational`, `source`, `capacity`, `sigmas` | orbit recovery trusts cached `capacity + sigmas.first().perm` to bypass full EHZ; stale or divergent rows change the fast path |

Observed hash evidence:

- `experiments/sys-landscape/cache.jsonl`
- `experiments/combinatorial-cells/polytopes.jsonl`
- `experiments/verification/orbit-recovery/polytopes.jsonl`

All three hashed to `8679b89763a10bf1380410f288845f03bcdc8e365035aa31235ff00c9cc07363` on 2026-04-16.

### Topic-Local Transient

| path | role candidate | producer | consumer(s) | trusted fields | drift risk |
| --- | --- | --- | --- | --- | --- |
| `experiments/sys-landscape/variable-f-ascent/cache.jsonl` | local cache, not shared catalog | `variable-f-ascent` writes it; smoke mode writes a temp cache under the system temp directory | only `variable-f-ascent` | `dual_vertices_rational`, `vertices_rational`, `source`, `volume`, `capacity`, `sigmas` | this file accumulates intermediate gradient-step polytopes and is explicitly kept local so the shared cache does not grow with transient states |
| `experiments/sys-landscape/variable-f-ascent` temp smoke files | throwaway smoke outputs | `smoke_run()` writes `variable-f-ascent-smoke.jsonl` and a temp `cache.jsonl` under `std::env::temp_dir()` | only the smoke path | same as above, but only for the smoke run | low long-term risk because the paths are ephemeral; high risk if a cleanup script accidentally targets the temp location while a run is active |

### Analysis Output

| path | role candidate | producer | consumer(s) | trusted fields | drift risk |
| --- | --- | --- | --- | --- | --- |
| `experiments/verification/orbit-recovery/orbit-recovery.jsonl` | analysis output | `axioms-orbit-recovery` | `orbit-recovery/analyze.py`, `plot_orbit_recovery.py` | `source`, `capacity`, `active_facets`, `solution_dim`, `max_violation`, `closure_error`, `on_facet_error`, `action_error`, `time_capacity_ms`, `time_recovery_ms` | downstream scripts assume the row schema and compare error fields against fixed tolerances |
| `experiments/combinatorial-cells/omega-hypothesis/omega-obstacle.jsonl` | analysis output | `cell-omega` | `omega-hypothesis/analyze.py` | `source`, `facet_count`, `sys`, `orbit_*`, `ridge_*`, `gradient_dots` | analyzer splits `source` by prefix and depends on the present orbit/ridge feature columns |
| `experiments/combinatorial-cells/boundary-characterization/combinatorial-boundaries-{anatomy,crossing,gradient}.jsonl` | analysis outputs | `boundary-characterization` | `boundary-characterization/analyze.py`, `gradient-discontinuity/analyze.py` | `event_type`, `t_max`, `sys`, `orbit_gap`, `direction_type` | these files are separate projections of one run; a schema drift in one file can break the shared plotting code |
| `experiments/combinatorial-cells/cell-widths/combinatorial-boundaries-profiling.jsonl` | analysis output | `cell-widths` | `cell-widths/analyze.py`, `gradient-discontinuity/analyze.py` | `facet_in_orbit`, `t_max`, `event_type`, `facet_count` | downstream plots treat `t_max < 100` as finite width and infer orbit-vs-non-orbit differences from that field |
| `experiments/combinatorial-cells/multiple-crossings/combinatorial-boundaries-sweep.jsonl` | analysis output | `multiple-crossings` | `multiple-crossings/analyze.py` | `direction_type`, `n_boundaries`, `sys_values`, `event_types` | the sweep is only useful if the boundary-event schema stays consistent |
| `experiments/sys-landscape/random-sample/random-sweep.jsonl` | analysis output | `random-sample` | `random-sample/analyze.py` | `volume`, `capacity`, `sys`, `iterations`, `h_min`, `h_max` | cache hits set `iterations = 0`, so the analyzer must keep treating those rows as cache-backed |
| `experiments/sys-landscape/random-product-sample/random-product-sweep.jsonl` | analysis output | `random-product-sample` | `random-product-sample/analyze.py` | `k`, `m`, `volume`, `capacity`, `sys`, `bounces` | row schema is keyed to the product family; it is not a general polytope cache |
| `experiments/sys-landscape/gradient-ascent-general/gradient-ascent-general.jsonl` and `experiments/sys-landscape/gradient-ascent-products/gradient-ascent-products.jsonl` | analysis outputs and resume inputs | `gradient-ascent-general` / `gradient-ascent-products` | `variable-f-ascent` loads `gradient-ascent-general.jsonl`; each package also maintains a companion trace JSONL | summary rows carry `name` and `final_sys`; the companion trace files carry `(name, phase, iteration)` after canonicalization | these files behave like published run artifacts, not caches; the replay logic assumes canonicalized row order |
| `experiments/sys-landscape/variable-f-ascent/variable-f-ascent.jsonl` | analysis output | `variable-f-ascent` | `variable-f-ascent/analyze.py` | `name`, `starting_f`, `final_sys`, `delta_vs_source`, `final_dual_vertices` | the output is coupled to the resume story and to the upstream gradient-ascent summary file |

### Unclear

| path | why it is still unclear |
| --- | --- |
| none after this scan | the ambiguous part is policy, not path role: the shared polytope catalog is duplicated cleanly today, but the repo does not yet say which path should own future edits |

## Current Producer / Consumer Notes

- `crates/symplectic/src/database.rs` is storage machinery, not policy. It loads and saves arbitrary JSONL files, merges multiple files fieldwise, and refuses to choose between conflicting values. See [database.rs](</workspaces/msc-math/crates/symplectic/src/database.rs:1>).
- `PolytopeRecord` treats `dual_vertices_rational` and `vertices_rational` as defining data. `source`, `volume`, `capacity`, `sigma_gap_cutoff`, and `sigmas` are optional fields filled in later. See [database.rs](</workspaces/msc-math/crates/symplectic/src/database.rs:114>).
- `orbit-recovery` trusts cached `capacity` and `sigmas.first().perm` to skip full EHZ on hits. See [orbit-recovery/main.rs](</workspaces/msc-math/experiments/verification/orbit-recovery/main.rs:89>).
- `omega-hypothesis`, `cell-widths`, and `boundary-characterization` all consume the shared catalog as input and assume cached `capacity` plus `sigmas` are available when present. See [omega-hypothesis/main.rs](</workspaces/msc-math/experiments/combinatorial-cells/omega-hypothesis/main.rs:17>).
- `sys-landscape` uses one family cache for the random-sample and random-product-sample runs, and the gradient-ascent binaries also read and write that same file. See [random-sample/main.rs](</workspaces/msc-math/experiments/sys-landscape/random-sample/main.rs:5>) and [gradient-ascent-general/main.rs](</workspaces/msc-math/experiments/sys-landscape/gradient-ascent-general/main.rs:17>).
- `variable-f-ascent` keeps its own local cache because it would otherwise bloat the shared sys-landscape family cache with many intermediate gradient-step polytopes. See [variable-f-ascent/main.rs](</workspaces/msc-math/experiments/sys-landscape/variable-f-ascent/main.rs:719>).
- `experiments/sys-landscape/src/lib.rs` canonicalizes the summary/trace outputs after parallel runs, but that mechanism applies to analysis outputs, not to the shared polytope cache. See [sys-landscape lib](</workspaces/msc-math/experiments/sys-landscape/src/lib.rs:511>).

## Risks / Open Questions

- The canonical-vs-mirror decision is still open. Today the shared catalog and its two mirror candidates are byte-identical, but that is only an observation.
- A cleanup that renames or relocates committed JSONL files would force policy changes in multiple experiment binaries and could change the committed data values, so this packet should not try to do that.
- The most fragile trusted fields are `source`, `capacity`, and `sigmas.first().perm` because they control cache hits and fast-path reconstruction.
- `load_many()` merges by key and fails on field conflicts, so mixed-path loads can surface drift only after the same polytope appears in multiple files.
- Several analyzers key off `source` string conventions (`random`, `known`, and `LagrangianProduct`), so schema drift in `source` would break downstream classification before it breaks serialization.

## Next Safe Resume Point

- Review the producer set for `experiments/combinatorial-cells/polytopes.jsonl` and the three mirror candidates, then decide whether the repo should treat one path as canonical or keep the current mirrored layout.
- If the policy stays descriptive only, the next packet can draft a non-destructive checksum/consistency check without touching tracked JSONL contents.
