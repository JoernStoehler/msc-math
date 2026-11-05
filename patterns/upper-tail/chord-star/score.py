#!/usr/bin/env python3
"""Join immutable feature predictions to existing evidence only."""
import hashlib,json,sys
from fractions import Fraction as F
from pathlib import Path
sys.set_int_max_str_digits(0)
R=Path('/workspaces/msc-math-worktrees/patterns-2026-10-17');D=Path(__file__).resolve().parent
p=D/'predictions.json';pred=json.loads(p.read_text());truth={};prior={}
for r in json.loads((R/'patterns/upper-tail/near-hko-transfer/results.json').read_text())['rows']:
 truth[r['name']]=dict(capacity=F(r['truth']['capacity']),ratio=F(r['ratio']),status='imported exact sparse two-shadow oracle')
 lo=min(F(v['capacity_lower']) for v in r['variants']);hi=F(r['rounding']['capacity']);prior[r['name']]=dict(lower=lo,upper=hi,source='conditional coupling plus one response')
for r in json.loads((R/'patterns/upper-tail/conditional-rounding/results.json').read_text())['rows']:
 prior[r['name']]=dict(lower=F(r['capacity_lower']),upper=F(r['combined_capacity']),source='retained strongest combined conditional-rounding interval')
 if r['exact_capacity']:truth[r['name']]=dict(capacity=F(r['combined_capacity']),status='imported exact conditional primal/dual equality')
gc=json.loads((R/'patterns/upper-tail/contact-stability/global-counterexample.json').read_text());truth[gc['name']]=dict(capacity=F(gc['exact_capacity']),ratio=F(gc['exact_ratio']),status='imported exact sparse oracle')
rows=[]
for r in pred['rows']:
 s=r['selection'];c,e,lo,ratio=map(F,[s['capacity'],s['eta'],s['lower_capacity'],s['ratio']]);out=dict(name=r['name'],capacity=float(c),eta=float(e),capacity_interval_factor=float(1/e),ratio_upper=float(ratio),new_lower_capacity=float(lo))
 if r['name'] in truth:
  t=truth[r['name']];tc=t['capacity'];assert lo<=tc<=c;out.update(capacity_over_true=float(c/tc),new_lower_over_true=float(lo/tc),truth_status=t['status'])
 if r['name'] in prior:
  z=prior[r['name']];cl=max(lo,z['lower']);ch=min(c,z['upper']);assert cl<=ch;out.update(prior_lower=float(z['lower']),prior_upper=float(z['upper']),prior_factor=float(z['upper']/z['lower']),new_lower_over_prior=float(lo/z['lower']),combined_factor=float(ch/cl),upper_improvement=float(1-c/z['upper']))
 rows.append(out)
out=dict(rows=rows,predictions_sha256=hashlib.sha256(p.read_bytes()).hexdigest(),scope='ten frozen retrospective controls; no new paired-capacity oracle or broad scan')
(D/'scores.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
