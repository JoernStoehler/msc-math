#!/usr/bin/env python3
"""Standard-library audit using preexisting independent rational geometry/HK code.
No producer, numerical dependency or optimizer is imported.
"""
import argparse,hashlib,importlib.util,json
from fractions import Fraction as F
from pathlib import Path

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def serial(v):
 if isinstance(v,F):return str(v)
 if isinstance(v,dict):return {k:serial(x) for k,x in v.items()}
 if isinstance(v,list):return [serial(x) for x in v]
 return v

def main():
 p=argparse.ArgumentParser();p.add_argument('mode',choices=['geometry','all']);p.add_argument('--repo',type=Path,required=True);p.add_argument('--directory',type=Path,required=True);o=p.parse_args();r=o.repo;d=o.directory
 sp=importlib.util.spec_from_file_location('independent_rational',r/'patterns/upper-tail/near-hko-transfer/verify.py');m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m)
 frozen=json.loads((d/'frozen-manifest.json').read_text())
 for k,v in frozen['sources'].items():assert sha(r/k)==v
 for k,v in frozen['freeze'].items():assert sha(d/k)==v
 sample=json.loads((d/'sample.json').read_text());h=json.loads((r/'patterns/upper-tail/conditional-coupling/hko-exact.json').read_text());base=[F(v['rational'])+F(v['sqrt5'])*F(2236068,1000000) for k in ['A','B'] for u in h[k] for v in u];geometry=[]
 assert len(sample['rows'])==12
 for i,s in enumerate(sample['rows']):
  seed=i//2;eps=[F(1,100),F(1,20)][i%2];raw=hashlib.sha256(('msc-math/thesis/lp-seeded-response/validation/v1:'+str(seed)).encode()).digest();assert s['seed']==seed and F(s['epsilon'])==eps and s['direction_digest']==raw.hex();assert [F(x) for k in ['A','B'] for u in s[k] for x in u]==[x+eps*F(byte%9-4,4) for x,byte in zip(base,raw)]
  a=[list(map(F,v)) for v in s['A']];b=[list(map(F,v)) for v in s['B']];ca=m.vertices(a);cb=m.vertices(b);assert len(ca)==len(cb)==5;volume=m.area(a)*m.area(b);geometry.append(dict(name=s['name'],closure_vertex_counts=[len(ca),len(cb)],genuine_bounded_five_facet_factors=True,volume=volume))
 if o.mode=='geometry':
  out=dict(status='PASS',sample_sha256=sha(d/'sample.json'),checks=geometry,verify_sha256=sha(Path(__file__)));(d/'geometry.json').write_text(json.dumps(serial(out),indent=2)+'\n');print('GEOMETRY PASS12');return
 def check_prediction(s):
  assert s['status']=='OK';a=[list(map(F,v)) for v in s['A']];b=[list(map(F,v)) for v in s['B']];ca=m.vertices(a);cb=m.vertices(b);m.area(a);m.area(b);seeds=s['extraction']['seeds'];at=0
  for v in s['variants']:
   W=m.lp(a,b,v)
   for kind in ['row','column']:
    order=['A','B'] if kind=='row' else ['B','A']
    for k in range(5):
     vals=W[k*5:k*5+5] if kind=='row' else [W[i*5+k] for i in range(5)];mass=sum(vals)
     if not mass:continue
     seed=seeds[at];at+=1;assert (seed['shadow_sign'],seed['kind'],seed['index'])==(v['shadow_sign'],kind,k);assert F(seed['condition_mass'])==mass and seed['word']==v['word'];fixed=[x/mass for x in vals];alpha=None if kind=='row' else fixed;gamma=fixed if kind=='row' else None;expected=[]
     def best_response(which):
      cv=ca if which=='A' else cb;values=[m.q(a,b,v['word'],u,gamma) if which=='A' else m.q(a,b,v['word'],alpha,u) for u in cv];return cv[values.index(max(values))],max(values)
     for which in order:
      best,value=best_response(which)
      if which=='A':alpha=best
      else:gamma=best
      expected.append(dict(stage='initialization',factor=which,alpha=alpha[:],gamma=gamma[:],Q=value))
     visited={(tuple(alpha),tuple(gamma))};sweeps=0
     while True:
      sweeps+=1;changed=False
      for which in order:
       best,value=best_response(which);current=m.q(a,b,v['word'],alpha,gamma)
       if value>current:
        if which=='A':alpha=best
        else:gamma=best
        key=(tuple(alpha),tuple(gamma));assert key not in visited;visited.add(key);changed=True;expected.append(dict(stage='strict_update',factor=which,alpha=alpha[:],gamma=gamma[:],Q=value))
      if not changed:break
     assert len(visited)<=len(ca)*len(cb);assert m.canon(seed['states'])==m.canon(expected);assert m.canon(seed['terminal_alpha'])==alpha and m.canon(seed['terminal_gamma'])==gamma;value=m.q(a,b,v['word'],alpha,gamma);assert F(seed['Q'])==value and F(seed['capacity'])==1/(2*value) and seed['sweeps']==sweeps
  assert at==len(seeds)==s['extraction']['candidate_count'];best=max(seeds,key=lambda v:F(v['Q']));assert s['extraction']['best']==best and s['extraction']['capacity']==best['capacity'];return dict(name=s['name'],LP_certificates=len(s['variants']),seeds=len(seeds),max_strict_updates=max(len(z['states'])-2 for z in seeds),max_sweeps=max(z['sweeps'] for z in seeds),terminal_pairs_by_shadow={str(sign):sorted({(tuple(map(str,z['terminal_alpha'])),tuple(map(str,z['terminal_gamma']))) for z in seeds if z['shadow_sign']==sign}) for sign in [1,-1]})
 diagnostics=json.loads((d/'diagnostics.json').read_text());diagnostic_checks=[check_prediction(s) for s in diagnostics['rows']];pred=json.loads((d/'predictions.json').read_text());assert len(pred['rows'])==12;checks=[]
 for i,s in enumerate(pred['rows']):
  assert all(s[k]==sample['rows'][i][k] for k in ['name','A','B']);checks.append(check_prediction(s))
 out=json.loads((d/'results.json').read_text());assert sha(d/'predictions.json')==out['predictions_sha256'];scores=[]
 for i,row in enumerate(out['rows']):
  s=pred['rows'][i];assert row['name']==s['name'];a=[list(map(F,v)) for v in s['A']];b=[list(map(F,v)) for v in s['B']];ca=m.vertices(a);cb=m.vertices(b);table=[]
  for v in s['variants']:
   for ia,alpha in enumerate(ca):
    for ib,gamma in enumerate(cb):table.append(dict(shadow_sign=v['shadow_sign'],A_vertex=ia,B_vertex=ib,alpha=alpha,gamma=gamma,word=v['word'],Q=m.q(a,b,v['word'],alpha,gamma)))
  assert m.canon(table)==m.canon(row['oracle_candidates']);qt=max(v['Q'] for v in table);qp=F(s['extraction']['best']['Q']);assert qp<=qt;volume=m.area(a)*m.area(b);assert F(row['Q_predicted'])==qp and F(row['Q_true'])==qt and F(row['capacity_true'])==1/(2*qt) and F(row['volume'])==volume and F(row['ratio'])==1/(8*volume*qt*qt);assert row['exact_recovery']==(qp==qt) and F(row['relative_capacity_error'])==qt/qp-1;assert m.canon(row['winners'])==m.canon([v for v in table if v['Q']==qt]);scores.append(dict(name=s['name'],exact_recovery=qp==qt,above_one=F(row['ratio'])>1,ratio=float(F(row['ratio'])),capacity_error=float(qt/qp-1)))
  if qp<qt:assert i==len(out['rows'])-1
 assert out['scored_count']==len(scores);assert out['unscored_names']==[s['name'] for s in pred['rows'][len(scores):]]
 result=dict(status='PASS',geometry=geometry,diagnostics=diagnostic_checks,predictions=checks,scores=scores,LP_certificates_checked=2*(len(checks)+len(diagnostic_checks)),fresh_oracle_objectives_checked=50*len(scores),hashes={k:sha(d/k) for k in ['PLAN.md','freeze.py','sample.json','analyze.py','frozen-manifest.json','diagnostics.json','predictions.json','results.json','verify.py']})
 (d/'verification.json').write_text(json.dumps(serial(result),indent=2)+'\n');print('PASS',len(scores),'scored',sum(s['exact_recovery'] for s in scores),'exact recoveries')
if __name__=='__main__':main()
