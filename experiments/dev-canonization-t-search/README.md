# Coordinate Canonization `T` Search Packet

This packet is a prototype and decision aid for canonicalizing random
polytope facet-normal rows. It is intentionally not wired into
`experiments/sys-datascience/prepare`.

The target is a coordinate representative. Given normalized inequalities
\[
  K=\{x:\langle a_i,x\rangle\le 1\},
\]
find a partial map \(C\) from facet-normal row lists to facet-normal row lists,
implemented by choosing symmetry data \(T(A)\), such that \(C(A)\) is
independent of sampled actions from
\[
  Sp(4)\times \mathbb R_+ \times \mathbb R^4 \times S_F.
\]
Rows are stored as `Vector4<f64>` column vectors even though they represent
facet covectors, so the candidate code applies matrix operations to each row
vector rather than literally left-multiplying an `F x 4` matrix.

The practical reason is feature search. Without a representative, a proposed
quantity \(X(A)\) must be designed to be invariant. With a successful
canonical representative \(C(A)\), any coordinate feature \(X(C(A))\) is
invariant on the success domain of \(C\). Carefully designed invariant features
are still preferable when available, but the canonical representative reduces
the amount of feature-specific symmetry work needed during exploration.

## Current Candidate

Current best registered candidate:

```text
volume_one_omega_labeled_symplectic_frame
```

It is implemented in
`src/candidates/volume_one_omega_labeled_symplectic_frame.rs`.

Algorithm, in the order used by code and proof:

1. Apply volume-one scaling.
2. Translate to the analytic center.
3. Compute \(\Omega_{ij}=\omega(a_i,a_j)=a_i^TJa_j\).
4. Give each facet a generic label by sorting its \(\Omega\)-row values.
5. Sort facets by these labels. If two labels tie after quantization, return
   status `nonunique_omega_signature`.
6. Scan ordered quadruples in this canonical facet order.
7. Use symplectic Gram-Schmidt on the first successful quadruple to build a
   symplectic covector frame \(F_A=(q_1,q_2,p_1,p_2)\).
8. Return the ordered coordinate rows \(F_A^{-1}b_i\).

The output is a `Vec<Vector4<f64>>` with the original row count. It is not the
invariant \(\Omega\)-matrix route.

## Theorem Status

The exact mathematical statement is in
`../../formal/generic-coordinate-canonization.tex`, label
`prop:generic-coordinate-canonization`. It is agent-written and not yet
Jörn-reviewed.

The theorem proves generic correctness for the symplectic and facet-permutation
parts. The generic condition is open dense:

- the \(\Omega\)-row signatures distinguish all facets;
- the rows span \(V^*\), so the ordered quadruple scan finds a symplectic
  frame.

On this domain, for \(R\in Sp(4)\) and a facet permutation \(\tau\),
\[
  C(R^{-T}\tau A)=C(A).
\]

The formal note proves only the symplectic and permutation section after
scale-fixing and centering. The Rust candidate also applies volume-one scaling
and analytic-center translation before that section. The stochastic evidence
below tests the combined prototype under scale and translation; this packet
does not add a new proof of those already-known steps.

This is not a universal canonical labeling theorem. Symmetric or nearly
symmetric inputs can have tied \(\Omega\)-row signatures. The current f64
prototype treats those cases as non-success instead of pretending to solve
them.

## Metrics

The main diagnostic metric is `nearest_neighbor_rms`:

- it compares two facet-normal row lists with the same row count;
- for each row on either side it finds the nearest row on the other side;
- it reports the RMS nearest-neighbor distance divided by the larger RMS row
  norm scale.

It is symmetric, nonnegative, row-permutation insensitive, and satisfies
`m(A,A)=0` in tests. It is not a proved mathematical metric because it can
under-report failures when different facets are very close. This is acceptable
for stochastic near-equivariance testing; ordered metrics are available for
matrix-like invariant representatives and for checking that a candidate's
canonical row order is stable.

## Reproducible Commands

From this worktree root:

```bash
cargo fmt --check -p exp-dev-canonization-t-search
cargo test -p exp-dev-canonization-t-search
cargo run -p exp-dev-canonization-t-search --bin canonization_stochastic -- \
  --cases 24 --samples-per-case 4
```

The smoke run writes
`experiments/dev-canonization-t-search/artifacts/stochastic-rust-summary.json`.
The JSON report records the command, profile, git revision/tree state,
facet-count distribution, transform-family notes, residual threshold,
candidate statuses, all-pair residuals, and `ok/ok` residuals.

Focused current-candidate run:

```bash
cargo run -p exp-dev-canonization-t-search --bin canonization_stochastic -- \
  --cases 256 --samples-per-case 4 \
  --candidate volume_one_omega_labeled_symplectic_frame \
  --metric nearest_neighbor_rms \
  --out experiments/dev-canonization-t-search/artifacts/omega-labeled-symplectic-frame-256-summary.json
```

