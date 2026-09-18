import collections,json,math,statistics,sys
from pathlib import Path
p=Path(sys.argv[1]); rows=[json.loads(x) for x in p.open()]; summary=collections.defaultdict(list); details=[]
expected={r['id'] for r in map(json.loads,(Path(__file__).parent/'pilot-inputs.jsonl').open())}
assert len(rows)==len(expected)==len({r['id'] for r in rows}) and {r['id'] for r in rows}==expected
for r in rows:
 b=r['input']; d={'id':r['id'],'bucket':b['bucket'],'status':r['status'],'wall_ms':r['wall_ms']}
 if r['status']=='ok':
  for f in ['capacity','volume','sys']:
   d[f+'_relative_delta']=(r[f]-b['legacy_'+f])/b['legacy_'+f]
  d['legacy_capacity_in_current_bounds']=r['capacity_lower']<=b['legacy_capacity']<=r['capacity_upper']
  d['capacity_route']=r['capacity_route']
 else:d['error_type']=r.get('error_type')
 details.append(d); summary[b['bucket']].append(d)
out={'rows':len(rows),'unique_ids':len({r['id'] for r in rows}),'successful':sum(r['status']=='ok' for r in rows),'total_wall_seconds':sum(r['wall_ms'] for r in rows)/1000,'strata':{},'details':details}
for b,rs in summary.items():
 o={'rows':len(rs),'statuses':dict(collections.Counter(r['status'] for r in rs)),'total_wall_ms':sum(r['wall_ms'] for r in rs)}
 for f in ['capacity','volume','sys']:
  ds=[abs(r[f+'_relative_delta']) for r in rs if r['status']=='ok'];o['max_abs_'+f+'_relative_delta']=max(ds) if ds else None
 out['strata'][b]=o
Path(sys.argv[2]).write_text(json.dumps(out,indent=2)+'\n')
print(json.dumps({k:v for k,v in out.items() if k!='details'},indent=2))
