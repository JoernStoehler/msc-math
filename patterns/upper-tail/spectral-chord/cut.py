#!/usr/bin/env python3
"""Exact material cut versus retained width+triangle bound, no capacity oracle."""
import argparse,gzip,hashlib,json,sys
from fractions import Fraction as F
from pathlib import Path
from itertools import combinations
sys.set_int_max_str_digits(0)
def dot(a,b):return sum(x*y for x,y in zip(a,b))
def cross(a,b):return a[0]*b[1]-a[1]*b[0]
def steps(a):
 vv=[]
 for u,v in combinations(a,2):
  det=cross(u,v)
  if not det:continue
  x=[(v[1]-u[1])/det,(u[0]-v[0])/det]
  if all(dot(z,x)<=1 for z in a):vv.append(x)
 return [[x/(1+max(-dot(u,v) for v in vv)) for x in u] for u in a]
def main():
 p=argparse.ArgumentParser();p.add_argument('--directory',type=Path,default=Path(__file__).parent);p.add_argument('--source',type=Path,required=True);p.add_argument('--repo',type=Path,required=True);o=p.parse_args();d=o.directory;name='random_5x5_s1_229';source=next(json.loads(z) for z in o.source.open() if json.loads(z)['name']==name);duals=[[F(x) for x in u] for u in source['dual_vertices_rational']];a=[u[:2] for u in duals[:5]];b=[u[2:] for u in duals[5:]];width=1/max(abs(dot(u,v)) for u in steps(a) for v in steps(b))
 with gzip.open(o.repo/'patterns/upper-tail/tail-rows.jsonl.gz','rt') as g:old=next(r for r in map(json.loads,g) if r['name']==name)
 triangle=1/(2*F(old['certificate']['exact_Q']));oldcap=min(width,triangle)
 with gzip.open(d/'broad-best-witnesses.jsonl.gz','rt') as g:new=next(r for r in map(json.loads,g) if r['name']==name)
 cap=1/(2*F(new['Q']));drop=1-cap/oldcap;assert drop>F(1,10) and 1-cap/triangle>F(1,10);out=dict(name=name,exact_width=str(width),exact_triangle=str(triangle),exact_old_min=str(oldcap),exact_spectral_capacity=str(cap),exact_capacity_drop=str(drop),capacity_drop_float=float(drop),exact_drop_over_triangle=str(1-cap/triangle),drop_over_triangle_float=float(1-cap/triangle),old_selected_support_indices=old['selected_support_indices'],new_support_indices=[[i for i,n in enumerate(new[k]['numerators']) if int(n)>0] for k in ['alpha','gamma']],new_original_closure_weights=[[float(F(int(n),int(new[k]['denominator']))) for n in new[k]['numerators']] for k in ['alpha','gamma']],new_word=new['word'],strict_drop_above_tenth=True,historical_capacity=source['capacity'],source_sha256=hashlib.sha256(o.source.read_bytes()).hexdigest(),checker_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),limits='Imports retained triangle certificate and difference-body width identity; spectral feasibility separately checked in broad-verification.json; historical capacity remains numerical.');(d/'cut.json').write_text(json.dumps(out,indent=2)+'\n');print(name,'exact old',float(oldcap),'new',float(cap),'drop',float(drop))
if __name__=='__main__':main()
