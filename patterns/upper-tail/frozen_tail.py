#!/usr/bin/env python3
"""Retrospective full-population diagnostic of the frozen selected-triangle rule.
Standard library only; does not invoke a full original-capacity producer.
"""
from fractions import Fraction as F
from pathlib import Path
from collections import Counter
import argparse,csv,gzip,hashlib,json,math,sys,time
sys.path.insert(0,str(Path(__file__).resolve().parent))
import triangle_envelope as rule

RULE_SHA='03e0a9459cf40696b57d0f2119779d06274c9232eef02c04b28ce5a0e2323704'
BUCKETS={(5,5),(5,6),(6,6)}
ORDERS=rule.ORDERS
SIGNS=[]
for word in ORDERS:
    pos={v:i for i,v in enumerate(word)}
    SIGNS.append([1 if pos[i]<pos[j+3] else -1 for i in range(3) for j in range(3)])

def evaluate(p,q):
    """Same 120 cyclic word objectives as rule.bound, also retain a witness."""
    best=None
    for x in p:
        for y in q:
            h=[x['weights'][i]*y['weights'][j]*rule.dot(x['normals'][i],y['normals'][j])/4 for i in range(3) for j in range(3)]
            # Summing with exact rational arithmetic preserves the frozen rule.
            vals=[sum(v if sign>0 else -v for v,sign in zip(h,signs)) for signs in SIGNS]
            idx=max(range(len(vals)),key=vals.__getitem__);Q=vals[idx]
            if best is None or Q>best[0]:best=(Q,x,y,ORDERS[idx])
    Q,x,y,word=best;assert Q>0
    return 1/(2*Q),x,y,dict(word=word,weights=[str(v/2) for v in x['weights']+y['weights']],exact_Q=str(Q))

def quantiles(xs):
    xs=sorted(xs)
    def quantile(q):
        pos=(len(xs)-1)*q;lo=math.floor(pos);hi=math.ceil(pos)
        return xs[lo]*(hi-pos)+xs[hi]*(pos-lo) if hi!=lo else xs[lo]
    return dict(zip(['min','q10','median','q90','q95','max'],[quantile(q) for q in [0,.1,.5,.9,.95,1]]))

def aggregate(rows):
    return dict(n=len(rows),historical_sys=quantiles([r['historical_sys'] for r in rows]),
        U_over_historical_capacity=quantiles([r['U_over_historical_capacity'] for r in rows]),
        width_over_historical_capacity=quantiles([r['width_over_historical_capacity'] for r in rows]),
        triangle_over_historical_capacity=quantiles([r['triangle_over_historical_capacity'] for r in rows]),
        U_gap_within={f'{pct}%':sum(r['U_over_historical_capacity']<=1+pct/100+1e-10 for r in rows) for pct in [1,5,10]},
        triangle_tightens_width=sum(r['triangle_capacity_upper_bound']<r['width_capacity_upper_bound']*(1-1e-10) for r in rows),
        U_below_historical_more_than_1e_8=sum(r['U_over_historical_capacity']<1-1e-8 for r in rows),
        selected_tie_counts=dict(Counter(f"{r['selected_min_area_tie_counts'][0]}x{r['selected_min_area_tie_counts'][1]}" for r in rows)),
        worst_rows=[{k:r[k] for k in ['name','bucket','historical_sys','U_over_historical_capacity','width_over_historical_capacity','triangle_over_historical_capacity','selected_support_indices','feasible_triangle_counts','producer_bounces']} for r in sorted(rows,key=lambda r:(-r['U_over_historical_capacity'],r['name']))[:5]])

