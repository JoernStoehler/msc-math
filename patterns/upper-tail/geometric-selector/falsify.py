#!/usr/bin/env python3
"""Separate fixed-body falsifier; stops at first >=9% certified selector gap.
Does not modify or rerun selector choices, and does not compute true capacity.
"""
from fractions import Fraction as F
from pathlib import Path
from itertools import combinations
import argparse,hashlib,json,time
import analyze as rule

def candidates(a):
    for ids in combinations(range(len(a)),3):
        u=[a[i] for i in ids];c=[rule.cross(u[1],u[2]),rule.cross(u[2],u[0]),rule.cross(u[0],u[1])];s=sum(c)
        if not s:continue
        w=[z/s for z in c]
        if min(w)>0:yield dict(indices=ids,weights=w)

def main():
    ap=argparse.ArgumentParser();ap.add_argument('--source',type=Path,required=True);ap.add_argument('--results',type=Path,required=True);ap.add_argument('--out',type=Path,required=True);a=ap.parse_args();start=time.perf_counter()
    assert hashlib.sha256(a.source.read_bytes()).hexdigest()==rule.SOURCE_SHA
    rows=json.loads(a.results.read_text());target=next(z for z in rows['rows'] if z['name']=='random_5x5_s1_64');sel=F(target['selection']['best']['capacity'])
    for line in a.source.open():
        r=json.loads(line)
        if r['name']==target['name']:break
    aa=[[F(v) for v in u] for u in r['dual_vertices_rational']];p=[u[:2] for u in aa[:r['k']]];q=[u[2:] for u in aa[r['k']:]];count=0
    done=False
    for x in candidates(p):
        for y in candidates(q):
            count+=1;w=rule.best_word(p,q,x,y)
            if w['capacity']*F(109,100)<=sel:done=True;break
        if done:break
    assert done
    gap=sel/w['capacity']
    packet=dict(name=r['name'],source_sha256=rule.SOURCE_SHA,selector_results_sha256=hashlib.sha256(a.results.read_bytes()).hexdigest(),role='Independent post-selector falsifier only; no support added to frozen rule; no true-capacity claim',stop_rule='First lexicographic positive-closure pair with Cwitness <= Cselector/1.09; not full minimization',support_pairs_inspected=count,word_evaluations=120*count,selector_capacity=sel,feasible_challenger=w,exact_lower_bound_selector_over_true_capacity=gap,relative_gap_lower_bound=float(gap)-1,exact_ratio_upper_bound=w['capacity']**2/(2*F(target['exact_original_area_product'])),elapsed_seconds=time.perf_counter()-start)
    a.out.write_text(json.dumps(rule.stringify(packet),indent=2)+'\n');print(json.dumps({k:v for k,v in packet.items() if not isinstance(v,(dict,F))},indent=2));print('CAPGAP',float(gap))
if __name__=='__main__':main()
