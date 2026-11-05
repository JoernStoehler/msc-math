# /// script
# requires-python = ">=3.11"
# dependencies = ["numpy>=2", "scipy>=1.14"]
# ///
"""Floating feasible-word-bank upper bounds; not a complete capacity solver."""
import argparse,itertools,json,time
from pathlib import Path
import numpy as np
from scipy.spatial import ConvexHull
from scipy.optimize import differential_evolution
ROOT=Path(__file__).resolve().parents[4]
S=np.sqrt(5.);TAU=np.tan(np.pi/5);CO=np.cos(np.pi/5)
AL=(3-S)/2;BE=TAU*(1+S)/2
N0=np.array([[1,TAU,0,0],[-AL,BE,0,0],[-(S-1),0,0,0],[-AL,-BE,0,0],[1,-TAU,0,0],[0,0,TAU,-1],[0,0,BE,AL],[0,0,0,S-1],[0,0,-BE,AL],[0,0,-TAU,-1]],float)
J=np.block([[np.zeros((2,2)),np.eye(2)],[-np.eye(2),np.zeros((2,2))]])
J2=np.array([[0,-1],[1,0.]])
T=np.block([[np.zeros((2,2)),-J2],[J2,np.zeros((2,2))]])
IDS=np.array(list(itertools.combinations(range(20),4)))
SOURCE=ROOT/'topics/capacity-and-qp/convex-systolic-optimization/certificates/hko-t03/leaves.txt'
BANK=[list(map(int,l.split()))[1:] for l in SOURCE.read_text().splitlines()]
BANK.extend([[0,1,6,7,3,12,9],[0,6,5,2,19,11,8,4]])
def scan(a,b):
    N=np.vstack([N0,a*N0+b*(N0@T)]);O=N@J@N.T
    best=0.;arg=None;weights=None;count=0;maxres=None
    for w in BANK:
        m=len(w);H=np.triu(O[np.ix_(w,w)],1);H+=H.T
        C=np.vstack([N[w].T,np.ones(m)]);K=np.block([[H,C.T],[C,np.zeros((5,5))]])
        rhs=np.r_[np.zeros(m+4),1.]
        z,_,_,_=np.linalg.lstsq(K,rhs,rcond=1e-11);be=z[:m]
        residual=float(max(abs(K@z-rhs)))
        if min(be)<-1e-9 or residual>1e-9:continue
        count+=1;q=float(be@H@be/2)
        if q>best:best=q;arg=w;weights=be.tolist();maxres=residual
    assert best>0
    M=N[IDS];keep=np.abs(np.linalg.det(M))>1e-10
    V=np.linalg.solve(M[keep],np.ones((int(keep.sum()),4,1)))[:,:,0]
    V=V[np.max(V@N.T,axis=1)<=1+1e-8]
    V=np.unique(np.round(V,10),axis=0);volume=float(ConvexHull(V).volume)
    return dict(a=float(a),b=float(b),Q=best,word=arg,weights=weights,volume=volume,vertices=len(V),rho_upper_estimate=1/(8*volume*best*best),numerically_feasible_words=count,KKT_residual=maxres)
def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--out',type=Path,required=True);p.add_argument('--search',action='store_true');args=p.parse_args()
    if args.out.exists():p.error('output exists; use a fresh run path')
    start=time.monotonic()
    controls=[scan(1-.6*t,t*CO) for t in [.3,.35]]
    assert abs(controls[0]['rho_upper_estimate']-1.0519141388294875)<1e-8
    candidate=scan(99/125,(7/20)*CO)
    result=dict(scope='Floating feasible-word upper estimates only. No graph coverage, exact feasibility or complete capacity claim.',method='2326 frozen .3 leaf words plus two retained branch words; nonnegative numerical KKT solutions; four-row geometry enumeration and Qhull volume',controls=controls,candidate_input=dict(a='99/125',b='(7/20)*cos(pi/5)'),candidate=candidate)
    if args.search:
        def obj(x):
            try:return -scan(*x)['rho_upper_estimate']
            except (ValueError,np.linalg.LinAlgError):return 1e3
        opt=differential_evolution(obj,[(.72,.85),(.18,.33)],popsize=8,maxiter=25,tol=1e-6,polish=False,seed=324)
        result['search_candidate']=scan(*opt.x);result['search_evaluations']=int(opt.nfev)
    result['seconds']=time.monotonic()-start
    args.out.write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
if __name__=='__main__':main()
