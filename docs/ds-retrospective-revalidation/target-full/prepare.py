import hashlib,json
from pathlib import Path
root=Path(__file__).resolve().parent; repo=root.parents[2]
src=Path('/home/joern/.cache/msc-math/artifacts/polytope-datasets/f7bc6be841e9d30741d5bf7ec8d4f0c0c74ec22745d3325b792e64cec333ca96/files')
expected={'random.jsonl':'a21ac62ba5c9496ef631d3cce74e8b663764516b76e9d4725f1e517d8dd55f9f','random-product.jsonl':'66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736'}
prov={(r['dataset'],r['source_name']):r['poly_id'] for r in map(json.loads,(repo/'experiments/polytope-invariant-table/polytope-provenance-table.jsonl').open())}
rows=[]
for f,ds in [('random.jsonl','random_sample'),('random-product.jsonl','random_product_sample')]:
 assert hashlib.sha256((src/f).read_bytes()).hexdigest()==expected[f]
 for r in map(json.loads,(src/f).open()):
  rows.append(dict(id=prov[ds,r['name']],bucket=f"product-{r['k']}x{r['m']}" if 'k'in r else f"generic-{r['facet_count']}",dual_vertices=r['dual_vertices'],legacy_capacity=r['capacity'],legacy_volume=r['volume'],legacy_sys=r['sys'],source_file=f,source_name=r['name']))
assert len(rows)==len({r['id'] for r in rows})==14336
rows.sort(key=lambda r:r['id'])
with (root/'inputs.jsonl').open('x') as out:
 for r in rows:out.write(json.dumps(r)+'\n')
(root/'input-manifest.json').write_text(json.dumps(dict(rows=len(rows),ordering='poly_id ascending; all rows',sha256=hashlib.sha256((root/'inputs.jsonl').read_bytes()).hexdigest(),source_sha256=expected),indent=2)+'\n')
