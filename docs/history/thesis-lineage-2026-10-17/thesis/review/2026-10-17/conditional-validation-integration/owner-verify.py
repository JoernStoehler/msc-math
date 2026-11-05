#!/usr/bin/env python3
"""Owner reconstruction using prior independent LP/closure/4D checkers, no producer."""
from pathlib import Path
from fractions import Fraction as F
from functools import cmp_to_key
import argparse,datetime,hashlib,importlib.util,json,sys

def module(name,path):
 s=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(s);sys.modules[name]=m;s.loader.exec_module(m);return m

def area(normals,base):
 pts=[]
 for i,a in enumerate(normals):
  for b in normals[i+1:]:
   d=base.cross(a,b)
   if not d:continue
   p=((b[1]-a[1])/d,(a[0]-b[0])/d)
   if all(base.dot(v,p)<=1 for v in normals) and p not in pts:pts.append(p)
 assert len(pts)==len(normals)
 center=[sum(p[k] for p in pts)/len(pts) for k in range(2)]
 def compare(p,q):
  u=[p[k]-center[k] for k in range(2)];v=[q[k]-center[k] for k in range(2)]
  hu=0 if u[1]>0 or (u[1]==0 and u[0]>=0) else 1
  hv=0 if v[1]>0 or (v[1]==0 and v[0]>=0) else 1
  if hu!=hv:return hu-hv
  d=base.cross(u,v);return -1 if d>0 else 1 if d<0 else 0
 pts.sort(key=cmp_to_key(compare))
 ar=sum(base.cross(p,q) for p,q in zip(pts,pts[1:]+pts[:1]))/2
 assert ar>0;return ar

