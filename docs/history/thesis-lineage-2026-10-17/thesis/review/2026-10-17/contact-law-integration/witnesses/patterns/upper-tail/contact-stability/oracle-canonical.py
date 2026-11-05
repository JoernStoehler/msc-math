#!/usr/bin/env python3
"""After all predictors freeze: exact all-closure/two-shadow product truth."""
import argparse,hashlib,importlib.util,json,sys,time
from fractions import Fraction as F
from pathlib import Path

def module(n,p):
 s=importlib.util.spec_from_file_location(n,p);m=importlib.util.module_from_spec(s);sys.modules[n]=m;s.loader.exec_module(m);return m

def serial(v):
 if isinstance(v,F):return str(v)
 if isinstance(v,dict):return {k:serial(w) for k,w in v.items()}
 if isinstance(v,(list,tuple)):return [serial(w) for w in v]
 return v

def main():
 p=argparse.ArgumentParser();p.add_argument('--repo',type=Path,required=True);p.add_argument('--directory',type=Path,required=True);o=p.parse_args();d=o.directory;assert not(d/'canonical-truth.json').exists();t=time.perf_counter()
 sh=module('contact_oracle_shadow',o.repo/'patterns/upper-tail/signed-prefix-response/analyze.py');vi=module('contact_oracle_independent',o.repo/'patterns/upper-tail/near-hko-transfer/verify.py');sample=json.loads((d/'canonical-sample.json').read_text());rows=[]
 for s in sample['rows']:
  a=[list(map(F,u)) for u in s['A']];b=[list(map(F,u)) for u in s['B']];ca=[list(map(F,v)) for v in sh.closures(a)];cb=[list(map(F,v)) for v in sh.closures(b)];V=vi.area(a)*vi.area(b);assert V==sh.old.area(a)*sh.old.area(b);candidates=[]
  for sign in [1,-1]:
   w=sh.shadow_word(a,b,sign)
   if sign<0:w=w[::-1]
   assert w==vi.word(a,b,sign)
   for ia,u in enumerate(ca):
    for ib,v in enumerate(cb):
     q=sh.Q(a,b,w,u,v);assert q==vi.q(a,b,w,u,v);candidates.append(dict(shadow_sign=sign,word=w,A_vertex=ia,B_vertex=ib,A_weights=u,B_weights=v,Q=q))
  q=max(v['Q'] for v in candidates);cap=1/(2*q);ratio=cap*cap/(2*V);row=dict(name=s['name'],family=s['family'],seed=s['seed'],epsilon=s['epsilon'],capacity=cap,volume=V,ratio=ratio,ratio_float=float(ratio),closure_counts=[len(ca),len(cb)],winning_candidates=[v for v in candidates if v['Q']==q],all_candidates=candidates);rows.append(row)
  print(s['name'],float(ratio),flush=True)
 out=dict(protocol='After frozen lane predictions; exact full closure-pair/two-shadow oracle; independent direct-HK equality for every objective',sample_sha256=hashlib.sha256((d/'canonical-sample.json').read_bytes()).hexdigest(),contact_predictions_sha256=hashlib.sha256((d/'canonical-predictions.json').read_bytes()).hexdigest(),oracle_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),rows=rows,elapsed_seconds=time.perf_counter()-t)
 (d/'canonical-truth.json').write_text(json.dumps(serial(out),indent=2)+'\n');print('TRUTH_SHA256',hashlib.sha256((d/'canonical-truth.json').read_bytes()).hexdigest())
if __name__=='__main__':main()
