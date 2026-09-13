# Retained random/product numerical lineage

This is the trust boundary for the 14,336-row random/product data used by the
thesis. The retained files establish a finite table of **stored numerical
targets**. They do not establish certified mathematical capacity values.

## Retained byte identities

The R2 registry identifies two immutable snapshots:

| Registry entry | Snapshot |
| --- | --- |
| `polytope-datasets` | `f7bc6be841e9d30741d5bf7ec8d4f0c0c74ec22745d3325b792e64cec333ca96` |
| `polytope-invariant-table` | `c3042846723dbb89db17f2f39d88004d937e1e790949ff6880188fa89fc1900a` |

The registry says both were migrated from commit `6049cf77`. At that commit,
the Git LFS pointers record these payload identities:

| File | Size (bytes) | SHA-256 / LFS object ID |
| --- | ---: | --- |
| `random.jsonl` | 53,045,675 | `a21ac62ba5c9496ef631d3cce74e8b663764516b76e9d4725f1e517d8dd55f9f` |
| `random-product.jsonl` | 73,814,237 | `66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736` |
| `polytope-table.jsonl` | 24,250,539 | `607c8731fa03d190d497edc3e8f1b4cca88f7d238260cce527680f568bc33d59` |

The two source-dataset object IDs already occur in commit `ef1b0f6b` (2026-06-12,
“Refresh datascience random baseline”). The invariant-table object ID occurs in
commit `ea8e2657` (2026-06-29, “Refresh invariant retained table artifacts”).
These facts identify retained bytes and their Git history; they are not a run
manifest.

The separately tracked `polytope-provenance-table.jsonl` has 14,336 rows:
4,096 `random_sample` and 10,240 `random_product_sample`, with 14,336 distinct
`poly_id` values. Its current SHA-256 is
`6ff88a5accce9a7ec7e5a494107350b0974b2ce0268ea44caae36a18a7494ef2`.

## What preparation and history establish

The current retained-table loader copies `capacity`, `volume`, and `sys` from
the source rows while computing invariant features. It assigns backend labels
`ehz_capacity` and `ehz_capacity_billiard` itself. The provenance rows therefore
do not independently attest which evaluator produced a stored scalar.

At `ef1b0f6b`, the source tree contains standalone producers whose fresh paths
call `capacity_auto` for generic random rows and `capacity_billiard` for product
rows. Both producers can also reuse cached scalar values. That commit contains
no per-run manifest binding the retained payloads to a clean source revision or
recording cache freshness. Consequently, the source is useful historical
context but not proof that every retained row executed that checkout's fresh
path.

The certified run-local producer migration is later, in commit `75daa6f4`
(2026-07-26). Its `certified-qp-minimizers-v1` contract applies to newly
produced rows carrying that provenance. It cannot be inferred for the retained
June payloads or the table derived from them.

## Supported use and remaining uncertainty

The retained method summary reports 14,336 trusted rows, maximum stored `sys`
`0.86258589584944`, and no stored `sys > 1`. That supports a finite-table claim
about recorded values. The summary explicitly does not validate capacity or
volume computations.

The tracked provenance table contains no `capacity`, `volume`, `sys`,
`capacity_method`, certificate, producer revision, or run-manifest fields.
Without a matching historical run manifest or a new certified evaluation, the
strongest supported lineage statement is therefore: the registered source
bytes are the June 12 retained random/product payloads, the registered invariant
table is a later derived payload that copies their numerical targets, and the
exact execution state and certification status of those targets remain
unrecorded.

Run the cheap tracked-data guard after changing retained datasets, loaders, or
this account:

```bash
python3 experiments/polytope-invariant-table/check-retained-lineage.py
```