def main():
 p=argparse.ArgumentParser();p.add_argument('--thesis',type=Path,required=True);p.add_argument('--patterns',type=Path,required=True);p.add_argument('--results',type=Path,required=True);p.add_argument('--out',type=Path,required=True);a=p.parse_args()
 base=module('owner_lp',a.thesis/'thesis/review/2026-10-17/conditional-coupling-integration/owner-verify.py')
 rounding=module('owner_round',a.thesis/'thesis/review/2026-10-17/conditional-rounding-integration/independent.py')
 d=json.loads(a.results.read_text());source_path=a.patterns/'patterns/upper-tail/full-partner-validation/results.json';source=json.loads(source_path.read_text());src={r['name']:r for r in source['rows']}
 sample=json.loads((a.results.parent/'sample.json').read_text())
 original=json.loads((a.patterns/'patterns/upper-tail/full-partner-validation/sample.json').read_text())['sample']
 designs={r['name'] for r in json.loads((a.patterns/'patterns/upper-tail/conditional-coupling/results.json').read_text())['rows']}
 expected=[r for r in original if r['name'] not in designs]
 frozen=sample['sample'] if isinstance(sample,dict) else sample
 assert [r['name'] for r in frozen]==[r['name'] for r in expected]
 assert [r['name'] for r in d['rows']]==[r['name'] for r in expected]
 assert len(expected)==35 and sum(r['role']=='principal_top2' for r in expected)==26
 for path,h in d.get('sources',{}).items():assert hashlib.sha256((a.patterns/path).read_bytes()).hexdigest()==h,path
 assert d['failures']==[]
 for key,path in [('plan_sha256',a.results.parent/'PLAN.md'),('sample_sha256',a.results.parent/'sample.json'),('analyze_sha256',a.results.parent/'analyze.py')]:assert hashlib.sha256(path.read_bytes()).hexdigest()==d[key]
 allrows=[];count=0;material=[];miss=[];exact=[]
 for row,entry in zip(d['rows'],expected):
  name=row['name'];old=src[name];n=old['k'];geom=[[F(x) for x in v] for v in old['geometry']]
  aa=[[F(x) for x in v] for v in row['A']];bb=[[F(x) for x in v] for v in row['B']]
  assert aa==[v[:2] for v in geom[:n]] and bb==[v[2:] for v in geom[n:]]
  A,rhs=base.equations(aa,bb);lpvals=[]
  for v in row['variants']:
   w=base.angular(aa,bb,v['shadow_sign']);assert w==v['word']
   c=base.objective(aa,bb,w);assert c==list(map(F,v['objective']))
   assert A==[list(map(F,t)) for t in v['A_eq']] and rhs==list(map(F,v['b_eq']))
   W=list(map(F,v['W']));y=list(map(F,v['dual']));q=F(v['Q'])
   assert min(W)>=0 and [base.dot(t,W) for t in A]==rhs
   sl=[sum(A[k][i]*y[k] for k in range(len(A)))-c[i] for i in range(len(W))]
   assert min(sl)>=0 and sl==list(map(F,v['dual_slack']))
   assert base.dot(c,W)==base.dot(rhs,y)==q
   lpvals.append(q)
  rebuilt=rounding.check_reported(row,row['rounding'],F);count+=len(rebuilt['all'])
  cap=rebuilt['C'];inc=min(F(old['capacity']),F(old['incumbent_capacity']));lo=1/(2*max(lpvals));comb=min(cap,inc)
  assert F(row['incumbent_capacity'])==inc and F(row['capacity'])==cap
  assert F(row['combined_capacity'])==comb and F(row['capacity_lower'])==lo
  assert cap>=lo and row['exact_capacity']==(lo==comb)
  ar=area(aa,base);br=area(bb,base);assert F(row['area_product'])==ar*br
  h=float(row['historical_capacity']);assert h==float(old['historical_capacity'])
  tail=entry['role']=='principal_top2';improvement=1-cap/inc;competitor=cap/inc
  if tail and improvement>=F(1,100):material.append(name)
  if tail and competitor>F(21,20):miss.append(name)
  if comb==lo:exact.append(name)
  allrows.append(dict(name=name,role=entry['role'],capacity=str(cap),incumbent=str(inc),lower=str(lo),exact_capacity=comb==lo,standalone_gap= float(cap)/h-1,combined_gap=float(comb)/h-1,incumbent_gap=float(inc)/h-1,capacity_cut=float(improvement),exact_standalone_over_incumbent=str(competitor),ratio_upper=str(cap*cap/(2*ar*br))))
 assert exact==d['summary']['all']['exact_capacities']
 assert material==d['summary']['tail']['material_improvements']
 assert miss==d['summary']['tail']['exact5pct_obstructions']
 hko=json.loads((a.patterns/'patterns/upper-tail/conditional-coupling/hko-exact.json').read_text())
 Q5=module('owner_q5',a.patterns/'patterns/upper-tail/conditional-coupling/quadratic.py').Q5
 hh=rounding.check_reported(hko,d['hko']['rounding'],Q5.parse)
 assert d['hko']['exact_recovery'] and hh['Q']==Q5(1)/10
 def summary(rows):
  return dict(n=len(rows),material_cuts=sum(r['capacity_cut']>=.01-1e-14 for r in rows),improves=sum(F(r['capacity'])<F(r['incumbent']) for r in rows),regresses=sum(F(r['capacity'])>F(r['incumbent']) for r in rows),standalone_within1=sum(r['standalone_gap']<=.01+1e-10 for r in rows),standalone_within5=sum(r['standalone_gap']<=.05+1e-10 for r in rows),combined_within1=sum(r['combined_gap']<=.01+1e-10 for r in rows),combined_within5=sum(r['combined_gap']<=.05+1e-10 for r in rows),standalone_max_gap=max(r['standalone_gap'] for r in rows),combined_max_gap=max(r['combined_gap'] for r in rows),incumbent_max_gap=max(r['incumbent_gap'] for r in rows),numerical_agreements=sum(abs(r['standalone_gap'])<=1e-10 for r in rows),exact_certificates=sum(r['exact_capacity'] for r in rows))
 out=dict(status='PASS',at=datetime.datetime.now(datetime.timezone.utc).isoformat(),scope='Independent geometry/area joins, angular words,70 rational LP primal-dual systems,all conditional responses with Cramer closure enumeration and direct4D objectives, frozen membership and strongest-incumbent metrics; imports no producer or optimizer.',source_geometries=len(allrows),direct4D_candidates=count,LP_certificates=2*len(allrows),HKO_exact_recovery=True,HKO_direct4D_candidates=len(hh['all']),material_tail_cuts=material,exact_tail_misses_over5=miss,exact_capacities=exact,tail_metrics=summary([r for r in allrows if r['role']=='principal_top2']),bulk_metrics=summary([r for r in allrows if r['role']!='principal_top2']),rows=allrows,input_sha256={str(path):hashlib.sha256(path.read_bytes()).hexdigest() for path in [a.results,a.results.parent/'sample.json',source_path,Path(__file__)]})
 a.out.write_text(json.dumps(out,indent=2)+'\n');print(json.dumps({k:v for k,v in out.items() if k not in ['rows','input_sha256']},indent=2))
if __name__=='__main__':main()
