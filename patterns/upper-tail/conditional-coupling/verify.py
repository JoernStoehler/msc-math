#!/usr/bin/env python3
"""Independent exact certificate replay: standard library, no optimizer imports."""
import argparse,hashlib,json
from fractions import Fraction as F
from functools import cmp_to_key
from pathlib import Path
from quadratic import Q5

def dot(a,b):return sum(x*y for x,y in zip(a,b))
def ordered_word(a,b,sign):
 v=a+[[-sign*x[1],sign*x[0]] for x in b]
 def cmp(i,j):
  u,w=v[i],v[j];hu=0 if u[1]>0 or (u[1]==0 and u[0]>=0) else 1;hw=0 if w[1]>0 or (w[1]==0 and w[0]>=0) else 1
  if hu!=hw:return hu-hw
  c=u[0]*w[1]-u[1]*w[0]
  return (-1 if c>0 else 1) if c else (i>j)-(i<j)
 word=sorted(range(len(v)),key=cmp_to_key(cmp));return word if sign>0 else word[::-1]

def check_variant(a,b,v,parse):
 n=len(a);m=len(b);N=n*m;word=v['word'];assert word==ordered_word(a,b,v['shadow_sign'])
 position={j:i for i,j in enumerate(word)};c=[dot(a[i],b[j])*(1 if position[i]<position[n+j] else -1)/4 for i in range(n) for j in range(m)]
 assert c==list(map(parse,v['objective']))
 z=a[0][0]*0;A=[[z+1]*N];rhs=[z+1]
 for i in range(n):
  for k in range(2):A.append([b[j][k] if ii==i else z for ii in range(n) for j in range(m)]);rhs.append(z)
 for j in range(m):
  for k in range(2):A.append([a[i][k] if jj==j else z for i in range(n) for jj in range(m)]);rhs.append(z)
 assert A==[list(map(parse,row)) for row in v['A_eq']] and rhs==list(map(parse,v['b_eq']))
 x=list(map(parse,v['W']));y=list(map(parse,v['dual']));assert len(x)==N and len(y)==len(A)
 assert min(x)>=0 and all(dot(row,x)==v for row,v in zip(A,rhs))
 slack=[sum(row[j]*yi for row,yi in zip(A,y))-c[j] for j in range(N)];assert min(slack)>=0;assert slack==list(map(parse,v['dual_slack']))
 q=dot(c,x);assert q==dot(rhs,y)==parse(v['Q']);assert 1/(2*q)==parse(v['capacity_lower'])
 assert all(xi*si==0 for xi,si in zip(x,slack))
 return q

def check_upper(a,b,w,capacity):
 alpha=list(map(F,w['alpha']));gamma=list(map(F,w['gamma']));assert min(alpha+gamma)>=0 and sum(alpha)==sum(gamma)==1
 assert all(sum(x*v[k] for x,v in zip(alpha,a))==0 and sum(x*v[k] for x,v in zip(gamma,b))==0 for k in range(2))
 n=len(a);vertices=[[*u,F(0),F(0)] for u in a]+[[F(0),F(0),*v] for v in b];weights=[x/2 for x in alpha+gamma];word=w['word'];assert len(word)==len(set(word))
 assert set(i for i,x in enumerate(weights) if x).issubset(set(word))
 q=F(0)
 for k,i in enumerate(word):
  for j in word[k+1:]:
   u,v=vertices[i],vertices[j];omega=u[0]*v[2]+u[1]*v[3]-u[2]*v[0]-u[3]*v[1];q+=weights[i]*weights[j]*omega
 assert q>0 and 1/(2*q)==capacity
 return q

def main():
 p=argparse.ArgumentParser();p.add_argument('--directory',type=Path,required=True);p.add_argument('--repo',type=Path);args=p.parse_args();d=args.directory
 data=json.loads((d/'results.json').read_text());rows=[]
 assert data['plan_sha256']==hashlib.sha256((d/'PLAN.md').read_bytes()).hexdigest()
 assert data['analyze_sha256']==hashlib.sha256((d/'analyze.py').read_bytes()).hexdigest()
 if args.repo:
  for path,sha in data['sources'].items():assert hashlib.sha256((args.repo/path).read_bytes()).hexdigest()==sha
 for r in data['rows']:
  a=[list(map(F,v)) for v in r['A']];b=[list(map(F,v)) for v in r['B']];q=max(check_variant(a,b,v,F) for v in r['variants']);lower=1/(2*q);upper=F(r['feasible_capacity_upper']);wq=check_upper(a,b,r['feasible_upper_witness'],upper)
  assert q>=wq and lower==F(r['capacity_lower']) and upper/lower==F(r['capacity_interval_factor']);assert r['exact_capacity']==(q==wq)
  rows.append(dict(name=r['name'],exact_capacity=q==wq,capacity_lower=float(lower),capacity_upper=float(upper),interval_factor=float(upper/lower)))
 hko=json.loads((d/'hko-exact.json').read_text());assert hko['algebraic_sha256']==hashlib.sha256((d/'algebraic.py').read_bytes()).hexdigest();a=[list(map(Q5.parse,v)) for v in hko['A']];b=[list(map(Q5.parse,v)) for v in hko['B']];q=max(check_variant(a,b,v,Q5.parse) for v in hko['variants']);s=Q5(0,1)
 assert q==Q5(1,0)/20+3*s/100 and q/Q5.parse(hko['Q_true'])==(1+3/s)/2
 assert Q5.parse(hko['capacity_lower_over_true'])==(3*s-5)/2
 cw=list(map(Q5.parse,hko['circulant_weights']));cx=list(map(Q5.parse,hko['circulant_W']));assert cw==[Q5(),1/s,Q5(),(1-1/s)/2,(1-1/s)/2] and cx==[cw[(j-i)%5]/5 for i in range(5) for j in range(5)]
 for v in hko['variants']:
  assert min(cx)>=0 and all(dot(list(map(Q5.parse,row)),cx)==Q5.parse(rhs) for row,rhs in zip(v['A_eq'],v['b_eq']))
  assert dot(list(map(Q5.parse,v['objective'])),cx)==Q5.parse(v['Q'])
 result=dict(status='PASS',rational_LP_certificates=6,algebraic_LP_certificates=2,direct_four_dimensional_feasible_witnesses=3,rows=rows,HKO_LP_Q_over_true=float(q/Q5.parse(hko['Q_true'])),HKO_capacity_lower_over_true=float((3*s-5)/2),source_hashes_checked=bool(args.repo),input_sha256={p:hashlib.sha256((d/p).read_bytes()).hexdigest() for p in ['results.json','hko-exact.json','verify.py','quadratic.py']})
 (d/'verification.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
if __name__=='__main__':main()
