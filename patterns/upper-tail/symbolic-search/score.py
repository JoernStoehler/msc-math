#!/usr/bin/env python3
"""Score already frozen symbolic predictions; may read challenge truth."""
import argparse,hashlib,importlib.util,json,math
from pathlib import Path
import numpy as np
from scipy.stats import spearmanr
FROZEN='80500673dfb68400c26601d043caf1d9ee6463388bc901ee920c1b337b4a37de'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def metrics(y,p):
 er=np.asarray(p)/y-1
 return dict(n=len(y),median_abs_relative_error=float(np.median(abs(er))),p90_abs_relative_error=float(np.quantile(abs(er),.9)),max_abs_relative_error=float(max(abs(er))),median_prediction_over_target=float(np.median(1+er)),spearman=float(spearmanr(y,p).statistic),predicted_above_one=int(np.sum(p>1)),actual_above_one=int(np.sum(y>1)))
def main():
 ap=argparse.ArgumentParser();ap.add_argument('--predictions',type=Path,required=True);ap.add_argument('--truth',type=Path,required=True);ap.add_argument('--out',type=Path,required=True);z=ap.parse_args();assert sha(z.predictions)==FROZEN
 f=json.loads(z.predictions.read_text());t=json.loads(z.truth.read_text());assert f['challenge_sha256']==t['sample_sha256'];truth={r['name']:r for r in t['rows']};assert set(truth)==set(f['predictions']);names=list(f['predictions']);y=np.array([truth[n]['ratio_float'] for n in names]);result={};rows=[]
 for lane in ['all_bodies','products']:
  p=np.array([f['predictions'][n][lane] for n in names]);result[lane]=dict(all=metrics(y,p),above_one=metrics(y[y>1],p[y>1]),HKO_prediction=f['selected'][lane]['HKO_prediction'],HKO_true=(3+math.sqrt(5))/5,HKO_relative_error=f['selected'][lane]['HKO_prediction']/((3+math.sqrt(5))/5)-1,by_family={})
  for family in sorted(set(truth[n]['family'] for n in names)):
   mask=np.array([truth[n]['family']==family for n in names]);result[lane]['by_family'][family]=metrics(y[mask],p[mask])
 for i,n in enumerate(names):rows.append(dict(name=n,family=truth[n]['family'],epsilon=truth[n]['epsilon'],ratio=float(y[i]),predictions=f['predictions'][n],relative_errors={k:float(p/y[i]-1) for k,p in f['predictions'][n].items()}))
 out=dict(frozen_predictions_sha256=FROZEN,truth_sha256=sha(z.truth),sample_sha256=f['challenge_sha256'],metrics=result,truth_range=[float(min(y)),float(max(y))],counterexamples=sorted(rows,key=lambda r:r['ratio'],reverse=True)[:3],rows=rows)
 z.out.write_text(json.dumps(out,indent=2)+'\n');print(json.dumps({k:v for k,v in out.items() if k not in ['rows']},indent=2))
if __name__=='__main__':main()
