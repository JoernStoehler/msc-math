#!/usr/bin/env python3
"""Exact dual-tight graph obstruction, one-factor vertices only; no pair objectives."""
import argparse,hashlib,importlib.util,json
from fractions import Fraction as F
from pathlib import Path

def main():
 p=argparse.ArgumentParser();p.add_argument('--repo',type=Path,required=True);p.add_argument('--out',type=Path,required=True);args=p.parse_args()
 sp=importlib.util.spec_from_file_location('independent_replay',Path(__file__).with_name('verify.py'));v=importlib.util.module_from_spec(sp);sp.loader.exec_module(v)
 source=args.repo/'patterns/upper-tail/conditional-coupling/results.json';s=next(r for r in json.loads(source.read_text())['rows'] if r['name']=='random_5x6_s1_198');a=[list(map(F,u)) for u in s['A']];b=[list(map(F,u)) for u in s['B']];qs=[v.lp_check(a,b,w,F)[0] for w in s['variants']];assert qs[0]<qs[1];w=s['variants'][1];slack=list(map(F,w['dual_slack']));n=len(a);m=len(b);rows=[]
 for alpha in v.vertices(a):
  support=[i for i,x in enumerate(alpha) if x];common=[j for j in range(m) if all(slack[i*m+j]==0 for i in support)];assert len(common)<=2
  if len(common)==1:assert b[common[0]]!=[F(0),F(0)];det=None
  elif len(common)==2:
   det=v.det(b[common[0]],b[common[1]]);assert det!=0 or v.dot(b[common[0]],b[common[1]])>=0
  else:det=None
  rows.append(dict(A_support=support,common_dual_tight_B_columns=common,B_pair_determinant=None if det is None else str(det),admits_B_closure=False))
 result=dict(status='PASS',name=s['name'],LP_Q_by_shadow=list(map(str,qs)),unique_maximizing_shadow=-1,all_A_closure_vertices=rows,rank_one_attainment_impossible=True,conclusion='Q_true < Q_LP, so capacity_lower < true capacity; no quantitative true gap claimed',source_sha256=hashlib.sha256(source.read_bytes()).hexdigest(),checker_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest());args.out.write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
if __name__=='__main__':main()
