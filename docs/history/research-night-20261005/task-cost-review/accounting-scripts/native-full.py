exec(open('/tmp/msc-math/night-cost-review/ownscratch/native.py').read().split('ids=')[0])
import collections
ids={t['thread_id'] for t in r['threads']};checks=[];extras=[];missing=[]
for b in r['bins_minutes']['15']:
 st=c.date(b['start_utc']);en=c.date(b['end_utc'])
 rows,err=c.query('http://127.0.0.1:5080',c.auth_header(os.environ),ids,st,en)
 # Standard half-open bins; native search may include endpoint.
 rows=[row for row in rows if int(st.timestamp()*1e6)<=row['_timestamp']<int(en.timestamp()*1e6)]
 tot=c.token_totals(rows); checks.append({'start_utc':st.isoformat(),'end_utc':en.isoformat(),'rows':len(rows),'query_errors':err,'native_totals':tot,'local_response_count':b['responses']})
 cc=collections.Counter((e['thread_id'],e['input_tokens'],e['cached_input_tokens'],e['output_tokens']) for e in r['events'] if st<=c.date(e['timestamp_utc'])<en)
 for row in rows:
  if row.get('event_kind')!='response.completed':continue
  try:k=(row['conversation_id'],)+tuple(int(row[n]) for n in ['input_token_count','cached_token_count','output_token_count'])
  except (KeyError,TypeError,ValueError):continue
  if cc[k]:cc[k]-=1
  else:extras.append(row)
 missing.extend({'bin_start_utc':st.isoformat(),'counter_tuple':list(k),'count':n} for k,n in cc.items() if n)
r['native_otel_bounded_crosscheck']={'samples':checks,'extra_native_counter_rows':extras,'unmatched_local_counter_tuples':missing,'caveat':'Whole window checked in17bounded safe-projection queries. Counters matched as per-thread multisets; response IDs absent from native projection, native export dedup unverified. Extra native rows are not silently added to owned-response totals.'}
p.write_text(json.dumps(r,indent=2)+'\n');print(json.dumps({'native_valid':sum(x['native_totals']['valid_token_records'] for x in checks),'native_missing':sum(x['native_totals']['invalid_or_missing_token_records'] for x in checks),'extra_native':len(extras),'extra_shapes':list(set((x['input_token_count'],x['cached_token_count'],x['output_token_count']) for x in extras)),'missing_local':len(missing),'query_errors':[x['query_errors'] for x in checks if x['query_errors']]}))
