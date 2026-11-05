#!/usr/bin/env python3
"""Independent artifact checks for symbolic-search; numpy required."""
import argparse,gzip,hashlib,json,math
from fractions import Fraction as F
from pathlib import Path
import numpy as np

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def checkmetric(rows,pkey,expected):
 y=np.log([r['sys'] for r in rows]);p=np.log([r[pkey] for r in rows]);b=[r['bucket'] for r in rows];tail=[]
 for bucket in sorted(set(b)):
  ii=[i for i,z in enumerate(b) if z==bucket];ii=sorted(ii,key=lambda i:y[i]);tail+=ii[-math.ceil(.1*len(ii)):]
 for label,ii in [('all',list(range(len(y)))),('upper_decile',tail)]:
  err=np.abs(np.expm1(p[ii]-y[ii]));m=expected[label]
  assert m['n']==len(ii)
  assert abs(float(np.median(err))-m['median_abs_relative_error'])<1e-12
  assert abs(float(np.quantile(err,.9))-m['p90_abs_relative_error'])<1e-12
 return dict(n=len(rows),tail_n=len(tail))

def main():
 ap=argparse.ArgumentParser();ap.add_argument('--dir',type=Path,default=Path(__file__).resolve().parent);z=ap.parse_args();d=z.dir
 r=json.loads((d/'results.json').read_text());f=json.loads((d/'frozen-predictions.json').read_text());br=json.loads((d/'blind-results.json').read_text())
 assert sha(d/'PLAN.md')==r['plan_sha256']==f['plan_sha256']
 assert sha(d/'frozen-predictions.json')==r['fresh_predictions_sha256']==br['frozen_predictions_sha256']=='80500673dfb68400c26601d043caf1d9ee6463388bc901ee920c1b337b4a37de'
 with gzip.open(d/'LOBO-predictions.jsonl.gz','rt') as s:rows=[json.loads(l) for l in s]
 assert len(rows)==14334 and len(set(z['name'] for z in rows))==14334
 checks={}
 for lane in ['all_bodies','products']:
  rr=rows if lane=='all_bodies' else [z for z in rows if 'products' in z]
  checks[lane]=checkmetric(rr,lane,r['results'][lane]['LOBO'])
  fs=f['selected'][lane];assert fs==r['results'][lane]['full_corpus_selected'];base=fs['normalization']
  for row in f['challenge_rows']:
   if lane=='all_bodies':E=(row['rho']/base['rho']+row['A']/base['A'])/2
   else:E=min(row['M']/base['M'],row['W']/base['W'])
   pred=math.exp(fs['a']+fs['b']*math.log(E));assert abs(pred-f['predictions'][row['name']][lane])<1e-12
  tt=br['rows'];actual=np.array([x['ratio'] for x in tt]);p=np.array([x['predictions'][lane] for x in tt]);err=abs(p/actual-1)
  assert sum(actual>1)==41 and sum(p>1)==0
  assert abs(float(np.median(err))-br['metrics'][lane]['all']['median_abs_relative_error'])<1e-12
  assert abs(float(np.quantile(err,.9))-br['metrics'][lane]['all']['p90_abs_relative_error'])<1e-12
 # Exact square/diamond bookkeeping: areas P=4,Q=2,Ppolar=2,Qpolar=4.
 V=F(4)*F(2);polarV=F(2)*F(4)/6;M=V*polarV;c=F(4);s=c*c/(2*V)
 assert (V,polarV,M,s)==(F(8),F(4,3),F(32,3),F(1))
 # Proposed HKO-calibrated Mahler ceiling is 1024(7+3sqrt5)/15625<1.
 # sqrt5<9/4 proves the strict inequality using rationals only.
 assert F(1024,15625)*(7+3*F(9,4))<1
 obstruction=dict(P='[-1,1]^2',Q='{|x|+|y|<=1}',volume=str(V),polar_volume=str(polarV),mahler=str(M),capacity=str(c),sys=str(s),candidate_ceiling='1024*(7+3*sqrt(5))/15625',candidate_ceiling_float=1024*(7+3*math.sqrt(5))/15625,assumption='Existing theorem for two centrally symmetric planar factors identifies capacity with the two-bounce width value 4; free-join polar volume factor 1/6.')
 out=dict(status='PASS',prediction_sha256=sha(d/'frozen-predictions.json'),metric_reconstructions=checks,challenge_rows=60,challenge_above_one=41,primitive_reconstruction_rows=len(r['geometry_reconstruction_checks']),primitive_reconstruction_max_relative_error=max(e for row in r['geometry_reconstruction_checks'] for e in row['relative_errors'].values()),mahler_envelope_obstruction=obstruction,limits='Artifact/formula/metric reconstruction; does not independently validate historical capacities or the fresh closure oracle theorem.')
 print(json.dumps(out,indent=2))
if __name__=='__main__':main()
