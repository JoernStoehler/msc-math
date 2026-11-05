#!/usr/bin/env python3
import argparse,json,math,hashlib
from fractions import Fraction as F
from pathlib import Path
H=Path(__file__).resolve().parent
ap=argparse.ArgumentParser();ap.add_argument('--truth',type=Path,required=True);ap.add_argument('--predictions',type=Path,default=H/'predictions.json');ap.add_argument('--out',type=Path,default=H/'scores.json');o=ap.parse_args();p=json.loads(o.predictions.read_text());t=json.loads(o.truth.read_text());truth={r['name']:r for r in t['rows']}
rows=[]
for r in p['rows']:
 s=truth[r['name']];sys0=float(F(s['ratio']));B=F(r['rigorous_bound']);assert B>=F(s['ratio']);rows.append(r|dict(ratio=sys0,above_one=sys0>1,errors={k:r[k]/sys0-1 for k in ['predicted_bound_scaled','predicted_rms']}))
def median(v):
 v=sorted(v);n=len(v);return (v[(n-1)//2]+v[n//2])/2
def ranks(v):
 return [sum(w<x for w in v)+(sum(w==x for w in v)-1)/2 for x in v]
def corr(x,y):
 mx=sum(x)/len(x);my=sum(y)/len(y);a=sum((a-mx)*(b-my) for a,b in zip(x,y));d=sum((a-mx)**2 for a in x)*sum((b-my)**2 for b in y);return a/math.sqrt(d) if d else None
def metrics(z):
 out={}
 for k in ['predicted_bound_scaled','predicted_rms']:
  e=[abs(r['errors'][k]) for r in z];out[k]=dict(count=len(z),relative_MAE=sum(e)/len(e),median=median(e),worst=max(e),mean_signed=sum(r['errors'][k] for r in z)/len(z),spearman=corr(ranks([r[k] for r in z]),ranks([r['ratio'] for r in z])),above_one_predicted=sum(r[k]>1 for r in z))
 out['constant_HKO_relative_MAE']=sum(abs(p['calibration']['HKO_ratio']/r['ratio']-1) for r in z)/len(z)
 return out
families=sorted(set(r['family'] for r in rows));eps=sorted(set(r['epsilon'] for r in rows),key=lambda v:float(F(v)))
out=dict(protocol='unseen-label validation on frozen60 panel; oracle artifact existed before feature freeze but was not inspected; no adaptive change',predictions_sha256=hashlib.sha256(o.predictions.read_bytes()).hexdigest(),truth_sha256=hashlib.sha256(o.truth.read_bytes()).hexdigest(),all=metrics(rows),above_one=metrics([r for r in rows if r['above_one']]),top_quarter=metrics(sorted(rows,key=lambda r:r['ratio'],reverse=True)[:15]),by_family={f:metrics([r for r in rows if r['family']==f]) for f in families},by_epsilon={f:metrics([r for r in rows if r['epsilon']==f]) for f in eps},rows=[{k:r[k] for k in ['name','family','seed','epsilon','ratio','above_one','rigorous_bound','predicted_bound_scaled','predicted_rms','errors']} for r in rows])
o.out.write_text(json.dumps(out,indent=2)+'\n');print(json.dumps({k:v for k,v in out.items() if k!='rows'},indent=2))
