#!/usr/bin/env python3
"""Complete floating-point HK response for triangle x triangle.

Each triangle has a unique normalized positive closure vector. Full product
closure is therefore one scalar t. Enumerating all 120 cyclic six-facet words
and maximizing their quadratic on [0,1] covers every feasible HK datum, including
zero-mass endpoints. This is a numerical calculation, not an interval certificate.
"""
import itertools
import json
import math
from pathlib import Path
import numpy as np

J2 = np.array([[0., 1.], [-1., 0.]])

def polygon_data(vertices):
    v = np.asarray(vertices, dtype=float)
    v = v - v.mean(axis=0)
    e = np.roll(v, -1, axis=0) - v
    n = e @ J2
    h = (n * v).sum(axis=1)
    dual = n / h[:, None]
    mu = np.linalg.solve(np.vstack([dual.T, np.ones(3)]), [0., 0., 1.])
    area = abs(np.linalg.det(np.array([v[1]-v[0], v[2]-v[0]]))) / 2
    assert min(mu) > 0
    assert np.max(abs(mu @ dual)) < 1e-12
    return v, dual, mu, area

def branch_coefficients(A, B):
    av, ad, am, a = polygon_data(A)
    bv, bd, bm, b = polygon_data(B)
    d = np.zeros((6, 4)); d[:3, :2] = ad; d[3:, 2:] = bd
    mu = np.concatenate([am, bm])
    j0 = np.zeros((4, 4)); j0[:2, :2] = J2; j0[2:, 2:] = -J2
    jl = np.block([[np.zeros((2, 2)), np.eye(2)], [-np.eye(2), np.zeros((2, 2))]])
    w0 = d @ j0 @ d.T
    wl = d @ jl @ d.T
    branches = []
    for tail in itertools.permutations(range(1, 6)):
        word = (0,) + tail
        x = y = z = 0.
        for i, j in itertools.combinations(word, 2):
            if i < 3 and j < 3:
                x += 2 * mu[i] * mu[j] * w0[i, j]
            elif i >= 3 and j >= 3:
                y += 2 * mu[i] * mu[j] * w0[i, j]
            else:
                z += 2 * mu[i] * mu[j] * wl[i, j]
        branches.append(dict(word=list(word), a_inverse=x, b_inverse=y, cross=z))
    assert max(abs(abs(r['a_inverse']) - 1/a) for r in branches) < 1e-10
    assert max(abs(abs(r['b_inverse']) - 1/b) for r in branches) < 1e-10
    return av, bv, a, b, branches

def quadratic_max(x, y, z):
    # x*t*t + y*(1-t)**2 + z*t*(1-t)
    candidates = [(x, 1.), (y, 0.)]
    q = x + y - z
    if q < 0:
        t = (2*y-z) / (2*q)
        if 0 < t < 1:
            candidates.append((x*t*t + y*(1-t)**2 + z*t*(1-t), t))
    return max(candidates)

def evaluate(branches, theta):
    c, s = math.cos(theta), math.sin(theta)
    result = []
    for k, r in enumerate(branches):
        value, t = quadratic_max(c*r['a_inverse'], c*r['b_inverse'], s*r['cross'])
        result.append((value, t, k))
    value, t, k = max(result)
    return dict(theta=theta, capacity=1/value, mass_a=t, branch=k)

def run():
    triangle = np.array([[1., 0.], [-.5, math.sqrt(3)/2], [-.5, -math.sqrt(3)/2]])
    cases = []
    # Rotation, anisotropy, shear and unequal area; these are controls for the
    # general derivation, not a representative statistical population.
    transforms = [np.eye(2), np.array([[2.,0.],[0.,.5]]),
                  np.array([[1.,1.3],[0.,1.]]), np.array([[1.7,.4],[.2,1.]])]
    for ti, transform in enumerate(transforms):
        for angle in [0., .23, .61]:
            R = np.array([[math.cos(angle), -math.sin(angle)],
                          [math.sin(angle), math.cos(angle)]])
            av, bv, a, b, branches = branch_coefficients(triangle, triangle @ (R @ transform).T)
            ell = evaluate(branches, math.pi/2)['capacity']
            threshold = math.atan(ell/(2*min(a,b)))
            angles = sorted(set([i*math.pi/240 for i in range(121)] + [threshold, threshold*.5]))
            curve = [evaluate(branches, t) for t in angles]
            bound_error = 0.
            regime_error = 0.
            for row in curve:
                t = row['theta']; c, s = math.cos(t), math.sin(t)
                upper_inverse, _ = quadratic_max(c/a, c/b, 4*s/ell)
                bound_error = max(bound_error, 1/row['capacity']-upper_inverse)
                if t <= threshold:
                    regime_error = max(regime_error, abs(row['capacity']*c/min(a,b)-1))
            peak = max(curve, key=lambda r:r['capacity'])
            first_mixed = next((r for r in curve if 1e-8 < r['mass_a'] < 1-1e-8), None)
            cases.append(dict(body_id=f'triangle_product_transform_{ti}_phase_{angle}',
                factor_a_vertices=av.tolist(), factor_b_vertices=bv.tolist(), area_a=a, area_b=b,
                lagrangian_capacity=ell, guaranteed_threshold=threshold,
                numerical_status='complete-HK-family-binary64-quadratic-max-not-certified',
                max_bound_violation=bound_error, max_regime_relative_error=regime_error,
                sampled_peak=peak, first_sampled_mixed=first_mixed, branches=branches, curve=curve))
    payload = dict(measurement_version='product-position-triangle-hk-v1',
        source_population='12 deterministic affine triangle products; controlled validation only',
        form='J_theta=[[cos(theta)*J2,sin(theta)*I],[-sin(theta)*I,-cos(theta)*J2]]',
        units='capacity and planar area length^2; angle radians; mass dimensionless', cases=cases)
    out = Path(__file__).with_name('triangle-response.json')
    out.write_text(json.dumps(payload, indent=2) + '\n')
    print(json.dumps([dict(id=r['body_id'], ell=r['lagrangian_capacity'],
        threshold=r['guaranteed_threshold'], first_mixed=r['first_sampled_mixed']['theta'],
        peak=r['sampled_peak']['capacity'], endpoint0=min(r['area_a'],r['area_b']),
        bound_error=r['max_bound_violation'], regime_error=r['max_regime_relative_error']) for r in cases],indent=2))

if __name__ == '__main__':
    run()
