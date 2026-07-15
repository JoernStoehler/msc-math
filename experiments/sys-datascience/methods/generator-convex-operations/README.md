# Natural convex-operation generator bundle

This is a bounded, target-free geometry packet for natural operations on the
current planar random-factor law. It is deliberately separate from the
`sys`/capacity pipeline: no row is selected by a target and no transformed
shape is called an independent sample.

## Operations and formulas

Every retained factor is translated by its area centroid and scaled so that
area is one. The producer validates a strict convex, counterclockwise cycle,
finite unit outward normals, and all active inequalities globally.

| operation | law kind | construction and lineage |
| --- | --- | --- |
| `baseline` | fresh current law | current-law IID normal angles and supports in `[0.8,1.2)`, conditioned on all requested inequalities being active |
| `minkowski-sum` | binary random | independently rotated current-law `A+B`; support addition is `h_{A+B}=h_A+h_B` |
| `intersection` | binary random | independently rotated current-law `A∩B`; all pairwise H-boundary intersections are tested against both halfspace systems, and active input inequalities are counted |
| `difference-body` | deterministic pushforward | `K+(-K)` from one current-law factor; this is marked as a pushforward, not independent breadth |
| `convex-hull-union` | binary random | `conv(A∪B)` from independently rotated vertex sets; source vertices on the output boundary are counted |
| `minkowski-symmetrization` | deterministic pushforward | classical polygonal Minkowski symmetrization `(K+R_uK)/2` in a uniformly random axis direction `u`; the output is reflection-symmetric about `u`, not generically centrally symmetric |

The Crofton/Poisson-line zero-cell arm is abandoned in this packet. A faithful
stationary line process with finite-window and side-count conditioning needs a
separate law definition; a finite-window substitute would be misleading.

Rows retain operation, law kind, parameters, seed, stable output/source IDs,
source side counts and rotation angles, output side count, active-subset counts,
lineage, canonical CCW area-one output vertices, embedded source vertices,
64-direction centered support signatures, and (for symmetrization) the axis and
reflection residual. The embedded source geometry makes binary versus
pushforward lineage replayable without a sidecar. Directed overlaps are
source-coordinate intersection-area ratios (`area(output ∩ source) /
area(source)`), not quotient shape distances; compare them only after explicit
side-count stratification. They are descriptive and not target associations.
Wall-clock generation/validation timings are printed to stdout only and are
not retained in deterministic artifacts.

## Reproduction

```text
cargo test -p exp-sys-landscape --bin sys-datascience-generator-convex-operations
cargo run -p exp-sys-landscape --release --bin sys-datascience-generator-convex-operations -- \
  --out-dir experiments/sys-datascience/methods/generator-convex-operations/artifacts \
  --attempts 32 --rows-per-bucket 2
```

For a byte replay from a clean source checkout, run twice into separate output
directories and compare both files:

```text
./target/release/sys-datascience-generator-convex-operations --out-dir /tmp/convex-a --attempts 32 --rows-per-bucket 2
./target/release/sys-datascience-generator-convex-operations --out-dir /tmp/convex-b --attempts 32 --rows-per-bucket 2
cmp /tmp/convex-a/rows.jsonl /tmp/convex-b/rows.jsonl
cmp /tmp/convex-a/batch-report.json /tmp/convex-b/batch-report.json
```

The default panel uses three deterministic seeds (`20260715`, `20260716`,
`20260717`), side-count buckets `3,4,6`, and two rows per bucket for each
operation. Exhausted attempts are retained as terminal rows; inconvenient
rows are never deleted to force a fixed output side count. The resulting
`rows.jsonl` is deterministic for a committed source and command parameters.
`batch-report.json` records a producer-source revision/tree (stable across the
later artifact commit), producer paths, the workspace `Cargo.lock` BLAKE3,
empty input-hash map (there are no external inputs), output-row BLAKE3/count,
status and side-count histograms, operation dispositions, and the exact
interpretation boundary. The producer refuses tracked dirty source and fails
closed on an incomplete/nonterminal row contract. Timings are stdout-only.

## Evidence boundary

This packet can support only geometry-plumbing and target-free shape-law
comparison. The small three-seed panel is not a population estimate, ranking,
transfer result, `sys`/capacity result, or independence claim. Minkowski
symmetrization supports a reflection-axis statement only; central symmetry is
reserved for the difference-body arm. Further common-shape comparisons should
stratify by observed side count and preserve binary-random versus
deterministic-pushforward labels.
