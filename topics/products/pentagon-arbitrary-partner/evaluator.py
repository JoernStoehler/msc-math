"""Floating-point normal-triangle fitting oracle; see README for mathematical contract."""
from itertools import combinations
from time import perf_counter
import json
from pathlib import Path
import numpy as np
import scipy
from scipy.optimize import linprog
from scipy.spatial import ConvexHull


def regular(n, angle=0.0):
    theta = angle + 2*np.pi*np.arange(n)/n
    return np.column_stack([np.cos(theta), np.sin(theta)])


def rotation(theta):
    return np.array([[np.cos(theta), -np.sin(theta)], [np.sin(theta), np.cos(theta)]])


def templates():
    # Q circumradius 1; facet normals at 2*pi*i/5; vertices at pi/5+2*pi*i/5.
    normals = regular(5)
    h = np.cos(np.pi/5)
    result = []
    for ids in combinations(range(5), 3):
        a = normals[list(ids)]
        weights = np.linalg.solve(np.vstack([a.T, np.ones(3)]), [0., 0., 1.])
        if min(weights) <= 1e-12:
            continue
        edges = a * (weights / h)[:, None]
        for order in [(0, 1, 2), (0, 2, 1)]:
            e = edges[list(order)]
            vertices = np.array([[0., 0.], e[0], e[0]+e[1]])
            result.append({'id': f'{ids}-{order}', 'normal_ids': list(ids),
                           'edge_order': list(order), 'vertices': vertices.tolist()})
    assert len(result) == 10
    return result


def evaluate(points):
    start = perf_counter()
    hull = ConvexHull(points)
    vertices = np.asarray(points)[hull.vertices]
    A, b = hull.equations[:, :2], -hull.equations[:, 2]
    fits = []
    for template in templates():
        T = np.array(template['vertices'])
        support = np.max(A @ T.T, axis=1)
        C = np.column_stack([A, support])
        lp = linprog([0., 0., -1.], A_ub=C, b_ub=b,
                     bounds=[(None, None), (None, None), (0., None)], method='highs')
        if not lp.success:
            raise RuntimeError(f'{template["id"]}: {lp.message}')
        slack = b-C@lp.x
        dual = -lp.ineqlin.marginals
        fits.append({'template_id': template['id'], 'scale': float(lp.x[2]),
                     'translation': lp.x[:2].tolist(), 'slacks': slack.tolist(),
                     'active_facets': np.flatnonzero(np.abs(slack)<1e-8).tolist(),
                     'dual_weights': dual.tolist(),
                     'primal_violation': float(max(0., -min(slack))),
                     'dual_stationarity_residual': float(np.max(np.abs(C.T@dual-[0,0,1]))),
                     'duality_gap': float(abs(b@dual-lp.x[2]))})
    c = min(f['scale'] for f in fits)
    q_area = 2.5*np.sin(2*np.pi/5)
    return {'vertices': vertices.tolist(), 'facet_normals': A.tolist(),
            'facet_offsets': b.tolist(), 'area': float(hull.volume),
            'capacity': c, 'sys': c*c/(2*q_area*hull.volume), 'fits': fits,
            'seconds': perf_counter()-start}


def calibrate():
    start = perf_counter()
    Q = regular(5, np.pi/5)
    normals = regular(5)
    w = 1+np.cos(np.pi/5)
    checks, records = [], []
    matrices = [rotation(t) for t in [0., .113, np.pi/10, .49]] + [
        np.array([[1.4,.3],[0.,.8]]), np.array([[-1.,.2],[.1,.9]])]
    for i, H in enumerate(matrices):
        result = evaluate(Q@H.T)
        expected = w*w / np.max(np.abs(normals@np.linalg.inv(H)@normals.T))
        # Here G=H in first factor and second factor is undeformed Q.
        checks.append({'check': f'affine_{i}', 'error': abs(result['capacity']-expected)})
        records.append({'id': f'affine_{i}', 'source': {'family':'affinely_regular_pentagon', 'matrix':H.tolist()}, **result})
    base = np.array([[-1.,-.6],[.6,-.9],[1.2,.1],[.5,1.1],[-.7,.8]])
    original = evaluate(base)
    records.append({'id':'irregular_control', 'source':{'family':'hand_specified_irregular_pentagon'}, **original})
    for label, vertices, factor in [('translated',base+[2.1,-3.2],1.), ('dilated',2.3*base,2.3),
                                    ('reflected',-base,1.), ('rotated_pi_over_5',base@rotation(np.pi/5).T,1.)]:
        result = evaluate(vertices)
        checks.append({'check':label,'error':abs(result['capacity']-factor*original['capacity'])})
        records.append({'id':label,'source':{'parent':'irregular_control','operation':label},**result})
    # Reflection permutes the ten templates. It is a capacity check, not per-index equality.
    for n in [3,4]:
        result = evaluate(regular(n,.17))
        checks.append({'check':f'theorem_control_{n}', 'error':max(0.,result['sys']-1.)})
        records.append({'id':f'regular_{n}', 'source':{'family':'regular','n':n,'angle':.17}, **result})
    rectangle = np.array([[-1.,-.7],[1.,-.7],[1.,.7],[-1.,.7]])
    result = evaluate(rectangle)
    direct = [min(2/np.ptp(np.array(t['vertices'])[:,0]),
                  1.4/np.ptp(np.array(t['vertices'])[:,1])) for t in templates()]
    checks.append({'check':'rectangle_direct_extent_formula', 'error':max(
        abs(f['scale']-d) for f,d in zip(result['fits'],direct))})
    records.append({'id':'rectangle','source':{'family':'axis_aligned_rectangle','half_widths':[1.,.7]},**result})
    # Consequence of fitting representation: <=30-vertex inner hull preserves full vector.
    placed = np.concatenate([np.array(t['vertices'])*f['scale']+f['translation']
                             for t,f in zip(templates(),original['fits'])])
    compressed = evaluate(placed)
    checks.append({'check':'full_vector_inner_hull','error':max(abs(a['scale']-b['scale'])
                    for a,b in zip(original['fits'],compressed['fits']))})
    records.append({'id':'inner_hull', 'source':{'parent':'irregular_control','operation':'hull_of_LP_optimal_placements'},**compressed})
    residual = max(max(f['primal_violation'], f['dual_stationarity_residual'], f['duality_gap'])
                   for r in records for f in r['fits'])
    checks.append({'check':'all_LP_residuals','error':residual})
    assert max(c['error'] for c in checks)<1e-8, checks
    return {'status':'floating_point_calibration_only_not_exact_certification',
            'numpy_version':np.__version__, 'scipy_version':scipy.__version__,
            'body_count':len(records),'wall_seconds':perf_counter()-start,
            'checks':checks,'templates':templates(),'records':records}

if __name__ == '__main__':
    result = calibrate()
    out = Path(__file__).with_name('calibration.json')
    out.write_text(json.dumps(result, indent=2)+'\n')
    print(json.dumps({k:v for k,v in result.items() if k not in ['records','templates']},indent=2))
