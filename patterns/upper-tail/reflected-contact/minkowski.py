#!/usr/bin/env python3
"""Frozen reflected-cover contact-face feature; exact rational controls."""
import argparse,hashlib,importlib.util,json,math,statistics,time
from fractions import Fraction as F
from pathlib import Path

def module(n,p):
 sp=importlib.util.spec_from_file_location(n,p);m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m);return m
def dot(x,y):return sum(a*b for a,b in zip(x,y))
def serial(x):
 if isinstance(x,F):return str(x)
 if isinstance(x,dict):return {k:serial(v) for k,v in x.items()}
 if isinstance(x,(list,tuple)):return [serial(v) for v in x]
 return x

def face(a,sh,tol=0):
 vv=sh.old.vertices(a,tol);H=[max(-dot(u,v) for v in vv) for u in a]
 cc=sh.closures(a,tol);objectives=[dot(w,H) for w in cc];best=max(objectives)
 selected=[w for w,x in zip(cc,objectives) if abs(x-best)<=tol]
 return dict(asymmetry=best,opposite_supports=H,all_closure_count=len(cc),selected_closure_count=len(selected),weights=selected,contact_indices=sorted({i for w in selected for i,x in enumerate(w) if x>tol}))

def score(a,b,sh,tol=0):
 fa,fb=face(a,sh,tol),face(b,sh,tol);candidates=[]
 for sign in [1,-1]:
  word=sh.shadow_word(a,b,sign,tol)
  if sign<0:word=word[::-1]
  for i,al in enumerate(fa['weights']):
   for j,ga in enumerate(fb['weights']):candidates.append(dict(shadow_sign=sign,word=word,alpha=al,gamma=ga,A_face_vertex=i,B_face_vertex=j,Q=sh.Q(a,b,word,al,ga)))
 witness=max(candidates,key=lambda r:r['Q']);assert witness['Q']>0
 def area(u):
  vv=sh.old.vertices(u,tol);center=[sum(v[k] for v in vv)/len(vv) for k in [0,1]];vv.sort(key=lambda v:math.atan2(float(v[1]-center[1]),float(v[0]-center[0])))
  return abs(sum(vv[i][0]*vv[(i+1)%len(vv)][1]-vv[i][1]*vv[(i+1)%len(vv)][0] for i in range(len(vv))))/2
 V=area(a)*area(b);C=1/(2*witness['Q']);S=C*C/(2*V)
 return dict(A_face=fa,B_face=fb,witness=witness,volume=V,capacity_bound=C,ratio_bound=S,evaluated_closure_pairs=len(fa['weights'])*len(fb['weights']),all_closure_pair_count=fa['all_closure_count']*fb['all_closure_count'])

def summarize(rows):
 gaps=[r['capacity_over_truth']-1 for r in rows];errors=[r['ratio_over_truth']-1 for r in rows]
 return dict(n=len(rows),capacity_gap_median=statistics.median(gaps),capacity_gap_max=max(gaps),ratio_gap_median=statistics.median(errors),ratio_gap_max=max(errors),ratio_bound_above_HKO=sum(r['ratio_bound_float']>(3+math.sqrt(5))/5+1e-12 for r in rows),one_selected_pair=sum(r['feature']['evaluated_closure_pairs']==1 for r in rows),exactly_tight=sum(abs(x)<1e-12 for x in gaps))

