# /// script
# dependencies = ["sympy", "gmpy2"]
# ///
"""Exhaustive exact/interval edge certificate for the affine 5 by 7 optimum.

No floating point decisions or retained discovery data are used.  Arithmetic
is exact in Q(cos(2*pi/5),cos(2*pi/7)); signs are certified by the existing
integer interval trig implementation.  Abort on every undecided comparison.
"""
import importlib.util
import itertools
import json
import time
from fractions import Fraction
from functools import lru_cache
from pathlib import Path
import sympy as sp

HERE = Path(__file__).resolve().parent
helper = HERE.parent/'tensor-certificate-20261022/check_certificate.py'
spec = importlib.util.spec_from_file_location('interval_helper', helper)
c = importlib.util.module_from_spec(spec)
spec.loader.exec_module(c)
I, SCALE = c.I, c.S
if not __debug__:
    raise RuntimeError('Run without -O; assertions are part of this checker.')

# The chosen real embedding, with primitive element t=a+b.
aa, bb = sp.cos(2*sp.pi/5), sp.cos(2*sp.pi/7)
K = sp.QQ.algebraic_field(aa, bb)
a, b = K.from_sympy(aa), K.from_sympy(bb)
Z, ONE = K.zero, K.one
assert 4*a*a+2*a-ONE == Z
assert 8*b*b*b+4*b*b-4*b-ONE == Z
ti = c.trig(2,5,'cos')+c.trig(2,7,'cos')

@lru_cache(None)
def enc(x):
    """Enclose exact ANP element at the selected real embedding."""
    answer = I.c(0)
    for q in x.to_list():
        answer = answer*ti + I.c(Fraction(int(q.numerator),int(q.denominator)))
    return answer

def sign(x):
    if x == Z:
        return 0
    y = enc(x)
    assert y.positive() or y.negative(), ('undecided field sign',x,y)
    return 1 if y.positive() else -1

def dot(x,y):
    return sum((p*q for p,q in zip(x,y)),Z)

def idot(x,y):
    return sum((p*q for p,q in zip(x,y)),I.c(0))

def cross(x,y):
    return (x[1]*y[2]-x[2]*y[1],x[2]*y[0]-x[0]*y[2],x[0]*y[1]-x[1]*y[0])

def normals(n, root):
    co, si = [ONE,root], [Z,ONE]
    for k in range(2,n):
        co.append(2*root*co[-1]-co[-2])
        si.append(2*root*si[-1]-si[-2])
    return list(zip(co,si))

N5,N7 = normals(5,a),normals(7,b)
LABELS = [(i,j,e) for i in range(5) for j in range(7) for e in (1,-1)]
ROWS = [tuple(e*p*q for p in N5[i] for q in N7[j]) for i,j,e in LABELS]
# Variables are X=(M00, sin(2*pi/7) M01, sin(2*pi/5) M10,
#                  sin(2*pi/5) sin(2*pi/7) M11).
FIXED = LABELS.index((0,0,1))
assert ROWS[FIXED] == (ONE,Z,Z,Z)
C = [r[1:] for r in ROWS]
D = [ONE-r[0] for r in ROWS]
CI = [tuple(enc(v) for v in row) for row in C]
DI = [enc(v) for v in D]

def exact_origin(u,v,direction):
    """Particular solution of two affine equations, with one coordinate zero."""
    p = next(i for i in range(3) if direction[i] != Z)
    q,r = [i for i in range(3) if i != p]
    denominator = C[u][q]*C[v][r]-C[u][r]*C[v][q]
    assert denominator != Z
    out = [Z,Z,Z]
    out[q] = (D[u]*C[v][r]-D[v]*C[u][r])/denominator
    out[r] = (C[u][q]*D[v]-C[v][q]*D[u])/denominator
    assert dot(C[u],out) == D[u] and dot(C[v],out) == D[v]
    return tuple(out)

@lru_cache(None)
def vertex(active):
    """Exact coordinates from three nonfixed contacts."""
    u,v,w = active
    direction = cross(C[u],C[v])
    origin = exact_origin(u,v,direction)
    denom = dot(C[w],direction)
    assert denom != Z
    t = (D[w]-dot(C[w],origin))/denom
    out = tuple(origin[i]+t*direction[i] for i in range(3))
    assert all(dot(C[k],out) == D[k] for k in active)
    # Active constraints are exact equalities; all others checked by sign.
    assert all(sign(D[k]-dot(C[k],out)) >= 0 for k in range(70))
    return out

def det(x):
    return x[2]-x[0]*x[1]  # positive constant times physical determinant

def as_json(x):
    return [[int(q.numerator),int(q.denominator)] for q in x.to_list()]

def solve_contacts(labels):
    """General exact 4 by 4 system in the scaled matrix coordinates."""
    A = [list(ROWS[LABELS.index(tuple(label))])+[ONE] for label in labels]
    for p in range(4):
        k = next(k for k in range(p,4) if A[k][p] != Z)
        A[p],A[k] = A[k],A[p]
        pivot = A[p][p]
        A[p] = [v/pivot for v in A[p]]
        for k in range(4):
            if k != p:
                multiplier = A[k][p]
                A[k] = [v-multiplier*w for v,w in zip(A[k],A[p])]
    return [r[-1] for r in A]

WITNESS = [(0,3,1),(3,0,1),(1,1,-1),(4,5,-1)]
wm = solve_contacts(WITNESS)
assert all(sign(ONE-dot(r,wm)) >= 0 for r in ROWS)
OPT = wm[0]*wm[3]-wm[1]*wm[2]
assert sign(OPT) > 0
REPRESENTATIVE = [(0,0,1),(1,5,-1),(2,3,1),(3,1,-1)]
representative = solve_contacts(REPRESENTATIVE)
assert representative[0] == ONE
assert all(sign(ONE-dot(r,representative)) >= 0 for r in ROWS)
assert representative[3]-representative[1]*representative[2] == OPT

