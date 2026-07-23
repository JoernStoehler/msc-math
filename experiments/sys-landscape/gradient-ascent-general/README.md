# Legacy General-Body Ascent

Question: did the original fixed-facet-count general-polytope ascent panel find
an endpoint with `sys > 1`?

The raw summary and trace JSONL files are no longer retained. The preserved
answer in `../legacy-ascent-continuation-debt.md` is: 10 seeds, maximum final
`sys = 0.9030`, no endpoint above `1`, and escape logic used in every seed.
The six tracked figures are legacy views of that removed panel; they are not
backed by current local analyzer inputs.

`main.rs` and the job scripts remain as historical producer machinery. The
bounded local script writes temp outputs:

```bash
bash experiments/sys-landscape/gradient-ascent-general/job-smoke.sh
```

Do not treat the endpoints as certified local maxima or the ten-seed result as
a no-improvement theorem. Reopening fixed-`F` optimization requires a new
packet and evidence policy; the preserved design requirements are in the debt
note.
