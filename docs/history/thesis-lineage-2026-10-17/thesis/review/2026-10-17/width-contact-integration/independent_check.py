#!/usr/bin/env python3
"""Independent rational geometry and direct 4D HK check; imports no selector code."""
import argparse
import hashlib
import itertools
import json
import math
from fractions import Fraction as F
from pathlib import Path


def dot(x, y):
    return sum(a*b for a,b in zip(x,y))


def cross(x,y):
    return x[0]*y[1]-x[1]*y[0]


def hull(points):
    points=sorted(set(points))
    def half(seq):
        out=[]
        for p in seq:
            while len(out)>1 and cross(tuple(out[-1][i]-out[-2][i] for i in range(2)),tuple(p[i]-out[-1][i] for i in range(2)))<=0:
                out.pop()
            out.append(p)
        return out
    return half(points)[:-1]+half(reversed(points))[:-1]


def geometry(normals):
    v=[]
    for a,b in itertools.combinations(normals,2):
        determinant=cross(a,b)
        if not determinant:
            continue
        point=((b[1]-a[1])/determinant,(a[0]-b[0])/determinant)
        if all(dot(n,point)<=1 for n in normals):
            v.append(point)
    v=hull(v)
    assert len(v)>=3
    area=sum(cross(a,b) for a,b in zip(v,v[1:]+v[:1]))/2
    assert area>0
    anchors=[]
    for n in normals:
        t=1/(1+max(-dot(n,p) for p in v))
        anchors.append((t,tuple(t*z for z in n)))
    return v,area,anchors


def omega(x,y):
    return x[0]*y[2]+x[1]*y[3]-x[2]*y[0]-x[3]*y[1]


def objective(normals,weights,word):
    return sum(weights[i]*weights[j]*omega(normals[i],normals[j]) for k,i in enumerate(word) for j in word[k+1:])


def check_witness(row,all_normals,k):
    witness=row['selection']['best']
    pi,qi=witness['supports']
    normals=[all_normals[i] for i in pi]+[all_normals[k+j] for j in qi]
    weights=list(map(F,witness['weights']))
    assert len(weights)==len(normals)
    assert min(weights)>0
    assert sum(weights[:len(pi)])==sum(weights[len(pi):])==F(1,2)
    assert all(sum(w*n[c] for w,n in zip(weights,normals))==0 for c in range(4))
    word=witness['word']
    assert sorted(word)==list(range(len(normals)))
    value=objective(normals,weights,word)
    assert value==F(witness['Q'])>0
    assert F(witness['capacity'])==1/(2*value)
    # A finite verification of the chosen pair only, not a support search.
    values=[objective(normals,weights,(0,)+tail) for tail in itertools.permutations(range(1,len(normals)))]
    assert value==max(values)
    return F(witness['capacity']),len(values)


