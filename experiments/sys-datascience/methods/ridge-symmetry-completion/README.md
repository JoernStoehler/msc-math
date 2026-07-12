# Regular 3x6 symmetry completion

Status: complete and independently accepted for further research reuse. The
two interior geometries were target-free frozen together. Sequential evaluation found
`sys(delta1)=0.9664541398260563 < 1`, so the frozen `delta2` call was permitted;
it found `sys(delta2)=0.9827820617202804 < 1`.

This bounded extension asks whether the regular triangle--hexagon path follows
the predicted three-bounce secant branch from the durable `.951` row to the
theorem-level endpoint `sys(pi/6)=1`, or overshoots before returning. It does
not estimate a population frequency or validate a general proposer.

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

The bounded scientific disposition is in `INTERPRETATION.md`: park further
empirical densification and hand the remaining question to an all-branch proof.
