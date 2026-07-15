# Generator combinatorial coverage

This packet asks a target-free question: at matched facet count `F`, do an
alternative generator law's rows contain exact incidence types not observed in
the current panel, or do they only reweight types already present? A separate
geometry summary records variation inside one incidence type. The packet does
not call `sys`, capacity, orbit searches, or target caches.

## Source inspection and adapter contract

The retained generic and product producers are
`experiments/sys-datascience/produce/random.jsonl` and
`experiments/sys-datascience/produce/random-product.jsonl`. Their rows carry
exact rational dual vertices and vertices, but not a persisted incidence
matrix. The adapter reconstructs incidence with `Fraction` from
`a_f^T v = 1`, the producer convention. Future generator rows should instead
emit `vertex_facet_incidence` directly under `combinatorial-row-v1`; no
floating-point tolerance is accepted for exact type identity.

I inspected the reusable incidence-derived face helpers in
`crates/euclidean-polytopes/src/faces.rs` and the existing generator-zoo,
within-distribution, two-distribution, and type-cone packets. Those helpers
derive faces and adjacency but do not provide a canonical-labeling utility.
Canonical labeling therefore stays copy-local here, with an explicit bounded
search and fail-closed status. The factor-only generator-zoo rows have no 4D
incidence and are accepted only if a future producer adds the sidecar.

## Exact identity and bounded failure

`analyze.py` canonicalizes the color-preserving bipartite vertex--facet graph
by equitable refinement and deterministic individualization. A completed leaf
search emits the full canonical incidence serialization (`canonical_status:
exact`). `--exact-node-cap` counts search nodes. If the cap is reached, the
row retains a WL-style digest and `canonical_status: capped`, but **no exact
type is emitted or counted**. A WL digest is never an exact certificate.

This is collision-safe only for rows marked `exact`: equal exact labels are
equal canonical incidence matrices under independent vertex and facet
permutations, and unequal labels are distinct serialized canonical matrices.
The cap is intentionally conservative. A nauty/traces-grade backend and a
formal complexity/performance study are abandoned for this cheap packet.

## Metrics

For every law and fixed `F`, the report contains exact-type occupancy,
singleton/doubleton counts, plugin entropy and effective number, observed
collision probability, a Good--Turing singleton diagnostic, and deterministic
prefix rarefaction/discovery curves. Between laws at matched `F`, it reports
shared exact types, directed observed-panel mass coverage, and incremental new
type yield under balanced budgets. `independence_group` is retained; rows with
the same root/factor identity are not treated as independent draws.

`within_type_geometry` reports normalized Euclidean pair-distance summaries
inside each exact type. This is a geometry-only diagnostic: it does not quotient
all affine maps, certify product structure, or imply that equal incidence means
equal bodies.

Good--Turing/Chao-style unseen diagnostics are small-`n` and dependence-
sensitive diagnostics, not support estimates. No result supports target,
`sys`, capacity, population-support, ranking, or “all combinatorics reached”
claims.

## Calibration and replay

The synthetic fixture tests four required failure modes:

* labeled row/column permutations collapse to one exact type;
* a nonisomorphic incidence separates despite equal dimensions;
* duplicate/correlated rows remain visible in raw occupancy while
  `independence_group` deduplication is reported separately;
* a forced small cap returns `capped` with no exact label.

Run the calibration and tests:

```bash
cd experiments/sys-datascience/methods/generator-combinatorial-coverage
uv run --script analyze.py --input fixtures/synthetic.jsonl --out-dir artifacts/synthetic
uv run --script test_analyze.py
```

The retained real smoke uses the first 24 rows from each current producer
file, a deterministic fixed panel (not a fresh random draw), with exact
incidence reconstructed from retained rationals:

```bash
uv run --script analyze.py \
  --input ../../produce/random.jsonl \
  --input ../../produce/random-product.jsonl \
  --facet-counts 6 --max-rows-per-input 24 --exact-node-cap 500 \
  --out-dir artifacts/real-smoke
```

`artifacts/*/report.json` records input SHA-256 values, analyzer SHA-256,
selection rule, seed, cap, rejected rows, and tracked-source cleanliness.
The clean-source guard checks tracked changes to this analyzer path; run from
a committed clean checkout before treating an artifact as evidence. Generated
reports contain no volatile timings. Regenerate reports; do not hand-edit
JSONL/TSV rows.

## Retained smoke observations (descriptive only)

The current 24-row smoke has one exact type in generic `F=5` and one exact type
in product `3x3` (`F=6`), with effective number one in each matched stratum.
This is consistent with the fixed generator families and panel selection; it
does not establish their population support or imply that alternative laws
cannot produce another type. The synthetic panel separates the two abstract
incidence graphs as intended.

## Scope and disposition

Implemented: exact rational incidence reconstruction, bounded exact canonical
labels, fail-closed cap/WL diagnostics, occupancy and rarefaction, observed
between-law coverage and balanced incremental yield, dependence metadata, and
within-type geometry variation.

Abandoned for this packet: exhaustive canonical labeling above the cap,
probabilistic unseen-support inference, affine/product equivalence, target or
capacity linkage, and any claim about the full combinatorial population. Reopen
only with a named decision, an independent-row contract, and a backend whose
canonicalization behavior has its own calibration.