def main():
    ap=argparse.ArgumentParser();ap.add_argument('--source',type=Path,required=True);ap.add_argument('--width',type=Path,required=True);ap.add_argument('--out',type=Path,required=True);ap.add_argument('--max-seconds',type=float,default=300);a=ap.parse_args();start=time.perf_counter()
    assert hashlib.sha256(Path(rule.__file__).read_bytes()).hexdigest()==RULE_SHA
    assert hashlib.sha256(a.source.read_bytes()).hexdigest()==rule.SOURCE_SHA
    assert hashlib.sha256(a.width.read_bytes()).hexdigest()==rule.WIDTH_SHA
    with gzip.open(a.width,'rt') as f:width={r['name']:r for r in csv.DictReader(f)}
    assert len(width)==10240
    a.out.mkdir(parents=True,exist_ok=True);rows=[];seen=set()
    with gzip.open(a.out/'tail-rows.jsonl.gz','wt') as fout:
        for line in a.source.open():
            r=json.loads(line);assert r['name'] not in seen;seen.add(r['name']);w=width[r['name']]
            assert (r['k'],r['m'])==(int(w['k']),int(w['m'])) and r['sys']==float(w['sys']) and r['capacity']==float(w['capacity'])
            if (r['k'],r['m']) not in BUCKETS:continue
            assert time.perf_counter()-start<a.max_seconds,'Initial wall cap reached; partial exact rows retained'
            aa=[[F(v) for v in u] for u in r['dual_vertices_rational']];p=[u[:2] for u in aa[:r['k']]];q=[u[2:] for u in aa[r['k']:]]
            tp,np=rule.triangles(p);tq,nq=rule.triangles(q);cap,x,y,cert=evaluate(tp,tq);vol=rule.area(p)*rule.area(q);s=cap*cap/(2*vol);wid=float(w['c2']);historical=r['capacity'];U=min(float(cap),wid)
            row=dict(name=r['name'],bucket=f"{r['k']}x{r['m']}",historical_sys=r['sys'],historical_capacity=historical,triangle_capacity_upper_bound=float(cap),width_capacity_upper_bound=wid,U=U,U_over_historical_capacity=U/historical,width_over_historical_capacity=wid/historical,triangle_over_historical_capacity=float(cap)/historical,exact_triangle_ratio=str(s),exact_original_area_product=str(vol),certificate=cert,selected_support_indices=[x['indices'],y['indices']],feasible_triangle_counts=[np,nq],selected_min_area_tie_counts=[len(tp),len(tq)],word_evaluations=120*len(tp)*len(tq),producer_bounces=r['bounces'])
            rows.append(row);fout.write(json.dumps(row,separators=(',',':'))+'\n')
            if len(rows)%512==0:print(f'{len(rows)} rows, {time.perf_counter()-start:.2f}s',flush=True)
    assert len(seen)==len(width)==10240 and len(rows)==3072
    summary=dict(method='Frozen exact minimum-area supporting triangles; all exact area minimizers per factor; all120cyclic orders per selected pair; U=min(exact triangle capacity bound,numerical retained width). No fitted selector or added supports.',source_sha256=rule.SOURCE_SHA,width_sha256=rule.WIDTH_SHA,selector_sha256=RULE_SHA,selector_module='triangle_envelope.py',tail_contract='Descending inherited historical sys; ties by ascending name; ceil(1024*fraction); quantiles use linear interpolation.',label_limits='Historical numerical capacity/sys and numerical width remain uncertified; exact rational triangle bound certificates do not certify these labels or U.',rows=len(rows),source_rows=len(seen),word_evaluations=sum(r['word_evaluations'] for r in rows),buckets={},hko=rule.hko())
    for bucket in ['5x5','5x6','6x6']:
        rr=sorted([r for r in rows if r['bucket']==bucket],key=lambda r:(-r['historical_sys'],r['name']));assert len(rr)==1024
        summary['buckets'][bucket]={'all':aggregate(rr),**{f'top{pct}%':aggregate(rr[:math.ceil(len(rr)*pct/100)]) for pct in [10,5,2]}}
    summary['stop_decision']='STOP fixed selector as uniformly tight upper-tail explanation' if any(summary['buckets'][b]['top2%']['U_over_historical_capacity']['max']>1.1 for b in summary['buckets']) else 'No >=10% top2% failure; return for interpretation without tuning'
    summary['elapsed_seconds']=time.perf_counter()-start
    (a.out/'tail-summary.json').write_text(json.dumps(summary,indent=2)+'\n')
    print(json.dumps({k:summary[k] for k in ['rows','word_evaluations','elapsed_seconds','stop_decision']},indent=2))
if __name__=='__main__':main()
