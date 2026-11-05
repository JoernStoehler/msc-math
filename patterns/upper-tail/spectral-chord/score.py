#!/usr/bin/env python3
"""Join only frozen predictions to exposed diagnostic / historical targets."""
import argparse,hashlib,json,math,sys
from pathlib import Path
from fractions import Fraction as F
import numpy as np
sys.set_int_max_str_digits(0)
R=Path('/workspaces/msc-math-worktrees/patterns-2026-10-17')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def number(x):return float(F(x))
def ranks(x):
 x=np.asarray(x);o=np.argsort(x,kind='stable');z=np.empty(len(x),float);i=0
 while i<len(x):
  j=i+1
  while j<len(x) and x[o[j]]==x[o[i]]:j+=1
  z[o[i:j]]=(i+j-1)/2;i=j
 return z

def width(a,b):
 from itertools import combinations
 def points(c):
  out=[]
  for u,v in combinations(c,2):
   det=u[0]*v[1]-u[1]*v[0]
   if not det:continue
   x=[(v[1]-u[1])/det,(u[0]-v[0])/det]
   if all(sum(z*w for z,w in zip(t,x))<=1 for t in c):out.append(x)
  return out
 def steps(c):
  vv=points(c);return [[x/(1+max(-sum(z*w for z,w in zip(u,v)) for v in vv)) for x in u] for u in c]
 aa=steps(a);bb=steps(b);return 1/max(abs(sum(x*y for x,y in zip(u,v))) for u in aa for v in bb)

def metrics(y,p,idx):
 yy=y[idx];pp=p[idx];e=abs(pp/yy-1);picked=np.argsort(p,kind='stable')[-len(idx):];return dict(n=len(idx),median_ratio_error=float(np.median(e)),p90_ratio_error=float(np.quantile(e,.9)),max_ratio_error=float(max(e)),median_prediction_over_true=float(np.median(pp/yy)),spearman=float(np.corrcoef(ranks(yy),ranks(pp))[0,1]),tail_recall=len(set(idx)&set(picked))/len(idx),ratio_R2=float(1-np.sum((pp-yy)**2)/np.sum((yy-np.mean(yy))**2)))

def main():
 p=argparse.ArgumentParser();p.add_argument('mode',choices=['diagnostic','broad']);p.add_argument('--directory',type=Path,required=True);p.add_argument('--expected-sha',required=True);p.add_argument('--source',type=Path);o=p.parse_args();d=o.directory;path=d/(o.mode+'-predictions.json');assert sha(path)==o.expected_sha;pred=json.loads(path.read_text());rows=[]
 if o.mode=='diagnostic':
  truth={v['name']:v for v in json.loads((R/'patterns/upper-tail/lp-seeded-response/results.json').read_text())['rows']};ce=json.loads((R/'patterns/upper-tail/contact-stability/global-counterexample.json').read_text());truth[ce['name']]=dict(capacity_true=ce['exact_capacity'],ratio=ce['exact_ratio'])
  for z in pred['rows']:
   if z['name'] not in truth:continue
   t=truth[z['name']];cp=number(z['selection']['capacity']);ct=number(t['capacity_true']);yp=number(z['selection']['ratio']);yt=number(t['ratio']);wc=float(width([[F(x) for x in u] for u in z['selection']['centered_A']],[[F(x) for x in u] for u in z['selection']['centered_B']]));rows.append(dict(name=z['name'],capacity_predicted=cp,capacity_true=ct,capacity_error=cp/ct-1,ratio_predicted=yp,ratio_true=yt,ratio_error=yp/yt-1,width_capacity=wc,width_ratio=wc*wc/(2*number(z['selection']['volume'])),combined_ratio=min(yp,wc*wc/(2*number(z['selection']['volume']))),above_one=yt>1))
  local=[z for z in rows if z['name'].startswith('lpseed')];out=dict(rows=rows,predictions_sha256=o.expected_sha,metrics={})
  for label,sub in [('whole_local12',local),('above_one_local6',[z for z in local if z['above_one']])]:
   yy=np.array([v['ratio_true'] for v in sub]);pp=np.array([v['ratio_predicted'] for v in sub]);out['metrics'][label]=metrics(yy,pp,np.arange(len(yy)))|dict(subone_point_predictions_above_one=sum(v['ratio_true']<1<v['ratio_predicted'] for v in sub))
 else:
  baseline=json.loads((R/'patterns/upper-tail/canonical-transfer/results.json').read_text());base={z['name']:z for z in baseline['all_rows']};source=[json.loads(z) for z in o.source.open()];truth={z['name']:z for z in source if z['k']==z['m']==5};assert len(pred['rows'])==len(truth)==1024
  for z in pred['rows']:
   name=z['name'];b=base[name];assert b['sys']==truth[name]['sys'];s=z['selection'];row=dict(name=name,ratio_true=truth[name]['sys'],capacity_true=truth[name]['capacity'],spectral=number(s['ratio']),spectral_capacity=number(s['capacity']),width=b['predictions']['width_ceiling'],canonical_exponential=b['predictions']['exp_prediction'],canonical_reciprocal=b['predictions']['reciprocal_prediction']);row['combined']=min(row['spectral'],row['width']);rows.append(row)
  yy=np.array([v['ratio_true'] for v in rows]);order=np.argsort(yy,kind='stable');out=dict(rows=rows,predictions_sha256=o.expected_sha,source_sha256=sha(o.source),baseline_sha256=sha(R/'patterns/upper-tail/canonical-transfer/results.json'),metrics={})
  for model in ['spectral','combined','width','canonical_exponential','canonical_reciprocal']:
   pp=np.array([v[model] for v in rows]);out['metrics'][model]={}
   for frac in [1.,.1,.05,.02]:out['metrics'][model][f'top_{frac:g}']=metrics(yy,pp,order[-math.ceil(frac*len(yy)):])
  out['improvement_over_width']=dict(count=sum(v['spectral']<v['width']*(1-1e-12) for v in rows),material_capacity_drop_gt5pct=sum(math.sqrt(v['spectral']/v['width'])<.95 for v in rows),largest=sorted([dict(name=v['name'],relative_capacity_drop=1-math.sqrt(v['spectral']/v['width']),ratio_true=v['ratio_true'],spectral=v['spectral'],width=v['width']) for v in rows],key=lambda v:v['relative_capacity_drop'],reverse=True)[:5]);out['status']='Frozen geometry predictions joined after completion; historical target accuracy not certified.'
 target=d/(o.mode+'-scores-with-width.json' if o.mode=='diagnostic' else o.mode+'-scores.json');assert not target.exists();target.write_text(json.dumps(out,indent=2)+'\n');print(json.dumps({k:v for k,v in out.items() if k!='rows'},indent=2))
if __name__=='__main__':main()
