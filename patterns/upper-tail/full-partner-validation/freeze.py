import csv,gzip,hashlib,json
from pathlib import Path
root=Path('/workspaces/msc-math-worktrees/patterns-2026-10-17');out=Path(__file__).parent
width=root/'patterns/product-shape/features.csv.gz';excluded=[r['name'] for r in json.loads((root/'patterns/upper-tail/full-partner-containment/results.json').read_text())['rows']]
with gzip.open(width,'rt') as f:rows=list(csv.DictReader(f))
sample=[]
for k,m in [(5,5),(5,6),(6,6)]:
    family=sorted([r for r in rows if (int(r['k']),int(r['m']))==(k,m)],key=lambda r:(-float(r['sys']),r['name']))
    tail=[r for r in family[:21] if r['name'] not in excluded][:9]
    assert len(tail)==9
    bulk=sorted([r for r in family[103:] if r['name'] not in excluded],key=lambda r:hashlib.sha256(r['name'].encode()).hexdigest())[:3]
    for role,chosen in [('principal_top2',tail),('hash_bulk_outside_top10',bulk)]:
        sample.extend(dict(name=r['name'],bucket=f'{k}x{m}',role=role,family_rank=1+family.index(r)) for r in chosen)
assert len(sample)==len(set(r['name'] for r in sample))==36
sources={str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in [width,root/'patterns/upper-tail/full-partner-containment/analyze.py',root/'patterns/upper-tail/signed-prefix-response/analyze.py',root/'patterns/upper-tail/geometric-selector/analyze.py',root/'patterns/upper-tail/triangle_envelope.py',root/'patterns/upper-tail/tail-rows.jsonl.gz']}
p=dict(sample=sample,excluded20=excluded,source_sha256='66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736',sources=sources)
(out/'sample.json').write_text(json.dumps(p,indent=2)+'\n')
print(json.dumps(p,indent=2))
