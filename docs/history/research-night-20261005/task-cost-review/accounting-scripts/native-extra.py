exec(open('/tmp/msc-math/night-cost-review/ownscratch/native.py').read().split('ids=')[0])
import collections
ids={t['thread_id'] for t in r['threads']};st=c.date('2026-10-04T22:49:45Z');en=c.date('2026-10-04T23:04:45Z');rows,er=c.query('http://127.0.0.1:5080',c.auth_header(os.environ),ids,st,en)
for t in r['threads']:
 es=[e for e in r['events'] if e['thread_id']==t['thread_id'] and st<=c.date(e['timestamp_utc'])<=en];cc=collections.Counter((e['input_tokens'],e['cached_input_tokens'],e['output_tokens']) for e in es)
 extra=[]
 for row in rows:
  if row.get('conversation_id')!=t['thread_id'] or row.get('event_kind')!='response.completed':continue
  k=tuple(int(row.get(n)) for n in ['input_token_count','cached_token_count','output_token_count'])
  if cc[k]:cc[k]-=1
  else:extra.append(row)
 if extra:print(t['agent_path'],json.dumps(extra))
