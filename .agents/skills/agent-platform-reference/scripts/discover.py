#!/usr/bin/env python3
"""Inventory official indexes, add all in-scope pages, and retain exclusion evidence."""
import hashlib
import json
import re
from urllib.parse import urlparse
from refresh import ROOT, get_text, load_manifest, save_manifest, utc_now, maintenance_lock

INDEXES = [
 ('claude-code','anthropic','https://code.claude.com/docs/llms.txt'),
 ('chatgpt-work','openai','https://learn.chatgpt.com/docs/llms.txt'),
 ('openai-api','openai','https://developers.openai.com/api/docs/llms.txt'),
 ('openai-plugins','openai','https://developers.openai.com/plugins/llms.txt'),
 ('openai-agents-sdk','openai','https://openai.github.io/openai-agents-python/llms.txt'),
 ('claude-api','anthropic','https://platform.claude.com/docs/llms.txt'),
 ('agent-skills','standard','https://agentskills.io/llms.txt'),
]

def included(product,url):
    p=urlparse(url)
    if product=='claude-code':
        return p.hostname=='code.claude.com' and p.path.startswith('/docs/en/') and p.path.endswith('.md') and ('/whats-new/' not in p.path or any(w in p.path for w in ['2026-w40','2026-w41','2026-w42']))
    if product=='chatgpt-work':
        return p.hostname=='learn.chatgpt.com' and p.path.startswith('/docs/') and p.path.endswith('.md') and not p.path.endswith('codex-manual.md')
    if product=='openai-api':
        return p.hostname=='developers.openai.com' and p.path.endswith('.md') and bool(re.search(r'agents|responses|tools-|function-calling|conversation|background|context|compaction|prompt|evaluation|evals|text-generation|structured-outputs|moderation|mcp|safety|latest-model',p.path))
    if product=='openai-plugins':
        return p.hostname=='developers.openai.com' and '/plugins/' in p.path and p.path.endswith('.md')
    if product=='openai-agents-sdk':
        return p.hostname=='openai.github.io' and '/openai-agents-python/' in p.path and '/ref/' not in p.path
    if product=='claude-api':
        return p.hostname=='platform.claude.com' and p.path.endswith('.md') and bool(re.search(r'/agent-skills/|/tool-use/|/build-with-claude/(context|compaction|prompt|mid-conversation|skills)|/release-notes/overview',p.path))
    return p.hostname=='agentskills.io' and p.path.endswith('.md')

def main():
    records=load_manifest(); existing={r['url'] for r in records}; ids={r['id'] for r in records}; audit=[]
    for product,vendor,index in INDEXES:
        resp,raw=get_text(index)
        digest=hashlib.sha256(raw.encode()).hexdigest()
        path=ROOT/'corpus/indexes'/f'{product}.txt';path.parent.mkdir(parents=True,exist_ok=True);path.write_text(raw)
        urls=list(dict.fromkeys(re.findall(r'https://[^\s)<>]+',raw)))
        entries=[]
        for url in urls:
            keep=included(product,url)
            entries.append({'url':url,'selected':keep,'reason':'in scope' if keep else 'non-English, API endpoint reference, historic weekly note, combined duplicate, or unrelated product'})
            if not keep or url in existing:continue
            slug=urlparse(url).path.strip('/').replace('/','-').removesuffix('.md')
            id=product+'-'+re.sub('[^a-z0-9-]','-',slug.lower())
            if id in ids:continue
            ids.add(id);existing.add(url)
            records.append(dict(id=id,vendor=vendor,product=product,url=url,format='markdown' if url.endswith('.md') else 'html',priority=3,path=f'corpus/articles/{vendor}/{product}/{id}.md',status='pending',scope='English product documentation; snapshot, not runtime assertion',discovered_from=index))
        audit.append(dict(index=index,resolved_url=resp.url,retrieved_utc=utc_now(),bytes=len(raw.encode()),sha256=digest,candidates=len(urls),selected=sum(e['selected'] for e in entries),entries=entries))
        print(product,len(urls),'candidates;',sum(e['selected'] for e in entries),'selected',flush=True)
    save_manifest(records)
    (ROOT/'corpus/discovery.json').write_text(json.dumps(audit,indent=2))
    print('Manifest:',len(records),'sources')

if __name__=='__main__':
    with maintenance_lock(): main()