def main():
    ap=argparse.ArgumentParser()
    ap.add_argument('--source',type=Path,required=True)
    ap.add_argument('--results',type=Path,required=True)
    ap.add_argument('--out',type=Path,required=True)
    ap.add_argument('--falsifier',type=Path)
    args=ap.parse_args()
    report=json.loads(args.results.read_text())
    assert hashlib.sha256(args.source.read_bytes()).hexdigest()==report['source_sha256']
    rows={r['name']:r for r in report['rows']}
    expected_count=len(rows)
    checked=[]
    for line in args.source.open():
        original=json.loads(line)
        if original['name'] not in rows:
            continue
        r=rows.pop(original['name']); k=original['k']
        normals=[tuple(map(F,n)) for n in original['dual_vertices_rational']]
        p=[n[:2] for n in normals[:k]]; q=[n[2:] for n in normals[k:]]
        pv,pa,px=geometry(p); qv,qa,qx=geometry(q)
        products={(i,j):abs(dot(x[1],y[1])) for i,x in enumerate(px) for j,y in enumerate(qx)}
        maximum=max(products.values())
        contacts=sorted(ij for ij,v in products.items() if v==maximum)
        assert contacts==[tuple(v) for v in r['selection']['anchor_contacts']]
        assert F(r['selection']['width_capacity'])==1/maximum
        assert pa*qa==F(r['exact_original_area_product'])
        cap,orders=check_witness(r,normals,k)
        assert cap<=1/maximum
        pi,qi=r['selection']['best']['supports']
        w=list(map(F,r['selection']['best']['weights']))
        # Contact anchors really occur with their LP-maximal weight.
        assert any(i in pi and j in qi and 2*w[pi.index(i)]==px[i][0] and 2*w[len(pi)+qi.index(j)]==qx[j][0] for i,j in contacts)
        assert F(r['exact_ratio_upper_bound'])==cap*cap/(2*pa*qa)
        assert abs(r['ratio_upper_bound']-float(cap*cap/(2*pa*qa)))<1e-14
        assert r['historical_capacity']==original['capacity'] and r['historical_ratio']==original['sys']
        assert abs(r['capacity_over_historical']-float(cap)/original['capacity'])<1e-14
        checked.append({'name':r['name'],'word_orders_checked':orders,'capacity_over_historical':r['capacity_over_historical'],'exact_gain_over_width':str(1-cap*maximum)})
    assert not rows and len(checked)==expected_count
    # Independent area is the unit-circumradius regular pentagon area.
    area=5*math.sin(2*math.pi/5)/2
    regular=[]
    for r in report.get('regular_controls',[]):
        theta=r['angle']; expected=(1+math.cos(math.pi/5))**2/math.cos(theta)
        ratio=expected**2/(2*area**2)
        assert abs(r['known_capacity']/expected-1)<1e-12
        assert abs(r['known_ratio']-ratio)<1e-12
        assert abs(r['selected']['best']['capacity']/expected-1)<1e-10
        regular.append({'angle':theta,'independent_ratio':ratio})
    if regular:
        assert abs(regular[-1]['independent_ratio']-(3+math.sqrt(5))/5)<1e-12
    falsifier=None
    if args.falsifier:
        f=json.loads(args.falsifier.read_text())
        body=None
        for line in args.source.open():
            item=json.loads(line)
            if item['name']==f['name']:
                body=item
                break
        assert body
        n=[tuple(map(F,v)) for v in body['dual_vertices_rational']]
        challenger,orders=check_witness({'selection':{'best':f['feasible_challenger']}},n,body['k'])
        selector=F(f['selector_capacity'])
        gap=selector/challenger
        assert gap==F(f['exact_lower_bound_selector_over_true_capacity']) and gap>F(109,100)
        assert abs(float(gap)-1-f['relative_gap_lower_bound'])<1e-14
        falsifier={'name':f['name'],'exact_selector_over_feasible_challenger':str(gap),'guaranteed_capacity_gap':float(gap)-1,'word_orders_checked':orders,'scope':'True capacity is at most the independently verified challenger bound; no true-capacity optimum computed.'}
    envelope=[]
    for r in report['rows']:
        ratio=F(r['exact_ratio_upper_bound'])
        z=5*ratio-3
        if z>0 and z*z>5:
            envelope.append({'name':r['name'],'ratio_upper_bound':float(ratio),'strictly_above_HKO_by_exact_squaring':True})
    result={'status':'PASS','scope':'Independent all-intersection rational convex hull, LP anchor formula and maximal-contact check; direct 4D HK chosen-word verification and chosen-pair order optimum. Numerical regular controls use independent analytic area when present. No original-body capacity optimality claim.','rows':checked,'regular_controls':regular,'falsifier':falsifier,'bound_above_HKO':envelope,'results_sha256':hashlib.sha256(args.results.read_bytes()).hexdigest()}
    args.out.write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps({'status':'PASS','rational_rows':len(checked),'direct_4d_orders':sum(r['word_orders_checked'] for r in checked),'regular_controls':len(regular)}))


if __name__=='__main__':
    main()
