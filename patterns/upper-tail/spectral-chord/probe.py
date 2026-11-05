#!/usr/bin/env python3
import argparse,hashlib,importlib.util,json,math,sys,time
from pathlib import Path
from fractions import Fraction as F
from itertools import combinations
import numpy as np
sys.set_int_max_str_digits(0)
R=Path('/workspaces/msc-math-worktrees/patterns-2026-10-17')
spec=importlib.util.spec_from_file_location('spectral_shadow',R/'patterns/upper-tail/signed-prefix-response/analyze.py');sh=importlib.util.module_from_spec(spec);spec.loader.exec_module(sh)
def serial(x):
 if isinstance(x,F):return str(x)
 if isinstance(x,dict):return {k:serial(v) for k,v in x.items()}
 if isinstance(x,(list,tuple)):return [serial(v) for v in x]
 return x

def dot(x,y):return sum(a*b for a,b in zip(x,y))
def tr(a):return list(map(list,zip(*a)))
def mm(a,b):return [[dot(u,v) for v in tr(b)] for u in a]
def mv(a,v):return [dot(u,v) for u in a]
def inv(a):
 n=len(a);a=[u[:]+[F(i==j) for j in range(n)] for i,u in enumerate(a)]
 for j in range(n):
  k=next(i for i in range(j,n) if a[i][j]);a[j],a[k]=a[k],a[j];z=a[j][j];a[j]=[v/z for v in a[j]]
  for i in range(n):
   if i!=j:
    z=a[i][j];a[i]=[x-z*y for x,y in zip(a[i],a[j])]
 return [u[n:] for u in a]
def centroid(a):
 vv=sh.old.vertices(a);mean=[sum(v[k] for v in vv)/len(vv) for k in range(2)];vv.sort(key=lambda v:math.atan2(float(v[1]-mean[1]),float(v[0]-mean[0])))
 twice=sum(sh.cross(vv[i],vv[(i+1)%len(vv)]) for i in range(len(vv)));assert twice>0
 cen=[sum((vv[i][k]+vv[(i+1)%len(vv)][k])*sh.cross(vv[i],vv[(i+1)%len(vv)]) for i in range(len(vv)))/(3*twice) for k in range(2)]
 aa=[[v/(1-dot(u,cen)) for v in u] for u in a]
 weights=[F(0)]*len(a)
 for i,v in enumerate(vv):
  w=vv[(i+1)%len(vv)];active=[j for j,u in enumerate(a) if dot(u,v)==dot(u,w)==1];assert len(active)==1
  weights[active[0]]=sh.cross([v[k]-cen[k] for k in range(2)],[w[k]-cen[k] for k in range(2)])/twice
 assert min(weights)>0 and sum(weights)==1 and mv(tr(aa),weights)==[0,0]
 return aa,weights,twice/2,cen

def projector(a):
 n=len(a);h=[[F(1)]*n]+tr(a);z=mm(mm(tr(h),inv(mm(h,tr(h)))),h)
 return [[F(i==j)-z[i][j] for j in range(n)] for i in range(n)]
def endpoints(c,u):
 low=max(-x/v for x,v in zip(c,u) if v>0);high=min(-x/v for x,v in zip(c,u) if v<0)
 return [[x+t*v for x,v in zip(c,u)] for t in [low,high]]
def approximate(u,p):
 norm=np.linalg.norm(u)
 if norm<1e-10:return None
 v=[F(int(np.rint(x/norm*2**30)),2**30) for x in u];v=mv(p,v)
 if not any(v):return None
 return v

