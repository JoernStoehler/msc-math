#!/usr/bin/env python3
"""Exact strict separator for one fixed cone sector; standard library only.

Reads the immutable cone input, checks a rational moment witness over Q(sqrt5),
prints a JSON receipt, and performs no solve, producer run, or file write.
"""
from fractions import Fraction
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
        return F(x.a+y.a, x.b+y.b)
    __radd__ = __add__

    def __neg__(x):
        return F(-x.a, -x.b)

    def __sub__(x, y):
        return x + -F.coerce(y)

    def __rsub__(x, y):
        return F.coerce(y) + -x

    def __mul__(x, y):
        y = F.coerce(y)
        return F(x.a*y.a+5*x.b*y.b, x.a*y.b+x.b*y.a)
    __rmul__ = __mul__

    def __truediv__(x, y):
        y = F.coerce(y)
        n = y.a*y.a-5*y.b*y.b
        return x*F(y.a/n, -y.b/n)

    def __eq__(x, y):
        y = F.coerce(y)
        return x.a == y.a and x.b == y.b

    def sign(x):
        if x.b == 0:
            return (x.a > 0)-(x.a < 0)
        if x.a == 0:
            return (x.b > 0)-(x.b < 0)
        if (x.a > 0) == (x.b > 0):
            return 1 if x.a > 0 else -1
        d = x.a*x.a-5*x.b*x.b
        return ((d > 0)-(d < 0))*(1 if x.a > 0 else -1)

    def data(x):
        return [str(x.a), str(x.b)]


def dot(a, b):
    return sum((x*y for x, y in zip(a, b)), F())


def rank(a):
    a = [list(r) for r in a]
    k = 0
    for j in range(len(a[0])):
        selected = next((i for i in range(k, len(a)) if a[i][j] != 0), None)
        if selected is None:
            continue
        a[k], a[selected] = a[selected], a[k]
        v = a[k][j]
        a[k] = [x/v for x in a[k]]
        for i in range(k+1, len(a)):
            v = a[i][j]
            a[i] = [x-v*y for x, y in zip(a[i], a[k])]
        k += 1
        if k == len(a):
            break
    return k


def positive_ldl_pivots(a):
    a = [list(r) for r in a]
    pivots = []
    assert all(a[i][j] == a[j][i] for i in range(len(a)) for j in range(len(a)))
    for k in range(len(a)):
        pivot = a[k][k]
        assert pivot > 0
        pivots.append(pivot)
        for i in range(k+1, len(a)):
            for j in range(k+1, len(a)):
                a[i][j] -= a[i][k]*a[k][j]/pivot
    return pivots


source = Path(sys.argv[1])
payload = source.read_bytes()
source_hash = hashlib.sha256(payload).hexdigest()
assert source_hash == '9f02548e305ed586b83b2709719818281847ce2d2e5b66981b35973acc07e9b6'
data = json.loads(payload)
rows = [tuple(F(*x) for x in r['coefficients']) for r in data['rows']]
q = [[F(*x) for x in r] for r in data['positive_det_quadratic_matrix']]
z0 = [F(*x) for x in data['hko_control']['point']]
s = F(0, 1)
tau2 = 5-2*s
rows += [(F(), F(), F(), F(), -tau2, F(1), F()),
         (F(), F(), F(), tau2, tau2, F(-1), tau2)]
assert len(rows) == 612 and z0[6] == 1
assert all(q[i][j] == q[j][i] for i in range(7) for j in range(7))
assert dot(z0, [dot(r, z0) for r in q]) == 0
slacks = [dot(r, z0) for r in rows]
assert all(x.sign() >= 0 for x in slacks)
active = [i for i, x in enumerate(slacks) if x == 0]
assert len(active) == 20
active_rank = rank([rows[i] for i in active])
assert active_rank == 6
# The preliminary first-order test has strict feasible weights, so cannot
# itself remove any active row from the eventual product search.
gradient = [2*dot(r, z0) for r in q]
weights = [-F(3, 0)/4+7*s/20, F(49)/100-s/5,
           F(49)/100-s/5, F(1)/100, F(1)/100]
