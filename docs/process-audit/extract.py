"""Read-only, focused rollout index. Does not decrypt or copy tool output bodies."""
import collections, glob, hashlib, json, pathlib
OUT=pathlib.Path(__file__).resolve().parent
ROOTS={'01a0a021-b283-7fa3-9e16-aec87ce950df','01a09fd9-2ffe-7681-850d-139298baf0c3'}
CUTOFF='2026-09-15T16:44:07Z'
files={}
for p in glob.glob('/home/agent/.codex/sessions/**/*.jsonl',recursive=True)+glob.glob('/home/agent/.codex/archived_sessions/*.jsonl'):
 try:
  with open(p) as f: m=json.loads(next(f))['payload']
  files[m['id']]=(p,m)
 except (ValueError,KeyError,StopIteration):pass
selected=set(ROOTS)
while True:
 new={i for i,(p,m) in files.items() if m.get('parent_thread_id',m.get('forked_from_id')) in selected}
 if new<=selected:break
 selected|=new
# Forked message timestamps are reset. Compare exact role/content with ancestors,
# not timestamps alone; inherited repeats are not independent behavior/evidence.
messages={}
for i in selected:
 messages[i]=set()
 for line in open(files[i][0]):
  e=json.loads(line);q=e.get('payload',{})
  if q.get('type')=='message':messages[i].add(json.dumps([q.get('role'),q.get('content')],sort_keys=True))
inventory=[]
for i in sorted(selected):
 p,m=files[i]; parent=m.get('parent_thread_id',m.get('forked_from_id')); ancestors=set(); a=parent
 while a in files and a not in ancestors:
  ancestors.add(a);am=files[a][1];a=am.get('parent_thread_id',am.get('forked_from_id'))
 inherited=set().union(*(messages.get(a,set()) for a in ancestors)); rows=[]; counts=collections.Counter(); settings=[]; digest=hashlib.sha256();size=0; dedup=0
 for n,line in enumerate(open(p,'rb'),1):
  e=json.loads(line);q=e.get('payload',{});ts=e.get('timestamp','')
  if ts>CUTOFF:continue
  digest.update(line);size+=len(line);counts[(e['type'],q.get('type',''))]+=1
  if e['type']=='turn_context':settings.append({'line':n,'timestamp':ts,**{k:q[k] for k in ['model','effort','approval_policy','sandbox_policy'] if k in q}})
  if q.get('type')=='message':
   key=json.dumps([q.get('role'),q.get('content')],sort_keys=True)
   if key in inherited:dedup+=1;continue
   text=' '.join(c.get('text','') for c in q.get('content',[]) if isinstance(c,dict))
   if '<codex_internal_context' in text or '<environment_context' in text:continue
   rows.append(f"L{n} {ts} {q.get('role')} {q.get('phase','')}\n{text[:1800]}\n")
  if q.get('type')=='agent_message':
   encrypted=any(c.get('type')=='encrypted_content' for c in q.get('content',[]))
   rows.append(f"L{n} {ts} INTERNAL {q.get('author')} -> {q.get('recipient')} encrypted={encrypted}\n")
 (OUT/(i+'-index.txt')).write_text('\n'.join(rows))
 inventory.append({'id':i,'path':p,'parent':parent,'agent_path':m.get('agent_path'),'start':m['timestamp'],'cutoff':CUTOFF,'included_bytes':size,'included_sha256':digest.hexdigest(),'inherited_messages_removed':dedup,'settings':settings,'counts':{str(k):v for k,v in counts.items()}})
(OUT/'corpus.json').write_text(json.dumps(inventory,indent=2))
print('Sessions',len(inventory),'included bytes',sum(z['included_bytes'] for z in inventory),'inherited messages removed',sum(z['inherited_messages_removed'] for z in inventory))
