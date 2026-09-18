"""Strict accounting and descriptive paired comparison; no target calls."""
import hashlib,json,math,statistics
from pathlib import Path
P=Path(__file__).resolve().parent;A=P/'artifacts-supported-v2';ROOT=P.parents[3]
config=json.loads((A/'frozen-config.json').read_text())
assert all(hashlib.sha256((ROOT/k).read_bytes()).hexdigest()==v for k,v in config['sha256'].items())
status=json.loads((A/'run-status.json').read_text()) if (A/'run-status.json').exists() else {'complete':False,'error':'outer deadline/process termination'}
result_files=sorted(A.glob('*-results.jsonl'))
all_results=[json.loads(x) for f in result_files for x in f.read_text().splitlines()]
summary={'schema':'diagonal-cem-analysis-supported-v2','complete':status['complete'],'run_status':status,
         'actual_requests':len(all_results),'successes':sum(r['status']=='ok' for r in all_results),
         'actual_request_wall_ms':sum(r['wall_ms'] for r in all_results)}
if status['complete']:
 assert len(result_files)==21 and len(all_results)==1344 and all(r['status']=='ok' for r in all_results)
 assert len({r['id'] for r in all_results})==1344
 identities=[]
 for r in all_results:
  assert math.isclose(r['sys'],r['capacity']**2/(2*r['volume']),rel_tol=1e-12)
  assert r['capacity_lower']<=r['capacity']<=r['capacity_upper'] and r['capacity_route']=='product'
  identity=dict(r['evaluator']);identity.pop('input_file_sha256');identities.append(json.dumps(identity,sort_keys=True))
 assert len(set(identities))==1
 for f in result_files:
  ip=f.with_name(f.name.replace('-results.jsonl','-inputs.jsonl'))
  inputs=[json.loads(x) for x in ip.read_text().splitlines()]
  results=[json.loads(x) for x in f.read_text().splitlines()]
  assert len(inputs)==len(results)==64
  assert all(r['input']==i and r['evaluator']['input_file_sha256']==hashlib.sha256(ip.read_bytes()).hexdigest() for r,i in zip(results,inputs))
 comparisons=[]
 curves=[]
 for seed in config['seeds']:
  shared=[json.loads(x) for x in (A/f'{seed}-shared-0-results.jsonl').read_text().splitlines()]
  metrics={}
  for arm in ('iid','cem'):
   rows=list(shared)
   for gen in (1,2,3):rows.extend(json.loads(x) for x in (A/f'{seed}-{arm}-{gen}-results.jsonl').read_text().splitlines())
   assert len(rows)==256
   best=-math.inf;elapsed=0
   distinct={}
   for index,r in enumerate(rows,1):
    best=max(best,r['sys']);elapsed+=r['wall_ms']
    key=json.dumps(r['input']['dual_vertices'],separators=(',',':'))
    distinct[key]=r['sys']
    curves.append({'seed':seed,'arm':arm,'charged_call':index,'best_sys':best,'charged_evaluator_wall_ms':elapsed})
   top=sorted(distinct.values(),reverse=True)[:8]
   metrics[arm]={'max_sys':best,'distinct_top8_median':statistics.median(top),'distinct_geometries':len(distinct),'charged_evaluator_wall_ms':elapsed}
   names=[f'{seed}-shared-0']+[f'{seed}-{arm}-{g}' for g in (1,2,3)]
   costs=[json.loads((A/f'{name}-cost.json').read_text()) for name in names]
   gens=[json.loads((A/f'{name}-generation.json').read_text()) for name in names]
   metrics[arm].update(generation_process_ms=sum(c['generation_process_ms'] for c in costs), evaluation_process_ms=sum(c['evaluation_process_ms'] for c in costs), update_process_ms=sum(json.loads(f.read_text())['process_wall_ms'] for f in A.glob(f'{seed}-update-*.json')) if arm=='cem' else 0, construction_attempts=sum(len(g['construction']) for g in gens),construction_rejections=sum(sum(a['status']=='rejected' for a in g['construction']) for g in gens),distinct_top8_values=top)
  comparisons.append({'seed':seed,**metrics,'cem_minus_iid_max':metrics['cem']['max_sys']-metrics['iid']['max_sys'],
                      'cem_minus_iid_top8_median':metrics['cem']['distinct_top8_median']-metrics['iid']['distinct_top8_median']})
 with (A/'best-so-far.jsonl').open('w') as f:
  for r in curves:f.write(json.dumps(r)+'\n')
 summary.update(charged_calls=1536,shared_calls_charged_twice=192,comparisons=comparisons,max_sys=max(r['sys'] for r in all_results),
                threshold_crossings=sum(r['sys']>1 for r in all_results))
generations=[json.loads(f.read_text()) for f in A.glob('*-generation.json')]
summary['construction_attempts']=sum(len(g['construction']) for g in generations)
summary['construction_rejections']=sum(sum(r['status']=='rejected' for r in g['construction']) for g in generations)
summary['generation_process_ms']=sum(g['process_wall_ms'] for g in generations)
(A/'analysis.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps(summary,indent=2))
