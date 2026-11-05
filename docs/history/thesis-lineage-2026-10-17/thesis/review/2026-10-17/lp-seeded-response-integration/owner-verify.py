#!/usr/bin/env python3
"""Independent LP-seeded response replay using direct 4D Q and Gaussian closures."""
import argparse, hashlib, importlib.util, json
from fractions import Fraction as F
from pathlib import Path

def module(path):
    spec=importlib.util.spec_from_file_location('root_geometry',path)
    obj=importlib.util.module_from_spec(spec);spec.loader.exec_module(obj);return obj

def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def serial(x):
    if isinstance(x,F):return str(x)
    if isinstance(x,dict):return {k:serial(v) for k,v in x.items()}
    if isinstance(x,(list,tuple)):return [serial(v) for v in x]
    return x

def replay(g,a,b,variants):
    ca,cb=g.closures(a),g.closures(b);records=[]
    for variant in variants:
        g.audit_lp(a,b,variant);w=variant['word'];W=list(map(F,variant['W']));n,m=len(a),len(b)
        for kind in ['row','column']:
            for index in range(n if kind=='row' else m):
                c=[W[index*m+j] for j in range(m)] if kind=='row' else [W[i*m+index] for i in range(n)]
                mass=sum(c)
                if not mass:continue
                fixed=[v/mass for v in c]
                if kind=='row':
                    y=fixed;g.feasible(b,y)
                    x=max(ca,key=lambda z:g.direct(a,b,w,z,y))
                    one=g.direct(a,b,w,x,y)
                    y=max(cb,key=lambda z:g.direct(a,b,w,x,z))
                else:
                    x=fixed;g.feasible(a,x)
                    y=max(cb,key=lambda z:g.direct(a,b,w,x,z))
                    one=g.direct(a,b,w,x,y)
                    x=max(ca,key=lambda z:g.direct(a,b,w,z,y))
                states=[(x[:],y[:],g.direct(a,b,w,x,y))];assert states[-1][2]>=one
                changed=True
                while changed:
                    changed=False
                    for factor in (['A','B'] if kind=='row' else ['B','A']):
                        old=g.direct(a,b,w,x,y)
                        if factor=='A':
                            z=max(ca,key=lambda z:g.direct(a,b,w,z,y));val=g.direct(a,b,w,z,y)
                            if val>old:x=z;changed=True;states.append((x[:],y[:],val))
                        else:
                            z=max(cb,key=lambda z:g.direct(a,b,w,x,z));val=g.direct(a,b,w,x,z)
                            if val>old:y=z;changed=True;states.append((x[:],y[:],val))
                    assert len(states)<=len(ca)*len(cb)
                q=g.direct(a,b,w,x,y)
                assert all(g.direct(a,b,w,z,y)<=q for z in ca)
                assert all(g.direct(a,b,w,x,z)<=q for z in cb)
                assert all(states[i][2]<states[i+1][2] for i in range(len(states)-1))
                records.append(dict(shadow_sign=variant['shadow_sign'],kind=kind,index=index,alpha=x,gamma=y,Q=q,one_response_Q=one,initialized_Q=states[0][2],strict_updates=len(states)-1,states=states))
    best=max(records,key=lambda r:r['Q'])
    return dict(best=best,candidates=records,vertex_counts=[len(ca),len(cb)])

