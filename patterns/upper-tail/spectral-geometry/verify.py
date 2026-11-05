#!/usr/bin/env python3
"""Independent exact geometry/HK check and interval certificate.

No construct.py/score.py import; triangle closure weights use determinant
coordinates, and every active six-facet word is evaluated in R^4.
"""
from fractions import Fraction as F
from itertools import combinations,permutations
from math import comb
from pathlib import Path
import argparse,hashlib,json,time

def dot(u,v):return sum(x*y for x,y in zip(u,v))
def det(u,v):return u[0]*v[1]-u[1]*v[0]
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def serial(v):
 if isinstance(v,F):return str(v)
 if isinstance(v,dict):return {k:serial(x) for k,x in v.items()}
 if isinstance(v,(tuple,list)):return [serial(x) for x in v]
 return v
def omega(u,v):return u[0]*v[2]+u[1]*v[3]-u[2]*v[0]-u[3]*v[1]
def closures(a):
 out=[]
 for ids in combinations(range(5),3):
  i,j,k=ids;raw=[det(a[j],a[k]),det(a[k],a[i]),det(a[i],a[j])];mass=sum(raw)
  if not mass:continue
  w=[x/mass for x in raw]
  if min(w)<=0:continue
  assert sum(w)==1 and all(sum(x*a[y][z] for x,y in zip(w,ids))==0 for z in range(2));out.append((ids,w))
 # The frozen geometry has no opposite pair, so the list exhausts all vertices.
 assert all(det(u,v)!=0 for u,v in combinations(a,2))
 return out
def coefficients(a,ca,cb):
 """Full direct HK at angle0 and angle pi/2, for every support and order."""
 out=[]
 for (ii,alpha),(jj,gamma) in ((u,v) for u in ca for v in cb):
  A=[[a[i][0],a[i][1],F(0),F(0)] for i in ii];B0=[[F(0),F(0),a[j][0],a[j][1]] for j in jj];B1=[[F(0),F(0),-a[j][1],a[j][0]] for j in jj];w=[x/2 for x in alpha+gamma];points0=A+B0;points1=A+B1
  pair0={(i,j):w[i]*w[j]*omega(points0[i],points0[j]) for i in range(6) for j in range(i+1,6)};pair1={(i,j):w[i]*w[j]*omega(points1[i],points1[j]) for i in range(6) for j in range(i+1,6)}
  for tail in permutations(range(1,6)):
   word=(0,)+tail;pos={v:i for i,v in enumerate(word)};q0=sum(x if pos[i]<pos[j] else -x for (i,j),x in pair0.items());q1=sum(x if pos[i]<pos[j] else -x for (i,j),x in pair1.items());out.append((q0,q1))
 return out
def bernstein(A,B,V,l,h):
 # P(t)=(1+t^2)^2-8V*(A(1-t^2)+2Bt)^2.
 n=[A,2*B,-A];sq=[sum(n[i]*n[k-i] for i in range(3) if 0<=k-i<3) for k in range(5)];p=[F(v)-8*V*x for v,x in zip([1,0,2,0,1],sq)]
 translated=[sum(p[i]*comb(i,k)*l**(i-k)*(h-l)**k for i in range(k,5)) for k in range(5)]
 return [sum(translated[k]*F(comb(j,k),comb(4,k)) for k in range(j+1)) for j in range(5)]
