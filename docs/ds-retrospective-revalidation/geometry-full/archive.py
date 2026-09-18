#!/usr/bin/env python3
import gzip,hashlib,json,pathlib
here=pathlib.Path(__file__).resolve().parent
assert json.loads((here/'run-receipt.json').read_text())['status']=='completed'
receipt={}
for p in sorted((here/'output').glob('*.jsonl')):
 data=p.read_bytes(); packed=gzip.compress(data,mtime=0); dest=here/(p.name+'.gz');assert not dest.exists()
 dest.write_bytes(packed)
 assert gzip.decompress(packed)==data
 receipt[p.name]={'raw_bytes':len(data),'raw_sha256':hashlib.sha256(data).hexdigest(),'gzip_bytes':len(packed),'gzip_sha256':hashlib.sha256(packed).hexdigest()}
(here/'archive-receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
