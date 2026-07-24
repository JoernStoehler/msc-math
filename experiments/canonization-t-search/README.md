# Coordinate Canonization `T` Search

Status: frozen self-contained experiment as of 2026-06-29.

This experiment found and tested a useful generic coordinate canonizer for random
polytope facet-normal rows. It is intentionally not wired into
`experiments/polytope-invariant-table`, and future agents should not treat it as
the next default integration target.

The method is still worth keeping: it gives a clean generic construction,
stochastic tests for sampled
\(Sp(4)\times \mathbb R_+\times \mathbb R^4\times S_F\) actions, reusable
samplers, and a compact proof note. The reason it is frozen is downstream
value, not mathematical failure. A later sys-datascience comparison found that
volume-one plus analytic-center translation captured most of the observed
method-quality movement on the current random/product method slice, while the
full symplectic-frame representative added little over centering. Moreover,
Euclidean features after this full canonizer are measured in an
arbitrary-but-canonical facet-derived symplectic frame; they are invariant on
the generic success domain, but their geometric meaning is weaker than features
designed directly as invariants, or Euclidean features after an intrinsic
quadratic/ellipsoid normalization. Current sys-datascience therefore uses
direct invariant features rather than this full generic representative.

Keep this experiment runnable. Do not extend it unless the new work is specifically
about coordinate representatives \(T\), a better \(Sp(4)\) frame choice, or
reusing its stochastic group-action harness.

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

## Frozen Candidate

Best registered coordinate-representative candidate:

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
cargo fmt --check -p exp-canonization-t-search
cargo test -p exp-canonization-t-search
cargo run -p exp-canonization-t-search --bin canonization_stochastic -- \
  --cases 24 --samples-per-case 4
```

The smoke run writes
`experiments/canonization-t-search/artifacts/stochastic-rust-summary.json`.
The JSON report records the command, profile, git revision/tree state,
facet-count distribution, transform-family notes, residual threshold,
candidate statuses, all-pair residuals, and `ok/ok` residuals.

Focused current-candidate run:

```bash
cargo run -p exp-canonization-t-search --bin canonization_stochastic -- \
  --cases 256 --samples-per-case 4 \
  --candidate volume_one_omega_labeled_symplectic_frame \
  --metric nearest_neighbor_rms \
  --out experiments/canonization-t-search/artifacts/omega-labeled-symplectic-frame-256-summary.json
```

Cost comparison commands:

```bash
cargo build -q --release -p exp-canonization-t-search \
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

## Freeze Recommendation

Do not integrate this full coordinate representative into
`polytope-invariant-table` as the next step.

What worked:

- The packet gives a generic correctness theorem for the hard
  \(Sp(4)\times S_F\) part.
- The f64 prototype is stable on ordinary random rows in the current stochastic
  tests.
- Runtime of \(T\) itself is negligible compared with one `sys()` call for
  `F=6,10,12`.
- The implementation has explicit non-success statuses for generic-condition
  failure instead of silently choosing arbitrary tied labels.

Why it is frozen rather than integrated:

- The current sys-datascience need is not “make every arbitrary coordinate
  feature invariant at any cost”; it is to design features whose mathematical
  meaning survives the symmetry group and to test those invariance claims.
- Facet permutation canonization is low-value for method tables, because
  features that depend on incidental facet index order are usually bad feature
  designs even after deterministic relabeling.
- The symplectic-frame section chooses a frame from generic omega labels and
  the first successful facet quadruple. This is canonical on the generic domain,
  but Euclidean quantities after that step mean “Euclidean in this
  facet-derived canonical frame,” not “Euclidean in an intrinsic metric of the
  body.”
- A later scratch comparison on the sys-datascience method-sized random/product
  slice showed that scale+analytic-center translation captured the useful
  method-quality movement observed there. Full frame canonization mostly
  matched the centered variant: top associations and random-forest importances
  stayed omega/ridge-area based, and full canonization did not clearly improve
  the current method packets enough to justify the extra semantics and
  genericity boundary.

If this line is reopened, the most promising new branch is probably not more
polish on the current omega-label/facet-frame construction. It is an intrinsic
quadratic-form route: choose a canonical positive definite object from the
polytope, such as a barrier Hessian, body/vertex covariance, or John/Löwner
ellipsoid, then use Williamson normalization. That would better match the
intuition “turn symplectic ellipsoids into standard representatives so Euclidean
norms become meaningful.”

The correct integration posture, if ever revived, is “generic canonical
representative with observable non-success,” not “universal canonical form.”
