import itertools as it, numpy as np, json, math, time
from collections import Counter
from pathlib import Path
P=Path(__file__).resolve().parent; tm=time.time()
ang=np.pi/2+np.arange(5)*2*np.pi/5;n=np.stack([np.cos(ang),np.sin(ang)],axis=1)/np.cos(np.pi/5)
def blocks(ids):
 b=[(i,) for i in ids]
 for i,j in it.combinations(ids,2):
  if (i-j)%5 in (1,4):b.extend([(i,j),(j,i)])
 return b
def selections(bs,k):
 return [c for c in it.combinations(bs,k) if sum(map(len,c))==len(set(sum(c,())))]
def words(k):
 for qs in selections(blocks(range(5)),k):
  for ps in selections(blocks(range(5,10)),k):
   for perm in it.permutations(qs[1:]):
    for pp in it.permutations(ps):yield sum((qb+pb for qb,pb in zip((qs[0],)+perm,pp)),())
def mats(theta):
 a=np.zeros((10,4)); a[:5,:2]=n
 R=np.array([[np.cos(theta),-np.sin(theta)],[np.sin(theta),np.cos(theta)]])
 a[5:,2:]=n@R.T
 omega=a[:,:2]@a[:,2:].T-a[:,2:]@a[:,:2].T
 return a,omega
ad,od=mats(np.pi/20)
raw=[w for k in (2,3) for w in words(k)]
ws=[w for w in raw if all((i//5==j//5 and (i-j)%5 in(1,4)) or (i//5!=j//5 and od[i,j]>0) for i,j in zip(w,w[1:]+w[:1]))]
print('raw',len(raw),Counter(len(w) for w in raw),'pruned',len(ws),len(set(ws)),flush=True)
counts=Counter(); singular=[]
for w in ws:
 a=ad[list(w)];m=len(w);o=od[np.ix_(w,w)];H=np.triu(o,1);H+=H.T.copy();C=np.vstack([a.T,np.ones(m)])
 M=np.block([[H,C.T],[C,np.zeros((5,5))]])
 rhs=np.r_[np.zeros(m+4),1.]
 rank=np.linalg.matrix_rank(M,tol=1e-10)
 augrank=np.linalg.matrix_rank(np.c_[M,rhs],tol=1e-10)
 counts[(m,rank-(m+5),augrank-rank)]+=1
 if rank<m+5 and rank==augrank:
  sol=np.linalg.lstsq(M,rhs,rcond=1e-10)[0];q=.5*sol[:m]@H@sol[:m]
  singular.append({'word':w,'q_mid':q,'rank':int(rank),'m':m})
print('ranks',counts,'singularconsistent',len(singular),flush=True)
print('singular nonzeroq',Counter(v['m'] for v in singular if abs(v['q_mid'])>1e-9),flush=True)
(P/'classifier_rank_diagnostics.json').write_text(json.dumps({'counts':{str(k):v for k,v in counts.items()},'singular':singular},indent=2))
print('seconds',time.time()-tm)
