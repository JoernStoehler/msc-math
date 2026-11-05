#!/usr/bin/env python3
"""Geometry-only canonical transfer. Does not read source capacity/sys values."""
from __future__ import annotations
import argparse,hashlib,importlib.util,json,math,sys,time
from fractions import Fraction
from pathlib import Path

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def loadmodule(p):
 spec=importlib.util.spec_from_file_location('canonical_transfer_api',p);m=importlib.util.module_from_spec(spec);sys.modules[spec.name]=m;spec.loader.exec_module(m);return m

def numeric(x):
 if isinstance(x,dict):return float(Fraction(x['rational']))+math.sqrt(5)*float(Fraction(x['sqrt5']))
 return float(Fraction(str(x)))
def nmat(a):return [[numeric(x) for x in row] for row in a]
def transformed(c,a,b,G,scale=1.,translate=True):
 # Primal P'=scale*G P+t and Q'=scale*G^-T Q+s.
 aa=[[x/scale for x in c.mat(c.transpose(c.inverse(G)),u)] for u in a]
 bb=[[x/scale for x in c.mat(G,u)] for u in b]
 if translate:
  ta=[.13/max(math.hypot(*u) for u in aa),-.07/max(math.hypot(*u) for u in aa)]
  tb=[-.11/max(math.hypot(*u) for u in bb),.09/max(math.hypot(*u) for u in bb)]
  aa=[[x/(1+c.dot(u,ta)) for x in u] for u in aa]
  bb=[[x/(1+c.dot(u,tb)) for x in u] for u in bb]
 return aa,bb

def main():
 ap=argparse.ArgumentParser();ap.add_argument('--source',type=Path,required=True);ap.add_argument('--canonical',type=Path,required=True);ap.add_argument('--contact',type=Path,required=True);ap.add_argument('--plan',type=Path,required=True);ap.add_argument('--out',type=Path,required=True);ap.add_argument('--limit',type=int,default=0);ap.add_argument('--controls',type=int,default=10);z=ap.parse_args();z.out.mkdir(parents=True,exist_ok=True);start=time.perf_counter();c=loadmodule(z.canonical);bank=json.loads(z.contact.read_text());refA=nmat(bank['base']['A']);refB=nmat(bank['base']['B']);grads=[list(map(numeric,v['loss_gradient'])) for v in bank['bank']];assert len(grads)==20
 geometries=[]
 for line in z.source.open():
  raw=json.loads(line)
  if raw['k']==raw['m']==5:
   dual=nmat(raw['dual_vertices_rational']);geometries.append(dict(name=raw['name'],A=dual[:5],B=[u[2:] for u in dual[5:]]))
   # Each A row is four-dimensional in the source; only its first two coordinates are used.
   geometries[-1]['A']=[u[:2] for u in dual[:5]]
 assert len(geometries)==1024
 if z.limit:geometries=geometries[:z.limit]
 rows=[];controls=[];SH=(3+math.sqrt(5))/5
 for i,r in enumerate(geometries):
  a,b=r['A'],r['B'];g=c.invariant(a,b,refA,refB,grads);I=float(g['defect'])
  try:ep=SH*math.exp(-I)
  except OverflowError:ep=None
  rp=SH/(1+I) if I!=-1 else None
  rows.append(dict(name=r['name'],invariant=I,exp_prediction=ep,reciprocal_prediction=rp,valid_candidates=g['valid_candidates'],minimizer_labels=g['labels'],minimizer_A=g['A'],minimizer_B=g['B']))
  if i<z.controls:
   cases={}
   for label,G,scale in [('paired_positive',[[1.3,.4],[-.2,.8]],1.7),('paired_negative',[[1.2,.3],[.2,-.9]],.8),('translation_only',[[1.,0.],[0.,1.]],1.),('scale_only',[[1.,0.],[0.,1.]],2.1)]:
    cases[label]=transformed(c,a,b,G,scale,translate=label!='scale_only')
   cases['permutation']=([a[j] for j in [3,0,4,1,2]],[b[j] for j in [4,2,0,3,1]])
   cases['factor_swap']=(b,a)
   for label,(aa,bb) in cases.items():
    h=c.invariant(aa,bb,refA,refB,grads);err=abs(h['defect']-I);rel=err/max(1.,abs(I));controls.append(dict(name=r['name'],transformation=label,original=I,transformed=float(h['defect']),absolute_error=err,scaled_error=rel));assert rel<1e-8,(r['name'],label,I,h['defect'])
 elapsed=time.perf_counter()-start;out=dict(protocol='Frozen geometry-only unlabelled canonical contact transfer; no source sys/capacity accesses',source_sha256=sha(z.source),canonical_sha256=sha(z.canonical),contact_predictions_sha256=sha(z.contact),plan_sha256=sha(z.plan),predict_sha256=sha(Path(__file__)),known_HKO_ratio=SH,n=len(rows),gradient_forms=len(grads),rows=rows,invariance_controls=controls,elapsed_seconds=elapsed)
 name='pilot-predictions.json' if z.limit else 'frozen-predictions.json';(z.out/name).write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(dict(status='PASS',rows=len(rows),seconds=elapsed,seconds_per_body=elapsed/len(rows),projection_1024_seconds=elapsed/len(rows)*1024,controls=len(controls),maximum_control_scaled_error=max([r['scaled_error'] for r in controls] or[0]),prediction_sha256=sha(z.out/name))))
if __name__=='__main__':main()
