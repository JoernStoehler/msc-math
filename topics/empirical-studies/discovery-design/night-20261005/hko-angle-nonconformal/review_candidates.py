"""Read-only independent HKO-angle candidate audit; run with python -B review_candidates.py.

Only the standard library is used. Arithmetic is rational pairs a+b sqrt(5),
linear systems use direct determinants/Cramer's rule, and quadratic minima
use the discriminant stationary-value formula. Producer code is not imported.
The denominator-envelope packet is an input, not independently derived here.
"""
from fractions import Fraction as Q
from itertools import combinations
from pathlib import Path
import json
import time


class E:
    def __init__(self, a=0, b=0):
        self.a, self.b = Q(a), Q(b)

    def __add__(u, v):
        v = v if isinstance(v, E) else E(v)
        return E(u.a + v.a, u.b + v.b)

    __radd__ = __add__

    def __neg__(u):
        return E(-u.a, -u.b)

    def __sub__(u, v):
        return u + -v

    def __mul__(u, v):
        v = v if isinstance(v, E) else E(v)
        return E(u.a * v.a + 5 * u.b * v.b, u.a * v.b + u.b * v.a)

    __rmul__ = __mul__

    def __truediv__(u, v):
        v = v if isinstance(v, E) else E(v)
        den = v.a * v.a - 5 * v.b * v.b
        return E((u.a * v.a - 5 * u.b * v.b) / den,
                 (u.b * v.a - u.a * v.b) / den)

    def sign(u):
        a, b = u.a, u.b
        if not b:
            return (a > 0) - (a < 0)
        if not a:
            return (b > 0) - (b < 0)
        if (a > 0) == (b > 0):
            return 1 if a > 0 else -1
        t = a * a - 5 * b * b
        return ((t > 0) - (t < 0)) * (1 if a > 0 else -1)

    def key(u):
        return u.a, u.b

    def __eq__(u, v):
        return isinstance(v, E) and u.key() == v.key()


z, one, r = E(), E(1), E(0, 1)
a, b = (r - one) / E(4), (r + one) / E(4)
w = one + b
l0 = [one / (2 * w), one / (2 * w), z, b / w, z]
lams = [[l0[(i - l) % 5] for i in range(5)] for l in range(5)]
rows = [v[:3] for v in lams]
for i in range(5):
    v = [z] * 5
    v[i], v[(i - 1) % 5], v[(i + 1) % 5] = -2 * a, one, one
    rows.append(v[:3])


def det(m):
    return sum((m[0][j] * (m[1][(j + 1) % 3] * m[2][(j + 2) % 3]
                - m[1][(j + 2) % 3] * m[2][(j + 1) % 3])
                for j in range(3)), z)


def solve(m, rhs):
    den = det(m)
    return [det([[rhs[i] if j == col else m[i][j] for j in range(3)]
                 for i in range(3)]) / den for col in range(3)]


def decd(q):
    return E(*q)


def decc(q):
    return E() if not q else E(q[0]) if len(q) == 1 else E(q[1], q[0])


def hkey(h):
    return tuple(u.key() for coord in h for u in coord)


def ev(p, x):
    return p[0] + p[1] * x + p[2] * x * x


def minimum_checks(q, lo, hi):
    checks = [(lo, ev(q, lo))]
    if hi is not None:
        checks.append((hi, ev(q, hi)))
    else:
        assert q[2].sign() >= 0
        assert q[2] != z or q[1].sign() >= 0
    if q[2].sign() > 0:
        at = -q[1] / (2 * q[2])
        if (at - lo).sign() >= 0 and (hi is None or (hi - at).sign() >= 0):
            checks.append((at, (4 * q[0] * q[2] - q[1] * q[1]) / (4 * q[2])))
    return checks


