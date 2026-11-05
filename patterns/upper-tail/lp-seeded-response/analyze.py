#!/usr/bin/env python3
"""Prediction and ground truth are separate immutable stages."""
import argparse,hashlib,importlib.util,json,resource,sys,time
from fractions import Fraction as F
from pathlib import Path

def module(name,path):
 s=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(s);sys.modules[name]=m;s.loader.exec_module(m);return m

def serial(v):
 if isinstance(v,F):return str(v)
 if isinstance(v,dict):return {k:serial(x) for k,x in v.items()}
 if isinstance(v,(tuple,list)):return [serial(x) for x in v]
 return v

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def cpu():return resource.getrusage(resource.RUSAGE_SELF).ru_utime+resource.getrusage(resource.RUSAGE_SELF).ru_stime

def extract(a,b,variants,sh):
 ca=sh.closures(a);cb=sh.closures(b);seeds=[]
 for variant in variants:
  W=list(map(F,variant['W']));word=variant['word']
  for kind in ['row','column']:
   order=['A','B'] if kind=='row' else ['B','A']
   for k in range(5):
    condition=W[k*5:k*5+5] if kind=='row' else [W[i*5+k] for i in range(5)];mass=sum(condition)
    if not mass:continue
    fixed=[x/mass for x in condition];alpha=None if kind=='row' else fixed;gamma=fixed if kind=='row' else None;states=[]
    def response(which):
     co=sh.coefficients(a,b,word,gamma if which=='A' else alpha,which);return max(ca if which=='A' else cb,key=lambda v:sh.dot(v,co))
    for which in order:
     best=response(which)
     if which=='A':alpha=best
     else:gamma=best
     states.append(dict(stage='initialization',factor=which,alpha=alpha[:],gamma=gamma[:],Q=sh.Q(a,b,word,alpha,gamma)))
    sweeps=0
    while True:
     sweeps+=1;changed=False
     for which in order:
      best=response(which);current=sh.Q(a,b,word,alpha,gamma);value=sh.Q(a,b,word,best,gamma) if which=='A' else sh.Q(a,b,word,alpha,best)
      if value>current:
       if which=='A':alpha=best
       else:gamma=best
       states.append(dict(stage='strict_update',factor=which,alpha=alpha[:],gamma=gamma[:],Q=value));changed=True
     if not changed:break
    value=sh.Q(a,b,word,alpha,gamma);seeds.append(dict(shadow_sign=variant['shadow_sign'],kind=kind,index=k,condition_mass=mass,word=word,states=states,terminal_alpha=alpha,terminal_gamma=gamma,Q=value,capacity=1/(2*value),sweeps=sweeps))
 best=max(seeds,key=lambda v:v['Q']);return dict(candidate_count=len(seeds),seeds=seeds,best=best,capacity=best['capacity'],vertex_counts=[len(ca),len(cb)])

def main():
 p=argparse.ArgumentParser();p.add_argument('mode',choices=['diagnostic','predict','oracle']);p.add_argument('--repo',type=Path,required=True);p.add_argument('--directory',type=Path,required=True);o=p.parse_args();r=o.repo;d=o.directory;t=time.perf_counter();ct=cpu()
 frozen=json.loads((d/'frozen-manifest.json').read_text())
 for k,v in frozen['sources'].items():assert sha(r/k)==v
 for k,v in frozen['freeze'].items():assert sha(d/k)==v
 sh=module('lpseed_shadow',r/'patterns/upper-tail/signed-prefix-response/analyze.py')
 if o.mode in ['diagnostic','predict']:
  output='diagnostics.json' if o.mode=='diagnostic' else 'predictions.json';assert not (d/output).exists();rows=[]
  if o.mode=='diagnostic':
   old=json.loads((r/'patterns/upper-tail/near-hko-transfer/predictions.json').read_text());src=[next(z for z in old['rows'] if z['name']=='near_hko_s4_e100')];coupling=json.loads((r/'patterns/upper-tail/conditional-coupling/results.json').read_text());src.append(next(z for z in coupling['rows'] if z['name']=='random_5x5_s1_64'))
  else:
   cp=module('lpseed_coupling',r/'patterns/upper-tail/conditional-coupling/analyze.py');src=json.loads((d/'sample.json').read_text())['rows']
  for source in src:
   row={k:source[k] for k in ['name','A','B']};a=[list(map(F,v)) for v in row['A']];b=[list(map(F,v)) for v in row['B']]
   try:
    if o.mode=='diagnostic':variants=source['variants']
    else:
     variants=[]
     for sign in [1,-1]:
      word=sh.shadow_word(a,b,sign)
      if sign<0:word=word[::-1]
      variants.append(cp.pilot(a,b,word,True)|dict(shadow_sign=sign))
    row.update(variants=variants,extraction=extract(a,b,variants,sh),status='OK')
   except Exception as e:
    row.update(status='FAILURE',error=repr(e));rows.append(row);break
   rows.append(row);print(row['name'],row['status'],float(row['extraction']['capacity']),flush=True)
  out=dict(rows=rows,sample_sha256=sha(d/'sample.json'),plan_sha256=sha(d/'PLAN.md'),analyze_sha256=sha(Path(__file__)),elapsed_seconds=time.perf_counter()-t,cpu_seconds=cpu()-ct,role=('exposed design diagnostics; not validation' if o.mode=='diagnostic' else 'all fresh predictions frozen before oracle'))
  (d/output).write_text(json.dumps(serial(out),indent=2)+'\n');print('OUTPUT_SHA256',sha(d/output));print('CPU_SECONDS',out['cpu_seconds']);return
 assert not (d/'results.json').exists();pred=json.loads((d/'predictions.json').read_text());rows=[]
 for source in pred['rows']:
  if source['status']!='OK':break
  a=[list(map(F,v)) for v in source['A']];b=[list(map(F,v)) for v in source['B']];ca=sh.closures(a);cb=sh.closures(b);table=[]
  for variant in source['variants']:
   for ia,alpha in enumerate(ca):
    for ib,gamma in enumerate(cb):table.append(dict(shadow_sign=variant['shadow_sign'],A_vertex=ia,B_vertex=ib,alpha=alpha,gamma=gamma,word=variant['word'],Q=sh.Q(a,b,variant['word'],alpha,gamma)))
  best=max(table,key=lambda v:v['Q']);qt=best['Q'];qp=F(source['extraction']['best']['Q']);volume=sh.old.area(a)*sh.old.area(b);row=dict(name=source['name'],Q_predicted=qp,Q_true=qt,capacity_true=1/(2*qt),exact_recovery=qp==qt,relative_capacity_error=qt/qp-1,volume=volume,ratio=1/(8*qt*qt*volume),winners=[v for v in table if v['Q']==qt],oracle_candidates=table);rows.append(row);print(row['name'],'exact',row['exact_recovery'],'sys',float(row['ratio']),'err',float(row['relative_capacity_error']),flush=True)
  if qp!=qt:break
 out=dict(rows=rows,scored_count=len(rows),unscored_names=[v['name'] for v in pred['rows'][len(rows):]],predictions_sha256=sha(d/'predictions.json'),elapsed_seconds=time.perf_counter()-t,cpu_seconds=cpu()-ct,protocol='Oracle only after all predictions, first strict miss stop')
 (d/'results.json').write_text(json.dumps(serial(out),indent=2)+'\n');print('CPU_SECONDS',out['cpu_seconds'])
if __name__=='__main__':main()
