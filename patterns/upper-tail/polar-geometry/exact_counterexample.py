#!/usr/bin/env python3
"""Stdlib exact normalized-polar-alpha collision above one.
Independent all-cyclic-word product closure enumeration for the retained input.
Capacity coverage imports the existing product six-facet reduction, not a new theorem.
"""
from fractions import Fraction as F
from itertools import combinations,permutations
from functools import cmp_to_key
from pathlib import Path
import argparse,hashlib,json,math,time
NAME='contact_full_s3_e3of100'
def dot(a,b):return sum(x*y for x,y in zip(a,b))
def cross(a,b):return a[0]*b[1]-a[1]*b[0]
def vertices(a):
 vv=[]
 for i,j in combinations(range(len(a)),2):
  d=cross(a[i],a[j])
  if not d:continue
  v=((a[j][1]-a[i][1])/d,(a[i][0]-a[j][0])/d)
  if all(dot(z,v)<=1 for z in a):vv.append(v)
 vv=list(set(vv));g=tuple(sum(v[k] for v in vv)/len(vv) for k in range(2))
 def order(v,w):
  v=(v[0]-g[0],v[1]-g[1]);w=(w[0]-g[0],w[1]-g[1]);hv=v[1]>0 or v[1]==0 and v[0]>=0;hw=w[1]>0 or w[1]==0 and w[0]>=0
  if hv!=hw:return -1 if hv else 1
  c=cross(v,w);return -1 if c>0 else 1 if c<0 else 0
 vv.sort(key=cmp_to_key(order));return vv
def area_centroid(a):
 vv=vertices(a);cv=[cross(vv[i],vv[(i+1)%len(vv)]) for i in range(len(vv))];area=sum(cv)/2;assert area>0
 g=tuple(sum((vv[i][k]+vv[(i+1)%len(vv)][k])*cv[i] for i in range(len(vv)))/(6*area) for k in range(2));assert all(1-dot(u,g)>0 for u in a)
 assert len(vv)==len(a) and all(sum(dot(u,v)==1 for v in vv)==2 for u in a)
 return area,g,vv

def closures(a):
 out=[]
 for i,j in combinations(range(len(a)),2):
  if cross(a[i],a[j])==0 and dot(a[i],a[j])<0:
   k=next(k for k in range(2) if a[i][k]);w=-a[j][k]/(a[i][k]-a[j][k]);out.append(((i,j),(w,1-w)))
 for ids in combinations(range(len(a)),3):
  aa=[a[i] for i in ids];w=[cross(aa[1],aa[2]),cross(aa[2],aa[0]),cross(aa[0],aa[1])];s=sum(w)
  if s and all(z/s>0 for z in w):out.append((ids,tuple(z/s for z in w)))
 for ids,ws in out:assert all(sum(w*a[i][k] for i,w in zip(ids,ws))==0 for k in range(2))
 return out

def capacity(a,b):
 best=F(0);winner=None;evals=0
 ca=closures(a);cb=closures(b)
 for ii,wa in ca:
  for jj,wb in cb:
   n=len(ii);m=len(jj);terms=[[wa[i]*wb[j]*dot(a[ai],b[bj])/4 for j,bj in enumerate(jj)] for i,ai in enumerate(ii)]
   for tail in permutations(range(1,n+m)):
    word=(0,)+tail;pos={z:k for k,z in enumerate(word)};Q=sum(terms[i][j]*(1 if pos[i]<pos[n+j] else -1) for i in range(n) for j in range(m));evals+=1
    if Q>best:best=Q;winner=dict(A_support=ii,B_support=jj,A_weights=wa,B_weights=wb,cyclic_word=word,Q=Q)
 return 1/(2*best),winner,len(ca),len(cb),evals

def stringify(x):
 if isinstance(x,F):return str(x)
 if isinstance(x,dict):return {k:stringify(v) for k,v in x.items()}
 if isinstance(x,(tuple,list)):return [stringify(z) for z in x]
 return x

def main():
 ap=argparse.ArgumentParser();ap.add_argument('--sample',type=Path);ap.add_argument('--truth',type=Path);ap.add_argument('--certificate',type=Path);ap.add_argument('--name',default=NAME);ap.add_argument('--out',type=Path,required=True);z=ap.parse_args();start=time.perf_counter();
 if z.certificate:
  old=json.loads(z.certificate.read_text());r=old['body'];t=dict(volume=old['volume'],capacity=old['independently_enumerated_capacity'],ratio=old['nonregular_ratio']);name=old['name'];sample_sha=old['sample_sha256'];truth_sha=old['truth_sha256']
 else:
  sample=json.loads(z.sample.read_text());truth=json.loads(z.truth.read_text());r=next(r for r in sample['rows'] if r['name']==z.name);t=next(r for r in truth['rows'] if r['name']==z.name);name=z.name;sample_sha=hashlib.sha256(z.sample.read_bytes()).hexdigest();truth_sha=hashlib.sha256(z.truth.read_bytes()).hexdigest()
 a=[tuple(map(F,u)) for u in r['A']];b=[tuple(map(F,u)) for u in r['B']];AA,ga,va=area_centroid(a);AB,gb,vb=area_centroid(b);aa=[tuple(v/(1-dot(u,ga)) for v in u) for u in a];bb=[tuple(v/(1-dot(u,gb)) for v in u) for u in b];pairs=[(abs(dot(u,v)),i,j) for i,u in enumerate(aa) for j,v in enumerate(bb)];alpha,i,j=max(pairs);V=AA*AB;x2=alpha*alpha*V;regular=25/(2*x2);cap,witness,na,nb,evals=capacity(a,b);ratio=cap*cap/(2*V);assert V==F(t['volume']);assert cap==F(t['capacity']);assert ratio==F(t['ratio']);assert F(49,50)<regular<F(26,25);assert 1<ratio<F(26,25);assert ratio-regular>F(1,100)
 out=dict(name=name,body=dict(A=a,B=b),sample_sha256=sample_sha,truth_sha256=truth_sha,areaA=AA,areaB=AB,centroidA=ga,centroidB=gb,centered_polarA=aa,centered_polarB=bb,alpha=alpha,alpha_attaining_pair=[i,j],volume=V,x_squared=x2,independently_enumerated_capacity=cap,capacity_witness=witness,closure_counts=[na,nb],cyclic_word_evaluations=evals,nonregular_ratio=ratio,same_alpha_root_volume_regular_ratio=regular,ratio_gap=ratio-regular,checks=dict(independent_capacity_matches_oracle=True,nonregular_ratio_above_one=ratio>1,regular_ratio_above_one=regular>1,regular_ratio_below_one=regular<1,regular_ratio_above_49_over_50=regular>F(49,50),both_below_26_over_25=max(ratio,regular)<F(26,25),gap_exceeds_one_hundredth=ratio-regular>F(1,100)),decimals=dict(alpha=float(alpha),x_squared=float(x2),nonregular_ratio=float(ratio),regular_ratio=float(regular),gap=float(ratio-regular)),elapsed_seconds=time.perf_counter()-start);z.out.write_text(json.dumps(stringify(out),indent=2)+'\n');print(json.dumps(out['decimals']|out['checks']|{'evaluations':evals,'elapsed_seconds':out['elapsed_seconds']},indent=2))
if __name__=='__main__':main()
