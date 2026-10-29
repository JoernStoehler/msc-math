"""Exact falsifier of an all-direction strengthening, not of the main conjecture."""
from itertools import combinations
import json

A = [[0, 1, 2, 2], [-1, 0, 2, -2], [-2, -2, 0, 1], [-2, 2, -1, 0]]
assert [[sum(A[i][k]*A[k][j] for k in range(4)) for j in range(4)]
        for i in range(4)] == [[-9*int(i == j) for j in range(4)] for i in range(4)]
W = [A[i]+[3*int(i == j) for j in range(4)] for i in range(4)]
W += [[-3*int(i == j) for j in range(4)]+A[i] for i in range(4)]

def pf4(ids):
    i, j, k, l = ids
    return W[i][j]*W[k][l]-W[i][k]*W[j][l]+W[i][l]*W[j][k]

def direction(c):
    y = [sum(W[i][j]*c[j] for j in range(4)) for i in range(8)]
    a = sum(map(abs, y))
    b = sum(abs(W[i][j]*y[k]-W[i][k]*y[j]+W[j][k]*y[i])
            for i, j, k in combinations(range(8), 3))
    return y, a, b

Phi = sum(abs(W[i][j]) for i, j in combinations(range(8), 2))
Psi = sum(abs(pf4(ids)) for ids in combinations(range(8), 4))
y, a, b = direction([1, 1, 1, 1])
defect = b*b-Phi*a*b+Psi*a*a
assert (Phi, Psi, a, b, defect) == (32, 256, 22, 324, 784)
assert direction([1, 0, 0, 0])[1:] == (8, 128)
assert 2*Phi*a-4*b == 112 and a*a == 484
print(json.dumps({"Phi": Phi, "Psi": Psi, "y": y, "a": a, "b": b,
                  "all_direction_defect": defect,
                  "segment_target_deficit_coefficients": [0, 112, 484]}, indent=2))
