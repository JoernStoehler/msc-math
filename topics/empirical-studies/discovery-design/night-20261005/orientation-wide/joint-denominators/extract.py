#!/usr/bin/env python3
"""Complete joint denominator extraction from the accepted screened catalogue.

Standard library only. Writes only sibling joint-denominators.json.
All real algebraic calculations use one isolated quadratic extension of
Q(sqrt(5)) at a time. This is not a body optimizer or positivity certificate.
"""
from fractions import Fraction as F
from functools import total_ordering
from itertools import product
import json
from pathlib import Path
from itertools import combinations
import hashlib
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

class EventRoot:
    """An irreducible quadratic root over F with its exact real embedding."""
    def __init__(self,poly,left,right):
        c,b,a=poly; self.c=c/a; self.b=b/a
        self.left=left; self.right=right; self.lsign=self.value(left).sign()
        self.branch=0 if right<=-self.b/2 else 1
    def value(self,x): return x*x+self.b*x+self.c
    def refine(self):
        m=(self.left+self.right)/2
        if self.value(m).sign()==self.lsign: self.left=m
        else: self.right=m
    def compare_field(self,v):
        # Irreducibility means equality with a field element is impossible.
        assert self.value(v)!=0
        while self.left<=v<=self.right: self.refine()
        return 1 if v<self.left else -1
    def __hash__(self): return hash((self.c,self.b,self.branch))
    def __eq__(self,v): return isinstance(v,EventRoot) and self.c==v.c and self.b==v.b and self.branch==v.branch
    def __lt__(self,v):
        if not isinstance(v,EventRoot): return self.compare_field(K.cast(v))<0
        if self==v:return False
        while not (self.right<v.left or v.right<self.left): self.refine();v.refine()
        return self.right<v.left
    def __le__(self,v): return self==v or self<v
    def __gt__(self,v): return not self<=v
    def __ge__(self,v): return not self<v
    def data(self): return dict(kind='quadratic_event_root',monic_coefficients=[self.c.data(),self.b.data(),K(1).data()],branch=self.branch,left=self.left.data(),right=self.right.data())
    def expression(self):return FieldValue(self,K(0),K(1))
    def __mul__(self,v):return self.expression()*v
    __rmul__=__mul__

def field_value(root,p,q): return p if q==0 else FieldValue(root,p,q)

class FieldValue:
    def __init__(self,root,p,q):self.root,self.p,self.q=root,p,q
    def parts(self,v):
        if isinstance(v,EventRoot):v=v.expression()
        if isinstance(v,FieldValue):
            assert self.root==v.root,'No global compositum arithmetic is permitted'
            return v.p,v.q
        return K.cast(v),K(0)
    def __add__(self,v):
        p,q=self.parts(v);return field_value(self.root,self.p+p,self.q+q)
    __radd__=__add__
    def __neg__(self):return FieldValue(self.root,-self.p,-self.q)
    def __sub__(self,v):return self+-v
    def __rsub__(self,v):return -self+v
    def __mul__(self,v):
        p,q=self.parts(v)
        return field_value(self.root,self.p*p-self.q*q*self.root.c,self.p*q+self.q*p-self.q*q*self.root.b)
    __rmul__=__mul__
    def __truediv__(self,v):
        p,q=self.parts(v)
        # conjugate p+q*alpha is p-q*b-q*alpha
        norm=p*p-p*q*self.root.b+q*q*self.root.c
        assert norm!=0
        return self*FieldValue(self.root,(p-q*self.root.b)/norm,-q/norm)
    def __rtruediv__(self,v):return FieldValue(self.root,K.cast(v),K(0))/self
    def sign(self):return self.q.sign()*self.root.compare_field(-self.p/self.q) if self.q!=0 else self.p.sign()
    def __eq__(self,v):
        if isinstance(v,FieldValue):return self.root==v.root and self.p==v.p and self.q==v.q
        return False
    def __hash__(self):return hash((self.root,self.p,self.q))
    def __lt__(self,v):return (self-v).sign()<0
    def __le__(self,v):return (self-v).sign()<=0
    def __gt__(self,v):return (self-v).sign()>0
    def __ge__(self,v):return (self-v).sign()>=0
    def data(self):return dict(kind='quadratic_field_value',root=self.root.data(),coefficients=[self.p.data(),self.q.data()])

