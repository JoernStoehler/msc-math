#!/usr/bin/env python3
import argparse,hashlib,json,statistics
from fractions import Fraction as F
from pathlib import Path

def score(rows,key):
 e=[abs(r[key]/r['truth']-1) for r in rows];y=[r['truth'] for r in rows];mu=statistics.mean(y);ss=sum((v-mu)**2 for v in y);return dict(n=len(rows),median_relative_error=statistics.median(e),max_relative_error=max(e),mean_relative_error=statistics.mean(e),R2=1-sum((r[key]-r['truth'])**2 for r in rows)/ss if ss else None,correct_above_one=sum((r[key]>1)==(r['truth']>1) for r in rows),true_above_one=sum(r['truth']>1 for r in rows),prediction_below_truth=sum(r[key]<r['truth'] for r in rows))
def main():
 p=argparse.ArgumentParser();p.add_argument('--directory',type=Path,required=True);o=p.parse_args();d=o.directory;pred=json.loads((d/'canonical-predictions.json').read_text());truth=json.loads((d/'canonical-truth.json').read_text());ts={r['name']:r for r in truth['rows']};rows=[r|dict(truth=float(F(ts[r['name']]['ratio']))) for r in pred['rows']];ind=[r for r in rows if r['transformed_from'] is None];groups={'independent45':ind,'above_one_independent':[r for r in ind if r['truth']>1],'highest12_independent':sorted(ind,key=lambda r:-r['truth'])[:12],'group_transforms9':[r for r in rows if r['transformed_from'] is not None]}
 for eps in ['1/100','1/25','1/10']:groups['eps_'+eps]=[r for r in ind if r['epsilon']==eps]
 out=dict(scores={k:{f:score(g,f) for f in ['exp_prediction','reciprocal_prediction','ordered_prediction','raw_prediction']} for k,g in groups.items()},rows=rows,input_sha256={n:hashlib.sha256((d/n).read_bytes()).hexdigest() for n in ['canonical-predictions.json','canonical-truth.json']});(d/'canonical-metrics.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out['scores'],indent=2))
if __name__=='__main__':main()
