#!/usr/bin/env python3
"""Independent nondegeneracy/strict-gap certificate for the retained fresh obstruction."""
import argparse,hashlib,importlib.util,json
from fractions import Fraction as F
from itertools import combinations
from pathlib import Path

def rank(rows):
 a=[list(r) for r in rows];i=0
 for j in range(len(a[0])):
  k=next((k for k in range(i,len(a)) if a[k][j]),None)
  if k is None:continue
  a[i],a[k]=a[k],a[i];q=a[i][j];a[i]=[v/q for v in a[i]]
  for k in range(i+1,len(a)):
   q=a[k][j]
   if q:a[k]=[x-q*y for x,y in zip(a[k],a[i])]
  i+=1
 return i

def main():
 p=argparse.ArgumentParser();p.add_argument('--directory',required=True,type=Path);p.add_argument('--out',required=True,type=Path);args=p.parse_args();d=args.directory
 spec=importlib.util.spec_from_file_location('independent',Path(__file__).with_name('owner-verify.py'));rv=importlib.util.module_from_spec(spec);spec.loader.exec_module(rv)
 r=json.loads((d/'results.json').read_text())['rows'][-1];assert not r['exact_recovery']
 a=[list(map(F,u)) for u in r['A']];b=[list(map(F,u)) for u in r['B']];n,m=len(a),len(b)
 rows=[[F(1)]*(n*m)]
 for i in range(n):
  for k in range(2):rows.append([b[j][k] if ii==i else F(0) for ii in range(n) for j in range(m)])
 for j in range(m):
  for k in range(2):rows.append([a[i][k] if jj==j else F(0) for i in range(n) for jj in range(m)])
 fullrank=rank(rows);assert fullrank==17
 certs=[]
 for v in r['variants']:
  W=list(map(F,v['W']));sl=list(map(F,v['dual_slack']));active=[i for i,x in enumerate(W) if x>0];inactive=[i for i,x in enumerate(W) if x==0]
  active_rank=rank([[row[i] for i in active] for row in rows]);assert len(active)==active_rank==fullrank
  assert all(sl[i]==0 for i in active) and min(sl[i] for i in inactive)>0
  # Thus all optimal mass is on active columns, whose independent weights are unique.
  minimum_cross=min(abs(rv.cross(u,w)) for u,w in combinations(a+[(v['shadow_sign']*-p[1],v['shadow_sign']*p[0]) for p in b],2))
  assert minimum_cross>0
  certs.append(dict(shadow_sign=v['shadow_sign'],positive_entries=len(active),constraint_rank=fullrank,active_column_rank=active_rank,minimum_positive_weight=str(min(W[i] for i in active)),minimum_inactive_slack=str(min(sl[i] for i in inactive)),minimum_absolute_angular_cross=str(minimum_cross),unique_nondegenerate_LP_optimum=True))
 geometry_chamber=[]
 for normals in [a,b]:
  pair_min=min(abs(rv.cross(u,v)) for u,v in combinations(normals,2))
  triple_min=min(abs(rv.cross((v[0]-u[0],v[1]-u[1]),(w[0]-u[0],w[1]-u[1]))) for u,v,w in combinations(normals,3))
  assert pair_min>0 and triple_min>0
  geometry_chamber.append(dict(minimum_pair_determinant=str(pair_min),minimum_affine_triple_determinant=str(triple_min)))
 ca,cb=rv.closures(a),rv.closures(b);truth=F(r['truth']['Q']);rounded=F(r['rounding']['best']['Q'])
 assert truth>rounded and F(r['ratio'])>1
 winner=[(x,y) for x in ca for y in cb if rv.prefix_value(a,b,x,y)==truth];assert len(winner)==1
 # Both winning factors are absent among ALL row/column conditionals; mixed supports also differ.
 absent=[]
 for v in r['variants']:
  W=list(map(F,v['W']));seen_a=[];seen_b=[]
  for i in range(n):
   mass=sum(W[i*m+j] for j in range(m));seen_b.append([W[i*m+j]/mass for j in range(m)])
  for j in range(m):
   mass=sum(W[i*m+j] for i in range(n));seen_a.append([W[i*m+j]/mass for i in range(n)])
  assert winner[0][0] not in seen_a and winner[0][1] not in seen_b
  absent.append(dict(shadow_sign=v['shadow_sign'],winning_A_absent=True,winning_B_absent=True))
 selected=r['rounding']['best'];sx=list(map(F,selected['alpha']));sy=list(map(F,selected['gamma']));assert sx==winner[0][0]
 ytrue=winner[0][1];bad_candidates=[]
 for ybad in cb:
  if ybad==ytrue:continue
  k=next(i for i in range(m) if ytrue[i]!=ybad[i]);lam=(sy[k]-ybad[k])/(ytrue[k]-ybad[k])
  if 0<lam<1 and all(sy[i]==lam*ytrue[i]+(1-lam)*ybad[i] for i in range(m)):bad_candidates.append((ybad,lam))
 assert len(bad_candidates)==1;ybad,lam=bad_candidates[0]
 qbad=rv.direct(a,b,selected['word'],sx,ybad);qgood=rv.direct(a,b,selected['word'],sx,ytrue)
 assert qgood==truth and truth-rounded==(1-lam)*(truth-qbad)
 responses=[rv.direct(a,b,selected['word'],sx,y) for y in cb]
 assert max(responses)==truth and sum(q==truth for q in responses)==1
 residual=dict(true_B_fraction=str(lam),losing_B_fraction=str(1-lam),losing_B_support=[i for i,w in enumerate(ybad) if w],identity='Q_true-Q_round=(1-lambda)(Q_true-Q_bad)',Q_bad=str(qbad),true_A_already_extracted=True,unique_B_response_to_extracted_A_is_true_B=True)
 out=dict(status='PASS',name=r['name'],geometry_chamber=geometry_chamber,residual_mixture=residual,LP_nondegeneracy=certs,strict_objective_deficit=str(truth-rounded),ratio_above_one_margin=str(F(r['ratio'])-1),unique_optimal_closure_pair=True,winning_supports=[[i for i,w in enumerate(x) if w] for x in winner[0]],absence=absent,consequence='Both cyclic angular orders stay fixed and both LP optima continue uniquely on their strictly complementary nondegenerate bases. Feasible closure vertices and best-response values vary continuously locally; strict true-versus-rounded objective deficit and sys>1 persist on a nonempty open neighborhood. No explicit neighborhood radius, arbitrary proximity to HKO, or 1% error lower bound is claimed.',results_sha256=hashlib.sha256((d/'results.json').read_bytes()).hexdigest(),root_stability_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest())
 args.out.write_text(json.dumps(out,indent=2)+'\n');print('PASS',r['name'],'both unique nondegenerate LP optima; missing pair',out['winning_supports'],'strict error',float(truth/rounded-1))
if __name__=='__main__':main()
