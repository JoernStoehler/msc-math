import collections,json
from fractions import Fraction
from pathlib import Path
root=Path(__file__).resolve().parent; details=[]; summary={}
oldpilot={r['id']:r for r in map(json.loads,(root.parent/'target-pilot/results.jsonl').open())}
for group,path,inp in [('controls',root/'control-results.jsonl',root/'control-inputs.jsonl'),('interval',root/'interval/results.jsonl',root/'interval/inputs.jsonl'),('norm',root/'norm-results.jsonl',root/'norm/inputs.jsonl')]:
 rows=list(map(json.loads,path.open())); expected={r['id']:r for r in map(json.loads,inp.open())};assert len(rows)==len(expected)==len({r['id'] for r in rows});assert {r['id'] for r in rows}==set(expected)
 ds=[]
 for r in rows:
  assert r['input']==expected[r['id']]
  b=r['input'];k=b.get('transform',{}).get('k',0);t=Fraction(2)**k
  d=dict(group=group,id=r['id'],status=r['status'],wall_ms=r['wall_ms'],primal_scale=str(t))
  if r['status']=='capacity_interval':
   assert r['fresh_geometry_and_input_policy_passed']
   lo=Fraction(r['capacity_lower'])/t**2;hi=Fraction(r['capacity_upper'])/t**2;vol=Fraction(r['volume'])/t**4;legacy=Fraction(b['legacy_capacity']);mid=(lo+hi)/2
   assert 0<lo<=hi
   d.update(capacity_lower_original=str(lo),capacity_upper_original=str(hi),volume_original_numerical=float(vol),legacy_capacity_in_bounds=lo<=legacy<=hi,capacity_midpoint_relative_delta=float((mid-legacy)/legacy),capacity_relative_halfwidth=float((hi-lo)/(hi+lo)),volume_relative_delta=float((vol-Fraction(b['legacy_volume']))/Fraction(b['legacy_volume'])),scalar_available=r['capacity_scalar'] is not None,scalar_error=r['scalar_error'])
   if r['capacity_exact'] is not None:
    exact=Fraction(r['capacity_exact'])/t**2;assert lo<=exact<=hi;d['capacity_exact_original']=str(exact)
   if group=='controls':
    old=oldpilot[r['id']]
    assert r['capacity_lower']==old['capacity_lower'] and r['capacity_upper']==old['capacity_upper'] and r['volume']==old['volume'] and r['capacity_scalar']==old['capacity']
  else:d['error_type']=r.get('error_type')
  ds.append(d);details.append(d)
 ok=[d for d in ds if d['status']=='capacity_interval']
 summary[group]=dict(requests=len(ds),statuses=dict(collections.Counter(d['status'] for d in ds)),wall_seconds=sum(d['wall_ms'] for d in ds)/1000,scalar_available=sum(d['scalar_available'] for d in ok),legacy_inside=sum(d['legacy_capacity_in_bounds'] for d in ok),max_relative_halfwidth=max((d['capacity_relative_halfwidth'] for d in ok),default=None),max_abs_capacity_midpoint_relative_delta=max((abs(d['capacity_midpoint_relative_delta']) for d in ok),default=None),max_abs_volume_relative_delta=max((abs(d['volume_relative_delta']) for d in ok),default=None))
(root/'comparison.json').write_text(json.dumps(dict(summary=summary,details=details),indent=2)+'\n');print(json.dumps(summary,indent=2))
