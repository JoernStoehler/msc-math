# Routing report

For `sys>1` search, the line-level decision is to retain ridge as a coarse
screen, not an optimization objective. The generic 1% filter substantially
raises typical `sys`, but harder ridge conditioning does not improve the
selected region. Any second stage must contribute information beyond lower
ridge; the evidence does not require that information to be non-scalar.

The decomposition is numerically checked on all 200 generic rows and 142
retained product `5x5` rows. `sum(E*kappa)=sum(A_symp)` has maximum relative
error below `2e-16`; the computed normalized means/sums agree with the source
feature values to below `8e-15`. No face is unordered or degenerate, every
product row has exactly 25 mixed and 10 structural-zero faces, and
translation/positive-scaling checks are below `5e-11` absolute area error.
`validation.json` binds the input provenance hash and records exact counts and
tolerances.

The generic selected panel has a strong coarse transfer but no interior
hardening. Both normalized ordinary Euclidean area and weighted `kappa` are
lower in the selected gate, so lower `kappa` reinforces lower symplectic area;
neither component is isolated as a causal mechanism. The first ten selected
rows have lower `sys` despite slightly higher normalized ordinary area and
lower `kappa`, and selected-panel rank associations are descriptive and
post-target. Product rho-only and ridge-only arms both beat their frozen
controls, while the eight-row overlap has wide small-sample uncertainty; this
is a discordant-arm observation, not authorization to tune an intersection.

Routing:

- Do not run another generic scalar decade or a new target-bearing rho arm.
- If the line reopens, preserve both normalized ordinary-area and mixed-face
  `kappa` summaries under a source-preserving design. No intervention is
  selected by this packet.
- Hand the bounce line nothing from this packet yet: the available action
  artifacts lack a source-identifiable row join to these random `5x5` IDs.
- If a future fixed matching/stratification collapses both arms, stop the
  mechanism line with the supported statement “strong coarse enrichment, no
  supported scalar mechanism.”
