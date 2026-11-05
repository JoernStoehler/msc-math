#!/usr/bin/env python3
"""Compress the twenty first-order forms under the exact HKO stabilizer."""
import argparse,hashlib,importlib.util,json,sys
from pathlib import Path

def main():
 p=argparse.ArgumentParser();p.add_argument('--repo',type=Path,required=True);p.add_argument('--directory',type=Path,required=True);o=p.parse_args();d=o.directory;s=importlib.util.spec_from_file_location('sym_q5',o.repo/'patterns/upper-tail/conditional-coupling/quadratic.py');m=importlib.util.module_from_spec(s);s.loader.exec_module(m);Q=m.Q5;pr=json.loads((d/'predictions.json').read_text());gs=[[Q.parse(x) for x in r['loss_gradient']] for r in pr['bank']];c=(Q(0,1)-1)/4;M=[[c,-(Q(0,1)+5)/8],[Q(1),c]];MiT=[[c,-Q(1)],[(Q(0,1)+5)/8,c]]
 def mat(A,v):return [sum(x*y for x,y in zip(row,v)) for row in A]
 def transform(z,which):
  a=[z[i:i+2] for i in range(0,10,2)];b=[z[i:i+2] for i in range(10,20,2)]
  if which=='rotation':a,b=[mat(M,a[(i-1)%5]) for i in range(5)],[mat(MiT,b[(i-1)%5]) for i in range(5)]
  elif which=='reflection':a,b=[[a[(-i)%5][0],-a[(-i)%5][1]] for i in range(5)],[[-b[(-i)%5][0],b[(-i)%5][1]] for i in range(5)]
  elif which=='swap':a,b=[[v[1],-v[0]] for v in b],[[-v[1],v[0]] for v in a]
  return [x for ar in [a,b] for u in ar for x in u]
 z0=[Q.parse(x) for k in ['A','B'] for u in pr['base'][k] for x in u];perms={}
 for gen in ['rotation','reflection','swap']:
  assert transform(z0,gen)==z0;cols=[transform([Q(int(i==j)) for i in range(20)],gen) for j in range(20)];pullbacks=[[sum(x*y for x,y in zip(g,col)) for col in cols] for g in gs];perm=[]
  for g in pullbacks:assert g in gs;perm.append(gs.index(g))
  assert len(set(perm))==20;perms[gen]=perm
 remaining=set(range(20));orbits=[]
 while remaining:
  orbit={min(remaining)};old=set()
  while orbit!=old:
   old=orbit.copy();orbit.update(perm[i] for perm in perms.values() for i in old)
  orbits.append(sorted(orbit));remaining-=orbit
 out=dict(status='PASS',orbits=orbits,template_count=len(orbits),templates=[pr['bank'][orb[0]] for orb in orbits],generator_permutations=perms,rotation_matrix=[[x.serial() for x in row] for row in M],inverse_transpose_rotation=[[x.serial() for x in row] for row in MiT],notes='reflection is antisymplectic; rotation and factorquarterturn swap are symplectic; all fixHKO afterlabels',input_sha256=hashlib.sha256((d/'predictions.json').read_bytes()).hexdigest());(d/'symmetry.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps({k:v for k,v in out.items() if k not in ['templates']},indent=2))
if __name__=='__main__':main()
