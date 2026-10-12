#!/usr/bin/env python3
"""Exact general-matrix seven-variable cone inventory. Writes only cone.json."""
from fractions import Fraction as F
from functools import total_ordering
from itertools import product
import json
from pathlib import Path

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

from itertools import combinations
import hashlib,time
ROOT=Path(__file__).resolve().parent

def dot(v,w):return sum((a*b for a,b in zip(v,w)),K(0))
def solve2(n,m,r,s):
    d=n[0]*m[1]-n[1]*m[0];assert d!=0
    return [(r*m[1]-s*n[1])/d,(n[0]*s-m[0]*r)/d]
def normalize(row):
    pivot=next(v for v in row if v!=0);positive=pivot if pivot>0 else -pivot
    assert positive>0
    return tuple(v/positive for v in row),positive
def rank(rows):
    basis={};ids=[]
    for idx,row in enumerate(rows):
        row=list(row)
        for p,b in sorted(basis.items()):
            q=row[p];row=[v-q*w for v,w in zip(row,b)]
        if all(v==0 for v in row):continue
        p=next(i for i,v in enumerate(row) if v!=0);q=row[p];row=[v/q for v in row]
        assert p not in basis
        for old,b in list(basis.items()):
            q=b[p];basis[old]=[v-q*w for v,w in zip(b,row)]
        basis[p]=row;ids.append(idx)
    return len(basis),sorted(basis),ids
