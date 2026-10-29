"""Exact fence score predicates implying C6 pyramid coverage.

Depends on full_classification.py, whose exact orbit/edge closure theorem gives
Q6 vertices = 720 orders + 840 fence normals.
"""
import full_classification as C
from collections import Counter
from sage.all import QQ, ZZ, matrix, vector, Polyhedron
import json, time

t0=time.process_time()
def form_orbit(x):
    seen={tuple(x)}; stack=[tuple(x)]
    while stack:
        A=C.skew(stack.pop())
        for H in C.generators:
            G=H.inverse()
            y=C.flat(G.transpose()*A*G)
            if y not in seen:seen.add(y);stack.append(y)
    return seen

Fmat=matrix(ZZ,6,6)
for i in range(3):
    for j in range(3):
        Fmat[i,j+3]=1 if i==j else -1
        Fmat[j+3,i]=-Fmat[i,j+3]
F0=C.flat(Fmat)
W=form_orbit(F0)
assert len(W)==840
def dot(a,b):return sum(x*y for x,y in zip(a,b))
V_orbit=form_orbit(C.V[0])
assert V_orbit==set(C.V)
V0_tight_orders=[o for o in C.order_orbit if dot(C.V[0],o)==1]
assert len(V0_tight_orders)==360
V0_tight_affine_rank=matrix(QQ,[vector(QQ,o)-vector(QQ,V0_tight_orders[0]) for o in V0_tight_orders[1:]]).rank()
assert V0_tight_affine_rank==14
order_scores=Counter(dot(F0,o) for o in C.order_orbit)
fence_scores=Counter(dot(w,C.p0) for w in W)
assert max(order_scores)==5
assert fence_scores[6]==1
assert max(k for k in fence_scores if k!=6)<=4

# The unique fence beyond p0 is the displayed F0; its base is the 18-vertex
# facet of D6 dual to p0. Verify affine rank 14 and exact vertex figure scores.
active=[v for v in C.V if dot(v,C.p0)==1]
assert len(active)==18
base_rank=matrix(QQ,[vector(QQ,v)-vector(QQ,active[0]) for v in active[1:]]).rank()
assert base_rank==14
p=vector(QQ,C.p0)
cone=Polyhedron(ieqs=[[0]+[-a for a in v] for v in active],base_ring=QQ,backend='cdd')
neighbors=[]
for ray in cone.rays():
    d=vector(QQ,ray)
    caps=[(1-vector(QQ,v).dot_product(p))/vector(QQ,v).dot_product(d)
          for v in C.V if vector(QQ,v).dot_product(d)>0]
    y=tuple(p+min(caps)*d)
    assert y in C.order_orbit
    neighbors.append(y)
assert len(neighbors)==18 and len(set(neighbors))==18
neighbor_scores=Counter(dot(F0,y) for y in neighbors)
assert neighbor_scores=={5:18}
tight_orders=[o for o in C.order_orbit if dot(F0,o)==5]
assert len(tight_orders)==18
tight_order_affine_rank=matrix(QQ,[vector(QQ,o)-vector(QQ,tight_orders[0]) for o in tight_orders[1:]]).rank()
assert tight_order_affine_rank==14

result={
    'V70_form_orbit_size':len(V_orbit),
    'V0_tight_order_count':len(V0_tight_orders),
    'V0_tight_order_affine_rank':V0_tight_affine_rank,
    'fence_orbit_size':len(W),
    'canonical_fence_order_score_distribution_5times':{str(k):v for k,v in sorted(order_scores.items())},
    'canonical_p0_fence_score_distribution_5times':{str(k):v for k,v in sorted(fence_scores.items())},
    'p0_unique_violating_fence_score':'6/5',
    'p0_other_fence_max_score':'4/5',
    'p0_D6_facet_vertex_count':len(active),
    'p0_D6_facet_affine_rank':base_rank,
    'p0_neighbor_order_count':len(neighbors),
    'F0_tight_order_affine_rank':tight_order_affine_rank,
    'canonical_fence_scores_on_p0_neighbors_5times':{str(k):v for k,v in neighbor_scores.items()},
    'cpu_seconds_excluding_import':time.process_time()-t0,
}
with open('dual_coverage.json','w') as f:json.dump(result,f,indent=2)
print(json.dumps(result,indent=2))
