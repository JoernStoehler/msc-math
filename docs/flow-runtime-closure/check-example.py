#!/usr/bin/env python3
"""Check retained F6 witness independently using Python rational arithmetic."""
from fractions import Fraction as Q
from pathlib import Path
import json

root = Path(__file__).resolve().parents[2]
p = root / 'experiments/dev-flow-graph/visualize-tube/flow-graph-f6-tube.json'
x = json.loads(p.read_text())
t, o = x['closed_tube'], x['closed_orbit']
a = [[Q(v) for v in row] for row in x['dual_vertices_exact']]
points = [[Q(v) for v in row] for row in o['breakpoints_exact']]
times = list(map(Q, o['segment_times_exact']))
dot = lambda a,b: sum(u*v for u,v in zip(a,b))
assert x['sigma'] == [1,2,4,5,3]
# Snapshot starts on F1/F2 immediately before the segment along F2.
active = x['sigma'][1:] + x['sigma'][:1]
for k,(point,tau,facet) in enumerate(zip(points,times,active)):
    assert tau > 0
    assert all(dot(row,point) <= 1 for row in a)
    assert dot(a[facet],point) == 1
    row = a[facet]
    velocity = [-2*row[2],-2*row[3],2*row[0],2*row[1]]
    assert [v+tau*r for v,r in zip(point,velocity)] == points[(k+1)%5]
assert sum(times) == Q(o['action_exact'])
u = list(map(Q,t['fixed_point']['point_exact']))
m = [[Q(v) for v in row] for row in t['start_to_end']['matrix_exact']]
b = list(map(Q,t['start_to_end']['offset_exact']))
assert [dot(row,u)+offset for row,offset in zip(m,b)] == u
s = t['start_polygon']
base,eu,ev = (list(map(Q,s[f'construction_frame_{k}_exact'])) for k in ('base','u','v'))
assert [v+u[0]*r+u[1]*s for v,r,s in zip(base,eu,ev)] == points[0]
f = t['action_on_start']
assert dot(list(map(Q,f['coeff_exact'])),u)+Q(f['constant_exact']) == sum(times)
expected = [-1.601002,1.364891,4.482887,-4.446374,1.626615,-1.821973,
            36.063319,6.721729,-6.397030,.791856,.317243,
            .538394,5.134160,14.681452,16.367140,2.635397,39.356542]
values = sum(m,[]) + b + [Q(f['constant_exact'])] + list(map(Q,f['coeff_exact'])) + u + times + [sum(times)]
assert all(abs(float(v)-e) <= .50001e-6 for v,e in zip(values,expected))
print('PASS: exact closure, facet membership, positive times, fixed point, action, and all displayed 6-decimal values')
