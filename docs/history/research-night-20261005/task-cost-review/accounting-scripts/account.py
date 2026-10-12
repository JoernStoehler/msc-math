import importlib.util,pathlib,json,collections,datetime,hashlib
ROOT='01a1089f-b4f6-7031-9e98-b648dabb1770'
def imp(name,path):
 s=importlib.util.spec_from_file_location(name,path); m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
m=imp('u','/workspaces/msc-math/scripts/subtree-usage.py');home=pathlib.Path('/home/joern/.codex');a=m.inventory(home)
start=m.date('2026-10-04T22:49:45Z');end=m.date('2026-10-05T02:56:25Z');warnings=set();events=[];threadrows=[];quota=[];reasoning={};seen_global=set();native=0
fields=['input_tokens','cached_input_tokens','noncached_input_tokens','output_tokens','reasoning_output_tokens','cache_write_input_tokens','responses']
def lane(p):
 if p=='/root':return 'root coordination'
 if 'night_supervisor' in p:return 'supervision'
 if 'direct_cover_lead' in p:return 'pentagon global bound / matrix extension'
 if 'f11_acceptance' in p or 'f11_cross_review' in p or 'f11_proof_reader' in p:return 'HKO F11 acceptance'
 if 'extremality_discovery' in p:return 'P* facet-birth local geometry'
 if 'geometry_discovery' in p:return 'geometry mechanisms and source integration'
 if 'scientific_compare' in p:return 'scientific comparison and integration'
 return 'earlier preparation / other'
def totals(es):
 d={k:0 for k in fields};d['reasoning_missing_responses']=0;d['cache_write_missing_responses']=0
 for e in es:
  for k in fields:
   if k=='responses':d[k]+=1
   elif e[k] is None:d[k]=None
   elif d[k] is not None:d[k]+=e[k]
  d['reasoning_missing_responses']+=e['reasoning_output_tokens'] is None;d['cache_write_missing_responses']+=e['cache_write_input_tokens'] is None
 return d
for i in sorted(m.descendants(a,ROOT)):
 p,v=a[i]
 if m.date(v['timestamp'])>end:continue
 ap=v.get('agent_path') or '/root'
 # Raw accounting envelope only. Mirrors excluded; owned response records give reasoning subtype.
 for no,l in enumerate(p.open(),1):
  if '"token_usage_record"' not in l and '"token_count"' not in l:continue
  r=json.loads(l);when=m.date(r['timestamp']);pl=r.get('payload',{})
  if not start<=when<=end:continue
  if r['type']=='token_usage_record' and pl.get('thread_id')==i:
   key=pl.get('response_id');u=pl.get('usage',{});reasoning[(i,when.isoformat())]=u.get('reasoning_output_tokens');native+=1
   if key in seen_global:warnings.add('global duplicate response ID detected')
   seen_global.add(key)
  if r['type']=='event_msg' and pl.get('type')=='token_count' and pl.get('rate_limits'):
   q=pl['rate_limits'];quota.append({'timestamp_utc':when.isoformat(),'thread_id':i,'source':str(p)+':'+str(no),'snapshot':q})
 es=[]
 for when,model,u in m.usage_events(p,v,end,warnings):
  if when<start:continue
  inp,cache,out,writes=u
  e={'timestamp_utc':when.isoformat(),'thread_id':i,'agent_path':ap,'lane':lane(ap),'model':model,'input_tokens':inp,'cached_input_tokens':cache,'noncached_input_tokens':inp-cache,'output_tokens':out,'cache_write_input_tokens':writes,'reasoning_output_tokens':reasoning.get((i,when.isoformat()))}
  es.append(e);events.append(e)
 threadrows.append({'thread_id':i,'agent_path':ap,'lane':lane(ap),'source':str(p),'source_sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'events_in_window':len(es),'first_usage_utc':es[0]['timestamp_utc'] if es else None,'last_usage_utc':es[-1]['timestamp_utc'] if es else None,**totals(es)})
events.sort(key=lambda e:e['timestamp_utc'])
groups=[]
for ln in sorted(set(e['lane'] for e in events)):groups.append({'lane':ln,**totals([e for e in events if e['lane']==ln])})
bins={}
for minutes in [5,15]:
 out=[];t=start
 while t<end:
  nxt=min(t+datetime.timedelta(minutes=minutes),end);es=[e for e in events if t<=m.date(e['timestamp_utc'])<nxt or (nxt==end and m.date(e['timestamp_utc'])==end)]
  out.append({'start_utc':t.isoformat(),'end_utc':nxt.isoformat(),**totals(es),'by_lane':[{'lane':ln,**totals([e for e in es if e['lane']==ln])} for ln in sorted(set(e['lane'] for e in es))]});t=nxt
 bins[str(minutes)]=out
# Deduplicate same account quota snapshot across threads, retaining first and last dates per state.
qstates={}
for q in sorted(quota,key=lambda q:q['timestamp_utc']):
 key=json.dumps(q['snapshot'],sort_keys=True)
 if key not in qstates:qstates[key]={'first_observed_utc':q['timestamp_utc'],'last_observed_utc':q['timestamp_utc'],'snapshot':q['snapshot'],'first_source':q['source'],'observation_count':1}
 else:qstates[key]['last_observed_utc']=q['timestamp_utc'];qstates[key]['observation_count']+=1
result={'scope':{'root_thread_id':ROOT,'start_utc':start.isoformat(),'end_utc':end.isoformat(),'elapsed_minutes':(end-start).total_seconds()/60,'discovered_pre_cutoff_threads':len(threadrows),'threads_with_usage':sum(bool(t['events_in_window']) for t in threadrows),'current_root_descendants':len(m.descendants(a,ROOT))},'totals':totals(events),'lanes':groups,'threads':threadrows,'bins_minutes':bins,'quota_snapshots':list(qstates.values()),'warnings':sorted(warnings),'events':events,'native_raw_owned_records_in_window':native,'actual_billed_spend':None,'subscription_quota_attributed_to_this_window':None,'notes':['input_tokens includes cached input; noncached=input-cached and includes cache-write tokens if any','reasoning_output_tokens is a subtype of output_tokens, never add twice','bins use accounting-event timestamps, not continuous walltime/token-generation timestamps','subscription quota accountwide, rounded, attribution/conversion unknown','semantic lanes are stable thread ownership, not per-turn science-vs-coordination classification','source files may continue growing after captured hash; accounting clipped to fixed historical interval']}
pathlib.Path('/tmp/msc-math/night-cost-review/resources.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({'scope':result['scope'],'totals':result['totals'],'lanes':groups,'warnings':sorted(warnings),'quota_states':len(qstates)},indent=2))