def main():
    p=argparse.ArgumentParser();p.add_argument('--directory',type=Path,required=True);p.add_argument('--thesis',type=Path,required=True);p.add_argument('--patterns',type=Path,required=True);p.add_argument('--out',type=Path,required=True);o=p.parse_args();d=o.directory
    g=module(o.thesis/'thesis/review/2026-10-17/near-hko-transfer-integration/owner-verify.py')
    frozen=json.loads(Path(__file__).with_name('root-freeze.json').read_text())
    for n,hsh in frozen.items():assert digest(d/n)==hsh,n
    manifest=json.loads((d/'frozen-manifest.json').read_text())
    for n,hsh in manifest['sources'].items():assert digest(o.patterns/n)==hsh,n
    for n,hsh in manifest['freeze'].items():assert digest(d/n)==hsh,n
    sample=json.loads((d/'sample.json').read_text());pred=json.loads((d/'predictions.json').read_text());result=json.loads((d/'results.json').read_text())
    pred_frozen=json.loads(Path(__file__).with_name('root-prediction-freeze.json').read_text());assert digest(d/'predictions.json')==pred_frozen['predictions.json']
    hko=json.loads((o.patterns/'patterns/upper-tail/conditional-coupling/hko-exact.json').read_text())
    base=[[F(x['rational'])+F(x['sqrt5'])*F(2236068,1000000) for x in row] for row in hko['A']+hko['B']]
    assert len(sample['rows'])==len(pred['rows'])==12
    assert [r['name'] for r in sample['rows']]==[r['name'] for r in pred['rows']]
    assert [r['name'] for r in result['rows']]==[r['name'] for r in sample['rows'][:len(result['rows'])]]
    replayed={};counts=dict(LP_certificates=0,seed_trajectories=0,strict_updates=0,completed_initializations=0,exact_oracle_pairs=0)
    for index,(s,r) in enumerate(zip(sample['rows'],pred['rows'])):
        seed=index//2;epsilon=F(1,[100,20][index%2]);raw=hashlib.sha256(('msc-math/thesis/lp-seeded-response/validation/v1:'+str(seed)).encode()).digest()
        expected=[[base[j][k]+epsilon*F(raw[2*j+k]%9-4,4) for k in range(2)] for j in range(10)]
        assert expected==[list(map(F,u)) for u in s['A']+s['B']]
        assert r['A']==s['A'] and r['B']==s['B']
        a,b=[list(map(F,u)) for u in r['A']],[list(map(F,u)) for u in r['B']]
        volume=g.area(a)*g.area(b);re=replay(g,a,b,r['variants']);counts['LP_certificates']+=len(r['variants'])
        e=r['extraction'];assert len(re['candidates'])==e['candidate_count']==len(e['seeds'])
        for actual,claimed in zip(re['candidates'],e['seeds']):
            assert (actual['shadow_sign'],actual['kind'],actual['index'])==(claimed['shadow_sign'],claimed['kind'],claimed['index'])
            assert actual['alpha']==list(map(F,claimed['terminal_alpha'])) and actual['gamma']==list(map(F,claimed['terminal_gamma']))
            assert actual['Q']==F(claimed['Q']);states=claimed['states']
            assert len(states)==2+actual['strict_updates']
            assert F(states[0]['Q'])==actual['one_response_Q']
            for (x,y,q),state in zip(actual['states'],states[1:]):
                assert x==list(map(F,state['alpha'])) and y==list(map(F,state['gamma'])) and q==F(state['Q'])
            counts['seed_trajectories']+=1;counts['completed_initializations']+=1;counts['strict_updates']+=actual['strict_updates']
        assert re['best']['Q']==F(e['best']['Q'])
        assert re['best']['alpha']==list(map(F,e['best']['terminal_alpha']))
        assert re['best']['gamma']==list(map(F,e['best']['terminal_gamma']))
        replayed[r['name']]=(r,a,b,volume,re)
    rows=[]
    for row in result['rows']:
        r,a,b,volume,re=replayed[row['name']];ca,cb=g.closures(a),g.closures(b)
        truth=max(g.prefix_value(a,b,x,y) for x in ca for y in cb);counts['exact_oracle_pairs']+=len(ca)*len(cb)
        predicted=re['best']['Q'];capacity=1/(2*truth);ratio=capacity*capacity/(2*volume)
        assert truth==F(row['Q_true']) and predicted==F(row['Q_predicted'])
        assert F(row['capacity_true'])==capacity and F(row['volume'])==volume and F(row['ratio'])==ratio
        assert row['exact_recovery']==(predicted==truth)
        assert F(row['relative_capacity_error'])==truth/predicted-1
        rows.append(dict(name=row['name'],ratio=ratio,ratio_float=float(ratio),exact_recovery=predicted==truth,capacity=capacity,relative_capacity_error=truth/predicted-1,vertex_counts=re['vertex_counts'],largest_strict_updates=max(x['strict_updates'] for x in re['candidates'])))
    out=dict(status='PASS',freeze=frozen,hashes={n:digest(d/n) for n in ['sample.json','predictions.json','results.json','PLAN.md']},checker_sha256=digest(Path(__file__)),math_utilities_sha256=digest(o.thesis/'thesis/review/2026-10-17/near-hko-transfer-integration/owner-verify.py'),predicted=12,scored=len(rows),unscored=12-len(rows),exact_recoveries=sum(r['exact_recovery'] for r in rows),above_one=sum(r['ratio']>1 for r in rows),counts=counts,rows=rows,scope='Independent Gaussian closures/direct4D deterministic seed-response replay and LP dual audit for all predictions; separate prefix oracle for scored controls only. No global guarantee or LP uniqueness claim.')
    o.out.write_text(json.dumps(serial(out),indent=2)+'\n');print(json.dumps({k:v for k,v in serial(out).items() if k not in ['rows','freeze','hashes']},indent=2))

if __name__=='__main__':main()
