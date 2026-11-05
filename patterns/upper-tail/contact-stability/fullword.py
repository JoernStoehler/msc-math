#!/usr/bin/env python3
"""Exact exhaustive base census; direct analytic closure differentiation."""
import argparse,hashlib,importlib.util,json,sys
from itertools import combinations,permutations
from pathlib import Path

def module(n,p):
 s=importlib.util.spec_from_file_location(n,p);m=importlib.util.module_from_spec(s);sys.modules[n]=m;s.loader.exec_module(m);return m

def main():
 p=argparse.ArgumentParser();p.add_argument('--repo',type=Path,required=True);p.add_argument('--directory',type=Path,required=True);o=p.parse_args();d=o.directory;Q=module('fullword_q5',o.repo/'patterns/upper-tail/conditional-coupling/quadratic.py').Q5;pr=json.loads((d/'predictions.json').read_text());a=[[Q.parse(x) for x in u] for u in pr['base']['A']];b=[[Q.parse(x) for x in u] for u in pr['base']['B']]
 def dot(u,v):return sum(x*y for x,y in zip(u,v))
 def solve(A,rhs):
  M=[[Q(x) for x in row]+[Q(y)] for row,y in zip(A,rhs)]
  for j in range(3):
   i=next(i for i in range(j,3) if M[i][j]);M[j],M[i]=M[i],M[j];v=M[j][j];M[j]=[x/v for x in M[j]]
   for i in range(3):
    if i!=j:
     v=M[i][j];M[i]=[x-v*y for x,y in zip(M[i],M[j])]
  return [row[-1] for row in M]
 def closures(a):
  out=[]
  for ids in combinations(range(5),3):
   M=[[1]*3,[a[i][0] for i in ids],[a[i][1] for i in ids]];w=solve(M,[1,0,0])
   if min(w)<=0:continue
   inv=[solve(M,[int(k==j) for k in range(3)]) for j in range(3)];out.append((ids,w,inv))
  return out
 ca=closures(a);cb=closures(b);q0=Q(1)/10;seen={};count=0;active=0;maxinactive=None
 for ids,aa,invA in ca:
  for jds,bb,invB in cb:
   labels=list(ids)+[5+j for j in jds]
   for tail in permutations(labels[1:]):
    word=[labels[0]]+list(tail);pos={j:i for i,j in enumerate(word)};eps=[[1 if pos[i]<pos[5+j] else -1 for j in jds] for i in ids];ha=[sum(bb[j]*eps[i][j]*dot(a[ids[i]],b[jds[j]]) for j in range(3))/4 for i in range(3)];hb=[sum(aa[i]*eps[i][j]*dot(a[ids[i]],b[jds[j]]) for i in range(3))/4 for j in range(3)];q=dot(aa,ha);assert q<=q0;count+=1
    if q!=q0:
     if maxinactive is None or q>maxinactive:maxinactive=q
     continue
    active+=1;g=[Q() for _ in range(20)]
    for i in range(3):
     for c in range(2):g[2*ids[i]+c]=aa[i]*(sum(bb[j]*eps[i][j]*b[jds[j]][c] for j in range(3))/4-dot(invA[c+1],ha))
    for j in range(3):
     for c in range(2):g[10+2*jds[j]+c]=bb[j]*(sum(aa[i]*eps[i][j]*a[ids[i]][c] for i in range(3))/4-dot(invB[c+1],hb))
    key=tuple((x.a,x.b) for x in g)
    if key not in seen:seen[key]=dict(A_support=ids,B_support=jds,word=word,Q_gradient=[x.serial() for x in g])
 bank={tuple((Q.parse(x).a,Q.parse(x).b) for x in row['Q_gradient']) for row in pr['bank']};assert set(seen)==bank
 out=dict(status='PASS',all_cyclic_words=count,all_closure_pairs=len(ca)*len(cb),active_words=active,distinct_active_gradients=len(seen),matches_frozen_bank=True,Q0=q0.serial(),largest_inactive_Q=maxinactive.serial(),strict_inactive_gap=(q0-maxinactive).serial(),gradient_bank=list(seen.values()),prediction_sha256=hashlib.sha256((d/'predictions.json').read_bytes()).hexdigest(),source_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest())
 (d/'fullword.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps({k:v for k,v in out.items() if k!='gradient_bank'},indent=2))
if __name__=='__main__':main()
