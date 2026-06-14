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
