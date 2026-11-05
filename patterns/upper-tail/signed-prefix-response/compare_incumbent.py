#!/usr/bin/env python3
"""Retained lookup only; never changes the frozen predictor or support choices."""
from fractions import Fraction as F
from pathlib import Path
import argparse,gzip,hashlib,json

def main():
    p=argparse.ArgumentParser();p.add_argument('--run',type=Path,required=True);p.add_argument('--tail-rows',type=Path,required=True);p.add_argument('--falsifier',type=Path,required=True);args=p.parse_args();r=json.loads((args.run/'results.json').read_text())
    with gzip.open(args.tail_rows,'rt') as f:old={x['name']:x for x in map(json.loads,f)}
    out=[]
    for x in r['rows']:
        z=old[x['name']];tri=1/(2*F(z['certificate']['exact_Q']));contact=F(x['selection']['seed']['best']['capacity']);baseline=min(tri,contact);new=F(x['selection']['capacity'])
        out.append(dict(name=x['name'],principal_top3=x['principal_top3'],exact_triangle_capacity=str(tri),exact_incumbent_capacity=str(baseline),exact_new_capacity=str(new),new_improves_incumbent=new<baseline,new_gain_over_incumbent=1-float(new/baseline),incumbent_capacity_over_historical=float(baseline)/x['historical_capacity'],new_capacity_over_historical=x['capacity_over_historical']))
    f=json.loads(args.falsifier.read_text());new=F(next(x for x in r['rows'] if x['name']==f['name'])['selection']['capacity']);competing=F(f['feasible_challenger']['capacity']);ratio=new/competing
    assert ratio>F('1.044275')
    result=dict(method='Comparison only: unchanged new rule vs pre-existing min(minimum-area-triangle,contact); contact already dominates width',results_sha256=hashlib.sha256((args.run/'results.json').read_bytes()).hexdigest(),retained_tail_rows_sha256=hashlib.sha256(args.tail_rows.read_bytes()).hexdigest(),rows=out,remaining_exact_obstruction=dict(name=f['name'],prior_falsifier_sha256=hashlib.sha256(args.falsifier.read_bytes()).hexdigest(),exact_new_over_feasible_challenger=str(ratio),new_over_true_capacity_lower_bound=float(ratio),claim='Ctrue <= Cchallenger, hence Cnew/Ctrue >= Cnew/Cchallenger; no equality claim for true capacity'))
    (args.run/'incumbent-comparison.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(dict(improves_incumbent=sum(x['new_improves_incumbent'] for x in out),principal_improves=sum(x['principal_top3'] and x['new_improves_incumbent'] for x in out),remaining_gap_lower_bound=float(ratio)-1)))
if __name__=='__main__':main()
