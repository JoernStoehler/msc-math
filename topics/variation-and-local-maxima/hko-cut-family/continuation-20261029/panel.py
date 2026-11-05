#!/usr/bin/env python3
"""Continue the retained HKO seven/eight-label words at rational t values.

The KKT solve is exact in Q(tau); volume is independently evaluated by Sage's
AA polyhedron backend. These are feasible-word capacity bounds only.
"""
from sage.all import *
import json
from itertools import combinations

P = PolynomialRing(QQ, 'z'); z = P.gen()
K = NumberField(z**4 - 10*z**2 + 5, 'tau',
                embedding=AA.polynomial_root(z**4-10*z**2+5, RIF('0.72','0.73')))
tau = K.gen(); sq5 = (5-tau**2)/2
alpha=(3-sq5)/2; beta0=tau*(1+sq5)/2
J=matrix(K, [[0,0,1,0],[0,0,0,1],[-1,0,0,0],[0,-1,0,0]])
N0=[vector(K,[1,tau,0,0]),vector(K,[-alpha,beta0,0,0]),vector(K,[-(sq5-1),0,0,0]),vector(K,[-alpha,-beta0,0,0]),vector(K,[1,-tau,0,0]),vector(K,[0,0,tau,-1]),vector(K,[0,0,beta0,alpha]),vector(K,[0,0,0,sq5-1]),vector(K,[0,0,-beta0,alpha]),vector(K,[0,0,-tau,-1])]
def drow(i,n):
    if i<5: return vector(K,[-3*n[0]/5,-3*n[1]/5,-n[1]/(sq5-1),n[0]/(sq5-1)])
    return vector(K,[n[3]/(sq5-1),-n[2]/(sq5-1),-3*n[2]/5,-3*n[3]/5])
def rows(t): return N0+[N0[i]+K(t)*drow(i,N0[i]) for i in range(10)]
def solve(t,word):
    ns=rows(t); m=len(word); A=matrix(K,m,m,lambda i,j: (ns[word[i]]*J*ns[word[j]]) if i<j else ((ns[word[j]]*J*ns[word[i]]) if j<i else 0))
    C=matrix(K,5,m,lambda i,j: (ns[word[j]][i] if i<4 else 1))
    B=block_matrix(K,[[A,C.transpose()],[C,zero_matrix(K,5,5)]])
    rhs=vector(K,[0]*m+[0,0,0,0,1]); sol=B.solve_right(rhs); w=sol[:m]
    q=(w*A*w)/2
    assert C*w==vector(K,[0,0,0,0,1])
    return ns,w,q
def run(t,word):
    ns,w,q=solve(t,word); ta=AA(tau); qa=AA(q); vals=[AA(x) for x in w]
    # Exact vertex enumeration and Lawrence tangent-cone volume, as in the
    # frozen exact_geometry.py; unlike Polyhedron.volume this is quick in K.
    verts={}
    for ids in combinations(range(20),4):
        M=matrix(K,[ns[i] for i in ids])
        if M.det()==0: continue
        v=M.solve_right(vector(K,[1]*4)); sl=[1-(ns[i]*v) for i in range(20)]
        if any(AA(x)<0 for x in sl): continue
        active=tuple(i for i,x in enumerate(sl) if x==0)
        if len(active)!=4: raise AssertionError(('nonsimple feasible vertex',t,ids,active))
        verts[active]=v
    assert set().union(*(set(ids) for ids in verts))==set(range(20))
    vols=[]
    for ell in [vector(K,[1,2,4,8]),vector(K,[1,3,9,27])]:
        total=K(0)
        for ids,v in verts.items():
            M=matrix(K,[ns[i] for i in ids]); det=M.det(); det=det if AA(det)>0 else -det
            slopes=ell*M.inverse(); assert all(x!=0 for x in slopes)
            total+=(ell*v)**4/(24*det*prod(slopes))
        vols.append(total)
    assert vols[0]==vols[1] and min(vals)>0
    vol=AA(vols[0]); assert vol>0
    return {'t':str(t),'word':word,'weights_exact':[str(x) for x in w], 'weights':[float(x) for x in vals], 'min_weight':float(min(vals)), 'closure_normalization_exact':[str(x) for x in matrix(K,4,len(word),lambda i,j:ns[word[j]][i])*w], 'Q_exact':str(q),'Q':float(qa),'volume_exact':str(vols[0]),'volume':float(vol),'rho_feasible_upper':float(1/(8*vol*qa**2)),'vertices':len(verts),'facets_genuine':True}
panel=[(QQ(7)/20,[0,1,6,7,3,12,9]),(QQ(2)/5,[0,1,6,7,3,12,9]),(QQ(9)/20,[0,1,6,7,3,12,9]),(QQ(7)/20,[0,6,5,2,19,11,8,4]),(QQ(2)/5,[0,6,5,2,19,11,8,4]),(QQ(9)/20,[0,6,5,2,19,11,8,4])]
out=[run(t,w) for t,w in panel]
print(json.dumps(out,indent=2))
