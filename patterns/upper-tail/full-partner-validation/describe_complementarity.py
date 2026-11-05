#!/usr/bin/env python3
"""Post-score descriptive minimum of stored feasible bounds. No rule execution."""
import argparse,hashlib,json
from fractions import Fraction as F
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('directory',type=Path);a=p.parse_args();r=json.loads((a.directory/'results.json').read_text());rows=[]
for x in r['rows']:
    if x['role']!='principal_top2':continue
    cap=min(F(x['capacity']),F(x['incumbent_capacity']));rows.append(dict(name=x['name'],exact_minimum_capacity=str(cap),gap=float(cap)/x['historical_capacity']-1))
g=[x['gap'] for x in rows];z=dict(protocol='Post-score descriptive only: min(new,incumbent) of stored exact feasible bounds; not a predeclared selector, fit or prospective evaluation',results_sha256=hashlib.sha256((a.directory/'results.json').read_bytes()).hexdigest(),tail_n=len(rows),within_1pct=sum(-1e-8<=x<=.01+1e-10 for x in g),within_5pct=sum(-1e-8<=x<=.05+1e-10 for x in g),numerical_agreement=sum(abs(x)<1e-10 for x in g),max_gap=max(g),rows=rows)
assert (z['within_1pct'],z['within_5pct'],z['numerical_agreement'])==(26,27,25)
(a.directory/'complementarity.json').write_text(json.dumps(z,indent=2)+'\n');print(json.dumps({k:v for k,v in z.items() if k!='rows'},indent=2))
