#!/usr/bin/env python3
"""Frozen design-excluded validation. Reads pinned inputs; writes --out only."""
import argparse,csv,gzip,hashlib,importlib.util,json,math,statistics,time
from fractions import Fraction as F
from pathlib import Path

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def load(name,path):
    s=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m

def hko_leq(x):
    return 5*x<=3 or (5*x-3)**2<=5

def metrics(rows):
    ans=dict(n=len(rows))
    for tag,key in [('new','capacity'),('incumbent','incumbent_capacity'),('width','width_capacity'),('triangle','triangle_capacity'),('contact','contact_capacity'),('prior_response','prior_response_capacity'),('canonical','canonical_capacity')]:
        gaps=[float(F(r[key]))/r['historical_capacity']-1 for r in rows]
        ans[tag]=dict(within_1pct=sum(-1e-8<=g<=.01+1e-10 for g in gaps),within_5pct=sum(-1e-8<=g<=.05+1e-10 for g in gaps),numerical_agreement=sum(abs(g)<1e-10 for g in gaps),negative_below_tolerance=sum(g< -1e-8 for g in gaps),median_gap=statistics.median(gaps),max_gap=max(gaps),worst_name=rows[max(range(len(rows)),key=lambda i:gaps[i])]['name'])
    ans.update(exact_improvements=sum(F(r['capacity'])<F(r['incumbent_capacity']) for r in rows),improvements_ge_1pct=sum(F(r['capacity'])<=F('0.99')*F(r['incumbent_capacity']) for r in rows),strict_regressions=sum(F(r['capacity'])>F(r['incumbent_capacity']) for r in rows),regressions_gt_1pct=sum(F(r['capacity'])>F('1.01')*F(r['incumbent_capacity']) for r in rows),regressions_gt_5pct=sum(F(r['capacity'])>F('1.05')*F(r['incumbent_capacity']) for r in rows),exact_ratio_le_1=sum(F(r['exact_ratio_bound'])<=1 for r in rows),exact_ratio_le_HKO=sum(hko_leq(F(r['exact_ratio_bound'])) for r in rows),incumbent_ratio_le_1=sum(F(r['exact_incumbent_ratio'])<=1 for r in rows),incumbent_ratio_le_HKO=sum(hko_leq(F(r['exact_incumbent_ratio'])) for r in rows),new_exclusions_le_1=sum(F(r['exact_ratio_bound'])<=1<F(r['exact_incumbent_ratio']) for r in rows),new_exclusions_le_HKO=sum(hko_leq(F(r['exact_ratio_bound'])) and not hko_leq(F(r['exact_incumbent_ratio'])) for r in rows),max_ratio_bound=max(float(F(r['exact_ratio_bound'])) for r in rows))
    return ans

