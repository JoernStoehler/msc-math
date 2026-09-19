# Closing the 91 current-evaluator refusals

2026-09-18. **90 refusals repaired as capacity intervals; one remains unresolved.**
Together with the full run, current interval coverage is **14,335 / 14,336**
original bodies. No policy limit or scalar tolerance was relaxed.

## What ran

A separate interval-preserving adapter retains the production capacity result
before optional `capacity_value(..., 1e-10)` conversion. The original adapter,
its receipts and old failures remain unchanged. The new status is
`capacity_interval`; it does not assert an accepted scalar. Scalars and derived
numerical sys are null on tolerance refusal, while the reason and actual bounds
remain present.

- **37 tolerance failures:** unchanged inputs; all returned intervals. None met
  the original scalar tolerance, and all 37 old capacity scalars lie within the
  returned intervals. Maximum relative half-width 3.64e-9.
- **53 norm failures:** inputs multiplied by exact powers of two as documented
  in `norm/`. Every transformed body underwent fresh exact geometry reconstruction
  and normal primal/dual policy checks; all returned intervals. Of these, 44 met
  scalar tolerance and 9 did not. Maximum relative half-width 1.10e-6.
- **Two successful controls:** one generic and one product. New bounds, numerical
  volume and accepted scalar are exactly equal to the original pilot results.

92 calls in total, including controls: 1.42 seconds outer wall; no errors or
timeouts. Build separately took 65 seconds. Per-request timeout 5 seconds;
outer repair execution cap 600 seconds. All requests are retained; no retries.

## Exact transfer to original bodies

For `transform.k = k`, the new primal body is `K' = 2^k K`, with dual vertices
scaled by `2^-k`. Every coordinate was checked as an exact dyadic transformation,
not rounded to an approximate new body. Homogeneity gives

- `c(K) = c(K') / 2^(2k)`;
- `V(K) = V(K') / 2^(4k)`;
- systolic ratio unchanged.

`analyze.py` transfers capacity endpoints as exact Python Fractions and records
rational endpoints for the original body. Any exact product capacity is transferred
and checked inside those endpoints. Numerical volume is similarly rescaled;
this does not turn the f64 volume computation into certified volume. Fresh
geometry checks eliminate reliance on stored primal vertices for acceptance.

For the 53 scaled bodies, maximum relative midpoint capacity difference from the
historical scalar is 1.16e-12; maximum numerical-volume difference is 1.87e-12.
37 historical capacities lie within intervals; 16 fall outside narrow intervals
despite tiny central differences. These are retained distinct observations,
not rounded away. Certified capacity intervals still do not constitute certified
sys intervals without a volume guarantee.

## Actual remaining gap

`random_F8_s3_45` cannot satisfy policy under any positive uniform dilation:
required `t >= 0.000993373` conflicts with `t <= 0.000622186`. See `norm/README.md`
for the exact input, rational bounds and bounded diagonal-symplectic screen.
The screen found no candidate but is not a proof against general symplectic
preconditioning. No policy bypass or unsupported capacity claim was made.

A possible next bounded investigation is an exact symplectic shear followed by
power-of-two balancing, verifying exact representability and ordinary fresh
policy checks before evaluating. It is not necessary to regenerate the
population. Alternative: retain this one explicit unresolved target until a
suitable exact evaluator route is available.

Coverage now consists of 14,289 bodies with accepted 1e-10 capacity scalars,
46 additional bodies with wider certified capacity intervals, and one unresolved
body. This is a statement about current computation on matched historical
geometries, not restored historical execution or new-population replication.

## Evidence and reproduction

`build-receipt.json`, `build.log`, `run.time`, raw JSONL inputs/results,
`interval/selection.json`, `norm/selection.json` and `comparison.json` bind the
inputs, outputs and transfer calculations. Every result embeds source/binary
hashes and input hashes. New adapter lives at
`experiments/sys-datascience/methods/retrospective-interval-evaluator/`.

Build using its `build.py` into a separate target directory; preserve the old
scalar binary. Run its `evaluate.py` with `--timeout-seconds 5` and the new build
receipt on `interval/inputs.jsonl`, `norm/inputs.jsonl` and `control-inputs.jsonl`,
writing fresh output paths under a 600-second outer limit. The retained result
paths are `interval/results.jsonl`, `norm-results.jsonl`, `control-results.jsonl`.
`analyze.py` verifies exact expected ID coverage, control equality, fresh geometry
acceptance and exact interval transfer. Original failures remain in `target-full/`.

Independent review by the norm-input worker checked the interval-transfer code,
fresh geometry/policy ordering and null-scalar contract against all 90 repaired
responses. It found no defect. This review is separate from the numerical checks
and does not establish historical execution identity or exact volume.
