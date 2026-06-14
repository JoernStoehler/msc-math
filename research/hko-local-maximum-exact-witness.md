# Exact Witness Checker

Older exact-route tooling for the HKO2024 `M_10` local-maximality work. This
is not the current theorem certificate.

## Status

Packet 1 completed:

- exact HKO geometry and the explicit symmetry tangent basis are implemented in
  `check.sage`;
- the local devcontainer now exposes a working `sage` command via the baked
  Miniforge / conda-forge install in `.devcontainer/Dockerfile`;
- the same Packet 1 artifacts remain runnable via `check_packet1.py` as a
  pure-Python fallback and for smaller scaffolding packets;
- current numerical exact-minimum surfaces can be summarized with
  `summarize_numerical_minima.py`;
- Packet 2 numerical reconciliation now has a durable bookkeeping surface via
  `classify_numerical_minima.py`;
- the old route's orbit catalog is not closed and remains the main blocker
  between this exact tooling and a final proof surface.
- the exact HKO dual-coordinate volume row is now derived in closed form and
  emitted as a dedicated Packet 3 support artifact.
- the reduced endpoint/midpoint prototype `sys` rows are now emitted exactly in
  the Packet 3 `R^40` coordinate order.
- widened representative-row artifacts were removed from the live tree after
  `smooth-only-rank-defect/` became the maintained evidence surface for the
  clean failed smooth-only route attempt; use git history for the deleted
  widened-route machinery.

## Files

| File | Role |
|---|---|
| `check.sage` | SageMath entrypoint for exact HKO geometry and symmetry-tangent certificates |
| `check_packet1.py` | Pure-Python sympy fallback for the Packet 1 exact geometry / symmetry certificates |
| `summarize_numerical_minima.py` | Cross-check summary of the current numerical exact-minimum surface |
| `classify_numerical_minima.py` | Packet 2 bookkeeping classifier for six-facet endpoint classes and seven-facet equality-case classes |
| `derive_endpoint_prototype.py` | Exact Packet 2 certificate for one endpoint prototype, one midpoint prototype, and the full equality-case beta segment between neighboring endpoints |
| `derive_segment_gradient_reduction.py` | Exact Packet 2 certificate that the seven-facet KKT segment gives an affine height-gradient family |
| `derive_segment_a_gradient_reduction.py` | Exact Packet 2 certificate that the seven-facet KKT segment gives a degree-2 dual-vertex row family spanned by three prototype rows |
| `derive_hko_volume_derivative.py` | Exact Packet 3 support script deriving the HKO dual-coordinate volume row in facet-major `R^40` order |
| `derive_reduced_sys_prototypes.py` | Exact Packet 3 support script combining prototype capacity rows with the HKO volume row to emit exact prototype `sys` rows |
| `hko-geometry.json` | Generated exact geometry record |
| `hko-volume-derivative.json` | Generated exact HKO volume-row certificate |
| `reduced-sys-prototypes.json` | Generated exact reduced prototype `sys` rows and their interpolation/coincidence checks |
| `hko-symmetry-tangent.json` | Generated exact symmetry tangent-space certificate |
| `numerical-minima-summary.json` | Generated current numerical minima summary |
| `numerical-family-reconciliation.json` | Generated Packet 2 bookkeeping summary of endpoint/equality-case classes |
| `endpoint-prototype-certificate.json` | Generated exact endpoint/midpoint prototype beta/action certificate |
| `segment-gradient-reduction.json` | Generated exact gradient-reduction certificate for the equality-case KKT segment |
| `segment-a-gradient-reduction.json` | Generated exact dual-vertex row-reduction certificate for the equality-case KKT segment |

## How To Run

```bash
cd experiments/hko-local-maximum/history/exact-witness
sage check.sage
python3 check_packet1.py
python3 summarize_numerical_minima.py
python3 classify_numerical_minima.py
python3 derive_endpoint_prototype.py
python3 derive_segment_gradient_reduction.py
python3 derive_segment_a_gradient_reduction.py
python3 derive_hko_volume_derivative.py
python3 derive_reduced_sys_prototypes.py
```

## Scope Boundary

`check.sage` and `check_packet1.py` currently close Packet 1 only:

- exact dual-vertex coordinates;
- actual coefficient field;
- explicit `R^40` symmetry tangent basis and exact rank.

It does **not** yet close the full theorem route, because the final
paper-derived orbit catalog and exact active-gradient matrix are still pending.

## Field Note

Packet 1 confirms that the exact dual geometry lives in the quartic field
`Q(tan(pi/5))`, where `tan(pi/5)` satisfies `t^4 - 10 t^2 + 5 = 0`.

This quartic field is not an avoidable normalization artifact:

- the common support height is quadratic;
- the standard-coordinate facet normals already contain `sin(pi/5)` terms
  outside `Q(sqrt(5))`;
- dividing by the support height and globally rescaling the primal polytope do
  not reduce the dual-vertex coordinates to `Q(sqrt(5))`.

## Packet 2 Note

The current numerical exact-minimum artifact splits into two visible surfaces:

- `44` six-facet exact minima, corresponding to the older endpoint-style
  numerical story;
- `106` seven-facet exact minima, consistent with equality-case trajectories
  produced by the paper's three-bounce minimizing-family remark.

The numerical reconciliation script records the current evidence that the
seven-facet gradient classes lie on segments between neighboring six-facet
endpoint classes. This is bookkeeping support for Packet 2, not yet the final
paper-derived theorem-facing catalog.

At the current numerical level, the beta-pattern surface compresses much
further than the raw gradient-class counts:

- the six-facet exact minima use one beta multiset up to floating jitter;
- the seven-facet exact minima appear numerically as two beta multisets, but
  the exact prototype certificate now shows that these are representatives on
  one constant-action segment between neighboring endpoint beta profiles.
- for each seven-facet class, the current reconciliation artifact also records
  a facetwise beta-profile convex-combination witness against two neighboring
  six-facet endpoint classes.

The current symmetry-reduced prototype hypothesis is summarized in:

- `research/hko-local-maximum.md`
