#!/usr/bin/env python3
"""Replay the exact contact and six-ray mixed-polynomial discriminator."""
from fractions import Fraction as F
from functools import reduce
from itertools import combinations, permutations
from math import gcd, prod
from pathlib import Path
import json

DATA = json.loads(Path(__file__).with_name('contact-calculus.json').read_text())
A = DATA['base_A']
U = [tuple(u) for u in DATA['contact_directions']]
ORIG = DATA['original_indices']
RAYS = [[F(x) for x in row] for row in DATA['rays']]
W = [row + [3*(i==j) for j in range(4)] for i,row in enumerate(A)] \
    + [[-3*(i==j) for j in range(4)] + row for i,row in enumerate(A)]
P = A + [[-3*(i==j) for j in range(4)] for i in range(4)]


def dot(a,b): return sum(x*y for x,y in zip(a,b))
def mv(m,v): return [dot(row,v) for row in m]
def omega(u,v): return dot(u,mv(A,v))
def det(m):
    n=len(m)
    return sum((-1)**sum(p[i]>p[j] for i in range(n) for j in range(i+1,n))
               *prod(m[i][p[i]] for i in range(n)) for p in permutations(range(n)))


def pmul(p,q):
    out={}
    for ij,a in p.items():
        for kl,b in q.items():
            e=(ij[0]+kl[0],ij[1]+kl[1])
            out[e]=out.get(e,F(0))+a*b
    return out


def padd(out,p,c):
    for e,a in p.items(): out[e]=out.get(e,F(0))+c*a


def main():
    found=set()
    for rows in combinations(range(8),3):
        u=[(-1)**j*det([[P[i][k] for k in range(4) if k!=j] for i in rows]) for j in range(4)]
        g=reduce(gcd,u)
        if not g: continue
        u=[x//g for x in u]
        if next(x for x in u if x)<0: u=[-x for x in u]
        found.add(tuple(u))
    assert found==set(U) and len(U)==32
    norms=[sum(map(abs,mv(P,u))) for u in U]
    assert norms==[F(x) for x in DATA['a_norm']]
    for u,a in zip(U,norms):
        y=mv(P,u)
        b=sum(abs(W[i][j]*y[k]-W[i][k]*y[j]+W[j][k]*y[i])
              for i,j,k in combinations(range(8),3))
        assert b==16*a
    C=[[F(omega(u,v),norms[i]*norms[j]) for j,v in enumerate(U)] for i,u in enumerate(U)]
    H=[]
    for i,u in enumerate(U):
        pu=mv(P,u); row=[]
        for j,v in enumerate(U):
            pv=mv(P,v);c=omega(u,v)
            e=sum(abs(W[k][l]*c-pu[k]*pv[l]+pv[k]*pu[l]) for k,l in combinations(range(8),2))
            row.append(1+F(32*abs(c)-2*e,norms[i]*norms[j]))
        H.append(row)
    assert H==[[F(x) for x in row] for row in DATA['H']]
    for ray in RAYS:
        assert sum(a*H[i][j]*b for i,a in enumerate(ray) for j,b in enumerate(ray))==0
    assert all(x>=0 for ray in RAYS[:6] for x in ray)
    assert {tuple(record['rays']) for record in DATA['bivariate_six_nonnegative_rays']} == set(combinations(range(6),2))
    for record in DATA['bivariate_six_nonnegative_rays']:
        ra,rb=record['rays'];x,y=RAYS[ra],RAYS[rb]
        indices=sorted(set(ORIG+[i for i in range(32) if x[i] or y[i]]))
        weights={i:{(0,0):F(8) if i in ORIG else F(0),(1,0):x[i],(0,1):y[i]} for i in indices}
        phi={};psi={}
        for i,j in combinations(indices,2):
            padd(phi,pmul(weights[i],weights[j]),abs(C[i][j]))
        for i,j,k,l in combinations(indices,4):
            pf=C[i][j]*C[k][l]-C[i][k]*C[j][l]+C[i][l]*C[j][k]
            padd(psi,pmul(pmul(weights[i],weights[j]),pmul(weights[k],weights[l])),abs(pf))
        gap=pmul(phi,phi);padd(gap,psi,-4);gap={e:v for e,v in gap.items() if v}
        expected={tuple(map(int,key.split(','))):F(value) for key,value in record['gap'].items()}
        assert gap==expected
        assert gap and all(v>0 for v in gap.values())
        assert all(i>0 and j>0 for i,j in gap)
    print('PASS: 32 exact contact directions up to sign; saved second variation; 12 zero quadratic rays.')
    print('PASS: all 15 two-ray deficits have nonnegative coefficients and are positive for s,t>0.')
    print('Scope: these six rays and their pairwise sums only; no all-size inequality or general copositivity claim.')


if __name__=='__main__': main()
