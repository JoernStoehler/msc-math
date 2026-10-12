#!/usr/bin/env python3
"""Exact complete fixed-Q template denominators. Python standard library only.

Run from any directory. Writes ONLY sibling denominators.json, deterministically.
Field encoding [a,b] means a+b*sqrt(5), with rational strings a,b.
Coordinates and template labels reproduce verify_equiangular_denominators.py.
"""
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

def envelope(records):
    """All-real upper hull, clipped exactly to x>0. Each record=(slope,intercept,source)."""
    by_slope={}
    for a,b,src in records:
        if a not in by_slope or b>by_slope[a][0]: by_slope[a]=(b,{src})
        elif b==by_slope[a][0]: by_slope[a][1].add(src)
    hull=[]
    for a,(b,sources) in sorted(by_slope.items()):
        start=None
        while hull:
            pa,pb,_,pstart=hull[-1]
            start=(pb-b)/(a-pa)
            if pstart is None or start>pstart: break
            hull.pop()
        if not hull: start=None
        hull.append((a,b,sources,start))
    clipped=[]
    for k,(a,b,sources,start) in enumerate(hull):
        end=hull[k+1][3] if k+1<len(hull) else None
        if end is not None and end<=0: continue
        clipped.append(dict(a=a,b=b,sources=sources,left=K(0) if start is None or start<0 else start,right=end))
    return clipped

def line_value(line,x): return line['a']*x+line['b']
def certify_envelope(cells,records):
    # Every discarded affine candidate lies below the reported hull everywhere.
    # Affine differences need only endpoint checks (or left endpoint and slope).
    for cell in cells:
        for a,b,_ in records:
            assert line_value(cell,cell['left'])>=a*cell['left']+b
            if cell['right'] is None: assert cell['a']>=a
            else: assert line_value(cell,cell['right'])>=a*cell['right']+b
    for first,second in zip(cells,cells[1:]):
        assert first['right']==second['left']
        assert line_value(first,first['right'])==line_value(second,second['left'])

def encoded_cell(c):
    return dict(left=c['left'].data(),right=None if c['right'] is None else c['right'].data(),
                slope=c['a'].data(),intercept=c['b'].data(),
                attaining_template_support_choices=[list(v) for v in sorted(c['sources'])])

def main():
    z=K(0,1); b=(1+z)/4; w=1+b; R=(5-z)/10
    normals=[(K(1),K(0)),((z-1)/4,(5-z)/4),(-(1+z)/4,(3*z-5)/4),
             (-(1+z)/4,-(3*z-5)/4),((z-1)/4,-(5-z)/4)]
    def rotate(v): return (b*v[0]-(3*z-5)/4*v[1],b*(v[0]+v[1]))
    base=[(-R,K(0)),(R,K(0)),(K(0),R)]; cur=base; triangles=[]
    for j in range(10): triangles.append(cur); cur=[rotate(v) for v in cur]
    assert cur==base
    lam0=[K(1)/(2*w),K(1)/(2*w),K(0),b/w,K(0)]
    lams=[[lam0[(i-l)%5] for i in range(5)] for l in range(5)]
    output=dict(field='Q(sqrt(5))',field_encoding='[a,b] = a+b*sqrt(5); rational strings',
                variable='x=1+s>0',endpoint_policy='Continuous; adjacent formulas agree at finite boundaries.',
                normals=[[v.data() for v in n] for n in normals],
                triangles=[[[v.data() for v in p] for p in tri] for tri in triangles],
                circuits=[[v.data() for v in lam] for lam in lams],denominators=[])
    all_bounds={K(0)}
    for l,lam in enumerate(lams):
        assert sum(lam)==1
        assert all(sum(lam[i]*normals[i][k] for i in range(5))==0 for k in (0,1))
        active=[i for i,v in enumerate(lam) if v>0]
        assert len(active)==3
        records=[]; pertri=[]
        for j,tri in enumerate(triangles):
            rows=[]
            for choice in product(range(3),repeat=3):
                a=sum(lam[i]*normals[i][0]*tri[k][0] for i,k in zip(active,choice))
                c=sum(lam[i]*normals[i][1]*tri[k][1] for i,k in zip(active,choice))
                rows.append((a,c,(j,*choice)))
            assert len(rows)==27
            records.extend(rows)
            cells=envelope(rows)
            certify_envelope(cells,rows)
            # Exact positivity on every closed cell: finite endpoints and final slope.
            assert all(line_value(c,c['left'])>0 for c in cells)
            assert all(line_value(c,c['right'])>0 for c in cells if c['right'] is not None)
            assert cells[-1]['a']>0
            pertri.append(dict(template=j,cells=[encoded_cell(c) for c in cells],
                               value_at_one=max(a+c for a,c,_ in rows).data(),positive_for_x_ge_zero=True))
        cells=envelope(records)
        certify_envelope(cells,records)
        assert max(a+c for a,c,_ in records)==(z-1)/5
        for c in cells:
            all_bounds.add(c['left'])
            # Consequential direct support check at an exact interior sample.
            sample=(c['left']+c['right'])/2 if c['right'] is not None else c['left']+1
            direct=max(sum(lam[i]*max(normals[i][0]*sample*v[0]+normals[i][1]*v[1] for v in tri)
                           for i in range(5)) for tri in triangles)
            assert direct==line_value(c,sample)
        output['denominators'].append(dict(circuit=l,raw_affine_candidates=len(records),
             unique_affine_candidates=len({(a,c) for a,c,_ in records}),
             cells=[encoded_cell(c) for c in cells],templates=pertri,value_at_one=((z-1)/5).data()))
    output['common_refinement_breakpoints']=[v.data() for v in sorted(all_bounds)]
    output['common_refinement_cell_count']=len(all_bounds)
    output['counts']=dict(circuits=5,templates=10,positive_template_denominators=50,
                         raw_candidates=1350,raw_candidates_per_circuit=270,
                         complete_denominator_cells=sum(len(v['cells']) for v in output['denominators']))
    target=Path(__file__).with_suffix('.json')
    target.write_text(json.dumps(output,indent=2)+'\n')
    print(json.dumps({'counts':output['counts'],'common_cells':len(all_bounds),
                      'circuit_cells':[len(v['cells']) for v in output['denominators']],
                      'unique_candidates':[v['unique_affine_candidates'] for v in output['denominators']],
                      'write':str(target)}))
    for l,v in enumerate(output['denominators']):
        print('circuit',l)
        for c in v['cells']: print(c['left'],c['right'],'slope',c['slope'],'intercept',c['intercept'])

if __name__=='__main__': main()