def main():
 ap=argparse.ArgumentParser();ap.add_argument('--directory',type=Path,required=True);o=ap.parse_args();d=o.directory;src=Path(__file__).resolve().parent;t0=time.perf_counter();sample=json.loads((d/'sample.json').read_text());result=json.loads((d/'results.json').read_text());freeze=json.loads((d/'frozen-manifest.json').read_text());assert sha(d/'sample.json')==freeze['sample_sha256']==result['source_sample_sha256'];assert sha(src/'construct.py')==freeze['construct_sha256'];assert sha(src/'PLAN.md')==freeze['plan_sha256'];assert sha(src/'score.py')==result['score_sha256']
 p=[[F(x) for x in r] for r in sample['rows'][0]['P']];edges=[[p[(i+1)%5][k]-p[i][k] for k in range(2)] for i in range(5)];area=sum(det(p[i],p[(i+1)%5]) for i in range(5))/2;V=area*area;gram=[[sum(e[i]*e[j] for e in edges) for j in range(2)] for i in range(2)];lam=gram[0][0];assert gram==[[lam,0],[0,lam]];assert F(sample['area'])==area
 corners=[det(edges[i],edges[(i+1)%5]) for i in range(5)];assert min(corners)>0 and len(set(corners))>1
 a=[[F(x) for x in r] for r in sample['rows'][0]['A']];ca=closures(a);assert len(ca)==5
 for i,n in enumerate(a):
  assert dot(n,p[i])==dot(n,p[(i+1)%5])==1 and all(dot(n,p[j])<1 for j in range(5) if j not in [i,(i+1)%5])
 coeffs=coefficients(a,ca,ca);assert len(coeffs)==3000;checks=[]
 for s,r in zip(sample['rows'],result['rows']):
  half=F(s['half_angle_tangent']);c=(1-half*half)/(1+half*half);ss=2*half/(1+half*half);q=max(A*c+B*ss for A,B in coeffs);assert q==F(r['Q']);cap=1/(2*q);ratio=cap*cap/(2*V);assert cap==F(r['capacity']) and ratio==F(r['ratio'])>1;rot=[[c*x-ss*y,ss*x+c*y] for x,y in p];assert rot==[[F(x) for x in z] for z in s['Q']]
  rot_a=[[c*x-ss*y,ss*x+c*y] for x,y in a];assert rot_a==[[F(x) for x in z] for z in s['B']]
  f=[[rot[(i+1)%5][k]-rot[i][k] for k in range(2)] for i in range(5)];B=[[-dot(e,g)/area for g in f] for e in edges];assert B==[[F(x) for x in z] for z in s['signed_ridge_cross_block']];square=[[-lam*dot(e,g)/V for g in edges] for e in edges];assert square==[[F(x) for x in z] for z in s['signed_ridge_square_block']];assert [[-sum(B[i][k]*B[j][k] for k in range(5)) for j in range(5)] for i in range(5)]==square;assert [[-sum(B[k][i]*B[k][j] for k in range(5)) for j in range(5)] for i in range(5)]==square
  checks.append(dict(name=s['name'],ratio=ratio,ratio_float=float(ratio),Q=q,capacity=cap))
 l=F(1,8);h=F(4,25);certificate=[bernstein(A,B,V,l,h) for A,B in set(coeffs)];minimum=min(x for row in certificate for x in row);assert minimum>0
 lo=min(v['ratio'] for v in checks);hi=max(v['ratio'] for v in checks);assert hi/lo-1==F(result['relative_ratio_separation']);assert (hi-lo)/(hi+lo)==F(result['common_prediction_minimum_worst_relative_ratio_error'])
 out=dict(status='PASS',independent_direct_HK_orders=len(coeffs)*2,all_support_pair_count=25,closure_vertices_per_factor=5,strictly_convex_genuine_pentagon=True,non_affine_regular_proof='consecutive oriented edge determinants are not all equal',signed_ridge_square_identical_entrywise=True,normalized_spectral_magnitude=lam/area,checks=checks,continuum_certificate=dict(tangent_half_angle_interval=[l,h],distinct_sinusoidal_word_coefficients=len(set(coeffs)),quartic_Bernstein_coefficients_per_objective=5,minimum_positive_Bernstein_coefficient=minimum,minimum_positive_Bernstein_coefficient_float=float(minimum),all_exact_ratios_strictly_above_one=True),elapsed_seconds=time.perf_counter()-t0,hashes={**{k:sha(src/k) for k in ['PLAN.md','construct.py','score.py','verify.py']},**{k:sha(d/k) for k in ['sample.json','frozen-manifest.json','results.json']}})
 (d/'verification.json').write_text(json.dumps(serial(out),indent=2)+'\n');print(json.dumps(serial(out),indent=2))
if __name__=='__main__':main()
