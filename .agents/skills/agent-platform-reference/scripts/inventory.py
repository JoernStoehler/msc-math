#!/usr/bin/env python3
"""Generate measured inventory, duplicates, fidelity and source gaps."""
import collections,hashlib,json
from pathlib import Path
from refresh import ROOT,load_manifest,utc_now

def inventory(root=ROOT):
 rows=load_manifest(root/'corpus/manifest.jsonl');groups=collections.defaultdict(list)
 for r in rows:groups[r['product']].append(r)
 files=[p for p in (root/'corpus/articles').rglob('*.md')]
 hashes=collections.defaultdict(list)
 for r in rows:
  if r.get('sha256'):hashes[r['sha256']].append(r['id'])
 return {'generated_utc':utc_now(),'sources':len(rows),'available_verified':sum(r['status']=='ok' for r in rows),'mirror_bytes':sum(p.stat().st_size for p in files),'raw_snapshot_bytes':sum(p.stat().st_size for p in (root/'corpus/snapshots').glob('*') if p.is_file()),'mirror_files':len(files),'formats':dict(collections.Counter(r.get('actual_format','unfetched') for r in rows)),'products':{k:{'sources':len(v),'ok':sum(r['status']=='ok' for r in v),'bytes':sum(r.get('bytes') or 0 for r in v)} for k,v in sorted(groups.items())},'duplicate_content_groups':[ids for ids in hashes.values() if len(ids)>1],'gaps':[{'id':r['id'],'url':r['url'],'error':r.get('error')} for r in rows if r['status']!='ok'],'refresh_failures':[{'id':r['id'],'error':r['error']} for r in rows if r.get('error')],'fidelity':{'native_markdown':'No content rewrite; CRLF and outer whitespace normalized. Code, tables, links retained.','html':'Article/main extraction using BeautifulSoup and markdownify. Representative tests preserve headings, links, tables and code; not a claim of universal lossless HTML conversion.'},'versioning':'URL/resolved URL, retrieval time, ETag/Last-Modified, normalized SHA256, immutable raw snapshots and per-content metadata history. Conditional HTTP refresh. Vendor code pinned separately by tag/commit.'}
if __name__=='__main__':
 result=inventory();(ROOT/'evidence/inventory.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({k:result[k] for k in ['sources','available_verified','mirror_bytes','raw_snapshot_bytes','mirror_files','formats']},indent=2))
