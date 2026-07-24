# Legacy Product-Body Ascent

Question: did the original fixed-facet-count Lagrangian-product ascent panel
find an endpoint with `sys > 1`?

The raw summary and trace JSONL files are no longer retained. The preserved
answer in `../legacy-ascent-continuation-debt.md` is: 12 seeds, maximum final
`sys = 0.8727`, and no endpoint above `1`. The six tracked figures are legacy
views of that removed panel; they are not backed by current local analyzer
inputs.

`main.rs` and the job scripts remain as historical producer machinery. The
bounded local script writes temp outputs:

```bash
bash experiments/sys-landscape/gradient-ascent-products/job-smoke.sh
```

This result is limited to the old sampler, projected ascent policy, and seed
panel. Reopening product optimization requires a new packet and evidence
policy; active random-product data instead starts at
`../../polytope-datasets/README.md`.
