#!/usr/bin/env python3
"""Frozen 35-control conditional rounding; reads pinned packets, writes --out only."""
import argparse,hashlib,importlib.util,json,resource,sys,time,statistics
from fractions import Fraction as F
from pathlib import Path

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def module(name,path):
 s=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(s);sys.modules[name]=m;s.loader.exec_module(m);return m

def metrics(rows):
 out=dict(n=len(rows))
 for key in ['capacity','incumbent_capacity','combined_capacity']:
  gaps=[float(F(r[key]))/r['historical_capacity']-1 for r in rows]
  out[key]=dict(within1=sum(-1e-8<=x<=.01+1e-10 for x in gaps),within5=sum(-1e-8<=x<=.05+1e-10 for x in gaps),agreement=sum(abs(x)<1e-10 for x in gaps),below_label_tolerance=sum(x< -1e-8 for x in gaps),median_gap=statistics.median(gaps),max_gap=max(gaps),worst=rows[max(range(len(rows)),key=lambda i:gaps[i])]['name'])
 out.update(material_improvements=[r['name'] for r in rows if F(r['capacity'])<=F('0.99')*F(r['incumbent_capacity'])],strict_improvements=[r['name'] for r in rows if F(r['capacity'])<F(r['incumbent_capacity'])],regressions=[r['name'] for r in rows if F(r['capacity'])>F(r['incumbent_capacity'])],exact5pct_obstructions=[r['name'] for r in rows if F(r['capacity'])>F('1.05')*F(r['incumbent_capacity'])],exact_capacities=[r['name'] for r in rows if r['exact_capacity']],prior_exact_capacities=[r['name'] for r in rows if r['prior_exact_capacity']],new_exact_capacities=[r['name'] for r in rows if r['exact_capacity'] and not r['prior_exact_capacity']])
 return out

def main():
 p=argparse.ArgumentParser();p.add_argument('--repo',required=True,type=Path);p.add_argument('--out',required=True,type=Path);args=p.parse_args();root=args.repo;directory=Path(__file__).parent;start=time.monotonic();cpu=time.process_time();resource.setrlimit(resource.RLIMIT_CPU,(120,120));sample=json.loads((directory/'sample.json').read_text())
 for path,digest in sample['sources'].items():assert sha(root/path)==digest,path
 lp=module('validation_lp',root/'patterns/upper-tail/conditional-coupling/analyze.py');rnd=module('validation_round',root/'patterns/upper-tail/conditional-rounding/analyze.py');sh=module('validation_shadow',root/'patterns/upper-tail/signed-prefix-response/analyze.py');Q5=module('validation_q5',root/'patterns/upper-tail/conditional-coupling/quadratic.py').Q5
 h=json.loads((root/'patterns/upper-tail/conditional-coupling/hko-exact.json').read_text());ha=[[Q5.parse(x) for x in u] for u in h['A']];hb=[[Q5.parse(x) for x in u] for u in h['B']];hv=[dict(word=v['word'],W=[Q5.parse(x) for x in v['W']],shadow_sign=v['shadow_sign']) for v in h['variants']];hround=rnd.round_coupling(ha,hb,hv,sh);assert hround['best']['Q']==Q5.parse(h['Q_true']);hko=dict(exact_recovery=True,true_Q=h['Q_true'],rounding=hround)
 prior=json.loads((root/'patterns/upper-tail/full-partner-validation/results.json').read_text());lookup={r['name']:r for r in prior['rows']};rows=[];failures=[]
 for membership in sample['sample']:
  assert time.monotonic()-start<1800
  r=lookup[membership['name']];v=[list(map(F,u)) for u in r['geometry']];a=[u[:2] for u in v[:r['k']]];b=[u[2:] for u in v[r['k']:]]
  try:
   variants=[]
   for sign in [1,-1]:
    word=sh.shadow_word(a,b,sign)
    if sign<0:word=word[::-1]
    variants.append(lp.pilot(a,b,word,True)|{'shadow_sign':sign})
   rounding=rnd.round_coupling(a,b,variants,sh);cap=rounding['capacity'];inc=min(F(r['capacity']),F(r['incumbent_capacity']));combined=min(cap,inc);lower=1/(2*max(v['Q'] for v in variants));assert lower<=combined
   row=dict(**membership,A=a,B=b,variants=variants,rounding=rounding,capacity=cap,incumbent_capacity=inc,incumbent_methods=['full_partner' if F(r['capacity'])==inc else None,'previous_incumbent' if F(r['incumbent_capacity'])==inc else None],combined_capacity=combined,capacity_lower=lower,exact_capacity=combined==lower,prior_exact_capacity=inc==lower,historical_capacity=r['historical_capacity'],historical_ratio=r['historical_ratio'],standalone_over_incumbent=cap/inc,combined_interval_factor=combined/lower,prior_interval_factor=inc/lower,area_product=F(r['selection']['area_A'])*F(r['selection']['area_B']))
   rows.append(row);print(r['name'],'new/inc',float(cap/inc),'interval',float(combined/lower),'exact',row['exact_capacity'],flush=True)
  except Exception as e:
   failures.append(dict(name=r['name'],exception=type(e).__name__,message=str(e)));print('FAILURE',r['name'],repr(e),flush=True)
 summary={tag:metrics(rs) for tag,rs in [('all',rows),('tail',[r for r in rows if r['role']=='principal_top2']),('bulk',[r for r in rows if r['role']!='principal_top2'])]};summary['family_tail']={bucket:metrics([r for r in rows if r['bucket']==bucket and r['role']=='principal_top2']) for bucket in ['5x5','5x6','6x6']};summary['material_positive']=bool(summary['tail']['material_improvements']);summary['standalone5pct_bucket']=not failures and summary['tail']['capacity']['within5']==26
 out=dict(protocol='Frozen conditional LP and unchanged conditional-rounding rule; retrospective35design-excluded controls',sources=sample['sources'],plan_sha256=sha(directory/'PLAN.md'),sample_sha256=sha(directory/'sample.json'),analyze_sha256=sha(Path(__file__)),hko=hko,rows=rows,failures=failures,summary=summary,elapsed_seconds=time.monotonic()-start,cpu_seconds=time.process_time()-cpu)
 args.out.write_text(json.dumps(rnd.serial(out),indent=2)+'\n');print(json.dumps(summary,indent=2));print('FAILURES',failures,'SECONDS',out['elapsed_seconds'],'CPU',out['cpu_seconds'])
if __name__=='__main__':main()
