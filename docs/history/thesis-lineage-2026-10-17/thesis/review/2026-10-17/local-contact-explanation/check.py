"""Audit the explanation's retained evidence and metrics; does not compute capacities."""
import argparse,datetime,hashlib,json,math,statistics
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('--patterns',type=Path,required=True);p.add_argument('--out',type=Path,required=True);args=p.parse_args()
r=args.patterns.resolve();c=r/'patterns/upper-tail/contact-stability'
metrics=json.loads((c/'canonical-metrics.json').read_text())
rows=[x for x in metrics['rows'] if x['transformed_from'] is None]
above=[x for x in rows if x['truth']>1]
assert len(rows)==45 and len(above)==23 and len({x['seed'] for x in rows})==5
errors=[abs(x['exp_prediction']/x['truth']-1) for x in above]
score=metrics['scores']['above_one_independent']['exp_prediction']
for actual,key in [(statistics.median(errors),'median_relative_error'),(max(errors),'max_relative_error')]:assert math.isclose(actual,score[key],rel_tol=1e-10)
assert round(100*statistics.median(errors),5)==.00656
assert round(100*max(errors),5)==.12860
transfer=json.loads((r/'patterns/upper-tail/canonical-transfer/results.json').read_text())
tr=transfer['all_rows'];assert len(tr)==1024
subset=sorted(tr,key=lambda x:x['sys'],reverse=True)[:math.ceil(.1*len(tr))]
def ranks(xs):
 order=sorted(range(len(xs)),key=lambda i:xs[i]);out=[0.]*len(xs);i=0
 while i<len(order):
  j=i+1
  while j<len(order) and xs[order[j]]==xs[order[i]]:j+=1
  for k in order[i:j]:out[k]=(i+j-1)/2
  i=j
 return out
xs=ranks([x['sys'] for x in subset]);ys=ranks([x['predictions']['exp_prediction'] for x in subset])
xm=statistics.mean(xs);ym=statistics.mean(ys)
rho=sum((x-xm)*(y-ym) for x,y in zip(xs,ys))/math.sqrt(sum((x-xm)**2 for x in xs)*sum((y-ym)**2 for y in ys))
assert math.isclose(rho,transfer['metrics']['exp_prediction']['top_0.1']['spearman'],abs_tol=1e-12)
assert round(rho,3)==-.008
full=json.loads((c/'fullword.json').read_text());v=json.loads((c/'verification.json').read_text());canon=json.loads((c/'canonical-verification.json').read_text())
assert full['status']=='PASS' and full['active_words']==50 and full['distinct_active_gradients']==20
assert full['all_cyclic_words']==3000 and v['gradient_rank']==11 and v['symmetry_tangent_rank']==9
# Evaluate the plotted radial direction on the frozen exact gradient bank.
from fractions import Fraction as F
def field(x):return F(x['rational']),F(x['sqrt5'])
def add(x,y):return x[0]+y[0],x[1]+y[1]
def mul(x,y):return x[0]*y[0]+5*x[1]*y[1],x[0]*y[1]+x[1]*y[0]
def neg(x):return -x[0],-x[1]
def sign(x):
 a,b=x
 if a==0:return (b>0)-(b<0)
 if b==0:return (a>0)-(a<0)
 if (a>0)==(b>0):return (a>0)-(a<0)
 d=a*a-5*b*b
 return ((a>0)-(a<0)) if d>0 else ((b>0)-(b<0))
pred=json.loads((c/'predictions.json').read_text())
zero=(F(0),F(0));direction=[zero]*20
for i in range(2):direction[i]=field(pred['base']['A'][0][i]);direction[2+i]=neg(field(pred['base']['A'][1][i]))
vals=[]
for row in pred['bank']:
 value=zero
 for coefficient,direction_i in zip(row['loss_gradient'],direction):value=add(value,mul(field(coefficient),direction_i))
 vals.append(value)
coefficient=(F(0),F(2,5))
assert coefficient in vals and neg(coefficient) in vals
assert all(sign(add(coefficient,neg(v)))>=0 and sign(add(coefficient,v))>=0 for v in vals)
bindings={}
for name in ['REPORT.md','CANONICAL.md','OBSTRUCTION.md','fullword.json','verification.json','canonical-verification.json','canonical-metrics.json','global-verification.json','predictions.json']:
 bindings['patterns/upper-tail/contact-stability/'+name]=hashlib.sha256((c/name).read_bytes()).hexdigest()
bindings['patterns/upper-tail/canonical-transfer/results.json']=hashlib.sha256((r/'patterns/upper-tail/canonical-transfer/results.json').read_bytes()).hexdigest()
out={'checked_at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'status':'PASS','source_bindings':bindings,'canonical_distinct':len(rows),'above_one':len(above),'seed_directions':5,'above_one_median_error_percent':100*statistics.median(errors),'above_one_worst_error_percent':100*max(errors),'historical_count':len(tr),'upper_decile_count':len(subset),'upper_decile_spearman':rho,'exact_cusp_bank_check':'PASS: max and negative min both2sqrt5/5','scope':'Independent exact cusp evaluation and recomputation of displayed finite metrics and source binding only; imported mathematical certificates retain their original scope, no capacities or theorem census recomputed.'}
args.out.parent.mkdir(parents=True,exist_ok=True);args.out.write_text(json.dumps(out,indent=2)+'\n');print('PASS:45/23 canonical controls;103 historical-tail ranks; source bindings')
