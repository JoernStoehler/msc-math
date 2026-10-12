"""Read-only independent audit of the currently retained joint-stage outcome."""
import importlib.util
from pathlib import Path
from fractions import Fraction
from math import isqrt
import hashlib
import json

root = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('review_arithmetic',
    root.parent.parent / 'hko-angle-nonconformal' / 'review_candidates.py')
m = importlib.util.module_from_spec(spec)
spec.loader.exec_module(m)
E, z = m.E, m.z
dec = lambda q: E(*q)
packet = json.loads((root / 'joint-denominators.json').read_text())
source = root.parent / 'preflight' / 'active-events.json'
accepted = json.loads(source.read_text())
assert hashlib.sha256(source.read_bytes()).hexdigest() == packet['source_sha256']
assert packet['circuits'] == accepted['circuits']
assert all(packet['individual'][0][k]==v for k,v in accepted['circuit_zero'].items()
           if k!='event_root_field')
stop = json.loads((root / 'first-field-stop.json').read_text())
assert stop['status'] == 'named_field_stop'
context = stop['context']
assert context == {'kind': 'triple', 'candidate_ids': [61, 76, 119], 'circuit': 1}
raw = [[dec(q) for q in c['coefficients']] for c in packet['circuits'][1]['candidates']]
i, j, k = context['candidate_ids']
p = [a-b for a,b in zip(raw[i],raw[j])]
q = [a-b for a,b in zip(raw[i],raw[k])]
poly = [p[1]*q[0]-q[1]*p[0],
        p[1]*q[2]+p[3]*q[0]-q[1]*p[2]-q[3]*p[0], p[3]*q[2]-q[3]*p[2]]
assert poly == list(map(dec, stop['polynomial_coefficients']))
c, b, a = poly
discriminant = b*b-4*a*c
assert discriminant == dec(stop['discriminant']) and discriminant.sign() > 0
norm = discriminant.a**2-5*discriminant.b**2
assert norm == Fraction(101,15625)
assert isqrt(norm.numerator)**2 != norm.numerator
normpoly = [z]*5
for s,v in enumerate(poly):
    for t,w in enumerate(poly):
        normpoly[s+t] += v*E(w.a,-w.b)
assert all(v.b == 0 for v in normpoly)
assert [v.a for v in normpoly] == list(map(Fraction,stop['rational_norm_polynomial_coefficients']))
ev = lambda q,u: q[0]+q[1]*u+q[2]*u*u
U = E(0,'1/5')
assert b.sign() == (b+2*a*U).sign() == -1


def sign_on_interval(q, lo, hi):
    values = [ev(q,lo),ev(q,hi)]
    if q[2] != z:
        at = -q[1]/(2*q[2])
        if (at-lo).sign()>0 and (hi-at).sign()>0:
            values.append(ev(q,at))
    signs = {v.sign() for v in values}
    assert len(signs)==1 and 0 not in signs
    return signs.pop()


def product(v,w):
    return [v[0]*w[0],v[0]*w[1]+v[1]*w[0],v[1]*w[1]]


for interval in stop['rational_isolation_brackets']:
    lo,hi=dec(interval['left']),dec(interval['right'])
    assert lo.sign()>0 and (hi-lo).sign()>0 and (U-hi).sign()>0
    assert ev(poly,lo).sign()==interval['left_sign']==1
    assert ev(poly,hi).sign()==interval['right_sign']==-1
    assert (b+2*a*lo).sign()==(b+2*a*hi).sign()==interval['derivative_sign']==-1
    # Exact concurrent x=N/D, throughout the isolating bracket.
    N=[raw[i][0]-raw[j][0],raw[i][2]-raw[j][2]]
    D=[raw[j][1]-raw[i][1],raw[j][3]-raw[i][3]]
    dsign=sign_on_interval([*D,z],lo,hi)
    assert sign_on_interval([*N,z],lo,hi)==dsign
    higher=raw[75]
    slope=[higher[1]-raw[i][1],higher[3]-raw[i][3]]
    intercept=[higher[0]-raw[i][0],higher[2]-raw[i][2]]
    difference=[u+v for u,v in zip(product(slope,N),product(intercept,D))]
    assert sign_on_interval(difference,lo,hi)==dsign
