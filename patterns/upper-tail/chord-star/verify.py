#!/usr/bin/env python3
"""Independent exact finite bracket checker; imports no discovery code."""
import argparse,gzip,hashlib,itertools,json,math,sys,time
from fractions import Fraction as F
from pathlib import Path
sys.set_int_max_str_digits(0)
D=Path(__file__).resolve().parent
def dot(a,b):return sum(x*y for x,y in zip(a,b))
def cross(a,b):return a[0]*b[1]-a[1]*b[0]
def matvec(a,v):return [dot(x,v) for x in a]
def solve(a,b):
 n=len(b);z=[list(x)+[y] for x,y in zip(a,b)]
 for j in range(n):
  choices=[i for i in range(j,n) if z[i][j]]
  if not choices:return None
  k=max(choices,key=lambda i:abs(z[i][j]));z[j],z[k]=z[k],z[j];p=z[j][j];z[j]=[x/p for x in z[j]]
  for i in range(n):
   if i!=j:
    p=z[i][j];z[i]=[x-p*y for x,y in zip(z[i],z[j])]
 return [x[-1] for x in z]
def closures(normals):
 n=len(normals);out=[]
 for m in [2,3]:
  for ids in itertools.combinations(range(n),m):
   if m==2:
    x,y=[normals[i] for i in ids]
    if cross(x,y) or dot(x,y)>=0:continue
    k=next(j for j in range(2) if x[j]);w=[-y[k]/(x[k]-y[k]),x[k]/(x[k]-y[k])]
   else:
    x,y,z=[normals[i] for i in ids];w=[cross(y,z),cross(z,x),cross(x,y)];s=sum(w)
    if not s:continue
    w=[x/s for x in w]
    if min(w)<=0:continue
   v=[F(0)]*n
   for i,x in zip(ids,w):v[i]=x
   if v not in out:out.append(v)
 return out
def direct(a,b,word,al,ga):
 # Literal four-dimensional HK symplectic sum, including zero same-factor terms.
 n=len(a);vectors=[[u[0],u[1],F(0),F(0)] for u in a]+[[F(0),F(0),u[0],u[1]] for u in b];weights=[x/2 for x in al+ga]
 def omega(x,y):return x[0]*y[2]+x[1]*y[3]-x[2]*y[0]-x[3]*y[1]
 return sum(weights[i]*weights[j]*omega(vectors[i],vectors[j]) for p,i in enumerate(word) for j in word[p+1:])
def parsevs(x):return [[F(t) for t in u] for u in x]
def barycentric(c,points,y,ids):
 d=len(ids);xs=[[p[i]-c[i] for i in ids] for p in points if p!=c];target=[y[i]-c[i] for i in ids]
 for js in itertools.combinations(range(len(xs)),d):
  matrix=[[xs[j][i] for j in js] for i in range(d)];vf=solve([[float(x) for x in row] for row in matrix],[float(x) for x in target])
  if vf is None or min(vf)<-1e-8 or sum(vf)>1+1e-8:continue
  v=solve(matrix,target)
  if v is not None and min(v)>=0 and sum(v)<=1:
   exactpoints=[p for p in points if p!=c]
   assert all((1-sum(v))*c[i]+sum(vk*exactpoints[j][i] for vk,j in zip(v,js))==y[i] for i in range(len(c)))
   return dict(indices=js,weights=list(map(str,v)),center_weight=str(1-sum(v)))
 raise AssertionError('No exact star barycentric certificate')
def verify_row(row):
 z=row['selection'];a,b=parsevs(z['centered_A']),parsevs(z['centered_B']);ca,cb=[parsevs(x) for x in z['closure_vertices']];pointsA,pointsB=[parsevs(x) for x in z['stars']];centers=[list(map(F,x)) for x in z['canonical_closures']];projectors=[parsevs(x) for x in z['projectors']];receipts=[]
 for normals,vs,points,c,p,coverage in zip([a,b],[ca,cb],[pointsA,pointsB],centers,projectors,z['containment']):
  assert sorted(vs)==sorted(closures(normals));n=len(normals)
  assert all(sum(x)==1 and min(x)>=0 and all(sum(xi*ai[k] for xi,ai in zip(x,normals))==0 for k in [0,1]) for x in points)
  assert all(p[i][j]==p[j][i] and sum(p[i][k]*p[k][j] for k in range(n))==p[i][j] for i in range(n) for j in range(n))
  assert sum(p[i][i] for i in range(n))==n-3
  assert all(sum(p[i][j] for i in range(n))==0 and all(sum(normals[i][k]*p[i][j] for i in range(n))==0 for k in [0,1]) for j in range(n))
  for j in range(n):
   ray=[p[i][j] for i in range(n)]
   if not any(ray):continue
   for t in [max(-x/u for x,u in zip(c,ray) if u>0),min(-x/u for x,u in zip(c,ray) if u<0)]:assert [x+t*u for x,u in zip(c,ray)] in points
  eta=F(coverage['eta']);ids=coverage['chart_indices'];reps=[]
  for v in vs:reps.append(barycentric(c,points,[ci+eta*(vi-ci) for ci,vi in zip(c,v)],ids))
  receipts.append(dict(eta=str(eta),vertex_count=len(vs),star_count=len(points),barycentric_certificates=reps))
 actual=[];checked=0
 for variant in z['variants']:
  word=variant['word'];qs=[]
  for point in pointsA:qs.append(max(direct(a,b,word,point,y) for y in cb));checked+=len(cb)
  for point in pointsB:qs.append(max(direct(a,b,word,y,point) for y in ca));checked+=len(ca)
  assert max(qs)==F(variant['best']['Q'])
  witness=variant['best'];assert direct(a,b,word,list(map(F,witness['alpha'])),list(map(F,witness['gamma'])))==max(qs);actual.extend(qs)
 q=max(actual);eta=max(F(x['eta']) for x in z['containment']);cap=F(z['capacity']);volume=F(z['volume']);assert cap==1/(2*q) and F(z['eta'])==eta and F(z['lower_capacity'])==eta*cap and F(z['ratio'])==cap*cap/(2*volume)
 return dict(name=row['name'],status='PASS',direct_four_dimensional_HK_values=checked,containment=receipts)
def main():
 parser=argparse.ArgumentParser();parser.add_argument('--input',type=Path,default=D/'predictions.json');parser.add_argument('--out',type=Path,default=D/'verification.json');args=parser.parse_args()
 t=time.perf_counter();path=args.input;p=json.loads(gzip.decompress(path.read_bytes()) if path.suffix=='.gz' else path.read_bytes());rows=[verify_row(row) for row in p['rows']];out=dict(status='PASS',rows=rows,input_file=path.name,input_sha256=hashlib.sha256(path.read_bytes()).hexdigest(),script_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),elapsed_seconds=time.perf_counter()-t,scope='Feasibility, tangent projectors, radial endpoints, complete single-factor vertices, exact contracted-vertex convex combinations, and complete star-response maxima; no paired original-capacity oracle')
 args.out.write_text(json.dumps(out,indent=2)+'\n');print('PASS',len(rows),'rows',sum(x['direct_four_dimensional_HK_values'] for x in rows),'direct HK values','SECONDS',out['elapsed_seconds'])
if __name__=='__main__':main()
