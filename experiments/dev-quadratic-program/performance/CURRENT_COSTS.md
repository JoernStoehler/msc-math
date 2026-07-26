# Current QP cost constants

Use this page for order-of-magnitude experiment estimates. These are warm
release-mode measurements from 2026-07-26 on the local devcontainer. Replace
the rows after a relevant algorithm, compiler, or machine change; detailed and
historical comparisons remain in the owning result files.

| Measured unit | Input and count | Current cost |
| --- | --- | ---: |
| raw f64 one-word KKT solve | one valid full-length word, `F=5,6,7,8,9,10,11` | `6.58, 6.16, 7.10, 6.13, 6.46, 6.06, 5.62 us` |
| production validation plus exact geometry | four actual transformed generic `F=10` measurements | `288--305 ms/input` |
| exact transition graph plus cycle-enumeration diagnostic | same inputs, `892--1,487` cycles | `1.55--2.24 ms/input` |
| production general capacity after geometry | same inputs | `5.76--10.15 ms/input` |
| complete production pipeline, excluding the optional duplicate diagnostic | same inputs | `294--315 ms/input` |
| complete retained legacy pipeline | same inputs | `10.3--17.4 ms/input` |

The three-point production smoke therefore predicts about `49 s` for the
164-point generic orientation scan; the previous stopped run took about `52 s`
through that body. Geometry preparation, not capacity candidate processing,
explains that generic slowdown.

Do not extrapolate only from candidate count when exact fallback is possible.
On one transformed product-source input, the production general route processed
3,346 cycles but spent `2.609 s` in capacity. A sampled profile attributed a
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
