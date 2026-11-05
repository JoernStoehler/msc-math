import numpy as np,itertools,json,time
from pathlib import Path
T=np.array(list(itertools.combinations(range(5),3)))
W=np.array([(0,)+p for p in itertools.permutations(range(1,6))])
POS=np.argsort(W,axis=1)
SGN=np.where(POS[:,:3,None]<POS[:,None,3:],1.,-1.)
def cross(a,b):return a[...,0]*b[...,1]-a[...,1]*b[...,0]
def closure(p):
 e=np.roll(p,-1,axis=0)-p
 n=np.stack([e[:,1],-e[:,0]],axis=1)
 h=np.sum(n*p,axis=1)
 if min(h)<=1e-10:return None
 n=n/h[:,None]
 a=n[T];w=np.stack([cross(a[:,1],a[:,2]),cross(a[:,2],a[:,0]),cross(a[:,0],a[:,1])],axis=1)
 s=np.sum(w,axis=1)
 valid=abs(s)>1e-12;w[valid]/=s[valid,None]
 valid &=np.min(w,axis=1)>=-1e-10
 return n,e,T[valid],w[valid]
def obj(p,q,ret=False):
 pa=closure(p);qb=closure(q)
 if pa is None or qb is None:return 1e6
 a,e,idx,u=pa;b,f,jdx,v=qb
 if len(u)==0 or len(v)==0:return 1e6
 weighted_a=a[idx]*u[:,:,None]/2;weighted_b=b[jdx]*v[:,:,None]/2
 h=np.einsum('aix,bjx->abij',weighted_a,weighted_b)
 vals=np.einsum('abij,wij->abw',h,SGN)
 ind=np.unravel_index(np.argmax(vals),vals.shape);Q=vals[ind]
 area=lambda x:abs(np.sum(cross(x,np.roll(x,-1,axis=0))))/2
 Amax=np.max(abs(e@f.T));c=1/(2*Q);z=c*Amax/(area(p)*area(q))
 if ret:return z,ind,(idx[ind[0]],jdx[ind[1]]),(u[ind[0]]/2,v[ind[1]]/2),W[ind[2]],Q
 return z
if __name__=='__main__':
 S=np.array([[-1,0],[-1,1],[0,1],[1,0],[1,-1],[0,-1]],float)[::-1]
 for i in range(6):
  p=np.delete(S,i,0);p-=p.mean(axis=0);q=np.stack([-p[:,1],p[:,0]],1)
  print(i,obj(p,q,True),flush=True)
