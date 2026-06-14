# Performance Experiments

This directory owns reusable runtime and memory profiling targets. Correctness
and regression experiments belong in `experiments/verification/`. Numerical
error-audit runs belong in `experiments/numerics/`.

The default pattern is:

1. run a concrete end-to-end target binary,
2. write raw structured outputs under an explicit output directory,
3. summarize or render those outputs with scripts,
4. wrap the same binary with sampling, callgrind, or heap tools when a
   question needs lower-level attribution.

Commands below assume the repo root as the working directory.

Generated outputs should usually go under `/tmp`. Reports under `/tmp` are
review artifacts, not durable project state.

Each target has two named modes:

- `smoke`: small input for checking that the binary and summaries still work.
- `production`: the normal heavier run for profiling decisions.

Changing facet counts, sample counts, height ranges, or graph density is a
different experiment variant. Add a named mode or change the mode constants in a
worktree so the variant is visible in the code diff.

## Targets

### `f64-capacity-e2e`

This target measures the realistic pure-f64 capacity path on retained
`datascience/` artifacts. It compares `strict`, `lp_origin_vertex`,
`lp_origin_vertex_product_billiard_or_hk`, and `lp` from
`experiments/dev-f64-capacity` on the same rows. `lp_origin_vertex` is the generic
candidate default: LP for the origin decision, vertex-scan geometry for facet
presence/intersection. `lp_origin_vertex_product_billiard_or_hk` keeps that
validation policy but uses the billiard sigma stream for Lagrangian products
and falls back to generic transition-pruned HK otherwise. This is the method to
inspect for fastest product-aware datascience scans. Correctness and coverage
interpretation still belongs in `experiments/dev-f64-capacity`.

The measured `f64_capacity_e2e` phase includes f64 validation, f64 capacity when
validation accepts, and row classification. It excludes artifact JSON loading
and exact audit. Artifact capacity labels are used only by the already existing
f64 scan-row agreement fields.

The summary table uses these f64-specific timing names:

- `e2e_mean_ms`: the complete measured phase for one input row and one method.
- `validation_bundle_ms`: the complete f64 validation stage reported by
  `experiments/dev-f64-capacity`. This is not an origin-in-interior sub-timer.
- `capacity_bundle_ms`: the capacity stage only for rows where capacity actually
  ran. Rows rejected or sent to fallback before capacity are counted in
  `capacity_not_run` and excluded from this mean.

Candidate and predicate-count columns ending in `_if_capacity` are also
computed only over rows where capacity ran.
`sigma_count` is the number of candidate sigma words tested by the exhaustive
search; it is not an optimization iteration count.

The same rows also include routine-level subphase timers. Validation subphase
timers are averaged over all ok rows. Capacity subphase timers are averaged only
over rows where capacity ran. The most useful columns for comparing strict and
LP policies are:

- `val_origin_lp_diag_ms`: the diagnostic origin LP solve. It is emitted for
  both policies because the f64 scan row records origin-LP diagnostics.
- `val_origin_policy_ms`: the predicate used by the selected policy for
  origin-in-interior.
- `val_combinatorics_ms`, with `val_vertex_scan_ms`,
  `val_lp_facet_statuses_ms`, and `val_lp_facet_pairs_ms` as major pieces.
- `cap_combinatorics_ms`, `cap_transition_matrix_ms`, and
  `cap_kkt_candidates_ms`. For `lp_origin_vertex_product_billiard_or_hk`,
  `cap_kkt_candidates_ms` includes f64 product classification plus either
  billiard sigma solving or generic HK fallback.

Current performance diagnosis:

- Product-aware routing is the useful product speedup already found in this
  line of work. Commit `f3d06b47` added the product-aware path, and commit
  `dc8cac0b` split candidate-stream time into KKT and non-KKT pieces.
- For retained `random_product F=12` rows, the current maintained diagnostic
  `/tmp/perf-f64-capacity-f12-kkt-split-50` measured about 96% of
  candidate-stream time in KKT solves for transition-surviving sigmas, with
  enumeration/filter/non-KKT work around 33 ms per row. Re-run the command below
  before relying on these local numbers.
- A disposable duplicate/canonicalization spike on the same 50 retained `F=12`
  rows found no exact duplicate or cyclic-equivalent sigmas among the 670,292
  transition-surviving sigmas. It also counted about 19.7M block-structure
  sigmas rejected before KKT by the transition filter. Duplicate removal and
  pre-KKT enumeration optimization therefore look low-potential for this
  cohort.
- Remaining plausible speedups are not cleanup: a product-specific
  action/admissibility formula, a genuinely cheaper pre-KKT rejector, or a
  trusted faster replacement for the eigendecomposition KKT solve would each
  need separate design and review.

Run a smoke profile:

```bash
cargo run -p exp-performance --release --bin f64-capacity-e2e -- \
  --mode smoke \
  --out-dir /tmp/perf-f64-capacity-smoke
```

Run the bounded production profile:

```bash
cargo run -p exp-performance --release --bin f64-capacity-e2e -- \
  --mode production \
  --out-dir /tmp/perf-f64-capacity-production
```

Summarize phase events:

```bash
python3 experiments/performance/scripts/summarize_phase_jsonl.py \
  /tmp/perf-f64-capacity-smoke
```

Run the retained `random_product` `F=12` product-aware KKT split diagnostic:

```bash
cargo run -p exp-performance --release --bin f64-capacity-e2e -- \
  --mode production \
  --case-filter random_product_f12 \
  --method-filter product_billiard_or_hk \
  --max-cases 50 \
  --out-dir /tmp/perf-f64-capacity-f12-kkt-split-50
```

The binary writes:

- `phase-events.jsonl`: one row for each selected input row and method. Rows
  include `family`, `method`, f64 outcome fields, candidate counts, transition
  counts, `validation_bundle_time_ms`, `capacity_bundle_time_ms` when capacity
  ran, and the complete `elapsed_ms` for the measured phase.

`smoke` uses two retained rows per family plus HKO. `production` uses a bounded
100 retained rows per family plus HKO. A full retained-artifact mode should be a
separate reviewed run, likely on LICCA or in a background session, because the
generic and full-LP product methods can enumerate many more candidates than the
product-aware billiard method.

### `f64-decision-compare`

This target measures the f64 decision routines directly, without bundling them
into a capacity run. Use it when the question is which algorithm decides a
specific proposition faster or more decisively:

- origin-in-interior: strict origin predicate versus LP origin;
- facet presence: vertex-scan coverage, per-facet LP, batched primal LP, and
  batched polar LP;
- facet-pair intersection: vertex incidence versus LP facet-pair checks;
- omega signs: the current f64 omega routine, reported without an alternative.

It writes `decision-events.jsonl`, not `phase-events.jsonl`, because each row is
a direct routine comparison rather than an end-to-end phase.

Run a smoke comparison:

```bash
cargo run -p exp-performance --release --bin f64-decision-compare -- \
  --mode smoke \
  --out-dir /tmp/perf-f64-decision-compare-smoke
```

Run the bounded production comparison:

```bash
cargo run -p exp-performance --release --bin f64-decision-compare -- \
  --mode production \
  --out-dir /tmp/perf-f64-decision-compare-production
```

Summarize decision events:

```bash
python3 experiments/performance/scripts/summarize_decision_jsonl.py \
  /tmp/perf-f64-decision-compare-production \
  --csv /tmp/perf-f64-decision-compare-production/decision-summary.csv
```

The current high-value comparison is `strict_origin_predicate` versus
`lp_origin`: LP origin resolves product rows that strict leaves indeterminate,
and its cost is below the capacity runtime for retained rows. Facet
presence/intersection variants are retained because they are cheap to maintain,
but when their costs are below 1% of E2E time, trust and clarity should decide
which one is promoted.

### `hk2017-pruned-f64`

This target profiles the pruned HK2017 f64 candidate path on deterministic
synthetic flat polytopes. It is for algorithm-path profiling, not for measuring
the retained datascience tables. It mirrors the scalar capacity path:

1. acquire an accepted random fixture,
2. build exact geometry,
3. build the transition matrix,
4. solve pruned HK2017 candidates,
5. aggregate the zero-gap minimum orbit set with exact fallback.

The `accepted_fixture_acquisition` phase calls the existing random fixture
generator until it returns an accepted polytope. It includes the generator's
validation work. The later `exact_geometry` phase rebuilds geometry from that
accepted fixture for the profiled capacity path.

The phase boundaries are real wrapper functions marked `#[inline(never)]` in
the profiling binary. This makes flamegraphs and callgraphs more likely to show
the domain phases as useful call-stack anchors.

Run a smoke profile:

```bash
cargo run -p exp-performance --release --bin hk2017-pruned-f64 -- \
  --mode smoke \
  --out-dir /tmp/perf-hk2017-smoke
```

Run the production profile:

```bash
cargo run -p exp-performance --release --bin hk2017-pruned-f64 -- \
  --mode production \
  --out-dir /tmp/perf-hk2017-production
```

Summarize phase events:

```bash
python3 experiments/performance/scripts/summarize_phase_jsonl.py \
  /tmp/perf-hk2017-smoke
```

The binary writes:

- `phase-events.jsonl`: one flat sibling event for each phase reached by a
  sample. Do not put nested subphase rows here; use tracing for subphases.

For subphase questions, use opt-in tracing:

```bash
mkdir -p /tmp/perf-hk2017-trace
cargo run -q -p exp-performance --release --bin hk2017-pruned-f64 -- \
  --mode production \
  --trace \
  --out-dir /tmp/perf-hk2017-trace \
  2> /tmp/perf-hk2017-trace/span-close.log
```

The trace stream is raw measurement output. Current HK2017 trace summaries are
grouped by target, event name, and facet count; use `phase-events.jsonl` for
per-sample timing and error status.
HK2017 trace summary events can be summarized with:

```bash
python3 experiments/performance/scripts/summarize_hk2017_trace.py \
  /tmp/perf-hk2017-trace/span-close.log
```

The `hk2017_candidate_solve_summary` event includes `unattributed_search_ms`.
That value is a residual, not measured traversal time. Use `perf` or callgrind
when the question is which concrete functions consume CPU time. Enumeration
trace events are split by producer: `hk2017_directed_cycle_summary` for the
graph-native pruned iterator and `hk2017_unpruned_enumeration_summary` for the
unpruned subset/permutation traversal.

### `hk2017-cycle-enumeration`

This target profiles only the directed-graph simple-cycle iterator used by
HK2017 transition pruning. It uses synthetic directed graphs; `facet_count` is
the node count for those graphs. Use the e2e target when representativeness for
random polytopes matters.

The Rust API is
`symplectic::algorithms::hk2017::SimpleDirectedCyclesCanonical`.
`for_each_simple_directed_cycle_canonical` is a callback adapter for existing
HK2017 call sites.

Run an isolated cycle-enumeration profile:

```bash
cargo run -q -p exp-performance --release --bin hk2017-cycle-enumeration -- \
  --mode production \
  --out-dir /tmp/perf-hk2017-cycles
```

With tracing:

```bash
mkdir -p /tmp/perf-hk2017-cycles-trace
cargo run -q -p exp-performance --release --bin hk2017-cycle-enumeration -- \
  --mode production \
  --trace \
  --out-dir /tmp/perf-hk2017-cycles-trace \
  2> /tmp/perf-hk2017-cycles-trace/span-close.log
```

Summarize the phase rows with `summarize_phase_jsonl.py`. Summarize trace rows
with `summarize_hk2017_trace.py`.

## Tool Use

Use the same target binary with different tools.

- Phase JSONL: workflow timing, cold/hot separation, and post-processing.
- Tracing: opt-in subphase spans when the question needs domain boundaries
  below the normal phase rows.
- `cargo flamegraph` or `perf`: sampled CPU call stacks for CPU time under a
  release run.
- `valgrind --tool=callgrind`: deterministic instruction attribution and a
  separate attribution tool when `perf` cannot run.
- Heap profilers: allocation and memory questions.
- Criterion: tight kernel comparisons after an end-to-end run has identified a
  kernel worth isolating.

Do not treat these tools as competing architectures. The stable unit is the
reproducible target binary plus its raw outputs.
Measurements are empirical and machine-dependent. Compare runs only when the
input selector, release/debug profile, command, and machine context are fit for
the question being asked.

### Flamegraph

Example:

```bash
cargo flamegraph -p exp-performance --release --bin hk2017-pruned-f64 -- \
  --mode production \
  --out-dir /tmp/perf-hk2017-flame
```

If `cargo flamegraph` fails before running the binary, check `perf --version`.
The devcontainer pins userland, but `perf` also depends on the host kernel. A
typical failure is `/usr/bin/perf` pointing at linux-tools for a different
kernel than the running host kernel. Refresh the apt index and install the
matching tools inside the container when this happens:

```bash
sudo apt-get update
sudo apt-get install -y linux-tools-$(uname -r) linux-cloud-tools-$(uname -r)
```

If `perf_event_paranoid` blocks unprivileged profiling, use `--root`:

```bash
cargo flamegraph --root -p exp-performance --release --bin hk2017-pruned-f64 -- \
  --mode production \
  --out-dir /tmp/perf-hk2017-flame
```

Use callgrind when kernel permissions or host policy block `perf`.

### Callgrind

Example:

```bash
cargo build -p exp-performance --release --bin hk2017-pruned-f64

valgrind --tool=callgrind \
  --callgrind-out-file=/tmp/perf-hk2017.callgrind \
  target/release/hk2017-pruned-f64 \
  --mode smoke \
  --out-dir /tmp/perf-hk2017-callgrind

callgrind_annotate --threshold=0.5 /tmp/perf-hk2017.callgrind
```

Callgrind changes wall-clock behavior. Use it for attribution, not as a direct
production runtime estimate.

## Output Policy

1. Every binary must accept `--out-dir`.
2. Defaults must point under `/tmp` and include the target, mode, timestamp, and
   process id to avoid common collisions.
3. Reusing an output directory may overwrite that target's raw output files.
4. Binaries should print only the output directory or a short status line.
5. Raw measurements should be structured files, not ad-hoc prose on stdout.
6. Reviewed reports should link their raw command, target, input selector, and
   generated artifact paths.

## Adding Targets

Add a direct Cargo binary under `src/bin/`. Do not add a dispatcher unless the
repo has a concrete repeated need for one.

Each target should document:

1. what path it profiles,
2. what its phase boundaries include and exclude,
3. what input selector or fixture family it uses,
4. what output files it writes,
5. which lower-level profiler is appropriate for likely follow-up questions.
