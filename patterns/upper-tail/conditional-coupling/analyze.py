#!/usr/bin/env python3
"""Six frozen conditional-coupling pilots; no closure support-pair enumeration."""
import argparse, hashlib, importlib.util, json, math, time
from fractions import Fraction as F
from pathlib import Path
import numpy as np
from scipy.optimize import linprog

def dot(a,b): return sum(x*y for x,y in zip(a,b))
def stringify(x):
 if hasattr(x,'serial'):return x.serial()
 if isinstance(x,F):return str(x)
 if isinstance(x,dict):return {k:stringify(v) for k,v in x.items()}
 if isinstance(x,(tuple,list)):return [stringify(v) for v in x]
 return x

def independent_rows(rows):
 basis={};ids=[]
 for i,row in enumerate(rows):
  v=list(row)
  for p,u in sorted(basis.items()):
   q=v[p]
   if q:v=[x-q*y for x,y in zip(v,u)]
  p=next((j for j,x in enumerate(v) if x),None)
  if p is not None:
   q=v[p];basis[p]=[x/q for x in v];ids.append(i)
 return ids

def solve_square(A,b):
 a=[list(row)+[v] for row,v in zip(A,b)];n=len(a)
 for j in range(n):
  i=next(i for i in range(j,n) if a[i][j]);a[j],a[i]=a[i],a[j]
  q=a[j][j];a[j]=[x/q for x in a[j]]
  for i in range(n):
   if i!=j:
    q=a[i][j]
    if q:a[i]=[x-q*y for x,y in zip(a[i],a[j])]
 return [row[-1] for row in a]

def matrix(a,b):
 n=len(a);m=len(b);z=a[0][0]*0;one=z+1
 rows=[[one]*(n*m)];rhs=[one]
 for i in range(n):
  for k in range(2): rows.append([b[j][k] if ii==i else z for ii in range(n) for j in range(m)]);rhs.append(z)
 for j in range(m):
  for k in range(2): rows.append([a[i][k] if jj==j else z for i in range(n) for jj in range(m)]);rhs.append(z)
 return rows,rhs

def pilot(a,b,word,exact):
 n=len(a);m=len(b);pos={i:p for p,i in enumerate(word)}
 c=[dot(a[i],b[j])*(1 if pos[i]<pos[n+j] else -1)/4 for i in range(n) for j in range(m)]
 A,rhs=matrix(a,b);ids=independent_rows(A) if exact else None
 Ar=[A[i] for i in ids] if exact else A;br=[rhs[i] for i in ids] if exact else rhs
 result=linprog(-np.array(c,dtype=float),A_eq=np.array(Ar,dtype=float),b_eq=np.array(br,dtype=float),bounds=(0,None),method='highs',options={'dual_feasibility_tolerance':1e-9,'primal_feasibility_tolerance':1e-9})
 assert result.success,result.message
 out=dict(word=word,objective=c,A_eq=A,b_eq=rhs,numerical_Q=-result.fun,numerical_W=result.x.tolist(),status=result.message)
 if exact:
  active=sorted(range(n*m),key=lambda j:(result.x[j]<=1e-9,abs(result.lower.marginals[j]),j))
  active=[j for j in active if result.x[j]>1e-9 or abs(result.lower.marginals[j])<1e-7]
  chosen=independent_rows([[row[j] for row in Ar] for j in active]);basis=[active[i] for i in chosen];r=len(Ar)
  AB=[[row[j] for j in basis] for row in Ar];primal_rows=independent_rows(AB);xb=solve_square([AB[i] for i in primal_rows],[br[i] for i in primal_rows])
  x=[F(0)]*(n*m)
  for j,v in zip(basis,xb):x[j]=v
  dual_equations=list(map(list,zip(*AB)));dual_rhs=[c[j] for j in basis];free_pins=[]
  for k in range(r):
   unit=[F(int(j==k)) for j in range(r)]
   if len(independent_rows(dual_equations+[unit]))>len(dual_equations):
    pin=F(float(-result.eqlin.marginals[k])).limit_denominator(10**6);dual_equations.append(unit);dual_rhs.append(pin);free_pins.append([k,pin])
   if len(dual_equations)==r:break
  y=solve_square(dual_equations,dual_rhs);yf=[F(0)]*len(A)
  for i,v in zip(ids,y):yf[i]=v
  slack=[sum(row[j]*v for row,v in zip(A,yf))-c[j] for j in range(n*m)]
  assert min(x)>=0 and min(slack)>=0
  assert all(dot(row,x)==v for row,v in zip(A,rhs))
  q=dot(c,x);assert q==dot(rhs,yf)
  assert abs(float(q)+result.fun)<1e-8
  out.update(independent_rows=ids,basis=basis,primal_rows=primal_rows,free_dual_pins=free_pins,W=x,dual=yf,dual_slack=slack,Q=q,capacity_lower=1/(2*q))
 return out

