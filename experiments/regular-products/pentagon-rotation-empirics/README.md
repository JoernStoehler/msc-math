# Pentagon Rotation Empirics

This folder owns sampled data, static figures, and the interactive viewer for

```text
P_5 x_L R(theta)P_5.
```

These artifacts support explanation and orientation. They are not proof
inputs. The exact proof lives in `../pentagon-rotation-formula-proof/`; use its
README and retained transcript for theorem verification.

Local filenames below are relative to this folder. Paths outside this folder
are repo-root relative unless they begin with `../`.

## Enumerated KKT-Branch Landscape

The current thesis-facing packet samples the theorem's two- and three-block
candidate family on

```text
0 <= theta <= pi/10.
```

It freezes the raw sigma universe once at `theta = pi/20` (9 degrees), then
solves that same universe at every sampled angle, including both endpoints.
The Rust generator exposes 3,340 unique raw words at generic interior angles,
with both block counts present. The producer checks that its sets at 4.5, 9,
and 13.5 degrees agree. It compares the count, but not the set, with the exact
certificate's 3,340-word open-domain family: no non-Sage exact-family artifact
exists, and this empirical packet does not invoke Sage.

The canonical grid has 73 angles at 0.25-degree spacing. The three-angle spike
uses only 0, 9, and 18 degrees and is plumbing/feasibility evidence, never
publication evidence.

### Raw artifact

```text
kkt-branch-landscape.jsonl
```

The heterogeneous JSONL schema is `pentagon-kkt-branch-landscape-v1`:

1. one `metadata` record contains the grid, frozen-universe identity, source
   hashes, Git state, command, numerical contract, and fixed display cutoff;
2. one `branch` record per raw sigma contains its raw ID, full sigma word,
   block count, and one outcome for every sample angle;
3. one completed `run_summary` record checks row/outcome counts, four-way
   status totals, runtime, and output scale.

Every sample has exactly one status:

- `admissible`: the existing Rust solve returned an orbit classified
  `AdmissibleF64` or `AdmissibleExact`;
- `numerically_inadmissible`: the existing solver returned
  `OrbitSolveError::Inadmissible`, including its nonpositive-Q or negative-beta
  numerical cases;
- `indeterminate`: the solve returned an orbit in the solver's beta-margin
  indeterminate band;
- `solve_failure`: the solve returned
  `OrbitSolveError::NumericalFailure`.

The last two states remain distinct in data and rendering. A run with zero
solve failures records a zero count; it does not omit the category. The
metadata records both the saddle solver's earlier `1e-12` beta-feasibility
scale and the later `+/-1e-9` margin-classification band; the earlier filter
means not every small negative beta reaches `indeterminate` status.

The raw rows preserve the shared `action_lower`, `action_upper`, and
`q_error_bound` diagnostic fields, including unbounded upper diagnostics. The
shared solver source explicitly says the underlying Q-error-bound lemma needs
replacement before thesis-facing use. This packet therefore does not use
those fields to claim ties, ordering, or independent confirmation of the
lower envelope. The black active curve is identified from the theorem source;
the plot itself is nominal-action exposition.

### Analysis and figures

```text
kkt-branch-analysis.json
enumerated_kkt_branch_landscape.png
enumerated_kkt_branch_landscape.pdf
enumerated_kkt_branch_landscape_raw.png
enumerated_kkt_branch_landscape_grouped.png
kkt_branch_sampled_classification.png
kkt_branch_sampled_classification.pdf
```

`kkt-branch-analysis.json` is the generated compact source for status counts,
sampled-presence counts, endpoint statuses, grouping facts, highlighted raw
words, input identity, and allowed/prohibited interpretation.

The raw figure draws one line per raw sigma. The grouped candidate draws one
fixed raw representative for each whole sampled profile: block count, every
four-way status, and every action rounded to 10 decimal places must agree.
The representative is fixed for the full domain, so grouping never takes a
pointwise minimum, aggregates curves angle by angle, or splices raw branches.
The grouped panel labels report the 7 two-block and 33 three-block sampled-
profile groups together with their 950 and 2390 represented raw words. Gray
curve width and opacity increase with raw multiplicity; exact multiplicities
remain in the report. The grouped view is the selected
`enumerated_kkt_branch_landscape.png`, with a fully vector PDF counterpart.
The raw view and separately named grouped PNG remain available for audit and
comparison. All generated PNGs use 300 dpi.

