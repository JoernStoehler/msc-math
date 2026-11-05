import argparse,hashlib,json,re
from fractions import Fraction as F
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('--thesis',type=Path,required=True);p.add_argument('--patterns',type=Path,required=True);p.add_argument('--packet',type=Path,required=True);p.add_argument('--out',type=Path,required=True);a=p.parse_args()
source=a.patterns/'patterns/upper-tail/conditional-validation/results.json';s=json.loads(source.read_text());rs=s['rows'];exact=[r for r in rs if F(r['capacity'])==F(r['capacity_lower'])];tail=[r for r in rs if r['role']=='principal_top2'];et=[r for r in tail if r in exact]
assert len(rs)==35 and len(tail)==26 and len(exact)==26 and len(et)==17
assert all(abs(float(F(r['capacity']))/r['historical_capacity']-1)<1e-10 for r in rs)
receipt=json.loads((a.thesis/'thesis/review/2026-10-17/conditional-validation-integration/verification.json').read_text());assert receipt['status']=='PASS';assert set(receipt['exact_capacities'])=={r['name'] for r in exact}
r=next(r for r in rs if r['name']=='random_5x5_s3_227');cut=1-F(r['capacity'])/F(r['incumbent_capacity']);assert F('0.01495')<cut<F('0.01497')
chapter=a.thesis/'thesis/chapters/04-quadratic-program.tex';t=chapter.read_text();assert 'thm:quadratic-program-product-six-facet' in t and '120' in t
# Focused absence check, plus direct reader inspection of the product-algorithm section.
assert not re.search(r'two.shadow|angular order|angular merge',t,re.I)
assert (a.thesis/'thesis/build/main.pdf').read_bytes()==(a.thesis/'thesis/review/kai-packet/thesis-for-kai.pdf').read_bytes()
sha=lambda f:hashlib.sha256(f.read_bytes()).hexdigest()
files=[source,chapter,a.patterns/'patterns/upper-tail/mixed-area-order/REPORT.md',a.thesis/'thesis/review/2026-10-17/conditional-validation-integration/verification.json']
files+=sorted(f for f in a.packet.iterdir() if f.suffix=='.md')
out={'status':'PASS','scope':'Narrow editorial claim/source and unchanged-candidate checks; existing primal/dual science checks reused, no producer or new theorem validation.','counts':{'all':35,'tails':26,'exact':26,'exact_tails':17,'uncertified':9},'material_capacity_cut':float(cut),'manuscript_insertion_prerequisite':'Sparse reduction present; two-shadow theorem absent in inspected product section.','candidate_sha256':sha(a.thesis/'thesis/build/main.pdf'),'files':{str(f):sha(f) for f in files}}
a.out.write_text(json.dumps(out,indent=2)+'\n');print(json.dumps({k:v for k,v in out.items() if k!='files'},indent=2))