print(json.dumps({'status':'PASS stop polynomial/embedding; consequential inactive-event qualification',
    'discriminant_norm':'101/15625, nonsquare', 'in_strip_root':True,
    'unique_root_on_whole_strip':True,
    'concurrency_x_positive':True,'candidate75_strictly_above_entire_isolating_bracket':True,
    'complete_joint_denominators':False}))

# Independent quadratic-fiber arithmetic. No extraction code is imported.
from functools import cmp_to_key
from itertools import combinations
from math import isqrt


def rational_sqrt(q):
    if q < 0:
        return None
    n,d=isqrt(q.numerator),isqrt(q.denominator)
    return Fraction(n,d) if n*n==q.numerator and d*d==q.denominator else None


def is_field_square(q):
    if q.sign()<0:
        return False
    if q==z:
        return True
    norm=rational_sqrt(q.a*q.a-5*q.b*q.b)
    if norm is None:
        return False
    for s in (1,-1):
        a=rational_sqrt((q.a+s*norm)/2)
        if a:
            v=E(a,q.b/(2*a))
            if v*v==q:
                return True
    return q.b==0 and rational_sqrt(q.a/5) is not None


class Root:
    def __init__(self,c,b,branch,lo,hi):
        self.c,self.b,self.branch,self.lo,self.hi=c,b,branch,lo,hi
        self.key=(c.key(),b.key(),branch)

    def value(self,u):
        return u*u+self.b*u+self.c

    def refine(self):
        at=(self.lo+self.hi)/E(2)
        if self.value(at).sign()==self.value(self.lo).sign():
            self.lo=at
        else:
            self.hi=at

    def linear_sign(self,p,q):
        if q==z:
            return p.sign()
        at=-p/q
        assert self.value(at)!=z
        while (at-self.lo).sign()>=0 and (self.hi-at).sign()>=0:
            self.refine()
        return q.sign()*(1 if (self.lo-at).sign()>0 else -1)


registry={}
def root_decode(data):
    c,b,a=map(dec,data['monic_coefficients'])
    assert a==E(1) and not is_field_square(b*b-4*c)
    lo,hi=dec(data['left']),dec(data['right'])
    assert (hi-lo).sign()>0
    values=[lo*lo+b*lo+c,hi*hi+b*hi+c]
    assert values[0].sign()*values[1].sign()==-1
    derivative=[2*lo+b,2*hi+b]
    assert derivative[0].sign()==derivative[1].sign()!=0
    branch=data['branch']
    assert branch==(0 if derivative[0].sign()<0 else 1)
    key=(c.key(),b.key(),branch)
    if key not in registry:
        registry[key]=Root(c,b,branch,lo,hi)
    return registry[key]


class V:
    def __init__(self,p=z,q=z,root=None):
        self.p,self.q,self.root=p,q,root if q!=z else None

    @staticmethod
    def cast(v):
        return v if isinstance(v,V) else V(v if isinstance(v,E) else E(v))

    def pair(self,v):
        v=V.cast(v)
        if self.root is not None and v.root is not None:
            assert self.root.key==v.root.key
        return v,self.root or v.root

    def __add__(self,v):
        v,root=self.pair(v)
        return V(self.p+v.p,self.q+v.q,root)

    __radd__=__add__
    def __neg__(self):
        return V(-self.p,-self.q,self.root)
    def __sub__(self,v):
        return self+-V.cast(v)
    def __rsub__(self,v):
        return V.cast(v)+-self
    def __mul__(self,v):
        v,root=self.pair(v)
        if root is None:
            return V(self.p*v.p)
        return V(self.p*v.p-self.q*v.q*root.c,
                 self.p*v.q+self.q*v.p-self.q*v.q*root.b,root)
    __rmul__=__mul__
    def __truediv__(self,v):
        v,root=self.pair(v)
        if v.q==z:
            return V(self.p/v.p,self.q/v.p,root)
        norm=v.p*v.p-v.p*v.q*root.b+v.q*v.q*root.c
        assert norm!=z
        return self*V((v.p-v.q*root.b)/norm,-v.q/norm,root)
    def sign(self):
        return self.p.sign() if self.root is None else self.root.linear_sign(self.p,self.q)
    def key(self):
        return self.p.key(),self.q.key(),None if self.root is None else self.root.key
    def __eq__(self,v):
        return self.key()==V.cast(v).key()


