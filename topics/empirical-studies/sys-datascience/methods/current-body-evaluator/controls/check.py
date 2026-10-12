"""Check retained analytic controls; no target calls."""
import json
import math
from pathlib import Path
p=Path(__file__).resolve().parent
rows=[json.loads(line) for line in (p/'results.jsonl').read_text().splitlines()]
r={x['id']:x for x in rows}
assert len(rows)==8 and len(r)==8
for key in ('cube','cube_scaled2','tri','tri_u2','tri_so4','cube_so4'):
    v=r[key];assert v['status']=='ok',v
    assert v['capacity_lower']<=v['capacity']<=v['capacity_upper']
    assert math.isclose(v['sys'],v['capacity']**2/(2*v['volume']),rel_tol=1e-13)
assert math.isclose(r['cube']['capacity'],4,rel_tol=1e-12)
assert math.isclose(r['cube']['volume'],16,rel_tol=1e-12)
assert math.isclose(r['cube_scaled2']['capacity']/r['cube']['capacity'],4,rel_tol=1e-12)
assert math.isclose(r['cube_scaled2']['volume']/r['cube']['volume'],16,rel_tol=1e-12)
assert math.isclose(r['tri_u2']['capacity'],r['tri']['capacity'],rel_tol=1e-10)
for a,b in [('tri','tri_u2'),('tri','tri_so4'),('cube','cube_so4')]:
    assert math.isclose(r[a]['volume'],r[b]['volume'],rel_tol=1e-12)
assert r['cube']['capacity_route']=='product'
assert r['cube_so4']['capacity_route']=='general'
assert r['unbounded']['status']=='error' and r['unbounded']['error_stage']=='geometry'
assert r['bad_schema']['status']=='error' and r['bad_schema']['error_stage']=='input_schema'
print('PASS: analytic cube, scaling, U(2), rotated volume, structural dispatch, typed failures; six valid and two invalid requests')
failure_rows=[json.loads(line) for line in (p/'failure-results.jsonl').read_text().splitlines()]
assert len(failure_rows)==4
assert failure_rows[0]['error_type']=='Timeout'
assert failure_rows[1]['error']=='duplicate id'
assert all(v['status']=='error' and 'input_raw' in v for v in failure_rows)
print('PASS: timeout, duplicate ID, nonfinite number and malformed JSON retained')
