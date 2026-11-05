exec(open(__file__.replace('bank.py','probe.py')).read().split("if __name__")[0])
source=Path('/workspaces/msc-math-viterbo-continuation/topics/capacity-and-qp/convex-systolic-optimization/certificates/hko-t03/leaves.txt')
bank=[list(map(int,l.split()))[1:] for l in source.read_text().splitlines()]
def scan(a,b):
 N=np.vstack([N0,a*N0+b*NT]);O=N@J@N.T;best=0;arg=None;nb=0;maxres=0
 for w in bank:
  m=len(w);H=np.triu(O[np.ix_(w,w)],1);H+=H.T;C=np.vstack([N[w].T,np.ones(m)]);K=np.block([[H,C.T],[C,np.zeros((5,5))]])
  z,res,rank,sv=np.linalg.lstsq(K,np.r_[np.zeros(m+4),1.],rcond=1e-11);be=z[:m]
  residual=max(abs(K@z-np.r_[np.zeros(m+4),1.]))
  if min(be)<-1e-9 or residual>1e-9:continue
  nb+=1;q=float(be@H@be/2)
  if q>best:best=q;arg=w;maxres=residual
 # Compute geometry independently of whether the two seed branches stay feasible.
 M=N[ids];det=np.linalg.det(M);keep=np.abs(det)>1e-10
 V=np.linalg.solve(M[keep],np.ones((keep.sum(),4,1)))[:,:,0]
 V=V[np.max(V@N.T,axis=1)<=1+1e-8];V=np.unique(np.round(V,10),axis=0)
 v=ConvexHull(V).volume

 return dict(a=a,b=b,Q=best,word=arg,volume=v,rho_upper=1/(8*v*best*best),feasible_count=nb,residual=maxres)
if __name__=='__main__':
 out=[]
 for a,b in [(1-.6*.3,np.cos(np.pi/5)*.3),(1-.6*.35,np.cos(np.pi/5)*.35),(1.01999841,.41996547),(.8,.3),(.85,.3),(.9,.3)]:
  r=scan(a,b);out.append(r);print(r,flush=True)
 (Path(__file__).parent/'bank-results.json').write_text(json.dumps(out,indent=2))
