# Performance Experiments

This directory owns reusable performance targets and profiling practice. It
does not own production datasets, thesis evidence, or durable datascience
results. Correctness, regression, and numerical-validation work belongs in
crate tests or `experiments/verification/`, not in this performance package.
Numerical error-bound collection and predicate diagnostics belong in
`experiments/numerics/`.

Correctness-adjacent information may appear here only when it is part of the
measurement record: status fields, error messages, input selectors, traced
phase counters, and instrumentation needed to explain where time or memory was
spent. Do not add correctness experiments, proof notes, regression suites, or
numerical validation datasets here just because the same algorithm is being
profiled.

The default pattern is:

1. run a concrete end-to-end target binary,
2. write raw structured outputs under an explicit output directory,
3. summarize or render those outputs with scripts,
4. wrap the same binary with sampling, callgrind, or heap tools when the
   question needs lower-level attribution.

Commands below assume the repo root as the working directory.

Generated outputs should usually go under `/tmp`. Commit only small reports or
assets whose current value is higher than their maintenance cost. Reports under
`/tmp` are review artifacts, not durable project state.

## Targets

### `hk2017-pruned-f64`

This target profiles the pruned HK2017 f64 candidate path on deterministic
synthetic flat polytopes. It is for algorithm-path profiling, not for measuring
the retained datascience tables. It mirrors the scalar capacity path:

1. generate an accepted random fixture,
2. build exact geometry,
3. build the transition matrix,
4. solve pruned HK2017 candidates,
5. aggregate the zero-gap minimum orbit set with exact fallback.

Fixture generation is not raw random sampling only. It calls the existing
accepted-fixture generator, which validates candidate dual vertices through the
rational construction pipeline before returning. The later `exact_geometry`
phase rebuilds the geometry being profiled from that accepted fixture.
Interpret `fixture_generation` as accepted-input acquisition overhead, not as a
subphase of the capacity computation on an already-loaded fixture.

The phase boundaries are real wrapper functions marked `#[inline(never)]` in
the profiling binary. This makes flamegraphs and callgraphs more likely to show
the domain phases as useful call-stack anchors.

Run a smoke profile:

```bash
cargo run -p exp-performance --release --bin hk2017-pruned-f64 -- \
  --facet-counts 10 \
  --samples 1 \
  --out-dir /tmp/perf-hk2017-smoke
```

Summarize phase events:

```bash
python3 experiments/performance/scripts/summarize_phase_jsonl.py \
  /tmp/perf-hk2017-smoke
```

The binary writes:

- `run-metadata.json`: target, command, cwd, configuration, output file paths,
  timestamp, git head/dirty status when available, and `rustc --version` when
  available.
- `phase-events.jsonl`: one raw event for each phase that was reached by a
  sample. If a fixture or solve phase fails, later phases for that sample are
  omitted. The summarizer reports error counts and computes timing means from
  successful phase rows.

For subphase questions, keep the normal JSONL schema stable and use opt-in
tracing:

```bash
mkdir -p /tmp/perf-hk2017-trace
cargo run -q -p exp-performance --release --bin hk2017-pruned-f64 -- \
  --facet-counts 10,11 \
  --samples 3 \
  --trace \
  --out-dir /tmp/perf-hk2017-trace \
  2> /tmp/perf-hk2017-trace/span-close.log
```

The trace stream is raw measurement output. Post-process it for a specific
question instead of adding one-off analysis columns to `phase-events.jsonl`.
Current HK2017 trace summaries are aggregate subphase rows keyed by facet count;
use `phase-events.jsonl` for per-sample timing and error status.
HK2017 candidate-solve summary events can be summarized with:

```bash
python3 experiments/performance/scripts/summarize_hk2017_trace.py \
  /tmp/perf-hk2017-trace/span-close.log
```