def value_decode(data):
    if isinstance(data,list):
        return V(dec(data))
    root=root_decode(data if data['kind']=='quadratic_event_root' else data['root'])
    return V(z,E(1),root) if data['kind']=='quadratic_event_root' else V(*map(dec,data['coefficients']),root)


def compare(u,v):
    if u.root is None or v.root is None or u.root.key==v.root.key:
        return (u-v).sign()
    # Orientation comparisons concern pure event roots, not composita.
    assert u.p==v.p==z and u.q==v.q==E(1)
    while (u.root.hi-v.root.lo).sign()>=0 and (v.root.hi-u.root.lo).sign()>=0:
        u.root.refine();v.root.refine()
    return -1 if (v.root.lo-u.root.hi).sign()>0 else 1


def complete_roots(poly,records):
    c,b,a=poly
    got=list(map(value_decode,records))
    for v in got:
        assert V(c)+v*b+v*v*a==V(z)
        assert compare(v,V(z))>=0 and compare(v,V(U))<=0
    if a==z:
        if b==z:
            assert not got
            return c==z
        v=V(-c/b)
        expected=[v] if compare(v,V(z))>=0 and compare(v,V(U))<=0 else []
    elif got:
        first=got[0]
        other=V(-b/a)-first
        candidates=[first]
        if other.root is not None:
            # Encode its other real embedding without constructing a compositum.
            if other.p!=-first.root.b or other.q!=-E(1):
                assert False,'Unexpected quadratic conjugation'
            otherroot=Root(first.root.c,first.root.b,1-first.root.branch,
                           -first.root.b-first.root.hi,-first.root.b-first.root.lo)
            registry.setdefault(otherroot.key,otherroot)
            other=V(z,E(1),registry[otherroot.key])
        if compare(other,V(z))>=0 and compare(other,V(U))<=0 and other!=first:
            candidates.append(other)
        expected=candidates
    else:
        values=[c,c+b*U+a*U*U]
        vertex=-b/(2*a)
        if vertex.sign()>0 and (U-vertex).sign()>0:
            values.append(c+b*vertex+a*vertex*vertex)
        assert all(v.sign()>0 for v in values) or all(v.sign()<0 for v in values)
        expected=[]
    assert {v.key() for v in got}=={v.key() for v in expected}
    return False


def fiber(raw,u,hull):
    lines=[(V(c[1])+u*c[3],V(c[0])+u*c[2]) for c in raw]
    previous=V(z)
    boundarykeys={previous.key()}
    for cell in hull['cells']:
        lo=value_decode(cell['left']);hi=None if cell['right'] is None else value_decode(cell['right'])
        assert lo==previous
        assert hi is None or (hi-lo).sign()>0
        ids=cell['active'];a,b=lines[ids[0]]
        assert [i for i,line in enumerate(lines) if line==(a,b)]==ids
        for c,d in lines:
            assert ((a-c)*lo+b-d).sign()>=0
            assert (a-c).sign()>=0 if hi is None else ((a-c)*hi+b-d).sign()>=0
        boundarykeys.add(lo.key());previous=hi
    assert previous is None
    assert {value_decode(t['x']).key() for t in hull['ties']}==boundarykeys
    for tie in hull['ties']:
        x=value_decode(tie['x']);values=[a*x+b for a,b in lines]
        highest=values[0]
        for v in values[1:]:
            if (v-highest).sign()>0:
                highest=v
        assert highest==value_decode(tie['value'])
        assert [i for i,v in enumerate(values) if v==highest]==tie['active']
    maxslope=lines[0][0]
    for slope,_ in lines[1:]:
        if (slope-maxslope).sign()>0:
            maxslope=slope
    assert [i for i,(slope,_) in enumerate(lines) if slope==maxslope]==hull['asymptotic_max_slope_candidates']


