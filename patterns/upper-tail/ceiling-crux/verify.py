#!/usr/bin/env python3
"""Independent standard-library rational witness and reference-bracket checker."""
import argparse,hashlib,importlib.util,json,math,sys,gzip,time
from functools import cmp_to_key
from fractions import Fraction as F
from itertools import combinations
from pathlib import Path
sys.set_int_max_str_digits(0)
def dot(a,b):return sum(x*y for x,y in zip(a,b))
def cross(a,b):return a[0]*b[1]-a[1]*b[0]
def trans(a):return list(map(list,zip(*a)))
def times(a,b):return [[dot(u,v) for v in trans(b)] for u in a]
def apply(a,v):return [dot(u,v) for u in a]
def inverse(a):
 n=len(a);z=[u[:]+[F(i==j) for j in range(n)] for i,u in enumerate(a)]
 for j in range(n):
  k=next(i for i in range(j,n) if z[i][j]);z[j],z[k]=z[k],z[j];v=z[j][j];z[j]=[x/v for x in z[j]]
  for i in range(n):
   if i!=j:
    v=z[i][j];z[i]=[x-v*y for x,y in zip(z[i],z[j])]
 return [u[n:] for u in z]
def poly(a):
 vs=[]
 for u,v in combinations(a,2):
  det=cross(u,v)
  if not det:continue
  x=[(v[1]-u[1])/det,(u[0]-v[0])/det]
  if all(dot(w,x)<=1 for w in a) and x not in vs:vs.append(x)
 mean=[sum(x[k] for x in vs)/len(vs) for k in [0,1]];vs.sort(key=lambda x:math.atan2(float(x[1]-mean[1]),float(x[0]-mean[0])));twice=sum(cross(vs[i],vs[(i+1)%len(vs)]) for i in range(len(vs)));center=[sum((vs[i][k]+vs[(i+1)%len(vs)][k])*cross(vs[i],vs[(i+1)%len(vs)]) for i in range(len(vs)))/(3*twice) for k in [0,1]];weights=[F(0)]*len(a)
 for i,v in enumerate(vs):
  w=vs[(i+1)%len(vs)];active=[j for j,u in enumerate(a) if dot(u,v)==dot(u,w)==1];assert len(active)==1;weights[active[0]]=cross([v[k]-center[k] for k in [0,1]],[w[k]-center[k] for k in [0,1]])/twice
 centered=[[x/(1-dot(u,center)) for x in u] for u in a];h=[[F(1)]*len(a)]+trans(centered);q=times(times(trans(h),inverse(times(h,trans(h)))),h);projector=[[F(i==j)-q[i][j] for j in range(len(a))] for i in range(len(a))];return centered,weights,projector,twice/2,vs

def ends(c,u):
 lo=max(-x/y for x,y in zip(c,u) if y>0);hi=min(-x/y for x,y in zip(c,u) if y<0);return [[x+t*y for x,y in zip(c,u)] for t in [lo,hi]]
def hk(a,b,w,x,y):
 normals=[[u[0],u[1],F(0),F(0)] for u in a]+[[F(0),F(0),v[0],v[1]] for v in b];weights=[v/2 for v in x+y];assert sum(weights)==1 and all(sum(weights[i]*normals[i][k] for i in range(len(normals)))==0 for k in range(4));q=F(0)
 for k,i in enumerate(w):
  for j in w[k+1:]:
   u,v=normals[i],normals[j];q+=weights[i]*weights[j]*(u[0]*v[2]+u[1]*v[3]-u[2]*v[0]-u[3]*v[1])
 return q

def ceiling(r):
 x=5*r-3;return x<=0 or x*x<=5
def angular(a,b,sign):
 vectors=a+[[-sign*v[1],sign*v[0]] for v in b]
 def half(v):return 0 if v[1]>0 or v[1]==0 and v[0]>=0 else 1
 def cmp(i,j):
  h=half(vectors[i])-half(vectors[j])
  if h:return h
  d=cross(vectors[i],vectors[j]);return -1 if d>0 else 1 if d<0 else (i>j)-(i<j)
 word=sorted(range(len(vectors)),key=cmp_to_key(cmp));return word if sign>0 else word[::-1]
def star(c,p):
 points=[c]
 for direction in trans(p):
  if any(direction):
   for x in ends(c,direction):
    if x not in points:points.append(x)
 return points
def width(a,b,va,vb):
 aa=[[x/(1+max(-dot(u,v) for v in va)) for x in u] for u in a];bb=[[x/(1+max(-dot(u,v) for v in vb)) for x in u] for u in b]
 return 1/max(abs(dot(u,v)) for u in aa for v in bb)
