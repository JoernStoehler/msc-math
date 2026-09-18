import collections,gzip,hashlib,json,statistics
from pathlib import Path
root=Path(__file__).resolve().parent
expected={r['id']:r for r in map(json.loads,(root/'inputs.jsonl').open())}
strata=collections.defaultdict(list); details=[]; seen=set(); errs=[]
with (root/'results.jsonl').open() as f:
 for line in f:
  r=json.loads(line); pid=r['id']; assert pid in expected and pid not in seen;seen.add(pid)
  old=expected[pid]; assert r['input']==old
  d=dict(id=pid,bucket=old['bucket'],status=r['status'],wall_ms=r['wall_ms'])
  if r['status']=='ok':
   for k in ['capacity','volume','sys']:
    d[k+'_relative_delta']=(r[k]-old['legacy_'+k])/old['legacy_'+k]
    d['new_'+k]=r[k];d['legacy_'+k]=old['legacy_'+k]
   d.update(capacity_route=r['capacity_route'],legacy_capacity_in_current_bounds=r['capacity_lower']<=old['legacy_capacity']<=r['capacity_upper'],capacity_lower=r['capacity_lower'],capacity_upper=r['capacity_upper'])
  else:d.update(error_type=r.get('error_type'),error_stage=r.get('error_stage'));errs.append(d)
  details.append(d);strata[d['bucket']].append(d)
summary=dict(error_classes=dict(collections.Counter(r.get('error_type','').split(' {')[0] for r in errs)),expected=len(expected),returned=len(seen),missing_ids=sorted(set(expected)-seen),errors=errs,strata={},source_input_sha256=hashlib.sha256((root/'inputs.jsonl').read_bytes()).hexdigest(),result_sha256=hashlib.sha256((root/'results.jsonl').read_bytes()).hexdigest())
for name,rs in [('all',details),*sorted(strata.items())]:
 ok=[r for r in rs if r['status']=='ok'];s=dict(rows=len(rs),successful=len(ok),wall_seconds=sum(r['wall_ms'] for r in rs)/1000,max_wall_ms=max(r['wall_ms'] for r in rs),legacy_capacity_outside_bounds=sum(not r['legacy_capacity_in_current_bounds'] for r in ok))
 for k in ['capacity','volume','sys']:
  s['max_abs_'+k+'_relative_delta']=max((abs(r[k+'_relative_delta']) for r in ok),default=None)
 s['max_new_sys']=max((r['new_sys'] for r in ok),default=None)
 if name=='all':summary['all']=s
 else:summary['strata'][name]=s
(root/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
with gzip.open(root/'comparison.jsonl.gz','wt') as f:
 for d in details:f.write(json.dumps(d)+'\n')
print(json.dumps(summary['all'],indent=2))
