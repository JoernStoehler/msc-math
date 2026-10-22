#!/usr/bin/env python3
"""Bounded numerical full-matrix T6 in T4 discriminator for odd regular factors."""
import hashlib
import itertools
import json
import math
import sys
from pathlib import Path

import numpy as np
import scipy
from scipy.spatial import ConvexHull

ROOT = Path('/workspaces/msc-math-local-cover-review-20261022')
SOURCE = ROOT / 'topics/products/pentagon-affine-products/research.tex'
OUT = Path('/tmp/msc-math/tensor-discriminator/20261022')


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def cross(a, b):
    return float(a[0] * b[1] - a[1] * b[0])


def factor(n):
    h = math.cos(math.pi / n)
    normals = np.array([(math.cos(2 * math.pi * i / n), math.sin(2 * math.pi * i / n)) for i in range(n)])
    duals = normals / h
    triples = []
    for indices in itertools.combinations(range(n), 3):
        a, b, c = (duals[i] for i in indices)
        raw = np.array([cross(b, c), cross(c, a), cross(a, b)])
        if not np.all(raw > 1e-12):
            continue
        weights = raw / np.sum(raw)
        steps = weights[:, None] * duals[list(indices)]
        assert np.max(np.abs(np.sum(steps, axis=0))) < 2e-14
        triples.append({'indices': list(indices), 'weights': weights.tolist(), 'steps': steps.tolist(), 'closure_residual': np.sum(steps, axis=0).tolist()})
    return {'n': n, 'h': h, 'normals': normals.tolist(), 'duals': duals.tolist(), 'triples': triples}


def matrix_record(t):
    return np.asarray(t).reshape(2, 2).tolist()


def case(a, b):
    n, m = a['n'], b['n']
    s_a, s_b = 1 / (1 + a['h']), 1 / (1 + b['h'])
    pa, pb = np.asarray(a['normals']), np.asarray(b['normals'])
    four = []
    for i, j in itertools.product(range(n), range(m)):
        t = (0.5 * s_a * s_b * np.outer(pa[i], pb[j])).reshape(4)
        four.append(t)
        four.append(-t)
    four = np.array(four)
    hull = ConvexHull(four)
    equations = hull.equations
    orders = []
    for ai, at in enumerate(a['triples']):
        astep = np.asarray(at['steps'])
        for bi, bt in enumerate(b['triples']):
            bstep = np.asarray(bt['steps'])
            # Fix the first A step; the two remaining A orders times six B orders
            # enumerate all 12 cyclic alternating words for this triple pair.
            for tail in itertools.permutations((1, 2)):
                ap = (0,) + tail
                for bp in itertools.permutations((0, 1, 2)):
                    tensor = 0.5 * (np.outer(astep[ap[0]], bstep[bp[0]]) - np.outer(astep[ap[2]], bstep[bp[1]])).reshape(4)
                    residuals = equations[:, :4] @ tensor + equations[:, 4]
                    face = int(np.argmax(residuals))
                    orders.append({'a_triple': ai, 'b_triple': bi, 'a_order': list(ap), 'b_order': list(bp), 'tensor': matrix_record(tensor), 'max_facet_residual': float(residuals[face]), 'max_facet': face})
    worst = max(orders, key=lambda z: z['max_facet_residual'])
    face = worst['max_facet']
    direction = equations[face, :4].reshape(2, 2)
    witness = {'order': worst, 'separating_direction': direction.tolist(), 'facet_offset': float(equations[face, 4]), 'facet_support': float(-equations[face, 4]), 'direction_det': float(np.linalg.det(direction)), 'tensor_det': float(np.linalg.det(np.asarray(worst['tensor']))), 'max_facet_residual': worst['max_facet_residual']}
    result = {'pair': [n, m], 'a_triple_count': len(a['triples']), 'b_triple_count': len(b['triples']), 'triple_pair_count': len(a['triples']) * len(b['triples']), 'orders_per_pair': 12, 'six_tensor_count': len(orders), 'four_point_count_with_signs': len(four), 'four_hull_vertices': len(hull.vertices), 'four_hull_facets': len(equations), 'four_generators': [matrix_record(t) for t in four], 'six_tensors': orders, 'worst': witness, 'facet_residual_tolerance': 1e-10, 'numerical_membership': 'outside' if worst['max_facet_residual'] > 1e-10 else 'contained_within_tolerance'}
    return result


def main():
    factors = {n: factor(n) for n in (3, 5, 7)}
    cases = [case(factors[3], factors[3]), case(factors[5], factors[5])]
    if cases[0]['worst']['max_facet_residual'] <= 1e-5:
        raise AssertionError('3x3 negative control did not separate')
    if cases[1]['worst']['max_facet_residual'] > 1e-10:
        raise AssertionError('5x5 positive control failed containment tolerance')
    cases.append(case(factors[5], factors[7]))
    obj = {'definition': {'dual_vertices': 'u_i=n_i/cos(pi/n)', 'closure_weights': 'cross-product barycentric identity, normalized to sum 1; only all-positive triples', 'T6': '0.5*(x1*y1^T-x3*y2^T), fixed first A step and all 2*6 cyclic orders', 'T4': 'conv of ±0.5 outer(n_i/(1+h_n), n_j/(1+h_m))', 'flatten_order': 'row-major 2x2', 'facet_residual': 'unit Euclidean facet normal dot vec(T)+offset; positive is outside'}, 'identity': {'source': str(SOURCE), 'source_sha256': sha256(SOURCE), 'script': str(Path(__file__)), 'script_sha256': sha256(Path(__file__)), 'python': sys.version, 'numpy': np.__version__, 'scipy': scipy.__version__}, 'factors': {str(k): v for k, v in factors.items()}, 'cases': cases}
    target = OUT / 'results.json'
    if target.exists():
        raise FileExistsError(target)
    target.write_text(json.dumps(obj, indent=2) + '\n')
    for row in cases:
        print(f"{row['pair']}: triples={row['a_triple_count']}x{row['b_triple_count']}, T6={row['six_tensor_count']}, T4 points={row['four_point_count_with_signs']}, facets={row['four_hull_facets']}, worst residual={row['worst']['max_facet_residual']:.12g}, {row['numerical_membership']}")


if __name__ == '__main__':
    main()
