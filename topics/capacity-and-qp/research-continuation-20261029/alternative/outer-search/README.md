# Bounded vertical-offset outer search

These scratch runs use the ten exact normal-triangle templates from
`topics/products/pentagon-arbitrary-partner/local-cover/pentagon_fixture.py`,
converted to floating point for a numerical outer search. At each proposed
relative vertical-offset vector, the script solves a horizontal-slide envelope
LP. The outer objective coalesces raw vertex heights within `1e-6` to avoid
ill-conditioned slope rows. That coalescing perturbs the LP model. The LP's
proposed horizontal shifts for each running best were then evaluated on the
original uncoalesced point set by an independent `scipy.spatial.ConvexHull`.

The unrestricted-offset run made 2,652 LP calls in 18.76 seconds wall time.
All twelve Powell starts stopped at the 220-evaluation cap. The explicit-box
run searched `[-1,1]^9`, made 2,606 LP calls in 18.80 seconds wall time; the
HKO start terminated successfully after 174 evaluations, while the other eleven
starts stopped at the 220-evaluation cap. These are wall times, not CPU-time
measurements. The starts were HKO, negated HKO offsets, centered, two rational
remote vectors, alternating, linear, reverse linear, and four deterministic HKO
perturbations.

Both records' best directly rescored hull is the HKO area to floating precision
(`0.20081141588622725`, ratio `0.9999999999999999`); it occurs at the initial
HKO vertical offsets. The direct hull has five vertices. No below-HKO candidate
was found. Powell's cap terminations do not certify local convergence, and the
box `[-1,1]^9` is only an explicit search restriction, not a proved bound on all
competitive covers. Neither run proves a global minimum or a global obstruction.

An earlier exploratory run kept near-equal heights distinct. HiGHS reported
success on a model with a gap about `1.4e-17`, but its endpoint slopes reached
about `1e16`; the purported area `0.1453` was false as an original-cover
improvement. The returned horizontal placement had direct hull area about
`1.809` times HKO. That failed run is not included in the result JSONs.

Files:

- `search.py`, `results.json`: unrestricted-offset bounded triage.
- `search_box.py`, `results_box.json`: triage restricted to `[-1,1]^9`.

The exact common-direction LP producer and its exact primal/dual verifier are in
`../parallel_envelope.py`; these outer runs do not invoke exact certification.
