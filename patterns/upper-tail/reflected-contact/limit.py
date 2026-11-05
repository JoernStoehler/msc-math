#!/usr/bin/env python3
"""Exact first-order reflected-cover contact-face collapse at HKO."""
import argparse,hashlib,importlib.util,itertools,json
from functools import cmp_to_key
from fractions import Fraction as F
from pathlib import Path

def main():
 p=argparse.ArgumentParser();p.add_argument('--repo',type=Path,required=True);p.add_argument('--directory',type=Path,required=True);o=p.parse_args();d=o.directory;qp=o.repo/'patterns/upper-tail/conditional-coupling/quadratic.py';sp=importlib.util.spec_from_file_location('limit_q5',qp);m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m);E=m.Q5
 class Dual:
  def __init__(self,v=0,t=0):
   if isinstance(v,Dual):self.v,self.t=v.v,v.t
   else:self.v,self.t=E(v),E(t)
  def __add__(self,y):y=Dual(y);return Dual(self.v+y.v,self.t+y.t)
  __radd__=__add__
  def __neg__(self):return Dual(-self.v,-self.t)
  def __sub__(self,y):return self+-Dual(y)
  def __rsub__(self,y):return Dual(y)+-self
  def __mul__(self,y):y=Dual(y);return Dual(self.v*y.v,self.t*y.v+self.v*y.t)
  __rmul__=__mul__
  def __truediv__(self,y):y=Dual(y);return Dual(self.v/y.v,(self.t*y.v-self.v*y.t)/(y.v*y.v))
  def __rtruediv__(self,y):return Dual(y)/self
 def dot(a,b):return sum(x*y for x,y in zip(a,b))
 def cross(a,b):return a[0]*b[1]-a[1]*b[0]
 hko=json.loads((d/'hko-exact.json').read_text());base={k:[[E.parse(x) for x in u] for u in hko[k]] for k in ['A','B']};src=o.repo/'patterns/upper-tail/contact-stability/sample.json';sample=json.loads(src.read_text());r=next(r for r in sample['rows'] if r['name']=='contact_full_s0_e3of1000');directions={k:[[(F(x)-F(y))/F(3,1000) for x,y in zip(u,v)] for u,v in zip(r[k],sample['base'][k])] for k in ['A','B']};out={};selected=[]
 for k in ['A','B']:
  a=[[Dual(x,t) for x,t in zip(u,v)] for u,v in zip(base[k],directions[k])];vv=[]
  for i in range(5):
   u,v=a[i],a[(i+1)%5];de=cross(u,v);assert de.v>0;vv.append([(v[1]-u[1])/de,(u[0]-v[0])/de])
  for i,v in enumerate(vv):
   for j,u in enumerate(a):
    z=dot(u,v).v;assert z==1 if j in [i,(i+1)%5] else z<1
  H=[];opposite=[]
  for u in a:
   vals=[-dot(u,v) for v in vv];best=max(range(5),key=lambda j:vals[j].v);assert all(vals[best].v>z.v for j,z in enumerate(vals) if j!=best);H.append(vals[best]);opposite.append(best)
  candidates=[]
  for ids in itertools.combinations(range(5),3):
   u,v,w=[a[i] for i in ids];nums=[cross(v,w),cross(w,u),cross(u,v)];den=sum(nums)
   if den.v==0:continue
   ww=[x/den for x in nums]
   if min(x.v for x in ww)<=0:continue
   lam=sum(x*H[i] for i,x in zip(ids,ww));assert lam.v==E(-1,1)
   candidates.append(dict(support=list(ids),weights_at_base=[x.v.serial() for x in ww],lambda_at_base=lam.v.serial(),lambda_derivative=lam.t.serial()))
  assert len(candidates)==5;winner=max(candidates,key=lambda x:E.parse(x['lambda_derivative']));gaps=[E.parse(winner['lambda_derivative'])-E.parse(c['lambda_derivative']) for c in candidates if c!=winner];assert min(gaps)>0;selected.append(winner)
  edges=[[y-x for x,y in zip(vv[i],vv[(i+1)%5])] for i in range(5)];ed=[cross(edges[i],edges[(i+1)%5]) for i in range(5)];contrast=ed[0]-ed[1];assert contrast.v==0 and contrast.t!=0
  out[k]=dict(direction=[[str(x) for x in v] for v in directions[k]],opposite_vertex_indices=opposite,candidates=candidates,selected=winner,strict_first_derivative_gaps=[x.serial() for x in gaps],edge_determinant_contrast_derivative=contrast.t.serial())
 Aids,Bids=[x['support'] for x in selected];X=[[E.parse(w)*z for z in base['A'][i]] for i,w in zip(Aids,selected[0]['weights_at_base'])];Y=[[E.parse(w)*z for z in base['B'][i]] for i,w in zip(Bids,selected[1]['weights_at_base'])];qs=[]
 for tail in itertools.permutations(range(1,6)):
  word=(0,)+tail;pos={v:i for i,v in enumerate(word)};q=sum((1 if pos[i]<pos[j+3] else -1)*dot(u,v)/4 for i,u in enumerate(X) for j,v in enumerate(Y));qs.append(q)
 q=max(qs);true=E(1)/10;cinf=true/q;sinf=cinf*cinf;rho=(E(3)+E(0,1))/5;assert q<true
 # Explicit positive-shadow attainment; its ten distinct rays persist near0.
 rays=base['A']+[[-v[1],v[0]] for v in base['B']]
 def half(v):return int(v[1]<0 or(v[1]==0 and v[0]<0))
 def angle_cmp(i,j):
  u,v=rays[i],rays[j];h=half(u)-half(v)
  if h:return h
  z=cross(u,v);return -1 if z>0 else 1 if z<0 else (i>j)-(i<j)
 angular=sorted(range(10),key=cmp_to_key(angle_cmp));cut=angular.index(0);angular=angular[cut:]+angular[:cut]
 assert angular==[0,8,1,9,2,5,3,6,4,7]
 assert all(cross(u,v)!=0 or dot(u,v)<0 for i,u in enumerate(rays) for v in rays[i+1:])
 aa=[E(0)]*5;gg=[E(0)]*5
 for i,w in zip(Aids,selected[0]['weights_at_base']):aa[i]=E.parse(w)
 for i,w in zip(Bids,selected[1]['weights_at_base']):gg[i]=E.parse(w)
 pos={v:i for i,v in enumerate(angular)};shadow=sum((1 if pos[i]<pos[j+5] else -1)*aa[i]*gg[j]*dot(u,v)/4 for i,u in enumerate(base['A']) for j,v in enumerate(base['B']));assert shadow==q
 result=dict(status='PASS',path='exact HKO+t times fixed contact_full_s0 published rational direction',factor_certificates=out,selected_supports=[Aids,Bids],limiting_max_Q=q.serial(),attaining_positive_shadow_word=angular,positive_shadow_Q=shadow.serial(),positive_shadow_ten_distinct_rays=True,true_HKO_Q=true.serial(),limiting_capacity_inflation=cinf.serial(),limiting_ratio_inflation=sinf.serial(),limiting_ratio_upper_bound=(sinf*rho).serial(),limiting_capacity_inflation_float=float(cinf),limiting_ratio_upper_bound_float=float(sinf*rho),all_selected_cyclic_words=120,base_input_sha256=hashlib.sha256((d/'hko-exact.json').read_bytes()).hexdigest(),sample_sha256=hashlib.sha256(src.read_bytes()).hexdigest(),plan_sha256=hashlib.sha256((d/'LIMIT-PLAN.md').read_bytes()).hexdigest(),script_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest());(d/'limit-certificate.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({k:v for k,v in result.items() if k!='factor_certificates'},indent=2))
if __name__=='__main__':main()
