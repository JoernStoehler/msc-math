#!/usr/bin/env python3
"""Independent canonical-gauge reconstruction and all-word exact obstruction.
Imports neither canonical.py nor global-counterexample.py.
"""
import argparse,hashlib,importlib.util,json,sys
from fractions import Fraction as F
from functools import cmp_to_key
from itertools import combinations,permutations
from pathlib import Path

def module(n,p):
 s=importlib.util.spec_from_file_location(n,p);m=importlib.util.module_from_spec(s);sys.modules[n]=m;s.loader.exec_module(m);return m

def dot(u,v):return sum(x*y for x,y in zip(u,v))
def det(u,v):return u[0]*v[1]-u[1]*v[0]
def solve(A,b):
 M=[list(row)+[x] for row,x in zip(A,b)];n=len(M)
 for j in range(n):
  k=next(i for i in range(j,n) if M[i][j]);M[j],M[k]=M[k],M[j];s=M[j][j];M[j]=[x/s for x in M[j]]
  for i in range(n):
   if i!=j:
    s=M[i][j];M[i]=[x-s*y for x,y in zip(M[i],M[j])]
 return [r[-1] for r in M]
def angular(a):
 def cmp(i,j):
  u,v=a[i],a[j];hu=int(u[1]<0 or(u[1]==0 and u[0]<0));hv=int(v[1]<0 or(v[1]==0 and v[0]<0))
  if hu!=hv:return hu-hv
  c=det(u,v);return (-1 if c>0 else 1) if c else 0
 return sorted(range(len(a)),key=cmp_to_key(cmp))
def polygon(a):return [solve([a[i],a[(i+1)%len(a)]],[a[0][0]*0+1,a[0][0]*0+1]) for i in range(len(a))]
def center(a):
 vv=polygon(a);cross=[det(vv[i],vv[(i+1)%len(vv)]) for i in range(len(vv))];return [sum((vv[i][c]+vv[(i+1)%len(vv)][c])*cross[i] for i in range(len(vv)))/(3*sum(cross)) for c in range(2)]
def canon(a,b,refA,refB):
 pv=polygon(a);rv=polygon(refA);coef=[[pv[i][0],pv[i][1],pv[i][0]*0+1] for i in [0,2,4]];affine=[solve(coef,[rv[i][c] for i in [0,2,4]]) for c in range(2)];L=[row[:2] for row in affine];t=[row[2] for row in affine];Lt=list(map(list,zip(*L)));aa=[]
 for u in a:
  raw=solve(Lt,u);den=1+dot(raw,t);assert den>0;aa.append([x/den for x in raw])
 bb=[[dot(row,u) for row in L] for u in b];cen=center(bb);bb=[[x/(1-dot(u,cen)) for x in u] for u in bb];sc=bb[0][1]/refB[0][1]
 if sc<=0:return None
 return aa,[[x/sc for x in u] for u in bb]
def closures(a):
 n=len(a);out=[];zero=a[0][0]*0
 for ids in combinations(range(n),2):
  x,y=[a[i] for i in ids]
  if det(x,y) or dot(x,y)>=0:continue
  c=next(c for c in range(2) if x[c]);w=-y[c]/(x[c]-y[c]);v=[zero]*n;v[ids[0]]=w;v[ids[1]]=1-w;out.append(v)
 for ids in combinations(range(n),3):
  try:w=solve([[zero+1]*3,[a[i][0] for i in ids],[a[i][1] for i in ids]],[zero+1,zero,zero])
  except StopIteration:continue
  if min(w)<0:continue
  v=[zero]*n
  for i,x in zip(ids,w):v[i]=x
  if v not in out:out.append(v)
 return out