# Delegate mixed operations to the event fiber field, retaining F everywhere else.
for name in ['__add__','__mul__','__truediv__','__eq__','__lt__']:
    original=getattr(K,name)
    def mixed(self,v,_name=name,_original=original):
        if isinstance(v,EventRoot):v=v.expression()
        if isinstance(v,FieldValue):
            if _name=='__add__':return v+self
            if _name=='__mul__':return v*self
            if _name=='__truediv__':return v.__rtruediv__(self)
            if _name=='__eq__':return False
            return v>self
        return _original(self,v)
    setattr(K,name,mixed)
K.__radd__=K.__add__;K.__rmul__=K.__mul__
K.__sub__=lambda self,v:self+(-v if isinstance(v,(K,FieldValue)) else -K.cast(v))
K.__rsub__=lambda self,v:(v+-self) if isinstance(v,FieldValue) else K.cast(v)+-self

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

class UnsupportedRoot(ValueError):
    def __init__(self,poly,discriminant):
        self.poly=poly; self.discriminant=discriminant; self.context=None
        super().__init__('In-strip quadratic root is outside the Q(sqrt(5)) engine')

def roots(poly):
    """Return exact Qsqrt5 roots; explicitly stop if a quadratic extension is needed."""
    c,b,a=poly
    if a==0:
        return [] if b==0 else [-c/b]
    discr=b*b-4*a*c
    if discr<0: return []
    radical=ksqrt(discr)
    if radical is None: raise UnsupportedRoot(poly,discr)
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
    try:return [r for r in roots(poly) if 0<=r<=U]
    except UnsupportedRoot:
        points=[K(0),U]
        vertex=-b/(2*a)
        if 0<vertex<U:points.insert(1,vertex)
        result=[]
        for left,right in zip(points,points[1:]):
            if (c+b*left+a*left*left).sign()*(c+b*right+a*right*right).sign()<0:
                result.append(EventRoot(poly,left,right))
        assert result
        return result

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
    cells=[];bound_values={K(0)}
    for k,(a,b,ids,start) in enumerate(hull):
        end=hull[k+1][3] if k+1<len(hull) else None
        if end is not None and end<=0: continue
        left=K(0) if start is None or start<0 else start
        bound_values.add(left)
        cells.append(dict(left=left.data(),right=None if end is None else end.data(),active=ids))
    ties=[]
    for x in sorted(bound_values):
        value=max(a*x+b for a,(b,_) in lines.items())
        ids=[i for i,c in coeffs.items() if sum(v*w for v,w in zip(at_u(c,u),(x,K(1))))==value]
        ties.append(dict(x=x.data(),value=value.data(),active=ids))
    maxslope=max(a for a,_ in lines.items())
    return dict(cells=cells,ties=ties,infinite_tail=cells[-1]['active'],
        asymptotic_max_slope_candidates=[i for i,c in coeffs.items() if at_u(c,u)[0]==maxslope])

