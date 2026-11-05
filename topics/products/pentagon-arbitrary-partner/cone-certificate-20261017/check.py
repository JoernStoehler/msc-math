#!/usr/bin/env python3
"""Exact read-only control for a scoped quadratic-certificate obstruction.

Reads the retained cone packet; prints JSON; no optimizer or producer is run.
Standard library only. All field arithmetic is independently implemented here.
"""
from fractions import Fraction
from collections import Counter
from pathlib import Path
import hashlib
import json
import sys


class F:
    def __init__(self, a=0, b=0):
        self.a, self.b = Fraction(a), Fraction(b)

    @staticmethod
    def coerce(x):
        return x if isinstance(x, F) else F(x)

    def __add__(x, y):
        y = F.coerce(y)
        return F(x.a + y.a, x.b + y.b)
    __radd__ = __add__

    def __neg__(x):
        return F(-x.a, -x.b)

    def __sub__(x, y):
        return x + -F.coerce(y)

    def __rsub__(x, y):
        return F.coerce(y) - x

    def __mul__(x, y):
        y = F.coerce(y)
        return F(x.a*y.a + 5*x.b*y.b, x.a*y.b + x.b*y.a)
    __rmul__ = __mul__

    def __truediv__(x, y):
        y = F.coerce(y)
        n = y.a*y.a - 5*y.b*y.b
        return x * F(y.a/n, -y.b/n)

    def __eq__(x, y):
        y = F.coerce(y)
        return x.a == y.a and x.b == y.b

    def __hash__(x):
        return hash((x.a, x.b))

    def sign(x):
        if x.b == 0:
            return (x.a > 0) - (x.a < 0)
        if x.a == 0:
            return (x.b > 0) - (x.b < 0)
        if (x.a > 0) == (x.b > 0):
            return 1 if x.a > 0 else -1
        d = x.a*x.a - 5*x.b*x.b
        return ((d > 0) - (d < 0)) * (1 if x.a > 0 else -1)

    def data(x):
        return [str(x.a), str(x.b)]


def dot(a, b):
    return sum((x*y for x, y in zip(a, b)), F())


def quad(q, z):
    return dot(z, [dot(row, z) for row in q])


def rank(rows):
    pivots = {}
    for row in rows:
        row = list(row)
        for j, p in sorted(pivots.items()):
            a = row[j]
            row = [x-a*y for x, y in zip(row, p)]
        nonzero = [j for j, x in enumerate(row) if x != 0]
        if nonzero:
            j = nonzero[0]
            a = row[j]
            pivots[j] = [x/a for x in row]
    return len(pivots)


def normalize(row):
    a = next(x for x in row if x != 0)
    if a.sign() < 0:
        a = -a
    return tuple(x/a for x in row)


source = Path(sys.argv[1])
payload = source.read_bytes()
data = json.loads(payload)
rows = [tuple(F(*x) for x in r['coefficients']) for r in data['rows']]
q = [[F(*x) for x in r] for r in data['positive_det_quadratic_matrix']]
z = [F(*x) for x in data['hko_control']['point']]
assert len(rows) == 610 and len(set(rows)) == 610
sqrt5 = F(0, 1)
h = (1+sqrt5)/4
l = (-5+3*sqrt5)/4
tau2 = 5-2*sqrt5


def rotate(z):
    a, b, c, d = z[3:]
    return z[:3] + [h*(a-c), h*(b-d), l*a+h*c, l*b+h*d]


def pullback(r):
    a, b, c, d = r[3:]
    return r[:3] + (h*a+l*c, h*b+l*d, h*(c-a), h*(d-b))


# Exact invariance of the WHOLE cone and target under the tenfold rotation.
bank = {r: i for i, r in enumerate(rows)}
permutation = [bank[normalize(pullback(r))] for r in rows]
assert len(set(permutation)) == 610
columns = [rotate([F(int(i == j)) for i in range(7)]) for j in range(7)]
assert all(dot(columns[i], [dot(r, columns[j]) for r in q]) == q[i][j]
           for i in range(7) for j in range(7))
assert h*h+h*l == 1  # determinant of the 2x2 rotation matrix

points, active_sets, sector_slacks = [], [], []
for k in range(10):
    points.append(z[:])
    slacks = [dot(r, z) for r in rows]
    assert all(x.sign() >= 0 for x in slacks)
    assert quad(q, z) == 0
    assert (z[3]*z[6]-z[4]*z[5]).sign() > 0
    active_sets.append([i for i, x in enumerate(slacks) if x == 0])
    t, u = z[3]+z[6], z[5]-tau2*z[4]
    sector_slacks.append([u, tau2*t-u])
    z = rotate(z)
assert z == points[0]
assert len({tuple(p) for p in points}) == 10
counts = [sum(i in a for a in active_sets) for i in range(610)]
assert max(counts) == 2 and 2*max(counts) < len(points)
v = [(a-b)/2 for a, b in zip(points[0], points[5])]
negative = quad(q, v)
assert negative.sign() < 0
sector_members = [i for i, ss in enumerate(sector_slacks)
                  if all(x.sign() >= 0 for x in ss)]
assert sector_members == [0]

print(json.dumps({
    'status': 'PASS scoped global degree-two certificate obstruction; NO positivity theorem',
    'source_path': str(source.resolve()),
    'source_sha256': hashlib.sha256(payload).hexdigest(),
    'field': 'Q(sqrt(5)); exact Fraction coefficients and ordered-field signs',
    'row_count': len(rows),
    'rotation_row_permutation_verified': True,
    'rotation_target_invariance_verified': True,
    'equality_ray_count': len(points),
    'equality_ray_span_rank': rank(points),
    'active_row_counts': list(map(len, active_sets)),
    'row_zero_count_histogram': dict(sorted(Counter(counts).items())),
    'each_row_product_positive_at_at_least': len(points)-2*max(counts),
    'negative_span_witness': [x.data() for x in v],
    'negative_span_witness_target': negative.data(),
    'sector_inequalities': ['u>=0', '(5-2sqrt(5))*t-u>=0'],
    'sector_variables': ['t=b00+b11', 'u=b10-(5-2sqrt(5))*b01'],
    'sector_equality_ray_indices': sector_members,
    'hko_sector_slacks': [x.data() for x in sector_slacks[0]],
    'active_sets': active_sets,
}, indent=2))
