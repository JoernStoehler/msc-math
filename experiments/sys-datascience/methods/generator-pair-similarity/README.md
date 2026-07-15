# Pairwise polytope similarity catalog

## Decision and scope

This target-free packet supplies comparison building blocks for later
within-law diversity and two-law coverage work.  It deliberately keeps several
non-equivalent views rather than selecting one privileged feature vector.  The
implementation is self-contained: planar generator-zoo rows are used only for
the smoke, and synthetic 4D configurations calibrate the 4D methods because
the existing product-smoke summary intentionally has no 4D coordinates.

`pair_similarity.py` contains the executable catalog.  Its JSON report repeats
the semantic contract, quotient, complexity, knobs, and failure boundary for
every method.  The first representation rungs are intentionally naive:

1. raw source-order coordinate `L1`, `L2`, and `Linf`;
2. deterministic lexicographically least *cyclic* vertex start;
3. exhaustive unordered-vertex assignment for at most eight vertices, with a
   separately named greedy upper bound above that cap.

A raw distance generated solely by a cyclic source reorder is retained as an
order-sensitivity diagnostic.  It is not a failed geometry comparison.

Later rungs add planar translation/area/finite-rotation-grid quotienting (with
reflection off or separately requested), a body-level sampled-support comparison that
permits unequal facet counts, fixed-frame variable-cardinality vertex-cloud
Hausdorff distance, fixed-label 4D Euclidean Gram/Procrustes information,
fixed-`F` symplectic-Gram quotient information, unlabeled incidence comparison,
named scalar feature vectors, and a fixed transformation-bank response
signature.  Response similarity is deliberately only one heuristic view.

## Mathematical contracts

The report catalog is the machine-readable contract.  Important distinctions:

- Raw, canonical-start, exhaustive assignment, Euclidean Gram, Procrustes,
  and normalized vertex-cloud Hausdorff are metrics on the *stated normalized
  representation/quotient*, subject to their domains.
- The sampled planar support distance is a numerical pseudometric modulo the
  *finite* group `C_G`, where `G` is the declared support grid (default 128),
  or `D_G` only when reflection is requested.  It quotients translation by the
  polygon centroid and positive scale by area.  A rotation by an integer
  multiple of `2π/G` is a zero-control; an off-grid rotation generally has a
  nonzero residual.  Thus it is a useful direct geometric comparison and an
  approximation surface for a future grid-convergence study, not a claim of a
  continuous rotation, support, or Hausdorff quotient.  It also accepts
  polygons with unequal side counts as this packet's body-level
  variable-facet comparison.
- The full symplectic-Gram permutation quotient is a metric only for equal-
  `F`, validated analytic-center/volume-one spanning facet covectors.  The
  float implementation here is an information prototype; the exact bounded
  certificate lives in the separate accepted quotient packet.  This packet
  exhausts only `F <= 8`, and returns no result over the cap.
- Incidence Hamming after exhaustive facet relabeling and canonical vertex
  columns is a metric on fixed-size unlabeled incidence matrices, not a
  geometric metric.  It returns no result above seven facets.
- Named feature and response distances are representation distances or
  heuristics unless a downstream owner supplies a completeness argument.

Facet matching is not one operation here.  Unrestricted assignment examines
all permutations only below the stated cap.  Cyclic and dihedral matching keep
the source cyclic order; `permitted_permutation_assignment_l2` accepts only an
upstream-supplied incidence-automorphism/product-factor matching family; and
the product-factor comparator compares the two factors separately, optionally
allowing only a whole `q/p` swap.  The packet does not infer a combinatorial
automorphism group or silently turn one matching regime into another.

The planar normalizer uses the polygon centroid and area, then samples support
on a declared grid.  It does not delete Fourier modes or claim affine or
symplectic invariance.  The variable-facet method is vertex-cloud Hausdorff,
not polytope Hausdorff: it intentionally exposes the missing point-to-face
calculation rather than quietly substituting it.

## Calibration and smoke

The test suite uses an adversarial finite synthetic set.  It checks identity,
symmetry and all ordered triangle triples for every method advertised there as
a metric/pseudometric; translation/scale/exact `C_G` rotation, an explicit
off-grid nonzero residual, and separately `D_G` reflection;
facet permutation; a symplectic non-orthogonal map; an orthogonal
non-symplectic map; and simple planar/variable-cardinality deformations.
These finite checks are calibration evidence only, not proofs of metric claims.
The generated report adds an exact-equivalence regression matrix: every row
names a relation, method, expected `zero`/`positive`/`unavailable` disposition,
the numerical observation, and pass state.  It includes source reorder,
`C_G` rotation/scale/translation, off-grid residual, `D_G` reflection,
symplectic versus orthogonal controls, facet permutation, unequal side counts,
and factor swap.

Run the focused tests:

```bash
uv run --script experiments/sys-datascience/methods/generator-pair-similarity/test_pair_similarity.py
```

Regenerate the small multi-population smoke after hydrating the line artifact:

```bash
git lfs checkout experiments/sys-datascience/methods/generator-zoo-smoke/artifacts/factor-shapes.jsonl
uv run --script experiments/sys-datascience/methods/generator-pair-similarity/pair_similarity.py \
  --input experiments/sys-datascience/methods/generator-zoo-smoke/artifacts/factor-shapes.jsonl \
  --out-dir experiments/sys-datascience/methods/generator-pair-similarity/artifacts \
  --per-population 2
```

The smoke chooses the two lowest hash-ranked rows per existing population,
compares only equal-side-count pairs for planar/order/assignment methods, and
writes inspectable pair examples plus `comparison.tsv`.  It is deterministic
descriptive plumbing, not a population diversity/coverage estimate.  Its
generated artifacts are bound to the input path, input hash, producer hash, and
cap in `report.json`; no target value, cache, capacity, or `sys` field is read.
The producer prints a volatile wall-clock measurement to stderr for local cost
observations, but deliberately does not retain it in `report.json` or the TSV.
The focused test regenerates the real hydrated smoke twice and compares both
retained files byte-for-byte.

## Failure boundaries and downstream use

Do not pool unequal facet/vertex counts just because a feature embedding has a
number.  Do not call greedy assignment optimal, a sampled support distance
certified, a response signature a geometry definition, or a low Gram distance
evidence of coverage in a variable-`F` union.  Exact symplectic claims require
the accepted exact packet's normalization and search contract.

`symplectic_containment_gauge` is intentionally deferred.  It needs a defined
affine/linear symplectic containment optimization and a certificate/status
contract; this packet neither implements it nor substitutes a Gram, support,
or feature score under that name.

Later packets may reuse this catalog to make diversity and coverage summaries
more interpretable.  They must predeclare the representation/quotient,
stratify or explicitly compare facet counts, retain the finite search cap and
numeric grid, and separately justify any population-level inference.
