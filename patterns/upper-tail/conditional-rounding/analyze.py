#!/usr/bin/env python3
"""Frozen retained-coupling conditional rounding; no pair search or LP solve."""
import argparse,hashlib,importlib.util,json,sys,time
from fractions import Fraction as F
from pathlib import Path

def module(name,path):
 s=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(s);sys.modules[name]=m;s.loader.exec_module(m);return m

def serial(v):
 if hasattr(v,'serial'):return v.serial()
 if isinstance(v,F):return str(v)
 if isinstance(v,dict):return {k:serial(w) for k,w in v.items()}
 if isinstance(v,(list,tuple)):return [serial(w) for w in v]
 return v

def round_coupling(a,b,variants,shadow,exact=True):
 ca=shadow.closures(a,0 if exact else 1e-11);cb=shadow.closures(b,0 if exact else 1e-11);n=len(a);m=len(b);candidates=[]
 for v in variants:
  w=v['word'];W=v['W'] if exact else v['numerical_W']
  for kind in ['row','column']:
   for k in range(n if kind=='row' else m):
    condition=[W[k*m+j] for j in range(m)] if kind=='row' else [W[i*m+k] for i in range(n)]
    total=sum(condition)
    if total<=(0 if exact else 1e-10):continue
    fixed=[x/total for x in condition]
    co=shadow.coefficients(a,b,w,fixed,'A' if kind=='row' else 'B')
    response=max(ca if kind=='row' else cb,key=lambda u:shadow.dot(u,co))
    alpha,gamma=(response,fixed) if kind=='row' else (fixed,response)
    q=shadow.Q(a,b,w,alpha,gamma)
    candidates.append(dict(shadow_sign=v['shadow_sign'],kind=kind,index=k,condition_mass=total,word=w,alpha=alpha,gamma=gamma,Q=q,capacity=1/(2*q) if q>0 else None))
 best=max(candidates,key=lambda v:v['Q']);return dict(closure_vertex_counts=[len(ca),len(cb)],candidate_count=len(candidates),candidates=candidates,best=best,capacity=best['capacity'])

def main():
 p=argparse.ArgumentParser();p.add_argument('--repo',type=Path,required=True);p.add_argument('--out',type=Path,required=True);args=p.parse_args();t=time.perf_counter();root=args.repo
 sh=module('round_shadow',root/'patterns/upper-tail/signed-prefix-response/analyze.py');Q5=module('round_quad',root/'patterns/upper-tail/conditional-coupling/quadratic.py').Q5
 paths=['patterns/upper-tail/conditional-coupling/hko-exact.json','patterns/upper-tail/conditional-coupling/results.json'];h=json.loads((root/paths[0]).read_text());a=[[Q5.parse(x) for x in u] for u in h['A']];b=[[Q5.parse(x) for x in u] for u in h['B']];variants=[dict(word=v['word'],W=[Q5.parse(x) for x in v['W']],shadow_sign=v['shadow_sign']) for v in h['variants']]
 hk=round_coupling(a,b,variants,sh);true=Q5.parse(h['Q_true']);hk['true_Q']=true;hk['exact_recovery']=hk['best']['Q']==true;hk['standalone_capacity_over_true']=true/hk['best']['Q'];print('HKO',hk['exact_recovery'],float(hk['best']['Q']),flush=True)
 out=dict(protocol='Frozen conditional row/column rounding, one opposite best response each; retained LP optima only',hko=hk,rows=[],regular=[],sources={v:hashlib.sha256((root/v).read_bytes()).hexdigest() for v in paths})
 if hk['exact_recovery']:
  data=json.loads((root/paths[1]).read_text())
  for r in data['rows']:
   a=[[F(x) for x in u] for u in r['A']];b=[[F(x) for x in u] for u in r['B']];v=[dict(word=v['word'],W=[F(x) for x in v['W']],shadow_sign=v['shadow_sign']) for v in r['variants']];s=round_coupling(a,b,v,sh);inc=F(r['feasible_capacity_upper']);lower=F(r['capacity_lower']);combined=min(s['capacity'],inc);row=dict(name=r['name'],rounding=s,incumbent_capacity=inc,combined_capacity=combined,capacity_lower=lower,round_improves_incumbent=s['capacity']<inc,round_gain_over_incumbent=1-float(s['capacity']/inc),standalone_over_lower=s['capacity']/lower,combined_over_lower=combined/lower,exact_capacity=combined==lower,historical_capacity=r['historical_capacity']);out['rows'].append(row);print(r['name'],'round',float(s['capacity']),'inc',float(inc),'gain',row['round_gain_over_incumbent'],'interval',float(combined/lower),flush=True)
  for r in data['regular']:
   s=round_coupling(r['A'],r['B'],r['variants'],sh,False);out['regular'].append(dict(theta=r['theta'],rounding=s,known_capacity=r['known_capacity'],standalone_over_known=s['capacity']/r['known_capacity']));print('regular',r['theta'],s['capacity']/r['known_capacity'],flush=True)
 out.update(elapsed_seconds=time.perf_counter()-t,plan_sha256=hashlib.sha256(Path(__file__).with_name('PLAN.md').read_bytes()).hexdigest(),analyze_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest());args.out.write_text(json.dumps(serial(out),indent=2)+'\n')
if __name__=='__main__':main()
