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
| `minkowski-symmetrization` | deterministic pushforward | classical polygonal Minkowski symmetrization `(K+R_uK)/2` in a uniformly random direction `u` |

The Crofton/Poisson-line zero-cell arm is abandoned in this packet. A faithful
stationary line process with finite-window and side-count conditioning needs a
separate law definition; a finite-window substitute would be misleading.

Rows retain operation, law kind, parameter, seed, source side counts, output
side count, active-subset counts, lineage, area-normalized shape views, and
directed overlaps. Overlaps are only recorded against the source factors and
are descriptive; they are not target associations. Generation and validation
wall-clock totals are retained only in `batch-report.json` under an explicit
volatile-timing field and never enter deterministic row identities.

## Reproduction

```text
cargo test -p exp-sys-landscape --bin sys-datascience-generator-convex-operations
cargo run -p exp-sys-landscape --release --bin sys-datascience-generator-convex-operations -- \
  --out-dir experiments/sys-datascience/methods/generator-convex-operations/artifacts \
  --attempts 32 --rows-per-bucket 2
```

The default panel uses three deterministic seeds (`20260715`, `20260716`,
`20260717`), side-count buckets `3,4,6`, and two rows per bucket for each
operation. Exhausted attempts are retained as terminal rows; inconvenient
rows are never deleted to force a fixed output side count. The resulting
`rows.jsonl` is deterministic for a committed source and command parameters.
`batch-report.json` records the source revision/tree, status and side-count
histograms, operation dispositions, measured (volatile) generation/validation
cost, and the exact interpretation boundary.

## Evidence boundary

This packet can support only geometry-plumbing and target-free shape-law
comparison. The small three-seed panel is not a population estimate, ranking,
transfer result, `sys`/capacity result, or independence claim. Further common
shape comparisons should stratify by observed side count and preserve binary
random versus deterministic-pushforward labels.
