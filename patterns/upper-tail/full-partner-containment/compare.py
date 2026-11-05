#!/usr/bin/env python3
"""Read-only strongest-incumbent comparison; does not change selector."""
import argparse,json
from fractions import Fraction as F
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('--run',type=Path,required=True);p.add_argument('--prior-comparison',type=Path,required=True);p.add_argument('--out',type=Path,required=True);args=p.parse_args()
r=json.loads(args.run.read_text());old=json.loads(args.prior_comparison.read_text());rows=[]
for x in r['rows']:
    y=next(z for z in old['rows'] if z['name']==x['name']);inc=min(F(y['exact_incumbent_capacity']),F(x['prior_response_capacity']));new=F(x['selection']['capacity'])
    rows.append(dict(name=x['name'],principal_top3=x['principal_top3'],incumbent_capacity=str(inc),new_capacity=str(new),new_over_incumbent=str(new/inc),gain=1-float(new/inc),historical_gap=x['capacity_over_historical']-1,ratio_bound=x['ratio_bound']))
args.out.write_text(json.dumps(rows,indent=2)+'\n')
print(json.dumps(dict(exact_improvements=sum(F(x['new_over_incumbent'])<1 for x in rows),principal_improvements=sum(x['principal_top3'] and F(x['new_over_incumbent'])<1 for x in rows),historical_agreement=sum(abs(x['historical_gap'])<1e-10 for x in rows),principal_agreement=sum(x['principal_top3'] and abs(x['historical_gap'])<1e-10 for x in rows))))
