# Performance Experiments

This directory owns reusable runtime and memory profiling targets. Correctness
and regression experiments belong in `experiments/verification/`. Numerical
error-audit runs belong in `experiments/dev-quadratic-program/numerics-audit/`.

For datascience-style f64 capacity workflow questions, the workflow-level
performance packet has moved to
[`../dev-quadratic-program/performance/README.md`](../dev-quadratic-program/performance/README.md).
For direct generic random-polytope route comparison, use `capacity-paths-random`
below.

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

Do not force one performance tool to answer all capacity-route questions:

- use `capacity-route-costs` for paired exact-vs-f64 route rows with shared
  fixture context, capacities, route counts, hardware context, and load context;
- use `capacity-profile-one` for repeated timing or perf/callgrind attribution
  of one selected path on one fixture;
- use `capacity-paths-random` when the question only compares the two ordinary
  f64-based scalar capacity paths over several random fixtures.

## Targets

### `capacity-paths-random`

This target answers the recurring question:

- on deterministic random four-dimensional polytopes, how fast are the two
  generic scalar capacity paths?
- do they return the same f64 capacity on the sampled fixtures?

It compares only these two paths:

- `f64_transition_pruned_hk`: pure-f64 route from
  `experiments/dev-quadratic-program/src/f64_route/`;
- `exact_transition_pruned_f64_then_exact_fallback`: exact transition-pruned
  HK2017 candidate generation plus
  `aggregate_orbits_with_dual_vertices_exact(..., MinimaSafe)`.

It deliberately does not include wrappers that differ only by experiment
ownership, product billiard routing on generic random non-products, unpruned
HK2017, or flow-graph development paths. Use a separate target when those are
the question.

Run a smoke comparison:

```bash
cargo run -p exp-performance --release --bin capacity-paths-random -- \
  --mode smoke \
  --out-dir /tmp/capacity-paths-random-smoke
```

Run the standard F=6/F=10 comparison:

```bash
cargo run -p exp-performance --release --bin capacity-paths-random -- \
  --mode production \
  --out-dir /tmp/capacity-paths-random-production
```

Summarize:

```bash
python3 experiments/performance/scripts/summarize_capacity_paths_random.py \
  /tmp/capacity-paths-random-production \
  --csv /tmp/capacity-paths-random-production/summary.csv
```

The binary writes:

- `metadata.jsonl`: target, mode, facet counts, sample count, generator seed,
  and height range;
- `setup-events.jsonl`: one row per fixture with accepted-fixture attempts,
  exact transition setup time, and allowed transition count;
- `path-events.jsonl`: one row per `(facet_count, sample, path)` with
  measurement scope, elapsed time, capacity, candidate counts, and
  f64-vs-fallback absolute capacity difference when both paths succeeded.

Interpretation boundaries:

- Fixture generation and exact transition setup are outside the per-path timer
  and recorded separately. The fallback row uses
  `after_exact_transition_setup`; the f64 row uses
  `full_f64_route_after_fixture_setup`.
- Measurements are local-machine wall-clock timings. Compare only paired runs
  with the same mode and source revision.
- On ordinary random F=10 fixtures, current evidence says both paths spend
  almost all route time in the same per-sigma f64 KKT solve. Exact fallback
  aggregation is usually not the observed bottleneck unless indeterminate
  candidates overlap the minimum.

### `capacity-profile-one`

This target repeats one capacity path on one deterministic random fixture. It
is the stable command to wrap with `perf`, `cargo flamegraph`, or callgrind
after `capacity-paths-random` or `capacity-route-costs` identifies the
route/input worth attributing.

Build once:

```bash
cargo build -p exp-performance --release --bin capacity-profile-one
```

Run direct wall-clock timing for the pure-f64 route:

```bash
target/release/capacity-profile-one \
  --path f64 \
  --facet-count 10 \
  --sample 1 \
  --repetitions 100 \
  --out-dir /tmp/capacity-profile-one-f64
```

Run direct wall-clock timing for the fallback route:

```bash
target/release/capacity-profile-one \
  --path fallback \
  --facet-count 10 \
  --sample 1 \
  --repetitions 100 \
  --out-dir /tmp/capacity-profile-one-fallback
```

Run the exact transition-pruned route once:

```bash
target/release/capacity-profile-one \
  --path exact \
  --facet-count 6 \
  --sample 0 \
  --repetitions 1 \
  --out-dir /tmp/capacity-profile-one-exact
```

Callgrind example:

```bash
valgrind --tool=callgrind \
  --callgrind-out-file=/tmp/capacity-profile-one-f64.callgrind \
  target/release/capacity-profile-one \
    --path f64 \
    --facet-count 10 \
    --sample 1 \
    --repetitions 10 \
    --out-dir /tmp/capacity-profile-one-f64-callgrind

callgrind_annotate --inclusive=yes --threshold=1 \
  /tmp/capacity-profile-one-f64.callgrind
```

Repeat with `--path fallback` for the exact-fallback route.

The binary writes `profile-summary.jsonl` with the path, measurement scope,
fixture selector, repetition count, elapsed time, per-repetition time, and last
capacity. The profiled loop excludes fixture acquisition. The f64 row uses
`full_f64_route`; fallback and exact rows use `after_exact_transition_setup`.
For f64/fallback rows, use enough repetitions that the repeated capacity loop
dominates one-time setup. The exact route is guarded to `--repetitions 1`
because the F=10 exact row is slow enough that repeated exact timing is usually
the wrong tool.

Observed hotspot in one local 2026-06-25 F=10/sample-1 callgrind run, to be
rechecked with the command above when it matters:

- Both `f64` and `fallback` spent most callgrind instruction refs in
  `solve_kkt_for_dual_vertices` / `solve_saddle_point`.
- `SymmetricEigen::new` was the largest single self-cost visible in that run.
- KKT matrix assembly, transition-pruned enumeration, f64 combinatorics, and
  exact fallback aggregation were each small on that input.

This is empirical evidence for the named fixture and revision, not a theorem
about all random polytopes. Re-run the target after KKT solver changes,
candidate-generation changes, or when the sampled fixtures have many
indeterminate near-minimum candidates.

### `capacity-route-costs`

This target is the paired executable cost demonstration for the capacity-route
ladder in `experiments/dev-quadratic-program/src/route_demonstrations/`. It
compares the current small set of materially different scalar routes on the
same deterministic random fixtures:

- `exact_transition_pruned_sigmas`: exact rational KKT over every sigma visited
  by the exact transition-pruned graph;
- `exact_transition_pruned_f64_then_exact_fallback`: f64 KKT candidate solve
  over the exact transition-pruned graph, followed by exact fallback
  aggregation;
- `f64_transition_pruned_hk`: pure-f64 transition-pruned route.

Run the smoke measurement:

```bash
cargo run -p exp-performance --release --bin capacity-route-costs -- \
  --mode smoke \
  --out-dir /tmp/capacity-route-costs-smoke
```

Run the standard random F=6/F=10 measurement:

```bash
cargo run -p exp-performance --release --bin capacity-route-costs -- \
  --mode production \
  --out-dir /tmp/capacity-route-costs-production
```

Production mode uses one deterministic fixture for each of F=6 and F=10. The
exact row runs once. The fast f64-based rows run 1000 repetitions and report
both total wall time and per-call wall time, because single sub-millisecond
measurements are noise-dominated. On a 2026-06-27 AMD Ryzen 5 1600X
devcontainer run, the F=10 exact row took about 79s, so repeated F=10 samples
are deliberately not the default.

The binary prints hardware and load context to stdout and writes:

- `metadata.jsonl`: target, mode, fixture parameters, hardware context, and
  initial load average;
- `setup-events.jsonl`: one row per fixture for fixture attempts, exact
  transition setup time, and allowed transition count;
- `path-events.jsonl`: one row per measured route with measurement scope,
  repetition count, total wall time, per-call wall time, process CPU time when
  `/proc/self/stat` and `getconf CLK_TCK` are available, load average before
  and after the route, capacity, visited counts, and difference from the
  exact-transition-pruned reference.

Interpretation boundaries:

- There is no timing assertion. Absolute timings are local-machine evidence;
  use the printed hardware, load average, and process-CPU/wall ratio to reject
  noisy runs.
- Fixture acquisition and exact transition setup are recorded separately. Exact
  and exact-transition-fallback rows use `after_exact_transition_setup`; the
  pure-f64 row uses `full_f64_route` because it intentionally runs its own f64
  combinatorics and transition construction.
- `exact_transition_pruned_sigmas` is exact over the transition-pruned sigma
  stream. It is not an unpruned exact-every-HK-sigma route.
- This target is for paired comparison and context. Use `capacity-profile-one`
  plus perf/callgrind for detailed attribution of one selected path.

### Datascience f64 capacity targets moved

The datascience f64 capacity performance binaries, f64-specific summarizers,
and manifest workflow wrapper now live in
`experiments/dev-quadratic-program/performance/`.

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
