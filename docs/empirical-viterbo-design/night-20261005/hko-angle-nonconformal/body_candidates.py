# /// script
# dependencies = ["sympy"]
# ///
"""Bounded exact HKO-angle nonconformal-fan body candidate reduction, Q(sqrt5)."""
from itertools import combinations
import sympy as sp
from sympy.polys.matrices import DomainMatrix as DM
F=sp.QQ.algebraic_field(sp.sqrt(5));Z=F.zero;O=F.one;r5=F.unit
detC=F(2)-F(2)*r5/F(5)
a=(r5-O)/F(4);b=(r5+O)/F(4);w=O+b
lam0=[O/(F(2)*w),O/(F(2)*w),Z,b/w,Z]
lams=[[lam0[(i-l)%5] for i in range(5)] for l in range(5)]
edges=[]
for i in range(5):
 row=[Z]*5;row[(i-1)%5]=O;row[(i+1)%5]=O;row[i]=-F(2)*a;edges.append(row)
rows=[row[:3] for row in lams+edges]
def sign(v):
 cs=v.to_list()
 if not cs:return 0
 if len(cs)==1:return int(sp.sign(cs[0]))
 c,a0=cs
 if not c:return int(sp.sign(a0))
 if not a0:return int(sp.sign(c))
 if a0>0 and c>0:return 1
 if a0<0 and c<0:return -1
 q=int(sp.sign(a0*a0-5*c*c))
 return q if a0>0 else -q
def ser(v):return [str(c) for c in v.to_list()]
def num(v):return float(F.to_sympy(v))
inverses=[];singular=[]
for ids in combinations(range(10),3):
 mat=DM([rows[i] for i in ids],(3,3),F)
 if mat.det()==Z:singular.append(ids)
 else:inverses.append((ids,mat.inv().to_list()))
if __name__=='__main__':
 print('systems',120,'nonsingular',len(inverses),'singular',len(singular))
 print('singular row triples',singular)

