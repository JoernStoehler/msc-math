#!/usr/bin/env python3
"""Bounded orientation screening and one-circuit event preflight."""
from fractions import Fraction as F
from functools import total_ordering
from itertools import product
import json
from pathlib import Path
from itertools import combinations
from math import isqrt

@total_ordering
class K:
    __slots__ = ('a', 'b')
    def __init__(self, a=0, b=0): self.a, self.b = F(a), F(b)
    @staticmethod
    def cast(v): return v if isinstance(v, K) else K(v)
    def __add__(u,v):
        v=K.cast(v); return K(u.a+v.a,u.b+v.b)
    __radd__=__add__
    def __neg__(u): return K(-u.a,-u.b)
    def __sub__(u,v): return u+-K.cast(v)
    def __rsub__(u,v): return K.cast(v)+-u
    def __mul__(u,v):
        v=K.cast(v); return K(u.a*v.a+5*u.b*v.b,u.a*v.b+u.b*v.a)
    __rmul__=__mul__
    def __truediv__(u,v):
        v=K.cast(v); norm=v.a*v.a-5*v.b*v.b
        if not norm: raise ZeroDivisionError
        return u*K(v.a/norm,-v.b/norm)
    def sign(u):
        a,b=u.a,u.b
        if not b: return (a>0)-(a<0)
        if not a: return (b>0)-(b<0)
        if (a>0)==(b>0): return 1 if a>0 else -1
        q=(a*a>5*b*b)-(a*a<5*b*b)
        return q if a>0 else -q
    def __eq__(u,v):
        v=K.cast(v); return u.a==v.a and u.b==v.b
    def __lt__(u,v): return (u-K.cast(v)).sign()<0
    def __hash__(u): return hash((u.a,u.b))
    def data(u): return [str(u.a),str(u.b)]
    def __repr__(u): return f'({u.a})+({u.b})sqrt5'

def source_candidates():
    z=K(0,1); tau2=5-2*z; b=(1+z)/4; w=1+b; R=(5-z)/10
    normals=[(K(1),K(0)),((z-1)/4,(5-z)/4),(-(1+z)/4,(3*z-5)/4),
             (-(1+z)/4,-(3*z-5)/4),((z-1)/4,-(5-z)/4)]
    def rotate(v): return (b*v[0]-(3*z-5)/4*v[1],b*(v[0]+v[1]))
    cur=[(-R,K(0)),(R,K(0)),(K(0),R)]; base=cur; triangles=[]
    for _ in range(10): triangles.append(cur); cur=[rotate(v) for v in cur]
    assert cur==base
    lam0=[K(1)/(2*w),K(1)/(2*w),K(0),b/w,K(0)]
    circuits=[]
    for l in range(5):
        lam=[lam0[(i-l)%5] for i in range(5)]
        active=[i for i,v in enumerate(lam) if v>0]; records={}
        for j,tri in enumerate(triangles):
            for choice in product(range(3),repeat=3):
                coeff=[K(0)]*4
                for i,k in zip(active,choice):
                    nx,ny=normals[i]; vx,vy=tri[k]
                    terms=(ny*vy,nx*vx,tau2*nx*vy,-ny*vx)
                    coeff=[a+lam[i]*c for a,c in zip(coeff,terms)]
                records.setdefault(tuple(coeff),[]).append([j,*choice])
        circuits.append(records)
    return circuits

def at_u(coeff,u):
    c,a,d,b=coeff; return (a+b*u,c+d*u)

def screen(records):
    U=K(0,F(1,5)); coeffs=list(records); eliminated={}; survivors=[]
    for i,s in enumerate(coeffs):
        dominators=[]
        for j,r in enumerate(coeffs):
            if i==j: continue
            diff=tuple(a-b for a,b in zip(r,s))
            ends=[at_u(diff,u) for u in (K(0),U)]
            if all(a>=0 and b>=0 for a,b in ends):
                dominators.append(dict(candidate=j,
                    endpoint_differences=[dict(slope=a.data(),intercept=b.data()) for a,b in ends],
                    boundary_entire_x_ties=[k for k,(a,b) in enumerate(ends) if a==0 and b==0],
                    boundary_x_zero_ties=[k for k,(a,b) in enumerate(ends) if b==0]))
        if dominators: eliminated[i]=dominators
        else: survivors.append(i)
    return coeffs,survivors,eliminated

