#!/usr/bin/env python3
"""Independent arithmetic/metric/example replay; no capacity computation."""
from __future__ import annotations
import argparse,hashlib,importlib.util,json,math,sys
from fractions import Fraction
from pathlib import Path
import numpy as np

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def ranks(v):
 order=np.argsort(v,kind='stable');out=np.empty(len(v),float);i=0
 while i<len(v):
  j=i+1
  while j<len(v) and v[order[j]]==v[order[i]]:j+=1
  out[order[i:j]]=(i+j-1)/2;i=j
 return out

def numeric(x):
 if isinstance(x,dict):return float(Fraction(x['rational']))+math.sqrt(5)*float(Fraction(x['sqrt5']))
 return float(Fraction(str(x)))
def main():
 ap=argparse.ArgumentParser();ap.add_argument('--directory',type=Path,default=Path(__file__).resolve().parent);z=ap.parse_args();d=z.directory;f=json.loads((d/'frozen-predictions.json').read_text());r=json.loads((d/'results.json').read_text());examples=json.loads((d/'examples.json').read_text());assert sha(d/'frozen-predictions.json')==r['predictions_sha256']=='4d787bfe27fada28d0dce6a247c68d0551f46f3ce5eac3fcfebe66f363a9b853';assert sha(d/'PLAN.md')==f['plan_sha256'];assert f['n']==r['rows']==1024 and len(f['rows'])==len(r['all_rows'])==1024
 sh=(3+math.sqrt(5))/5;pred={q['name']:q for q in f['rows']};assert len(pred)==1024;rr=r['all_rows'];assert set(q['name'] for q in rr)==set(pred)
 for q in rr:
  p=pred[q['name']];assert abs(q['invariant']-p['invariant'])<1e-14
  ep=sh*math.exp(-p['invariant']);rp=sh/(1+p['invariant'])
  assert abs(ep-q['predictions']['exp_prediction'])<1e-14 and abs(rp-q['predictions']['reciprocal_prediction'])<1e-14
 y=np.array([q['sys'] for q in rr]);order=np.argsort(y,kind='stable');checks=[]
 for model,mm in r['metrics'].items():
  p=np.array([q['predictions'][model] for q in rr])
  for frac in [1.,.1,.05,.02]:
   n=math.ceil(len(y)*frac);idx=order[-n:];picked=np.argsort(p,kind='stable')[-n:];m=mm[f'top_{frac:g}'];err=abs(p[idx]/y[idx]-1);assert m['n']==n
   assert abs(float(np.median(err))-m['median_abs_relative_error'])<1e-12;assert abs(float(np.quantile(err,.9))-m['p90_abs_relative_error'])<1e-12
   if m['spearman'] is not None:
    corr=float(np.corrcoef(ranks(y[idx]),ranks(p[idx]))[0,1]);assert abs(corr-m['spearman'])<1e-12
   if m['top_fraction_recall'] is not None:assert abs(len(set(idx)&set(picked))/n-m['top_fraction_recall'])<1e-12
   checks.append(dict(model=model,fraction=frac,n=n))
 assert len(f['invariance_controls'])==60;controlmax=max(q['scaled_error'] for q in f['invariance_controls']);assert controlmax<1e-8
 # Replay four selected input geometries through the independently owned immutable API.
 api=d.parent/'contact-stability/canonical.py';bankp=d.parent/'contact-stability/predictions.json';assert sha(api)==f['canonical_sha256'];assert sha(bankp)==f['contact_predictions_sha256'];spec=importlib.util.spec_from_file_location('canonical_transfer_verify_api',api);c=importlib.util.module_from_spec(spec);sys.modules[spec.name]=c;spec.loader.exec_module(c);bank=json.loads(bankp.read_text());base={k:[[numeric(x) for x in row] for row in bank['base'][k]] for k in ['A','B']};grads=[[numeric(x) for x in row['loss_gradient']] for row in bank['bank']];replays=[]
 for q in examples['examples']:
  a=[[numeric(x) for x in row] for row in q['A']];b=[[numeric(x) for x in row] for row in q['B']];I=c.invariant(a,b,base['A'],base['B'],grads)['defect'];error=abs(I-q['invariant']);assert error<1e-12;assert q['historical_sys']==next(x['sys'] for x in rr if x['name']==q['name']);replays.append(dict(name=q['name'],invariant_error=error))
 print(json.dumps(dict(status='PASS',predictions_sha256=sha(d/'frozen-predictions.json'),rows=1024,metric_checks=len(checks),invariance_controls=60,max_scaled_invariance_error=controlmax,example_replays=replays,limits='Reconstructs formulas, arithmetic, ranks, recall, identity and numerical coordinate controls. Historical source capacities are not certified here; no global capacity inequality is established.'),indent=2))
if __name__=='__main__':main()
