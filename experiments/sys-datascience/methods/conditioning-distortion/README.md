# Conditioning-distortion audit

This target-free packet measures how bounded retry, all-active-facet checks,
boundedness/origin checks, and exact side-count reconstruction change the law
that a generator actually returns.  It records one JSONL row per proposal, not
only accepted rows.  A bounded retry stream is therefore reported as a
conditional (and, when it exhausts, censored) law.

The proposal formulas are copied from the reviewed generator-zoo implementation
at source revision `fd9c3e7d`, source blob
`ea59cb1b3d123e630fdc034a95f4a2a43812b0a6`:
`current_baseline`, `zonogon`, `primal_hull`, `repulsive_gap`,
`regular_mutation`, `from_vertices`, and `normalize`.  The product producer was
also inspected at blob `9a15d5545efd85a5396fa818d7e604ffaab46b9c`.  The packet
keeps the factor-level proposal boundary explicit because the product producer
does not expose the intermediate factor candidate or a reason taxonomy; a
faithful product-level audit would require producer redesign.  Product-law
conditioning is consequently abandoned here rather than inferred from an
accepted-product table.

## Retained and abandoned strata

Retained candidate-level laws are:

* current baseline, `delta=0.2`;
* Dirichlet-gap controls `alpha=1,4,16` and exact regular;
* zonogon, `lengths=uniform(0.5,1.5)` (even side counts only);
* primal-hull uniform disk, `points=n+4,origin=interior`;
* four-step regular mutation, `scale=0.03`.

The surface-area-closure law is abandoned because no faithful edge-measure
proposal is present.  Smooth finite-mode support fields and centroid-polar
pushforwards are abandoned because the pre-acceptance body law is not a
candidate-level law in the inspected producer.  The generic 4D and product
producers are not called: this packet has no target, `sys`, or capacity input.

## Reproduce

From a clean committed checkout (the clean-source guard is fail-closed):

```text
python3 conditioning_audit.py \
  --out-dir artifacts/smoke --seed 20260715 \
  --attempts 32 --rows-per-stratum 12 --sides 3,4,6
python3 -m pytest -q test_conditioning_audit.py
```

The smoke emits `artifacts/smoke/attempts.jsonl` and
`artifacts/smoke/report.json`.  Each attempt row has a deterministic
`sample_id`, source seed/stratum, primitive proposal features, one terminal
reason, and accepted-body features only when the reason is `accepted`.
Undefined primitive values (for example a log spread of a proposal with a
non-positive support) are JSON `null`; they are not imputed.

The report includes attempts per accepted draw, reason counts and proportions,
acceptance by maximum-gap bins, accepted-versus-proposed primitive means and
quantiles, accepted-body feature means, and a deterministic calibration:

* a known `U(0,1)` rule accepting `x<0.25` recovers acceptance near `0.25` and
  accepted mean near `0.125`;
* an always-accept control has zero mean shift; and
* a corrupted reason-code control is required to fail closed.

No retained volatile timings are written.  The report binds the producer hash,
repository revision, source blob IDs, command, seed, stratum, and retry bound.
Run a second invocation into a different directory and compare the two
`attempts.jsonl` files byte-for-byte for full replay; independent seeds must be
compared as separate streams, never pooled as extra independent factors.

## Interpretation boundary

Allowed: quantify selection cost and reason composition; expose association
between acceptance and proposal primitives; and describe accepted-vs-proposed
diagnostic shifts for this finite deterministic smoke.  Rejected candidates have
no body metric.  Reasons remain separate: invalid geometry, inactive prescribed
facets, unbounded/origin failure, wrong side count, exact reconstruction/
incidence failure, and bounded-attempt exhaustion are not pooled.

Prohibited: a density or mathematical-support claim; claiming that an accepted
row is an unconditional generator draw; treating exhausted rows as accepted;
claiming target/`sys`/capacity association; or treating a paired product's two
factors as independent.  The current packet is a bounded method/reuse smoke,
not thesis evidence.  Reopen when the producer exports a product-level
candidate/reason stream or a new law has a defined pre-conditioning geometry.
