from search import *
rng=np.random.default_rng(20261017);start=time.monotonic();counts={};best={};evals=0
import argparse
parser=argparse.ArgumentParser();parser.add_argument('--out',type=Path,required=True);args=parser.parse_args()
out=args.out;out.mkdir(parents=True,exist_ok=True)

def valid(p):return min(cross(np.roll(p,-1,axis=0)-p,np.roll(p,-2,axis=0)-np.roll(p,-1,axis=0)))>1e-8

def evaluate(p,q,family):
 global evals
 if not valid(p) or not valid(q):return 1e6
 p=p-p.mean(axis=0);q=q-q.mean(axis=0)
 z=obj(p,q);evals+=1;counts[family]=counts.get(family,0)+1
 if z<best.get(family,{}).get('value',1e6):
  best[family]={'value':z,'P':p.tolist(),'Q':q.tolist()}
 if z<.799999:
  (out/'candidate.json').write_text(json.dumps({'value':z,'P':p.tolist(),'Q':q.tolist(),'family':family},indent=2))
  (out/'search-receipt.json').write_text(json.dumps({'seed':20261017,'feasible_pair_evaluations':evals,'counts':counts,'initial_floating_value':z,'elapsed_seconds':time.monotonic()-start,'stop_reason':'first strict floating falsifier; optimization halted for rational certificate','family':family},indent=2));print('COUNTEREXAMPLE',family,z,flush=True);raise SystemExit
 return z

def transform(p):
 t=rng.uniform(0,2*np.pi);rot=np.array([[np.cos(t),-np.sin(t)],[np.sin(t),np.cos(t)]])
 s=np.exp(rng.uniform(-1,1));mat=rot@np.array([[s,rng.normal(0,.7)],[0,1/s]])
 return p@mat.T

def optimize(p,q,family,steps=600):
 z=evaluate(p,q,family)
 for k in range(steps):
  scale=.2*(.02/.2)**(k/steps)
  pp=p.copy();qq=q.copy()
  if rng.random()<.5:
   i=rng.integers(5);pp[i]+=rng.normal(size=2)*scale
  else:
   i=rng.integers(5);qq[i]+=rng.normal(size=2)*scale
  zz=evaluate(pp,qq,family)
  if zz<z:p,q,z=pp,qq,zz
 return z

def hull(a):
 pts=sorted(map(tuple,a));l=[];u=[]
 for p in pts:
  while len(l)>1 and np.cross(np.subtract(l[-1],l[-2]),np.subtract(p,l[-1]))<=0:l.pop()
  l.append(p)
 for p in pts[::-1]:
  while len(u)>1 and np.cross(np.subtract(u[-1],u[-2]),np.subtract(p,u[-1]))<=0:u.pop()
  u.append(p)
 return np.array(l[:-1]+u[:-1])

angle=np.arange(5)*2*np.pi/5;reg=np.stack([np.cos(angle),np.sin(angle)],1)
print('regular equality',obj(reg,reg),flush=True)
S=np.array([[-1,0],[-1,1],[0,1],[1,0],[1,-1],[0,-1]],float)[::-1]
for run in range(300):
 family=['regular-perturbations','hexagon-deletions','arbitrary-convex'][run%3]
 if run%3==0:p=transform(reg);q=transform(reg)
 elif run%3==1:p=transform(np.delete(S,rng.integers(6),0));q=transform(np.delete(S,rng.integers(6),0))
 else:
  p=hull(rng.normal(size=(rng.integers(6,13),2)));q=hull(rng.normal(size=(rng.integers(6,13),2)))
  if len(p)!=5 or len(q)!=5:continue
 optimize(p,q,family,steps=600)
 if run%15==0:
  print(run,evals,{f:b['value'] for f,b in best.items()},'secs',time.monotonic()-start,flush=True)
  (out/'summary.json').write_text(json.dumps({'counts':counts,'best':best,'elapsed':time.monotonic()-start,'seed':20261017},indent=2))
 if time.monotonic()-start>300:break
(out/'summary.json').write_text(json.dumps({'counts':counts,'best':best,'elapsed':time.monotonic()-start,'seed':20261017},indent=2))
print('DONE',counts,{f:b['value'] for f,b in best.items()},flush=True)
