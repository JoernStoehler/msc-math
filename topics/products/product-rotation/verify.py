#!/usr/bin/env python3
"""Exact triangle endpoint and rotation generator checks; no capacity solver."""
from fractions import Fraction as F
from itertools import permutations

u = ((F(0), F(-1)), (F(1), F(1)), (F(-1), F(0)))
v = ((F(0), F(-2)), (F(1, 2), F(2)), (F(-1, 2), F(0)))
a = ((F(-1), F(-1)), (F(2), F(-1)), (F(-1), F(2)))
b = tuple((2*x, y/2) for x, y in a)
def dot(x, y): return sum(i*j for i, j in zip(x, y))
for normals, vertices in ((u, a), (v, b)):
    assert all(sum(n[k] for n in normals) == 0 for k in range(2))
    assert all(max(dot(n, x) for x in vertices) == 1 for n in normals)
# With the u order fixed, insert each v after any prefix. Closure makes the
# signed cross contribution twice that prefix dot v. This upper bound is
# attained by independently choosing insertion slots (v-v pairings vanish).
for i, j, k in permutations(range(3)):
    largest = 2*sum(max(F(0), dot(u[i], w), -dot(u[k], w)) for w in v)
    assert largest == 5
# Independent complete cyclic-order check, including the main-text witness.
def cross(word):
    pos = {x: i for i, x in enumerate(word)}
    return sum((1 if pos[i] < pos[j+3] else -1)*dot(u[i], v[j])
               for i in range(3) for j in range(3))
assert cross((0, 3, 1, 4, 2, 5)) == 5
assert max(cross((0,)+tail) for tail in permutations(range(1, 6))) == 5
assert 2*F(1, 6)**2*5 == F(5, 18)
# Matrix identities imply the conjugation formula for all real theta.
J0 = ((0,1,0,0),(-1,0,0,0),(0,0,0,-1),(0,0,1,0))
JL = ((0,0,1,0),(0,0,0,1),(-1,0,0,0),(0,-1,0,0))
def mul(a,b): return tuple(tuple(sum(a[i][k]*b[k][j] for k in range(4)) for j in range(4)) for i in range(4))
K = mul(J0, JL)
assert all(K[i][j] == -K[j][i] for i in range(4) for j in range(4))
assert mul(K,K) == tuple(tuple(-int(i == j) for j in range(4)) for i in range(4))
assert mul(K,J0) == JL
assert mul(J0,K) == tuple(tuple(-x for x in row) for row in JL)
print('PASS: six-case endpoint bound; 120 cyclic orders; witness; rotation generator identities')