def circuit_events(circuit):
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
        try: rr=roots_in_strip(poly,U)
        except UnsupportedRoot as error:
            error.context=dict(kind='triple',candidate_ids=[i,j,k],circuit=circuit['circuit'])
            raise
        polynomial_records.append(dict(kind='triple',ids=[i,j,k],coefficients=[c.data() for c in poly],roots=[r.data() for r in rr],identically_zero=all(c==0 for c in poly)))
        for r in rr: events.setdefault(r,[]).append(['triple',i,j,k])
    ordered=sorted(events); patches=[]
    raw={i:tuple(K(*v) for v in c['coefficients']) for i,c in enumerate(circuit['candidates'])}
    for left,right in zip(ordered,ordered[1:]):
        # Sample strictly between isolated events in F, never in a compositum.
        while True:
            lo=left.right if isinstance(left,EventRoot) else left
            hi=right.left if isinstance(right,EventRoot) else right
            if lo<hi:break
            if isinstance(left,EventRoot):left.refine()
            if isinstance(right,EventRoot):right.refine()
        sample=(lo+hi)/2
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
    return dict(circuit=circuit['circuit'],event_root_field='Q(sqrt(5)) and individually isolated quadratic extensions over it; no compositum evaluation',
        event_polynomial_count=len(polynomial_records),polynomials=polynomial_records,
        event_orientation_count=len(ordered),open_patches=patches,event_orientations=endpoints,
        complete_one_circuit=True)

def failure_certificate(error):
    poly=error.poly; U=K(0,F(1,5))
    def value(x): return poly[0]+poly[1]*x+poly[2]*x*x
    # Norm polynomial over Q, retaining the specified positive sqrt(5) embedding.
    norm=[K(0)]*5
    for i,p in enumerate(poly):
        for j,q in enumerate(poly): norm[i+j]+=p*K(q.a,-q.b)
    assert all(c.b==0 for c in norm)
    # Find an explicit rational sign-changing isolation bracket in the strip.
    # This documents the actual obstruction; it is not a general algebraic engine.
    intervals=[]; last=K(0); lastvalue=value(last)
    for k in range(1,4097):
        current=K(F(k,8192))
        if current>U: break
        currentvalue=value(current)
        if lastvalue.sign()*currentvalue.sign()<0:
            left,right=last,current
            for _ in range(48):
                midpoint=(left+right)/2
                if value(left).sign()*value(midpoint).sign()<0: right=midpoint
                else: left=midpoint
            # Derivative has constant nonzero sign on this bracket; uniqueness.
            dl=poly[1]+2*poly[2]*left; dr=poly[1]+2*poly[2]*right
            assert dl.sign()==dr.sign()!=0
            assert 0<left<right<U
            intervals.append(dict(left=left.data(),right=right.data(),
                left_sign=value(left).sign(),right_sign=value(right).sign(),
                derivative_sign=dl.sign()))
        last,lastvalue=current,currentvalue
    assert intervals, 'Missing-root stop must exhibit an actual in-strip root'
    return dict(status='named_field_stop',context=error.context,
        polynomial_coefficients=[p.data() for p in poly],
        discriminant=error.discriminant.data(),
        required_field='Q(sqrt(5),sqrt(discriminant)), with positive sqrt(5) embedding',
        rational_norm_polynomial_coefficients=[str(c.a) for c in norm],
        rational_isolation_brackets=intervals,
        missing_component='Exact arithmetic/order/evaluation at quadratic-extension event roots and their merged embeddings; joint topology not produced')

def decode_u(data):
    if isinstance(data,list):return K(*data)
    assert data['kind']=='quadratic_event_root'
    root=EventRoot([K(*v) for v in data['monic_coefficients']],K(*data['left']),K(*data['right']))
    assert root.branch==data['branch']
    return root

def field_sample(left,right):
    while True:
        lo=left.right if isinstance(left,EventRoot) else left
        hi=right.left if isinstance(right,EventRoot) else right
        if lo<hi:return (lo+hi)/2
        if isinstance(left,EventRoot):left.refine()
        if isinstance(right,EventRoot):right.refine()

def curve_value(curve,u):
    n,d=curve['numerator'],curve['denominator']
    return (n[0]+n[1]*u)/(d[0]+d[1]*u)

def raw_maps(circuits):
    return [{i:tuple(K(*v) for v in c['coefficients']) for i,c in enumerate(circuit['candidates'])} for circuit in circuits]

