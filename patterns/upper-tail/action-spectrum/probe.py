#!/usr/bin/env python3
import sys,importlib.util,math,json
from pathlib import Path
root=next((p for p in Path(__file__).resolve().parents if (p/'AGENTS.md').is_file()),Path.cwd())
s=importlib.util.spec_from_file_location('sh',root/'patterns/upper-tail/signed-prefix-response/analyze.py');sh=importlib.util.module_from_spec(s);s.loader.exec_module(sh)
def moments(vs):
 n=len(vs[0]);N=len(vs)
 return [sum(v[i] for v in vs)/N for i in range(n)],[[sum(v[i]*v[k] for v in vs)/N for k in range(n)] for i in range(n)]
def score(a,b,tol=0):
 va=sh.closures(a,tol);vb=sh.closures(b,tol);ma,sa=moments(va);mb,sb=moments(vb);out=[]
 for sign in [1,-1]:
  word=sh.shadow_word(a,b,sign,tol)
  if sign<0:word=word[::-1]
  pos={i:p for p,i in enumerate(word)};n=len(a);m=len(b)
  H=[[sh.dot(x,y)*(1 if pos[i]<pos[n+j] else -1)/4 for j,y in enumerate(b)] for i,x in enumerate(a)]
  m1=sum(ma[i]*H[i][j]*mb[j] for i in range(n) for j in range(m))
  m2=sum(H[i][j]*H[k][l]*sa[i][k]*sb[j][l] for i in range(n) for j in range(m) for k in range(n) for l in range(m))
  out.append(dict(sign=sign,word=word,m1=m1,m2=m2,L=m2/m1))
 return out
if __name__=='__main__':
 h=math.cos(math.pi/5);v=(5*math.sin(2*math.pi/5)/2)**2
 for theta in [0,math.pi/20,math.pi/10]:
  a=[(math.cos((2*i+1)*math.pi/5)/h,math.sin((2*i+1)*math.pi/5)/h) for i in range(5)];b=[(math.cos((2*i+1)*math.pi/5+theta)/h,math.sin((2*i+1)*math.pi/5+theta)/h) for i in range(5)]
  r=score(a,b,1e-11);L=max(z['L'] for z in r);cap=(1+h)**2/math.cos(theta);sys0=cap**2/(2*v)
  print(json.dumps(dict(theta=theta,moments=r,B=1/(8*v*L*L),sys=sys0,inflation=1/(8*v*L*L*sys0))))