individual_polynomials=0;raw_fibers=0;persistent=0
for source,result in zip(packet['circuits'],packet['individual']):
    raw=[[dec(v) for v in c['coefficients']] for c in source['candidates']]
    ids=source['survivors'];expected={}
    for i,j in combinations(ids,2):
        diff=[a-b for a,b in zip(raw[i],raw[j])]
        expected[('slope',(i,j))]=[diff[1],diff[3],z]
        expected[('intercept',(i,j))]=[diff[0],diff[2],z]
    for i,j,k in combinations(ids,3):
        p=[a-b for a,b in zip(raw[i],raw[j])];q=[a-b for a,b in zip(raw[i],raw[k])]
        expected[('triple',(i,j,k))]=[p[1]*q[0]-q[1]*p[0],
            p[1]*q[2]+p[3]*q[0]-q[1]*p[2]-q[3]*p[0],p[3]*q[2]-q[3]*p[2]]
    allroots={V(z).key():V(z),V(U).key():V(U)}
    assert len(expected)==len(result['polynomials'])
    for record in result['polynomials']:
        poly=expected.pop((record['kind'],tuple(record['ids'])))
        assert poly==list(map(dec,record['coefficients']))
        persistent+=complete_roots(poly,record['roots'])
        for rdata in record['roots']:
            v=value_decode(rdata);allroots[v.key()]=v
        individual_polynomials+=1
    assert not expected
    ordered=sorted(allroots.values(),key=cmp_to_key(compare))
    assert [v.key() for v in ordered]==[value_decode(e['u']).key() for e in result['event_orientations']]
    assert len(result['open_patches'])==len(ordered)-1
    for i,patch in enumerate(result['open_patches']):
        lo,hi=value_decode(patch['left']),value_decode(patch['right'])
        assert lo==ordered[i] and hi==ordered[i+1]
        sample=value_decode(patch['sample'])
        assert sample.root is None and compare(lo,sample)<0 and compare(sample,hi)<0
        fiber(raw,sample,patch['hull']);raw_fibers+=1
        for curve in patch['parametric_x_boundaries']:
            i,j=curve['adjacent_candidates'];p,q=raw[i],raw[j]
            N=[p[0]-q[0],p[2]-q[2]];D=[q[1]-p[1],q[3]-p[3]]
            assert N==list(map(dec,curve['numerator'])) and D==list(map(dec,curve['denominator']))
            for u in (lo,hi):
                assert (V(D[0])+u*D[1]).sign()>=0
                assert (V(N[0])+u*N[1]).sign()>=0
            assert (V(D[0])+sample*D[1]).sign()>0
    for event in result['event_orientations']:
        fiber(raw,value_decode(event['u']),event['hull']);raw_fibers+=1
print(json.dumps({'status':'PASS independent individual-event and closed-fiber coverage',
    'polynomials':individual_polynomials,'raw_whole_x_fibers':raw_fibers,
    'quadratic_root_embeddings':len(registry),'persistent_zero_predicates':persistent,
    'joint_cross_refinement_accepted':False}))


def conjugate(v):
    if v.root is None:
        return v
    root=v.root
    other=Root(root.c,root.b,1-root.branch,-root.b-root.hi,-root.b-root.lo)
    registry.setdefault(other.key,other)
    return V(z,E(1),registry[other.key])