The `hk2017_candidate_solve_summary` event reports total candidate-search time,
residual traversal overhead, accumulated KKT solve time, orbit-payload time,
sigma-length summaries, KKT outcome counts, and admissibility counts. The
`hk2017_enumeration_summary` event reports graph-native simple-cycle traversal
counters: DFS prefixes, rejected directed-edge extensions, and emitted cycles.
On successful runs emitted cycles equal the candidate-solve `iterations` count;
on fatal solver failure, `iterations` is the number actually attempted before
stopping. This is a domain-level split; use `perf` or callgrind on the same
binary when the next question is which concrete functions consume a reported
bucket.

### `hk2017-cycle-enumeration`

This target profiles only the directed-graph simple-cycle enumeration used by
HK2017 transition pruning. It does not generate polytopes, build exact
geometry, or solve KKT systems. Nodes are facet indices and directed edges are
allowed transitions `i -> j`.

The Rust API is
`symplectic::algorithms::hk2017::SimpleDirectedCyclesCanonical`.
`for_each_simple_directed_cycle_canonical` is a callback adapter for existing
HK2017 call sites.

Run an isolated cycle-enumeration profile:

```bash
cargo run -q -p exp-performance --release --bin hk2017-cycle-enumeration -- \
  --facet-counts 10,11 \
  --samples 8 \
  --edge-probability 0.25 \
  --out-dir /tmp/perf-hk2017-cycles
```

With tracing:

```bash
mkdir -p /tmp/perf-hk2017-cycles-trace
cargo run -q -p exp-performance --release --bin hk2017-cycle-enumeration -- \
  --facet-counts 10,11 \
  --samples 8 \
  --edge-probability 0.25 \
  --trace \
  --out-dir /tmp/perf-hk2017-cycles-trace \
  2> /tmp/perf-hk2017-cycles-trace/span-close.log
```

Summarize the normal phase rows with `summarize_phase_jsonl.py`. Summarize the
HK2017 enumeration trace rows with `summarize_hk2017_trace.py`.

## Tool Use

Use the same target binary with different tools.

- Phase JSONL: workflow timing, cold/hot separation, and post-processing.
- Tracing: opt-in subphase spans when the question needs domain boundaries
  below the normal phase rows.
- `cargo flamegraph` or `perf`: sampled CPU call stacks. This is the preferred
  call-stack view when the question is CPU time under a release run.
- `valgrind --tool=callgrind`: deterministic instruction attribution and a
  portable fallback when host-kernel `perf` tooling blocks flamegraphs.
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
  --facet-counts 10 \
  --samples 8 \
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
  --facet-counts 10 \
  --samples 8 \
  --out-dir /tmp/perf-hk2017-flame
```

Use callgrind as a fallback when kernel permissions or host policy still block
`perf`.

### Callgrind

Example:

```bash
cargo build -p exp-performance --release --bin hk2017-pruned-f64

valgrind --tool=callgrind \
  --callgrind-out-file=/tmp/perf-hk2017.callgrind \
  target/release/hk2017-pruned-f64 \
  --facet-counts 10 \
  --samples 2 \
  --out-dir /tmp/perf-hk2017-callgrind

callgrind_annotate --threshold=0.5 /tmp/perf-hk2017.callgrind
```

Callgrind changes wall-clock behavior. Use it for attribution, not as a direct
production runtime estimate.

## Output Policy

1. Every binary must accept `--out-dir`.
2. Defaults must point under `/tmp` and include the process id to avoid common
   timestamp collisions.
3. Reusing an output directory may overwrite that target's raw output files.
4. Binaries should print only the output directory or a short status line.
5. Raw measurements should be structured files, not ad-hoc prose on stdout.
6. Reviewed reports should link their raw command, target, git state if
   relevant, input selector, and generated artifact paths.

## Adding Targets

Add a direct Cargo binary under `src/bin/`. Do not add a dispatcher unless the
repo has a concrete repeated need for one.

Each target should document:

1. what path it profiles,
2. what its phase boundaries include and exclude,
3. what input selector or fixture family it uses,
4. what output files it writes,
5. which lower-level profiler is appropriate for likely follow-up questions.
