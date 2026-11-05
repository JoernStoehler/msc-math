#!/usr/bin/env python3
"""Exact coefficient proof of two ten-direction equality rays. Stdlib only.

The symplectic form has matrix A and Pf(A)=9. Psi is the sum of absolute
principal four-Pfaffians, so Euclidean volume is 16*Psi/9 in these coordinates.
All generator weights are nonnegative for s>=0; absolute values therefore
factor before the parameter polynomial is expanded.
"""
from fractions import Fraction as F
from itertools import combinations, permutations
from math import prod

A = [[0, 1, 2, 2], [-1, 0, 2, -2], [-2, -2, 0, 1], [-2, 2, -1, 0]]
E = [[F(i == j) for i in range(4)] for j in range(4)]
BASE = E + [[F(-A[i][j], 3) for i in range(4)] for j in range(4)]


def omega(u, v):
    return sum(u[i] * A[i][j] * v[j] for i in range(4) for j in range(4))


def mul(p, q):
    out = [F(0)] * (len(p) + len(q) - 1)
    for i, a in enumerate(p):
        for j, b in enumerate(q):
            out[i + j] += a * b
    return out


def add_scaled(out, p, scale):
    for i, a in enumerate(p):
        out[i] += scale * a


def det(m):
    n = len(m)
    return sum((-1) ** sum(p[i] > p[j] for i in range(n) for j in range(i + 1, n))
               * prod(m[i][p[i]] for i in range(n)) for p in permutations(range(n)))


def parallel(u, v):
    return all(u[i] * v[j] == u[j] * v[i] for i, j in combinations(range(4), 2))


def verify(name, u, v, expected_phi, expected_psi):
    directions = BASE + [list(map(F, u)), list(map(F, v))]
    # Z_s=Z_0+sP. Two old directions increase, and two new ones appear.
    weights = [[F(1), F(6)] if i in (0, 5) else [F(1)] for i in range(8)]
    weights += [[F(0), F(1)], [F(0), F(3)]]
    pairings = [[omega(x, y) for y in directions] for x in directions]
    phi, psi = [F(0)] * 3, [F(0)] * 5
    for i, j in combinations(range(10), 2):
        add_scaled(phi, mul(weights[i], weights[j]), abs(pairings[i][j]))
    for i, j, k, l in combinations(range(10), 4):
        pf = pairings[i][j] * pairings[k][l] - pairings[i][k] * pairings[j][l] \
             + pairings[i][l] * pairings[j][k]
        p = mul(mul(weights[i], weights[j]), mul(weights[k], weights[l]))
        add_scaled(psi, p, abs(pf))
    gap = mul(phi, phi)
    add_scaled(gap, psi, -4)
    assert phi == list(map(F, expected_phi)), (name, phi)
    assert psi == list(map(F, expected_psi)), (name, psi)
    assert gap == [0] * 5, (name, gap)
    assert not any(parallel(directions[i], directions[j]) for i, j in combinations(range(10), 2))
    print(f'{name}: Phi coefficients {phi}; Psi coefficients {psi}; exact gap zero')
    P = [[6*x for x in BASE[0]], [6*x for x in BASE[5]], directions[8], [3*x for x in directions[9]]]
    print(f'{name}: summand pairings {[omega(P[i], P[j]) for i,j in combinations(range(4),2)]}')


def main():
    assert det(A) == 81
    assert A[0][1] * A[2][3] - A[0][2] * A[1][3] + A[0][3] * A[1][2] == 9
    # Existing eight directions still prevent a product of two planes:
    # every triple is linearly independent.
    for triple in combinations(BASE, 3):
        assert any(det([[v[i] for v in triple] for i in rows]) != 0
                   for rows in combinations(range(4), 3))
    verify('balanced parallelotope', (0, 4, -1, -1), (0, 0, 1, 1),
           [32, 168, 144], [256, 2688, 9360, 12096, 5184])
    verify('Lagrangian planar summand', (4, 0, 1, -1), (0, 0, 1, -1),
           [32, 168, 0], [256, 2688, 7056, 0, 0])
    print('PASS: both identities hold for all s>=0; for s>0 there are ten distinct directions.')


if __name__ == '__main__':
    main()
