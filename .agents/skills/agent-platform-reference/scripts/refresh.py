#!/usr/bin/env python3
"""Incrementally retrieve official agent-platform documentation into greppable Markdown.

No AI rewrite; auto-generated vendor mirrors and metadata are separate from corrections.
Only allowlisted HTTPS hostnames can be fetched. Python deps: requests, bs4, markdownify.
"""
import argparse
import concurrent.futures
import hashlib
import json
import os
from pathlib import Path
import re
import sys
import tempfile
import time
from contextlib import contextmanager
from datetime import datetime, timezone
from urllib.parse import urlparse
from bs4 import BeautifulSoup
from markdownify import markdownify as md
import requests

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / 'corpus/manifest.jsonl'
HOSTS={'help.openai.com','support.claude.com','support.anthropic.com','developers.openai.com','learn.chatgpt.com','code.claude.com','platform.claude.com','docs.anthropic.com','agentskills.io','raw.githubusercontent.com','openai.github.io','api.github.com','claude.com'}
MIN_CHARS=100
MAX_DOWNLOAD=12_000_000
HEADERS={'User-Agent':'Mozilla/5.0', 'Accept':'text/html,text/markdown,*/*'}

@contextmanager
def maintenance_lock():
    """Fail early rather than lose manifest updates to concurrent maintainers (POSIX)."""
    import fcntl
    with (ROOT/'.refresh.lock').open('a') as lock:
        try: fcntl.flock(lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
        except BlockingIOError: raise RuntimeError('another maintenance process is running')
        try: yield
        finally: fcntl.flock(lock,fcntl.LOCK_UN)


def validate_url(url):
    p=urlparse(url)
    if p.scheme!='https' or p.hostname not in HOSTS or p.username or p.password or p.port:
        raise ValueError(f'URL not allowlisted: {url}')


def utc_now():
    return datetime.now(timezone.utc).isoformat(timespec='seconds')


def load_manifest(path=MANIFEST):
    result=[]; seen=set()
    for n,line in enumerate(path.read_text(encoding='utf-8').splitlines(),1):
        if not line.strip(): continue
        row=json.loads(line)
        if row['id'] in seen: raise ValueError(f'duplicate source ID: {row["id"]}')
        seen.add(row['id']); validate_url(row['url'])
        if not re.fullmatch(r'[a-z0-9-]+',row['id']): raise ValueError(f'invalid ID line {n}')
        if not row['path'].startswith('corpus/articles/') or '..' in Path(row['path']).parts or not row['path'].endswith('.md'):
            raise ValueError(f'unsafe path: {row["path"]}')
        result.append(row)
    return result


def save_manifest(records,path=MANIFEST):
    data=''.join(json.dumps(row,sort_keys=True,ensure_ascii=False)+'\n' for row in sorted(records,key=lambda x:x['id']))
    atomic_write(path,data)


def atomic_write(path,data):
    path.parent.mkdir(parents=True,exist_ok=True)
    fd,tmp=tempfile.mkstemp(prefix='.refresh-',dir=path.parent)
    try:
        with os.fdopen(fd,'w',encoding='utf-8') as f:
            f.write(data); f.flush(); os.fsync(f.fileno())
        os.replace(tmp,path)
    finally:
        if os.path.exists(tmp): os.unlink(tmp)


def to_markdown(raw,format,url):
    if format in ('markdown','plain'):
        result=raw.replace('\r\n','\n').strip()+'\n'
    elif format=='html':
        soup=BeautifulSoup(raw,'html.parser')
        for x in soup(['script','style','noscript','nav','footer','aside','form','svg']): x.decompose()
        container=soup.find(class_='article__body') or soup.find(id='content-area') or soup.find('article') or soup.find('main') or soup.find(attrs={'role':'main'}) or soup.body
        if container is None: raise ValueError('HTML has no article/main/body')
        for a in container.find_all(['a','img']):
            from urllib.parse import urljoin
            key='href' if a.name=='a' else 'src'
            if a.get(key): a[key]=urljoin(url,a[key])
        for pre in container.find_all('pre'):
            code=pre.find('code')
            if code:
                for br in code.find_all('br'): br.replace_with('\n')
        result=md(str(container),heading_style='ATX',strip=['iframe']).strip()+'\n'
        title=soup.find('h1')
        if title and not result.lstrip().startswith('# '): result='# '+title.get_text(' ',strip=True)+'\n\n'+result
    else: raise ValueError('unknown source format: '+format)
    if len(result.strip())<MIN_CHARS: raise ValueError(f'too-short extracted content: {len(result)} chars')
    low=result[:700].lower()
    if any(x in low for x in ('just a moment...','checking your browser','enable javascript to continue','sign in to continue','access denied','page not found','404 not found')):
        raise ValueError('likely bot challenge')
    return result


def get_text(url,headers=None,session=None):
    validate_url(url)
    requester=session or requests
    from urllib.parse import urljoin
    for _ in range(8):
        validate_url(url)
        resp=requester.get(url,headers=headers or HEADERS,timeout=(25,45),allow_redirects=False,stream=True)
        validate_url(resp.url)
        if resp.status_code not in (301,302,303,307,308): break
        next_url=urljoin(url,resp.headers['Location']); validate_url(next_url)
        resp.close(); url=next_url
    else: raise ValueError('too many redirects')
    if resp.status_code==304: return resp,None
    resp.raise_for_status()
    chunks=[]; size=0
    for c in resp.iter_content(chunk_size=65536):
        size+=len(c)
        if size>MAX_DOWNLOAD: raise ValueError(f'source exceeds {MAX_DOWNLOAD} bytes')
        chunks.append(c)
    data=b''.join(chunks)
    resp.raw_bytes=data
    return resp,data.decode('utf-8',errors='strict')


def fetch_one(rec,session=None):
    rec=dict(rec)
    target=ROOT/rec['path']
    headers=dict(HEADERS)
    if target.exists():
        if rec.get('etag'): headers['If-None-Match']=rec['etag']
        if rec.get('last_modified'): headers['If-Modified-Since']=rec['last_modified']
    try:
        resp,raw=get_text(rec['url'],headers=headers,session=session)
        rec['last_checked_utc']=utc_now()
        if resp.status_code==304:
            if not target.exists(): raise ValueError('304 without local copy')
            if hashlib.sha256(target.read_bytes()).hexdigest()!=rec.get('sha256'): raise ValueError('304 with corrupt local copy')
            rec['status']='ok'; rec.pop('error',None); rec.pop('refresh_status',None)
            return rec,'unchanged-304'
        final_url=resp.url
        format=rec.get('format','html')
        ct=resp.headers.get('Content-Type','').lower()
        if ct.startswith('text/markdown'): format='markdown'
        elif ct.startswith('text/plain') and format=='html': format='plain'
        if ct.startswith('text/html'): format='html'
        if ct and not any(ct.startswith(t) for t in ('text/html','text/plain','text/markdown','application/octet-stream')):
            raise ValueError('unexpected content type: '+ct)
        content=to_markdown(raw,format,final_url)
        if rec.get('required_terms') and not all(t.lower() in content.lower() for t in rec['required_terms']):
            raise ValueError('missing expected article identity or required terms')
        raw_bytes=getattr(resp,'raw_bytes',raw.encode('utf-8'))
        raw_hash=hashlib.sha256(raw_bytes).hexdigest()
        snapshot=ROOT/'corpus/snapshots'/raw_hash
        snapshot.parent.mkdir(parents=True,exist_ok=True)
        if not snapshot.exists(): snapshot.write_bytes(raw_bytes)
        digest=hashlib.sha256(content.encode('utf-8')).hexdigest()
        changed=digest!=rec.get('sha256') or not target.exists()
        if changed: atomic_write(target,content)
        rec.update(status='ok',last_success_utc=utc_now(),etag=resp.headers.get('ETag'),last_modified=resp.headers.get('Last-Modified'),sha256=digest,bytes=len(content.encode('utf-8')),resolved_url=final_url,raw_sha256=raw_hash,raw_path=str(snapshot.relative_to(ROOT)),raw_bytes=len(raw_bytes),content_type=ct,actual_format=format,converter='native' if format!='html' else 'markdownify-1.2.3',conversion_errors=[])
        rec.pop('error',None); rec.pop('refresh_status',None)
        history=ROOT/'corpus/history'/rec['id']/f'{raw_hash}.json'
        if not history.exists(): atomic_write(history,json.dumps(rec,indent=2)+'\n')
        return rec,'changed' if changed else 'unchanged'
    except Exception as e:
        rec['last_checked_utc']=utc_now(); rec['error']=f'{type(e).__name__}: {str(e)[:250]}'
        # A failed upstream check does not invalidate a previously verified local copy.
        valid=target.exists() and hashlib.sha256(target.read_bytes()).hexdigest()==rec.get('sha256')
        rec['status']='ok' if valid else 'error'
        rec['refresh_status']='error'
        return rec,'error'


def discover_claude(records,max_new):
    url='https://code.claude.com/docs/llms.txt'
    _,text=get_text(url)
    urls=re.findall(r'https://code\.claude\.com/docs/en/[\w./-]+\.md',text)
    existing_urls={r['url'] for r in records}
    added=[]
    for url in dict.fromkeys(urls):
        if url in existing_urls: continue
        slug=url.rsplit('/',1)[-1][:-3]
        id='claude-code-'+re.sub('[^a-z0-9-]+','-',slug.lower()).strip('-')
        if id in {r['id'] for r in records}: continue
        rec=dict(id=id,vendor='anthropic',product='claude-code',url=url,format='markdown',priority=3,path=f'corpus/articles/anthropic/claude-code/{id}.md',status='pending',last_checked_utc=None,last_success_utc=None,etag=None,last_modified=None,sha256=None,bytes=None)
        records.append(rec); added.append(id)
        if len(added)>=max_new: break
    return added


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    sub=parser.add_subparsers(dest='cmd',required=True)
    sub.add_parser('list')
    d=sub.add_parser('discover-claude'); d.add_argument('--max-new',type=int,default=500)
    p=sub.add_parser('sync'); p.add_argument('--jobs',type=int,default=6); p.add_argument('--only',nargs='*'); p.add_argument('--priority',type=int,default=3); p.add_argument('--max',type=int,default=10000)
    args=parser.parse_args()
    records=load_manifest()
    if args.cmd=='list':
        for r in sorted(records,key=lambda x:(x['priority'],x['id'])): print(f"{r['status']:8} p{r['priority']} {r['id']:34} {r['url']}")
        print(f'Total: {len(records)}; ok: {sum(r["status"]=="ok" for r in records)}')
    elif args.cmd=='discover-claude':
        if args.max_new<1: parser.error('--max-new must be >=1')
        added=discover_claude(records,args.max_new); save_manifest(records)
        print(f'Added {len(added)} new sources; total {len(records)}')
    elif args.cmd=='sync':
        if args.jobs<1 or args.jobs>16: parser.error('jobs must be 1..16')
        wanted=set(args.only) if args.only else None
        selected=[r for r in records if r['priority']<=args.priority and (wanted is None or r['id'] in wanted)][:args.max]
        if wanted and set(r['id'] for r in selected)!=wanted:
            parser.error(f'unknown or excluded IDs: {wanted-set(r["id"] for r in selected)}')
        done={}
        with concurrent.futures.ThreadPoolExecutor(max_workers=args.jobs) as pool:
            futures={pool.submit(fetch_one,r):r['id'] for r in selected}
            for f in concurrent.futures.as_completed(futures):
                rec,result=f.result(); done[rec['id']]=rec
                # Persist each completion so interrupted bulk runs are resumable.
                save_manifest([done.get(r['id'],r) for r in records])
                print(f'{result:14} {rec["id"]}' + (f'  {rec.get("error", "")}' if result=='error' else ''),flush=True)
        save_manifest([done.get(r['id'],r) for r in records])
        print(f'Total {len(selected)}; errors {sum(x.get("error") is not None for x in done.values())}')
        if any(x.get('error') for x in done.values()): sys.exit(2)

if __name__=='__main__':
    with maintenance_lock(): main()
