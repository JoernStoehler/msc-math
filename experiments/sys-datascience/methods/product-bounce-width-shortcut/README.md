# Product two-bounce width shortcut

## Decision question

Does the exact retained two-bounce class minimum have a geometry-only formula
that avoids the full billiard target search, and can that formula account for
the persistent capacity component of the retained bounce-label association?

The frozen candidate, derived before inspecting the full retained table, is

```text
W2(P,Q) = min_{d in boundary(P-P)} h_{Q-Q}(d).
```

The current mathematical authority is the stronger merged proof in
`formal/product-two-bounce-class.tex`: it proves the formula at
agent-reviewed, not Jörn-reviewed, status.  `DERIVATION.md` is retained only
as the historical local pre-merge derivation; it is not current proof
authority.  `analyze.py` is the method-local exact polygon implementation.
The retained `A2` artifact is used only after the formula is frozen, as a
bounded finite validation surface.

## Inputs and execution contract

The reviewed retained inputs are:

- `../product-bounce-distribution/artifacts/class-minima.jsonl`, SHA-256
  `187089804bd17fdac76bdaf51a8d8202e67fb2b14779fe9a418cc8da47c7b4c4`;
- `../../../produce/random-product.jsonl`, SHA-256
  `66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736`.

The optional prepared table and provenance inputs reproduce the reviewed
generator/ridge-control models.  Rebuild them using the command recorded by
`../product-bounce-mechanism/README.md`.

Run deterministic analytic fixtures:

```bash
python3 experiments/sys-datascience/methods/product-bounce-width-shortcut/analyze.py --fixtures
```

Run a two-row-per-bucket stratified smoke before the full table:

```bash
python3 experiments/sys-datascience/methods/product-bounce-width-shortcut/analyze.py \
  --raw experiments/sys-datascience/produce/random-product.jsonl \
  --classes experiments/sys-datascience/methods/product-bounce-distribution/artifacts/class-minima.jsonl \
  --per-bucket-limit 2 \
  --out /tmp/product-bounce-width-smoke.json
```

Run the full retained **post-target bookkeeping**, including the reviewed
control design:

```bash
TABLES_DIR="$(mktemp -d /tmp/sys-ds-product-bounce-width.XXXXXX)"
experiments/sys-datascience/prepare/build-random-only-slice.sh full "$TABLES_DIR"
python3 experiments/sys-datascience/methods/product-bounce-width-shortcut/analyze.py \
  --raw experiments/sys-datascience/produce/random-product.jsonl \
  --classes experiments/sys-datascience/methods/product-bounce-distribution/artifacts/class-minima.jsonl \
  --table "$TABLES_DIR/polytope-table.jsonl" \
  --provenance "$TABLES_DIR/polytope-provenance-table.jsonl" \
  --association-from-retained-a2 \
  --out experiments/sys-datascience/methods/product-bounce-width-shortcut/artifacts/retained-association.json
```

This second command deliberately substitutes the target-derived certified `A2`
column for `W2`; it does not validate the independent formula.  Its purpose is
only to quantify the structural decomposition

```text
2 log(capacity) = 2 log(W2) + 2 log(capacity/W2).
```

The prepared-table identities are the fixed hashes in `analyze.py` and the
reviewed rebuild route in `../product-bounce-mechanism/README.md`.  The retained
association artifact records an ephemeral `/tmp` execution path; that path is
not an input authority and need not survive.  The hashes and rebuild contract
are authoritative.

## Results

`artifacts/summary.json` is the independent exact validation artifact.  It
contains exactly 20 deterministic retained rows, the first two rows in each of
the ten `(k,m)` buckets.  The geometry-only rational formula equals certified
retained `A2` on all 20/20 rows, with zero exact mismatches and zero reported
float relative error.  The three analytic fixtures also pass exactly: square
pair `4`, rectangle pair `20`, and triangle/rotated-difference-body pair `2`.
All 20 retained minima occur at an interior point of a difference-body edge
with a support tie in the norm factor.  This phenotype count is diagnostic at
smoke scale, not a population estimate.

`artifacts/retained-association.json` contains all 10,240 rows, but uses the
target-derived certified `A2` values for numerical bookkeeping.  In the exact
same fixed-effects designs as the reviewed precursor, the bounce-three
coefficient in `2 log(capacity)` splits as follows:

| controls | `2 log(A2)` geometry term | `2 log(capacity/A2)` selection term | capacity term |
|---|---:|---:|---:|
| bucket only | 0.35433 | -0.14570 | 0.20863 |
| generator coordinates | 0.37853 | -0.14225 | 0.23628 |
| ridge distribution | 0.36458 | -0.12940 | 0.23519 |
| generator plus ridge distribution | 0.37597 | -0.12765 | 0.24832 |

Thus the positive retained capacity component is more than accounted for by
higher two-bounce difference-body inradii among three-bounce-labelled rows;
three-bounce lower-envelope takeover attenuates that contrast.  The width AUC
for the three-bounce label is `0.6987--0.7515` across buckets; after volume
normalization, the definitionally related `s2` AUC is `0.7807--0.8360`, and its
within-bucket Spearman association with stored `sys` is `0.9712--0.9943`.
These are post-target descriptive associations.  They show how the exact
two-bounce geometry organizes the retained capacity component, not that `W2`
alone predicts the winner: deciding the label still requires comparison with
`A3`.

## Stopped work and compute cost

The all-row independent geometry validation was not completed.  A full
prepared-table rebuild was stopped after roughly `54--55` CPU-minutes without
an artifact.  The subsequent all-row exact rational audit was stopped after
about `3:45` CPU-minutes and likewise produced no partial artifact.  Therefore
the exact checked denominator is 20 and the unchecked denominator is 10,220.

The retained smoke took about 7 seconds wall / 2.3 CPU-seconds in its final
run; the final fixed-hash association pass took 13.3 seconds wall / 7.7 CPU-
seconds.  Aggregate packet work was approximately `58--59` CPU-minutes,
including the two stopped attempts, so it remained narrowly below the one-
core-hour cap.  No further run is authorized merely to complete the
denominator or refresh provenance.

`artifacts/provenance.json` identifies the retained files and current analyzer.
The 20-row summary predates addition of the association-only output metadata;
the exact geometry routine did not change, but byte-identical regeneration
with the current analyzer was deliberately not performed after approaching the
cap.

## Decision

This is the exact-formula branch of the decision tree, not a proxy or a
target-equivalent rewrite.  The merged formal proof now establishes the
formula at agent-reviewed status; Jörn review remains required before thesis
use.  Retain this packet for its method-local exact implementation and bounded
cross-bucket validation, not as the current proof source.  Do not launch full
active-facet resampling to explain the capacity component: the two-bounce
component already has the difference-body inradius formula.  A separate
lower-envelope or structured-family study is needed only if the thesis
requires an explanation of why `A3<A2`, not to establish `A2` itself.

## Allowed and prohibited interpretations

Allowed: use the formal proof for the current agent-reviewed mathematical
status of the geometry-only two-bounce formula; use this packet's retained
20-row equality as bounded finite implementation validation, not exhaustive
validation; decompose the descriptive capacity coefficient into two-bounce
geometry and lower-envelope selection.

Prohibited: call the producer label or `s2` an independently validated target;
claim that `W2` alone predicts whether the three-bounce class wins; treat
in-table label association as generated-candidate validation; or infer a
generic three-bounce, causal, or cross-generator mechanism.
