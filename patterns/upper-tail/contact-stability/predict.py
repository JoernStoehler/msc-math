#!/usr/bin/env python3
"""Frozen local first-order contact geometry. No new-capacity scoring."""
import argparse,hashlib,importlib.util,json,math,sys
from itertools import combinations,product
from pathlib import Path

def module(n,p):
 s=importlib.util.spec_from_file_location(n,p);m=importlib.util.module_from_spec(s);sys.modules[n]=m;s.loader.exec_module(m);return m

def main():
 p=argparse.ArgumentParser();p.add_argument('--repo',type=Path,required=True);p.add_argument('--directory',type=Path,required=True);o=p.parse_args();d=o.directory;assert not (d/'predictions.json').exists()
 Q5=module('contact_q5',o.repo/'patterns/upper-tail/conditional-coupling/quadratic.py').Q5
 class Dual:
  def __init__(self,v,g=None):self.v=Q5(v);self.g=[Q5() for _ in range(20)] if g is None else g
  def __add__(self,o):
   if not isinstance(o,Dual):o=Dual(o)
   return Dual(self.v+o.v,[a+b for a,b in zip(self.g,o.g)])
  __radd__=__add__
  def __neg__(self):return Dual(-self.v,[-x for x in self.g])
  def __sub__(self,o):return self+-o if isinstance(o,Dual) else self+-Dual(o)
  def __rsub__(self,o):return Dual(o)+-self
  def __mul__(self,o):
   if not isinstance(o,Dual):o=Dual(o)
   return Dual(self.v*o.v,[a*o.v+b*self.v for a,b in zip(self.g,o.g)])
  __rmul__=__mul__
  def __truediv__(self,o):
   if not isinstance(o,Dual):o=Dual(o)
   return Dual(self.v/o.v,[(a*o.v-b*self.v)/(o.v*o.v) for a,b in zip(self.g,o.g)])
  def __rtruediv__(self,o):return Dual(o)/self
 def det(u,v):return u[0]*v[1]-u[1]*v[0]
 def dot(u,v):return sum(x*y for x,y in zip(u,v))
 def closure(a,ids):
  x,y,z=[a[i] for i in ids];v=[det(y,z),det(z,x),det(x,y)];ss=sum(v);w=[x/ss for x in v];return {i:x for i,x in zip(ids,w)}
 def area(a):
  vv=[]
  for i in range(5):
   u,v=a[i],a[(i+1)%5];c=det(u,v);vv.append([(v[1]-u[1])/c,(u[0]-v[0])/c])
  return sum(det(vv[i],vv[(i+1)%5]) for i in range(5))/2
 def objective(a,b,word,ids,jds):
  aa=closure(a,ids);bb=closure(b,jds);pos={i:p for p,i in enumerate(word)}
  return sum(aa[i]*bb[j]*dot(a[i],b[j])*(1 if pos[i]<pos[5+j] else -1) for i in ids for j in jds)/4
 src=o.repo/'patterns/upper-tail/conditional-coupling/hko-exact.json';h=json.loads(src.read_text());base={k:[[Q5.parse(v) for v in u] for u in h[k]] for k in ['A','B']};za=[x for k in ['A','B'] for u in base[k] for x in u];dual=[]
 for i,v in enumerate(za):
  g=[Q5(int(j==i)) for j in range(20)];dual.append(Dual(v,g))
 a=[dual[i:i+2] for i in range(0,10,2)];b=[dual[i:i+2] for i in range(10,20,2)];volume=area(a)*area(b)
 supports=[ids for ids in combinations(range(5),3) if min(closure(base['A'],ids).values())>0]
 words=[[0,8,1,9,2,5,3,6,4,7]]
 for swaps in product([False,True],repeat=5):
  w=[]
  for i,sw in enumerate(swaps):w.extend([5+i,i] if sw else [i,5+i])
  words.append(w[::-1])
 bank=[];seen=set();q0=Q5(1)/10
 for w in words:
  for ids in supports:
   for jds in supports:
    q=objective(base['A'],base['B'],w,ids,jds)
    assert q<=q0
    if q!=q0:continue
    q=objective(a,b,w,ids,jds);g=[2*x/q.v+y/volume.v for x,y in zip(q.g,volume.g)];key=tuple((x.a,x.b) for x in g)
    if key in seen:continue
    seen.add(key);bank.append(dict(A_support=ids,B_support=jds,word=w,Q_gradient=[x.serial() for x in q.g],loss_gradient=[x.serial() for x in g]))
 print('UNIQUE_BANK',len(bank),'V',float(volume.v),flush=True)
 data=json.loads((d/'sample.json').read_text());rows=[]
 for r in data['rows']:
  # Deliberately exact algebraic differences before converting each dot product.
  z=[Q5(x) for k in ['A','B'] for u in r[k] for x in u];delta=[x-y for x,y in zip(z,za)]
  loss=[sum(Q5.parse(x)*y for x,y in zip(v['loss_gradient'],delta)) for v in bank];qdiff=[sum(Q5.parse(x)*y for x,y in zip(v['Q_gradient'],delta)) for v in bank];D=max(loss);ql=q0+max(qdiff)
  ar=[[Q5(x) for x in u] for u in r['A']];br=[[Q5(x) for x in u] for u in r['B']];vol=area(ar)*area(br);r0=(Q5(3)+Q5(0,1))/5
  rows.append(dict(name=r['name'],contact_defect=float(D),contact_prediction=float(r0)*math.exp(-float(D)),Qlinear_prediction=float(1/(8*ql*ql*vol)),exact_volume=vol.serial(),active_forms=[i for i,x in enumerate(loss) if x==D]))
 out=dict(protocol='all formula bank and 60 predictions before new truth; no labels or fitted parameters',sample_sha256=hashlib.sha256((d/'sample.json').read_bytes()).hexdigest(),plan_sha256=hashlib.sha256((d/'PLAN.md').read_bytes()).hexdigest(),predict_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),known_HKO_ratio=float((Q5(3)+Q5(0,1))/5),bank=bank,base=base and {k:[[x.serial() for x in u] for u in base[k]] for k in base},rows=rows)
 (d/'predictions.json').write_text(json.dumps(out,indent=2)+'\n');print('SHA256',hashlib.sha256((d/'predictions.json').read_bytes()).hexdigest())
if __name__=='__main__':main()
