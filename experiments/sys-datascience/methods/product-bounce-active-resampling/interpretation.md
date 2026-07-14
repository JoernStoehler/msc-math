# Interpretation and disposition

## Direct observations

The retained smoke completed 128 accepted target evaluations from 303
proposals. The only proposal rejection was the original generator validity
boundary (175 rows); no target evaluation failed. Acceptance by base/law was
22.2%--72.7%, above the 10% stop rule. The 128 exact fixed actions all agreed
with their frozen base action. Every fixed constraint matrix had exact rank 5,
one-dimensional beta kernel, and zero-dimensional recovered base-point
solution space. The largest newly computed `sys` was `0.7831251004`; no
`sys > 1` row appeared.

For the primary fixed-rank law, all 64 remapped words remained in the current
candidate stream. The recovered-orbit/global-minimum split was:

| base (producer label) | proposal acceptance | survives and stays global | survives, takeover | recovered infeasible | all accepted log-volume span | survives+global log-volume span |
|---|---:|---:|---:|---:|---:|---:|
| `random_5x5_s0_155` (2) | 29.1% | 11 | 3 | 2 | 0.141 | 0.110 |
| `random_5x5_s2_207` (3) | 22.2% | 13 | 0 | 3 | 0.102 | 0.102 |
| `random_5x5_s3_145` (2) | 72.7% | 10 | 0 | 6 | 1.308 | 1.308 |
| `random_5x5_s2_243` (3) | 69.6% | 2 | 3 | 11 | 0.522 | 0.129 |

Thus fixed-word recovered survival and exact takeover are separately
observable: 36/64 stayed recovered-feasible and global, 6/64 were
recovered-feasible but lost to another exact word, and 22/64 failed the
recovered-orbit halfspace check. Minimum inactive-facet slack ranged from
`-0.2321` to `0.2884` over primary rows, exposing both sides of the failure
boundary.

The unlabeled-support sensitivity is also implementable. It produced 18/64
recovered-feasible/global rows, 5/64 recovered-feasible takeovers, 31/64
recovered-infeasible rows, and 10/64 rows where the remapped word disappeared
from the candidate stream. Its larger failure rate is a substantive effect of
marginalizing rank allocations, not a plumbing failure.

Detailed per-base/law acceptance, clearance, global bounce labels, exact
takeover identities, and volume/`sys` ranges are generated in
`artifacts/summary.json` and `artifacts/proposals.jsonl`.

## What this changes

Both conditional laws are faithful to their stated finite-generator objects
and affordable at smoke scale. The primary law is cleaner: fixed ranks remove
one combinatorial source of loss and retain the fixed word in the candidate
stream on all accepted geometries. The sensitivity law is useful as a boundary
check but should not replace it as the main law.

Volume movement is large enough that a later multi-base design is technically
plausible: one target-blind two-bounce base spans 1.308 in log volume even
among rows where the recovered word remains global, while the other primary
bases span 0.102--0.129 there. This heterogeneity is itself a warning against
claiming a class contrast from these four bases.

Nonzero volume movement is not mechanism evidence. At fixed normals, the
first variation of a factor's area with respect to support height `h_i` is the
active edge length. While the frozen word remains admissible and global,
`d log(sys) / d h_i = -edge_length_i / area_factor`. Generic inactive-height
variation therefore already predicts volume and fixed-branch `sys` movement.
The informative observations here are clearance, recovered survival, and
takeover, not the existence of volume variation.

## Limitation and exact repair

The present public surfaces do not provide one authoritative exact physical
admissibility/clearance result for an arbitrary supplied word. The packet can
separate exact KKT action/rank, transition-pruned candidate presence, f64
recovered-orbit halfspace slack, and exact candidate-stream minimality, but it
must not collapse them. Before a full statistical packet makes physical
survival claims, add or validate a supplied-word API that returns exact or
explicitly bounded blocker slack/closed-tube feasibility and identifies the
first blocking inactive facet. No shared API change was needed for this smoke.

## Recommendation

Defer a full statistical resampling packet. Run a controlled one-parameter
inactive-support-height family next, with fixed normals and the current active
word, on a target-blind high-clearance base and a boundary-near base. Freeze
the event definitions in advance and locate the first recovered-feasibility or
takeover event. This directly tests the generic area derivative, supplies an
actual parameter-space failure-event distance, and resolves the diagnostic
contract more cleanly than another simultaneous four-coordinate resample.

Reopen full fixed-rank resampling only after that event contract is validated
and the study can afford enough independently selected base rows for the base,
not the resample, to be the inferential unit. Retain the unlabeled law as a
sensitivity analysis. Do not abandon the route: the laws are feasible, the
cost is low, and the observed event mixture has information value.
