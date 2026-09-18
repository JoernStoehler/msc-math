import collections,hashlib,json
from pathlib import Path
root=Path(__file__).resolve().parent
source=Path('/home/joern/.cache/msc-math/artifacts/polytope-datasets/f7bc6be841e9d30741d5bf7ec8d4f0c0c74ec22745d3325b792e64cec333ca96/files')
repo=root.parents[1]
prov={(r['dataset'],r['source_name']):r['poly_id'] for r in map(json.loads,(repo/'experiments/polytope-invariant-table/polytope-provenance-table.jsonl').open())}
buckets=collections.defaultdict(list)
for file,dataset in [('random.jsonl','random_sample'),('random-product.jsonl','random_product_sample')]:
 for line in (source/file).open():
  r=json.loads(line); pid=prov[dataset,r['name']]
  bucket=f"product-{r['k']}x{r['m']}" if 'k'in r else f"generic-{r['facet_count']}"
  buckets[bucket].append((pid,file,r))
selected=[]; outs=collections.defaultdict(list)
for bucket,rows in sorted(buckets.items()):
 for pid,file,r in sorted(rows)[:2]:
  selected.append(dict(id=pid,bucket=bucket,dual_vertices=r['dual_vertices'],legacy_capacity=r['capacity'],legacy_volume=r['volume'],legacy_sys=r['sys'],source_file=file,source_name=r['name']))
  outs[file].append(r)
for file,rows in outs.items():(root/'pilot-source'/file).write_text(''.join(json.dumps(r)+'\n' for r in rows))
f=root/'pilot-inputs.jsonl';f.write_text(''.join(json.dumps(r)+'\n' for r in selected))
(root/'pilot-selection.json').write_text(json.dumps(dict(rule='Two lexicographically smallest poly_id per each of 18 facet/product buckets; no target-dependent selection',rows=len(selected),sha256=hashlib.sha256(f.read_bytes()).hexdigest()),indent=2)+'\n')
