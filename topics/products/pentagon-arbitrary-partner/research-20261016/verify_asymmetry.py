#!/usr/bin/env python3
"""Bounded numerical controls for asymmetry.md; never an exact certificate.

Requires NumPy/SciPy; writes only --output if provided. Imports the existing
normal-triangle fitting capacity evaluator without running its producer main.
"""
import argparse
import importlib.util
import json
from pathlib import Path
import platform
import sys
from time import perf_counter

import numpy as np
import scipy
from scipy.optimize import linprog
from scipy.spatial import ConvexHull


def minkowski_asymmetry(points):
    """Minimize s with -K+(s+1)z subset sK, via y=(s+1)z."""
    hull = ConvexHull(points)
    vertices = np.asarray(points)[hull.vertices]
    A, b = hull.equations[:, :2], -hull.equations[:, 2]
    reflected_support = np.max(-A @ vertices.T, axis=1)
    C = np.column_stack([A, -b])
    lp = linprog([0., 0., 1.], A_ub=C, b_ub=-reflected_support,
                 bounds=[(None, None), (None, None), (1., None)], method='highs')
    if not lp.success:
        raise RuntimeError(lp.message)
    s = float(lp.x[2])
    center = lp.x[:2] / (1+s)
    support = b-A@center
    opposite_support = reflected_support+A@center
    violation = float(max(0., np.max(opposite_support-s*support)))
    return s, center, violation


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo-root', type=Path,
                        default=Path(__file__).resolve().parents[4])
    parser.add_argument('--output', type=Path)
    args = parser.parse_args()
    start = perf_counter()
    source = args.repo_root/'topics/products/pentagon-arbitrary-partner/evaluator.py'
    spec = importlib.util.spec_from_file_location('partner_evaluator', source)
    evaluator = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(evaluator)
    pi = np.pi
    kappa = (10+6*np.sqrt(5))/25
    hko = (3+np.sqrt(5))/5
    threshold = np.sqrt(5)-1
    rows = []
    fixtures = [
        ('hko_regular_pentagon', evaluator.regular(5, 3*pi/10), threshold, hko),
        ('aligned_regular_pentagon', evaluator.regular(5, pi/5), threshold, None),
        ('symmetric_extremal_decagon', evaluator.regular(10)/(2*(1+np.cos(pi/5))), 1., kappa),
        ('regular_triangle', evaluator.regular(3, .17), 2., None),
        ('regular_heptagon', evaluator.regular(7, .17), 1/np.cos(pi/7), None),
        ('rectangle', np.array([[-1.,-.7], [1.,-.7], [1.,.7], [-1.,.7]]), 1., None),
        ('irregular_pentagon', np.array([[-1.,-.6],[.6,-.9],[1.2,.1],[.5,1.1],[-.7,.8]]), None, None),
    ]
    # A finite polygon on the analytic support curve h=1+.08 cos(3theta).
    # h+h''=1-.64cos(3theta)>0, so this is a convex smooth curve sampled at
    # 180 prescribed angles; only its polygonal convex hull is evaluated.
    theta = 2*pi*np.arange(180)/180
    normals = np.column_stack([np.cos(theta), np.sin(theta)])
    tangent = np.column_stack([-np.sin(theta), np.cos(theta)])
    support = 1+.08*np.cos(3*theta)
    derivative = -.24*np.sin(3*theta)
    fixtures.append(('sampled_smooth_support_curve', support[:, None]*normals+
                     derivative[:, None]*tangent, None, None))
    # Bounded seeded heterogeneous controls, retaining all raw input points.
    rng = np.random.default_rng(20261016)
    for i in range(12):
        angle = np.sort(rng.uniform(0., 2*pi, 9+i%4))
        radius = rng.uniform(.8, 1.3, len(angle))
        points = radius[:, None]*np.column_stack([np.cos(angle), np.sin(angle)])
        fixtures.append((f'seeded_hull_{i:02}', points, None, None))
    for label, points, expected_s, expected_sys in fixtures:
        result = evaluator.evaluate(points)
        s, center, violation = minkowski_asymmetry(points)
        vertices = np.asarray(result['vertices'])
        differences = (vertices[:, None, :]-vertices[None, :, :]).reshape(-1, 2)
        difference_area = float(ConvexHull(differences).volume)
        area_ratio = difference_area/(4*result['area'])
        upper = kappa*(1+s)/2
        row = {
            'id': label, 'input_points': np.asarray(points).tolist(),
            'hull_vertex_count': len(vertices), 's': s,
            'minkowski_center': center.tolist(),
            'centered_inclusion_violation': violation,
            'area_difference_body_over_4area': area_ratio,
            'mixed_area_bound_slack': (1+s)/2-area_ratio,
            'capacity': result['capacity'], 'sys': result['sys'],
            'asymmetry_upper_bound': upper, 'capacity_bound_slack': upper-result['sys'],
            'within_hko_asymmetry_class': bool(s <= threshold+1e-9),
            'max_capacity_lp_residual': max(max(f['primal_violation'],
                f['dual_stationarity_residual'], f['duality_gap']) for f in result['fits']),
        }
        if expected_s is not None:
            row['expected_s_error'] = abs(s-expected_s)
            assert row['expected_s_error'] < 1e-8, row
        if expected_sys is not None:
            row['expected_sys_error'] = abs(result['sys']-expected_sys)
            assert row['expected_sys_error'] < 1e-8, row
        assert violation < 1e-8 and row['mixed_area_bound_slack'] > -1e-8, row
        assert row['capacity_bound_slack'] > -1e-8, row
        if row['within_hko_asymmetry_class']:
            assert result['sys'] <= hko+1e-8, row
        rows.append(row)
    # Affine invariance and translation of the independent asymmetry LP.
    base = fixtures[6][1]
    M = np.array([[1.7, .4],[-.2, .8]])
    a = minkowski_asymmetry(base)[0]
    b = minkowski_asymmetry(base@M.T+np.array([2.3, -.8]))[0]
    assert abs(a-b) < 1e-8
    packet = {
        'status': 'floating_point_controls_not_a_proof_or_exact_capacity_certificate',
        'python': platform.python_version(), 'numpy': np.__version__,
        'scipy': scipy.__version__, 'random_seed': 20261016,
        'fixture_count': len(rows), 'kappa_symmetric': kappa,
        'hko_ratio': hko, 'hko_asymmetry': threshold,
        'necessary_s_for_sys_above_one': 2/kappa-1,
        'asymmetry_affine_translation_error': abs(a-b),
        'max_capacity_lp_residual': max(r['max_capacity_lp_residual'] for r in rows),
        'seconds': perf_counter()-start, 'records': rows,
    }
    encoded = json.dumps(packet, indent=2)+'\n'
    if args.output:
        args.output.write_text(encoded)
    print(json.dumps({k:v for k,v in packet.items() if k != 'records'}, indent=2))


if __name__ == '__main__':
    main()