def vector_at(raw,x,u):
    result=[]
    for circuit in raw:
        values={i:a*x+b for i,c in circuit.items() for a,b in [at_u(c,u)]}
        value=max(values.values())
        ids=[i for i,v in values.items() if v==value]
        result.append(dict(value=value.data(),candidate_ids=ids))
    return result

def extract_joint(out):
    raw=raw_maps(out['circuits']); U=K(0,F(1,5))
    merged=sorted({decode_u(e['u']) for circuit in out['individual'] for e in circuit['event_orientations']})
    all_events=set(merged); strips=[]; comparisons=0
    for left,right in zip(merged,merged[1:]):
        sample=field_sample(left,right); curves=[]
        for l,circuit in enumerate(raw):
            hull=exact_hull(circuit,sample)
            for first,second in zip(hull['cells'],hull['cells'][1:]):
                i,j=first['active'][0],second['active'][0]; p,q=circuit[i],circuit[j]
                curve=dict(circuit=l,adjacent_candidates=[i,j],
                    numerator=[p[0]-q[0],p[2]-q[2]],denominator=[q[1]-p[1],q[3]-p[3]])
                dl=curve['denominator'][0]+curve['denominator'][1]*left
                dr=curve['denominator'][0]+curve['denominator'][1]*right
                assert dl>=0 and dr>=0 and curve['denominator'][0]+curve['denominator'][1]*sample>0
                curves.append(curve)
        events=set(); equalities=[]
        for i,j in combinations(range(len(curves)),2):
            p,q=curves[i],curves[j]
            if p['circuit']==q['circuit']:continue
            comparisons+=1
            a=product_linear(p['numerator'],q['denominator']);b=product_linear(q['numerator'],p['denominator'])
            poly=[v-w for v,w in zip(a,b)]; persistent=all(v==0 for v in poly)
            roots_here=[] if persistent else [r for r in roots_in_strip(poly,U) if left<r<right]
            events.update(roots_here)
            equalities.append(dict(curve_ids=[i,j],polynomial=[v.data() for v in poly],
                persistent=persistent,roots=[r.data() for r in roots_here]))
        all_events.update(events)
        substrips=[]; bounds=sorted({left,right}|events)
        for lo,hi in zip(bounds,bounds[1:]):
            mid=field_sample(lo,hi)
            ordered=sorted(range(len(curves)),key=lambda i:curve_value(curves[i],mid))
            groups=[]
            for i in ordered:
                if groups and curve_value(curves[groups[-1][0]],mid)==curve_value(curves[i],mid):groups[-1].append(i)
                else:groups.append([i])
            xs=[K(0)]+[curve_value(curves[g[0]],mid) for g in groups]
            cells=[]
            for k,xleft in enumerate(xs):
                xright=xs[k+1] if k+1<len(xs) else None
                x=(xleft+xright)/2 if xright is not None else xleft+1
                vector=vector_at(raw,x,mid)
                cells.append(dict(left_boundary=None if k==0 else groups[k-1],
                    right_boundary=None if xright is None else groups[k],
                    formula_candidates=[v['candidate_ids'] for v in vector],sample_x=x.data()))
            substrips.append(dict(left=lo.data(),right=hi.data(),sample_u=mid.data(),
                ordered_boundary_groups=groups,cells=cells,
                boundary_vectors=[dict(curves=g,vector=vector_at(raw,curve_value(curves[g[0]],mid),mid)) for g in groups]))
        strips.append(dict(left=left.data(),right=right.data(),sample=sample.data(),
            curves=[dict(circuit=c['circuit'],adjacent_candidates=c['adjacent_candidates'],
                numerator=[v.data() for v in c['numerator']],denominator=[v.data() for v in c['denominator']]) for c in curves],
            comparisons=equalities,substrips=substrips))
        print(json.dumps(dict(individual_strip=len(strips),active_curves=len(curves),joint_substrips=len(substrips))),flush=True)
    fibers=[]
    for u in sorted(all_events):
        hulls=[exact_hull(circuit,u) for circuit in raw]
        boundaries=sorted({decode_value(c['left']) for h in hulls for c in h['cells']})
        cells=[]
        for k,left in enumerate(boundaries):
            right=boundaries[k+1] if k+1<len(boundaries) else None
            x=(left+right)/2 if right is not None else left+1
            cells.append(dict(left=left.data(),right=None if right is None else right.data(),vector=vector_at(raw,x,u)))
        fibers.append(dict(u=u.data(),raw_hulls=hulls,cells=cells,
            point_vectors=[dict(x=x.data(),vector=vector_at(raw,x,u)) for x in boundaries]))
    out['joint']=dict(individual_orientation_count=len(merged),individual_strips=len(merged)-1,
        cross_curve_comparisons=comparisons,joint_orientation_count=len(all_events),
        joint_open_strips=sum(len(s['substrips']) for s in strips),
        joint_open_cells=sum(len(t['cells']) for s in strips for t in s['substrips']),
        topology_strips=strips,closed_fibers=fibers)

