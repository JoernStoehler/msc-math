# Exact Clarke Checker

Exact first-order proof tooling for the HKO2024 `M_10` local-maximality route.

## Status

Packet 1 completed:

- exact HKO geometry and the explicit symmetry tangent basis are implemented in
  `check.sage`;
- the same Packet 1 artifacts are currently runnable in this devcontainer via
  `check_packet1.py`, because this Ubuntu base image does not provide a
  directly installable `sagemath` package by name;
- current numerical exact-minimum surfaces can be summarized with
  `summarize_numerical_minima.py`;
- the theorem-facing orbit catalog is not closed yet and remains the main
  blocker between the current exact tooling and the final proof surface.

## Files

| File | Role |
|---|---|
| `check.sage` | SageMath entrypoint for exact HKO geometry and symmetry-tangent certificates |
| `check_packet1.py` | Pure-Python sympy fallback for the Packet 1 exact geometry / symmetry certificates |
| `summarize_numerical_minima.py` | Cross-check summary of the current numerical exact-minimum surface |
| `hko-geometry.json` | Generated exact geometry record |
| `hko-symmetry-tangent.json` | Generated exact symmetry tangent-space certificate |
| `numerical-minima-summary.json` | Generated current numerical minima summary |

## How To Run

```bash
cd experiments/hko-local-maximum/exact-clarke
sage check.sage
python3 check_packet1.py
python3 summarize_numerical_minima.py
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
