#!/usr/bin/env python3
"""Construct the complete label-free frozen sample only; no scoring imports."""
import argparse,hashlib,json
from fractions import Fraction as F
from pathlib import Path

def main():
 p=argparse.ArgumentParser();p.add_argument('--repo',type=Path,required=True);p.add_argument('--directory',type=Path,required=True);o=p.parse_args();r=o.repo;d=o.directory
 h=json.loads((r/'patterns/upper-tail/conditional-coupling/hko-exact.json').read_text());q=F(2236068,1000000)
 base={k:[[F(x['rational'])+F(x['sqrt5'])*q for x in u] for u in h[k]] for k in ['A','B']};rows=[]
 for seed in range(6):
  raw=hashlib.sha256(('msc-math/thesis/near-hko-validation/v1:'+str(seed)).encode()).digest();direction=[F(v%9-4,4) for v in raw[:20]]
  for eps in [F(1,1000),F(1,100)]:
   coords=[v+eps*t for v,t in zip([x for k in ['A','B'] for u in base[k] for x in u],direction)];rows.append(dict(name=f'near_hko_s{seed}_e{eps.denominator}',seed=seed,epsilon=str(eps),direction_digest=raw.hex(),A=[[str(x) for x in coords[i:i+2]] for i in range(0,10,2)],B=[[str(x) for x in coords[i:i+2]] for i in range(10,20,2)]))
 sample=dict(protocol='v1 sha256 seed-major scale-minor all12 fixed rational perturbations; no labels',sqrt5_substitute=str(q),rows=rows);(d/'sample.json').write_text(json.dumps(sample,indent=2)+'\n')
 paths=['patterns/upper-tail/conditional-coupling/hko-exact.json','patterns/upper-tail/conditional-coupling/analyze.py','patterns/upper-tail/conditional-rounding/analyze.py','patterns/upper-tail/signed-prefix-response/analyze.py','patterns/upper-tail/geometric-selector/analyze.py','patterns/upper-tail/mixed-area-order/REPORT.md','topics/products/product-qp-six-facet-reduction.tex'];manifest=dict(sources={str(v):hashlib.sha256((r/v).read_bytes()).hexdigest() for v in paths},freeze={v:hashlib.sha256((d/v).read_bytes()).hexdigest() for v in ['PLAN.md','freeze.py','sample.json']});(d/'frozen-manifest.json').write_text(json.dumps(manifest,indent=2)+'\n');print(json.dumps(manifest,indent=2))
if __name__=='__main__':main()
