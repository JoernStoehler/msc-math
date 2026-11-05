#!/usr/bin/env python3
"""Exact counterexample to two frozen global-bound extensions, no fitting."""
import argparse,hashlib,importlib.util,json,sys
from fractions import Fraction as F
from pathlib import Path

def module(n,p):
 s=importlib.util.spec_from_file_location(n,p);m=importlib.util.module_from_spec(s);sys.modules[n]=m;s.loader.exec_module(m);return m

def serial(v):
 if isinstance(v,F):return str(v)
 if isinstance(v,dict):return {k:serial(x) for k,x in v.items()}
 if isinstance(v,(list,tuple)):return [serial(x) for x in v]
 return v

def main():
 p=argparse.ArgumentParser();p.add_argument('--repo',type=Path,required=True);p.add_argument('--directory',type=Path,required=True);o=p.parse_args();d=o.directory;C=module('global_can',d/'canonical.py');Q=module('global_q5',o.repo/'patterns/upper-tail/conditional-coupling/quadratic.py').Q5;V=module('global_independent',o.repo/'patterns/upper-tail/near-hko-transfer/verify.py');pr=json.loads((d/'predictions.json').read_text());base={k:[[Q.parse(x) for x in u] for u in pr['base'][k]] for k in ['A','B']};gs=[[Q.parse(x) for x in r['loss_gradient']] for r in pr['bank']];r=json.loads((d/'global-counterexample-input.json').read_text());a=[list(map(F,u)) for u in r['A']];b=[list(map(F,u)) for u in r['B']];aq=[[Q(x) for x in u] for u in a];bq=[[Q(x) for x in u] for u in b];allgauges=[]
 for labels,(aa,bb) in C.candidates(aq,bq,base['A'],base['B']):
  de=C.defect(aa,bb,base['A'],base['B'],gs);allgauges.append(dict(labels=labels,defect=de,canonical_A=aa,canonical_B=bb))
 I=min(x['defect'] for x in allgauges);assert I>0;ca=V.vertices(a);cb=V.vertices(b);vol=V.area(a)*V.area(b);qs=[]
 for sign in [1,-1]:
  word=V.word(a,b,sign)
  for al in ca:
   for ga in cb:qs.append(dict(shadow_sign=sign,word=word,A_weights=al,B_weights=ga,Q=V.q(a,b,word,al,ga)))
 q=max(x['Q'] for x in qs);cap=1/(2*q);rho=cap*cap/(2*vol);r0=(Q(3)+Q(0,1))/5;rec=r0/(1+I);gap=Q(rho)-rec;assert gap>0
 def qser(v):
  if isinstance(v,Q):return v.serial()
  if isinstance(v,dict):return {k:qser(x) for k,x in v.items()}
  if isinstance(v,(list,tuple)):return [qser(x) for x in v]
  return serial(v)
 out=dict(status='PASS',name=r['name'],input_sha256={n:hashlib.sha256((d/n).read_bytes()).hexdigest() for n in ['global-counterexample-input.json','canonical.py','predictions.json','global-counterexample.py']},exact_invariant=I,exact_ratio=rho,exact_capacity=cap,exact_volume=vol,exact_reciprocal_prediction=rec,strict_ratio_gap=gap,invariant_float=float(I),ratio_float=float(rho),reciprocal_prediction_float=float(rec),strict_gap_float=float(gap),closure_counts=[len(ca),len(cb)],genuine_five_facets_checked=True,valid_canonical_gauges=len(allgauges),all_gauges=allgauges,all_HK_candidates=qs,proof='Exactsys>rhoH/(1+I), withI>0. Since exp(I)>=1+I, rhoH exp(-I)<=rhoH/(1+I)<exactsys. Thus neither frozenformula is a universal ratio upperbound.')
 (d/'global-counterexample.json').write_text(json.dumps(qser(out),indent=2)+'\n');print(json.dumps({k:out[k] for k in ['status','name','invariant_float','ratio_float','reciprocal_prediction_float','strict_gap_float','closure_counts','valid_canonical_gauges']},indent=2))
if __name__=='__main__':main()
