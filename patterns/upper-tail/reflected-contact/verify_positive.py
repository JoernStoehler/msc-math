#!/usr/bin/env python3
"""Independent9! all-cyclic-word exact check. No discovery imports."""
import argparse,hashlib,itertools,json,math,time
from fractions import Fraction as F
from pathlib import Path

def pair(x):return F(x['rational']),F(x['sqrt5'])
def plus(x,y):return x[0]+y[0],x[1]+y[1]
def times(x,y):return x[0]*y[0]+5*x[1]*y[1],x[0]*y[1]+x[1]*y[0]
def sign(x):
 a,b=x
 if a==0:return (b>0)-(b<0)
 if b==0:return (a>0)-(a<0)
 if (a>0)==(b>0):return 1 if a>0 else -1
 return ((a*a>5*b*b)-(a*a<5*b*b))*(1 if a>0 else -1)

def main():
 p=argparse.ArgumentParser();p.add_argument('--directory',type=Path,required=True);o=p.parse_args();d=o.directory;t=time.perf_counter();src=d/'hko-exact.json';r=json.loads(src.read_text());A=[[pair(x) for x in v] for v in r['A']];B=[[pair(x) for x in v] for v in r['B']]
 Dots=[[plus(times(x[0],y[0]),times(x[1],y[1])) for y in B] for x in A];den=math.lcm(*(v.denominator for row in Dots for q in row for v in q));co=[[(int(q[0]*den),int(q[1]*den)) for q in row] for row in Dots];best=None;words=0;maximizers=0
 for tail in itertools.permutations(range(1,10)):
  word=(0,)+tail;pos=[0]*10
  for k,i in enumerate(word):pos[i]=k
  aa=bb=0
  for i in range(5):
   for j in range(5):
    a,b=co[i][j]
    if pos[i]<pos[j+5]:aa+=a;bb+=b
    else:aa-=a;bb-=b
  val=aa,bb
  if best is None or sign((aa-best[0],bb-best[1]))>0:best=val;maximizers=1;bestword=word
  elif val==best:maximizers+=1
  words+=1
 Q=(F(best[0],100*den),F(best[1],100*den));assert Q==pair(r['max_Q'])==(F(0),F(1,25));assert pair(r['Q_fraction'])==(F(0),F(2,5));assert pair(r['ratio_inflation'])==(F(5,4),F(0));assert words==math.factorial(9)
 out=dict(status='PASS',all_cyclic_words=words,uniform_max_Q=r['max_Q'],maximizing_words=maximizers,one_maximizing_word=bestword,ratio_inflation_exact='5/4',seconds=time.perf_counter()-t,input_sha256=hashlib.sha256(src.read_bytes()).hexdigest(),verifier_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest());(d/'positive-verification.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
if __name__=='__main__':main()