def main():
 p=argparse.ArgumentParser();p.add_argument('--repo',type=Path,required=True);p.add_argument('--out',type=Path,required=True);args=p.parse_args();start=time.perf_counter()
 root=args.repo;sp=importlib.util.spec_from_file_location('shadow',root/'patterns/upper-tail/signed-prefix-response/analyze.py');shadow=importlib.util.module_from_spec(sp);sp.loader.exec_module(shadow)
 sources=['patterns/upper-tail/full-partner-containment/results.json','patterns/upper-tail/full-partner-containment/comparison.json','patterns/upper-tail/full-partner-validation/results.json'];d=[json.loads((root/v).read_text()) for v in sources]
 rows=[]
 for name in ['random_5x5_s1_64','random_5x6_s1_198','random_6x6_s3_124']:
  source=next((r for dataset in [d[0]['rows'],d[2]['rows']] for r in dataset if r['name']==name));v=[list(map(F,u)) for u in source['geometry']];k=source['k'];a=[u[:2] for u in v[:k]];b=[u[2:] for u in v[k:]]
  if 'incumbent_capacity' in source:
   upper=min(F(source['capacity']),F(source['incumbent_capacity']));upper_source=sources[2]
   contact=source['comparators']['contact']['best'];alpha=[F(0)]*len(a);gamma=[F(0)]*len(b);si,sj=contact['supports']
   for i,w in zip(si,contact['weights'][:len(si)]):alpha[i]=2*F(w)
   for j,w in zip(sj,contact['weights'][len(si):]):gamma[j]=2*F(w)
   witness=dict(alpha=alpha,gamma=gamma,word=[si[i] if i<len(si) else len(a)+sj[i-len(si)] for i in contact['word']],capacity=F(contact['capacity']))
   assert witness['capacity']==upper
  else:
   comp=next(r for r in d[1] if r['name']==name);upper=min(F(comp['new_capacity']),F(comp['incumbent_capacity']));upper_source=sources[1];witness=source['selection']['chosen']
   assert F(witness['capacity'])==upper
  variants=[]
  for sign in [1,-1]:
   word=shadow.shadow_word(a,b,sign)
   if sign<0:word=word[::-1]
   variants.append(pilot(a,b,word,True)|{'shadow_sign':sign})
  q=max(r['Q'] for r in variants);lower=1/(2*q)
  row=dict(name=name,A=a,B=b,variants=variants,Q_LP=q,capacity_lower=lower,feasible_capacity_upper=upper,feasible_upper_source=upper_source,feasible_upper_witness=witness,capacity_interval_factor=upper/lower,exact_capacity=lower==upper,historical_capacity=source['historical_capacity']);rows.append(row)
  print(name,'LPQ',float(q),'c interval',float(lower),float(upper),'factor',float(upper/lower),flush=True)
 regular=[];h=math.cos(math.pi/5)
 for theta in [0,math.pi/20,math.pi/10]:
  a=[(math.cos((2*i+1)*math.pi/5)/h,math.sin((2*i+1)*math.pi/5)/h) for i in range(5)];b=[(math.cos((2*i+1)*math.pi/5+theta)/h,math.sin((2*i+1)*math.pi/5+theta)/h) for i in range(5)];variants=[]
  for sign in [1,-1]:
   word=shadow.shadow_word(a,b,sign,1e-11)
   if sign<0:word=word[::-1]
   variants.append(pilot(a,b,word,False)|{'shadow_sign':sign})
  known=(1+h)**2/math.cos(theta);q=max(r['numerical_Q'] for r in variants);regular.append(dict(theta=theta,A=a,B=b,variants=variants,known_capacity=known,LP_capacity_lower=1/(2*q),LP_Q_over_known_Q=2*q*known));print('regular',theta,'Qlp/Qtrue',2*q*known,flush=True)
 result=dict(protocol='Frozen six-pilot conditional coupling LP; rational active-basis certificates; numerical regular controls only',sources={p:hashlib.sha256((root/p).read_bytes()).hexdigest() for p in sources},rows=rows,regular=regular,elapsed_seconds=time.perf_counter()-start,plan_sha256=hashlib.sha256(Path(__file__).with_name('PLAN.md').read_bytes()).hexdigest(),analyze_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest());args.out.write_text(json.dumps(stringify(result),indent=2)+'\n')
if __name__=='__main__':main()