def interval_roots(poly,records,lo,hi):
    c,b,a=poly;got=list(map(value_decode,records))
    for v in got:
        assert V(c)+v*b+v*v*a==V(z)
        assert compare(lo,v)<0 and compare(v,hi)<0
    if a==z and b==z:
        assert not got
        return c==z
    if a==z:
        v=V(-c/b);expected=[v] if compare(lo,v)<0 and compare(v,hi)<0 else []
    elif got:
        first=got[0]
        other=conjugate(first) if first.root is not None else V(-b/a)-first
        expected=[first]
        if other!=first and compare(lo,other)<0 and compare(other,hi)<0:
            expected.append(other)
    else:
        vals=[V(c)+lo*b+lo*lo*a,V(c)+hi*b+hi*hi*a]
        vertex=V(-b/(2*a))
        if compare(lo,vertex)<0 and compare(vertex,hi)<0:
            v=V(c)+vertex*b+vertex*vertex*a
            assert v!=V(z),'Missing double root'
            vals.append(v)
        assert all(v.sign()>=0 for v in vals) or all(v.sign()<=0 for v in vals), 'Missing interior root'
        expected=[]
    assert {v.key() for v in got}=={v.key() for v in expected}
    return False


def vector(raws,x,u,record):
    assert len(record)==5
    for raw,rec in zip(raws,record):
        values=[(V(c[1])+u*c[3])*x+V(c[0])+u*c[2] for c in raw]
        highest=values[0]
        for v in values[1:]:
            if (v-highest).sign()>0:
                highest=v
        assert highest==value_decode(rec['value'])
        assert [i for i,v in enumerate(values) if v==highest]==rec['candidate_ids']


def curve_value(c,u):
    N,D=list(map(dec,c['numerator'])),list(map(dec,c['denominator']))
    return (V(N[0])+u*N[1])/(V(D[0])+u*D[1])


