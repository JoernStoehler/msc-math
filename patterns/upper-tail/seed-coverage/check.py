#!/usr/bin/env python3
"""Exact audit of existing LP seeds; no producer/optimizer imports or new geometry."""
import argparse,hashlib,importlib.util,json,resource,time
from fractions import Fraction as F
from pathlib import Path

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def ser(x):
 if isinstance(x,F):return str(x)
 if isinstance(x,dict):return {k:ser(v) for k,v in x.items()}
 if isinstance(x,(list,tuple)):return [ser(v) for v in x]
 return x

def main():
 ap=argparse.ArgumentParser();ap.add_argument('--repo',type=Path,required=True);ap.add_argument('--directory',type=Path,required=True);ap.add_argument('--out',type=Path,required=True);o=ap.parse_args();ct=time.process_time();t=time.perf_counter()
 src=json.loads((o.directory/'sources.json').read_text())
 for p,h in src.items():assert sha(o.repo/p)==h
 sp=importlib.util.spec_from_file_location('independent',o.repo/'patterns/upper-tail/near-hko-transfer/verify.py');m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
 pred=json.loads((o.repo/'patterns/upper-tail/lp-seeded-response/predictions.json').read_text());truth=json.loads((o.repo/'patterns/upper-tail/lp-seeded-response/results.json').read_text());reports=[];words_covered=0;bodies_covered=0;witness=None;allseeds=0
 for source,result in zip(pred['rows'],truth['rows']):
  assert source['name']==result['name'];a=[[F(x) for x in u] for u in source['A']];b=[[F(x) for x in u] for u in source['B']];ca=m.vertices(a);cb=m.vertices(b);assert len(ca)==len(cb)==5;wordrows=[]
  for v in source['variants']:
   W=m.lp(a,b,v);w=v['word'];tab=[[m.q(a,b,w,x,y) for y in cb] for x in ca]
   for z in result['oracle_candidates']:
    if z['shadow_sign']==v['shadow_sign']:assert tab[z['A_vertex']][z['B_vertex']]==F(z['Q'])
   firstA=set();firstB=set();ends=set();paths=[]
   seeds=[z for z in source['extraction']['seeds'] if z['shadow_sign']==v['shadow_sign']]
   for z in seeds:
    k=z['index'];isrow=z['kind']=='row';order=['A','B'] if isrow else ['B','A'];vals=W[5*k:5*k+5] if isrow else [W[5*i+k] for i in range(5)];mass=sum(vals);assert mass>0;fixed=[x/mass for x in vals];alpha=None if isrow else fixed;gamma=fixed if isrow else None;states=[]
    def response(which):
     vs=ca if which=='A' else cb;qs=[m.q(a,b,w,x,gamma) if which=='A' else m.q(a,b,w,alpha,x) for x in vs];return vs[qs.index(max(qs))]
    def record(stage,which):states.append(dict(stage=stage,factor=which,alpha=alpha[:],gamma=gamma[:],Q=m.q(a,b,w,alpha,gamma)))
    for which in order:
     best=response(which)
     if which=='A':alpha=best
     else:gamma=best
     record('initialization',which)
    if isrow:firstA.add(ca.index(states[0]['alpha']))
    else:firstB.add(cb.index(states[0]['gamma']))
    seen={(ca.index(alpha),cb.index(gamma))}
    while True:
     changed=False
     for which in order:
      best=response(which);q=m.q(a,b,w,alpha,gamma);better=m.q(a,b,w,best,gamma) if which=='A' else m.q(a,b,w,alpha,best)
      if better>q:
       if which=='A':alpha=best
       else:gamma=best
       record('strict_update',which);pair=(ca.index(alpha),cb.index(gamma));assert pair not in seen;seen.add(pair);changed=True
     if not changed:break
    assert m.canon(z['states'])==states;assert m.canon(z['terminal_alpha'])==alpha and m.canon(z['terminal_gamma'])==gamma
    pair=(ca.index(alpha),cb.index(gamma));ends.add(pair);paths.append(dict(kind=z['kind'],index=k,visited_vertex_pairs=sorted(seen),terminal_pair=pair));allseeds+=1
   maxima=[(i,j) for i in range(5) for j in range(5) if tab[i][j]>=max(tab[i]) and tab[i][j]>=max(tab[k][j] for k in range(5))];missing=sorted(set(maxima)-ends);covered=len(firstA)==5 or len(firstB)==5;words_covered+=covered
   def support(x):return [i for i,z in enumerate(x) if z]
   wordrows.append(dict(sign=v['shadow_sign'],first_A_vertices=sorted(firstA),first_B_vertices=sorted(firstB),missing_A_supports=[support(ca[i]) for i in range(5) if i not in firstA],missing_B_supports=[support(cb[i]) for i in range(5) if i not in firstB],full_one_factor_coverage=covered,strict_or_weak_coordinate_maxima=maxima,terminal_pairs=sorted(ends),unreached_coordinate_maxima=missing))
   if source['name']=='lpseed_near_hko_s0_e100' and v['shadow_sign']==-1:
    assert missing==[(2,0)];i,j=missing[0];q=tab[i][j];agap=min(q-tab[k][j] for k in range(5) if k!=i);bgap=min(q-tab[i][k] for k in range(5) if k!=j);assert agap>0 and bgap>0 and q<F(result['Q_true'])
    assert all([i,j] not in [list(x) for x in p['visited_vertex_pairs']] for p in paths)
    positive=next(x for x in source['variants'] if x['shadow_sign']==1);samepositive=m.q(a,b,positive['word'],ca[i],cb[j]);assert samepositive<q
    assert any(m.canon(z['terminal_alpha'])==ca[i] and m.canon(z['terminal_gamma'])==cb[j] for z in source['extraction']['seeds'] if z['shadow_sign']==1)
    witness=dict(name=source['name'],shadow_sign=-1,A_vertex=i,B_vertex=j,A_support=support(ca[i]),B_support=support(cb[j]),alpha=ca[i],gamma=cb[j],word=w,Q=q,A_response_gap=agap,B_response_gap=bgap,global_objective_gap=F(result['Q_true'])-q,positive_word_same_pair_Q=samepositive,negative_minus_positive=q-samepositive,all_ten_negative_seed_paths=paths,visited_by_no_negative_seed=True,reached_in_positive_word=True,ratio=F(result['ratio']))
  both=all(x['full_one_factor_coverage'] for x in wordrows);bodies_covered+=both;reports.append(dict(name=source['name'],both_words_full_coverage=both,words=wordrows))
 assert len(reports)==12 and words_covered==8 and bodies_covered==2 and allseeds==240 and witness
 out=dict(status='PASS',full_factor_certificate_words=words_covered,total_words=24,full_factor_certificate_bodies=bodies_covered,total_bodies=12,existing_LP_certificates_checked=24,existing_direct_HK_pair_values_checked=600,seed_paths_checked=allseeds,rows=reports,witness=witness,cpu_seconds=time.process_time()-ct,elapsed_seconds=time.perf_counter()-t,input_hashes=src,check_sha256=sha(Path(__file__)))
 o.out.parent.mkdir(parents=True,exist_ok=True);o.out.write_text(json.dumps(ser(out),indent=2)+'\n');print('PASS: full-factor certificate 8/24 words, 2/12 bodies; exact unseeded strict negative-word basin; CPU',out['cpu_seconds'])
if __name__=='__main__':main()
