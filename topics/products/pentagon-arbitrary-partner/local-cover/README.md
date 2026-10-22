# Local-cover exact check

This directory contains the exact finite check accompanying
[`../local-cover-rigidity.md`](../local-cover-rigidity.md). It reconstructs the
ten normal triangles for the fixed regular pentagon, verifies the HKO placement,
and checks the positive 50-marginal relation and rank-18 translation quotient.

The finite check does not certify continuum implications. The separate
[22 October review](continuum-review-20261022.md) proves them using vertex disks,
positive slot marginals and unique-placement compactness; the
[capacity applicability check](capacity-interface-check-20261022.md) checks the
imported theorem and complete template interface. The local research result is
not promoted to a thesis theorem and has no established human acceptance.

Run from this directory with a fresh output path:

```sh
uv run --with sympy==1.14.0 python verify_local_cover.py \
  --fixture-code pentagon_fixture.py --out /tmp/local-cover-check.json
```

The retained 24 September 2026 output is in `verification/20260924.json`.

The [22 October rerun](verification/20261022/report.md) retains exact source hashes,
environment, result and measured process costs. It matches the September output
apart from elapsed time.
