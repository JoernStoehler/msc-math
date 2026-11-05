#!/usr/bin/env python3
"""Independent standard-library replay; no producer imports or optimizer."""
import argparse,hashlib,importlib.util,json,sys
from fractions import Fraction as F
from itertools import combinations
from pathlib import Path

def dot(x,y):return sum(a*b for a,b in zip(x,y))
def det(u,v):return u[0]*v[1]-u[1]*v[0]
def solve(A,b):
 z=A[1][0]*0;M=[[v+z for v in x]+[y+z] for x,y in zip(A,b)];n=len(M)
 for k in range(n):
  pivot=next((i for i in range(k,n) if M[i][k]),None)
  if pivot is None:return None
  M[k],M[pivot]=M[pivot],M[k];s=M[k][k];M[k]=[x/s for x in M[k]]
  for i in range(n):
   if i!=k:
    s=M[i][k];M[i]=[x-s*y for x,y in zip(M[i],M[k])]
 return [x[-1] for x in M]
def vertices(a):
 # Basic feasible solutions of the three equality rows; two-support cases separately.
 out=[];n=len(a)
 for ids in combinations(range(n),3):
  v=solve([[1]*3,[a[i][0] for i in ids],[a[i][1] for i in ids]],[1,0,0])
  if v is None or min(v)<0:continue
  w=[a[0][0]*0]*n
  for i,x in zip(ids,v):w[i]=x
  if w not in out:out.append(w)
 for i,j in combinations(range(n),2):
  if det(a[i],a[j]) or dot(a[i],a[j])>=0:continue
  k=next(k for k in range(2) if a[i][k]);t=-a[j][k]/(a[i][k]-a[j][k]);w=[a[0][0]*0]*n;w[i]=t;w[j]=1-t
  if w not in out:out.append(w)
 return out

def feasible(a,alpha):
 assert len(a)==len(alpha) and min(alpha)>=0 and sum(alpha)==1
 assert all(sum(x*u[k] for x,u in zip(alpha,a))==0 for k in range(2))
def actual_Q(a,b,w,alpha,gamma):
 feasible(a,alpha);feasible(b,gamma);n=len(a);z=a[0][0]*0;v=[[*u,z,z] for u in a]+[[z,z,*u] for u in b];weights=[x/2 for x in alpha+gamma];assert len(w)==len(set(w)) and set(w).issubset(range(len(v))) and {i for i,x in enumerate(weights) if x}.issubset(w)
 return sum(weights[i]*weights[j]*(v[i][0]*v[j][2]+v[i][1]*v[j][3]-v[i][2]*v[j][0]-v[i][3]*v[j][1]) for k,i in enumerate(w) for j in w[k+1:])
def lp_check(a,b,v,parse):
 n=len(a);m=len(b);W=list(map(parse,v['W']));assert min(W)>=0 and sum(W)==1
 for i in range(n):assert all(sum(W[i*m+j]*b[j][k] for j in range(m))==0 for k in range(2))
 for j in range(m):assert all(sum(W[i*m+j]*a[i][k] for i in range(n))==0 for k in range(2))
 pos={u:i for i,u in enumerate(v['word'])};c=[dot(a[i],b[j])*(1 if pos[i]<pos[n+j] else -1)/4 for i in range(n) for j in range(m)];dual=list(map(parse,v['dual']));assert len(dual)==1+2*n+2*m
 for i in range(n):
  for j in range(m):
   slack=dual[0]+dot(dual[1+2*i:3+2*i],b[j])+dot(dual[1+2*n+2*j:3+2*n+2*j],a[i])-c[i*m+j];assert slack>=0 and slack==parse(v['dual_slack'][i*m+j])
 q=dot(W,c);assert q==dual[0]==parse(v['Q']);ar=[sum(W[i*m+j] for j in range(m)) for i in range(n)];br=[sum(W[i*m+j] for i in range(n)) for j in range(m)]
 return q,W==[x*y for x in ar for y in br]