assert all(x.sign() > 0 for x in weights)
assert all(sum((weights[k//4]*rows[i][j] for k, i in enumerate(active)), F())
           == gradient[j] for j in range(7))

# Hardcoded rational separator, obtained once and now checked independently
# of its numerical discovery. Coordinates: lambda=z[6], y=z[:6]-lambda*z0[:6].
v = [Fraction(n, 1000) for n in [-133, 195, -25, -419, 177, -63]]
nums = [[1000, 418, -151, 529, 231, 269],
        [418, 561, 118, -138, 100, 58],
        [-151, 118, 747, 199, -516, -220],
        [529, -138, 199, 965, -224, 83],
        [231, 100, -516, -224, 738, 56],
        [269, 58, -220, 83, 56, 219]]
D = [[Fraction(n, 1000) for n in r] for r in nums]
d_pivots = positive_ldl_pivots(D)
c = 2048
moment = [[Fraction(c)]+v]+[[v[i]]+D[i] for i in range(6)]
full_pivots = positive_ldl_pivots(moment)
# For a row ell, its transformed coefficients are (slack, row[:6]).
Dv = [[dot(r[:6], D[k]) for k in range(6)] for r in rows]
rv = [dot(r[:6], v) for r in rows]
pairs = sorted({tuple(sorted((i, j))) for i in active for j in range(612)})
assert len(pairs) == 12050
minimum = None
minimum_pair = None
for i, j in pairs:
    val = dot(rows[i][:6], Dv[j])+slacks[i]*rv[j]+slacks[j]*rv[i]
    assert val.sign() > 0
    if minimum is None or (val-minimum).sign() < 0:
        minimum, minimum_pair = val, [i, j]
assert minimum == F('4171/400', '-46489/10000')
# All other pairs have two positive slacks. Bound each normalized coefficient
# by A, so their moments / (slack_i*slack_j) are >= c-2||v||_1 A-||D||_1 A^2.
A = F()
for row, slack in zip(rows, slacks):
    if slack == 0:
        continue
    assert slack.sign() > 0
    for entry in row[:6]:
        val = entry/slack
        if val.sign() < 0:
            val = -val
        if (val-A).sign() > 0:
            A = val
V = sum(map(abs, v))
Dnorm = sum(abs(x) for r in D for x in r)
bound = 2*V*A+Dnorm*A*A
assert (c-bound).sign() > 0
# z=lambda*z0+E*y; E comprises the first six standard ambient basis vectors.
ambient = [[c*z0[i]*z0[j]
            +z0[i]*(v[j] if j < 6 else 0)
            +z0[j]*(v[i] if i < 6 else 0)
            +(D[i][j] if i < 6 and j < 6 else 0)
            for j in range(7)] for i in range(7)]
target = sum((q[i][j]*ambient[i][j] for i in range(7) for j in range(7)), F())
reduced_target = dot(gradient[:6], v)+sum((q[i][j]*D[i][j] for i in range(6) for j in range(6)), F())
assert target == reduced_target == F('3719/4000', '-8857/20000')
assert target.sign() < 0
print(json.dumps({
    'status': 'PASS exact fixed-sector PSD-plus-row-products obstruction',
    'source_sha256': source_hash,
    'field': 'Q(sqrt(5)); rational separator in transformed coordinates',
    'row_count': len(rows),
    'active_rows': active,
    'active_rank': active_rank,
    'strict_gradient_group_weights': [x.data() for x in weights],
    'coordinate_convention': 'lambda=z[6]; y[i]=z[i]-lambda*z0[i], i=0..5; z0[6]=1',
    'dual_cross': [str(x) for x in v],
    'dual_y_matrix': [[str(x) for x in r] for r in D],
    'dual_y_ldl_pivots': [str(x) for x in d_pivots],
    'allowed_product_count': len(pairs),
    'minimum_allowed_pair': minimum_pair,
    'minimum_allowed_value': minimum.data(),
    'inactive_normalized_coordinate_bound': A.data(),
    'inactive_product_dominating_bound': bound.data(),
    'lambda_square_moment': c,
    'full_moment_ldl_pivots': [str(x) for x in full_pivots],
    'ambient_moment_matrix': [[x.data() for x in r] for r in ambient],
    'target_pairing': target.data(),
    'all_unordered_row_products_strictly_positive': True,
    'ambient_moment_positive_definite': True,
}, indent=2))