def run():
    start=time.monotonic()
    count = dict(pairs=0,rank_deficient=0,inconsistent=0,empty=0,point=0,edges=0,
                 convex=0,affine=0,concave=0,stationary=0)
    edges=[]
    maximizing_vertices={}
    nonfixed=[k for k in range(70) if k != FIXED]
    for u,v in itertools.combinations(nonfixed,2):
        count['pairs']+=1
        direction = cross(C[u],C[v])
        if all(x == Z for x in direction):
            count['rank_deficient']+=1
            continue
        origin = exact_origin(u,v,direction)
        oi,ni = [enc(x) for x in origin],[enc(x) for x in direction]
        lower,upper=[],[]
        inconsistent=False
        for k in nonfixed:
            if k in (u,v):
                continue
            slopei = idot(CI[k],ni)
            residuali = DI[k]-idot(CI[k],oi)
            if slopei.positive():
                upper.append((k,residuali/slopei))
            elif slopei.negative():
                lower.append((k,residuali/slopei))
            else:
                slope = dot(C[k],direction)
                assert slope == Z, ('unresolved slope',u,v,k)
                if residuali.negative():
                    inconsistent=True
                    break
                if not residuali.positive():
                    residual=D[k]-dot(C[k],origin)
                    assert sign(residual) >= 0
        if inconsistent:
            count['inconsistent']+=1
            continue
        assert lower and upper, ('unbounded line',u,v)
        # Rational interval comparison proves strict emptiness when possible.
        lo=max(lower,key=lambda p:p[1].lo)
        hi=min(upper,key=lambda p:p[1].hi)
        if lo[1].lo > hi[1].hi:
            count['empty']+=1
            continue
        # This branch contains only the genuine edge lines. Establish the exact
        # endpoint selectors against every competing lower/upper bound.
        def parameter(k):
            return (D[k]-dot(C[k],origin))/dot(C[k],direction)
        lt,ht=parameter(lo[0]),parameter(hi[0])
        for k,interval in lower:
            if k != lo[0] and interval.hi >= lo[1].lo:
                assert sign(lt-parameter(k)) >= 0
        for k,interval in upper:
            if k != hi[0] and interval.lo <= hi[1].hi:
                assert sign(parameter(k)-ht) >= 0
        order=sign(ht-lt)
        if order < 0:
            count['empty']+=1
            continue
        if order == 0:
            count['point']+=1
        A=vertex(tuple(sorted((u,v,lo[0]))))
        B=vertex(tuple(sorted((u,v,hi[0]))))
        assert all(A[i] == origin[i]+lt*direction[i] for i in range(3))
        assert all(B[i] == origin[i]+ht*direction[i] for i in range(3))
        da,db=det(A),det(B)
        assert sign(OPT-da) >= 0 and sign(OPT-db) >= 0
        for coords,value in ((A,da),(B,db)):
            if value == OPT:
                active = tuple(LABELS[k] for k in range(70)
                               if D[k]-dot(C[k],coords) == Z)
                maximizing_vertices[active]=coords
        delta=tuple(y-x for x,y in zip(A,B))
        qa=-delta[0]*delta[1]
        qb=delta[2]-A[0]*delta[1]-delta[0]*A[1]
        curvature=sign(qa)
        # No edge is constantly equal to the optimum. This also excludes
        # an optimal two-face plateau via its boundary edges.
        assert not (da == OPT and db == OPT and qa == Z)
        count[{-1:'concave',0:'affine',1:'convex'}[curvature]]+=1
        stationary=None
        if curvature < 0:
            t=-qb/(2*qa)
            if sign(t)>0 and sign(ONE-t)>0:
                count['stationary']+=1
                value=da+qb*t+qa*t*t
                assert sign(OPT-value)>0
                stationary=as_json(value)
        count['edges']+=1
        edges.append(dict(contacts=[LABELS[k] for k in (FIXED,u,v)],
                          endpoints=[lo[0],hi[0]],curvature=curvature,
                          equal_endpoints=[sign(OPT-da)==0,sign(OPT-db)==0],
                          stationary_value=stationary))
    assert count['pairs']==2346
    # Moving each of the representative's four contacts to (0,0,+) gives
    # precisely the four maximizing vertices in this facet. The action is
    # M -> e R_(-2*pi*i/5) M R_(2*pi*j/7).
    normalized_orbit = {
        tuple(sorted(((k-i)%5,(l-j)%7,f*e)
                     for k,l,f in REPRESENTATIVE))
        for i,j,e in REPRESENTATIVE
    }
    found = {tuple(sorted(active)) for active in maximizing_vertices}
    assert found == normalized_orbit
    assert len(found) == 4
    assert all(len(active)==4 for active in found)
    out=dict(counts=count,seconds=time.monotonic()-start,
             primitive_polynomial=[str(q) for q in K.mod.to_list()],
             optimum_scaled=as_json(OPT),witness=WITNESS,
             representative=REPRESENTATIVE,
             maximizing_contact_sets=[list(active) for active in sorted(found)],
             edges=edges)
    physical=enc(OPT)/(c.trig(2,5,'sin')*c.trig(2,7,'sin'))
    out['physical_determinant_interval']=[physical.lo,physical.hi,SCALE]
    print(json.dumps(out,indent=2))

if __name__=='__main__':
    run()
