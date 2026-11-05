#!/usr/bin/env python3
"""Exact local-success certificate. No optimizer, producer, or numerical library.

Reads a single frozen fixture and the previous independent replay arithmetic.
All pair scores are diagnostic proof obligations, never predictor input.
"""
import argparse, hashlib, importlib.util, json, resource, time
from fractions import Fraction as F
from pathlib import Path

def module(name,path):
 s=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def serial(x):
 if hasattr(x,'serial'):return x.serial()
 if isinstance(x,F):return str(x)
 if isinstance(x,dict):return {k:serial(v) for k,v in x.items()}
 if isinstance(x,(list,tuple)):return [serial(v) for v in x]
 return x

def support(x):return [i for i,v in enumerate(x) if v]

def block_signature(word,active,n=5):
 """Cyclic same-factor runs, normalized over cyclic rotations of those runs."""
 w=[i for i in word if i in active];assert set(w)==active
 runs=[]
 for i in w:
  if runs and (runs[-1][0]<n)==(i<n):runs[-1].append(i)
  else:runs.append([i])
 if len(runs)>1 and (runs[0][0]<n)==(runs[-1][0]<n):runs=[runs[-1]+runs[0]]+runs[1:-1]
 t=[tuple(sorted(x)) for x in runs]
 return min(tuple(t[i:]+t[:i]) for i in range(len(t)))