def run_candidates():
 from pathlib import Path
 import json,time
 from functools import cmp_to_key
 started=time.monotonic();root=Path(__file__).parent
 packet=json.loads((root/'denominators.json').read_text())
 dec=lambda p:F(sp.Rational(p[0]))+F(sp.Rational(p[1]))*r5
 breaks=[dec(z) for z in packet['common_refinement_breakpoints']]
 def denom_lines(probe):
  lines=[]
  for item in packet['denominators']:
   cell=next(c for c in item['cells'] if sign(probe-dec(c['left']))>=0 and (c['right'] is None or sign(dec(c['right'])-probe)>0))
   lines.append((dec(cell['intercept']),dec(cell['slope'])))
  return lines
 def evaluate(p,x):return p[0]+p[1]*x+(p[2]*x*x if len(p)>2 else Z)
 def area_poly(h):
  out=[Z]*3
  for i in range(5):
   for j in range(2):
    for k in range(2):out[j+k]+=h[i][j]*h[(i+1)%5][k]-a*h[i][j]*h[i][k]
  return out
 def nonnegative(poly,lo,hi):
  checks=[(lo,evaluate(poly,lo))]
  if hi is not None:checks.append((hi,evaluate(poly,hi)))
  if sign(poly[2])>0:
   at=-poly[1]/(F(2)*poly[2])
   if sign(at-lo)>=0 and (hi is None or sign(hi-at)>=0):checks.append((at,evaluate(poly,at)))
  if hi is None:
   if sign(poly[2])<0 or (poly[2]==Z and sign(poly[1])<0):return False,checks
  return all(sign(v)>=0 for x,v in checks),checks
 records=[];cellcounts=[];seen_global=set()
 for ci,left in enumerate(breaks):
  right=breaks[ci+1] if ci+1<len(breaks) else None
  probe=(left+right)/F(2) if right is not None else left+O
  ds=denom_lines(probe);found={};feasible_systems=0
  for ids,inv in inverses:
   rhs=[ds[i] if i<5 else (Z,Z) for i in ids]
   h=[tuple(sum((inv[j][k]*rhs[k][power] for k in range(3)),Z) for power in range(2)) for j in range(3)]+[(Z,Z),(Z,Z)]
   lo=left;hi=right;okay=True
   for i,row in enumerate(rows):
    residual=[sum((row[j]*h[j][power] for j in range(3)),Z)-(ds[i][power] if i<5 else Z) for power in range(2)]
    inter,slope=residual
    if slope==Z:
     if sign(inter)<0:okay=False;break
    elif sign(slope)>0:
     bound=-inter/slope
     if sign(bound-lo)>0:lo=bound
    else:
     bound=-inter/slope
     if hi is None or sign(hi-bound)>0:hi=bound
    if hi is not None and sign(hi-lo)<0:okay=False;break
   if not okay or (hi is not None and hi==Z):continue
   feasible_systems+=1
   key=tuple(tuple(str(c) for c in z.to_list()) for coord in h[:3] for z in coord)
   if key in found:found[key]['bases'].append(list(ids));continue
   poly=area_poly(h)
   tests={}
   for label,target in [('HKO',detC/(F(3)+r5))]:
    q=[poly[0],poly[1]-target,poly[2]];passed,checks=nonnegative(q,lo,hi)
    tests[label]={'passed':passed,'checked_values':[{'x':ser(x),'sign':sign(v)} for x,v in checks]}
   rec={'cell':ci,'bases':[list(ids)],'interval':[ser(lo),None if hi is None else ser(hi)],'interval_numeric':[num(lo),None if hi is None else num(hi)],'supports_affine':[[ser(z) for z in coord] for coord in h], 'area_numerator_quadratic':[ser(z) for z in poly],'tests':tests}
   found[key]=rec
  records.extend(found.values());cellcounts.append({'cell':ci,'feasible_bases':feasible_systems,'distinct_candidates':len(found)})
 out={'status':'exact candidate systems/feasibility intervals; theorem premise independently owned','variable':'x=1+s>0','field':'Q(sqrt5)','coefficient_encoding':'descending powers of sqrt5, rational strings','quotient':'h3=h4=0','area_contract':'P=sum h_i*h_(i+1)−((sqrt5−1)/4)*sum h_i²; sys=x*detC/(5P) at capacity1','systems':120,'nonsingular':len(inverses),'singular':[list(q) for q in singular],'cells':cellcounts,'candidates':records,'all_HKO_pass':all(r['tests']['HKO']['passed'] for r in records),'detC':ser(detC),'elapsed_seconds':time.monotonic()-started}
 # Complete equality and excluded parameter limits for the HKO target.
 equality={};zero_records=0
 for rec in records:
  poly=[F([sp.Rational(c) for c in z]) for z in rec['area_numerator_quadratic']]
  gap=[poly[0],poly[1]-detC/(F(3)+r5),poly[2]]
  assert any(z!=Z for z in gap)
  zeros=[test for test in rec['tests']['HKO']['checked_values'] if test['sign']==0]
  if zeros:zero_records+=1
  for test in zeros:
   at=F([sp.Rational(c) for c in test['x']]);assert at==O
   hh=[F([sp.Rational(c) for c in coord[0]])+at*F([sp.Rational(c) for c in coord[1]]) for coord in rec['supports_affine']]
   key=tuple(tuple(str(c) for c in z.to_list()) for z in hh);equality[key]=hh
  lo,hi=rec['interval']
  if lo==[]:assert sign(poly[0])>0
  if hi is None:assert sign(poly[2])>0
 assert len(equality)==1
 unique_h=next(iter(equality.values()))
 assert unique_h==[-O+F(3)*r5/F(5),O-r5/F(5),-O+F(3)*r5/F(5),Z,Z]
 out['sharp_bound']=ser((F(3)+r5)/F(5))
 out['equality']={'x':ser(O),'supports':[ser(z) for z in unique_h],'zero_containing_records':zero_records,'distinct_support_classes':1}
 out['excluded_endpoint_limits']='maximal ratio tends to0 as x→0+ and x→infinity: positive constant/leading P coefficients on every respective feasible candidate interval'
 (root/'body-candidates.json').write_text(json.dumps(out,indent=2)+'\n')
 print(json.dumps({k:out[k] for k in ['cells','all_HKO_pass','detC','elapsed_seconds']},indent=2))
 print('failed_HKO',[(r['cell'],r['bases'][0],r['interval_numeric']) for r in records if not r['tests']['HKO']['passed']])
 return out

if __name__=='__main__':run_candidates()