def check_round(a,b,variants,r,parse):
 ca,cb=vertices(a),vertices(b);assert [len(ca),len(cb)]==r['closure_vertex_counts'];n=len(a);m=len(b);lookup={v['shadow_sign']:v for v in variants};expected=0
 for v in variants:
  W=list(map(parse,v['W']));expected+=sum(bool(sum(W[i*m+j] for j in range(m))) for i in range(n))+sum(bool(sum(W[i*m+j] for i in range(n))) for j in range(m))
 assert expected==r['candidate_count']==len(r['candidates']);seen=set();qs=[]
 for c in r['candidates']:
  key=(c['shadow_sign'],c['kind'],c['index']);assert key not in seen;seen.add(key);v=lookup[c['shadow_sign']];assert c['word']==v['word'];W=list(map(parse,v['W']));k=c['index'];vals=[W[k*m+j] for j in range(m)] if c['kind']=='row' else [W[i*m+k] for i in range(n)];mass=sum(vals);assert mass==parse(c['condition_mass']) and mass>0;fixed=[x/mass for x in vals];alpha=list(map(parse,c['alpha']));gamma=list(map(parse,c['gamma']));assert (gamma if c['kind']=='row' else alpha)==fixed
  q=actual_Q(a,b,c['word'],alpha,gamma);assert q==parse(c['Q']) and 1/(2*q)==parse(c['capacity'])
  best=max(actual_Q(a,b,c['word'],u,gamma) for u in ca) if c['kind']=='row' else max(actual_Q(a,b,c['word'],alpha,u) for u in cb);assert q==best;qs.append(q)
 assert parse(r['best']['Q'])==max(qs) and r['best'] in r['candidates'] and parse(r['capacity'])==1/(2*max(qs));return max(qs)
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def metrics(rows):
 import statistics
 out=dict(n=len(rows))
 for key in ['capacity','incumbent_capacity','combined_capacity']:
  gaps=[float(F(r[key]))/r['historical_capacity']-1 for r in rows]
  out[key]=dict(within1=sum(-1e-8<=x<=.01+1e-10 for x in gaps),within5=sum(-1e-8<=x<=.05+1e-10 for x in gaps),agreement=sum(abs(x)<1e-10 for x in gaps),below_label_tolerance=sum(x< -1e-8 for x in gaps),median_gap=statistics.median(gaps),max_gap=max(gaps),worst=rows[max(range(len(rows)),key=lambda i:gaps[i])]['name'])
 out.update(material_improvements=[r['name'] for r in rows if F(r['capacity'])<=F('0.99')*F(r['incumbent_capacity'])],strict_improvements=[r['name'] for r in rows if F(r['capacity'])<F(r['incumbent_capacity'])],regressions=[r['name'] for r in rows if F(r['capacity'])>F(r['incumbent_capacity'])],exact5pct_obstructions=[r['name'] for r in rows if F(r['capacity'])>F('1.05')*F(r['incumbent_capacity'])],exact_capacities=[r['name'] for r in rows if r['exact_capacity']],prior_exact_capacities=[r['name'] for r in rows if r['prior_exact_capacity']],new_exact_capacities=[r['name'] for r in rows if r['exact_capacity'] and not r['prior_exact_capacity']])
 return out

def ordered_word(a,b,sign):
 from functools import cmp_to_key
 vectors=a+[[-sign*x[1],sign*x[0]] for x in b]
 def cmp(i,j):
  u,v=vectors[i],vectors[j];hu=int(u[1]<0 or(u[1]==0 and u[0]<0));hv=int(v[1]<0 or(v[1]==0 and v[0]<0))
  if hu!=hv:return hu-hv
  cross=det(u,v)
  return (-1 if cross>0 else 1) if cross else (i>j)-(i<j)
 word=sorted(range(len(vectors)),key=cmp_to_key(cmp));return word if sign>0 else word[::-1]

def polygon_area(normals):
 pts=[]
 for u,v in combinations(normals,2):
  determinant=det(u,v)
  if not determinant:continue
  x=((v[1]-u[1])/determinant,(u[0]-v[0])/determinant)
  if all(dot(a,x)<=1 for a in normals):pts.append(x)
 pts=sorted(set(pts));assert len(pts)>=3
 def turn(o,a,b):return det((a[0]-o[0],a[1]-o[1]),(b[0]-o[0],b[1]-o[1]))
 def chain(xs):
  h=[]
  for x in xs:
   while len(h)>1 and turn(h[-2],h[-1],x)<=0:h.pop()
   h.append(x)
  return h
 hull=chain(pts)[:-1]+chain(pts[::-1])[:-1]
 area=sum(det(x,y) for x,y in zip(hull,hull[1:]+hull[:1]))/2
 assert area>0;return area

