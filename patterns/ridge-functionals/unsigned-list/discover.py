#!/usr/bin/env python3
"""Exact complete closure-vertex/cyclic-word enumeration; standard library only.
Scope: at most ten prescribed A+B/A-B examples, terminating at first gap.
"""
from fractions import Fraction as F
from itertools import combinations, permutations
from pathlib import Path
import argparse, hashlib, json, time

def cross(a,b):return a[0]*b[1]-a[1]*b[0]
def dot(a,b):return a[0]*b[0]+a[1]*b[1]
def sub(a,b):return tuple(x-y for x,y in zip(a,b))
def hull(points):
    points=sorted(set(tuple(map(F,p)) for p in points));lower=[];upper=[]
    for p in points:
        while len(lower)>1 and cross(sub(lower[-1],lower[-2]),sub(p,lower[-1]))<=0:lower.pop()
        lower.append(p)
    for p in reversed(points):
        while len(upper)>1 and cross(sub(upper[-1],upper[-2]),sub(p,upper[-1]))<=0:upper.pop()
        upper.append(p)
    return lower[:-1]+upper[:-1]
def edges(p):return [sub(p[(i+1)%len(p)],p[i]) for i in range(len(p))]
def area(p):return sum(cross(p[i],p[(i+1)%len(p)]) for i in range(len(p)))/2
def normals(p):
    out=[]
    for x,e in zip(p,edges(p)):
        n=(e[1],-e[0]);h=dot(n,x);assert h>0
        out.append(tuple(z/h for z in n))
    assert all(dot(n,x)<=1 for n in out for x in p)
    # Exact converse H-vertex check, including nonadjacent boundary lines.
    vv=set()
    for n,m in combinations(out,2):
        d=cross(n,m)
        if not d:continue
        x=((m[1]-n[1])/d,(n[0]-m[0])/d)
        if all(dot(u,x)<=1 for u in out):vv.add(x)
    assert vv==set(p)
    return out

def closure_vertices(u):
    out=[]
    # Singletons impossible: normalized outward normals are nonzero.
    assert all(v!=(0,0) for v in u)
    for i,j in combinations(range(len(u)),2):
        a,b=u[i],u[j]
        if cross(a,b)!=0 or dot(a,b)>=0:continue
        k=next(k for k in range(2) if a[k]!=b[k]);t=-b[k]/(a[k]-b[k]);w=(t,1-t)
        assert min(w)>0 and all(w[0]*a[k]+w[1]*b[k]==0 for k in range(2))
        out.append(((i,j),w))
    for idx in combinations(range(len(u)),3):
        a,b,c=(u[i] for i in idx);z=(cross(b,c),cross(c,a),cross(a,b));s=sum(z)
        if not s:continue
        w=tuple(v/s for v in z)
        if min(w)<=0:continue
        assert sum(w)==1 and all(sum(w[j]*u[idx[j]][k] for j in range(3))==0 for k in range(2))
        out.append((idx,w))
    assert out
    return out

def capacity(p,q):
    u,v=normals(p),normals(q);a,b=closure_vertices(u),closure_vertices(v);best=F(0);count=0;win=None
    for (ii,aa),(jj,bb) in ((x,y) for x in a for y in b):
        n,m=len(ii),len(jj)
        terms=[(i,n+j,aa[i]*bb[j]*dot(u[ii[i]],v[jj[j]])/4) for i in range(n) for j in range(m)]
        for tail in permutations(range(1,n+m)):
            word=(0,)+tail;pos={x:i for i,x in enumerate(word)}
            val=sum((z if pos[i]<pos[j] else -z) for i,j,z in terms);count+=1
            if val>best:
                best=val;win=dict(q_support=ii,p_support=jj,q_normalized_weights=aa,p_normalized_weights=bb,word=word)
    assert best>0
    return dict(capacity=1/(2*best),Q_max=best,closure_vertices_q=len(a),closure_vertices_p=len(b),two_support_q=sum(len(i)==2 for i,w in a),two_support_p=sum(len(i)==2 for i,w in b),cyclic_words=count,witness=win,normals_q=u,normals_p=v)

def serial(o):
    if isinstance(o,F):return str(o)
    raise TypeError(o)

