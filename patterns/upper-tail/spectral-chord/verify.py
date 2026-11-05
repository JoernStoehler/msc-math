#!/usr/bin/env python3
"""Independent standard-library rational witness and reference-bracket checker."""
import argparse,hashlib,importlib.util,json,math,sys
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

def main():
 p=argparse.ArgumentParser();p.add_argument('--directory',type=Path,default=Path(__file__).parent);p.add_argument('--repo',type=Path,required=True);o=p.parse_args();d=o.directory;doc=json.loads((d/'obstruction.json').read_text());a0=[[F(x) for x in v] for v in doc['input']['A']];b0=[[F(x) for x in v] for v in doc['input']['B']];a,ac,pa,area_a,_=poly(a0);b,bc,pb,area_b,_=poly(b0);allq=[];count=0
 for variant in doc['selection']['variants']:
  w=variant['word'];pos={i:k for k,i in enumerate(w)};s=[[F(1 if pos[i]<pos[len(a)+j] else -1,4)*dot(u,v) for j,v in enumerate(b)] for i,u in enumerate(a)];z=times(times(pa,s),pb);qs=[hk(a,b,w,ac,bc)]
  for ray in variant['rays']:
   seed=[F(x,2**30) for x in ray['seed']]
   if ray['kind']=='A':u=apply(pa,seed);v=apply(trans(z),u);aa=ends(ac,u);bb=ends(bc,v)
   else:v=apply(pb,seed);u=apply(z,v);aa=ends(ac,u);bb=ends(bc,v)
   for alpha in aa:
    for gamma in bb:
     assert min(alpha)>=0 and min(gamma)>=0 and sum(alpha)==sum(gamma)==1;qs.append(hk(a,b,w,alpha,gamma));count+=1
  assert max(qs)==F(variant['best']['Q']);allq+=qs
 q=max(allq);assert 1/(2*q)==F(doc['selection']['capacity'])>F(5015,1000)
 spec=importlib.util.spec_from_file_location('spectral_verify_q5',o.repo/'patterns/upper-tail/conditional-coupling/quadratic.py');f=importlib.util.module_from_spec(spec);spec.loader.exec_module(f);Q=f.Q5;sqrt5=Q(0,1);refa=[[Q(1),Q(0)],[(sqrt5-1)/4,Q(1)],[-(sqrt5+1)/4,(sqrt5-1)/2],[-(sqrt5+1)/4,-(sqrt5-1)/2],[(sqrt5-1)/4,-Q(1)]];refb=[[-v[1],v[0]] for v in refa];delta=F(doc['delta'])
 # The exact reference polygons lie in [-2,2]^2, hence their polars contain
 # the l1 ball of radius 1/2; facet errors of l1 norm <delta/2 lie in delta polar.
 for source,ref in [(a0,refa),(b0,refb)]:
  for u,v in zip(source,ref):assert sum(abs(Q(x)-y) for x,y in zip(u,v))<Q(delta/2)
  vertices=[]
  for u,v in combinations(ref,2):
   det=cross(u,v)
   if not det:continue
   x=[(v[1]-u[1])/det,(u[0]-v[0])/det]
   if all(dot(w,x)<=1 for w in ref):vertices.append(x)
  assert vertices and all(abs(x)<2 for v in vertices for x in v)
 lower=F(5)/(1+delta)**2;upper=F(5)/(1-delta)**2;assert lower==F(doc['capacity_lower']) and upper==F(doc['capacity_upper'])<F(5001,1000);volume=area_a*area_b;assert volume==F(doc['selection']['volume']);assert lower*lower/(2*volume)==F(doc['ratio_lower'])>1;assert F(doc['selection']['ratio'])>F(21,20) and (3+sqrt5)/5<Q(F(21,20))
 result=dict(status='PASS',rational_chord_corners=count,direct_4d_HK_values=len(allq),reference_coordinate_error_and_box_checks=True,known_reference_capacity_imported=5,capacity_lower=float(lower),capacity_upper=float(upper),selector_capacity=float(1/(2*q)),selector_ratio=float(F(doc['selection']['ratio'])),selector_ratio_exceeds_HKO=True,true_ratio_lower=float(lower*lower/(2*volume)),relative_capacity_miss_lower=float(F(5015,5001)-1),obstruction_sha256=hashlib.sha256((d/'obstruction.json').read_bytes()).hexdigest(),checker_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),limits='Certifies exact finite rounded-ray output and reference containment bracket; does not certify numerical eigenspace selection or an analytic discontinuity theorem.')
 (d/'verification.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
if __name__=='__main__':main()
