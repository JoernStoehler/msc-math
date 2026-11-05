"""Floating triage of three radial HKO hinge profiles against all 190 charts.

This is exploratory only. It reads the retained exact chart catalogue and exact
primitive weights, reconstructs each chart's affine KKT positions using the
certificate field code, then evaluates piecewise-quadratic responses in float.
"""
import itertools
import json
import sys
from fractions import Fraction
from pathlib import Path

import numpy as np

CERT = Path(__file__).resolve().parents[4] / "topics/capacity-and-qp/convex-systolic-optimization/certificates/hko-ascent"
sys.path.insert(0, str(CERT))
import exact_core as E

primitive = json.loads((CERT / "expected/primitives.json").read_text())
catalogue = json.loads((CERT / "expected/catalogue.json").read_text())


def decode(coefficients):
    return E.K([Fraction(c) for c in coefficients])


closure_vertices = []
for key in ("triangles_q", "triangles_p"):
    vertices = []
    for labels, weights in primitive[key]:
        beta = [E.ZERO] * 10
        for label, weight in zip(labels, weights):
            beta[label] = decode(weight) / 2
        vertices.append(beta)
    closure_vertices.append(vertices)

T = 1 / (2 * E.Q0)
normal_length = E.SQRT5 - 1


def J(a):
    return [-a[2], -a[3], a[0], a[1]]


def gradient(word, beta):
    return [
        sum((beta[j] * (E.omega(E.A[word[j]], E.A[word[i]]) if j < i else E.omega(E.A[word[i]], E.A[word[j]]))
             for j in range(len(word)) if j != i), E.ZERO)
        for i in range(len(word))
    ]


def radial_positions(word, beta, pivot, inverse):
    multiplier = E.mv(inverse, [gradient(word, beta)[i] for i in pivot])
    x = E.scale(-T, multiplier[:4])
    positions = []
    for label, weight in zip(word, beta):
        a = E.A[label]
        if label < 5:
            direction = [-a[1] / normal_length, a[0] / normal_length]
            ell = E.dot(direction, x[2:])
        else:
            direction = [a[3] / normal_length, -a[2] / normal_length]
            ell = E.dot(direction, x[:2])
        positions.append(ell)
        x = E.add(x, E.scale(2 * T * weight, J(a)))
    return positions


def nominal(x):
    return E.nominal(x)


charts = []
for chart in catalogue["charts"]:
    word = chart["sigma"]
    qvertices = [i for i in range(5) if chart["q_vertices_mask"] & (1 << i)]
    pvertices = [i for i in range(5) if chart["p_vertices_mask"] & (1 << i)]
    endpoints = [
        [closure_vertices[0][qi][i] + closure_vertices[1][pi][i] for i in word]
        for qi, pi in itertools.product(qvertices, pvertices)
    ]
    beta0, beta1 = endpoints[0], endpoints[-1]
    betad = E.add(beta1, E.scale(-1, beta0))
    rows = [E.A[label] + [E.ONE] for label in word]
    for pivot in itertools.combinations(range(len(word)), 5):
        try:
            inverse = E.inv([rows[i] for i in pivot])
            break
        except ArithmeticError:
            pass
    else:
        raise ArithmeticError("no rank-five chart pivot")
    pos0 = radial_positions(word, beta0, pivot, inverse)
    pos1 = radial_positions(word, beta1, pivot, inverse)
    posd = E.add(pos1, E.scale(-1, pos0))
    charts.append((
        np.array([nominal(x) for x in beta0]),
        np.array([nominal(x) for x in betad]),
        np.array([nominal(x) for x in pos0]),
        np.array([nominal(x) for x in posd]),
        word,
    ))


def chart_response(thresholds, labels):
    """Max over 190 charts, with every z-interval quadratic optimized exactly in shape."""
    best = 0.0
    for beta0, betad, pos0, posd, word in charts:
        active_labels = [j for j, label in enumerate(word) if label in labels]
        if not active_labels:
            continue
        breaks = [0.0, 1.0]
        for j in active_labels:
            threshold = thresholds[labels.index(word[j])]
            if abs(posd[j]) > 1e-15:
                z = (threshold - pos0[j]) / posd[j]
                if 0 < z < 1:
                    breaks.append(z)
        breaks = sorted(set(breaks))
        candidates = list(breaks)
        for left, right in zip(breaks[:-1], breaks[1:]):
            middle = (left + right) / 2
            active = [j for j in active_labels
                      if pos0[j] + middle * posd[j] > thresholds[labels.index(word[j])]]
            aa = sum(betad[j] * posd[j] for j in active)
            bb = sum(betad[j] * (pos0[j] - thresholds[labels.index(word[j])])
                     + beta0[j] * posd[j] for j in active)
            if aa < 0:
                z = -bb / (2 * aa)
                if left < z < right:
                    candidates.append(z)
        for z in candidates:
            response = sum((beta0[j] + z * betad[j]) * max(
                0.0, pos0[j] + z * posd[j] - thresholds[labels.index(word[j])])
                for j in active_labels)
            best = max(best, response)
    return best


# The appendix's facet coordinate is ell=m+h*y with its stated piecewise density.
sqrt5 = np.sqrt(5.0)
k = sqrt5 - 2
m = (1 - np.cos(np.pi / 5)) / 2
h = (1 + np.cos(np.pi / 5)) / 2
b = sqrt5 / 10
C = (5 + 2 * sqrt5) / 10
grid_y = np.linspace(-1, 1, 20001)
density = np.where(grid_y <= k, b * (grid_y + 3), C * (1 - grid_y))


def facet_volume(threshold):
    hinge = np.maximum(0.0, m + h * grid_y - threshold)
    return np.trapezoid(hinge * density, grid_y) / 10


results = []
for threshold in (0.5, 0.6, 0.7):
    all_triples = []
    for labels in itertools.combinations(range(10), 3):
        thresholds = (threshold,) * 3
        volume = 3 * facet_volume(threshold)
        response = chart_response(thresholds, labels)
        all_triples.append((volume - response, labels, volume, response))
    results.append({
        "threshold": threshold,
        "best_gap_over_120_label_triples": max(all_triples)[0],
        "best_triple": max(all_triples)[1],
        "best_volume": max(all_triples)[2],
        "best_response": max(all_triples)[3],
        "worst_gap_over_120_label_triples": min(all_triples)[0],
    })
print(json.dumps({"chart_count": len(charts), "results": results}, indent=2))