def main():
    p=argparse.ArgumentParser();p.add_argument('--repo',type=Path,required=True);p.add_argument('--source',type=Path,required=True);p.add_argument('--out',type=Path,required=True);p.add_argument('--sample',type=Path,required=True);a=p.parse_args();start=time.monotonic();cpu=time.process_time();sample=json.loads(a.sample.read_text());root=a.repo
    assert sha(a.source)==sample['source_sha256']
    for path,digest in sample['sources'].items():assert sha(root/path)==digest,path
    rule=load('full',root/'patterns/upper-tail/full-partner-containment/analyze.py');response=rule.m;contact=response.old
    names={r['name'] for r in sample['sample']};assert len(names)==36 and not names.intersection(sample['excluded20'])
    source={}
    for line in a.source.open():
        r=json.loads(line)
        if r['name'] in names:source[r['name']]=r
    assert set(source)==names
    with gzip.open(root/'patterns/upper-tail/tail-rows.jsonl.gz','rt') as f:tri={r['name']:r for r in map(json.loads,f) if r['name'] in names}
    assert set(tri)==names
    with gzip.open(root/'patterns/upper-tail/geometry-aware-probe/rows.jsonl.gz','rt') as f:core={r['name']:r for r in map(json.loads,f) if r['name'] in names}
    rows=[]
    for membership in sample['sample']:
        assert time.monotonic()-start<1800 and time.process_time()-cpu<900
        r=source[membership['name']];v=[list(map(F,u)) for u in r['dual_vertices_rational']];aa=[u[:2] for u in v[:r['k']]];bb=[u[2:] for u in v[r['k']:]]
        s=rule.select(aa,bb);prior=response.select(aa,bb);con=prior['seed'];triangle=tri[r['name']];tc=1/(2*F(triangle['certificate']['exact_Q']));width=con['width_capacity'];cc=con['best']['capacity'];pc=prior['capacity'];ac=s['canonical_alpha'];bc=s['canonical_gamma'];canonical=[]
        for sign in [1,-1]:
            word=response.shadow_word(aa,bb,sign)
            if sign<0:word=word[::-1]
            q=response.Q(aa,bb,word,ac,bc);assert q>0
            canonical.append(dict(sign=sign,word=word,Q=q,alpha=ac,gamma=bc,capacity=1/(2*q)))
        can=min(x['capacity'] for x in canonical);capacities=dict(width=width,triangle=tc,contact=cc,prior_response=pc,canonical=can);inc=min(capacities.values());cap=s['capacity'];area=s['area_A']*s['area_B'];assert area==F(triangle['exact_original_area_product']);ratio=cap*cap/(2*area);iratio=inc*inc/(2*area)
        assert cap<=can
        row=dict(**membership,k=r['k'],m=r['m'],geometry=r['dual_vertices_rational'],selection=s,capacity=cap,comparators=dict(width_capacity=width,triangle=triangle,contact=con,prior_response=prior,canonical=canonical,incumbent_capacity=inc,incumbent_methods=[k for k,c in capacities.items() if c==inc]),incumbent_capacity=inc,width_capacity=width,triangle_capacity=tc,contact_capacity=cc,prior_response_capacity=pc,canonical_capacity=can,historical_capacity=r['capacity'],historical_ratio=r['sys'],capacity_over_historical=float(cap)/r['capacity'],new_over_incumbent=cap/inc,exact_ratio_bound=ratio,ratio_bound=float(ratio),exact_incumbent_ratio=iratio,ancillary_core_prediction=core[r['name']]['prediction'],ancillary_core_relative_error=core[r['name']]['prediction']/r['capacity']-1)
        rows.append(row)
    tail=[r for r in rows if r['role']=='principal_top2'];bulk=[r for r in rows if r['role']!='principal_top2'];summary=dict(all=metrics(rows),tail=metrics(tail),bulk=metrics(bulk),families={b:dict(all=metrics([r for r in rows if r['bucket']==b]),tail=metrics([r for r in tail if r['bucket']==b])) for b in ['5x5','5x6','6x6']})
    summary['primary_all_tail_within5pct']=summary['tail']['new']['within_5pct']==27
    summary['useful_positive']=summary['tail']['new']['within_1pct']>=24 and summary['primary_all_tail_within5pct'] and summary['tail']['improvements_ge_1pct']>=3
    summary['exact_5pct_obstructions']=[r['name'] for r in tail if r['capacity']>F('1.05')*r['incumbent_capacity']]
    result=dict(protocol='Frozen36design-excluded retrospective validation; unchanged four-variant full-partner rule; no fitting',source_sha256=sample['source_sha256'],sample_sha256=sha(a.sample),plan_sha256=sha(a.sample.with_name('PLAN.md')),runner_sha256=sha(Path(__file__)),sources=sample['sources'],rows=rows,summary=summary,elapsed_seconds=time.monotonic()-start,cpu_seconds=time.process_time()-cpu)
    a.out.write_text(json.dumps(response.stringify(result),indent=2)+'\n');print(json.dumps(summary,indent=2));print('SECONDS',result['elapsed_seconds'],'CPU',result['cpu_seconds'])
if __name__=='__main__':main()
