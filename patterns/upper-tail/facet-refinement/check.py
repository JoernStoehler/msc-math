#!/usr/bin/env python3
"""Independent exact reconstruction of one analytic simultaneous-cut family.
Imports only existing ordered field arithmetic; no discovery code or capacity oracle.
"""
import importlib.util,json,itertools,math,hashlib,time,argparse
from pathlib import Path
from fractions import Fraction as F
from functools import cmp_to_key
R=Path('/workspaces/msc-math-worktrees/patterns-2026-10-17')
spec=importlib.util.spec_from_file_location('q5_root',R/'patterns/upper-tail/conditional-coupling/quadratic.py');mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod);E=mod.Q5
D=E(0,1)
def dot(x,y):return sum(a*b for a,b in zip(x,y))
def cross(x,y):return x[0]*y[1]-x[1]*y[0]
def transpose(x):return list(map(list,zip(*x)))
def matmul(a,b):return [[dot(u,v) for v in transpose(b)] for u in a]
def inverse(a):
 n=len(a);z=[u[:]+[E(i==j) for j in range(n)] for i,u in enumerate(a)]
 for j in range(n):
  k=next(i for i in range(j,n) if z[i][j]);z[j],z[k]=z[k],z[j];pivot=z[j][j];z[j]=[v/pivot for v in z[j]]
  for i in range(n):
   if i!=j:
    scale=z[i][j];z[i]=[v-scale*w for v,w in zip(z[i],z[j])]
 return [u[n:] for u in z]
def geometry(normals):
 verts=[]
 for u,v in itertools.combinations(normals,2):
  det=cross(u,v)
  if not det:continue
  x=[(v[1]-u[1])/det,(u[0]-v[0])/det]
  if all(dot(w,x)<=1 for w in normals) and x not in verts:verts.append(x)
 verts.sort(key=lambda x:math.atan2(float(x[1]),float(x[0])))
 twice=sum(cross(verts[i],verts[(i+1)%len(verts)]) for i in range(len(verts)))
 center=[sum((verts[i][k]+verts[(i+1)%len(verts)][k])*cross(verts[i],verts[(i+1)%len(verts)]) for i in range(len(verts)))/(3*twice) for k in (0,1)]
 assert center==[0,0]
 weights=[E(0)]*len(normals)
 for i,u in enumerate(verts):
  v=verts[(i+1)%len(verts)];active=[j for j,n in enumerate(normals) if dot(n,u)==dot(n,v)==1];assert len(active)==1
  weights[active[0]]=cross(u,v)/twice
 assert sum(weights)==1 and dot(weights,[u[0] for u in normals])==dot(weights,[u[1] for u in normals])==0
 return verts,twice/2,weights

def closures(a):
 out=[];n=len(a)
 for i,j in itertools.combinations(range(n),2):
  if not cross(a[i],a[j]) and dot(a[i],a[j])<0:
   k=next(k for k in (0,1) if a[i][k]);w=-a[j][k]/(a[i][k]-a[j][k]);v=[E(0)]*n;v[i]=w;v[j]=1-w;out.append(v)
 for ids in itertools.combinations(range(n),3):
  x,y,z=[a[i] for i in ids];w=[cross(y,z),cross(z,x),cross(x,y)];s=sum(w)
  if not s:continue
  w=[v/s for v in w]
  if min(w)<=0:continue
  v=[E(0)]*n
  for i,q in zip(ids,w):v[i]=q
  out.append(v)
 assert out
 return out

def projector(a):
 h=[[E(1)]*len(a)]+transpose(a);z=matmul(matmul(transpose(h),inverse(matmul(h,transpose(h)))),h)
 pi=[[E(i==j)-z[i][j] for j in range(len(a))] for i in range(len(a))]
 assert matmul(h,pi)==[[0]*len(a) for _ in h] and matmul(pi,pi)==pi
 return pi

def word(a,b,sign):
 v=a+[[-sign*x[1],sign*x[0]] for x in b]
 def half(x):return 0 if x[1]>0 or x[1]==0 and x[0]>=0 else 1
 def cmp(i,j):
  k=half(v[i])-half(v[j])
  if k:return k
  c=cross(v[i],v[j]);return -c.sign() if c else (i>j)-(i<j)
 w=sorted(range(len(v)),key=cmp_to_key(cmp));return w if sign>0 else w[::-1]
def hk(a,b,order,x,y):
 normals=[u+[E(0),E(0)] for u in a]+[[E(0),E(0)]+v for v in b];weights=[v/2 for v in x+y]
 assert sum(weights)==1 and all(dot(weights,[u[k] for u in normals])==0 for k in range(4))
 ans=E(0)
 for k,i in enumerate(order):
  for j in order[k+1:]:
   u,v=normals[i],normals[j];ans+=weights[i]*weights[j]*(u[0]*v[2]+u[1]*v[3]-u[2]*v[0]-u[3]*v[1])
 return ans

