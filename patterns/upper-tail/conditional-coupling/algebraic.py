#!/usr/bin/env python3
"""One exact HKO LP certificate in Q(sqrt5), in determinant-rescaled algebraic coordinates."""
import argparse,hashlib,json
from pathlib import Path
from functools import cmp_to_key
from quadratic import Q5
from analyze import pilot,stringify

def cross(a,b):return a[0]*b[1]-a[1]*b[0]
def word(a,b,sign):
 vectors=a+[[-sign*v[1],sign*v[0]] for v in b]
 def half(u):return 0 if u[1]>0 or (u[1]==0 and u[0]>=0) else 1
 def compare(i,j):
  u,v=vectors[i],vectors[j];h=half(u)-half(v)
  if h:return h
  d=cross(u,v)
  if d:return -1 if d>0 else 1
  return (i>j)-(i<j)
 w=sorted(range(len(vectors)),key=cmp_to_key(compare))
 return w if sign>0 else w[::-1]
def main():
 p=argparse.ArgumentParser();p.add_argument('--out',type=Path,required=True);args=p.parse_args()
 z=Q5();one=Q5(1);s=Q5(0,1);c=(s-1)/4;d=-(s+1)/4;k=(s-1)/2
 a=[[one,z],[c,one],[d,k],[d,-k],[c,-one]];b=[[-v[1],v[0]] for v in a]
 variants=[pilot(a,b,word(a,b,sign),True)|{'shadow_sign':sign} for sign in [1,-1]]
 q=max(v['Q'] for v in variants);known=Q5(1)/10
 assert q/known==(1+3/s)/2
 circulant_weights=[z,1/s,z,(1-1/s)/2,(1-1/s)/2];circulant_W=[circulant_weights[(j-i)%5]/5 for i in range(5) for j in range(5)]
 assert all(sum(x*y for x,y in zip(v['objective'],circulant_W))==v['Q'] for v in variants)
 result=dict(circulant_weights=circulant_weights,circulant_W=circulant_W,coordinates='A=T regular unit normals, T=diag(1,1/sin72); B=JA. Relative to the original regular product, Q scales by detT=1/sin72 and capacity by sin72; hence imported Qtrue=1/10.',A=a,B=b,variants=variants,Q_LP=q,Q_true=known,LP_Q_over_true=q/known,capacity_lower=1/(2*q),true_capacity=1/(2*known),capacity_lower_over_true=known/q,algebraic_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest())
 args.out.write_text(json.dumps(stringify(result),indent=2)+'\n');print(float(q),q,float(known/q))
if __name__=='__main__':main()
