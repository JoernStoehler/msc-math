import numpy as np,json,itertools,time
from scipy.spatial import ConvexHull
from scipy.optimize import differential_evolution, minimize
from pathlib import Path
out=Path(__file__).resolve().parent
s=np.sqrt(5.);tau=np.tan(np.pi/5);al=(3-s)/2;be=tau*(1+s)/2
N0=np.array([[1,tau,0,0],[-al,be,0,0],[-(s-1),0,0,0],[-al,-be,0,0],[1,-tau,0,0],[0,0,tau,-1],[0,0,be,al],[0,0,0,s-1],[0,0,-be,al],[0,0,-tau,-1]],float)
J=np.block([[np.zeros((2,2)),np.eye(2)],[-np.eye(2),np.zeros((2,2))]])
J2=np.array([[0,-1],[1,0.]])
T=np.block([[np.zeros((2,2)),-J2],[J2,np.zeros((2,2))]])
NT=N0@T
ids=np.array(list(itertools.combinations(range(20),4)))
words=[np.array([0,1,6,7,3,12,9]),np.array([0,6,5,2,19,11,8,4])]
records=[];ncall=0;start=time.monotonic()
def calc(par,save=False):
 global ncall
 ncall+=1;a,b=par;N=np.vstack([N0,a*N0+b*NT]);M=N[ids];det=np.linalg.det(M);keep=np.abs(det)>1e-10
 V=np.linalg.solve(M[keep],np.ones((keep.sum(),4,1)))[:,:,0]; V=V[np.max(V@N.T,axis=1)<=1+1e-8]
 V=np.unique(np.round(V,10),axis=0)
 if len(V)<5:return 1e3
 volume=ConvexHull(V).volume
 O=N@J@N.T;qs=[];weights=[]
 for w in words:
  m=len(w);H=np.triu(O[np.ix_(w,w)],1);H=H+H.T;C=np.vstack([N[w].T,np.ones(m)])
  K=np.block([[H,C.T],[C,np.zeros((5,5))]])
  try:z=np.linalg.solve(K,np.r_[np.zeros(m+4),1.]);beta=z[:m]
  except np.linalg.LinAlgError:return 1e3
  if min(beta)<-1e-8:return 1e3
  qs.append(float(beta@H@beta/2));weights.append(beta.tolist())
 rho=1/(8*volume*max(qs)**2)
 if save:records.append(dict(a=a,b=b,volume=volume,vertices=len(V),Q=qs,weights=weights,rho_feasible_upper=rho))
 if ncall%100==0:print('calls',ncall,'secs',time.monotonic()-start,'rho',rho,flush=True)
 return -rho
if __name__=='__main__':
 for t in [.3,.34,.35,.4]:calc([1-.6*t,t*np.cos(np.pi/5)],True)
 print('controls',records,flush=True)
 opt=differential_evolution(calc,[(.72,1.02),(.14,.42)],maxiter=25,popsize=8,tol=1e-7,polish=False,seed=3947,workers=1)
 calc(opt.x,True)
 (out/'result.json').write_text(json.dumps(dict(method='floating two-word feasible-bound optimization only; no capacity determination',calls=ncall,seconds=time.monotonic()-start,controls_and_candidate=records,result=str(opt)),indent=2))
 print('best',opt.x,-opt.fun,'calls',ncall,flush=True)
