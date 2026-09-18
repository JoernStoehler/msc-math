"""Inventory recovered cache and rational-body correspondence; no evaluator calls."""
import collections,hashlib,json,statistics,sys,time
from fractions import Fraction
from pathlib import Path
p=Path(sys.argv[1]); out=Path(sys.argv[2]); start=time.monotonic()
def key(r):return tuple(tuple(str(Fraction(x)) for x in v) for v in r['dual_vertices_rational'])
cache={}; fields=collections.Counter(); duplicates=0
for line in (p/'shared-cache.jsonl').open():
 r=json.loads(line); fields.update(r.keys()); k=key(r); duplicates+=k in cache; cache[k]=r
stats=collections.defaultdict(list); mismatches=collections.Counter(); source_counts=collections.Counter(); fields_source=collections.Counter(); n=0
for name in ['random.jsonl','random-product.jsonl']:
 for line in (p/name).open():
  r=json.loads(line); n+=1; fields_source.update(r.keys()); k=key(r); c=cache.get(k)
  bucket=f"product-{r['k']}x{r['m']}" if 'k' in r else f"generic-{r['facet_count']}"
  stats[bucket].append(r['time_capacity_ms'])
  for vf,vr in zip(r['dual_vertices'],r['dual_vertices_rational']):
   for f,q in zip(vf,vr):
    if Fraction(f)!=Fraction(q):mismatches['rational_dual_not_exact_binary64']+=1
  if c is None:mismatches['source_not_in_cache']+=1;continue
  source_counts[str((c.get('source') or {}).get('family'))]+=1
  for f in ['capacity','volume']:
   if r[f]!=c[f]:mismatches['cache_'+f+'_differs']+=1
summary=dict(rows=n,cache_rows=len(cache),cache_duplicate_keys=duplicates,cache_field_counts=dict(fields),source_field_counts=dict(fields_source),cache_sources=dict(source_counts),mismatch_counts=dict(mismatches),historical_capacity_timings_ms={k:dict(n=len(v),median=statistics.median(v),maximum=max(v),sum=sum(v)) for k,v in sorted(stats.items())},cache_sha256=hashlib.sha256((p/'shared-cache.jsonl').read_bytes()).hexdigest(),wall_seconds=time.monotonic()-start)
out.write_text(json.dumps(summary,indent=2,sort_keys=True)+'\n');print(json.dumps(summary,indent=2))
