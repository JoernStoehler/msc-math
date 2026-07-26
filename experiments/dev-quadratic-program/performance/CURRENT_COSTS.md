# Current QP cost constants

Use this page for order-of-magnitude experiment estimates. These are warm
release-mode measurements from 2026-07-26 on the local devcontainer. Replace
the rows after a relevant algorithm, compiler, or machine change; detailed and
historical comparisons remain in the owning result files.

| Measured unit | Input and count | Current cost |
| --- | --- | ---: |
| raw f64 one-word KKT solve | one valid full-length word, `F=5,6,7,8,9,10,11` | `6.58, 6.16, 7.10, 6.13, 6.46, 6.06, 5.62 us` |
| production validation plus certified/exact geometry | twelve actual transformed generic `F=10` measurements in three warm four-point runs | `4.72--4.74 ms/input` |
| exact transition graph plus cycle-enumeration diagnostic | same inputs, `892--1,487` cycles | `1.53--1.55 ms/input` |
| production general capacity after geometry | same inputs | `6.53--6.70 ms/input` |
| complete production pipeline, excluding the optional duplicate diagnostic | same inputs | `11.25--11.44 ms/input` |
| complete retained legacy pipeline | paired four-point run on the same inputs | `12.29 ms/input` |
| f64 volume on exact-derived incidence | one cold run-local HKO producer miss | `0.078 ms` |
| exact-rational volume reference | one accepted generic `F=10` case | `1.049 s` |

Before the 2026-07-26 exact-arithmetic audit, generic exact enumeration and
generic rational row reduction made geometry alone cost `288--305 ms/input`;
the complete production pipeline cost `294--315 ms/input`. The current
caller-shaped result is about `26--28x` faster and predicts about `1.9 s` for
the 164-point generic orientation scan instead of the previously observed
roughly `52 s`. All paired smoke outputs retained the same capacity value
(`delta=0` in the caller output).

The same audit removed exact rational volume and repeated exact source
reconstruction from the fixed-shape scan's ordinary f64 path. One generic
one-evaluation process fell from `1.31 s` to `0.12 s`; its volume, capacity,
and `sys` were bit-identical. The product identity capacity was also
bit-identical; f64 incidence volume changed by about `7e-15` absolute and
`sys` by about `5e-16`. The broader retained 512-row volume comparison is
documented in
`experiments/sys-datascience/methods/generic-ridge-tail-stage1/artifacts/smoke-summary.json`.
That paired 512-row audit measured a `16,427x` aggregate worker-time ratio;
the two single-row timing entries above are cost anchors from different inputs,
not a paired speedup measurement.

Do not extrapolate only from candidate count when exact fallback is possible.
On one transformed product-source input, the production general route processed
3,346 cycles but spent `2.632 s` in capacity after the geometry repair. A
sampled profile attributed a
large share to `general::exact_decision -> solve_kkt_exact`; the public
production result currently does not expose the exact-fallback count needed for
a better scaling model.

Reproduce the kernel row:

```bash
cargo bench -p symplectic --bench profiling 'kkt_single' -- \
  --warm-up-time 0.05 --measurement-time 0.15 --sample-size 10
```

Reproduce the caller-shaped smoke after building
`sys-fixed-shape-orientation-search`:

```bash
target/release/sys-fixed-shape-orientation-search \
  --source-kind generic \
  --maximum-evaluations 3 \
  --capacity-backend production \
  --profile-stages \
  --output /tmp/qp-orientation-production-smoke.jsonl
```

Use `--capacity-backend legacy` for the paired control. The smoke CLI writes
per-evaluation stage timings and candidate counts to its JSONL output.