def select(a,b):
 a,ac,area_a,center_a=centroid(a);b,bc,area_b,center_b=centroid(b);pa=projector(a);pb=projector(b);n=len(a);m=len(b);variants=[]
 for sign in [1,-1]:
  w=sh.shadow_word(a,b,sign)
  if sign<0:w=w[::-1]
  pos={i:k for k,i in enumerate(w)};s=[[F(1 if pos[i]<pos[n+j] else -1,4)*dot(u,v) for j,v in enumerate(b)] for i,u in enumerate(a)];z=mm(mm(pa,s),pb);zn=np.array([[float(x) for x in u] for u in z]);ul,sv,vt=np.linalg.svd(zn,full_matrices=False);top=sv>=sv[0]*(1-1e-10);left=ul[:,top]@ul[:,top].T;right=vt[top,:].T@vt[top,:];rays=[];candidates=[dict(Q=dot(ac,mv(s,bc)),alpha=ac,gamma=bc,kind='canonical')]
  for kind,projected,p in [('A',left,pa),('B',right,pb)]:
   for k in range(projected.shape[0]):
    u=approximate(projected[:,k],p)
    if u is None:continue
    if kind=='A':v=mv(tr(z),u);aa=endpoints(ac,u);bb=endpoints(bc,v)
    else:v=mv(z,u);aa=endpoints(ac,v);bb=endpoints(bc,u)
    for i,alpha in enumerate(aa):
     for j,gamma in enumerate(bb):
      assert min(alpha)>=0 and min(gamma)>=0 and sum(alpha)==sum(gamma)==1
      assert mv(tr(a),alpha)==[0,0] and mv(tr(b),gamma)==[0,0]
      candidates.append(dict(Q=dot(alpha,mv(s,gamma)),alpha=alpha,gamma=gamma,kind=kind,index=k,corner=[i,j]))
  best=max(candidates,key=lambda v:v['Q']);variants.append(dict(sign=sign,word=w,singular_values=list(map(float,sv)),top_dimension=int(sum(top)),candidate_count=len(candidates),best=best))
 best=max(variants,key=lambda v:v['best']['Q']);q=best['best']['Q'];return dict(capacity=1/(2*q),ratio=1/(8*q*q*area_a*area_b),volume=area_a*area_b,centered_A=a,centered_B=b,centers=[center_a,center_b],variants=variants,best=best)

def main():
 p=argparse.ArgumentParser();p.add_argument('mode',choices=['diagnostic','broad','controls']);p.add_argument('--directory',type=Path,required=True);p.add_argument('--source',type=Path);o=p.parse_args();d=o.directory;t=time.perf_counter();rows=[]
 if o.mode=='diagnostic':
  sources=[json.loads((R/'patterns/upper-tail/contact-stability/global-counterexample-input.json').read_text())]+json.loads((R/'patterns/upper-tail/lp-seeded-response/sample.json').read_text())['rows'];s=math.sqrt(5);a=[[1.,0.],[(s-1)/4,1.],[-(s+1)/4,(s-1)/2],[-(s+1)/4,-(s-1)/2],[(s-1)/4,-1.]];b=[[-v[1],v[0]] for v in a];sources.insert(0,dict(name='HKO_binary_approximation',A=[[str(F(x)) for x in v] for v in a],B=[[str(F(x)) for x in v] for v in b]))
 elif o.mode=='broad':
  population=[json.loads(z) for z in o.source.open()];sources=[]
  for r in population:
   if (r['k'],r['m'])!=(5,5):continue
   normals=r['dual_vertices_rational'];sources.append(dict(name=r['name'],A=[v[:2] for v in normals[:r['k']]],B=[v[2:] for v in normals[r['k']:]]))
  (d/'broad-inputs.json').write_text(json.dumps(sources,indent=2)+'\n')
 else:
  base=json.loads((R/'patterns/upper-tail/contact-stability/global-counterexample-input.json').read_text());a=[[F(x) for x in v] for v in base['A']];b=[[F(x) for x in v] for v in base['B']];G=[[F(2),F(1)],[F(0),F(1)]];gn=[[F(2),F(1)],[F(0),F(-1)]];ta=[F(1,17),F(-1,31)];tb=[F(-1,23),F(1,37)];sources=[base,dict(name='permutation',A=[a[i] for i in [3,0,4,1,2]],B=[b[i] for i in [4,2,0,3,1]]),dict(name='factor_swap',A=b,B=a),dict(name='translation',A=[[x/(1+dot(u,ta)) for x in u] for u in a],B=[[x/(1+dot(u,tb)) for x in u] for u in b])]
  for name,g in [('positive_GL',G),('negative_GL',gn)]:sources.append(dict(name=name,A=tr(mm(tr(inv(g)),tr(a))),B=tr(mm(g,tr(b)))))
 for row_index,source in enumerate(sources):
  a=[[F(x) for x in v] for v in source['A']];b=[[F(x) for x in v] for v in source['B']];s=select(a,b);rows.append(dict(name=source['name'],principal_top3=source.get('principal_top3'),selection=s))
  if o.mode!='broad' or row_index%100==0:print(row_index,source['name'],float(s['capacity']),float(s['ratio']),flush=True)
 out=dict(rows=rows,elapsed_seconds=time.perf_counter()-t,plan_sha256=hashlib.sha256((d/'PLAN.md').read_bytes()).hexdigest(),code_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest());path=d/(o.mode+'-predictions.json');assert not path.exists();path.write_text(json.dumps(serial(out),indent=2)+'\n');print('SHA256',hashlib.sha256(path.read_bytes()).hexdigest(),'SECONDS',out['elapsed_seconds'])
if __name__=='__main__':main()
