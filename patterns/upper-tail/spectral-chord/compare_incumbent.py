#!/usr/bin/env python3
"""Post-score benchmark join; does not change the frozen selector."""
import gzip,hashlib,importlib.util,json,math,sys
from fractions import Fraction as F
from pathlib import Path
import numpy as np
sys.set_int_max_str_digits(0)
D=Path(__file__).parent;R=Path('/workspaces/msc-math-worktrees/patterns-2026-10-17');sp=importlib.util.spec_from_file_location('spectral_metrics',D/'score.py');m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m);scores=json.loads((D/'broad-scores.json').read_text());tailpath=R/'patterns/upper-tail/tail-rows.jsonl.gz'
with gzip.open(tailpath,'rt') as g:old={r['name']:r for r in map(json.loads,g) if r['bucket']=='5x5'}
assert len(old)==1024;rows=[]
for row in scores['rows']:
 t=old[row['name']];assert t['historical_sys']==row['ratio_true'];tri=float(F(t['exact_triangle_ratio']));oldratio=min(row['width'],tri);rr=dict(row,triangle=tri,old_width_triangle=oldratio,exploratory_min_old_spectral=min(oldratio,row['spectral']),old_capacity=t['U'],spectral_capacity_drop_over_old=1-math.sqrt(row['spectral']/oldratio));rows.append(rr)
y=np.array([r['ratio_true'] for r in rows]);order=np.argsort(y,kind='stable');models=['old_width_triangle','triangle','spectral','combined','exploratory_min_old_spectral'];metrics={}
for model in models:
 pp=np.array([v[model] for v in rows]);metrics[model]={}
 for frac in [1.,.1,.05,.02]:metrics[model][f'top_{frac:g}']=m.metrics(y,pp,order[-math.ceil(frac*len(y)):])
controls={}
for frac in [1.,.1,.05,.02]:
 idx=order[-math.ceil(frac*len(y)):];sub=[rows[i] for i in idx];cuts=[r for r in sub if r['spectral_capacity_drop_over_old']>1e-12];controls[f'top_{frac:g}']=dict(n=len(sub),improved_count=len(cuts),capacity_drop_over_5pct=sum(r['spectral_capacity_drop_over_old']>.05 for r in sub),maximum_capacity_drop=max(r['spectral_capacity_drop_over_old'] for r in sub))
out=dict(rows=rows,metrics=metrics,comparison_counts=controls,largest_new_cuts=sorted(rows,key=lambda z:z['spectral_capacity_drop_over_old'],reverse=True)[:5],base_scores_sha256=hashlib.sha256((D/'broad-scores.json').read_bytes()).hexdigest(),tail_rows_sha256=hashlib.sha256(tailpath.read_bytes()).hexdigest(),status='Existing-width-triangle benchmark and exploratory post-score minimum; frozen spectral and min(width,spectral) rules unchanged.');(D/'incumbent-scores.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps({k:v for k,v in out.items() if k not in ['rows','largest_new_cuts']},indent=2));print('largest cuts',[(z['name'],z['spectral_capacity_drop_over_old']) for z in out['largest_new_cuts']])