def main():
 p=argparse.ArgumentParser();p.add_argument('--directory',type=Path,default=Path(__file__).parent);p.add_argument('--repo',type=Path,default=Path('/workspaces/msc-math-worktrees/patterns-2026-10-17'));p.add_argument('--source',type=Path,default=Path('/home/joern/.cache/msc-math/artifacts/polytope-datasets/f7bc6be841e9d30741d5bf7ec8d4f0c0c74ec22745d3325b792e64cec333ca96/files/random-product.jsonl'));p.add_argument('--out',type=Path,default=Path('/tmp/msc-math/ceiling-crux-replay.json'));o=p.parse_args();t=time.perf_counter();d=o.directory;compact=d/'compact-witnesses.json.gz'
 with gzip.open(compact,'rt') as g:rows=json.load(g)['rows']
 assert len(rows)==74 and len({r['name'] for r in rows})==74
 source_sha=hashlib.sha256(o.source.read_bytes()).hexdigest();assert source_sha=='66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736';source={};needed={r['name'] for r in rows[:14]}
 for line in o.source.open():
  r=json.loads(line)
  if r['name'] in needed:
   k=r['k'];u=r['dual_vertices_rational'];source[r['name']]=dict(A=[v[:2] for v in u[:k]],B=[v[2:] for v in u[k:]])
 sample=o.repo/'patterns/upper-tail/contact-stability/sample.json';sample_sha=hashlib.sha256(sample.read_bytes()).hexdigest();assert sample_sha=='f70e7f9bb6444e2b132dbaedf73c8f4a1f8324e72c37f79dbb7c2908e27bc2c8';raw=json.loads(sample.read_text());sample_rows=raw['rows'] if isinstance(raw,dict) else raw;source.update({r['name']:r for r in sample_rows});checks=[]
 for index,row in enumerate(rows):
  a0=[[F(x) for x in u] for u in row['A']];b0=[[F(x) for x in u] for u in row['B']];binding=source[row['name']]
  assert a0==[[F(x) for x in u] for u in binding['A']] and b0==[[F(x) for x in u] for u in binding['B']]
  a,ac,pa,aa,va=poly(a0);b,bc,pb,ab,vb=poly(b0);assert aa>0 and ab>0 and min(ac)>0 and min(bc)>0 and sum(ac)==sum(bc)==1 and apply(trans(a),ac)==apply(trans(b),bc)==[0,0];assert a==[[F(x) for x in u] for u in row['centered_A']] and b==[[F(x) for x in u] for u in row['centered_B']]
  for normals,centered,recorded in [(a0,a,row['centers'][0]),(b0,b,row['centers'][1])]:
   center=list(map(F,recorded));assert all(1-dot(u,center)>0 for u in normals);assert all(v==[x/(1-dot(u,center)) for x in u] for u,v in zip(normals,centered))
  alpha=list(map(F,row['alpha']));gamma=list(map(F,row['gamma']));assert min(alpha)>=0 and min(gamma)>=0 and sum(alpha)==sum(gamma)==1 and apply(trans(a),alpha)==apply(trans(b),gamma)==[0,0]
  points=star(ac,pa) if row['role']=='A' else star(bc,pb);point=alpha if row['role']=='A' else gamma;assert points[row['index']]==point
  word=angular(a,b,row['sign']);assert word==row['word'];q=hk(a,b,word,alpha,gamma);assert q==F(row['Q'])>0;cap=1/(2*q);vol=aa*ab;ratio=cap*cap/(2*vol);assert cap==F(row['capacity']) and vol==F(row['volume']) and ratio==F(row['star_ratio']);assert ceiling(ratio)
  wc=width(a0,b0,va,vb);wr=wc*wc/(2*vol);assert wc==F(row['width']) and wr==F(row['width_ratio']);assert (not ceiling(wr)) if index<14 else ceiling(wr);mr=min(wc,cap)**2/(2*vol);assert mr==F(row['min_ratio']) and ceiling(mr);checks.append(dict(name=row['name'],star_member=True,exact_word=True,feasible_4d_HK=True,star_witness_ratio_le_HKO=True,width_ratio_above_HKO=index<14))
 out=dict(status='PASS',bodies=74,registered_random_bindings=14,contact_sample_bindings=60,centroid_and_normal_bindings=148,exact_areas=148,normalized_closures=148,exact_projector_star_memberships=74,exact_angular_words=74,direct_4d_HK_witnesses=74,standalone_star_witness_ceilings_le_HKO=74,exact_widths=74,width_above_HKO_count=14,width_at_most_HKO_count=60,combined_ceilings_le_HKO=74,elapsed_seconds=time.perf_counter()-t,compact_sha256=hashlib.sha256(compact.read_bytes()).hexdigest(),source_sha256=source_sha,contact_sample_sha256=sample_sha,checker_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),checks=checks,limits='One feasible eligible star witness perbody certifies Cstar <= Cwinner and Cwinner^2/(2V) <= rho_H. No independent response/global maxima audit, no capacity oracle or historical capacity target verification, no universal theorem.')
 o.out.parent.mkdir(parents=True,exist_ok=True);o.out.write_text(json.dumps(out,indent=2)+'\n');print(json.dumps({k:v for k,v in out.items() if k!='checks'},indent=2))
if __name__=='__main__':main()
