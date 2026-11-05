#!/usr/bin/env python3
"""Freeze rational irregular edge-isotropic pentagons without capacity calls."""
from fractions import Fraction as F
from pathlib import Path
import argparse, hashlib, json

def dot(u,v):return sum(x*y for x,y in zip(u,v))
def det(u,v):return u[0]*v[1]-u[1]*v[0]
def quad(x):
 b,d,t,u=x
 return 2*b*b-2*b*d+3*d*d-t*t-u*u
def serial(v):
 if isinstance(v,F):return str(v)
 if isinstance(v,dict):return {k:serial(x) for k,x in v.items()}
 if isinstance(v,(tuple,list)):return [serial(x) for x in v]
 return v
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()

def main():
 ap=argparse.ArgumentParser();ap.add_argument('--out',type=Path,required=True);o=ap.parse_args();o.out.mkdir(parents=True,exist_ok=True)
 seed=[F(1),F(0),F(1),F(1)];near=[F(951,1000),F(588,1000),F(691,1000),F(1118,1000)];v=[y-x for x,y in zip(seed,near)]
 linear=quad([x+y for x,y in zip(seed,v)])-quad(v);scale=-linear/quad(v);params=[x+scale*y for x,y in zip(seed,v)];assert quad(params)==0
 b,d,t,u=params
 raw=[[0,t+u],[b,u],[d,0],[-d,0],[-b,u]][::-1];center=[sum(p[k] for p in raw)/5 for k in range(2)];p=[[F(x)-c for x,c in zip(z,center)] for z in raw]
 edges=[[p[(i+1)%5][k]-p[i][k] for k in range(2)] for i in range(5)];area=sum(det(p[i],p[(i+1)%5]) for i in range(5))/2
 assert area>0 and min(det(edges[i],edges[(i+1)%5]) for i in range(5))>0
 gram=[[sum(e[i]*e[j] for e in edges) for j in range(2)] for i in range(2)];lam=gram[0][0];assert gram==[[lam,0],[0,lam]]
 corner=[det(edges[i],edges[(i+1)%5]) for i in range(5)];assert len(set(corner))>1
 def normals(poly):
  out=[]
  for i,v in enumerate(poly):
   w=poly[(i+1)%5];e=[w[k]-v[k] for k in range(2)];n=[e[1],-e[0]];h=dot(n,v);assert h>0;out.append([x/h for x in n])
  assert all(dot(n,v)<=1 for n in out for v in poly)
  return out
 rows=[]
 for label,half in [('low_angle',F(1,8)),('near_hko',F(4,25))]:
  c=(1-half*half)/(1+half*half);s=2*half/(1+half*half);q=[[c*x-s*y,s*x+c*y] for x,y in p]
  qe=[[q[(i+1)%5][k]-q[i][k] for k in range(2)] for i in range(5)];cross=[[dot(e,f) for f in qe] for e in edges]
  # S^2=-diag(B B^T,B^T B)/area^2; both blocks equal lam*E E^T/area^2.
  square=[[-lam*dot(e,f)/(area*area) for f in edges] for e in edges]
  assert [[sum(cross[i][k]*cross[j][k] for k in range(5)) for j in range(5)] for i in range(5)]==[[lam*dot(e,f) for f in edges] for e in edges]
  assert [[sum(cross[k][i]*cross[k][j] for k in range(5)) for j in range(5)] for i in range(5)]==[[lam*dot(e,f) for f in edges] for e in edges]
  rows.append(dict(name=label,half_angle_tangent=half,rotation=[[c,-s],[s,c]],P=p,Q=q,A=normals(p),B=normals(q),signed_ridge_cross_block=[[-x/area for x in row] for row in cross],signed_ridge_square_block=square))
 result=dict(construction='Rational second intersection of the frozen isotropy quadric line',parameters=dict(zip(['b','d','t','u'],params)),line_scale=scale,area=area,edge_gram=gram,consecutive_edge_determinants=corner,affine_regular=False,normalized_nonzero_spectral_magnitude=lam/area,normalized_characteristic_polynomial='x^6*(x^2+(lambda/area)^2)^2',signed_ridge_square_same_entrywise=True,rows=rows)
 (o.out/'sample.json').write_text(json.dumps(serial(result),indent=2)+'\n')
 manifest=dict(sample_sha256=sha(o.out/'sample.json'),construct_sha256=sha(Path(__file__)),plan_sha256=sha(Path(__file__).with_name('PLAN.md')),frozen_before_scoring=True)
 (o.out/'frozen-manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
 print(json.dumps(dict(rows=2,parameters=serial(result['parameters']),area=float(area),spectral_magnitude=float(lam/area),sample_sha256=manifest['sample_sha256']),indent=2))
if __name__=='__main__':main()