def build():
    start=time.perf_counter();z=K(0,1);tau2=5-2*z;a=(z-1)/4;b=(z+1)/4;w=1+b;R=(5-z)/10
    N=[(K(1),K(0)),((z-1)/4,(5-z)/4),(-(1+z)/4,(3*z-5)/4),(-(1+z)/4,-(3*z-5)/4),((z-1)/4,-(5-z)/4)]
    def rotate(v):return (b*v[0]-(3*z-5)/4*v[1],b*(v[0]+v[1]))
    base=[(-R,K(0)),(R,K(0)),(K(0),R)];cur=base;triangles=[]
    for j in range(10):triangles.append(cur);cur=[rotate(v) for v in cur]
    assert cur==base
    lam0=[K(1)/(2*w),K(1)/(2*w),K(0),b/w,K(0)]
    lams=[[lam0[(i-l)%5] for i in range(5)] for l in range(5)]
    edges=[]
    for i in range(5):
        row=[K(0)]*5;row[(i-1)%5]=K(1);row[(i+1)%5]=K(1);row[i]=-2*a;edges.append(row)
    for row in lams+edges:
        assert all(dot(row,[n[k] for n in N])==0 for k in range(2))
    H=[[(-a if i==j else K(F(1,2)) if (i-j)%5 in(1,4) else K(0)) for j in range(5)] for i in range(5)]
    assert all(dot(row,[n[k] for n in N])==0 for row in H for k in range(2))
    gauge=[[K(0)]*5 for _ in range(3)]
    for j in range(5):
        original=[K(int(i==j)) for i in range(5)]
        t=solve2(N[3],N[4],-original[3],-original[4])
        projected=[original[i]+dot(N[i],t) for i in range(5)]
        assert projected[3]==projected[4]==0
        for i in range(3):gauge[i][j]=projected[i]
    assert all(dot(row,[n[k] for n in N])==0 for row in gauge for k in range(2))
    for row in lams+edges:
        assert all(sum(row[i]*gauge[i][j] for i in range(3))==row[j] for j in range(5))
    assert all(sum(gauge[k][i]*H[k][l]*gauge[l][j] for k in range(3) for l in range(3))==H[i][j] for i in range(5) for j in range(5))
    records={};raw=[];raw_by_circuit=[]
    def add_row(row,source):
        normalized,scale=normalize(row)
        if normalized not in records:records[normalized]=[]
        records[normalized].append(dict(source=source,positive_normalization_divisor=scale.data()))
        raw.append((tuple(row),source))
    for l,lam in enumerate(lams):
        active=[i for i,v in enumerate(lam) if v>0];assert len(active)==3
        count=0
        for j,tri in enumerate(triangles):
            for choice in product(range(3),repeat=3):
                matrix=[K(0)]*4
                for i,k in zip(active,choice):
                    nx,ny=N[i];vx,vy=tri[k]
                    for q,v in enumerate((nx*vx,ny*vx,nx*vy,ny*vy)):matrix[q]+=lam[i]*v
                add_row(lam[:3]+[-v for v in matrix],dict(kind='fit',circuit=l,template=j,active_normals=active,vertices=list(choice)))
                count+=1
        raw_by_circuit.append(count)
    for i,row in enumerate(edges):add_row(row[:3]+[K(0)]*4,dict(kind='edge',edge=i))
    rows=list(records);rowrank,pivots,basisids=rank(rows)
    assert rowrank==7
    # Gauge exact regular supports t=1; both invertible and singular strict points.
    gauge_translation=solve2(N[3],N[4],K(-1),K(-1))
    h=[K(1)+dot(n,gauge_translation) for n in N];assert h[3]==h[4]==0
    witness=h[:3]+[K(1),K(0),K(0),K(1)];singular=h[:3]+[K(0)]*4
    slacks=[dot(row,witness) for row in rows];ss=[dot(row,singular) for row in rows]
    assert all(v>0 for v in slacks+ss)
    # Right reflection B->B J relabels N_i->N_-i and then restores the gauge.
    reflection=[[K(0)]*7 for _ in range(7)]
    for j in range(3):
        hj=[K(int(i==j)) for i in range(5)];g=[hj[(-i)%5] for i in range(5)]
        t=solve2(N[3],N[4],-g[3],-g[4]);g=[g[i]+dot(N[i],t) for i in range(5)]
        assert g[3]==g[4]==0
        for i in range(3):reflection[i][j]=g[i]
    for i,sgn in enumerate((1,-1,1,-1),3):reflection[i][i]=K(sgn)
    assert all(dot(reflection[i],[reflection[k][j] for k in range(7)])==int(i==j) for i in range(7) for j in range(7))
    mapped=[]
    for row in rows:
        image=[dot(row,[reflection[i][j] for i in range(7)]) for j in range(7)];nr,_=normalize(image)
        assert nr in records;mapped.append(rows.index(nr))
    # Exact source binding to the accepted scalar chart, without rerunning it.
    catalogue=ROOT.parent/'preflight'/'active-events.json';packet=json.loads(catalogue.read_text());source_choices={}
    for circuit in packet['circuits']:
        for candidate in circuit['candidates']:
            for j,k0,k1,k2 in candidate['sources']:
                source_choices[(circuit['circuit'],j,k0,k1,k2)]=tuple(K(*v) for v in candidate['coefficients'])
    for row,source in raw:
        if source['kind']!='fit':continue
        c00,c01,c10,c11=[-v for v in row[3:]]
        projected=(c11,c00,tau2*c10,-c01)
        key=(source['circuit'],source['template'],*source['vertices'])
        assert projected==source_choices[key]
    # HKO exact control is a point of the cone, not a positivity proof.
    u=z/5;hkoh=[-1+3*z/5,1-z/5,-1+3*z/5,K(0),K(0)]
    hko=hkoh[:3]+[K(1),-u,tau2*u,K(1)]
    hkoslacks=[dot(row,hko) for row in rows];assert all(v>=0 for v in hkoslacks)
    P=lambda hh:sum(hh[i]*hh[(i+1)%5]-a*hh[i]*hh[i] for i in range(5))
    determinant=lambda v:v[3]*v[6]-v[4]*v[5]
    assert P(hkoh)==determinant(hko)/(3+z)
    reflected=[dot(r,hko) for r in reflection];rh=reflected[:3]+[K(0),K(0)]
    assert determinant(reflected)==-determinant(hko) and P(rh)==P(hkoh)
    # Ambient inertia is only a constant-form diagnostic, not cone positivity.
    targetQ=[[K(0)]*7 for _ in range(7)]
    for i in range(3):
        for j in range(3):targetQ[i][j]=H[i][j]
    k=K(1)/(3+z);targetQ[3][6]=targetQ[6][3]=-k/2;targetQ[4][5]=targetQ[5][4]=k/2
    principal=[-a,a*a-K(F(1,4)),a*(K(F(1,2))-a*a)]
    assert [v.sign() for v in principal]==[-1,-1,1]
    bits=lambda q:max(abs(q.numerator).bit_length(),q.denominator.bit_length())
    height=max(bits(q) for row in rows for v in row for q in (v.a,v.b))
    out=dict(status='PASS exact seven-variable cone reduction/inventory; NO positivity or ray enumeration',
        variables=['h0','h1','h2','m00','m01','m10','m11'],field='Q(sqrt(5))',encoding='[a,b]=a+b sqrt(5); rational strings',
        matrix_convention='B=S A S^-1, S=diag(1,tau), physical normal transform A; point coordinates q_comp=S^-1 q_phys; detB=detA',
        support_transfer='Y_comp=B^T K_comp, hence Y_phys=A^T K_phys; canonical gauge h3=h4=0',
        support_gauge_projection=[[v.data() for v in row] for row in gauge],
        representative_basis=[[K(int(i==j)).data() for j in range(3)] for i in range(5)],
        translation_basis=[[v.data() for v in n] for n in N],
        gauge_checks='Q*N=0; every lambda/edge row equals its truncated row*Q; H5=Q^T H3 Q',
        determinant='m00*m11-m01*m10',physical_ratio='c²*abs(detB)/(5P(h)); at c=1: abs(detB)/(5P(h))',
        proposed_target='P(h)-abs(detB)/(3+sqrt5)>=0 on the invertible body sector; NOT proved',
        positive_det_quadratic_matrix=[[v.data() for v in row] for row in targetQ],
        ambient_inertia=dict(positive=3,negative=4,zero=0,
            support_principal_minors=[v.data() for v in principal],support_block='one positive, two negative by LDL',
            matrix_block='two positive, two negative: two nonzero off-diagonal 2x2 blocks',
            warning='Indefinite ambient form; no PSD or quadratic copositivity assertion'),
        normals=[[v.data() for v in n] for n in N],tau_squared=tau2.data(),
        templates=[[[v.data() for v in p] for p in tri] for tri in triangles],
        closure_rays=[[v.data() for v in row] for row in lams],canonical_edge_rows=[[v.data() for v in row] for row in edges],
        area_quadratic_matrix=[[v.data() for v in row] for row in H],
        raw_fit_count=sum(raw_by_circuit),raw_fit_by_circuit=raw_by_circuit,raw_edge_count=5,
        raw_total=len(raw),normalized_distinct_fit_rows=sum(any(s['source']['kind']=='fit' for s in sources) for sources in records.values()),
        normalized_distinct_edge_rows=sum(any(s['source']['kind']=='edge' for s in sources) for sources in records.values()),
        normalized_distinct_total=len(rows),normalization='Divide by absolute first nonzero coefficient, positive in actual sqrt5 embedding; preserve inequality direction',
        rows=[dict(coefficients=[v.data() for v in row],sources=records[row]) for row in rows],
        maximum_rational_coefficient_bit_size=height,rank=rowrank,pivot_columns=pivots,independent_row_indices=basisids,
        lineality_dimension=7-rowrank,lineality_basis=[],
        strict_invertible_witness=[v.data() for v in witness],strict_invertible_minimum_slack=min(slacks).data(),
        strict_singular_witness=[v.data() for v in singular],strict_singular_minimum_slack=min(ss).data(),
        determinant_zero_note='detB=0 can be interior of this linear cone; it is a boundary of the invertible/sign physical sector, not necessarily a polyhedral boundary',
        reflection_map=[[v.data() for v in row] for row in reflection],reflection_row_permutation=mapped,
        scalar_chart_source_sha256=hashlib.sha256(catalogue.read_bytes()).hexdigest(),
        scalar_chart_binding='All1350rawchoice coefficients project exactly to accepted (1,x,u,xu) catalogue via B=[[x,-xu],[tau²u,1]]',
        hko_control=dict(point=[v.data() for v in hko],determinant=determinant(hko).data(),P=P(hkoh).data(),
            minimum_slack=min(hkoslacks).data(),target_gap=(P(hkoh)-determinant(hko)/(3+z)).data(),
            reflected_point=[v.data() for v in reflected],reflected_determinant=determinant(reflected).data()),
        elapsed_seconds=round(time.perf_counter()-start,3))
    return out

def main():
    out=build()
    (ROOT/'cone.json').write_text(json.dumps(out,indent=2)+'\n')
    print(json.dumps({k:out[k] for k in ['raw_fit_count','raw_total','normalized_distinct_fit_rows','normalized_distinct_total','rank','lineality_dimension','maximum_rational_coefficient_bit_size','elapsed_seconds']}))

if __name__=='__main__':main()
