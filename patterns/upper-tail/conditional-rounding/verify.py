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
def main():
 p=argparse.ArgumentParser();p.add_argument('--repo',type=Path,required=True);p.add_argument('--directory',type=Path,required=True);args=p.parse_args();root=args.repo;d=args.directory
 path=root/'patterns/upper-tail/conditional-coupling/quadratic.py';sp=importlib.util.spec_from_file_location('verifyQ5',path);qm=importlib.util.module_from_spec(sp);sp.loader.exec_module(qm);Q5=qm.Q5
 out=json.loads((d/'results.json').read_text());assert out['plan_sha256']==hashlib.sha256((d/'PLAN.md').read_bytes()).hexdigest();assert out['analyze_sha256']==hashlib.sha256((d/'analyze.py').read_bytes()).hexdigest()
 for path,sha in out['sources'].items():assert hashlib.sha256((root/path).read_bytes()).hexdigest()==sha
 original=json.loads((root/'patterns/upper-tail/conditional-coupling/results.json').read_text());checks=[]
 for s,r in zip(original['rows'],out['rows']):
  assert s['name']==r['name'];a=[list(map(F,u)) for u in s['A']];b=[list(map(F,u)) for u in s['B']];lps=[lp_check(a,b,v,F) for v in s['variants']];q=check_round(a,b,s['variants'],r['rounding'],F);lower=1/(2*max(v[0] for v in lps));inc=F(s['feasible_capacity_upper']);w=s['feasible_upper_witness'];assert actual_Q(a,b,w['word'],list(map(F,w['alpha'])),list(map(F,w['gamma'])))==1/(2*inc);combined=min(inc,1/(2*q));assert lower==F(r['capacity_lower']) and combined==F(r['combined_capacity']) and r['exact_capacity']==(lower==combined);checks.append(dict(name=r['name'],rank_one_by_shadow=[v[1] for v in lps],exact_capacity=lower==combined,capacity=float(combined),gain=1-float(combined/inc),candidate_count=r['rounding']['candidate_count']))
 h=json.loads((root/'patterns/upper-tail/conditional-coupling/hko-exact.json').read_text());a=[list(map(Q5.parse,u)) for u in h['A']];b=[list(map(Q5.parse,u)) for u in h['B']];hlps=[lp_check(a,b,v,Q5.parse) for v in h['variants']];q=check_round(a,b,h['variants'],out['hko'],Q5.parse);assert q==Q5(1)/10
 # Structural control: the explicit circulant optimum's five conditional rows are exactly all B closure vertices.
 circ=list(map(Q5.parse,h['circulant_W']));conditional=[[5*circ[5*i+j] for j in range(5)] for i in range(5)];bv=vertices(b);assert len(bv)==5 and all(x in bv for x in conditional) and all(x in conditional for x in bv)
 result=dict(status='PASS',rational_controls=checks,HKO_exact_Q='1/10',HKO_retained_couplings_rank_one=[v[1] for v in hlps],HKO_circulant_conditionals_cover_all_5_closure_vertices=True,numerical_regular_controls=[dict(theta=r['theta'],capacity_over_known=r['standalone_over_known']) for r in out['regular']],input_sha256={p:hashlib.sha256((d/p).read_bytes()).hexdigest() for p in ['PLAN.md','analyze.py','results.json','verify.py']})
 (d/'verification.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
if __name__=='__main__':main()