def fsqrt(q):
    if q<0: return None
    p,r=isqrt(q.numerator),isqrt(q.denominator)
    return F(p,r) if p*p==q.numerator and r*r==q.denominator else None

def ksqrt(q):
    if q<0: return None
    if q==0: return K(0)
    norm=fsqrt(q.a*q.a-5*q.b*q.b)
    if norm is None: return None
    for sign in (1,-1):
        a=fsqrt((q.a+sign*norm)/2)
        if a:
            out=K(a,q.b/(2*a))
            if out*out==q: return out if out>0 else -out
    if q.b==0:
        b=fsqrt(q.a/5)
        if b is not None: return K(0,b)
    return None

def roots(poly):
    """Return exact Qsqrt5 roots; explicitly stop if a quadratic extension is needed."""
    c,b,a=poly
    if a==0:
        return [] if b==0 else [-c/b]
    discr=b*b-4*a*c
    if discr<0: return []
    radical=ksqrt(discr)
    if radical is None: raise ValueError('Quadratic-extension event root requires separate isolation: '+repr(poly))
    return sorted(set([(-b-radical)/(2*a),(-b+radical)/(2*a)]))

def roots_in_strip(poly,U):
    c,b,a=poly
    values=[c,c+b*U+a*U*U]
    if a!=0:
        vertex=-b/(2*a)
        if 0<vertex<U: values.append(c+b*vertex+a*vertex*vertex)
    # Exact extremum exclusion can discard roots outside the strip without
    # adjoining their (potentially nonsquare) quadratic discriminants.
    if all(v>0 for v in values) or all(v<0 for v in values): return []
    if a!=0 and c==0:
        return sorted({r for r in [K(0),-b/a] if 0<=r<=U})
    if a!=0 and values[1]==0:
        return sorted({r for r in [U,c/(a*U)] if 0<=r<=U})
    return [r for r in roots(poly) if 0<=r<=U]

def product_linear(v,w):
    return [v[0]*w[0],v[0]*w[1]+v[1]*w[0],v[1]*w[1]]

def exact_hull(coeffs,u):
    # At a fixed exact orientation, the whole x>0 upper hull and all ties.
    lines={}
    for i,c in coeffs.items():
        a,b=at_u(c,u)
        if a not in lines or b>lines[a][0]: lines[a]=(b,[i])
        elif b==lines[a][0]: lines[a][1].append(i)
    hull=[]
    for a,(b,ids) in sorted(lines.items()):
        start=None
        while hull:
            pa,pb,_,pstart=hull[-1]; start=(pb-b)/(a-pa)
            if pstart is None or start>pstart: break
            hull.pop()
        if not hull: start=None
        hull.append((a,b,ids,start))
    cells=[]
    for k,(a,b,ids,start) in enumerate(hull):
        end=hull[k+1][3] if k+1<len(hull) else None
        if end is not None and end<=0: continue
        left=K(0) if start is None or start<0 else start
        cells.append(dict(left=left.data(),right=None if end is None else end.data(),active=ids))
    ties=[]
    for x in sorted({K(0)}|{K(*c['left']) for c in cells}):
        value=max(a*x+b for a,(b,_) in lines.items())
        ids=[i for i,c in coeffs.items() if sum(v*w for v,w in zip(at_u(c,u),(x,K(1))))==value]
        ties.append(dict(x=x.data(),value=value.data(),active=ids))
    maxslope=max(a for a,_ in lines.items())
    return dict(cells=cells,ties=ties,infinite_tail=cells[-1]['active'],
        asymptotic_max_slope_candidates=[i for i,c in coeffs.items() if at_u(c,u)[0]==maxslope])

