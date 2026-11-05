#!/usr/bin/env python3
"""Geometry-only independent local challenge; never imports capacity code."""
import argparse, hashlib, json
from fractions import Fraction as F
from pathlib import Path

def main():
 p=argparse.ArgumentParser();p.add_argument('--repo',type=Path,required=True);p.add_argument('--out',type=Path,required=True);o=p.parse_args();o.out.mkdir(parents=True,exist_ok=True)
 src=o.repo/'patterns/upper-tail/conditional-coupling/hko-exact.json';h=json.loads(src.read_text());q=F(22360679775,10000000000)
 base={k:[[F(v['rational'])+q*F(v['sqrt5']) for v in u] for u in h[k]] for k in ['A','B']}
 rows=[]
 for seed in range(5):
  raw=hashlib.sha256(f'contact-stability-common-v1:{seed}'.encode()).digest();d=[F(v-128,128) for v in raw[:20]]
  for family in ['full','support','angular']:
   for eps in [F(3,1000),F(1,100),F(3,100),F(8,100)]:
    row=dict(name=f'contact_{family}_s{seed}_e{eps.numerator}of{eps.denominator}',family=family,seed=seed,epsilon=str(eps),direction_digest=raw.hex())
    for fi,k in enumerate(['A','B']):
     vals=[]
     for i,(x,y) in enumerate(base[k]):
      j=fi*10+2*i
      if family=='full':u,v=d[j],d[j+1]
      elif family=='support':u,v=d[j]*x,d[j]*y
      else:u,v=-d[j]*y,d[j]*x
      vals.append([str(x+eps*u),str(y+eps*v)])
     row[k]=vals
    rows.append(row)
 out=dict(protocol='geometry-only frozen 5 independent SHA256 directions x full/support/angular x 4 scales; no label selection; 60 five-by-five products',base={k:[[str(x) for x in u] for u in base[k]] for k in base},sqrt5_substitute=str(q),rows=rows)
 (o.out/'sample.json').write_text(json.dumps(out,indent=2)+'\n')
 man=dict(sample_sha256=hashlib.sha256((o.out/'sample.json').read_bytes()).hexdigest(),freeze_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),source_sha256=hashlib.sha256(src.read_bytes()).hexdigest(),n=60)
 (o.out/'sample-manifest.json').write_text(json.dumps(man,indent=2)+'\n');print(json.dumps(man))
if __name__=='__main__':main()
