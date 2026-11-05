#!/usr/bin/env python3
"""Independent frozen sample/summary audit. No discovery or capacity imports."""
import argparse,csv,gzip,hashlib,json,statistics
from fractions import Fraction as F
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('--repo',type=Path,required=True);p.add_argument('--directory',type=Path,required=True);a=p.parse_args();d=a.directory;r=json.loads((d/'results.json').read_text());sample=json.loads((d/'sample.json').read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
assert sha(d/'sample.json')==r['sample_sha256'] and sha(d/'PLAN.md')==r['plan_sha256'] and sha(d/'analyze.py')==r['runner_sha256']
for f,h in sample['sources'].items():assert sha(a.repo/f)==h
with gzip.open(a.repo/'patterns/product-shape/features.csv.gz','rt') as f:width=list(csv.DictReader(f))
expected=[]
for k,m in [(5,5),(5,6),(6,6)]:
    family=sorted([x for x in width if (int(x['k']),int(x['m']))==(k,m)],key=lambda x:(-float(x['sys']),x['name']))
    assert len(family)==1024
    tail=[x for x in family[:21] if x['name'] not in sample['excluded20']][:9]
    bulk=sorted([x for x in family[103:] if x['name'] not in sample['excluded20']],key=lambda x:hashlib.sha256(x['name'].encode()).hexdigest())[:3]
    for role,ss in [('principal_top2',tail),('hash_bulk_outside_top10',bulk)]:
        expected.extend(dict(name=x['name'],bucket=f'{k}x{m}',role=role,family_rank=family.index(x)+1) for x in ss)
assert expected==sample['sample'];assert len(expected)==36 and len(set(x['name'] for x in expected))==36
assert [dict((k,x[k]) for k in ['name','bucket','role','family_rank']) for x in r['rows']]==expected
w={x['name']:x for x in width}
for x in r['rows']:
    z=w[x['name']];assert x['historical_capacity']==float(z['capacity']) and x['historical_ratio']==float(z['sys'])
    assert F(x['capacity'])==F(x['selection']['capacity']);inc=min(F(x[k]) for k in ['width_capacity','triangle_capacity','contact_capacity','prior_response_capacity','canonical_capacity'])
    assert F(x['incumbent_capacity'])==inc and F(x['new_over_incumbent'])==F(x['capacity'])/inc
    assert F(x['triangle_capacity'])==1/(2*F(x['comparators']['triangle']['certificate']['exact_Q']))
    assert F(x['contact_capacity'])==F(x['comparators']['contact']['best']['capacity']) and F(x['prior_response_capacity'])==F(x['comparators']['prior_response']['capacity'])
    area=F(x['selection']['area_A'])*F(x['selection']['area_B'])
    assert F(x['exact_ratio_bound'])==F(x['capacity'])**2/(2*area) and F(x['exact_incumbent_ratio'])==inc**2/(2*area)
    assert x['capacity_over_historical']==float(F(x['capacity']))/x['historical_capacity'] and x['ratio_bound']==float(F(x['exact_ratio_bound']))

def hko(x):return x<=F(3,5) or (5*x-3)**2<=5

def audit(rows,s):
    assert s['n']==len(rows)
    for tag,key in [('new','capacity'),('incumbent','incumbent_capacity'),('width','width_capacity'),('triangle','triangle_capacity'),('contact','contact_capacity'),('prior_response','prior_response_capacity'),('canonical','canonical_capacity')]:
        gaps=[float(F(x[key]))/x['historical_capacity']-1 for x in rows];z=s[tag]
        assert z['within_1pct']==sum(-1e-8<=g<=.01+1e-10 for g in gaps) and z['within_5pct']==sum(-1e-8<=g<=.05+1e-10 for g in gaps)
        assert z['numerical_agreement']==sum(abs(g)<1e-10 for g in gaps) and z['negative_below_tolerance']==sum(g< -1e-8 for g in gaps)
        assert z['median_gap']==statistics.median(gaps) and z['max_gap']==max(gaps) and z['worst_name']==rows[max(range(len(rows)),key=lambda i:gaps[i])]['name']
    for key,condition in [('exact_improvements',lambda x:F(x['capacity'])<F(x['incumbent_capacity'])),('improvements_ge_1pct',lambda x:F(x['capacity'])<=F('0.99')*F(x['incumbent_capacity'])),('strict_regressions',lambda x:F(x['capacity'])>F(x['incumbent_capacity'])),('regressions_gt_1pct',lambda x:F(x['capacity'])>F('1.01')*F(x['incumbent_capacity'])),('regressions_gt_5pct',lambda x:F(x['capacity'])>F('1.05')*F(x['incumbent_capacity'])),('exact_ratio_le_1',lambda x:F(x['exact_ratio_bound'])<=1),('exact_ratio_le_HKO',lambda x:hko(F(x['exact_ratio_bound']))),('incumbent_ratio_le_1',lambda x:F(x['exact_incumbent_ratio'])<=1),('incumbent_ratio_le_HKO',lambda x:hko(F(x['exact_incumbent_ratio']))),('new_exclusions_le_1',lambda x:F(x['exact_ratio_bound'])<=1<F(x['exact_incumbent_ratio'])),('new_exclusions_le_HKO',lambda x:hko(F(x['exact_ratio_bound'])) and not hko(F(x['exact_incumbent_ratio'])))]:assert s[key]==sum(condition(x) for x in rows),key
    assert s['max_ratio_bound']==max(float(F(x['exact_ratio_bound'])) for x in rows)
allrows=r['rows'];tail=[x for x in allrows if x['role']=='principal_top2'];bulk=[x for x in allrows if x['role']!='principal_top2'];s=r['summary']
audit(allrows,s['all']);audit(tail,s['tail']);audit(bulk,s['bulk'])
for b in ['5x5','5x6','6x6']:
    audit([x for x in allrows if x['bucket']==b],s['families'][b]['all']);audit([x for x in tail if x['bucket']==b],s['families'][b]['tail'])
assert s['primary_all_tail_within5pct']==(s['tail']['new']['within_5pct']==27)
assert s['useful_positive']==(s['tail']['new']['within_1pct']>=24 and s['primary_all_tail_within5pct'] and s['tail']['improvements_ge_1pct']>=3)
obs=[x for x in tail if F(x['capacity'])>F('1.05')*F(x['incumbent_capacity'])];assert s['exact_5pct_obstructions']==[x['name'] for x in obs]
assert len(obs)==1 and obs[0]['name']=='random_5x6_s1_198' and F(obs[0]['new_over_incumbent'])>F('1.0789')
receipt=dict(status='PASS',sample_count=36,tail_count=27,bulk_count=9,source_and_code_hashes='PASS',sample_membership='PASS',all_stored_metrics='PASS',exact_ratio_arithmetic='PASS',obstruction_lower_bound='Cfull/Cinc > 1.0789 > 1.05',results_sha256=sha(d/'results.json'),scope='Independent membership, inherited width-label joins, exact comparison/ratio arithmetic and all metrics; geometry/witness feasibility delegated to separate checker.')
(d/'metrics-verification.json').write_text(json.dumps(receipt,indent=2)+'\n');print(json.dumps(receipt,indent=2))
