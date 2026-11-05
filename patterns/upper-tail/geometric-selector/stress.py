#!/usr/bin/env python3
"""Exactly two frozen second-stage width extremes, unchanged selector."""
from fractions import Fraction as F
from pathlib import Path
import argparse, hashlib, json, time
import analyze as rule

def main():
    ap=argparse.ArgumentParser();ap.add_argument('--source',type=Path,required=True);ap.add_argument('--names',type=Path,required=True);ap.add_argument('--out',type=Path,required=True);a=ap.parse_args();start=time.perf_counter()
    assert hashlib.sha256(a.source.read_bytes()).hexdigest()==rule.SOURCE_SHA
    names=json.loads(a.names.read_text());assert names==['random_5x6_s2_220','random_6x6_s1_194'];geometries={}
    for line in a.source.open():
        r=json.loads(line)
        if r['name'] in names:geometries[r['name']]=r
    selections={}
    for name in names:
        r=geometries[name];aa=[[F(v) for v in u] for u in r['dual_vertices_rational']];p=[u[:2] for u in aa[:r['k']]];q=[u[2:] for u in aa[r['k']:]];s=rule.select(p,q);vol=rule.area(p)*rule.area(q);selections[name]=(s,vol)
    rows=[]
    for name in names:
        r=geometries[name];s,vol=selections[name];cap=s['best']['capacity'];ratio=cap*cap/(2*vol)
        rows.append(dict(name=name,bucket=f"{r['k']}x{r['m']}",sample_kind='Second-stage fixed width extreme',selection=s,exact_original_area_product=vol,exact_ratio_upper_bound=ratio,ratio_upper_bound=float(ratio),historical_capacity=r['capacity'],historical_ratio=r['sys'],capacity_over_historical=float(cap)/r['capacity'],width_over_historical=float(s['width_capacity'])/r['capacity'],relative_gain_over_width=1-float(cap/s['width_capacity']),exact_bound_below_one=ratio<=1))
    packet=dict(source_sha256=rule.SOURCE_SHA,protocol='Second-stage explicitly retrospective amendment; exactly two existing width extremes frozen before evaluation, unchanged rule',rows=rows,elapsed_seconds=time.perf_counter()-start)
    a.out.write_text(json.dumps(rule.stringify(packet),indent=2)+'\n');print(json.dumps([{k:r[k] for k in ['name','ratio_upper_bound','historical_ratio','capacity_over_historical','width_over_historical','relative_gain_over_width','exact_bound_below_one']} for r in rows],indent=2))
if __name__=='__main__':main()