if 'joint' in packet:
    joint=packet['joint'];raws=[[[dec(v) for v in c['coefficients']] for c in source['candidates']]
                              for source in packet['circuits']]
    merged={value_decode(e['u']).key():value_decode(e['u']) for result in packet['individual'] for e in result['event_orientations']}
    merged_order=sorted(merged.values(),key=cmp_to_key(compare));all_events=dict(merged)
    assert len(merged_order)==joint['individual_orientation_count']
    assert len(joint['topology_strips'])==len(merged_order)-1==joint['individual_strips']
    cross_count=0;open_cells=0;open_strips=0
    for index,strip in enumerate(joint['topology_strips']):
        lo,hi,sample=map(value_decode,[strip['left'],strip['right'],strip['sample']])
        assert lo==merged_order[index] and hi==merged_order[index+1]
        assert compare(lo,sample)<0 and compare(sample,hi)<0
        curves=strip['curves'];expected_curves=[]
        for l,result in enumerate(packet['individual']):
            patch=next(p for p in result['open_patches'] if compare(value_decode(p['left']),sample)<0 and compare(sample,value_decode(p['right']))<0)
            expected_curves.extend(dict(circuit=l,**c) for c in patch['parametric_x_boundaries'])
        assert curves==expected_curves
        for c in curves:
            D=list(map(dec,c['denominator']))
            assert (V(D[0])+lo*D[1]).sign()>=0 and (V(D[0])+hi*D[1]).sign()>=0
            assert (V(D[0])+sample*D[1]).sign()>0
        expected_pairs={(i,j) for i,j in combinations(range(len(curves)),2) if curves[i]['circuit']!=curves[j]['circuit']}
        events={lo.key():lo,hi.key():hi}
        for rec in strip['comparisons']:
            i,j=rec['curve_ids'];assert (i,j) in expected_pairs;expected_pairs.remove((i,j))
            p,q=curves[i],curves[j]
            pN,pD=list(map(dec,p['numerator'])),list(map(dec,p['denominator']))
            qN,qD=list(map(dec,q['numerator'])),list(map(dec,q['denominator']))
            poly=[a-b for a,b in zip(product(pN,qD),product(qN,pD))]
            assert poly==list(map(dec,rec['polynomial']))
            assert rec['persistent']==interval_roots(poly,rec['roots'],lo,hi)
            for rdata in rec['roots']:
                v=value_decode(rdata);events[v.key()]=v;all_events[v.key()]=v
            cross_count+=1
        assert not expected_pairs
        bounds=sorted(events.values(),key=cmp_to_key(compare))
        assert len(strip['substrips'])==len(bounds)-1
        for si,sub in enumerate(strip['substrips']):
            left,right,u=map(value_decode,[sub['left'],sub['right'],sub['sample_u']])
            assert left==bounds[si] and right==bounds[si+1]
            assert compare(left,u)<0 and compare(u,right)<0
            groups=sub['ordered_boundary_groups'];flat=[i for group in groups for i in group]
            assert sorted(flat)==list(range(len(curves)))
            xs=[V(z)]
            for group in groups:
                x=curve_value(curves[group[0]],u)
                assert (x-xs[-1]).sign()>0
                assert all(curve_value(curves[i],u)==x for i in group)
                xs.append(x)
            assert len(sub['cells'])==len(xs)
            for k,cell in enumerate(sub['cells']):
                assert cell['left_boundary']==(None if k==0 else groups[k-1])
                assert cell['right_boundary']==(groups[k] if k+1<len(xs) else None)
                x=value_decode(cell['sample_x'])
                assert (x-xs[k]).sign()>0 and (k+1==len(xs) or (xs[k+1]-x).sign()>0)
                vals=[]
                for raw in raws:
                    values=[(V(c[1])+u*c[3])*x+V(c[0])+u*c[2] for c in raw]
                    highest=values[0]
                    for v in values[1:]:
                        if (v-highest).sign()>0:highest=v
                    vals.append([i for i,v in enumerate(values) if v==highest])
                assert vals==cell['formula_candidates'];open_cells+=1
            assert len(sub['boundary_vectors'])==len(groups)
            for group,rec in zip(groups,sub['boundary_vectors']):
                assert group==rec['curves']
                vector(raws,curve_value(curves[group[0]],u),u,rec['vector'])
            open_strips+=1
    assert cross_count==joint['cross_curve_comparisons']
    ordered=sorted(all_events.values(),key=cmp_to_key(compare))
    assert len(ordered)==joint['joint_orientation_count']==len(joint['closed_fibers'])
    assert open_cells==joint['joint_open_cells'] and open_strips==joint['joint_open_strips']
    closed_raw=0
    for u,data in zip(ordered,joint['closed_fibers']):
        assert u==value_decode(data['u'])
        boundaries={}
        for raw,hull in zip(raws,data['raw_hulls']):
            fiber(raw,u,hull);closed_raw+=1
            for cell in hull['cells']:
                x=value_decode(cell['left']);boundaries[x.key()]=x
        xs=sorted(boundaries.values(),key=cmp_to_key(lambda a,b:(a-b).sign()))
        assert len(xs)==len(data['cells'])==len(data['point_vectors'])
        for k,cell in enumerate(data['cells']):
            assert value_decode(cell['left'])==xs[k]
            assert cell['right'] is None if k+1==len(xs) else value_decode(cell['right'])==xs[k+1]
            x=(xs[k]+xs[k+1])/2 if k+1<len(xs) else xs[k]+1
            vector(raws,x,u,cell['vector'])
        for x,rec in zip(xs,data['point_vectors']):
            assert x==value_decode(rec['x']);vector(raws,x,u,rec['vector'])
    print(json.dumps({'status':'PASS complete joint denominator coverage',
        'individual_polynomials':individual_polynomials,'cross_curve_comparisons':cross_count,
        'joint_orientation_events':len(ordered),'joint_open_strips':open_strips,
        'joint_open_cells':open_cells,'merged_closed_raw_circuit_fibers':closed_raw,
        'producer_arithmetic_imported':False,'writes':False}))
