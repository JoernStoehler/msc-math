#!/usr/bin/env python3
"""Independent Gaussian closures, direct 4D payoffs and exact response graph audit."""
import argparse, hashlib, importlib.util, json, time
from fractions import Fraction as F
from pathlib import Path

def digest(p): return hashlib.sha256(p.read_bytes()).hexdigest()
def support(x): return [i for i,a in enumerate(x) if a]
def main():
 p=argparse.ArgumentParser();p.add_argument('--thesis',type=Path,required=True);p.add_argument('--patterns',type=Path,required=True);p.add_argument('--out',type=Path,required=True);o=p.parse_args();t=time.process_time()
 source=o.thesis/'thesis/review/2026-10-17/near-hko-transfer-integration/owner-verify.py'
 spec=importlib.util.spec_from_file_location('independent_geometry',source);g=importlib.util.module_from_spec(spec);spec.loader.exec_module(g)
 pp=o.patterns/'patterns/upper-tail/lp-seeded-response/predictions.json';rp=pp.with_name('results.json');pred=json.loads(pp.read_text());truth={r['name']:r for r in json.loads(rp.read_text())['rows']}
 rows=[];counter={'LPs':0,'direct_pair_word_payoffs':0,'seed_paths':0,'full_factor_covered_words':0,'bodies_both_words_covered':0};special=None
 for r in pred['rows']:
  a,b=[[list(map(F,u)) for u in r[k]] for k in ['A','B']];av,bv=g.closures(a),g.closures(b);info=[];body_word_max=[]
  for v in r['variants']:
   g.audit_lp(a,b,v);counter['LPs']+=1;w=v['word'];mat=[[g.direct(a,b,w,x,y) for y in bv] for x in av];counter['direct_pair_word_payoffs']+=len(av)*len(bv)
   body_word_max.append(max(q for row in mat for q in row));W=list(map(F,v['W']));reached=set();firstA=set();firstB=set();paths=[]
   for kind in ['row','column']:
    for idx in range(len(a) if kind=='row' else len(b)):
     weights=[W[idx*len(b)+j] for j in range(len(b))] if kind=='row' else [W[i*len(b)+idx] for i in range(len(a))]
     mass=sum(weights)
     if not mass:continue
     fixed=[x/mass for x in weights]
     if kind=='row':
      ia=max(range(len(av)),key=lambda i:g.direct(a,b,w,av[i],fixed));firstA.add(ia);ib=max(range(len(bv)),key=lambda j:mat[ia][j])
     else:
      ib=max(range(len(bv)),key=lambda j:g.direct(a,b,w,fixed,bv[j]));firstB.add(ib);ia=max(range(len(av)),key=lambda i:mat[i][ib])
     seen=[(ia,ib)];moving=True
     while moving:
      moving=False
      for factor in (['A','B'] if kind=='row' else ['B','A']):
       if factor=='A':
        nxt=max(range(len(av)),key=lambda i:mat[i][ib])
        if mat[nxt][ib]>mat[ia][ib]:ia=nxt;moving=True;seen.append((ia,ib))
       else:
        nxt=max(range(len(bv)),key=lambda j:mat[ia][j])
        if mat[ia][nxt]>mat[ia][ib]:ib=nxt;moving=True;seen.append((ia,ib))
      assert len(seen)<=len(av)*len(bv)
     old=next(s for s in r['extraction']['seeds'] if s['shadow_sign']==v['shadow_sign'] and s['kind']==kind and s['index']==idx)
     assert av[ia]==list(map(F,old['terminal_alpha'])) and bv[ib]==list(map(F,old['terminal_gamma'])) and mat[ia][ib]==F(old['Q'])
     reached.add((ia,ib));paths.append({'kind':kind,'index':idx,'pairs':seen});counter['seed_paths']+=1
   full=len(firstA)==len(av) or len(firstB)==len(bv);counter['full_factor_covered_words']+=full
   strict=[];missing=[]
   for i in range(len(av)):
    for j in range(len(bv)):
     mgA=min(mat[i][j]-mat[k][j] for k in range(len(av)) if k!=i);mgB=min(mat[i][j]-mat[i][k] for k in range(len(bv)) if k!=j)
     if min(mgA,mgB)>0:
      rec={'A_support':support(av[i]),'B_support':support(bv[j]),'Q':str(mat[i][j]),'A_margin':str(mgA),'B_margin':str(mgB),'reached':(i,j) in reached}
      strict.append(rec)
      if not rec['reached']:
       missing.append(rec)
       if special is None:
        special={'name':r['name'],'shadow_sign':v['shadow_sign'],'ratio':truth[r['name']]['ratio'],'unreached':rec,'paths':paths,'global_Q':truth[r['name']]['Q_true'],'strict_local_is_nonglobal':mat[i][j]<F(truth[r['name']]['Q_true'])}
   info.append({'shadow_sign':v['shadow_sign'],'first_A_supports':[support(av[i]) for i in sorted(firstA)],'first_B_supports':[support(bv[j]) for j in sorted(firstB)],'full_factor_coverage':full,'strict_local_maxima':strict,'missing_strict_local_maxima':missing})
  body_Q=max(body_word_max);assert body_Q==F(truth[r['name']]['Q_true']);assert (1/(2*body_Q))**2/(2*g.area(a)*g.area(b))==F(truth[r['name']]['ratio']);counter['bodies_both_words_covered']+=all(v['full_factor_coverage'] for v in info);rows.append({'name':r['name'],'words':info})
 assert special is not None and F(special['ratio'])>1 and special['strict_local_is_nonglobal']
 same=[x for w in rows[0]['words'] if w['shadow_sign']==1 for x in w['strict_local_maxima'] if x['A_support']==special['unreached']['A_support'] and x['B_support']==special['unreached']['B_support']];assert len(same)==1 and same[0]['reached'];assert F(same[0]['Q'])<F(special['unreached']['Q']);special['same_supports_positive_word']=same[0]
 out={'status':'PASS','counts':counter,'first_unreached_strict_basin':special,'rows':rows,'cpu_seconds':time.process_time()-t,'source_sha256':{str(source.relative_to(o.thesis)):digest(source),str(pp.relative_to(o.patterns)):digest(pp),str(rp.relative_to(o.patterns)):digest(rp)},'scope':'Existing exposed controls only. Missing strict local basins do not falsify all-seed global exactness; no new geometry, capacity or population estimate.'}
 o.out.parent.mkdir(parents=True,exist_ok=True);o.out.write_text(json.dumps(out,indent=2)+'\n');print(json.dumps({'status':out['status'],'counts':counter,'first_unreached':{k:special[k] for k in ['name','shadow_sign','unreached']},'cpu_seconds':out['cpu_seconds']}))
if __name__=='__main__':main()
