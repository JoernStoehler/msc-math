"""Strict paired-grid analysis; no evaluation."""
import hashlib,json,math,statistics
from pathlib import Path
p=Path(__file__).resolve().parent/'artifacts'
inputs=[json.loads(x) for x in (p/'inputs.jsonl').read_text().splitlines()]
results=[json.loads(x) for x in (p/'results.jsonl').read_text().splitlines()]
expected={(b,r,a) for b in ('4x4','4x6') for r in range(8) for a in ('00','10','01','11')}
assert len(inputs)==64 and {(v['bucket'],v['row'],v['arm']) for v in inputs}==expected
assert len({v['id'] for v in inputs})==64
byid={v['id']:v for v in results}
complete=len(results)==64 and len(byid)==64 and set(byid)=={v['id'] for v in inputs} and all(v['status']=='ok' for v in results)
summary={'complete':complete,'requests':len(results),'successes':sum(v['status']=='ok' for v in results),
         'total_request_wall_ms':sum(v['wall_ms'] for v in results)}
if complete:
 assert len({json.dumps(v['evaluator'],sort_keys=True) for v in results})==1
 assert next(iter(byid.values()))['evaluator']['input_file_sha256']==hashlib.sha256((p/'inputs.jsonl').read_bytes()).hexdigest()
 for v in inputs:
  r=byid[v['id']];assert r['input']==v
  assert math.isclose(r['sys'],r['capacity']**2/(2*r['volume']),rel_tol=1e-12)
  assert r['capacity_lower']<=r['capacity']<=r['capacity_upper']
  assert r['capacity_route']=='product'
 paired=[]
 for bucket in ('4x4','4x6'):
  for row in range(8):
   s={v['arm']:byid[v['id']]['sys'] for v in inputs if v['bucket']==bucket and v['row']==row}
   paired.append({'bucket':bucket,'row':row,'sys':s,'both_minus_base':s['11']-s['00'],
                  'q_minus_base':s['10']-s['00'],'p_minus_base':s['01']-s['00'],
                  'interaction':s['11']-s['10']-s['01']+s['00']})
 def stats(rows):
  return {key:{'mean':statistics.mean(r[key] for r in rows),'min':min(r[key] for r in rows),
               'max':max(r[key] for r in rows),'positive':sum(r[key]>0 for r in rows),
               'zero':sum(r[key]==0 for r in rows),'negative':sum(r[key]<0 for r in rows)}
          for key in ('both_minus_base','q_minus_base','p_minus_base','interaction')}
 summary.update(paired=paired,buckets={b:stats([r for r in paired if r['bucket']==b]) for b in ('4x4','4x6')},
                equal_bucket_summary=stats(paired),max_sys=max(v['sys'] for v in results),sys_gt_one=sum(v['sys']>1 for v in results))
(p/'analysis.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps({k:v for k,v in summary.items() if k!='paired'},indent=2))
