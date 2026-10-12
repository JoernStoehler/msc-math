"""Read-only exact check of the ten fitting-row bases in Q(sqrt5)."""
from fractions import Fraction as F
from itertools import combinations
import json


def add(u, v):
    return u[0] + v[0], u[1] + v[1]


def neg(u):
    return -u[0], -u[1]


def mul(u, v):
    return u[0] * v[0] + 5 * u[1] * v[1], u[0] * v[1] + u[1] * v[0]


z = F(0), F(0)
alpha = F(1, 2), F(-1, 10)
beta = F(0), F(1, 5)
base = [alpha, alpha, z, beta, z]
rows = [[base[(i - shift) % 5] for i in range(3)] for shift in range(5)]
results = []
for ids in combinations(range(5), 3):
    m = [rows[i] for i in ids]
    value = z
    for j in range(3):
        minor = add(mul(m[1][(j + 1) % 3], m[2][(j + 2) % 3]),
                    neg(mul(m[1][(j + 2) % 3], m[2][(j + 1) % 3])))
        value = add(value, mul(m[0][j], minor))
    assert value != z
    results.append({'basis': ids, 'det_a_plus_b_sqrt5': list(map(str, value))})
assert len(results) == 10
print(json.dumps({'status': 'PASS', 'nonsingular_fit_triples': 10,
                  'gauge': 'h3=h4=0', 'determinants': results}))
