# Random-Generator Rejection Calibration

Question: how often does the legacy random-polytope generator accept candidates
across its fixed facet-count and height-range grid, and how much time is spent
on accepted versus rejected attempts?

`main.rs` generates 1,000 attempts for each of 18 hardcoded configurations
(`F=5..10` and three height ranges) with seed `42`, then rewrites
`acceptance.jsonl`. The retained JSONL is the answer artifact; inspect it for
exact per-configuration ratios and timings.

## Retained answer and population

The retained 18 rows report the following acceptance percentages; each cell
has 1,000 attempted bodies.

| Facets | Heights `[0.5,2.0)` | Heights `[0.1,5.0)` | Heights `[0.8,1.2)` |
| --- | ---: | ---: | ---: |
| 5 | 5.7% | 5.7% | 5.7% |
| 6 | 16.8% | 16.2% | 17.0% |
| 7 | 33.4% | 30.5% | 34.4% |
| 8 | 45.0% | 37.1% | 50.6% |
| 9 | 48.9% | 33.2% | 60.2% |
| 10 | 50.4% | 28.3% | 72.9% |

For the narrow height interval, acceptance rose across the sampled facet counts;
for the widest interval it peaked at `F=8` and then fell. These are finite
generator measurements, not an attribution of rejection to a particular
geometric mechanism. The rows do not classify rejection reasons.

The current `run_sweep` reseeds a ChaCha8 stream to 42 for every grid cell;
the cells therefore do not represent 18 independent seeds. It calls
`SysLandscapePolytopeCache::sample_random`: unit normals on `S^3` and uniform
heights produce dual vertices, then geometry validation accepts bounded,
nonredundant bodies and constructs their primal incidence. See
[`../src/sys_landscape_cache.rs`](../src/sys_landscape_cache.rs) and the
[candidate sampler](../../../../crates/euclidean-polytopes/src/random.rs).
The measured time includes generation and accepted-body geometry construction;
it excludes capacity evaluation. In the retained rows, accepted-attempt means
range from about 1.36 to 5.78 ms and rejected-attempt means from about 0.081 to
1.34 ms.

The artifact has no per-row seed, source revision, rejection reason or geometry
payload. The seed and grid above describe the source's fixed configuration;
they do not by themselves establish byte reproduction of the historical
artifact after geometry or RNG changes. Recalibration would create a new
measurement, including new machine-dependent timings.

## Execution boundary

```bash
cargo run -p exp-sys-landscape --release --bin sys-rejection-calibration
```

There is no smoke or output-path override, so do not use that command as a
quick check. The test module runs smaller sweeps for row-count and arithmetic
invariants. Timing values are local-machine measurements, and acceptance
ratios belong to the named generator, seed policy, grid, and attempt count.