def main():
 p=argparse.ArgumentParser();p.add_argument('--repo',type=Path,required=True);p.add_argument('--out',type=Path,required=True);p.add_argument('--source',type=Path,required=True);o=p.parse_args();start=time.perf_counter();d=o.out;d.mkdir(parents=True,exist_ok=True)
 sh=module('minkowski_shadow',o.repo/'patterns/upper-tail/signed-prefix-response/analyze.py');src=o.repo/'patterns/upper-tail/contact-stability'
 sample=json.loads((src/'sample.json').read_text());counter=json.loads((src/'global-counterexample-input.json').read_text())
 pilot_names=['random_5x5_s1_64','random_5x6_s2_150','random_6x6_s3_124'];pilots={}
 assert hashlib.sha256(o.source.read_bytes()).hexdigest()=='66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736'
 for line in o.source.open():
  r=json.loads(line)
  if r['name'] in pilot_names:pilots[r['name']]=r
 assert len(pilots)==3
 geom=[dict(name=r['name'],kind='constructed',A=r['A'],B=r['B']) for r in sample['rows']]+[dict(name=counter['name'],kind='global-counterexample',A=counter['A'],B=counter['B'])]
 for name in pilot_names:
  r=pilots[name];v=r['dual_vertices_rational'];geom.append(dict(name=name,kind='random-pilot',A=[u[:2] for u in v[:r['k']]],B=[u[2:] for u in v[r['k']:]]))
 # ALL geometry features are frozen to disk before loading any oracle truth.
 features=[]
 for r in geom:
  a=[list(map(F,u)) for u in r['A']];b=[list(map(F,u)) for u in r['B']]
  features.append(dict(name=r['name'],kind=r['kind'],feature=score(a,b,sh)))
 (d/'minkowski-predictions.json').write_text(json.dumps(serial(dict(rows=features,plan_sha256=hashlib.sha256((d/'MINKOWSKI-PLAN.md').read_bytes()).hexdigest())),indent=2)+'\n')
 truths={r['name']:(F(r['capacity']),F(r['ratio'])) for r in json.loads((src/'truth.json').read_text())['rows']}
 gc=json.loads((src/'global-counterexample.json').read_text());truths[counter['name']]=(F(gc['exact_capacity']),F(gc['exact_ratio']))
 for n,r in pilots.items():truths[n]=(F(r['capacity']),F(r['sys']))
 rows=[]
 for r in features:
  cap,ratio=truths[r['name']];f=r['feature'];assert float(f['capacity_bound']/cap)>1-1e-11
  rows.append(r|dict(truth_capacity=cap,truth_ratio=ratio,capacity_over_truth=float(f['capacity_bound']/cap),ratio_over_truth=float(f['ratio_bound']/ratio),ratio_bound_float=float(f['ratio_bound'])))
 regular=[];h=math.cos(math.pi/5)
 for theta in [0,math.pi/20,math.pi/10]:
  a=[(math.cos((2*i+1)*math.pi/5)/h,math.sin((2*i+1)*math.pi/5)/h) for i in range(5)];b=[(math.cos((2*i+1)*math.pi/5+theta)/h,math.sin((2*i+1)*math.pi/5+theta)/h) for i in range(5)]
  f=score(a,b,sh,1e-11);known=(1+h)**2/math.cos(theta);assert abs(f['capacity_bound']/known-1)<1e-10
  regular.append(dict(theta=theta,selected_face_counts=[f['A_face']['selected_closure_count'],f['B_face']['selected_closure_count']],capacity=float(f['capacity_bound']),known_capacity=known,ratio=float(f['ratio_bound'])))
 panel=[r for r in rows if r['kind']=='constructed'];above=[r for r in panel if r['truth_ratio']>1]
 summary=dict(all_constructed=summarize(panel),above_one_constructed=summarize(above),counterexample=[{k:v for k,v in r.items() if k!='feature'}|{'selected_face_counts':[r['feature']['A_face']['selected_closure_count'],r['feature']['B_face']['selected_closure_count']]} for r in rows if r['kind']=='global-counterexample'],random_pilots=[{k:v for k,v in r.items() if k!='feature'}|{'selected_face_counts':[r['feature']['A_face']['selected_closure_count'],r['feature']['B_face']['selected_closure_count']]} for r in rows if r['kind']=='random-pilot'],regular_controls=regular,seconds=time.perf_counter()-start)
 (d/'minkowski-results.json').write_text(json.dumps(serial(dict(rows=rows,summary=summary,script_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest())),indent=2)+'\n');print(json.dumps(serial(summary),indent=2))
if __name__=='__main__':main()
