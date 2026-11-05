"""Fresh exact geometry and volume certificate at the continued parameter."""
from itertools import combinations
import json, time
from sage.all import *
from common import *

start = time.monotonic()
vertices = {}
graph = matrix(ZZ, 20, 20)
for ids in combinations(range(20), 4):
    M = N.matrix_from_rows(ids)
    if M.det() == 0:
        continue
    v = M.solve_right(vector(K, [1] * 4))
    slack = vector(K, [1] * 20) - N * v
    if any(emb(x) < 0 for x in slack):
        continue
    active = tuple(i for i, x in enumerate(slack) if x == 0)
    assert len(active) == 4, (ids, active)
    vertices[active] = v
    for i in active:
        for j in active:
            if i != j and emb(O[i, j]) >= 0:
                graph[i, j] = 1
assert len(vertices) > 0
assert set().union(*(set(ids) for ids in vertices)) == set(range(20))

# Lawrence's simple-polytope formula, independently evaluated at two generic
# linear functionals. Each active matrix is nonsingular by construction.
volumes = []
for ell in [vector(K, [1, 2, 4, 8]), vector(K, [1, 3, 9, 27])]:
    total = K(0)
    for ids, v in vertices.items():
        M = N.matrix_from_rows(ids)
        det = M.det()
        absdet = det if emb(det) > 0 else -det
        slopes = ell * M.inverse()
        assert all(z != 0 for z in slopes), (ell, ids)
        total += (ell * v) ** 4 / (24 * absdet * prod(slopes))
    assert emb(total) > 0
    volumes.append(total)
assert volumes[0] == volumes[1]
V = volumes[0]

# Verify H/(1+2t/5) subset K_t by exact support maximization on all vertices
# of H={N0 x <= 1}. For each linear functional, its maximum on bounded H is
# attained at a vertex.
h_vertices = {}
for ids in combinations(range(10), 4):
    M = N0.matrix_from_rows(ids)
    if M.det() == 0:
        continue
    v = M.solve_right(vector(K, [1] * 4))
    slack = vector(K, [1] * 10) - N0 * v
    if any(emb(x) < 0 for x in slack):
        continue
    active = tuple(i for i, x in enumerate(slack) if x == 0)
    assert len(active) == 4
    h_vertices[active] = v
assert len(h_vertices) == 25, len(h_vertices)
max_supports = []
for i in range(20):
    exact = max((N.row(i) * v for v in h_vertices.values()), key=lambda z: emb(z))
    assert emb(exact) <= emb(1 + QQ(2) * t / 5), (i, exact)
    max_supports.append({'row': i, 'support': str(exact), 'numeric': float(emb(exact))})

(out / 'graph.txt').write_text('\n'.join(' '.join(map(str, graph.row(i))) for i in range(20)) + '\n')
summary = {
    'parameter': str(t),
    'vertices': len(vertices),
    'edges': int(sum(graph.list())),
    'all_twenty_facets_genuine': True,
    'all_vertices_simple': True,
    'vertex_active_sets': [list(ids) for ids in sorted(vertices)],
    'h_vertices': len(h_vertices),
    'coarse_inclusion': 'H/(1+2t/5) subset K_t',
    'coarse_scale': str(1 + QQ(2) * t / 5),
    'support_bounds_on_H': max_supports,
    'max_support_numeric': max(x['numeric'] for x in max_supports),
    'volume': str(V),
    'volume_numeric': float(emb(V)),
    'volume_method': 'Lawrence simple-polytope formula with two exact generic linear functionals',
    'seconds': time.monotonic() - start,
}
(out / 'exact-geometry-summary.json').write_text(json.dumps(summary, indent=2) + '\n')
print(json.dumps(summary, indent=2), flush=True)