def main():
    parser=argparse.ArgumentParser();parser.add_argument('--out',type=Path,required=True,help='Explicit scratch output directory');args=parser.parse_args()
    start=time.perf_counter()
    # Seed first; remaining nine vary widths/asymmetries, not near-equality tuning.
    seeds=[(2,1,3,1,2,3,5,3),(3,1,2,1,3,2,4,3),(1,2,3,2,1,3,5,2),(2,2,1,1,3,2,3,5),(3,2,1,2,1,4,4,5),(1,1,4,3,2,1,6,3),(2,3,1,3,1,2,3,4),(3,1,4,2,3,1,4,3),(1,3,2,2,1,4,3,5),(3,2,4,2,3,1,5,4)]
    trials=[]
    control=hull([(-1,-1),(2,-1),(-1,2)]);ctrl=capacity(control,control)
    assert ctrl['capacity']==F(9,2)
    for params in seeds:
        if time.perf_counter()-start>550:break
        ax,ay,at,bx,by,bt,qx,qy=params
        A=hull([(-ax,-ay),(ax,-ay),(0,at)]);B=hull([(-bx,-by),(-bx,by),(bt,0)]);Q=hull([(-1,-1),(qx-1,-1),(-1,qy-1)])
        plus=hull([(a[0]+b[0],a[1]+b[1]) for a in A for b in B]);minus=hull([(a[0]-b[0],a[1]-b[1]) for a in A for b in B])
        assert area(plus)==area(minus)>0 and hull([(-x,y) for x,y in plus])==minus
        unsigned_plus=sorted(abs(dot(e,f)) for e in edges(plus) for f in edges(Q));unsigned_minus=sorted(abs(dot(e,f)) for e in edges(minus) for f in edges(Q))
        assert unsigned_plus==unsigned_minus
        rplus=capacity(plus,Q);rminus=capacity(minus,Q);vol=area(plus)*area(Q)
        sp=rplus['capacity']**2/(2*vol);sm=rminus['capacity']**2/(2*vol)
        record=dict(params=params,A=A,B=B,Q=Q,P_plus=plus,P_minus=minus,area_P=area(plus),area_Q=area(Q),volume=vol,full_ridge_count=len(plus)*len(Q)+len(plus)+len(Q),mixed_ridge_count=len(unsigned_plus),unsigned_cross_ridge_multiset=unsigned_plus,full_unsigned_ridge_multiset=sorted(unsigned_plus+[F(0)]*(len(plus)+len(Q))),normalized_unsigned_cross_ridge_squared_multiset=[z*z/vol for z in unsigned_plus],signed_cross_multisets_equal=sorted(dot(e,f) for e in edges(plus) for f in edges(Q))==sorted(dot(e,f) for e in edges(minus) for f in edges(Q)),plus=rplus,minus=rminus,sys_plus=sp,sys_minus=sm,capacity_relative_gap=max(rplus['capacity'],rminus['capacity'])/min(rplus['capacity'],rminus['capacity'])-1,ratio_relative_gap=max(sp,sm)/min(sp,sm)-1)
        trials.append(record)
        if sp!=sm:break
    result=dict(status='EXACT_GAP' if trials and trials[-1]['sys_plus']!=trials[-1]['sys_minus'] else 'NO_GAP',control=dict(capacity=ctrl['capacity'],cyclic_words=ctrl['cyclic_words']),method='All size-2 and size-3 positive normalized closure supports; all cyclic orders; product six-facet reduction gives completeness',trials=trials,elapsed_seconds=time.perf_counter()-start)
    result['script_sha256']=hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
    args.out.mkdir(parents=True,exist_ok=True)
    (args.out/'result.json').write_text(json.dumps(result,default=serial,indent=2)+'\n')
    (args.out/'geometry.json').write_text(json.dumps({k:trials[-1][k] for k in ('P_plus','P_minus','Q')},default=serial,indent=2)+'\n')
    print(json.dumps(dict(status=result['status'],trials=len(trials),elapsed_seconds=result['elapsed_seconds'],last={k:trials[-1][k] for k in ['sys_plus','sys_minus','capacity_relative_gap','ratio_relative_gap']}),default=serial,indent=2))
if __name__=='__main__':main()
