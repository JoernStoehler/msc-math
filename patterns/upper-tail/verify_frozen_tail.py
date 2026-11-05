#!/usr/bin/env python3
"""Independent all-row feasible-word/area verification; no word optimization.
Uses the earlier verifier's all-pairs intersection/hull geometry, not discovery.
"""
from fractions import Fraction as F
from pathlib import Path
from collections import Counter
import argparse,gzip,hashlib,json,sys,time
sys.path.insert(0,str(Path(__file__).resolve().parent))
from verify_triangle_certificate import area_from_all_intersections

def main():
    ap=argparse.ArgumentParser();ap.add_argument('--source',type=Path,required=True);ap.add_argument('--rows',type=Path,required=True);ap.add_argument('--summary',type=Path,required=True);ap.add_argument('--prior',type=Path,required=True);ap.add_argument('--out',type=Path,required=True);a=ap.parse_args();start=time.perf_counter()
    summary=json.loads(a.summary.read_text());assert hashlib.sha256(a.source.read_bytes()).hexdigest()==summary['source_sha256']
    with gzip.open(a.rows,'rt') as f: rows=[json.loads(line) for line in f]
    certs={r['name']:r for r in rows};assert len(certs)==len(rows)==3072
    prior=json.loads(a.prior.read_text());prior={r['name']:r for r in prior['rows']};verified=0;prior_matches=0;counts=Counter();max_disagreement=0
    for line in a.source.open():
        r=json.loads(line)
        if r['name'] not in certs:continue
        row=certs.pop(r['name']);aa=[[F(v) for v in u] for u in r['dual_vertices_rational']];p=[u[:2] for u in aa[:r['k']]];q=[u[2:] for u in aa[r['k']:]]
        vol=area_from_all_intersections(p)*area_from_all_intersections(q);assert vol==F(row['exact_original_area_product'])
        i,j=row['selected_support_indices'];assert len(set(i))==len(set(j))==3 and all(0<=x<r['k'] for x in i) and all(0<=x<r['m'] for x in j)
        normals=[aa[x] for x in i]+[aa[r['k']+x] for x in j];c=row['certificate'];weights=list(map(F,c['weights']));word=c['word'];assert sorted(word)==list(range(6)) and len(weights)==6 and min(weights)>0
        assert sum(weights[:3])==sum(weights[3:])==F(1,2)
        assert all(sum(w*u[k] for w,u in zip(weights,normals))==0 for k in range(4))
        def omega(x,y):return x[0]*y[2]+x[1]*y[3]-x[2]*y[0]-x[3]*y[1]
        Q=sum(weights[x]*weights[y]*omega(normals[x],normals[y]) for pos,x in enumerate(word) for y in word[pos+1:]);assert Q==F(c['exact_Q']) and Q>0
        ratio=1/(8*Q*Q*vol);assert ratio==F(row['exact_triangle_ratio']);assert float(1/(2*Q))==row['triangle_capacity_upper_bound']
        assert row['U']==min(row['triangle_capacity_upper_bound'],row['width_capacity_upper_bound'])
        assert row['historical_capacity']==r['capacity'] and row['historical_sys']==r['sys'];assert row['U_over_historical_capacity']==row['U']/r['capacity']
        assert row['bucket']==f"{r['k']}x{r['m']}";counts[row['bucket']]+=1
        if r['name'] in prior:
            old=prior.pop(r['name']);assert old['exact_triangle_ratio']==row['exact_triangle_ratio'];assert old['selected_support_indices']==row['selected_support_indices'];assert old['certificate']==row['certificate'];prior_matches+=1
        verified+=1
    assert not certs and not prior and verified==3072 and prior_matches==26 and set(counts.values())=={1024}
    for bucket,v in summary['buckets'].items():
        rr=sorted([r for r in rows if r['bucket']==bucket],key=lambda r:(-r['historical_sys'],r['name']))
        for label,n in [('all',1024),('top10%',103),('top5%',52),('top2%',21)]:
            sel=rr[:n];s=v[label];assert s['n']==n
            assert s['U_over_historical_capacity']['max']==max(r['U_over_historical_capacity'] for r in sel)
            assert s['U_over_historical_capacity']['min']==min(r['U_over_historical_capacity'] for r in sel)
            for pct in [1,5,10]:assert s['U_gap_within'][f'{pct}%']==sum(r['U_over_historical_capacity']<=1+pct/100+1e-10 for r in sel)
    result=dict(status='PASS',rational_witnesses=verified,buckets=dict(counts),independent_geometry='All feasible pairwise rational supporting-line intersections and rational monotone hull; no optimization',prior26_exact_witness_matches=prior_matches,tail_selection_checks='All/top10%103/top5%52/top2%21 per bucket; checked membership,extrema and gap counts',elapsed_seconds=time.perf_counter()-start,limits='Feasible exact triangle upper bounds only; no original-body optimality, historical label certification, numerical-width certification, or independent selector optimality proof')
    a.out.write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
if __name__=='__main__':main()
