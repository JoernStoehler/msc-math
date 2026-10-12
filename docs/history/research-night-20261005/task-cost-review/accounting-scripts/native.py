import importlib.util,pathlib,json,os,datetime
p=pathlib.Path('/tmp/msc-math/night-cost-review/resources.json');r=json.loads(p.read_text())
s=importlib.util.spec_from_file_location('c','/workspaces/msc-math/scripts/coordination-load.py');c=importlib.util.module_from_spec(s);s.loader.exec_module(c)
ids={t['thread_id'] for t in r['threads']}; checks=[]
for st,en in [('2026-10-04T22:49:45Z','2026-10-04T23:04:45Z'),('2026-10-05T02:41:25Z','2026-10-05T02:56:25Z')]:
 try:
  rows,errors=c.query('http://127.0.0.1:5080',c.auth_header(os.environ),ids,c.date(st),c.date(en));tot=c.token_totals(rows)
  checks.append({'start_utc':st,'end_utc':en,'rows':len(rows),'native_totals':tot,'query_errors':errors,'local_response_count':sum(c.date(st)<=c.date(e['timestamp_utc'])<=c.date(en) for e in r['events'])})
 except Exception as e:checks.append({'start_utc':st,'end_utc':en,'status':'unavailable','exception_type':type(e).__name__})
r['native_otel_bounded_crosscheck']={'samples':checks,'caveat':'Independent safe metadata projection; native response IDs absent so export dedup is unverified; sample agreement does not prove whole-window completeness.'}
for q in r['quota_snapshots']:q['snapshot']={k:v for k,v in q['snapshot'].items() if k in ['limit_id','primary','secondary','spend_control_reached','plan_type','rate_limit_reached_type']}
p.write_text(json.dumps(r,indent=2)+'\n');print(json.dumps(checks,indent=2))
