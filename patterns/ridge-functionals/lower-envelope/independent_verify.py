"""Independent rational certificate verifier; HK2017 earlier-to-later word convention."""
from fractions import Fraction as F
from itertools import product
from pathlib import Path
import argparse, hashlib, json

def dot(u,v): return sum(a*b for a,b in zip(u,v))
def cross(u,v): return u[0]*v[1]-u[1]*v[0]
def sub(u,v): return tuple(a-b for a,b in zip(u,v))
def geometry(raw):
    p=[tuple(map(F,v)) for v in raw]
    assert len(p)==5 and len(set(p))==5, 'exactly five distinct vertices'
    e=[sub(p[(i+1)%5],p[i]) for i in range(5)]
    assert all(cross(e[i],e[(i+1)%5])>0 for i in range(5)), 'strict convex CCW turns'
    normals=[]
    for v,edge in zip(p,e):
        n=(edge[1],-edge[0]); h=dot(n,v)
        assert h>0, 'origin strictly inside every supporting line'
        a=tuple(x/h for x in n)
        assert all(dot(a,w)<=1 for w in p), 'all vertices in every halfplane'
        assert sum(dot(a,w)==1 for w in p)==2, 'genuine edge with two endpoints'
        normals.append(a)
    area=sum(cross(p[i],p[(i+1)%5]) for i in range(5))/2
    assert area>0
    return p,e,normals,area

def verify(cert):
    p,ep,np,ap=geometry(cert['polygon_P']);q,eq,nq,aq=geometry(cert['polygon_Q'])
    word=[tuple(map(int,v)) for v in cert['word']];weights=list(map(F,cert['weights']))
    assert len(word)==len(weights) and len(set(word))==len(word)
    assert all(f in (0,1) and 0<=i<5 for f,i in word)
    assert all(w>0 for w in weights) and sum(weights)==1
    normal4=[(np[i]+(F(0),F(0))) if f==0 else ((F(0),F(0))+nq[i]) for f,i in word]
    assert all(sum(w*n[j] for w,n in zip(weights,normal4))==0 for j in range(4))
    def omega(a,b):return a[0]*b[2]+a[1]*b[3]-a[2]*b[0]-a[3]*b[1]
    value=sum(weights[i]*weights[j]*omega(normal4[i],normal4[j]) for i in range(len(word)) for j in range(i+1,len(word)))
    assert value>0
    c=1/(2*value);amax=max(abs(dot(e,f)) for e,f in product(ep,eq));v=ap*aq
    ratio=c*amax/v
    # Independent geometric control: canonical q1, p1+p2/2 pair.
    # q2-q1/2 and p2 complete it to a symplectic coordinate system.
    u=(F(1),F(0));z=(F(1),F(1,2));assert dot(u,z)==1
    qrange=[min(dot(u,x) for x in p),max(dot(u,x) for x in p)]
    prange=[min(dot(z,x) for x in q),max(dot(z,x) for x in q)]
    cylinder=(qrange[1]-qrange[0])*(prange[1]-prange[0])
    return dict(cylinder_capacity_upper_bound=str(cylinder),canonical_q1_range=list(map(str,qrange)),canonical_p1_range=list(map(str,prange)),status='PASS',hk_Q=str(value),capacity_upper_bound=str(c),maximum_ridge_area=str(amax),area_P=str(ap),area_Q=str(aq),c_upper_times_Amax_over_volume=str(ratio),decimal=float(ratio),threshold='4/5',strict_counterexample=ratio<F(4,5),sys_times_Rmax_squared_upper_bound=str(ratio**2/2),assumptions=['HK2017 capacity formula, nonnegative normalized closed weights give capacity upper bound','Mixed symplectic face areas of a Lagrangian product equal absolute primal edge dot products'])

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('certificate',type=Path);args=parser.parse_args()
    result=verify(json.loads(args.certificate.read_text()));result['certificate_sha256']=hashlib.sha256(args.certificate.read_bytes()).hexdigest()
    print(json.dumps(result,indent=2))
