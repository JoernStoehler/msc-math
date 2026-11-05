#!/usr/bin/env python3
"""Scores frozen predictions; never changes formulas or sample membership."""
import argparse,hashlib,json,math,statistics
from pathlib import Path

def score(rows,key):
 y=[r['truth'] for r in rows];p=[r[key] for r in rows];e=[abs(a/b-1) for a,b in zip(p,y)];mu=statistics.mean(y);ss=sum((a-mu)**2 for a in y);return dict(n=len(rows),median_relative_error=statistics.median(e),max_relative_error=max(e),mean_relative_error=statistics.mean(e),R2=1-sum((a-b)**2 for a,b in zip(p,y))/ss if ss else None,correct_above_one=sum((a>1)==(b>1) for a,b in zip(p,y)),true_above_one=sum(x>1 for x in y),predicted_above_one=sum(x>1 for x in p))
def main():
 p=argparse.ArgumentParser();p.add_argument('--directory',type=Path,required=True);o=p.parse_args();d=o.directory;pred=json.loads((d/'predictions.json').read_text());truth=json.loads((d/'truth.json').read_text());s=json.loads((d/'sample.json').read_text());ps={r['name']:r for r in pred['rows']};ss={r['name']:r for r in s['rows']};rows=[]
 for t in truth['rows']:
  p=ps[t['name']];r={**ss[t['name']],**p,'truth':float(__import__('fractions').Fraction(t['ratio']))};r['constant_prediction']=pred['known_HKO_ratio'];rows.append(r)
 groups={'all':rows,'above_one':[r for r in rows if r['truth']>1],'highest15':sorted(rows,key=lambda r:-r['truth'])[:15],'eps_le_003':[r for r in rows if float(__import__('fractions').Fraction(r['epsilon']))<=.03]}
 for f in ['full','support','angular']:groups['family_'+f]=[r for r in rows if r['family']==f]
 for e in ['3/1000','1/100','3/100','2/25']:groups['epsilon_'+e]=[r for r in rows if r['epsilon']==e]
 out=dict(scores={k:{p:score(g,p) for p in ['contact_prediction','Qlinear_prediction','constant_prediction']} for k,g in groups.items()},rows=rows,input_sha256={n:hashlib.sha256((d/n).read_bytes()).hexdigest() for n in ['sample.json','predictions.json','truth.json']})
 (d/'metrics.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out['scores'],indent=2))
if __name__=='__main__':main()
