"""Exact ten-triangle regular-pentagon instance and one HKO placement.

This verifies feasibility and the value of the HKO cover, NOT optimality.
Arithmetic: QQ[tau]/(tau^4-10*tau^2+5), with tau the unique root in (7/10,3/4).
Sign tests use rational interval isolation, NOT AlgebraicField.is_positive
(which tests a leading coefficient rather than this ordered real embedding).
"""
from __future__ import annotations
from itertools import combinations, permutations
from functools import lru_cache,cmp_to_key
import json,sys
import sympy as sp
from sympy.polys.matrices import DomainMatrix as DM

F=sp.QQ.algebraic_field(sp.sqrt(5-2*sp.sqrt(5)))
tau=F.unit;Z=F.zero;O=F.one;s5=(F(5)-tau*tau)/F(2)

def dot(a,b):return sum((x*y for x,y in zip(a,b)),Z)
def sub(a,b):return [x-y for x,y in zip(a,b)]
def add(a,b):return [x+y for x,y in zip(a,b)]
def scale(c,a):return [c*x for x in a]
def cross(a,b):return a[0]*b[1]-a[1]*b[0]

@lru_cache(None)
def sign(a):
    if a==Z:return 0
    coeff=list(a.to_list());lo=sp.QQ(7,10);hi=sp.QQ(3,4)
    for _ in range(2048):
        L=H=sp.QQ.zero
        for c in coeff:
            vals=(L*lo,L*hi,H*lo,H*hi);L=min(vals)+c;H=max(vals)+c
        if L>0:return 1
        if H<0:return -1
        mid=(lo+hi)/2;value=mid**4-10*mid**2+5
        if value>0:lo=mid
        else:hi=mid
    raise ArithmeticError('Root isolation budget exceeded')

def solve(rows,rhs):
    n=len(rows)
    return [r[0] for r in (DM(rows,(n,n),F).inv()*DM([[v] for v in rhs],(n,1),F)).to_list()]

def hull(points):
    def cmp(a,b):return sign(a[0]-b[0]) or sign(a[1]-b[1])
    points=sorted(set(tuple(p) for p in points),key=cmp_to_key(cmp));lo=[];hi=[]
    for p in points:
        while len(lo)>1 and sign(cross(sub(lo[-1],lo[-2]),sub(p,lo[-2])))<=0:lo.pop()
        lo.append(p)
    for p in reversed(points):
        while len(hi)>1 and sign(cross(sub(hi[-1],hi[-2]),sub(p,hi[-2])))<=0:hi.pop()
        hi.append(p)
    return lo[:-1]+hi[:-1]

def area(poly):return sum((cross(poly[i],poly[(i+1)%len(poly)]) for i in range(len(poly))),Z)/F(2)
def serialize(a):return [str(q) for q in a.to_list()] # descending tau powers

def make_fixture():
    A=[[O,tau],[-(F(3)-s5)/F(2),tau*(O+s5)/F(2)],[-(s5-O),Z],
       [-(F(3)-s5)/F(2),-tau*(O+s5)/F(2)],[O,-tau]]
    Q=[[O,Z],[(s5-O)/F(4),tau*(F(3)+s5)/F(4)],
       [-(O+s5)/F(4),tau*(O+s5)/F(4)],[-(O+s5)/F(4),-tau*(O+s5)/F(4)],
       [(s5-O)/F(4),-tau*(F(3)+s5)/F(4)]]
    cH=F(5)/(F(2)*tau);rhoH=(F(3)+s5)/F(5)
    K=[[-p[1]/cH,p[0]/cH] for p in Q]
    B=[[-a[1]*cH,a[0]*cH] for a in A]
    shapes=[];supports=[]
    for ids in combinations(range(5),3):
        C=[[A[i][j] for i in ids] for j in range(2)]+[[O]*3]
        if DM(C,(3,3),F).det()==Z:continue
        beta=solve(C,[Z,Z,O])
        if not all(sign(b)>0 for b in beta):continue
        supports.append(ids)
        for order in ((0,1,2),(0,2,1)):
            nodes=[];pos=[Z,Z]
            for k in order:nodes.append(pos);pos=add(pos,scale(beta[k],A[ids[k]]))
            assert pos==[Z,Z]
            center=scale(O/F(3),[sum(p[j] for p in nodes) for j in range(2)])
            nodes=[sub(p,center) for p in nodes]
            shapes.append({'support':list(ids),'order':list(order),'nodes':nodes})
    assert len(shapes)==10 and len(supports)==5
    # No opposite pair of normals, hence no segment shapes.
    assert all(cross(a,b)!=Z for a,b in combinations(A,2))
    placed=[];translations=[]
    for shape in shapes:
        rhs=[]
        for b in B:
            values=[dot(b,p) for p in shape['nodes']];mx=values[0]
            for v in values[1:]:
                if sign(v-mx)>0:mx=v
            rhs.append(O-mx)
        found=None
        for i,j in combinations(range(5),2):
            if cross(B[i],B[j])==Z:continue
            t=solve([B[i],B[j]],[rhs[i],rhs[j]])
            if all(sign(rhs[k]-dot(B[k],t))>=0 for k in range(5)):found=t;break
        if found is None:raise AssertionError('No exact feasible translation')
        translations.append(found);placed.extend([add(p,found) for p in shape['nodes']])
    HK=hull(placed);known=hull(K);aq=area(hull(Q));ak=area(known)
    assert HK==known
    assert O/(F(2)*aq*ak)==rhoH
    out={'arithmetic':'ordered QQ(tau); every sign decided by rational isolating intervals',
      'field_polynomial':'tau^4-10*tau^2+5','root_interval':['7/10','3/4'],
      'coefficient_format':'Each coordinate is an array of rational strings in descending tau powers; [] means 0.',
      'Q_vertices':[[serialize(x) for x in p] for p in Q],
      'known_optimal_candidate_K':[[serialize(x) for x in p] for p in K],
      'Q_area':serialize(aq),'candidate_K_area':serialize(ak),'candidate_rho':serialize(rhoH),
      'capacity_of_product':'1 (HKO theorem after scaling)',
      'normal_shapes':[{'support':s['support'],'order':s['order'],'vertices':[[serialize(x) for x in p] for p in s['nodes']]} for s in shapes],
      'candidate_translations':[[serialize(x) for x in t] for t in translations],
      'verified':['complete ten-shape family','all translated shapes contained in candidate K','convex hull exactly equals candidate K','ratio equals (3+sqrt(5))/5'],
      'NOT_verified':['global lower bound for arbitrary placements','uniqueness of minimizing body','global bound for all products or all four-dimensional bodies']}
    return out

if __name__=='__main__':
    if not __debug__:raise RuntimeError('Run without -O')
    obj=make_fixture()
    if len(sys.argv)!=2:raise SystemExit('usage: python pentagon_fixture.py OUTPUT.json')
    from pathlib import Path
    p=Path(sys.argv[1])
    if p.exists():raise FileExistsError(p)
    p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(obj,indent=2)+'\n')
    print('PASS: exact ten-shape coverage; HKO placement; hull equality and ratio.')
    print('No optimum or uniqueness is asserted.')
