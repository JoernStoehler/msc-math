"""Format retained comparisons and fixed time checkpoints; no target calls."""
import json, collections
from pathlib import Path
p=Path(__file__).resolve().parent/'artifacts'
s=json.loads((p/'summary.json').read_text())
r=[json.loads(x) for x in (p/'results.jsonl').read_text().splitlines()]
c=[json.loads(x) for x in (p/'curves.jsonl').read_text().splitlines()]
checkpoints=[]
for b in s['blocks']:
    for seconds in [.01,.03,.1,.3,1,3,10,30,100,300,1000]:
        if not all(v['charged_ms']>=seconds*1000 for v in b['arms'].values()):continue
        row={'block':b['block'],'seconds':seconds}
        for arm in ['iid','orientation']:
            completed=[x for x in c if x['block']==b['block'] and x['arm']==arm and x['charged_ms']<=seconds*1000]
            row[arm]=completed[-1]['best_sys'] if completed else None
        row['orientation_minus_iid']=row['orientation']-row['iid'] if row['orientation'] is not None and row['iid'] is not None else None
        checkpoints.append(row)
(p/'time-checkpoints.json').write_text(json.dumps(checkpoints,indent=2)+'\n')
details={'routes':dict(collections.Counter(x['capacity_route'] for x in r)), 'overall_max_sys':max(x['sys'] for x in r),'zero_failure_rows':all(x['status']=='ok' for x in r),'orientation_improvement_from_identity':{b['block']:b['arms']['orientation']['best_sys']-next(x['sys'] for x in r if x['id']==b['block']+'-iid-0') for b in s['blocks']},'maximum_u2_delta':max(abs(x['delta_sys']) for x in s['u2_controls']),'maximum_rotated_volume_relative_change':max(abs(x['volume']/next(y['volume'] for y in r if y['id']==x['input']['block']+'-iid-0')-1.) for x in r if 'base_id' in x['input'])}
(p/'details.json').write_text(json.dumps(details,indent=2)+'\n')
print(json.dumps(details,indent=2))
