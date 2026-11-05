#!/usr/bin/env python3
"""Post-freeze matched historical scoring for canonical contact transfer."""
from __future__ import annotations
import argparse,csv,gzip,hashlib,json,math
from pathlib import Path
import numpy as np
from scipy.stats import spearmanr

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def metric(y,p,idx,full_top):
 yy=y[idx];pp=p[idx];err=pp/yy-1;valid=np.isfinite(pp)&(pp>0);den=np.sum((np.log(yy)-np.mean(np.log(yy)))**2)
 return dict(n=len(idx),invalid_predictions=int(np.sum(~valid)),median_abs_relative_error=float(np.median(abs(err))),p90_abs_relative_error=float(np.quantile(abs(err),.9)),max_abs_relative_error=float(max(abs(err))),median_prediction_over_target=float(np.median(pp/yy)),spearman=(float(spearmanr(yy,pp).statistic) if np.std(pp)>1e-14 else None),top_fraction_recall=(float(len(set(idx)&set(full_top))/len(idx)) if np.std(pp)>1e-14 or len(idx)==len(y) else None),log_R2=float(1-np.sum((np.log(pp)-np.log(yy))**2)/den) if all(valid) and den>0 else None)
def main():
 ap=argparse.ArgumentParser();ap.add_argument('--predictions',type=Path,required=True);ap.add_argument('--expected-sha',required=True);ap.add_argument('--source',type=Path,required=True);ap.add_argument('--width',type=Path,required=True);ap.add_argument('--polar',type=Path,required=True);ap.add_argument('--out',type=Path,required=True);z=ap.parse_args();assert sha(z.predictions)==z.expected_sha;f=json.loads(z.predictions.read_text());assert sha(z.source)==f['source_sha256'];pred={r['name']:r for r in f['rows']};assert len(pred)==1024
 truth={}
 for line in z.source.open():
  r=json.loads(line)
  if r['name'] in pred:
   assert r['k']==r['m']==5;truth[r['name']]=r['sys']
 with gzip.open(z.width,'rt') as g:width={r['name']:r for r in csv.DictReader(g) if r['name'] in pred}
 with gzip.open(z.polar,'rt') as g:polar={r['name']:r for r in map(json.loads,g) if r['name'] in pred}
 assert len(truth)==len(width)==len(polar)==1024;names=list(pred);y=np.array([truth[n] for n in names]);SH=f['known_HKO_ratio'];predictions={k:np.array([pred[n][k] if pred[n][k] is not None else np.nan for n in names]) for k in ['exp_prediction','reciprocal_prediction']};predictions.update(width_ceiling=np.array([float(width[n]['sys2']) for n in names]),polar_5_alpha=np.array([25/(2*polar[n]['alpha_root_volume']**2) for n in names]),constant_HKO=np.full(len(y),SH))
 for n in names:assert truth[n]==float(width[n]['sys'])==polar[n]['sys']
 metrics={};order=np.argsort(y,kind='stable');n=len(y)
 for model,p in predictions.items():
  m={}
  for frac in [1.,.1,.05,.02]:
   count=math.ceil(frac*n);idx=order[-count:];picked=np.argsort(p,kind='stable')[-count:];m[f'top_{frac:g}']=metric(y,p,idx,picked)
  metrics[model]=m
 rows=[dict(name=name,sys=float(y[i]),invariant=pred[name]['invariant'],predictions={k:float(v[i]) for k,v in predictions.items()},relative_errors={k:float(v[i]/y[i]-1) for k,v in predictions.items()},valid_candidates=pred[name]['valid_candidates']) for i,name in enumerate(names)]
 counter={}
 for model in ['exp_prediction','reciprocal_prediction']:
  counter[model]=dict(largest_absolute_relative_errors=sorted(rows,key=lambda r:abs(r['relative_errors'][model]),reverse=True)[:5],true_top=sorted(rows,key=lambda r:r['sys'],reverse=True)[:5],predicted_top=sorted(rows,key=lambda r:r['predictions'][model],reverse=True)[:5])
 result=dict(predictions_sha256=z.expected_sha,input_sha256={str(p):sha(p) for p in [z.source,z.width,z.polar]},rows=1024,sys_range=[float(min(y)),float(max(y))],I_range=[float(min(r['invariant'] for r in f['rows'])),float(max(r['invariant'] for r in f['rows']))],I_negative=sum(r['invariant']<0 for r in f['rows']),valid_candidate_counts=sorted(set(r['valid_candidates'] for r in f['rows'])),metrics=metrics,counterexamples=counter,all_rows=rows,status='Unchanged parameter-free predictions; retrospective historical targets; no post-score selection or calibration')
 z.out.write_text(json.dumps(result,indent=2,allow_nan=False)+'\n');print(json.dumps({k:v for k,v in result.items() if k not in ['input_sha256','all_rows','counterexamples']},indent=2))
if __name__=='__main__':main()
