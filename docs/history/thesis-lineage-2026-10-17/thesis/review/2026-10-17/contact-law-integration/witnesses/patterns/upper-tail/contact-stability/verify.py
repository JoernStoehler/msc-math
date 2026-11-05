#!/usr/bin/env python3
"""Independent exact predicate, geometry, oracle and contact-gradient checker."""
import argparse,hashlib,importlib.util,json,sys
from fractions import Fraction as F
from itertools import combinations,permutations,product
from pathlib import Path

def module(n,p):
 s=importlib.util.spec_from_file_location(n,p);m=importlib.util.module_from_spec(s);sys.modules[n]=m;s.loader.exec_module(m);return m

def rank(rows):
 basis={}
 for source in rows:
  row=source[:]
  for k,v in sorted(basis.items()):
   c=row[k];row=[x-c*y for x,y in zip(row,v)]
  k=next((i for i,x in enumerate(row) if x),None)
  if k is not None:
   c=row[k];basis[k]=[x/c for x in row]
 return len(basis)

def main():
 p=argparse.ArgumentParser();p.add_argument('--repo',type=Path,required=True);p.add_argument('--directory',type=Path,required=True);o=p.parse_args();d=o.directory
 q=module('verify_contact_q5',o.repo/'patterns/upper-tail/conditional-coupling/quadratic.py');Q=q.Q5;v=module('verify_contact_independent',o.repo/'patterns/upper-tail/near-hko-transfer/verify.py');pred=json.loads((d/'predictions.json').read_text());truth=json.loads((d/'truth.json').read_text());sample=json.loads((d/'sample.json').read_text());h=json.loads((o.repo/'patterns/upper-tail/conditional-coupling/hko-exact.json').read_text());base={k:[[Q.parse(x) for x in u] for u in h[k]] for k in ['A','B']};gs=[[Q.parse(x) for x in b['loss_gradient']] for b in pred['bank']]
 base_geometry={}
 for k in ['A','B']:
  fac=base[k];pair=[abs(v.det(x,y)) for x,y in combinations(fac,2)];triple=[abs(v.det([x[c]-z[c] for c in range(2)],[y[c]-z[c] for c in range(2)])) for x,y,z in combinations(fac,3)];assert min(pair)>0 and min(triple)>0;base_geometry[k]=dict(min_pair_determinant=min(pair).serial(),min_triple_affine_determinant=min(triple).serial())
 assert len(gs)==20 and rank(gs)==11 and all(sum(g[k] for g in gs)==0 for k in range(20))
 # Tangents of four translations, a common dilation and all four product symplectic GL(2) generators.
 tangents=[]
 for k in ['A','B']:
  for c in range(2):tangents.append([-(u[c]*x) if kk==k else Q() for kk in ['A','B'] for u in base[kk] for x in u])
 tangents.append([-x for kk in ['A','B'] for u in base[kk] for x in u])
 for i in range(2):
  for j in range(2):
   t=[]
   for k in ['A','B']:
    for u in base[k]:
     t.extend([-u[i] if c==j else Q() for c in range(2)] if k=='A' else [u[j] if c==i else Q() for c in range(2)])
   tangents.append(t)
 assert rank(tangents)==9 and all(sum(x*y for x,y in zip(g,t))==0 for g in gs for t in tangents)
 # Bank equality, Q and volume gradients checked independently by coefficient differentiation in a rational-function power series.
 # Only constant/linear terms are needed; each tuple is a univariate infinitesimal for one coordinate.
 class Jet:
  def __init__(self,v,c=0):self.v=Q(v);self.c=Q(c)
  def __add__(self,b):
   if not isinstance(b,Jet):b=Jet(b)
   return Jet(self.v+b.v,self.c+b.c)
  __radd__=__add__
  def __neg__(self):return Jet(-self.v,-self.c)
  def __sub__(self,b):return self+-b if isinstance(b,Jet) else self+-Jet(b)
  def __rsub__(self,b):return Jet(b)+-self
  def __mul__(self,b):
   if not isinstance(b,Jet):b=Jet(b)
   return Jet(self.v*b.v,self.c*b.v+self.v*b.c)
  __rmul__=__mul__
  def __truediv__(self,b):
   if not isinstance(b,Jet):b=Jet(b)
   return Jet(self.v/b.v,self.c/b.v-self.v*b.c/(b.v*b.v))
  def __rtruediv__(self,b):return Jet(b)/self
 def area(a):
  vv=[]
  for i in range(5):
   x,y=a[i],a[(i+1)%5];det=v.det(x,y);vv.append([(y[1]-x[1])/det,(x[0]-y[0])/det])
  return sum(v.det(vv[i],vv[(i+1)%5]) for i in range(5))/2
 def closure(a,ids):
  # Cramer's rule for [1; x; y] weights, using independent local solve in Q5.
  x,y,z=[a[i] for i in ids];c=[v.det(y,z),v.det(z,x),v.det(x,y)];s=sum(c);w=[x/s for x in c];return {i:x for i,x in zip(ids,w)}
 def branch(a,b,bank):
  aa=closure(a,bank['A_support']);bb=closure(b,bank['B_support']);pos={j:i for i,j in enumerate(bank['word'])};return sum(aa[i]*bb[j]*v.dot(a[i],b[j])*(1 if pos[i]<pos[5+j] else -1) for i in aa for j in bb)/4
 bankchecks=0
 for bank in pred['bank']:
  for c in range(20):
   arr={};idx=0
   for k in ['A','B']:
    arr[k]=[]
    for u in base[k]:
     w=[]
     for x in u:w.append(Jet(x,int(idx==c)));idx+=1
     arr[k].append(w)
   objective=branch(arr['A'],arr['B'],bank);vol=area(arr['A'])*area(arr['B']);assert objective.v==Q(1)/10;assert objective.c==Q.parse(bank['Q_gradient'][c]);assert 2*objective.c/objective.v+vol.c/vol.v==Q.parse(bank['loss_gradient'][c]);bankchecks+=1
 # Reconstruct every geometry independent of freeze.py.
 qrat=F(22360679775,10000000000);rbase={k:[[F(x['rational'])+qrat*F(x['sqrt5']) for x in u] for u in h[k]] for k in ['A','B']}
 for row in sample['rows']:
  raw=hashlib.sha256(f"contact-stability-common-v1:{row['seed']}".encode()).digest();directions=[F(byte-128,128) for byte in raw[:20]];eps=F(row['epsilon']);assert row['direction_digest']==raw.hex()
  for ki,k in enumerate(['A','B']):
   for i,(x,y) in enumerate(rbase[k]):
    j=ki*10+2*i
    if row['family']=='full':u,w=directions[j:j+2]
    elif row['family']=='support':u,w=directions[j]*x,directions[j]*y
    else:u,w=-directions[j]*y,directions[j]*x
    assert list(map(F,row[k][i]))==[x+eps*u,y+eps*w]
 # Oracle reconstructed using independent Gauss closure vertices + direct 4D HK (not producer signed matrix formula).
 assert len(truth['rows'])==len(sample['rows'])==len(pred['rows'])==60;objectives=0
 for row,t,pr in zip(sample['rows'],truth['rows'],pred['rows']):
  assert row['name']==t['name']==pr['name'];a=[list(map(F,u)) for u in row['A']];b=[list(map(F,u)) for u in row['B']];ca=v.vertices(a);cb=v.vertices(b);vol=v.area(a)*v.area(b);assert vol==F(t['volume'])
  qs=[]
  for sign in [1,-1]:
   word=v.word(a,b,sign)
   for aa in ca:
    for bb in cb:qs.append(v.q(a,b,word,aa,bb));objectives+=1
  maxq=max(qs);assert F(t['capacity'])==1/(2*maxq);assert F(t['ratio'])==1/(8*maxq*maxq*vol)
  z=[Q(x) for k in ['A','B'] for u in row[k] for x in u];z0=[x for k in ['A','B'] for u in base[k] for x in u];delta=[x-y for x,y in zip(z,z0)];D=max(sum(x*y for x,y in zip(g,delta)) for g in gs);assert abs(float(D)-pr['contact_defect'])<1e-14
 out=dict(status='PASS',input_sha256={n:hashlib.sha256((d/n).read_bytes()).hexdigest() for n in ['PLAN.md','sample.json','predictions.json','truth.json','metrics.json','verify.py']},exact_base_geometry=base_geometry,gradient_count=20,gradient_rank=11,uniform_positive_relation=True,symmetry_tangent_rank=9,all_gradients_annihilate_symmetries=True,independent_coordinate_derivative_checks=bankchecks,exact_geometry_inputs=60,independent_direct_HK_objectives=objectives,all_genuine_bounded_five_by_five=True)
 (d/'verification.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
if __name__=='__main__':main()
