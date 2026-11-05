#!/usr/bin/env python3
"""Independent exact feasibility/HK/volume audit of one retained witness per body."""
import argparse,gzip,hashlib,json,math,sys,time
from fractions import Fraction as F
from itertools import combinations
from pathlib import Path
sys.set_int_max_str_digits(0)
def dot(a,b):return sum(x*y for x,y in zip(a,b))
def cross(a,b):return a[0]*b[1]-a[1]*b[0]
def area(a):
 vertices=[]
 for u,v in combinations(a,2):
  det=cross(u,v)
  if not det:continue
  x=[(v[1]-u[1])/det,(u[0]-v[0])/det]
  if all(dot(z,x)<=1 for z in a) and x not in vertices:vertices.append(x)
 mean=[sum(v[k] for v in vertices)/len(vertices) for k in range(2)];vertices.sort(key=lambda v:math.atan2(float(v[1]-mean[1]),float(v[0]-mean[0])));twice=sum(cross(vertices[i],vertices[(i+1)%len(vertices)]) for i in range(len(vertices)));assert twice>0;return twice/2

def main():
 p=argparse.ArgumentParser();p.add_argument('--directory',type=Path,default=Path(__file__).parent);p.add_argument('--source',type=Path,required=True);o=p.parse_args();d=o.directory;t=time.perf_counter();assert hashlib.sha256(o.source.read_bytes()).hexdigest()=='66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736';summary=json.loads((d/'broad-frozen-summary.json').read_text());ss={r['name']:r for r in summary['rows']};source={}
 for line in o.source.open():
  row=json.loads(line)
  if row['k']==row['m']==5:source[row['name']]=row
 names=[];count=0;maxerr=0
 with gzip.open(d/'broad-best-witnesses.jsonl.gz','rt') as g:
  for line in g:
   row=json.loads(line);name=row['name'];assert name not in names;names.append(name);raw=source[name];duals=[[F(x) for x in v] for v in raw['dual_vertices_rational']];a=[v[:2] for v in duals[:5]];b=[v[2:] for v in duals[5:]];an=list(map(int,row['alpha']['numerators']));ad=int(row['alpha']['denominator']);bn=list(map(int,row['gamma']['numerators']));bd=int(row['gamma']['denominator']);assert min(an)>=0 and min(bn)>=0 and sum(an)==ad and sum(bn)==bd and ad>0 and bd>0
   for k in range(2):assert sum(n*u[k] for n,u in zip(an,a))==sum(n*u[k] for n,u in zip(bn,b))==0
   word=row['word'];assert sorted(word)==list(range(10));pos={v:i for i,v in enumerate(word)}
   # Direct 4d HK after zero same-factor pairings are omitted; factor masses 1/2.
   numerator=sum(an[i]*bn[j]*dot(u,v)*(1 if pos[i]<pos[j+5] else -1) for i,u in enumerate(a) for j,v in enumerate(b));q=numerator/(4*ad*bd);assert q==F(row['Q'])>0;capacity=1/(2*q);volume=area(a)*area(b);ratio=capacity*capacity/(2*volume);r=ss[name];errs=[abs(float(x)/r[k]-1) for x,k in [(capacity,'capacity'),(volume,'volume'),(ratio,'ratio')]];maxerr=max([maxerr]+errs);assert max(errs)<1e-12;count+=1
   if count%200==0:print(count,flush=True)
 assert len(names)==len(source)==len(ss)==1024;out=dict(status='PASS',bodies=count,exact_original_normal_closures=2*count,exact_HK_witnesses=count,exact_area_products=count,maximum_summary_relative_error=maxerr,elapsed_seconds=time.perf_counter()-t,source_sha256=hashlib.sha256(o.source.read_bytes()).hexdigest(),summary_sha256=hashlib.sha256((d/'broad-frozen-summary.json').read_bytes()).hexdigest(),witnesses_sha256=hashlib.sha256((d/'broad-best-witnesses.jsonl.gz').read_bytes()).hexdigest(),checker_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),limits='Certifies feasible capacity/ratio upper bounds for exact dyadic source bodies; no capacity optimality, eigenspace selection, metric audit or historical-target certification.')
 (d/'broad-verification.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
if __name__=='__main__':main()
