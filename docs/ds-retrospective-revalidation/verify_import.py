# /// script
# dependencies = ["blake3"]
# ///
"""Read-only legacy import verification; never claims current capacity execution."""
import argparse
import collections
import datetime
import hashlib
import json
import math
from pathlib import Path
import struct
import time
import blake3

EXPECTED = {
 'polytope-table.jsonl': '607c8731fa03d190d497edc3e8f1b4cca88f7d238260cce527680f568bc33d59',
 'polytope-provenance-table.jsonl': '6ff88a5accce9a7ec7e5a494107350b0974b2ce0268ea44caae36a18a7494ef2',
 'random.jsonl': 'a21ac62ba5c9496ef631d3cce74e8b663764516b76e9d4725f1e517d8dd55f9f',
 'random-product.jsonl': '66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736',
}
def read(path):
    raw = path.read_bytes()
    digest = hashlib.sha256(raw).hexdigest()
    if digest != EXPECTED[path.name]:
        raise ValueError(f'wrong immutable payload: {path.name}')
    return [json.loads(line) for line in raw.splitlines() if line.strip()], digest

def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--table', type=Path, required=True)
    p.add_argument('--provenance', type=Path, required=True)
    p.add_argument('--source-dir', type=Path)
    p.add_argument('--output', type=Path, required=True)
    a = p.parse_args()
    start = time.monotonic()
    table, th = read(a.table)
    provenance, ph = read(a.provenance)
    byid = {r['poly_id']: r for r in table}
    prov = {(r['dataset'], r['source_name']): r for r in provenance}
    assert len(table) == len(byid) == len(provenance) == len(prov) == 14336
    assert set(byid) == {r['poly_id'] for r in provenance}
    assert all(math.isfinite(r['sys']) and r['sys'] > 0 for r in table)
    for r in provenance:
        assert byid[r['poly_id']]['capacity_source'] == r['dataset']
    result = dict(schema='legacy-retrospective-verification-v1',
        verified_at_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
        verifier_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        files={a.table.name: th, a.provenance.name: ph}, rows=len(table),
        bucket_counts=dict(collections.Counter(r.get('path', r['dataset']) for r in provenance)),
        verified=['immutable table/provenance hashes', 'complete unique table/provenance join', 'finite positive stored sys', 'dataset labels agree'],
        current_capacity_execution=False, capacity_certified=False, volume_certified=False,
        historical_execution_revision=None, legacy_source_geometry_verified=False)
    if a.source_dir:
        matched=set()
        times=[]
        max_formula_error=0
        for filename,dataset in [('random.jsonl','random_sample'),('random-product.jsonl','random_product_sample')]:
            rows,digest=read(a.source_dir/filename)
            result['files'][filename]=digest
            for row in rows:
                key=(dataset,row['name'])
                assert key in prov and key not in matched
                matched.add(key)
                coords=[x for v in row['dual_vertices'] for x in v]
                assert len(coords)==4*len(row['dual_vertices']) and all(math.isfinite(x) for x in coords)
                # Mirrors sys-landscape/datascience_cache.rs, preserving vertex order.
                identity=blake3.blake3(b''.join(struct.pack('<d',0.0 if x==0 else x) for x in coords)).hexdigest()
                assert identity==prov[key]['poly_id']
                assert row['sys']==byid[identity]['sys']
                assert row['volume']>0 and row['capacity']>0
                max_formula_error=max(max_formula_error,abs(row['capacity']**2/(2*row['volume'])-row['sys']))
                times.append(row.get('time_capacity_ms'))
        assert matched==set(prov)
        result.update(legacy_source_geometry_verified=True, max_stored_sys_formula_absolute_error=max_formula_error,
                      historical_capacity_timing_field_present=sum(t is not None for t in times))
        result['verified'] += ['source payload hashes', 'all source names map uniquely', 'binary64 geometry hashes match poly_id', 'stored source sys equals table sys']
    result['verification_wall_seconds']=time.monotonic()-start
    with a.output.open('x') as f:
        json.dump(result,f,indent=2,sort_keys=True); f.write('\n')
    print(json.dumps(result,indent=2))
if __name__=='__main__': main()