def decode_value(data):
    if isinstance(data,list):return K(*data)
    if data['kind']=='quadratic_event_root':return decode_u(data)
    root=decode_u(data['root']); p,q=[K(*v) for v in data['coefficients']]
    return field_value(root,p,q)

def test_adapter():
    poly=[K('17/10','-19/25'),K('21/5','-47/25'),K('-17/2','19/5')]
    root=roots_in_strip(poly,K(0,'1/5'))[0]; value=root.expression()
    assert poly[0]+poly[1]*value+poly[2]*value*value==0
    assert value/value==1
    assert 0<root<K(0,'1/5')
    assert K(0)<value and value<K(0,'1/5')
    assert root.compare_field(K('13/100'))>0 and root.compare_field(K('15/100'))<0
    duplicate=EventRoot(poly,root.left,root.right)
    assert duplicate==root and hash(duplicate)==hash(root)
    second=roots_in_strip([K('-1/10'),K(0),K(1)],K(0,'1/5'))[0]
    assert root<second and not second<root
    # Signs and inverses reduce exactly in F[alpha]/(alpha²+b alpha+c).
    assert (value+1)/(value+1)==1
    assert ((value+1)*(value-1)-(value*value-1))==0
    return dict(status='passed',first_root=root.data(),
        tests=['event polynomial reduces to zero','inverse','positive real embedding',
               'exact comparison with 13/100 and 15/100','same-root equality/hash',
               'ordering two different quadratic fields by isolating intervals',
               'field multiplication and division identities'])

def main():
    import time
    started=time.perf_counter()
    source=Path(__file__).parent.parent/'preflight'/'active-events.json'
    accepted=json.loads(source.read_text())
    out=dict(stage='complete joint denominator extraction',source_catalogue=str(source),
        source_sha256=hashlib.sha256(source.read_bytes()).hexdigest(),
        variable_order=accepted['variable_order'],field=accepted['field'],domain=accepted['domain'],
        circuits=accepted['circuits'],individual=[])
    out['adapter_tests']=test_adapter()
    for circuit in accepted['circuits']:
        try: result=circuit_events(circuit)
        except UnsupportedRoot as error:
            out['stop']=failure_certificate(error)
            out['seconds']=round(time.perf_counter()-started,3)
            Path(__file__).with_name('joint-denominators.json').write_text(json.dumps(out,indent=2)+'\n')
            print(json.dumps(out['stop']),flush=True)
            print('named field stop seconds',out['seconds'])
            return
        out['individual'].append(result)
        print(json.dumps(dict(circuit=circuit['circuit'],event_orientations=result['event_orientation_count'],patches=len(result['open_patches']))),flush=True)
    extract_joint(out)
    out['seconds']=round(time.perf_counter()-started,3)
    Path(__file__).with_name('joint-denominators.json').write_text(json.dumps(out,indent=2)+'\n')
    print('complete joint extraction seconds',out['seconds'])

if __name__=='__main__': main()
