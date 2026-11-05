import sys,json,hashlib,argparse,math
from pathlib import Path
import numpy as np
sys.path.insert(0,str(Path(__file__).parent));from analyze import metrics
p=Path(__file__).parent;ap=argparse.ArgumentParser();ap.add_argument('--predictions',type=Path,default=p/'blind-predictions.json');ap.add_argument('--mechanistic',type=Path,default=p/'mechanistic-blind.json');ap.add_argument('--truth',type=Path,required=True);ap.add_argument('--out',type=Path,required=True);args=ap.parse_args();pred=json.loads(args.predictions.read_text());truthpath=args.truth;truth=json.loads(truthpath.read_text());tr={r['name']:r for r in truth['rows']};mechan={r['name']:r['predicted_sys'] for r in json.loads(args.mechanistic.read_text())['rows']};rows=[]
for r in pred['rows']:
 t=tr[r['name']];r['truth']=t['ratio_float'];r['predictions']['HKO_mechanistic']=mechan[r['name']];rows.append(r)
y=np.log([r['truth'] for r in rows]);models=rows[0]['predictions'];scores={}
for m in models:
 pp=np.log([r['predictions'][m] for r in rows]);scores[m]={'all':metrics(y,pp),'above_one':metrics(y[np.exp(y)>1],pp[np.exp(y)>1]),'above_one_recall':sum(r['predictions'][m]>1 and r['truth']>1 for r in rows)/sum(r['truth']>1 for r in rows),'above_one_precision':sum(r['predictions'][m]>1 and r['truth']>1 for r in rows)/max(1,sum(r['predictions'][m]>1 for r in rows)),'families':{f:metrics(y[[r['family']==f for r in rows]],pp[[r['family']==f for r in rows]]) for f in sorted(set(r['family'] for r in rows))}}
S=(3+math.sqrt(5))/5;constant=metrics(y,np.full(len(y),math.log(S)));deficits={}
for family in ['all']+sorted(set(r['family'] for r in rows)):
 rr=[r for r in rows if family=='all' or r['family']==family];dr=[(S-r['predictions']['HKO_mechanistic'])/(S-r['truth']) for r in rr];deficits[family]=dict(min=float(min(dr)),median=float(np.median(dr)),max=float(max(dr)))
out=dict(HKO_constant=constant,fraction_of_HKO_loss_captured=deficits,prediction_sha256=hashlib.sha256(args.predictions.read_bytes()).hexdigest(),mechanistic_prediction_sha256=hashlib.sha256(args.mechanistic.read_bytes()).hexdigest(),truth_sha256=hashlib.sha256(truthpath.read_bytes()).hexdigest(),score_labels='Fresh exact rational capacity and exact rational planar product volume from separate all-closure/two-shadow oracle',scores=scores,rows=rows);args.out.write_text(json.dumps(out,indent=2)+'\n');print(json.dumps({k:{'all':v['all'],'above_one_recall':v['above_one_recall'],'above_one_precision':v['above_one_precision']} for k,v in scores.items()},indent=2))
