#!/usr/bin/env python3
"""Exact endpoint full-root and closed equality census; writes equality.json."""
from sturm import *

def main():
    start=time.perf_counter();opened=json.loads((ROOT/'open-proof.json').read_text());closed=json.loads((ROOT/'closed-proof.json').read_text())
    assert not opened['failed_segments'] and not closed['failures']
    ids=sorted({c['query'] for r in opened['segments'] for c in r['endpoint_checks'] if 'query' in c})
    queries=[];interior_roots=[];zero_polys=[]
    for i in ids:
        rec=opened['queries'][i];p=decode(rec['polynomial']);left,right=engine.decode_u(rec['left']),engine.decode_u(rec['right'])
        if all(v==0 for v in p):zero_polys.append(i);continue
        count=count_open(p,left,right);queries.append(dict(query=i,full_root_query=count))
        if count['count']:interior_roots.append(dict(query=i,count=count['count']))
    flats=[i for i,r in enumerate(closed['checks']) if r['identically_zero_gap']]
    bodies={};parameters=set()
    for rec in closed['equality']:
        u=engine.decode_u(rec['u']);x=engine.decode_value(rec['x'])
        assert isinstance(u,K) and isinstance(x,K),'An additional algebraic equality requires explicit classification'
        h=[engine.decode_value(v) for v in rec['supports']]
        assert all(isinstance(v,K) for v in h)
        key=(u,x,tuple(h));bodies.setdefault(key,[]).append(rec['owner']);parameters.add((u,x))
    out=dict(status='Complete finite-candidate equality census' if not interior_roots and not zero_polys and not flats else 'Named equality predicates remain',
        open_proof_sha256=hashlib.sha256((ROOT/'open-proof.json').read_bytes()).hexdigest(),
        closed_proof_sha256=hashlib.sha256((ROOT/'closed-proof.json').read_bytes()).hexdigest(),
        endpoint_query_count=len(ids),full_root_checks=queries,interior_endpoint_zero_queries=interior_roots,
        identically_zero_open_endpoint_polynomials=zero_polys,flat_zero_closed_fibers=flats,
        parameter_classes=[dict(u=u.data(),x=x.data()) for u,x in parameters],
        equality_bodies=[dict(u=u.data(),x=x.data(),supports=[v.data() for v in h],owners=owners) for (u,x,h),owners in bodies.items()],
        elapsed_seconds=round(time.perf_counter()-start,3))
    (ROOT/'equality.json').write_text(json.dumps(out,indent=2)+'\n')
    print(json.dumps({k:out[k] for k in ['status','endpoint_query_count','interior_endpoint_zero_queries','identically_zero_open_endpoint_polynomials','flat_zero_closed_fibers','parameter_classes','elapsed_seconds']}));print('distinct_bodies',len(bodies))

if __name__=='__main__':main()
