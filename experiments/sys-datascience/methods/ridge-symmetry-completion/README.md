# Regular 3x6 symmetry completion

Status: complete and independently accepted for further research reuse. The
two interior geometries were target-free frozen together. Sequential evaluation found
`sys(delta1)=0.9664541398260563 < 1`, so the frozen `delta2` call was permitted;
it found `sys(delta2)=0.9827820617202804 < 1`.

The mathematical handoff now proves that the regular triangle--hexagon path
satisfies `sys(delta) <= (3/4)sec(delta)^2 <= 1` throughout its fundamental
interval. The proof uses an explicit admissible three-bounce billiard as a
capacity competitor and a separate lower bound for the exact endpoint
`sys(pi/6)=1`; see `MATHEMATICAL-HANDOFF.md`. The numerical extension tests the
stronger, optional question of whether that competitor is actually minimizing
at the sampled interior angles. It does not estimate a population frequency or
validate a general proposer.

The frozen points are one-third and two-thirds of the remaining angular gap:

- `delta1 = 0.4929785313028738`;
- `delta2 = 0.5082886534505863`.

`design_candidates.py` makes no capacity or target call. The preflight mode
checks both frozen product geometries before either target call. The evaluator
links each result to the candidate file, preflight artifact, evaluator source,
and the accepted capacity-implementation manifest owned by
`../ridge-endpoint-path/`.

`check_packet.py` checks the frozen identities, preflight and implementation
links, target arithmetic, strict sub-one results, and agreement with the frozen
branch prediction without making target calls.

The bounded scientific disposition is in `INTERPRETATION.md`: Viterbo's
inequality is proved on this path, so park further empirical densification.
An all-branch proof is optional and would serve only the stronger exact-profile
claim on the unsampled interior.