def circuit_zero_events(circuit):
    U=K(0,F(1,5)); ids=circuit['survivors']
    coeffs={i:tuple(K(*v) for v in circuit['candidates'][i]['coefficients']) for i in ids}
    events={K(0):[],U:[]}; polynomial_records=[]
    for i,j in combinations(ids,2):
        diff=tuple(a-b for a,b in zip(coeffs[i],coeffs[j]))
        for kind,poly in [('slope',[diff[1],diff[3],K(0)]),('intercept',[diff[0],diff[2],K(0)])]:
            rr=roots_in_strip(poly,U)
            polynomial_records.append(dict(kind=kind,ids=[i,j],coefficients=[c.data() for c in poly],roots=[r.data() for r in rr]))
            for r in rr: events.setdefault(r,[]).append([kind,i,j])
    for i,j,k in combinations(ids,3):
        ij=tuple(a-b for a,b in zip(coeffs[i],coeffs[j])); ik=tuple(a-b for a,b in zip(coeffs[i],coeffs[k]))
        p=product_linear((ij[1],ij[3]),(ik[0],ik[2])); q=product_linear((ik[1],ik[3]),(ij[0],ij[2]))
        poly=[a-b for a,b in zip(p,q)]
        rr=roots_in_strip(poly,U)
        polynomial_records.append(dict(kind='triple',ids=[i,j,k],coefficients=[c.data() for c in poly],roots=[r.data() for r in rr],identically_zero=all(c==0 for c in poly)))
        for r in rr: events.setdefault(r,[]).append(['triple',i,j,k])
    ordered=sorted(events); patches=[]
    raw={i:tuple(K(*v) for v in c['coefficients']) for i,c in enumerate(circuit['candidates'])}
    for left,right in zip(ordered,ordered[1:]):
        sample=(left+right)/2
        hull=exact_hull(raw,sample)
        boundaries=[]
        for first,second in zip(hull['cells'],hull['cells'][1:]):
            i,j=first['active'][0],second['active'][0]
            p,q=raw[i],raw[j]
            boundaries.append(dict(adjacent_candidates=[i,j],
                numerator=[(p[0]-q[0]).data(),(p[2]-q[2]).data()],
                denominator=[(q[1]-p[1]).data(),(q[3]-p[3]).data()]))
        patches.append(dict(left=left.data(),right=right.data(),sample=sample.data(),hull=hull,
             parametric_x_boundaries=boundaries))
    # Raw candidates restore every isolated or boundary-only tie after screening.
    endpoints=[dict(u=u.data(),causes=events[u],hull=exact_hull(raw,u)) for u in ordered]
    return dict(circuit=0,event_root_field='All in-strip event roots lie in Q(sqrt(5)); outside-strip nonsquare roots excluded by exact extrema',
        event_polynomial_count=len(polynomial_records),polynomials=polynomial_records,
        event_orientation_count=len(ordered),open_patches=patches,event_orientations=endpoints,
        complete_one_circuit=True)

def main():
    import time
    started=time.perf_counter(); circuits=source_candidates()
    out=dict(field='Q(sqrt(5))',encoding='[a,b]=a+b sqrt(5); rational strings',
        variable_order=['1','x','u','xu'],domain='x>0, 0<=u<=1/sqrt(5)',
        source='Local copy of regular fixed-Q coordinates, ten original templates and five closure rays',
        circuits=[])
    for l,records in enumerate(circuits):
        coeffs,survivors,eliminated=screen(records)
        out['circuits'].append(dict(circuit=l,raw_count=sum(map(len,records.values())),
            distinct_count=len(coeffs),survivor_count=len(survivors),survivors=survivors,
            candidates=[dict(coefficients=[v.data() for v in c],sources=records[c]) for c in coeffs],
            eliminated=eliminated))
    out['screen_seconds']=round(time.perf_counter()-started,3)
    out['circuit_zero']=circuit_zero_events(out['circuits'][0])
    out['total_seconds']=round(time.perf_counter()-started,3)
    Path(__file__).with_name('active-events.json').write_text(json.dumps(out,indent=2)+'\n')
    print(json.dumps(dict(distinct=[v['distinct_count'] for v in out['circuits']],
       survivors=[v['survivor_count'] for v in out['circuits']],seconds=out['total_seconds'],
       circuit_zero_event_orientations=out['circuit_zero']['event_orientation_count'])))

if __name__=='__main__': main()
