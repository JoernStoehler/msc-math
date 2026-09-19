"""Post-run partial inspection curves, never completed-arm comparisons."""
import json,statistics
from pathlib import Path
P=Path(__file__).resolve().parent;A=P/'artifacts'
config=json.loads((A/'frozen-config.json').read_text())
curves=[];coverage=[]
for seed in config['seeds']:
 for arm in ('iid','cem'):
  files=[A/f'{seed}-shared-0-results.jsonl']+[A/f'{seed}-{arm}-{g}-results.jsonl' for g in (1,2,3)]
  rows=[json.loads(s) for f in files if f.exists() for s in f.read_text().splitlines()]
  best=None;distinct={}
  for index,r in enumerate(rows,1):
   if r['status']=='ok':
    best=max(best,r['sys']) if best is not None else r['sys']
    distinct[json.dumps(r['input']['dual_vertices'])]=r['sys']
   curves.append({'seed':seed,'arm':arm,'charged_call':index,'status':r['status'],'best_sys_so_far':best})
  coverage.append({'seed':seed,'arm':arm,'charged_requests_observed':len(rows),'successful_observations':sum(r['status']=='ok' for r in rows),
                   'final_256_call_max':None,'final_256_call_top8_median':None,
                   'partial_observed_max':best,'partial_distinct_top8_median':statistics.median(sorted(distinct.values(),reverse=True)[:8]) if distinct else None})
(A/'partial-coverage.json').write_text(json.dumps({'status':'incomplete; partial observations are not final method results','coverage':coverage},indent=2)+'\n')
with (A/'partial-best-so-far.jsonl').open('w') as f:
 for r in curves:f.write(json.dumps(r)+'\n')
print(json.dumps(coverage,indent=2))
