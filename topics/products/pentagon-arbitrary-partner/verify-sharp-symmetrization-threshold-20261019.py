# /// script
# requires-python = ">=3.11"
# dependencies = ["sympy>=1.13"]
# ///
"""Read-only exact checks for the sharp trapezoid symmetrization threshold.

Uses the existing ordered QQ(tau) embedding helper; this is not an independent
implementation of its algebraic-field arithmetic or sign-isolation routine.
"""
from __future__ import annotations

import importlib.util
import json
from pathlib import Path

if not __debug__:
    raise RuntimeError("Run without -O")

fixture_path = Path(__file__).parent / "local-cover" / "pentagon_fixture.py"
spec = importlib.util.spec_from_file_location("pentagon_fixture", fixture_path)
fixture = importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(fixture)

F, zero, one = fixture.F, fixture.Z, fixture.O
sqrt5, tau = fixture.s5, fixture.tau
h = (one + sqrt5) / F(4)
c = tau * (F(3) + sqrt5) / F(4)  # cos(pi/10)=sin(2pi/5)
q = tau / sqrt5                 # tan(pi/10)
threshold = (F(3) - sqrt5) / F(2)
assert tau / (F(2) * c) == threshold
assert F(2) * c * q == one - threshold

normals = [
    [one, zero],
    [(sqrt5 - one) / F(4), c],
    [-h, tau * h],
    [-h, -tau * h],
    [(sqrt5 - one) / F(4), -c],
]
partner = [[-one, zero], [one, zero],
           [threshold, -F(2) * c], [-threshold, -F(2) * c]]
difference_half = fixture.hull([
    fixture.scale(one / F(2), fixture.sub(a, b))
    for a in partner for b in partner
])

# All ten circumradius-one decagon vertices lie in the actual difference hull.
for normal in normals:
    for direction in [one, -one]:
        point = fixture.scale(direction, normal)
        for i, vertex in enumerate(difference_half):
            edge = fixture.sub(difference_half[(i + 1) % len(difference_half)], vertex)
            assert fixture.sign(fixture.cross(edge, fixture.sub(point, vertex))) >= 0

# Its horizontal support is exactly one, so the width-normalized decagon
# cannot be enlarged. Both horizontal exposed faces of K are singletons.
assert max(fixture.sign(p[0] - one) for p in difference_half) == 0
assert sum(p[0] == one for p in partner) == 1
assert sum(p[0] == -one for p in partner) == 1

# The two triangles with endpoints (+/-1,0) and apex (0,+/-tau) are W-scaled
# normal templates, W=2(1+h). Their edge directions are n_0,n_2,n_3.
assert fixture.scale(one / h, normals[2]) == [-one, tau]
assert fixture.scale(one / h, normals[3]) == [-one, -tau]
assert h * (F(2) + F(2) / h) == F(2) * (one + h)

print(json.dumps({
    "status": "PASS exact trapezoid geometry and threshold arithmetic",
    "field": "ordered QQ(tau), tau=tan(pi/5) in (7/10,3/4)",
    "threshold": "(3-sqrt(5))/2",
    "width": "2(1+cos(pi/5))",
    "partner_vertices": [[fixture.serialize(v) for v in p] for p in partner],
    "half_difference_vertices": [[fixture.serialize(v) for v in p]
                                 for p in difference_half],
    "checked": ["all ten decagon vertices contained in actual half-difference hull",
                "horizontal width prevents enlargement",
                "unique horizontal exposed points of partner",
                "upward and downward normal-template membership",
                "exact threshold identity"],
    "analytic_input": "Minkowski support-face calculation and universal plateau theorem",
}, indent=2))