def main():
    started = time.monotonic()
    root = Path(__file__).resolve().parent
    packet = json.loads((root / 'denominators.json').read_text())
    certificate = json.loads((root / 'body-candidates.json').read_text())
    breaks = list(map(decd, packet['common_refinement_breakpoints']))
    bases, singular = [], []
    for ids in combinations(range(10), 3):
        mat = [rows[i] for i in ids]
        if det(mat) == z:
            singular.append(list(ids))
        else:
            bases.append((ids, mat))
    assert len(bases) == 100 and singular == certificate['singular']
    counts, checks_sharp = [], 0
    records_checked, singleton, tails, zero_records = 0, 0, 0, 0
    zero_positions, zero_bodies = set(), set()
    d = E(2) - E(2) / r
    sharp_target, xstar = d / (E(3) + r), one
    for ci, left in enumerate(breaks):
        right = breaks[ci + 1] if ci + 1 < len(breaks) else None
        probe = (left + right) / E(2) if right is not None else left + one
        ds = []
        for item in packet['denominators']:
            cell = next(c for c in item['cells']
                        if (probe - decd(c['left'])).sign() >= 0
                        and (c['right'] is None or (decd(c['right']) - probe).sign() > 0))
            ds.append([decd(cell['intercept']), decd(cell['slope'])])
        found, feasible = {}, 0
        for ids, mat in bases:
            rhs = [ds[i] if i < 5 else [z, z] for i in ids]
            hs = [solve(mat, [q[p] for q in rhs]) for p in range(2)]
            h = [[hs[0][j], hs[1][j]] for j in range(3)] + [[z, z], [z, z]]
            lo, hi, okay = left, right, True
            for i, row in enumerate(rows):
                f = [sum((row[j] * h[j][p] for j in range(3)), z)
                     - (ds[i][p] if i < 5 else z) for p in range(2)]
                if f[1] == z:
                    if f[0].sign() < 0:
                        okay = False
                        break
                else:
                    bound = -f[0] / f[1]
                    if f[1].sign() > 0:
                        if (bound - lo).sign() > 0:
                            lo = bound
                    elif hi is None or (hi - bound).sign() > 0:
                        hi = bound
                if hi is not None and (hi - lo).sign() < 0:
                    okay = False
                    break
            if not okay or hi == z:
                continue
            feasible += 1
            key = hkey(h)
            if key in found:
                continue
            p = [z, z, z]
            for i in range(5):
                for j in range(2):
                    for k in range(2):
                        p[j + k] += h[i][j] * h[(i + 1) % 5][k] - a * h[i][j] * h[i][k]
            sharp_poly = [p[0], p[1] - sharp_target, p[2]]
            assert sharp_poly != [z, z, z]
            sharp_tests = minimum_checks(sharp_poly, lo, hi)
            assert all(v.sign() >= 0 for _, v in sharp_tests)
            checks_sharp += len(sharp_tests)
            singleton += hi == lo
            tails += hi is None
            has_zero = False
            for x, v in sharp_tests:
                if v == z:
                    has_zero = True
                    assert x == xstar
                    zero_positions.add(x.key())
                    zero_bodies.add(tuple((u + x * v).key() for u, v in h))
            zero_records += has_zero
            found[key] = lo, hi, p
        certs = [c for c in certificate['candidates'] if c['cell'] == ci]
        assert len(found) == len(certs)
        for c in certs:
            key = hkey([[decc(v) for v in coord] for coord in c['supports_affine']])
            assert key in found
            lo, hi, p = found[key]
            assert lo == decc(c['interval'][0])
            assert (hi is None and c['interval'][1] is None
                    or hi is not None and c['interval'][1] is not None and hi == decc(c['interval'][1]))
            assert p == [decc(v) for v in c['area_numerator_quadratic']]
            records_checked += 1
        counts.append((feasible, len(found)))
    assert counts == [(c['feasible_bases'], c['distinct_candidates']) for c in certificate['cells']]
    expected = [-one + E(Q(3, 5)) * r, one - r / E(5), -one + E(Q(3, 5)) * r, z, z]
    assert zero_bodies == {tuple(u.key() for u in expected)}
    assert zero_positions == {xstar.key()}
    # Independent adjacent-line intersections and shoelace at equality.
    normals = [[decd(v) for v in row] for row in packet['normals']]
    cross = lambda u, v: u[0] * v[1] - u[1] * v[0]
    vertices = []
    for i in range(5):
        n, m = normals[i], normals[(i + 1) % 5]
        u, v = expected[i], expected[(i + 1) % 5]
        denominator = cross(n, m)
        vertices.append([(u * m[1] - v * n[1]) / denominator,
                         (n[0] * v - m[0] * u) / denominator])
    computational_area = sum((cross(vertices[i], vertices[(i + 1) % 5])
                              for i in range(5)), z) / E(2)
    numerator = sum((expected[i] * expected[(i + 1) % 5]
                     - a * expected[i] * expected[i] for i in range(5)), z)
    assert computational_area == numerator / (d * (E(5) - r) / E(4))
    assert numerator == sharp_target
    optimizer = json.loads((root / 'optimizer-certificate.json').read_text())
    assert [decc(v) for v in optimizer['canonical_quotient_supports']] == expected
    triangles = [[[decd(v) for v in point] for point in triangle]
                 for triangle in packet['triangles']]
    assert len(triangles) == 10 and len(optimizer['placements']) == 10
    dot = lambda u, v: sum((s * t for s, t in zip(u, v)), z)
    active_sum = sum((lams[0][i] * expected[i] for i in range(5)), z)
    dual_found = False
    for j, triangle in enumerate(triangles):
        placement = optimizer['placements'][j]
        assert placement['template'] == j
        translation = [decc(v) for v in placement['translation']]
        supports = []
        for i, normal in enumerate(normals):
            values = [dot(normal, point) for point in triangle]
            support = values[0]
            for value in values[1:]:
                if (value - support).sign() > 0:
                    support = value
            supports.append(support)
            slack = expected[i] - support - dot(normal, translation)
            assert slack.sign() >= 0 and slack == decc(placement['slacks'][i])
        dual_found |= sum((lams[0][i] * supports[i] for i in range(5)), z) == active_sum
    assert dual_found
    print(json.dumps({'status': 'PASS', 'nonsingular': len(bases), 'singular': len(singular),
                      'cell_counts': counts, 'matched_records': records_checked,
                      'hko_nonnegative_checks': checks_sharp,
                      'singleton_records': singleton, 'unbounded_records': tails,
                      'zero_records': zero_records, 'zero_support_classes': len(zero_bodies),
                      'equality_shoelace': 'PASS',
                      'equality_ten_primal_and_active_dual': 'PASS',
                      'elapsed_seconds': time.monotonic() - started}))


if __name__ == '__main__':
    main()
