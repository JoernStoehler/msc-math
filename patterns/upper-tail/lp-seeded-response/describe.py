#!/usr/bin/env python3
"""Explain frozen outcomes only; no new body selection or optimization policy."""
import argparse,importlib.util,json
from fractions import Fraction as F
from pathlib import Path

def serial(v):
 if isinstance(v,F):return str(v)
 if isinstance(v,dict):return {k:serial(x) for k,x in v.items()}
 if isinstance(v,(tuple,list)):return [serial(x) for x in v]
 return v

def sup(v):return [i for i,x in enumerate(v) if x]
def main():
 p=argparse.ArgumentParser();p.add_argument('--repo',type=Path,required=True);p.add_argument('--directory',type=Path,required=True);o=p.parse_args();r=o.repo;d=o.directory
 sp=importlib.util.spec_from_file_location('independent',r/'patterns/upper-tail/near-hko-transfer/verify.py');m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
 pred=json.loads((d/'predictions.json').read_text());truth=json.loads((d/'results.json').read_text());rows=[];strict_trap=None
 for s,t in zip(pred['rows'],truth['rows']):
  a=[list(map(F,v)) for v in s['A']];b=[list(map(F,v)) for v in s['B']];ca=m.vertices(a);cb=m.vertices(b);qt=F(t['Q_true']);seeds=s['extraction']['seeds'];winner_seeds=[z for z in seeds if F(z['Q'])==qt];strict_bad=[];strict_support_bad=[]
  for seed in seeds:
   alpha=list(map(F,seed['terminal_alpha']));gamma=list(map(F,seed['terminal_gamma']));q=F(seed['Q']);va=[m.q(a,b,seed['word'],u,gamma) for u in ca if u!=alpha];vb=[m.q(a,b,seed['word'],alpha,u) for u in cb if u!=gamma];margA=q-max(va);margB=q-max(vb)
   assert margA>=0 and margB>=0
   if q<qt and margA>0 and margB>0:
    strict_bad.append(seed)
    pair_values=[m.q(a,b,v['word'],alpha,gamma) for v in s['variants']];pair_best=max(pair_values);same_word_best=max(F(v['Q']) for v in t['oracle_candidates'] if v['shadow_sign']==seed['shadow_sign']);same_word_winner=next(v for v in t['oracle_candidates'] if v['shadow_sign']==seed['shadow_sign'] and F(v['Q'])==same_word_best)
    if pair_best<qt:strict_support_bad.append(seed)
    if strict_trap is None and pair_best==q and q<same_word_best:
     strict_trap=dict(name=s['name'],sys=t['ratio'],seed={k:seed[k] for k in ['shadow_sign','kind','index']},A_support=sup(alpha),B_support=sup(gamma),alpha=alpha,gamma=gamma,Q=q,Q_true=qt,global_Q_gap=qt-q,strict_A_response_margin=margA,strict_B_response_margin=margB,shadow_pair_values=pair_values,full_word_maximum_for_this_vertex_pair=pair_best,same_fixed_word_best_Q=same_word_best,same_fixed_word_improvement=same_word_best-q,same_fixed_word_competitor=same_word_winner,winning_seed={k:winner_seeds[0][k] for k in ['shadow_sign','kind','index']},winning_A_support=sup(list(map(F,winner_seeds[0]['terminal_alpha']))),winning_B_support=sup(list(map(F,winner_seeds[0]['terminal_gamma']))))
  qold=max(F(z['states'][0]['Q']) for z in seeds);qcomplete=max(F(z['states'][1]['Q']) for z in seeds);details=None
  if qold<qt:
   best=s['extraction']['best'];assert best['kind']=='row';first=best['states'][0];assert F(first['Q'])==qold;alpha=list(map(F,first['alpha']));gamma=list(map(F,first['gamma']));good=list(map(F,best['terminal_gamma']));assert alpha==list(map(F,best['terminal_alpha']));mix=None
   for bad in cb:
    if bad==good:continue
    j=next((i for i in range(5) if good[i]!=bad[i]),None);lam=(gamma[j]-bad[j])/(good[j]-bad[j])
    if 0<lam<1 and gamma==[lam*x+(1-lam)*y for x,y in zip(good,bad)]:
     qb=m.q(a,b,best['word'],alpha,bad);assert F(first['Q'])==lam*qt+(1-lam)*qb;mix=dict(good_B_support=sup(good),bad_B_support=sup(bad),good_weight=lam,bad_weight=1-lam,Q_bad=qb,initial_Q=F(first['Q']),completed_Q=qt,loss_identity=(1-lam)*(qt-qb));break
   assert mix is not None
   coverage={}
   for v in s['variants']:
    W=list(map(F,v['W']));ac=[];bc=[]
    for j in range(5):
     vals=[W[i*5+j] for i in range(5)];mass=sum(vals)
     if mass:ac.append([x/mass for x in vals])
    for i in range(5):
     vals=W[i*5:i*5+5];mass=sum(vals)
     if mass:bc.append([x/mass for x in vals])
    coverage[str(v['shadow_sign'])]=dict(winning_A_seen=alpha in ac,winning_B_seen=good in bc,pure_A_vertex_indices=sorted({i for i,u in enumerate(ca) if u in ac}),pure_B_vertex_indices=sorted({i for i,u in enumerate(cb) if u in bc}))
   details=dict(chosen_seed={k:best[k] for k in ['shadow_sign','kind','index']},winning_A_support=sup(alpha),winning_B_support=sup(good),conditional_coverage=coverage,mixture=mix,old_relative_capacity_error=qt/qold-1)
  rows.append(dict(name=s['name'],above_one=F(t['ratio'])>1,ratio=t['ratio'],candidate_count=len(seeds),one_response_exact=qold==qt,completion_exact=qcomplete==qt,converged_exact=F(s['extraction']['best']['Q'])==qt,winning_seed_count=len(winner_seeds),nonglobal_seed_count=len(seeds)-len(winner_seeds),strict_nonglobal_seed_count=len(strict_bad),strict_nonglobal_support_seed_count=len(strict_support_bad),extra_strict_updates_needed_for_selected_global_value=qcomplete<qt,repair=details))
 out=dict(rows=rows,first_strict_nonglobal_terminal=strict_trap,summary=dict(all_fresh_exact=sum(x['converged_exact'] for x in rows),one_response_misses=sum(not x['one_response_exact'] for x in rows),completion_misses=sum(not x['completion_exact'] for x in rows),strict_nonglobal_seed_total=sum(x['strict_nonglobal_seed_count'] for x in rows),strict_nonglobal_support_seed_total=sum(x['strict_nonglobal_support_seed_count'] for x in rows),all_seed_total=sum(x['candidate_count'] for x in rows)))
 (d/'mechanism.json').write_text(json.dumps(serial(out),indent=2)+'\n');print(json.dumps(serial(out['summary']),indent=2));print(json.dumps(serial(strict_trap),indent=2))
if __name__=='__main__':main()
