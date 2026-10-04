#!/usr/bin/env python3
"""Bounded, logged, read-only source delivery for a frozen archival test."""
from pathlib import Path
import argparse, datetime, fcntl, hashlib, json, re, sys, time
BASE=Path(__file__).resolve().parent
MAX_SOURCE_WORDS=25000
MAX_READ_WORDS=1800
MAX_SECONDS=480
p=argparse.ArgumentParser()
p.add_argument('reader')
sub=p.add_subparsers(dest='action',required=True)
sub.add_parser('start');sub.add_parser('index');sub.add_parser('status')
s=sub.add_parser('search');s.add_argument('query');s.add_argument('--file');s.add_argument('--limit',type=int,default=14)
r=sub.add_parser('read');r.add_argument('file');r.add_argument('start',type=int,nargs='?',default=1);r.add_argument('end',type=int,nargs='?');r.add_argument('--offset',type=int,default=0)
a=p.parse_args()
if not re.fullmatch(r'R\d{4}',a.reader):sys.exit('Invalid reader identifier.')
config=json.loads((BASE/'readers'/f'{a.reader}.json').read_text())
docs=config['documents']
state_dir=BASE/'receipts'/a.reader;state_dir.mkdir(parents=True,exist_ok=True)
lock=(state_dir/'lock').open('a+')
fcntl.flock(lock,fcntl.LOCK_EX)
state_path=state_dir/'state.json'
now=time.time()
state=json.loads(state_path.read_text()) if state_path.exists() else {'started_epoch':now,'source_words':0,'delivered_words':0,'calls':0}
elapsed=now-state['started_epoch'];remaining=MAX_SOURCE_WORDS-state['source_words']
body='';prefix='';limited=False
source_open=elapsed<MAX_SECONDS and remaining>0

def load_doc(key):
    if key not in docs:raise ValueError('File key is not in this reader’s allowed inventory.')
    d=docs[key];raw=Path(d['path']).read_bytes()
    if hashlib.sha256(raw).hexdigest()!=d['sha256']:raise ValueError('Frozen source hash mismatch; no source was delivered.')
    return raw.decode().splitlines()

def index():
    out=['AVAILABLE FROZEN SOURCES — file key | kind | source words | file lines']
    for key,d in docs.items():out.append(f"{key} | {d['kind']} | {d['words']} | {d['lines']}")
    return '\n'.join(out)

try:
    if a.action=='status':
        body='No source content requested.'
    elif elapsed>=MAX_SECONDS or remaining<=0:
        body='SOURCE ACCESS CLOSED: the time or source-word limit has been reached. Return your supported findings and explicitly mark missing evidence.';limited=True
    elif a.action=='start':
        prefix=config['instructions']+'\n\nCURRENT PROJECT GUIDANCE:\n'+config['guidance']+'\n\nTASK:\n'+config['task']+'\n\n'
        body=index()
    elif a.action=='index':body=index()
    elif a.action=='search':
        if len(a.query)>240:raise ValueError('Search query is too long.')
        needle=a.query.casefold()
        keys=[a.file] if a.file else list(docs)
        out=[];count=0
        for key in keys:
            lines=load_doc(key)
            for n,line in enumerate(lines,1):
                pos=line.casefold().find(needle)
                if pos>=0:
                    left=max(0,pos-180);right=min(len(line),pos+600)
                    snippet=('…' if left else '')+line[left:right]+('…' if right<len(line) else '')
                    out.append(f'{key}:{n} | {snippet}')
                    count+=1
                    if count>=max(1,min(a.limit,30)):break
            if count>=max(1,min(a.limit,30)):break
        body='\n'.join(out) if out else 'No matching source line was found in the allowed files.'
    elif a.action=='read':
        lines=load_doc(a.file)
        start=max(1,a.start);end=min(a.end or start+159,len(lines))
        out=[];used=0;cap=min(MAX_READ_WORDS,max(0,remaining-45))
        next_line=None;next_offset=0
        for n in range(start,end+1):
            words=lines[n-1].split();offset=a.offset if n==start else 0
            content=' '.join(words[offset:]) if offset else lines[n-1]
            line_text=f'{a.file}:{n} | {content}'
            charge=len(line_text.split())
            if used+charge>cap:
                if not out:
                    take=max(0,cap-3)
                    out.append(f'{a.file}:{n} | '+' '.join(words[offset:offset+take]))
                    next_line=n;next_offset=offset+take
                else:next_line=n
                limited=True;break
            out.append(line_text);used+=charge
        body='\n'.join(out)
        if next_line is not None:
            body+=f'\n[Read chunk ended. Continue with: read {a.file} {next_line} {end} --offset {next_offset}]'
        elif end<len(lines):body+=f'\n[Requested range ended; next file line is {end+1}.]'
        else:body+='\n[End of file.]'
    # Bound every source-bearing output, including search results and inventories.
    cap=min(MAX_READ_WORDS,max(0,remaining))
    if source_open and len(body.split())>cap:
        body=' '.join(body.split()[:cap]);limited=True
except (ValueError,OSError,json.JSONDecodeError) as e:
    body='READ ERROR: '+str(e);limited=True
source_words=len(body.split()) if a.action!='status' and source_open else 0
state['source_words']+=source_words
state['calls']+=1
footer=f"\n\n[Reader {a.reader}; source words delivered {state['source_words']}/{MAX_SOURCE_WORDS}; elapsed {elapsed:.1f}/{MAX_SECONDS} seconds; call {state['calls']}.]"
response=prefix+body+footer+'\n'
state['delivered_words']+=len(response.split())
state['last_epoch']=now
state_path.write_text(json.dumps(state,indent=2))
record={'reader':a.reader,'call':state['calls'],'timestamp':datetime.datetime.fromtimestamp(now,datetime.timezone.utc).isoformat(),'elapsed_seconds':elapsed,'action':a.action,'arguments':vars(a),'source_words_this_call':source_words,'source_words_total':state['source_words'],'instruction_words_this_call':len(prefix.split()),'delivered_words_this_call':len(response.split()),'delivered_words_total':state['delivered_words'],'limited_or_error':limited,'response_sha256':hashlib.sha256(response.encode()).hexdigest(),'response_bytes':len(response.encode())}
(state_dir/f"call-{state['calls']:03d}.txt").write_text(response)
with (state_dir/'calls.jsonl').open('a') as f:f.write(json.dumps(record)+'\n')
print(response,end='')