def main():
 p=argparse.ArgumentParser();p.add_argument('--repo',required=True,type=Path);p.add_argument('--directory',required=True,type=Path);p.add_argument('--out',required=True,type=Path);o=p.parse_args();r=o.repo;d=o.directory
 started=time.process_time();fixture=json.loads((d/'fixture.json').read_text())
 for source,digest in fixture['sources'].items():assert sha(r/source)==digest,(source,'hash changed')
 source_rational=next(z for z in json.loads((r/'patterns/upper-tail/near-hko-transfer/results.json').read_text())['rows'] if z['name']=='near_hko_s4_e100');assert fixture['rational']==source_rational
 source_diagnostic=next(z for z in json.loads((r/'patterns/upper-tail/lp-seeded-response/diagnostics.json').read_text())['rows'] if z['name']=='near_hko_s4_e100');assert fixture['completed_seed']==next(z for z in source_diagnostic['extraction']['seeds'] if (z['shadow_sign'],z['kind'],z['index'])==(1,'row',0))
 assert fixture['hko']==json.loads((r/'patterns/upper-tail/conditional-coupling/hko-exact.json').read_text())
 v=module('local_independent_rational',r/'patterns/upper-tail/near-hko-transfer/verify.py')
 s=fixture['rational'];a=[list(map(F,x)) for x in s['A']];b=[list(map(F,x)) for x in s['B']];ca=v.vertices(a);cb=v.vertices(b);assert len(ca)==len(cb)==5
 for variant in s['variants']:v.lp(a,b,variant)
 stable=v.stability(a,b,s['variants']);volume=v.area(a)*v.area(b)
 qr=v.rounding(a,b,s['variants'],s['rounding'],ca,cb)
 # Check the frozen existing oracle table entry-by-entry, recomputing direct 4D HK.
 table=[]
 for variant in s['variants']:
  for i,x in enumerate(ca):
   for j,y in enumerate(cb):table.append(dict(shadow_sign=variant['shadow_sign'],A_vertex=i,B_vertex=j,Q=v.q(a,b,variant['word'],x,y)))
 old=s['truth']['oracle_candidates'];assert len(old)==50
 for row,retained in zip(table,old):assert all(row[k]==F(retained[k]) if k=='Q' else row[k]==retained[k] for k in row)
 qt=max(x['Q'] for x in table);winners=[x for x in table if x['Q']==qt];pairs={(x['A_vertex'],x['B_vertex']) for x in winners};assert len(winners)==2 and len(pairs)==1
 ia,ib=next(iter(pairs));alpha,gamma=ca[ia],cb[ib];assert support(alpha)==[0,2,3] and support(gamma)==[1,2,4]
 global_gap=min(qt-x['Q'] for x in table if (x['A_vertex'],x['B_vertex'])!=(ia,ib));assert global_gap>0
 active=set(support(alpha)+[5+i for i in support(gamma)]);signatures=[block_signature(z['word'],active) for z in s['variants']];assert signatures[0]==signatures[1]
 # This exact block identity, not numerical equality at one point, keeps both winning word values equal nearby.
 selected=s['variants'][0];assert selected['shadow_sign']==1
 W=list(map(F,selected['W']));mass=sum(W[:5]);assert mass>0;condition=[x/mass for x in W[:5]];v.feasible(b,condition)
 first=[v.q(a,b,selected['word'],x,condition) for x in ca];assert first.index(max(first))==ia and sum(x==max(first) for x in first)==1
 first_gap=min(first[ia]-x for i,x in enumerate(first) if i!=ia);assert first_gap>0
 second=[v.q(a,b,selected['word'],alpha,y) for y in cb];assert second.index(max(second))==ib and sum(x==max(second) for x in second)==1
 second_gap=min(second[ib]-x for j,x in enumerate(second) if j!=ib);assert second_gap>0 and second[ib]==qt
 ratio=1/(8*qt*qt*volume);assert ratio>1 and qr<qt
 # The exact conditional is a strict mixture of the winning B and one losing B vertex.
 bad=cb[4];assert W[0]==0 and 0 not in selected['basis'];assert [x for x in cb if x[0]==0]==[gamma,bad];lam=condition[2]/gamma[2];assert 0<lam<1 and condition==[lam*x+(1-lam)*y for x,y in zip(gamma,bad)]
 qbad=v.q(a,b,selected['word'],alpha,bad);assert qt-qbad>0 and first[ia]==lam*qt+(1-lam)*qbad
 assert qr==first[ia] and qt-qr==(1-lam)*(qt-qbad)
 # Independently verify the actual completed seed states from the retained response packet.
 seed=fixture['completed_seed'];assert (seed['shadow_sign'],seed['kind'],seed['index'])==(1,'row',0)
 for index,(x,y,q) in enumerate([(alpha,condition,first[ia]),(alpha,gamma,qt)]):
  z=seed['states'][index];assert list(map(F,z['alpha']))==x and list(map(F,z['gamma']))==y and F(z['Q'])==q
 assert F(seed['Q'])==qt
 # Secondary HKO lemma: every pure factor closure has a globally optimal partner in each word.
 q5=module('local_Q5',r/'patterns/upper-tail/conditional-coupling/quadratic.py').Q5
 h=fixture['hko'];ha=[list(map(q5.parse,x)) for x in h['A']];hb=[list(map(q5.parse,x)) for x in h['B']]
 hv=module('local_independent_Q5',r/'patterns/upper-tail/conditional-rounding/verify.py');hca=hv.vertices(ha);hcb=hv.vertices(hb);assert len(hca)==len(hcb)==5
 matrices=[];gaps=[]
 for variant in h['variants']:
  hv.lp_check(ha,hb,variant,q5.parse)
  M=[[hv.actual_Q(ha,hb,variant['word'],x,y) for y in hcb] for x in hca];q0=q5(1)/10
  assert all(max(row)==q0 and sum(x==q0 for x in row)==2 for row in M)
  assert all(max(M[i][j] for i in range(5))==q0 and sum(M[i][j]==q0 for i in range(5))==2 for j in range(5))
  gaps.append(min(q0-x for row in M for x in row if x<q0));matrices.append(dict(sign=variant['shadow_sign'],matrix=M,winning_pairs=[(i,j) for i in range(5) for j in range(5) if M[i][j]==q0]))
 out=dict(status='PASS',rational_control=s['name'],diagnostic_not_new_prediction=True,new_geometries=0,LP_optimizations=0,strict_stability=stable,winning_pair=dict(A_vertex=ia,B_vertex=ib,A_support=support(alpha),B_support=support(gamma)),winning_cyclic_block_signature=signatures[0],two_winning_words_identically_equal_in_word_chamber=True,selected_seed=dict(sign=1,kind='row',index=0,condition_mass=mass,first_response_gap=first_gap,completion_response_gap=second_gap),global_pair_gap_both_words=global_gap,Q_true=qt,Q_old_one_response=qr,Q_completed=qt,old_Q_loss=qt-qr,mixture=dict(face_zero_coordinate=0,face_has_exactly_two_endpoints=True,zero_coordinate_nonbasic_in_selected_LP=True,good_weight=lam,bad_weight=1-lam,Q_bad=qbad,identity_rhs=(1-lam)*(qt-qbad)),systolic_ratio=ratio,ratio_margin=ratio-1,capacity_true=1/(2*qt),capacity_old=1/(2*qr),HKO=dict(matrices=matrices,strict_gaps_to_nonpartners=gaps,any_feasible_conditional_completes=True,full_neighborhood_exactness_proved=False),cpu_seconds=time.process_time()-started,hashes={k:sha(d/k) for k in ['PLAN.md','fixture.json','check.py']})
 o.out.write_text(json.dumps(serial(out),indent=2)+'\n');print('PASS: open exact-repair certificate; HKO all-feasible completion lemma; CPU',out['cpu_seconds'])
if __name__=='__main__':main()
