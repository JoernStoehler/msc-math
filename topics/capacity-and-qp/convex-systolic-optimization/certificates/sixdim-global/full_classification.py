"""Exact finite proof of TR6 vertex classification by orbit edge closure.

Run with Sage Python. cdd and PPL independently enumerate tangent rays at
two orbit representatives; rational arithmetic is used throughout.
"""
from itertools import combinations, permutations
from collections import Counter, deque
from sage.all import QQ, ZZ, matrix, vector, Polyhedron
import json, time

t0=time.process_time()
pairs=list(combinations(range(6),2)); ix={p:i for i,p in enumerate(pairs)}
def flat(A):return tuple(A[i,j] for i,j in pairs)
def skew(x):
    M=matrix(QQ,6,6)
    for (i,j),a in zip(pairs,x):M[i,j]=a;M[j,i]=-a
    return M

V=[]
for T in combinations(range(7),3):
    v=[0]*15
    if 6 in T:v[ix[T[0],T[1]]]=1
    else:
        i,j,k=T;v[ix[i,j]]=1;v[ix[i,k]]=-1;v[ix[j,k]]=1
    V.extend((tuple(v),tuple(-a for a in v)))
Vset=set(V)
assert len(V)==70 and len(Vset)==70
assert matrix(QQ,V).rank()==15
g=[vector(ZZ,[-1]*6)]+[vector(ZZ,[int(i==j) for i in range(6)]) for j in range(6)]
generators=[]
for k in range(6):
    perm=list(range(7));perm[k],perm[k+1]=perm[k+1],perm[k]
    G=matrix(ZZ,6,6,[g[perm[j]] for j in range(1,7)]).transpose()
    assert abs(G.det())==1
    assert {flat(G.transpose()*skew(v)*G) for v in V}==Vset
    H=G.inverse()
    generators.append(H)

def orbit(x):
    seen={tuple(x)}; stack=[tuple(x)]
    while stack:
        P=skew(stack.pop())
        for H in generators:
            y=flat(H*P*H.transpose())
            if y not in seen:seen.add(y);stack.append(y)
    return seen

p0=[0]*15
for i in range(3):
    for j in range(3):p0[ix[i,j+3]]=0 if i==j else -1
p0=tuple(p0)
order0=(1,)*15
fence_orbit=orbit(p0)
order_orbit=orbit(order0)
orders=set()
for perm in permutations(range(6)):
    pos={a:k for k,a in enumerate(perm)}
    orders.add(tuple(1 if pos[i]<pos[j] else -1 for i,j in pairs))
assert len(fence_orbit)==840
assert len(order_orbit)==720 and order_orbit==orders
assert matrix(QQ,[vector(QQ,o)-vector(QQ,order0) for o in order_orbit]).rank()==15
assert not order_orbit.intersection(fence_orbit)
assert all(all(sum(a*b for a,b in zip(v,x))<=1 for v in V) for x in fence_orbit|order_orbit)

def local(x):
    x=vector(QQ,x)
    active=[v for v in V if vector(QQ,v).dot_product(x)==1]
    assert matrix(QQ,active).rank()==15
    ieqs=[[0]+[-a for a in v] for v in active]
    cones={b:Polyhedron(ieqs=ieqs,base_ring=QQ,backend=b) for b in ('cdd','ppl')}
    raysets={b:{tuple(r) for r in C.rays()} for b,C in cones.items()}
    # Normalize primitive integer ray, so independent backends compare exactly.
    def primitive(r):
        r=vector(QQ,r)
        den=ZZ(1)
        for a in r:den=den.lcm(a.denominator())
        z=[ZZ(den*a) for a in r]
        content=ZZ(0)
        for a in z:content=content.gcd(a)
        return tuple(a/content for a in z)
    rays={b:{primitive(r) for r in raysets[b]} for b in raysets}
    assert rays['cdd']==rays['ppl']
    assert all(C.n_lines()==0 and C.n_vertices()==1 for C in cones.values())
    endpoints=[]
    for d0 in rays['cdd']:
        d=vector(QQ,d0)
        assert all(vector(QQ,v).dot_product(d)<=0 for v in active)
        caps=[]
        for v in V:
            vd=vector(QQ,v).dot_product(d)
            if vd>0:caps.append((1-vector(QQ,v).dot_product(x))/vd)
        assert caps
        y=tuple(x+min(caps)*d)
        assert y in order_orbit|fence_orbit
        assert all(sum(a*b for a,b in zip(v,y))<=1 for v in V)
        endpoints.append(y)
    assert len(set(endpoints))==len(rays['cdd'])
    return {
        'active_count':len(active), 'active_rank':15,
        'cdd_rays':len(rays['cdd']),'ppl_rays':len(rays['ppl']),
        'order_endpoints':sum(y in order_orbit for y in endpoints),
        'fence_endpoints':sum(y in fence_orbit for y in endpoints),
        'distinct_endpoints':len(set(endpoints)),
        'endpoint_tight_distribution':dict(Counter(sum(sum(a*b for a,b in zip(v,y))==1 for v in V) for y in endpoints)),
    }

result={'fence_orbit':len(fence_orbit),'order_orbit':len(order_orbit),
        'generators_preserve_70_rows':True,'V_rank':15,'order_affine_rank':15,
        'canonical_order':local(order0),
        'canonical_fence':local(p0),'cpu_seconds':time.process_time()-t0}
with open('full_classification.json','w') as f:json.dump(result,f,indent=2)
print(json.dumps(result,indent=2))
