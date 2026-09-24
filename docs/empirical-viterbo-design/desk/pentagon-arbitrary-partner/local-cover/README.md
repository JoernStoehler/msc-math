# Local-cover exact check

This directory contains the exact finite check accompanying
[`../local-cover-rigidity.md`](../local-cover-rigidity.md). It reconstructs the
ten normal triangles for the fixed regular pentagon, verifies the HKO placement,
and checks the positive 50-marginal relation and rank-18 translation quotient.

The check does not certify the signed-polygon winding estimate or the passage
from placements to all nearby convex bodies. The result therefore remains a
research-note candidate and is not promoted to a thesis theorem.

Run from this directory with a fresh output path:

```sh
uv run --with sympy==1.14.0 python verify_local_cover.py \
  --fixture-code pentagon_fixture.py --out /tmp/local-cover-check.json
```

The retained 24 September 2026 output is in `verification/20260924.json`.
