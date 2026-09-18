"""Retain exact full-run bytes in deterministic compressed form."""
import gzip,hashlib,json
from pathlib import Path
root=Path(__file__).resolve().parent; manifest={}
for name in ['inputs.jsonl','results.jsonl']:
 raw=(root/name).read_bytes(); packed=gzip.compress(raw,mtime=0)
 (root/(name+'.gz')).write_bytes(packed)
 manifest[name]=dict(raw_bytes=len(raw),raw_sha256=hashlib.sha256(raw).hexdigest(),gzip_bytes=len(packed),gzip_sha256=hashlib.sha256(packed).hexdigest())
(root/'payload-manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
