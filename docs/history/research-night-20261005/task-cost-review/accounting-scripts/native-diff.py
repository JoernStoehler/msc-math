exec(open('/tmp/msc-math/night-cost-review/ownscratch/native.py').read().split('ids=')[0])
ids={t['thread_id'] for t in r['threads']};st=c.date('2026-10-04T22:49:45Z');en=c.date('2026-10-04T23:04:45Z');rows,er=c.query('http://127.0.0.1:5080',c.auth_header(os.environ),ids,st,en)
for t in r['threads']:
 es=[e for e in r['events'] if e['thread_id']==t['thread_id'] and st<=c.date(e['timestamp_utc'])<=en];tt=c.token_totals([row for row in rows if row.get('conversation_id')==t['thread_id']]);n=tt['valid_token_records']
 if n!=len(es):print(t['agent_path'],n,len(es),tt['input_token_count'],sum(e['input_tokens'] for e in es))