Cost comparison commands:

```bash
cargo build -q --release -p exp-dev-canonization-t-search \
  --bin t_cost_smoke --bin sys_cost_smoke

hyperfine --warmup 2 -r 10 './target/release/t_cost_smoke 128 6 32'
hyperfine --warmup 2 -r 10 './target/release/t_cost_smoke 128 10 32'
hyperfine --warmup 2 -r 10 './target/release/t_cost_smoke 128 12 32'

hyperfine --warmup 1 -r 3 './target/release/sys_cost_smoke 8 6'
hyperfine --warmup 1 -r 3 './target/release/sys_cost_smoke 4 10'
hyperfine --warmup 1 -r 3 './target/release/sys_cost_smoke 2 12'
```

## Current Results

The 256-case run in
`artifacts/omega-labeled-symplectic-frame-256-summary.json` used accepted
random rows from `symplectic::random::generate_dual_vertices`, four sampled
transforms per base case, and seven transform families.

For 256 accepted base rows and 7,168 transformed evaluations, every candidate
status was `ok`. There were no observed `nonunique_omega_signature` failures,
no frame failures, and no residual failures above the report threshold
`1e-5`.

Largest observed residuals in the 256-case artifact:

| transform family | max canonicalized residual |
| --- | ---: |
| scale + translation + permutation | `7.8e-7` |
| translation | `7.1e-7` |
| sampled full group | `7.0e-7` |
| sampled full-dimensional local `Sp(4)` | `7.6e-11` |

Interpretation: the exact theorem predicts zero residual on the generic
domain. The observed nonzero values are consistent with f64 volume,
translation, linear solve, and matrix-exponential error. This is empirical
stability evidence for ordinary random inputs, not evidence for adversarial
near-tie cases.

Cost at the facet counts requested for current downstream relevance. The
stochastic harness itself cycles accepted random cases through
`F=8,10,12,14`; these timings separately benchmark `F=6,10,12`.

| F | `T` cost per call | `sys` cost per call | ratio |
| ---: | ---: | ---: | ---: |
| 6 | `0.0406 ms` | `0.152-0.173 s` | about `3.7k-4.3k x` |
| 10 | `0.196 ms` | `1.64-1.67 s` | about `8.4k-8.5k x` |
| 12 | `0.381 ms` | `3.36-3.39 s` | about `8.8k-8.9k x` |

`perf`/flamegraph were blocked in the container by
`perf_event_paranoid=4`; the cost numbers above are `hyperfine` wall-clock
measurements plus explicit application phase timers.

## Architecture For Future Search

Add a new coordinate candidate by:

1. Creating `src/candidates/<candidate_name>.rs`.
2. Exporting a `pub const SPEC: CandidateSpec`.
3. Returning `CandidateOutput { duals, status }`.
4. Registering it in `src/candidates/mod.rs`.
5. Running the smoke command above, then a focused run with `--candidate`.

Registered `T` candidates must preserve row count and return coordinate
facet-normal rows. Invariant matrix representatives may live under
`src/candidates/` for comparison, but they must not be registered in
`candidates::all()`.

Add a new diagnostic by:

1. Creating `src/metrics/<metric_name>.rs`.
2. Exporting a `pub const SPEC: MetricSpec`.
3. Registering it in `src/metrics/mod.rs`.
4. Adding tests for at least nonnegativity, `m(A,A)=0`, and the failure mode
   the metric is meant to detect.

The stochastic harness already samples:

- scale;
- translation;
- facet permutation;
- scale + translation + permutation;
- the block subgroup `diag(A,A^{-T})`;
- full-dimensional local `Sp(4)` samples `exp(JH)` with \(H\) symmetric;
- full group samples.

The `Sp(4)` sampler is not Haar-like on the noncompact group. It is enough to
test the Lie-algebra directions near the identity. The unit tests verify that
the Lie algebra samples satisfy \(X^TJ+JX=0\), that exponentials are
symplectic, and that the full sampler is not restricted to the block subgroup.

## Recommendation

Develop this candidate toward integration, but do not wire it into
`sys-datascience/prepare` in this packet.

Reasons:

- There is a clean generic correctness theorem for the hard
  \(Sp(4)\times S_F\) part.
- The f64 prototype is stable on ordinary random rows in the current stochastic
  tests.
- Runtime is negligible compared with one `sys()` call for `F=6,10,12`.
- The implementation has explicit non-success statuses for generic-condition
  failure instead of silently choosing arbitrary tied labels.

Before integration, add tests or design decisions for:

- deliberately symmetric and near-symmetric inputs;
- whether prepare should drop, mark, or fallback-canonicalize non-success rows;
- exact reproducibility of volume/center/canonicalization across machines;
- feature-table provenance fields recording candidate label, status, metric
  version, and tolerances.

The correct integration posture is “generic canonical representative with
observable non-success,” not “universal canonical form.”