def main():
 p=argparse.ArgumentParser();p.add_argument('--repo',type=Path,required=True);p.add_argument('--directory',type=Path,required=True);o=p.parse_args();d=o.directory;Q=module('global_verify_q5',o.repo/'patterns/upper-tail/conditional-coupling/quadratic.py').Q5;pr=json.loads((d/'predictions.json').read_text());src=json.loads((d/'global-counterexample-input.json').read_text());cert=json.loads((d/'global-counterexample.json').read_text());ref={k:[[Q.parse(x) for x in u] for u in pr['base'][k]] for k in ['A','B']};gs=[[Q.parse(x) for x in g['loss_gradient']] for g in pr['bank']];source={k:[[Q(x) for x in u] for u in src[k]] for k in ['A','B']};a=[source['A'][i] for i in angular(source['A'])];b=[source['B'][i] for i in angular(source['B'])];gauges=[];rowindex=0;floor=Q(4)/5
 for swap in [False,True]:
  x,y=(b,a) if swap else(a,b)
  for reverse in [False,True]:
   u,v=(x[::-1],y[::-1]) if reverse else(x,y)
   for ia in range(5):
    for ib in range(5):
     val=canon(u[ia:]+u[:ia],v[ib:]+v[:ib],ref['A'],ref['B'])
     if val is None:continue
     retained=cert['all_gauges'][rowindex];assert retained['labels']==dict(swapped=swap,reverse=reverse,A_start=ia,B_start=ib);aa,bb=val;assert aa==[[Q.parse(x) for x in z] for z in retained['canonical_A']];assert bb==[[Q.parse(x) for x in z] for z in retained['canonical_B']];delta=[x-y for lists,base in [(aa,ref['A']),(bb,ref['B'])] for row,rr in zip(lists,base) for x,y in zip(row,rr)];score=max(dot(g,delta) for g in gs);assert score==Q.parse(retained['defect']) and score>floor;gauges.append(score);rowindex+=1
 I=min(gauges);assert I==Q.parse(cert['exact_invariant']);assert rowindex==cert['valid_canonical_gauges']==50
 # Rational closure enumeration and independent full cyclic-word oracle.
 a=[list(map(F,u)) for u in src['A']];b=[list(map(F,u)) for u in src['B']];ca=closures(a);cb=closures(b);allqs=[];words=0
 for al in ca:
  for ga in cb:
   labels=[i for i,x in enumerate(al) if x]+[5+j for j,x in enumerate(ga) if x];weights=[x/2 for x in al+ga];points=[u+[F(0),F(0)] for u in a]+[[F(0),F(0)]+u for u in b]
   for tail in permutations(labels[1:]):
    word=[labels[0]]+list(tail);q=F(0)
    for k,i in enumerate(word):
     for j in word[k+1:]:
      u,v=points[i],points[j];q+=weights[i]*weights[j]*(u[0]*v[2]+u[1]*v[3]-u[2]*v[0]-u[3]*v[1])
    allqs.append(q);words+=1
 q=max(allqs);assert 1/(2*q)==F(cert['exact_capacity']);vol=F(1)
 for fac in [a,b]:
  vv=[]
  for i,j in combinations(range(5),2):
   try:v=solve([fac[i],fac[j]],[F(1),F(1)])
   except StopIteration:continue
   if all(dot(u,v)<=1 for u in fac) and v not in vv:vv.append(v)
  assert len(vv)==5 and all(sum(dot(u,v)==1 for v in vv)==2 for u in fac);cen=[sum(v[c] for v in vv)/5 for c in range(2)];order=angular([[x-y for x,y in zip(v,cen)] for v in vv]);vv=[vv[i] for i in order];vol*=abs(sum(det(vv[i],vv[(i+1)%5]) for i in range(5)))/2
 rho=1/(8*q*q*vol);r0=(Q(3)+Q(0,1))/5;rec=r0/(1+I);assert rho==F(cert['exact_ratio']);assert rec==Q.parse(cert['exact_reciprocal_prediction']) and Q(rho)>rec
 # Coarse rational separation proves thequalitativefailurewithoutlongprintedcoefficients.
 assert Q(rho)>Q(4)/5 and rec<Q(3)/5
 out=dict(status='PASS',name=src['name'],all_canonical_gauges_reconstructed=50,exact_invariant_above_four_fifths=True,exact_ratio_above_four_fifths=True,exact_reciprocal_prediction_below_three_fifths=True,all_cyclic_words=words,closure_counts=[len(ca),len(cb)],genuine_five_by_five_verified=True,both_global_bound_extensions_refuted=True,input_sha256={n:hashlib.sha256((d/n).read_bytes()).hexdigest() for n in ['global-counterexample-input.json','global-counterexample.json','verify-global.py']});(d/'global-verification.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
if __name__=='__main__':main()
