#!/usr/bin/env python3
"""Frozen input transfer: predict and oracle are separate commands."""
import argparse,hashlib,importlib.util,json,resource,sys,time
from fractions import Fraction as F
from pathlib import Path

def module(name,path):
 s=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(s);sys.modules[name]=m;s.loader.exec_module(m);return m

def serial(v):
 if isinstance(v,F):return str(v)
 if isinstance(v,dict):return {k:serial(w) for k,w in v.items()}
 if isinstance(v,(tuple,list)):return [serial(w) for w in v]
 return v

def support(v):return [i for i,x in enumerate(v) if x]
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def cpu():return resource.getrusage(resource.RUSAGE_SELF).ru_utime+resource.getrusage(resource.RUSAGE_SELF).ru_stime

def main():
 p=argparse.ArgumentParser();p.add_argument('mode',choices=['predict','oracle']);p.add_argument('--repo',type=Path,required=True);p.add_argument('--directory',type=Path,required=True);o=p.parse_args();r=o.repo;d=o.directory;t=time.perf_counter();cput=cpu()
 manifest=json.loads((d/'frozen-manifest.json').read_text())
 for path,value in manifest['sources'].items():assert sha(r/path)==value
 for path,value in manifest['freeze'].items():assert sha(d/path)==value
 sh=module('near_shadow',r/'patterns/upper-tail/signed-prefix-response/analyze.py');rd=module('near_round',r/'patterns/upper-tail/conditional-rounding/analyze.py')
 if o.mode=='predict':
  assert not (d/'predictions.json').exists(),'Predictions immutable; use fresh scratch for replay'
  cp=module('near_coupling',r/'patterns/upper-tail/conditional-coupling/analyze.py');rows=[]
  for source in json.loads((d/'sample.json').read_text())['rows']:
   row=dict(source);a=[list(map(F,u)) for u in row['A']];b=[list(map(F,u)) for u in row['B']];variants=[]
   try:
    for sign in [1,-1]:
     word=sh.shadow_word(a,b,sign)
     if sign<0:word=word[::-1]
     variants.append(cp.pilot(a,b,word,True)|dict(shadow_sign=sign))
    row.update(variants=variants,rounding=rd.round_coupling(a,b,variants,sh),status='OK')
   except Exception as e:
    row.update(variants=variants,status='RECONSTRUCTION_FAILURE',error=repr(e));rows.append(row);print(row['name'],row['status'],row['error'],flush=True);break
   rows.append(row);print(row['name'],'PREDICTED',float(row['rounding']['capacity']),flush=True)
  out=dict(rows=rows,frozen_membership=12,predicted_count=len(rows),sample_sha256=sha(d/'sample.json'),plan_sha256=sha(d/'PLAN.md'),analyze_sha256=sha(Path(__file__)),elapsed_seconds=time.perf_counter()-t,cpu_seconds=cpu()-cput)
  (d/'predictions.json').write_text(json.dumps(serial(out),indent=2)+'\n');print('PREDICTIONS_SHA256',sha(d/'predictions.json'));print('CPU_SECONDS',out['cpu_seconds']);return
 pred=json.loads((d/'predictions.json').read_text());rows=[]
 for s in pred['rows']:
  if s['status']!='OK':break
  row=dict(s);a=[list(map(F,u)) for u in s['A']];b=[list(map(F,u)) for u in s['B']];ca=sh.closures(a);cb=sh.closures(b);candidates=[]
  for variant in s['variants']:
   word=variant['word']
   for ia,alpha in enumerate(ca):
    for ib,gamma in enumerate(cb):
     q=sh.Q(a,b,word,alpha,gamma);candidates.append(dict(shadow_sign=variant['shadow_sign'],A_vertex=ia,B_vertex=ib,alpha=alpha,gamma=gamma,word=word,Q=q))
  q=max(v['Q'] for v in candidates);ties=[v for v in candidates if v['Q']==q];best=ties[0];cap=1/(2*q);volume=sh.old.area(a)*sh.old.area(b);rounded=F(s['rounding']['best']['Q']);assert rounded<=q
  conditional={};coverage={}
  for variant in s['variants']:
   W=list(map(F,variant['W']));key=str(variant['shadow_sign']);cc={}
   for factor,n,closure in [('A',5,ca),('B',5,cb)]:
    vals=[]
    for k in range(5):
     v=[W[i*5+k] for i in range(5)] if factor=='A' else W[k*5:k*5+5];mass=sum(v)
     if mass:vals.append(dict(index=k,weights=[x/mass for x in v],support=support(v),closure_vertex=next((i for i,c in enumerate(closure) if c==[x/mass for x in v]),None)))
    cc[factor]=vals
   conditional[key]=cc;coverage[key]={factor:sorted({v['closure_vertex'] for v in cc[factor] if v['closure_vertex'] is not None}) for factor in ['A','B']}
  mechanism=dict(closure_vertices_A=ca,closure_vertices_B=cb,conditionals=conditional,conditional_vertex_coverage=coverage,winning_pairs=[dict(shadow_sign=v['shadow_sign'],A_vertex=v['A_vertex'],B_vertex=v['B_vertex'],A_support=support(v['alpha']),B_support=support(v['gamma']),A_conditionally_seen=v['A_vertex'] in coverage[str(v['shadow_sign'])]['A'],B_conditionally_seen=v['B_vertex'] in coverage[str(v['shadow_sign'])]['B']) for v in ties],rounded_supports=dict(A=support(list(map(F,s['rounding']['best']['alpha']))),B=support(list(map(F,s['rounding']['best']['gamma'])))),rank_one_by_shadow={str(v['shadow_sign']):[F(x) for x in v['W']]==[sum(F(v['W'][i*5+j]) for j in range(5))*sum(F(v['W'][i*5+j]) for i in range(5)) for i in range(5) for j in range(5)] for v in s['variants']})
  row.update(truth=best|dict(capacity=cap,all_ties=ties,oracle_candidates=candidates),volume=volume,ratio=cap*cap/(2*volume),exact_recovery=rounded==q,capacity_over_true=q/rounded,relative_capacity_error=q/rounded-1,at_least_one_percent=q/rounded>=F(101,100),mechanism=mechanism);rows.append(row)
  print(row['name'],'exact',row['exact_recovery'],'err',float(q/rounded-1),'sys',float(row['ratio']),'coverage',coverage,'winner',mechanism['winning_pairs'],flush=True)
  if rounded!=q:break
 out=dict(rows=rows,frozen_membership=12,scored_count=len(rows),unscored_names=[v['name'] for v in pred['rows'][len(rows):]],predictions_sha256=sha(d/'predictions.json'),sample_sha256=sha(d/'sample.json'),plan_sha256=sha(d/'PLAN.md'),analyze_sha256=sha(Path(__file__)),elapsed_seconds=time.perf_counter()-t,cpu_seconds=cpu()-cput,protocol='Full closure-pair two-shadow exact oracle, after immutable predictions; stop at first strict miss')
 (d/'results.json').write_text(json.dumps(serial(out),indent=2)+'\n');print('CPU_SECONDS',out['cpu_seconds'])
if __name__=='__main__':main()
