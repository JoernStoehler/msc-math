# Harmonized ridge-tail source comparison

Status: completed pre-target source/design packet. Its frozen generic `F=10`
contract was subsequently executed and stopped at 10,000 candidates. The
generic target result belongs to `generic-ridge-tail-stage1-target/`; this
packet remains the source for the product sensitivity, proxy-definition audit,
and choice not to replicate more product buckets.

This packet freezes an existing-data comparison for generic/non-product F=10,
product 5x5, and product 4x6. It is an operational source comparison, not a
causal product-factorization test. The analyzer reads retained target rows and
already generated product target panels; it contains no geometry, capacity, or
`sys` producer.

## Regeneration

From the repository root:

```bash
git lfs checkout -- \
  experiments/sys-datascience/prepare/polytope-table.jsonl \
  experiments/sys-datascience/prepare/polytope-provenance-table.jsonl \
  experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/artifacts/100k-ridge-concentration-validation/selected-candidates-before-sys.jsonl \
  experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/artifacts/100k-ridge-concentration-validation/sys-evaluation-cache.jsonl \
  experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/artifacts/100k-promising-scalars/selected-candidates-before-sys.jsonl \
  experiments/sys-datascience/methods/extreme-scalar-rejection-proposer/artifacts/100k-promising-scalars/sys-evaluation-cache.jsonl

python3 experiments/sys-datascience/methods/ridge-tail-source-comparison/analyze.py
```

The no-argument command now validates the completed target-free generic 10k
manifest by default. An explicit path is useful when reviewing a different
candidate artifact:

```bash
python3 experiments/sys-datascience/methods/ridge-tail-source-comparison/analyze.py \
  --generic-manifest experiments/sys-datascience/methods/generic-ridge-tail-stage1/artifacts/stage1/manifest.json
```

An explicitly supplied missing path fails. The analyzer validates retained row
counts, finite values, product ridge-count constancy, selected/target identity,
generated seeds, selection cutoffs, target censoring, and the frozen generic
manifest counts and target-free boundary. It writes all owner artifacts from
source rows, with sorted JSON and fixed bootstrap seeds. Running it twice must
give identical bytes:

```bash
sha256sum experiments/sys-datascience/methods/ridge-tail-source-comparison/artifacts/current/*
```

## Source inventory and evidence boundary

`artifacts/current/source-inventory.json` hashes the retained table and
provenance table and records canonical hashes and denominators for 512 retained
generic F=10 rows and 1,024 retained rows in each product bucket. All retained
rows have target `sys` values. Product generated panels are joined by
`candidate_id` and `poly_id`:

| panel | seed | selector | candidate denominator | target-visible rows |
|---|---:|---|---:|---:|
| generated 1% | 1,618,033 | per-bucket low sum proxy, 1% | 10,000/bucket | 100/bucket |
| generated 0.1% | 271,828 | per-bucket low mean proxy, top 10 | 10,000/bucket | 10/bucket |

The product 5x5 and 4x6 rows are extracted from these panels only. Non-selected
generated candidates have censored targets. The two panel seeds are distinct;
alternate selectors in the same target cache are reused rows and are not
independent replication. Product 5x5 has fixed ridge count 35 and 4x6 fixed
ridge count 34, so mean and sum rank identically in each product population.

`retained-band-summary.tsv` gives full-target within-population bands supported
by the retained sample (generic 5%, 10%, 20%; products additionally 1%). It
reports baseline means, low-proxy enrichment against the retained generic
90th-percentile threshold, adjacent-band hardening, bootstrap mean intervals,
Wilson exceedance intervals, and unique-row effective counts.

`generated-product-summary.tsv` reports the two censored product panels and
their contrasts against the corresponding retained population baseline.
`product-sensitivity.tsv` compares 5x5 against 4x6 at the same generated
cutoffs. The observed mean differences are -0.00668 (1% panel) and +0.01382
(0.1% panel), both below the provisional 0.04 decision contrast; 4x6 does not
materially change the qualitative operational conclusion.

`mean-sum-proxy-audit.tsv` is only a proxy-definition audit. It quantifies
generic mean-versus-sum rank disagreement (the generic retained population has
variable ridge counts) and confirms exact rank equivalence in both fixed-count
product buckets. It must not be interpreted as tail evidence.

## Frozen 10k contract (executed)

`artifacts/current/future-analysis-contract.json` is the machine-readable
pre-target contract. Its `pending` status records the state in which it was
frozen and is intentionally not rewritten after target exposure. It froze
exactly 10,000 generic F=10 candidates ranked by the f64
mean proxy, target evaluation for the lowest 1% (100 rows), disjoint bands of
10 (`0-.1%`) and 90 (`.1-1%`), and a deterministic disjoint 100-row baseline.
The singleton `.01%` row has no inferential role. Future production uses f64
volume; retained rational-derived volume is historical data, not an audit.

Primary contrasts are low 1% versus baseline and `0-.1%` versus `.1-1%`, each
using the provisional smallest decision-relevant mean-`sys` contrast 0.04.
The sole high-`sys` threshold is the retained generic nearest-rank 90th
percentile, `0.5949424195457518`; use Wilson intervals for its exceedance rate.
Use deterministic percentile bootstrap intervals (20,000 resamples) for means,
described as uncertainty rather than asymptotic inference at n=10 and n=90.
Compare the generic contrasts with reused product 5x5 at matching nominal
cutoffs, retaining the operational/confounding caveat and the 4x6 sensitivity
check.

Continue to 100k only for material generic hardening or generic-minus-product
interaction, or unresolved uncertainty whose interval still spans both a flat
plateau and +0.04 while product evidence is not flat/reversed. Remaining
budget alone is not a reason. Stop when contrasts are flat/reversed with
uncertainty below +0.04 or both generic and product evidence are practically
flat/reversed. One million candidates is outside this packet.

## Current disposition

The current generated summary validates the completed target-free generic
manifest but deliberately consumes no generic target row. The separate target
packet reports selected mean `sys=0.62571` versus baseline `0.33857` and a
negative `0-.1%` versus `.1-1%` hardening contrast. The frozen continuation rule
therefore stopped at 10,000. Those numbers are repeated here only to explain
the contract's disposition; regenerate and cite them from the target packet.

The product sensitivity result remains independently useful: product `5x5`
versus `4x6` mean differences at the generated 1% and 0.1% cutoffs are both
below the predeclared `0.04` material contrast. No current ridge-tail hypothesis
requires another product bucket.
