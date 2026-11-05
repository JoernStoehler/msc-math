#!/usr/bin/env python3
"""Independent exact support-trap and mixture audit after frozen prediction scoring."""
import argparse,importlib.util,json
from pathlib import Path
from fractions import Fraction as F

def loadmodule(name,p):
    s=importlib.util.spec_from_file_location(name,p);m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m

def main():
    p=argparse.ArgumentParser();p.add_argument('--directory',type=Path,required=True);p.add_argument('--thesis',type=Path,required=True);p.add_argument('--out',type=Path,required=True);o=p.parse_args();d=o.directory
    m=loadmodule('root_replay',Path(__file__).with_name('owner-verify.py'));g=loadmodule('geometry',o.thesis/'thesis/review/2026-10-17/near-hko-transfer-integration/owner-verify.py')
    predictions=json.loads((d/'predictions.json').read_text());result=json.loads((d/'results.json').read_text());claimed=json.loads((d/'mechanism.json').read_text());rows=[];totalbad=totalfullbad=0;specific=None
    for s,t,c in zip(predictions['rows'],result['rows'],claimed['rows']):
        assert s['name']==t['name']==c['name'];a,b=[list(map(F,u)) for u in s['A']],[list(map(F,u)) for u in s['B']];ca,cb=g.closures(a),g.closures(b);truth=F(t['Q_true']);seed=s['extraction']['seeds'];bad=fullbad=ownbad=0
        wordmax={v['shadow_sign']:max(g.direct(a,b,v['word'],x,y) for x in ca for y in cb) for v in s['variants']}
        for z in seed:
            x,y=list(map(F,z['terminal_alpha'])),list(map(F,z['terminal_gamma']));q=F(z['Q']);w=z['word']
            ma=q-max(g.direct(a,b,w,u,y) for u in ca if u!=x);mb=q-max(g.direct(a,b,w,x,u) for u in cb if u!=y);assert ma>=0 and mb>=0
            if q<truth and ma>0 and mb>0:
                bad+=1;pairbest=g.prefix_value(a,b,x,y)
                if q<wordmax[z['shadow_sign']]:ownbad+=1
                if pairbest<truth:fullbad+=1
                if specific is None and z['shadow_sign']==1 and pairbest==q and q<wordmax[1]:
                    ux,uy=max(((u,v) for u in ca for v in cb),key=lambda uv:g.direct(a,b,w,*uv));better=g.direct(a,b,w,ux,uy);assert better>q
                    specific=dict(name=s['name'],shadow_sign=1,kind=z['kind'],index=z['index'],A_support=[i for i,v in enumerate(x) if v],B_support=[i for i,v in enumerate(y) if v],Q=q,Q_true=truth,pair_best_all_orders=pairbest,A_margin=ma,B_margin=mb,same_word_competitor_Q=better,same_word_competitor_A=ux,same_word_competitor_B=uy,capacity_over_global=truth/q)
        assert bad==c['strict_nonglobal_seed_count'] and fullbad==c['strict_nonglobal_support_seed_count'];totalbad+=bad;totalfullbad+=fullbad
        one=max(F(z['states'][0]['Q']) for z in seed);completed=max(F(z['states'][1]['Q']) for z in seed);assert completed==truth
        mix=c['repair']
        if one<truth:
            assert mix is not None;best=s['extraction']['best'];x=list(map(F,best['states'][0]['alpha']));fixed=list(map(F,best['states'][0]['gamma']));good=list(map(F,best['terminal_gamma']));assert x==list(map(F,best['terminal_alpha']));lam=F(mix['mixture']['good_weight']);assert 0<lam<1
            badv=next(v for v in cb if fixed==[lam*u+(1-lam)*z for u,z in zip(good,v)])
            qb=g.direct(a,b,best['word'],x,badv);assert truth-one==(1-lam)*(truth-qb)==F(mix['mixture']['loss_identity'])
        else:assert mix is None
        rows.append(dict(name=s['name'],one_response_exact=one==truth,completion_exact=completed==truth,strict_coordinate_below_global=bad,strict_coordinate_below_own_word_max=ownbad,strict_coordinate_below_global_after_best_order=fullbad))
    assert specific is not None and totalbad==185 and totalfullbad==159
    out=dict(status='PASS',rows=rows,strict_coordinate_endpoints_below_global=totalbad,strict_coordinate_endpoints_below_global_after_best_order=totalfullbad,specific_trap=specific,one_response_misses=sum(not r['one_response_exact'] for r in rows),completion_misses=0,scope='No new predictions: independently checks post-score seed/mixture explanation, same-word strict competitor and best-order support gap. 600 direct per-word pair objectives plus coordinate-margin checks.')
    o.out.write_text(json.dumps(m.serial(out),indent=2)+'\n');print(json.dumps({k:v for k,v in m.serial(out).items() if k not in ['rows','specific_trap']},indent=2));print(json.dumps(m.serial(specific),indent=2))
if __name__=='__main__':main()