def main():
 p=argparse.ArgumentParser();p.add_argument('--repo',type=Path,required=True);p.add_argument('--directory',type=Path,required=True);args=p.parse_args();root=args.repo;d=args.directory;out=json.loads((d/'results.json').read_text());sample=json.loads((d/'sample.json').read_text())
 for key,path in [('plan_sha256','PLAN.md'),('sample_sha256','sample.json'),('analyze_sha256','analyze.py')]:assert out[key]==sha(d/path)
 for path,digest in out['sources'].items():assert sha(root/path)==digest
 assert out['sources']==sample['sources'] and not out['failures']
 prior_sample=json.loads((root/sample['parent_panel']).read_text());expected=[r for r in prior_sample['sample'] if r['name'] not in sample['conditional_design_excluded']];assert expected==sample['sample'] and len(expected)==35
 assert sample['conditional_design_excluded']==['random_5x5_s1_64','random_5x6_s1_198','random_6x6_s3_124']
 prior={r['name']:r for r in json.loads((root/'patterns/upper-tail/full-partner-validation/results.json').read_text())['rows']};checks=[];responses=0
 assert [r['name'] for r in out['rows']]==[r['name'] for r in expected]
 for member,r in zip(expected,out['rows']):
  s=prior[r['name']];assert all(r[k]==v for k,v in member.items());a=[list(map(F,u)) for u in r['A']];b=[list(map(F,u)) for u in r['B']];vs=[list(map(F,u)) for u in s['geometry']];assert a==[u[:2] for u in vs[:s['k']]] and b==[u[2:] for u in vs[s['k']:]]
  for v in r['variants']:assert v['word']==ordered_word(a,b,v['shadow_sign'])
  lps=[lp_check(a,b,v,F) for v in r['variants']];q=check_round(a,b,r['variants'],r['rounding'],F);cap=1/(2*q);lower=1/(2*max(x[0] for x in lps));inc=min(F(s['capacity']),F(s['incumbent_capacity']));combined=min(cap,inc)
  assert F(r['capacity'])==cap==F(r['rounding']['capacity']) and F(r['incumbent_capacity'])==inc and F(r['combined_capacity'])==combined and F(r['capacity_lower'])==lower
  assert lower<=combined and r['exact_capacity']==(lower==combined) and r['prior_exact_capacity']==(inc==lower)
  assert F(r['combined_interval_factor'])==combined/lower and F(r['prior_interval_factor'])==inc/lower and F(r['standalone_over_incumbent'])==cap/inc
  assert r['historical_capacity']==s['historical_capacity'] and r['historical_ratio']==s['historical_ratio']
  area=polygon_area(a)*polygon_area(b);assert area==F(s['selection']['area_A'])*F(s['selection']['area_B'])==F(r['area_product'])
  # The strongest incumbent is a retained feasible witness; verify its full-partner branch here.
  if inc==F(s['capacity']):
   w=s['selection']['chosen'];assert actual_Q(a,b,w['word'],list(map(F,w['alpha'])),list(map(F,w['gamma'])))==1/(2*inc)
  checks.append(dict(name=r['name'],candidate_count=r['rounding']['candidate_count'],rank_one_by_shadow=[x[1] for x in lps],exact_capacity=combined==lower,prior_exact_capacity=inc==lower,gain=1-float(cap/inc)));responses+=r['rounding']['candidate_count']
 path=root/'patterns/upper-tail/conditional-coupling/quadratic.py';spec=importlib.util.spec_from_file_location('verifier_quadratic',path);qm=importlib.util.module_from_spec(spec);spec.loader.exec_module(qm);Q5=qm.Q5;h=json.loads((root/'patterns/upper-tail/conditional-coupling/hko-exact.json').read_text());a=[list(map(Q5.parse,u)) for u in h['A']];b=[list(map(Q5.parse,u)) for u in h['B']];hlp=[lp_check(a,b,v,Q5.parse) for v in h['variants']];hq=check_round(a,b,h['variants'],out['hko']['rounding'],Q5.parse);assert hq==Q5(1)/10 and out['hko']['exact_recovery']
 summary={tag:metrics(rs) for tag,rs in [('all',out['rows']),('tail',[r for r in out['rows'] if r['role']=='principal_top2']),('bulk',[r for r in out['rows'] if r['role']!='principal_top2'])]};summary['family_tail']={bucket:metrics([r for r in out['rows'] if r['bucket']==bucket and r['role']=='principal_top2']) for bucket in ['5x5','5x6','6x6']};summary['material_positive']=bool(summary['tail']['material_improvements']);summary['standalone5pct_bucket']=summary['tail']['capacity']['within5']==26;assert summary==out['summary']
 result=dict(status='PASS',rational_controls=35,rational_LP_certificates=70,conditional_responses_checked=responses,HKO_exact_Q='1/10',HKO_algebraic_LP_certificates=2,complete_design_excluded_membership=True,source_normal_areas_reconstructed=35,all_metrics_recomputed=True,checks=checks,summary=summary,input_sha256={p:sha(d/p) for p in ['PLAN.md','sample.json','analyze.py','results.json','verify.py']},limits='Checks retained-incumbent identity, full-partner incumbent witnesses when winning, all new LP and rounded witnesses; receiving owner separately verifies all incumbent geometry/feasibility from pinned source.')
 (d/'verification.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({k:v for k,v in result.items() if k not in ['checks','summary']},indent=2))
if __name__=='__main__':main()
