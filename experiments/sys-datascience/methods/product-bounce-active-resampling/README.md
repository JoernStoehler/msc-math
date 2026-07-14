# Active-support conditional-resampling smoke

## Decision and object

This packet tests whether the inactive facets of retained random `5x5`
Lagrangian products can be resampled while one exact winning six-facet
`3q+3p` word is frozen. It is a feasibility and failure-mode gate before any
multi-base statistical study. The base row, not a resample, is the prospective
inferential unit.

Four bases are selected without `sys`, capacity, or volume: two producer
two-bounce rows and two producer three-bounce rows, paired by standardized
Euclidean distance over the six recovered generator coordinates from the
precursor plus ridge normalized entropy and ridge maximum share. The two ridge
features are recomputed target-free for only the 1,024 retained `5x5` rows from
stored rational geometry. `artifacts/bases.json` freezes the identities,
features, canonical exact word, unique support, and matching rule.

The two implemented laws are:

- `fixed_ranks`: freeze active normals, support heights, and sorted ranks. In
  every interval cut out by `0`, the active angles, and `2*pi`, sample the
  prescribed number of inactive uniform-angle order statistics. Sample
  inactive heights independently from `Uniform[0.8,1.2]`.
- `unlabeled_support`: freeze only the active geometric support, sample the two
  inactive factor angles independently on the circle, sort all five facets,
  and remap the fixed word by its active geometry. This marginalizes the active
  rank allocation under the original angle law.

Both laws reapply `SysLandscapePolytopeCache::from_f64_dual_vertices`, the
original origin-interior/all-dual-points-extreme acceptance boundary. They
apply no area, volume, or scale normalization.

## Recorded failure layers

For every accepted geometry the producer copies active dual rows bit-for-bit
and records:

1. exact fixed-word action agreement and exact closure-constraint rank;
2. recovered-orbit maximum halfspace violation and minimum inactive-facet
   slack (`fixed_inactive_clearance_min`);
3. whether the remapped word occurs in the current billiard candidate stream;
4. the exact minimum and minimizer words of that candidate stream, recomputed
   bounce labels, and takeover identities;
5. unnormalized volume, fixed-branch `sys`, and global `sys`.

The recovered-orbit feasibility threshold is `max_violation <= 1e-8`. This is
an available f64 geometric diagnostic, not an exact physical-admissibility
certificate. Candidate-stream presence, KKT admissibility, recovered geometry,
and exact minimality are deliberately separate because the APIs establish
different facts. All accepted rows have these diagnostics; null is not used as
a measured zero.

## Reproduction

Hydrate the two reviewed LFS inputs first:

```bash
git lfs checkout -- \
  experiments/sys-datascience/produce/random-product.jsonl \
  experiments/sys-datascience/methods/product-bounce-distribution/artifacts/class-minima.jsonl
```

From the repository root, set the owner once:

```bash
O=experiments/sys-datascience/methods/product-bounce-active-resampling
```

Build-gate and produce the target-free match features:

```bash
cargo build --manifest-path "$O/Cargo.toml" \
  --bin product-bounce-active-match-features && \
"$O/target/debug/product-bounce-active-match-features" \
  --input experiments/sys-datascience/produce/random-product.jsonl \
  --output "$O/artifacts/match-features.jsonl"
```

Freeze the deterministic base match:

```bash
python3 "$O/select_bases.py" \
  --raw experiments/sys-datascience/produce/random-product.jsonl \
  --classes experiments/sys-datascience/methods/product-bounce-distribution/artifacts/class-minima.jsonl \
  --ridge-features "$O/artifacts/match-features.jsonl" \
  --out "$O/artifacts/bases.json"
```

Build-gate and run exactly 16 accepted proposals per base and law:

```bash
cargo build --release --manifest-path "$O/Cargo.toml" \
  --bin product-bounce-active-resampling && \
"$O/target/release/product-bounce-active-resampling" \
  --raw experiments/sys-datascience/produce/random-product.jsonl \
  --classes experiments/sys-datascience/methods/product-bounce-distribution/artifacts/class-minima.jsonl \
  --bases "$O/artifacts/bases.json" \
  --out "$O/artifacts/proposals.jsonl" \
  --accepted-per-base 16 \
  --max-attempts-per-base-law 160
```

Regenerate the compact summary and provenance:

```bash
python3 "$O/summarize.py" \
  --proposals "$O/artifacts/proposals.jsonl" \
  --bases "$O/artifacts/bases.json" \
  --runtime "$O/artifacts/runtime.txt" \
  --out "$O/artifacts/summary.json"
python3 "$O/write_provenance.py" --root . \
  --out "$O/artifacts/provenance.json"
```

Focused implementation checks:

```bash
cargo test --manifest-path "$O/Cargo.toml"
cargo fmt --manifest-path "$O/Cargo.toml" -- --check
cargo clippy --manifest-path "$O/Cargo.toml" --all-targets -- -D warnings
```

The producer stops at 160 proposals without 16 acceptances for a base/law,
after 128 target evaluations, or on any invariant that would invalidate exact
action freezing. The external packet contract additionally stops before 20
wall minutes or two local core-hours. The retained source/artifact hashes and
exact commands are in `artifacts/provenance.json`; measured costs are in the
two runtime artifacts.

## Evidence boundary

Allowed: assess whether the two conditional laws are implementable and
affordable; separate the recorded fixed-word failure layers; estimate whether
volume and `sys` move enough to justify another bounded design.

Prohibited: infer a two-/three-bounce class effect from four selected bases;
treat nonzero volume variation as an inactive-freedom mechanism; call the f64
recovery diagnostic an exact admissibility theorem; generalize beyond this
retained generator; use a resample as an independent inferential unit.

The observations and next-packet recommendation are in `interpretation.md`.
