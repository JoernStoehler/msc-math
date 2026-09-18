#!/usr/bin/env python3
"""Exact witness: a lifted planar orbit persists after losing global minimality.

No general capacity computation is needed. One feasible HK datum strictly beats
the pure-A value, and a rational translated containment proves the orbit exists.
The dependence on k is handled algebraically in lift-persistence.md.
"""
from fractions import Fraction as F
from itertools import combinations
import json
from pathlib import Path
from exact_endpoint_collision import dual, det, dot

def solve2(rows, rhs):
    d=det(rows[0],rows[1])
    return ((rhs[0]*rows[1][1]-rows[0][1]*rhs[1])/d,
            (rows[0][0]*rhs[1]-rhs[0]*rows[1][0])/d)

def witness(k):
    A=[(F(2),F(0)),(F(-1),F(2)),(F(-1),F(-2))]
    B=[(k*(4*x-3*y)/5,k*(3*x+4*y)/5) for x,y in A]
    ad,bd=dual(A),dual(B)
    word=(0,3,2,5,1,4)
    x=y=z=F(0)
    for i,j in combinations(word,2):
        if i<3 and j<3: x+=F(2,9)*det(ad[i],ad[j])
        elif i>=3 and j>=3: y-=F(2,9)*det(bd[i-3],bd[j-3])
        elif i<3: z+=F(2,9)*dot(ad[i],bd[j-3])
        else: z-=F(2,9)*dot(ad[j],bd[i-3])
    assert (x,y,z)==(-F(1,6),1/(6*k*k),2/(3*k))
    tangent=F(4,5)
    image=[(tangent*v,-tangent*u) for u,v in A]
    heights=[max(dot(n,p) for p in image) for n in bd]
    fit_radius=sum(heights)/3
    shift=solve2(bd[:2],[fit_radius-h for h in heights[:2]])
    translated=[tuple(p[i]+shift[i] for i in range(2)) for p in image]
    support_values=[[dot(n,p) for p in translated] for n in bd]
    assert fit_radius==F(24,25)/k < 1
    assert all(max(row)==fit_radius for row in support_values)
    t=F(1,5)
    q=x*t*t+y*(1-t)**2+tangent*z*t*(1-t)
    assert q==-F(1,150)+F(8,75)/(k*k)+F(32,375)/k > F(1,6)
    weights=[t/3]*3+[(1-t)/3]*3
    assert sum(weights)==1 and min(weights)>0
    assert all(sum(weights[j]*ad[j][i] for j in range(3))==0 for i in range(2))
    assert all(sum(weights[j+3]*bd[j][i] for j in range(3))==0 for i in range(2))
    return dict(k=k,vertices_a=A,vertices_b=B,area_a=F(6),area_b=6*k*k,
        tangent_theta=tangent,fit_radius=fit_radius,fit_translation=shift,
        translated_lift_vertices=translated,fit_facet_values=support_values,
        hk_word=word,hk_weights=weights,within_a=x,within_b=y,cross=z,
        inverse_action_divided_by_cos=q,pure_a_inverse_divided_by_cos=F(1,6),
        relative_inverse_action_gain=6*q-1)

def encode(x):
    if isinstance(x,F): return str(x)
    raise TypeError(type(x).__name__)

if __name__=='__main__':
    cases=[witness(F(1)),witness(F(21,20))]
    assert 65*F(21,20)**2-32*F(21,20)-40 < 0
    # This polynomial is increasing on k>=1 (derivative130k-32>0), so the
    # endpoint verifies the strict action inequality on [1,21/20].
    payload=dict(status='exact rational feasible-HK upper bound plus strict geometric containment',
        parameter_interval='1 < k <= 21/20 gives area(A)<area(B)',
        proof='coefficient k-dependence and interval inequalities in lift-persistence.md',cases=cases)
    Path(__file__).with_name('lift-persistence.json').write_text(json.dumps(payload,default=encode,indent=2)+'\n')
    print(json.dumps([{key:c[key] for key in ['k','fit_radius','inverse_action_divided_by_cos','relative_inverse_action_gain']} for c in cases],default=encode,indent=2))
