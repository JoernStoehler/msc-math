#!/usr/bin/env python3
"""Independent rational fixed-support word enumeration and support-prefix formula.
Only stdlib. Writes JSON to --out, uses no retained data or capacity producer.
"""
import argparse,hashlib,json,random
from fractions import Fraction as F
from functools import cmp_to_key
from itertools import permutations
from pathlib import Path

def dot(x,y):return sum(a*b for a,b in zip(x,y))
def cross(x,y):return x[0]*y[1]-x[1]*y[0]
def angle_order(vs):
 def half(v):return 0 if v[1]>0 or (v[1]==0 and v[0]>=0) else 1
 def compare(i,j):
  h=half(vs[i])-half(vs[j])
  if h:return h
  d=cross(vs[i],vs[j])
  return (-1 if d>0 else 1) if d else ((i>j)-(i<j))
 return sorted(range(len(vs)),key=cmp_to_key(compare))
def hk(word,x,y):
 n=len(x);pos={i:k for k,i in enumerate(word)}
 return sum(dot(a,b)*(1 if pos[i]<pos[n+j] else -1) for i,a in enumerate(x) for j,b in enumerate(y))/4

def prefix_optimum(x,y,a_order):
 p=(F(0),F(0));vertices=[]
 for i in a_order:
  vertices.append(p);p=tuple(p[k]+x[i][k] for k in range(2))
 assert p==(0,0)
 return sum(max(dot(p,b) for p in vertices) for b in y)/2

def walk_area(edges):
 p=(F(0),F(0));s=F(0)
 for a in edges:
  nxt=tuple(p[k]+a[k] for k in range(2));s+=cross(p,nxt);p=nxt
 assert p==(0,0)
 return s/2

def mixed(x,y,sgn):
 ey=[(-sgn*b[1],sgn*b[0]) for b in y];xs=[x[i] for i in angle_order(x)];ys=[ey[i] for i in angle_order(ey)];both=xs+ys
 return (walk_area([both[i] for i in angle_order(both)])-walk_area(xs)-walk_area(ys))/2

def closed(rng,n):
 while True:
  a=[tuple(F(rng.randint(-8,8)) for _ in range(2)) for _ in range(n-1)]
  a.append(tuple(-sum(v[k] for v in a) for k in range(2)))
  if all(v!=(0,0) for v in a) and (n==2 or any(cross(a[i],a[j]) for i in range(n) for j in range(i))):return a

def encode(x):
 if isinstance(x,F):return str(x)
 if isinstance(x,dict):return {k:encode(v) for k,v in x.items()}
 if isinstance(x,(list,tuple)):return [encode(v) for v in x]
 return x

def main():
 parser=argparse.ArgumentParser();parser.add_argument('--out',type=Path,required=True);args=parser.parse_args()
 rng=random.Random(20261017);fixtures=[]
 # Generic triples, opposite pairs, larger second closures, ties/parallel cross directions.
 for na,nb,repeats in [(3,3,24),(2,3,6),(3,2,6),(2,2,3),(3,4,5),(2,5,3)]:
  for _ in range(repeats):fixtures.append((closed(rng,na),closed(rng,nb)))
 fixtures.extend([([(F(1),F(0)),(F(0),F(1)),(F(-1),F(-1))],[(F(1),F(0)),(F(0),F(1)),(F(-1),F(-1))]), ([(F(1),F(0)),(F(0),F(1)),(F(-1),F(-1))],[(F(0),F(1)),(F(-1),F(0)),(F(1),F(-1))])])
 rows=[];nwords=0
 for x,y in fixtures:
  n=len(x);r=len(y);vals=[(hk([0]+list(p),x,y),[0]+list(p)) for p in permutations(range(1,n+r))];nwords+=len(vals);q,word=max(vals)
  sh=[]
  for s in [1,-1]:
   vectors=x+[(-s*b[1],s*b[0]) for b in y];w=angle_order(vectors);raw=hk(w,x,y);m=mixed(x,y,s)
   assert raw==s*m
   sh.append(dict(sign=s,word=w,raw_Q=raw,mixed_area=m))
  pref=[prefix_optimum(x,y,[0]+list(p)) for p in permutations(range(1,n))]
  assert q==max(pref)==max(z['mixed_area'] for z in sh)
  # Nonuniform factor weights change normals, not the weighted-step objective.
  aa=[F(i+1,sum(range(1,n+1))) for i in range(n)];gg=[F(r-j,sum(range(1,r+1))) for j in range(r)]
  normals_a=[tuple(v/aa[i] for v in x[i]) for i in range(n)];normals_b=[tuple(v/gg[j] for v in y[j]) for j in range(r)]
  assert sum(aa)==sum(gg)==1
  assert all(sum(aa[i]*normals_a[i][k] for i in range(n))==0 for k in range(2))
  # Four-dimensional omega check, independent of the preceding pairwise eps sum.
  weights=[v/2 for v in aa+gg];dv=[v+(F(0),F(0)) for v in normals_a]+[(F(0),F(0))+v for v in normals_b]
  qq=F(0)
  for i in range(len(word)):
   for j in range(i+1,len(word)):
    a,b=word[i],word[j];u,v=dv[a],dv[b];om=sum(u[k]*v[k+2]-u[k+2]*v[k] for k in range(2));qq+=weights[a]*weights[b]*om
  assert qq==q
  rows.append(dict(A_steps=x,B_steps=y,A_weights=aa,B_weights=gg,maximum_Q=q,maximizing_word=word,shadow=sh,cyclic_words=len(vals)))
 result=dict(status='PASS',scope='finite exact fixed-closure checks; not a substitute for the proof',seed=20261017,script_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),cases=len(rows),cyclic_words=nwords,rows=rows)
 args.out.write_text(json.dumps(encode(result),indent=2)+'\n');print(json.dumps({k:v for k,v in result.items() if k!='rows'}))
if __name__=='__main__':main()
