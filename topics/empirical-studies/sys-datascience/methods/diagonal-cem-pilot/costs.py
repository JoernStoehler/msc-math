"""Post-run descriptive cost accounting; no target or method changes."""
import json
from pathlib import Path
P=Path(__file__).resolve().parent;A=P/'artifacts'
config=json.loads((A/'frozen-config.json').read_text())
account=[]
for seed in config['seeds']:
 for arm in ('iid','cem'):
  names=[f'{seed}-shared-0']+[f'{seed}-{arm}-{g}' for g in (1,2,3)]
  construction=0.;evaluation=0.;updates=0.;completed_batches=0
  for name in names:
   gp=A/f'{name}-generation.json';cp=A/f'{name}-cost.json'
   if gp.exists():construction+=json.loads(gp.read_text())['process_wall_ms']
   if cp.exists():evaluation+=json.loads(cp.read_text())['evaluation_process_ms'];completed_batches+=1
  if arm=='cem':
   for g in (0,1,2):
    up=A/f'{seed}-update-{g}.json'
    if up.exists():updates+=json.loads(up.read_text())['process_wall_ms']
  account.append({'seed':seed,'arm':arm,'completed_batches':completed_batches,'generation_process_ms':construction,
                  'evaluation_process_ms':evaluation,'distribution_update_process_ms':updates,
                  'accounted_wall_ms':construction+evaluation+updates})
output={'note':'Descriptive batch process clock; shared initialization construction/evaluation charged to both arms. CEM updates charged only CEM. Omits driver/file overhead. Partial batches absent from cost files cannot support total-arm comparisons.', 'arms':account}
(A/'cost-account.json').write_text(json.dumps(output,indent=2)+'\n')
print(json.dumps(output,indent=2))
