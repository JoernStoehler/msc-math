"""Read-only independent audit of dominance and the ONE-circuit event packet."""
import importlib.util
from itertools import combinations
from pathlib import Path
import json
import time

root = Path(__file__).resolve().parent
started = time.monotonic()
# Reuse only the independent reviewer's rational-pair arithmetic definitions.
spec = importlib.util.spec_from_file_location('review_arithmetic',
    root.parent.parent / 'hko-angle-nonconformal' / 'review_candidates.py')
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)
E, z = m.E, m.z
U = E(0, '1/5')
packet = json.loads((root / 'active-events.json').read_text())
dec = lambda v: E(*v)


def line(c, u):
    return c[1] + c[3] * u, c[0] + c[2] * u


def dominance(a, b):
    diff = [u - v for u, v in zip(a, b)]
    return all(v.sign() >= 0 for u in (z, U) for v in line(diff, u))


def polynomial_roots(p):
    c, b, a = p
    if a == z:
        if b == z:
            assert c != z, 'Persistent zero polynomial requires explicit handling'
            return []
        r = -c / b
        return [r] if r.sign() >= 0 and (U - r).sign() >= 0 else []
    atU = c + b * U + a * U * U
    if c == z or atU == z:
        candidates = [z, -b / a] if c == z else [U, c / (a * U)]
        return list({r.key(): r for r in candidates
                     if r.sign() >= 0 and (U - r).sign() >= 0}.values())
    values = [c, atU]
    vertex = -b / (2 * a)
    if vertex.sign() > 0 and (U - vertex).sign() > 0:
        values.append(c + b * vertex + a * vertex * vertex)
    assert all(v.sign() > 0 for v in values) or all(v.sign() < 0 for v in values), \
        'Unexcluded interior root needs independent algebraic isolation'
    return []


def audit_hull(raw, u, record):
    lines = [line(c, u) for c in raw]
    cells = {}
    for i, (a, b) in enumerate(lines):
        lo, hi, possible = z, None, True
        for c, d in lines:
            slope, intercept = a - c, b - d
            if slope == z:
                if intercept.sign() < 0:
                    possible = False
                    break
            else:
                bound = -intercept / slope
                if slope.sign() > 0:
                    if (bound - lo).sign() > 0:
                        lo = bound
                elif hi is None or (hi - bound).sign() > 0:
                    hi = bound
            if hi is not None and (hi - lo).sign() < 0:
                possible = False
                break
        if possible and (hi is None or (hi - lo).sign() > 0):
            cells.setdefault((lo.key(), None if hi is None else hi.key()), []).append(i)
    actual = {(dec(c['left']).key(), None if c['right'] is None else dec(c['right']).key()):
              c['active'] for c in record['cells']}
    assert cells == actual
    for tie in record['ties']:
        x = dec(tie['x'])
        values = [a * x + b for a, b in lines]
        highest = values[0]
        for value in values[1:]:
            if (value - highest).sign() > 0:
                highest = value
        assert highest == dec(tie['value'])
        assert [i for i, value in enumerate(values) if value == highest] == tie['active']
    maxslope = lines[0][0]
    for slope, _ in lines[1:]:
        if (slope - maxslope).sign() > 0:
            maxslope = slope
    assert [i for i, (slope, _) in enumerate(lines) if slope == maxslope] == record['asymptotic_max_slope_candidates']


witnesses = 0
for circuit in packet['circuits']:
    raw = [[dec(v) for v in c['coefficients']] for c in circuit['candidates']]
    assert len(raw) == len({tuple(v.key() for v in c) for c in raw}) == 121
    survivors = circuit['survivors']
    eliminated = {int(i): witnesses for i, witnesses in circuit['eliminated'].items()}
    assert set(survivors) | set(eliminated) == set(range(121))
    assert not set(survivors) & set(eliminated)
    graph = {}
    for i, records in eliminated.items():
        assert records
        graph[i] = []
        for witness in records:
            j = witness['candidate']
            assert j != i and dominance(raw[j], raw[i])
            ends = [line([a - b for a, b in zip(raw[j], raw[i])], u) for u in (z, U)]
            assert [[v.key() for v in pair] for pair in ends] == [
                [dec(rec['slope']).key(), dec(rec['intercept']).key()]
                for rec in witness['endpoint_differences']]
            assert witness['boundary_entire_x_ties'] == [k for k, (a, b) in enumerate(ends) if a == z and b == z]
            assert witness['boundary_x_zero_ties'] == [k for k, (_, b) in enumerate(ends) if b == z]
            graph[i].append(j)
            witnesses += 1
    for i in survivors:
        assert not any(j != i and dominance(raw[j], raw[i]) for j in range(121))
    complete = set(survivors)
    def visit(i, path):
        if i in complete:
            return
        assert i not in path, 'Dominance cycle'
        for j in graph[i]:
            visit(j, path | {i})
        complete.add(i)
    for i in eliminated:
        visit(i, set())

circuit = packet['circuits'][0]
raw = [[dec(v) for v in c['coefficients']] for c in circuit['candidates']]
ids = circuit['survivors']
expected = {}
for i, j in combinations(ids, 2):
    diff = [a - b for a, b in zip(raw[i], raw[j])]
    expected[('slope', (i, j))] = [diff[1], diff[3], z]
    expected[('intercept', (i, j))] = [diff[0], diff[2], z]
for i, j, k in combinations(ids, 3):
    p = [a - b for a, b in zip(raw[i], raw[j])]
    q = [a - b for a, b in zip(raw[i], raw[k])]
    expected[('triple', (i, j, k))] = [p[1]*q[0]-q[1]*p[0],
        p[1]*q[2]+p[3]*q[0]-q[1]*p[2]-q[3]*p[0], p[3]*q[2]-q[3]*p[2]]
event = packet['circuit_zero']
assert len(expected) == len(event['polynomials']) == 30
allroots = {z.key(), U.key()}
for record in event['polynomials']:
    poly = expected.pop((record['kind'], tuple(record['ids'])))
    assert poly == list(map(dec, record['coefficients']))
    roots = polynomial_roots(poly)
    assert {r.key() for r in roots} == {dec(r).key() for r in record['roots']}
    allroots.update(r.key() for r in roots)
assert allroots == {z.key(), U.key()} and not expected
for patch in event['open_patches']:
    audit_hull(raw, dec(patch['sample']), patch['hull'])
for orientation in event['event_orientations']:
    audit_hull(raw, dec(orientation['u']), orientation['hull'])
print(json.dumps({'status': 'PASS', 'distinct_candidates': 605,
                  'survivors': [c['survivor_count'] for c in packet['circuits']],
                  'verified_dominance_witnesses': witnesses, 'circuit_zero_event_polynomials': 30,
                  'circuit_zero_interior_events': 0, 'raw_hulls_and_ties': 3,
                  'elapsed_seconds': time.monotonic() - started}))
