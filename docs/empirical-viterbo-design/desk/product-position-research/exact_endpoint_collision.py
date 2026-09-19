#!/usr/bin/env python3
"""Rational complete HK certificate for two endpoint-matched product responses.

At theta=pi/4, J=(J0+JL)/sqrt(2). All arithmetic except the final human decimal
display is Fraction arithmetic. Coverage: all 120 cyclic six-facet orderings
and exact maximization over the unique closure mass t in [0,1].
"""
from fractions import Fraction as F
from itertools import permutations, combinations
import json
from pathlib import Path

def dual(v):
    assert all(sum(x[k] for x in v) == 0 for k in range(2))
    ans = []
    for p, q in zip(v, v[1:] + v[:1]):
        n = (q[1]-p[1], p[0]-q[0])
        h = sum(n[k]*p[k] for k in range(2))
        ans.append(tuple(x/h for x in n))
    assert all(sum(x[k] for x in ans) == 0 for k in range(2))
    assert all(sum(n[k]*p[k] for k in range(2)) <= 1 for n in ans for p in v)
    return ans

def det(x,y): return x[0]*y[1]-x[1]*y[0]
def dot(x,y): return x[0]*y[0]+x[1]*y[1]

def maximize(x,y,z):
    candidates=[(x,F(1)),(y,F(0))]
    q=x+y-z
    if q < 0:
        t=(2*y-z)/(2*q)
        if 0<t<1:
            candidates.append((x*t*t+y*(1-t)**2+z*t*(1-t),t))
    return max(candidates)

def solve(shear):
    a=[(F(-1),F(-1)),(F(2),F(-1)),(F(-1),F(2))]
    b=[(2*x+shear*y,y/2) for x,y in a]
    ad,bd=dual(a),dual(b)
    branches=[]
    for tail in permutations(range(1,6)):
        word=(0,)+tail
        x=y=z=F(0)
        for i,j in combinations(word,2):
            if i<3 and j<3: x+=F(2,9)*det(ad[i],ad[j])
            elif i>=3 and j>=3: y-=F(2,9)*det(bd[i-3],bd[j-3])
            elif i<3: z+=F(2,9)*dot(ad[i],bd[j-3])
            else: z-=F(2,9)*dot(ad[j],bd[i-3])
        assert abs(x)==F(2,9) and abs(y)==F(2,9)
        v0,t0=maximize(x,y,F(0))
        vl,tl=maximize(F(0),F(0),z)
        vm,tm=maximize(x,y,z)
        branches.append(dict(word=word, within_a=x,within_b=y,cross=z,
            inverse0=v0,inverse_l=vl,inverse_mid_times_sqrt2=vm,mass_mid=tm))
    best0=max(r['inverse0'] for r in branches)
    bestl=max(r['inverse_l'] for r in branches)
    bestm=max(r['inverse_mid_times_sqrt2'] for r in branches)
    return dict(body_id='right_triangle_product_shear_'+str(shear),
        factor_a_vertices=a,factor_b_vertices=b,area_a=F(9,2),area_b=F(9,2),
        capacity0=1/best0,capacity_l=1/bestl,capacity_mid_over_sqrt2=1/bestm,
        mid_winners=[r for r in branches if r['inverse_mid_times_sqrt2']==bestm],
        branches=branches)

def encode(obj):
    if isinstance(obj,F): return str(obj)
    raise TypeError(type(obj).__name__)

if __name__=='__main__':
    cases=[solve(F(0)),solve(F(1,2))]
    assert all(r['capacity0']==F(9,2) and r['capacity_l']==F(18,5) for r in cases)
    assert [r['capacity_mid_over_sqrt2'] for r in cases]==[F(18,7),F(3)]
    # Every cross coefficient is affine in shear s (determinant of the factor
    # transformation is one). Endpoint checks therefore certify these bounds
    # throughout [0,1/2], not merely at sampled s values.
    for shear, case in zip([F(0), F(1,2)], cases):
        for r in case['branches']:
            assert r['cross'] <= F(10,9)
            if r['within_a'] == r['within_b'] == F(2,9):
                assert r['cross'] <= (10-4*shear)/9
    pure_cross_witness = []
    positive_joint_witness = []
    for r0, r1 in zip(cases[0]['branches'], cases[1]['branches']):
        assert r0['word'] == r1['word']
        if r0['cross'] == r1['cross'] == F(10,9):
            pure_cross_witness.append(r0['word'])
        if r0['within_a'] == r0['within_b'] == r1['within_a'] == r1['within_b'] == F(2,9) and r0['cross'] == F(10,9) and r1['cross'] == F(8,9):
            positive_joint_witness.append(r0['word'])
    assert pure_cross_witness and positive_joint_witness
    first_transition_word=(0,3,1,4,2,5)
    for case in cases:
        r=next(r for r in case['branches'] if r['word']==first_transition_word)
        assert (r['within_a'],r['within_b'],r['cross'])==(F(2,9),-F(2,9),F(10,9))
    # Mixed-sign branches are <=29/90, negative-negative <=1/6, while the
    # positive-positive envelope is (7-2s)/18 >=1/3 over the entire interval.
    assert maximize(F(2,9),-F(2,9),F(10,9))[0] == F(29,90) < F(1,3)
    assert maximize(-F(2,9),-F(2,9),F(10,9))[0] == F(1,6) < F(1,3)
    payload=dict(status='exact-rational-complete-HK-reduction; mathematical coverage explained in README',
        normalization='theta=pi/4: capacity=sqrt(2)*capacity_mid_over_sqrt2',
        continuum_certificate=dict(parameter_interval='0 <= s <= 1/2',
            capacity0='9/2',capacity_l='18/5',capacity_mid='18*sqrt(2)/(7-2*s)',
            exact_first_transition='arctan(2/5)',first_transition_word=first_transition_word,
            pure_cross_witness=pure_cross_witness[0],positive_joint_witness=positive_joint_witness[0],
            coverage='cross coefficients affine in s; exact endpoint inequalities extend to full interval'),cases=cases)
    Path(__file__).with_name('exact-endpoint-collision.json').write_text(json.dumps(payload,default=encode,indent=2)+'\n')
    print(json.dumps([{k:v for k,v in r.items() if k not in ['branches','mid_winners']} for r in cases],default=encode,indent=2))