def star(c,pi):
 points=[c]
 for d in transpose(pi):
  lo=max(-v/u for v,u in zip(c,d) if u>0);hi=min(-v/u for v,u in zip(c,d) if u<0);assert lo<=0<=hi
  for t in (lo,hi):
   x=[v+t*u for v,u in zip(c,d)]
   if x not in points:points.append(x)
 return points

def run(delta):
 a0=[[E(1),E(0)],[(D-1)/4,E(1)],[-(D+1)/4,(D-1)/2],[-(D+1)/4,-(D-1)/2],[(D-1)/4,E(-1)]]
 a=a0+[[(a0[i][k]+a0[(i+1)%5][k])/(2*(1-delta)) for k in (0,1)] for i in range(5)];b=[[-u[1],u[0]] for u in a]
 va,aa,ca=geometry(a);vb,ab,cb=geometry(b);pa=projector(a);pb=projector(b)
 if delta==0:
  for pi in (pa,pb):
   assert all(min(pi[k][j] for k in range(5,10))<0<max(pi[k][j] for k in range(5,10)) for j in range(10))
 else:assert len(va)==len(vb)==10 and min(ca)>0 and min(cb)>0
 sa,sb=star(ca,pa),star(cb,pb);cla,clb=closures(a),closures(b);best=None;response_values=0;direct=0
 for sign in (1,-1):
  w=word(a,b,sign);pos={i:k for k,i in enumerate(w)};matrix=[[dot(u,v)*(1 if pos[i]<pos[10+j] else -1)/4 for j,v in enumerate(b)] for i,u in enumerate(a)]
  for role,points,other in [('A',sa,clb),('B',sb,cla)]:
   for x in points:
    co=[dot(x,col) for col in transpose(matrix)] if role=='A' else [dot(row,x) for row in matrix]
    vals=[dot(co,y) for y in other];idx=max(range(len(other)),key=lambda i:vals[i]);q=vals[idx];response_values+=len(vals)
    alpha,gamma=(x,other[idx]) if role=='A' else (other[idx],x)
    assert q==hk(a,b,w,alpha,gamma);direct+=1
    if best is None or q>best[0]:best=(q,alpha,gamma,w)
 q=best[0];cap=1/(2*q);volume=aa*ab;s=cap*cap/(2*volume);rho=(3+D)/5
 widthsA=[[u[k]/(1+max(-dot(u,v) for v in va)) for k in (0,1)] for u in a];widthsB=[[u[k]/(1+max(-dot(u,v) for v in vb)) for k in (0,1)] for u in b]
 c2=1/max(abs(dot(u,v)) for u in widthsA for v in widthsB);s2=c2*c2/(2*volume)
 lower=E(5)*(1-delta)*(1-delta)
 true_ratio_lower=lower*lower/(2*volume)
 assert true_ratio_lower>1
 # Literal vertex support checks prove (1-delta)P0 subset Pdelta subset P0.
 outer_vertices=geometry(a0)[0]
 assert all(dot(n,[(1-delta)*v for v in vertex])<=1 for n in a for vertex in outer_vertices)
 assert all(dot(n,vertex)<=1 for n in a0 for vertex in va)
 if delta==0:
  assert len(sa)==len(sb)==1 and q==D/25 and s==(3+D)/4 and c2==5 and s2==rho
 else:assert s>rho and s2<rho
 return dict(delta=str(delta),facets=[len(va),len(vb)],star_points=[len(sa),len(sb)],closure_vertices=[len(cla),len(clb)],qstar=q.serial(),capacity_ceiling=cap.serial(),star_ratio=s.serial(),star_ratio_float=float(s),width_ratio=s2.serial(),width_ratio_float=float(s2),volume=volume.serial(),true_capacity_lower=lower.serial(),true_ratio_lower=true_ratio_lower.serial(),true_ratio_lower_float=float(true_ratio_lower),response_values=response_values,direct_HK_checks=direct)

def main():
 parser=argparse.ArgumentParser();parser.add_argument('--out',type=Path,default=Path('/tmp/msc-math/facet-refinement-root.json'));o=parser.parse_args();t=time.perf_counter();rows=[run(F(0)),run(F(1,1000))];out=dict(status='PASS',rows=rows,scope='Independent exact limiting/one finite analytic-family reconstruction; no capacity oracle, new cohort or global combined-bound claim.',elapsed_seconds=time.perf_counter()-t,checker_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest());o.out.parent.mkdir(parents=True,exist_ok=True);o.out.write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
if __name__=='__main__':main()
