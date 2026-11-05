#!/usr/bin/env python3
"""Independent Fraction geometry, closure RREF and exhaustive HK check.
No construction/optimization module is imported. Standard library only.
"""
import argparse
from collections import Counter
from fractions import Fraction as F
from itertools import combinations, permutations
import json
from pathlib import Path


def det(a,b):
    return a[0]*b[1]-a[1]*b[0]


def dot(a,b):
    return sum(x*y for x,y in zip(a,b))


def sub(a,b):
    return tuple(x-y for x,y in zip(a,b))


def geometry(v):
    assert len(set(v))==len(v)
    es=[sub(v[(i+1)%len(v)],x) for i,x in enumerate(v)]
    assert all(det(es[i],es[(i+1)%len(es)])>0 for i in range(len(es)))
    area=sum(det(x,v[(i+1)%len(v)]) for i,x in enumerate(v))/2
    assert area>0
    ns=[]
    for i,(x,e) in enumerate(zip(v,es)):
        n=(e[1],-e[0]);h=dot(n,x);assert h>0
        n=tuple(y/h for y in n)
        eq=[j for j,y in enumerate(v) if dot(n,y)==1]
        assert set(eq)=={i,(i+1)%len(v)}
        assert all(dot(n,y)<=1 for y in v)
        ns.append(n)
    return es,area,ns



def hull(points):
    points=sorted(set(points))
    def chain(seq):
        acc=[]
        for p in seq:
            while len(acc)>1 and det(sub(acc[-1],acc[-2]),sub(p,acc[-1]))<=0:
                acc.pop()
            acc.append(p)
        return acc
    return chain(points)[:-1]+chain(reversed(points))[:-1]


def difference_body(poly):
    return hull(sub(x,y) for x in poly for y in poly)


def closure_vertices(ns):
    out=[]
    for size in (2,3):
        for idx in combinations(range(len(ns)),size):
            a=[[F(1) for _ in idx]+[F(1)],
               [ns[i][0] for i in idx]+[F(0)],
               [ns[i][1] for i in idx]+[F(0)]]
            row=0;pivots=[]
            for col in range(size):
                pivot=next((r for r in range(row,3) if a[r][col]),None)
                if pivot is None:continue
                a[row],a[pivot]=a[pivot],a[row]
                z=a[row][col];a[row]=[x/z for x in a[row]]
                for r in range(3):
                    if r!=row:
                        z=a[r][col];a[r]=[x-z*y for x,y in zip(a[r],a[row])]
                pivots.append(col);row+=1
            if len(pivots)!=size or any(all(z==0 for z in r[:-1]) and r[-1] for r in a):continue
            weights=[a[r][-1] for r in range(size)]
            if min(weights)<=0:continue
            assert sum(weights)==1 and all(sum(weights[j]*ns[i][k] for j,i in enumerate(idx))==0 for k in range(2))
            out.append((idx,weights))
    assert out
    return out


def capacity(ns,ms):
    left,right=closure_vertices(ns),closure_vertices(ms)
    best=F(0);count=0;witness=None
    for ii,aa in left:
        for jj,bb in right:
            nodes=[(ns[i][0],ns[i][1],F(0),F(0)) for i in ii]+[(F(0),F(0),ms[j][0],ms[j][1]) for j in jj]
            beta=[x/2 for x in aa+bb]
            def omega(x,y):
                return x[0]*y[2]+x[1]*y[3]-x[2]*y[0]-x[3]*y[1]
            terms={(i,j):beta[i]*beta[j]*omega(x,y) for i,x in enumerate(nodes) for j,y in enumerate(nodes)}
            for tail in permutations(range(1,len(nodes))):
                word=(0,)+tail
                Q=sum(terms[i,j] for a,i in enumerate(word) for j in word[a+1:]);count+=1
                if Q>best:
                    best=Q;witness={'left_support':ii,'right_support':jj,'word':word,'weights':list(map(str,beta)),'Q':str(Q)}
    assert best>0
    return {'capacity':str(1/(2*best)),'Qmax':str(best),'word_evaluations':count,'closure_vertex_counts':[len(left),len(right)],'closure_support_sizes':[sorted(Counter(len(x[0]) for x in vv).items()) for vv in (left,right)],'maximizer':witness}


def check(plus,minus,q):
    p1,a1,n1=geometry(plus);p2,a2,n2=geometry(minus);qe,aq,nq=geometry(q)
    assert a1==a2
    assert difference_body(plus)==difference_body(minus)
    def ridges(es):
        # P x vertex(Q), vertex(P) x Q, and mixed edge products.
        return sorted([abs(dot(e,f)) for e in es for f in qe]+[F(0)]*(len(q)+len(es)))
    r1,r2=ridges(p1),ridges(p2);assert r1==r2
    c1,c2=capacity(n1,nq),capacity(n2,nq)
    C1,C2=F(c1['capacity']),F(c2['capacity']);V=a1*aq
    assert C1!=C2,'No capacity separation'
    return {'status':'PASS','difference_bodies_equal':True,'factor_areas':list(map(str,(a1,a2,aq))),'common_volume':str(V),'ridge_count':len(r1),'common_unsigned_ridge_areas':list(map(str,r1)),'plus':c1,'minus':c2,'sys_plus':str(C1*C1/(2*V)),'sys_minus':str(C2*C2/(2*V)),'sys_quotient':str(max(C1,C2)**2/min(C1,C2)**2),'scope':'Full exact 2/3-support enumeration with all cyclic permutations; completeness imports accepted planar product six-facet theorem, not physical reconstruction or formal proof-assistant verification.'}


def main():
    ap=argparse.ArgumentParser();ap.add_argument('--input',type=Path,required=True);ap.add_argument('--out',type=Path,required=True);args=ap.parse_args()
    d=json.loads(args.input.read_text())
    polys=[tuple(tuple(F(z) for z in v) for v in d[k]) for k in ('P_plus','P_minus','Q')]
    result=check(*polys)
    args.out.write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps({k:result[k] for k in ('status','factor_areas','common_volume','ridge_count','sys_plus','sys_minus','sys_quotient')},indent=2))

if __name__=='__main__':main()
