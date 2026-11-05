#!/usr/bin/env python3
"""Freeze genuine canonical product invariant before third-panel truth."""
import argparse,hashlib,importlib.util,json,math,sys
from fractions import Fraction as F
from pathlib import Path

def module(n,p):
 s=importlib.util.spec_from_file_location(n,p);m=importlib.util.module_from_spec(s);sys.modules[n]=m;s.loader.exec_module(m);return m

def main():
 p=argparse.ArgumentParser();p.add_argument('--repo',type=Path,required=True);p.add_argument('--directory',type=Path,required=True);o=p.parse_args();d=o.directory;assert not(d/'canonical-predictions.json').exists();c=module('contact_canonical',d/'canonical.py');pr=json.loads((d/'predictions.json').read_text());convert=lambda x:float(F(x['rational']))+math.sqrt(5)*float(F(x['sqrt5']));base={k:[[convert(x) for x in u] for u in pr['base'][k]] for k in ['A','B']};gs=[[convert(x) for x in g['loss_gradient']] for g in pr['bank']];r0=(3+math.sqrt(5))/5;rows=[]
 for r in json.loads((d/'canonical-sample.json').read_text())['rows']:
  a=[[float(F(x)) for x in u] for u in r['A']];b=[[float(F(x)) for x in u] for u in r['B']];best=c.invariant(a,b,base['A'],base['B'],gs);I=max(0.,best['defect']);ordered=c.canonical(a,b,base['A'],base['B']);D=max(0.,c.defect(a,b,base['A'],base['B'],gs));Dc=max(0.,c.defect(*ordered,base['A'],base['B'],gs)) if ordered else None
  rows.append(dict(name=r['name'],family=r['family'],seed=r['seed'],epsilon=r['epsilon'],transformed_from=r['transformed_from'],invariant=I,exp_prediction=r0*math.exp(-I),reciprocal_prediction=r0/(1+I),raw_prediction=r0*math.exp(-D),ordered_prediction=r0*math.exp(-Dc) if Dc is not None else None,selected_labels=best['labels'],valid_candidates=best['valid_candidates']));print(r['name'],'I',I,flush=True)
 out=dict(protocol='Unlabelledcanonicalfinitegauge invariant, twofixedno-fitformulae; all54 predictions prior tothirdtruth',rows=rows,input_sha256={n:hashlib.sha256((d/n).read_bytes()).hexdigest() for n in ['CANONICAL-PLAN.md','canonical.py','canonical-sample.json','predictions.json','predict-canonical.py']});(d/'canonical-predictions.json').write_text(json.dumps(out,indent=2)+'\n');print('CANONICAL_PREDICTIONS_SHA256',hashlib.sha256((d/'canonical-predictions.json').read_bytes()).hexdigest())
if __name__=='__main__':main()
