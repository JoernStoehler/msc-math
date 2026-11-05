#!/usr/bin/env python3
"""Freeze new independent geometry and source identities, without scoring imports."""
import argparse,hashlib,json
from fractions import Fraction as F
from pathlib import Path

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
 p=argparse.ArgumentParser();p.add_argument('--repo',type=Path,required=True);p.add_argument('--directory',type=Path,required=True);o=p.parse_args();r=o.repo;d=o.directory;assert not (d/'sample.json').exists()
 h=json.loads((r/'patterns/upper-tail/conditional-coupling/hko-exact.json').read_text());base=[F(v['rational'])+F(v['sqrt5'])*F(2236068,1000000) for k in ['A','B'] for u in h[k] for v in u];rows=[]
 for seed in range(6):
  raw=hashlib.sha256(('msc-math/thesis/lp-seeded-response/validation/v1:'+str(seed)).encode()).digest();direction=[F(byte%9-4,4) for byte in raw[:20]]
  for eps in [F(1,100),F(1,20)]:
   coords=[x+eps*v for x,v in zip(base,direction)];rows.append(dict(name=f'lpseed_near_hko_s{seed}_e{eps.denominator}',seed=seed,epsilon=str(eps),direction_digest=raw.hex(),A=[[str(x) for x in coords[i:i+2]] for i in range(0,10,2)],B=[[str(x) for x in coords[i:i+2]] for i in range(10,20,2)]))
 (d/'sample.json').write_text(json.dumps(dict(namespace='msc-math/thesis/lp-seeded-response/validation/v1:',rows=rows),indent=2)+'\n')
 paths=['patterns/upper-tail/conditional-coupling/hko-exact.json','patterns/upper-tail/conditional-coupling/analyze.py','patterns/upper-tail/conditional-coupling/results.json','patterns/upper-tail/signed-prefix-response/analyze.py','patterns/upper-tail/geometric-selector/analyze.py','patterns/upper-tail/near-hko-transfer/predictions.json','patterns/upper-tail/near-hko-transfer/results.json','patterns/upper-tail/near-hko-transfer/verify.py','patterns/upper-tail/mixed-area-order/REPORT.md','topics/products/product-qp-six-facet-reduction.tex']
 manifest=dict(sources={k:sha(r/k) for k in paths},freeze={k:sha(d/k) for k in ['PLAN.md','freeze.py','analyze.py','sample.json']});(d/'frozen-manifest.json').write_text(json.dumps(manifest,indent=2)+'\n');print(json.dumps(manifest,indent=2))
if __name__=='__main__':main()
