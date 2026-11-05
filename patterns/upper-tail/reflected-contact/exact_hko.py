#!/usr/bin/env python3
"""All-word uniform HKO optimum; one fixed closure, not capacity enumeration."""
import argparse, hashlib, importlib.util, itertools, json
from pathlib import Path

def main():
 p=argparse.ArgumentParser();p.add_argument('--repo',type=Path,required=True);p.add_argument('--out',type=Path,required=True);o=p.parse_args()
 qp=o.repo/'patterns/upper-tail/conditional-coupling/quadratic.py'
 sp=importlib.util.spec_from_file_location('q5',qp);m=importlib.util.module_from_spec(sp);sp.loader.exec_module(m);E=m.Q5
 d=E(0,1);A=[(E(1),E(0)),((d-1)/4,E(1)),(-(d+1)/4,(d-1)/2),(-(d+1)/4,-(d-1)/2),((d-1)/4,E(-1))];B=[(-y,x) for x,y in A]
 X=[tuple(t/5 for t in v) for v in A];Y=[tuple(t/5 for t in v) for v in B]
 assert all(sum(v[k] for v in X)==0 and sum(v[k] for v in Y)==0 for k in [0,1])
 def dot(u,v):return sum(a*b for a,b in zip(u,v))
 candidates=[]
 for tail in itertools.permutations(range(1,5)):
  order=(0,)+tail;prefix=[(E(0),E(0))]
  for i in order[:-1]:prefix.append(tuple(prefix[-1][k]+X[i][k] for k in [0,1]))
  gaps=[max(range(5),key=lambda k:dot(prefix[k],y)) for y in Y]
  Q=sum(dot(prefix[k],y) for k,y in zip(gaps,Y))/2
  word=[]
  for k,i in enumerate(order):word.extend(j+5 for j,g in enumerate(gaps) if g==k);word.append(i)
  direct=sum((1 if word.index(i)<word.index(j+5) else -1)*dot(X[i],Y[j])/4 for i in range(5) for j in range(5))
  assert direct==Q
  candidates.append(dict(A_order=list(order),B_gaps=gaps,word=word,Q=Q.serial()))
 maxQ=max(E.parse(c['Q']) for c in candidates);true=E(1)/10;ratio=maxQ/true
 assert maxQ<true
 out=dict(status='PASS',method='24 cyclic A orders; exact independent B gap maxima and direct word objective',A=[[v.serial() for v in u] for u in A],B=[[v.serial() for v in u] for u in B],normalized_closure_weights=['1/5']*5,all_candidates=candidates,max_Q=maxQ.serial(),true_Q=true.serial(),Q_fraction=ratio.serial(),capacity_inflation=(1/ratio).serial(),ratio_inflation=(1/(ratio*ratio)).serial(),max_Q_float=float(maxQ),capacity_inflation_float=float(1/ratio),ratio_inflation_float=float(1/(ratio*ratio)),input_hashes={str(qp.relative_to(o.repo)):hashlib.sha256(qp.read_bytes()).hexdigest()},script_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest())
 o.out.parent.mkdir(parents=True,exist_ok=True);o.out.write_text(json.dumps(out,indent=2)+'\n');print(json.dumps({k:v for k,v in out.items() if k not in ['A','B','all_candidates','input_hashes']},indent=2))
if __name__=='__main__':main()
