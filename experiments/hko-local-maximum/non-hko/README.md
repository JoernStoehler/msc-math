# Non-HKO ten-facet local maximum

This packet owns the exact proof candidate for the rational hexagon--quadrilateral
product (P_*). It is separate from the HKO certificate because it proves a
different local result: (P_*) has ratio (1), is not affinely equivalent to
HKO, and locally maximizes the ratio among convex four-polytopes with ten
genuine facets.

The reader-facing theorem is in the selected thesis, in the subsection
`A second ten-facet local maximum` of `thesis/chapters/07-hko.tex`. The detailed
proof is [PROOF.md](PROOF.md); the fixed rational witness is
`certificate/certificate_base.json`.

## Trust boundary

The verifier uses exact rational and rational-function arithmetic. Its feasible
HK sections provide upper bounds on the actual ratio; the product enumeration
is used separately to establish equality on the displayed two-parameter family.
The analytic uniform-neighborhood argument is part of `PROOF.md`, not supplied
by exact arithmetic alone. The independent checker in `code/independent_audit.py`
recomputes the geometry, volume derivatives, section gradients and exceptional
quadratic direction without importing the submitted verifier modules. This is
independent implementation evidence, not proof-assistant formalization or human
refereeing.

No numerical neighborhood radius is claimed. The result does not prove a
global maximum, uniqueness among all ten-facet bodies, or any theorem when the
facet count changes.

## Reproduction

From this directory, each output directory must be new or empty:

```sh
uv run --with sympy==1.14.0 python verify.py --output-dir /tmp/non-hko-exact
uv run --with sympy==1.14.0 python code/independent_audit.py \
  --source . --rerun /tmp/non-hko-exact \
  --output /tmp/non-hko-independent.json
```

The 24 September 2026 rerun is retained under `verification/`, including the
summary and console logs. Its exact outcomes include 624 product orderings,
24 positive sections, gradient rank 22, two equality parameters modulo
symmetries, full chart rank 40, and the verified upper-section coefficient
(-13/432) in the remaining transverse direction.