Only admissible samples are joined. Indeterminate samples and solve failures
have separate markers. A hollow upward triangle at the top boundary marks an
interpolated crossing where adjacent admissible samples continue above the
fixed action cutoff 6; a line break without that glyph comes from a non-
admissible sampled status. Hollow orange diamonds mark indeterminate samples
and are clamped to the nearest vertical boundary when their nominal action is
outside the displayed window, keeping endpoint statuses visible. The cutoff
affects only the display; the JSONL retains all numerical actions. The black
active branch and the worked competitor `(0,5,3,8,1,7)` are fixed raw words,
not angle-wise selected curves.

The sampled-presence classification has only these four exhaustive classes:

- no admissible sample;
- admissible at every sample;
- one contiguous sampled run;
- multiple sampled runs.

Endpoint solve statuses are reported separately. These labels describe the
finite grid only; they are not exact feasibility-interval or topology claims.

### Evidence boundary

Call the figure an **enumerated KKT-branch landscape**, not an all-billiard
branch plot. It illustrates the scale and competition of the finite candidate
problem and sampled branch appearance/disappearance. It does not prove the
lower envelope, classify exact feasibility intervals, display every billiard
orbit, or exclude narrow/isolated specialization-only components. The exact
certificate and continuity argument remain the theorem evidence.

## Other Sources And Artifacts

```text
main.rs
```

Rust producer for the frozen landscape, sampled minima sweep, and retained
legacy three-bounce mode. It is wired into
`experiments/regular-products/Cargo.toml` as
`regular-pentagon-rotation-empirics`.

```text
analyze.py
```

Python analyzer. Every mode requires explicit input paths. Publication figure
generation refuses a spike artifact, so a stale smoke file cannot silently
take precedence.

```text
theta-sweep.jsonl
minimum_orbit_projection_dataset.jsonl
```

The older sampled minima sweep and its derived viewer dataset.

```text
labeled_pentagons_theta.png
trajectory_projections_theta14.png
trajectory_projections_theta14_affine.png
three_bounce_branch_actions.png
signature_state_table_full.png
signature_state_table_competitive.png
signature_legend.txt
minimum_orbit_projection_viewer.html
```

Retained thesis illustrations and viewer. They are empirical only. The legacy
three-bounce plot groups by coarse support signature and is not the current
branch-landscape publication candidate.

## Regeneration Commands

Bounded three-angle spike:

```bash
cargo run -p exp-regular-products --release --bin regular-pentagon-rotation-empirics -- --branch-landscape --spike
uv run --script experiments/regular-products/pentagon-rotation-empirics/analyze.py landscape \
  --input experiments/regular-products/pentagon-rotation-empirics/smoke-kkt-branch-landscape.jsonl \
  --allow-spike --validate-only
```

Canonical landscape and publication candidates:

```bash
cargo run -p exp-regular-products --release --bin regular-pentagon-rotation-empirics -- --branch-landscape --canonical
uv run --script experiments/regular-products/pentagon-rotation-empirics/analyze.py landscape \
  --input experiments/regular-products/pentagon-rotation-empirics/kkt-branch-landscape.jsonl
```

Producer tests:

```bash
cargo test -p exp-regular-products --bin regular-pentagon-rotation-empirics
```

Older sampled minima sweep:

```bash
cargo run -p exp-regular-products --release --bin regular-pentagon-rotation-empirics -- --canonical
```

Legacy static figures require both explicit inputs:

```bash
cargo run -p exp-regular-products --release --bin regular-pentagon-rotation-empirics -- --three-bounce-branches --canonical
uv run --script experiments/regular-products/pentagon-rotation-empirics/analyze.py legacy \
  --minima-input experiments/regular-products/pentagon-rotation-empirics/theta-sweep.jsonl \
  --branch-input experiments/regular-products/pentagon-rotation-empirics/three-bounce-branches.jsonl
```

Interactive orbit viewer:

```bash
uv run --script experiments/regular-products/pentagon-rotation-empirics/build_interactive_orbit_viewer.py
```
