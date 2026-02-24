# Dismissal Error Bound Experiment

## Goal

Empirically confirm that the value loss from near-singular system dismissal
(Algorithm A.4 in the thesis, `alg:near-singular-handling`) is negligible.

## What to measure

For each polytope in the test dataset, run the capacity algorithm and record,
for every dismissed (S, sigma) pair:

1. The error bound from equation (A.3) / `eq:dismissal-error-bound`
2. The singular value sigma_j that triggered dismissal
3. The final capacity value

Then check: is the maximum error bound across all dismissed pairs negligible
compared to the final capacity? Report the ratio max_bound / capacity.

## Status

TODO. Not yet implemented.
