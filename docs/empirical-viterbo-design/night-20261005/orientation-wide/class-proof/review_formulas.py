"""Independent read-only Cramer derivation of all D-vector fit formulas.

Reads the accepted denominator packet; writes no files. No producer code is
imported. Counts boundary-only representatives separately from open formulas.
"""
import importlib.util
from pathlib import Path
from itertools import combinations
from collections import Counter
import hashlib
import json
import ast
from fractions import Fraction
from math import isqrt

root=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('review_arithmetic',root.parent.parent/'hko-angle-nonconformal'/'review_candidates.py')
m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
E,z,one,r=m.E,m.z,m.one,m.r
source=root.parent/'joint-denominators'/'joint-denominators.json'
assert hashlib.sha256(source.read_bytes()).hexdigest()=='602a61a96420a9f75ceb112d94473e0a7def14a34f467ea0bdf08f47ea88fa7f'
packet=json.loads(source.read_text())
raw=[[list(map(m.decd,c['coefficients'])) for c in s['candidates']] for s in packet['circuits']]
open_inc={};closed_inc={}
for si,s in enumerate(packet['joint']['topology_strips']):
    for ti,t in enumerate(s['substrips']):
        for ci,c in enumerate(t['cells']):
            key=tuple(v[0] for v in c['formula_candidates'])
            open_inc.setdefault(key,[]).append((si,ti,ci))
for fi,f in enumerate(packet['joint']['closed_fibers']):
    for ci,c in enumerate(f['cells']):
        key=tuple(v['candidate_ids'][0] for v in c['vector'])
        closed_inc.setdefault(key,[]).append((fi,ci))
bases=list(combinations(range(5),3))
edges=[]
for i in range(5):
    row=[z]*5;row[i]=-2*m.a;row[(i-1)%5]=one;row[(i+1)%5]=one;edges.append(row)
allrows=m.lams+edges


def trim(p):
    while len(p)>1 and p[-1]==z:p.pop()
    return p


def add(p,q):
    return trim([(p[i] if i<len(p) else z)+(q[i] if i<len(q) else z) for i in range(max(len(p),len(q)))])


def scale(p,a):
    return trim([v*a for v in p])


def mul(p,q):
    out=[z]*(len(p)+len(q)-1)
    for i,a in enumerate(p):
        for j,b in enumerate(q):out[i+j]+=a*b
    return trim(out)


def degree(p):
    p=trim(p[:]);return -1 if p==[z] else len(p)-1


def derive(key,ids):
    ds=[raw[l][i] for l,i in enumerate(key)]
    mat=[m.lams[i][:3] for i in ids]
    assert m.det(mat)!=z
    columns=[m.solve(mat,[ds[i][power] for i in ids]) for power in range(4)]
    h=[[columns[p][j] for p in range(4)] for j in range(3)]+[[z]*4,[z]*4]
    residuals=[[sum((row[j]*h[j][p] for j in range(5)),z)-(ds[i][p] if i<5 else z)
                for p in range(4)] for i,row in enumerate(allrows)]
    assert all(residuals[i]==[z]*4 for i in ids)
    q=[[z]*3 for _ in range(3)]
    for i in range(5):
        for xp in range(2):
            for yp in range(2):
                for up in range(2):
                    for vp in range(2):
                        j=xp+2*up;k=yp+2*vp
                        q[xp+yp][up+vp]+=h[i][j]*h[(i+1)%5][k]-m.a*h[i][j]*h[i][k]
    target=one/(E(3)+r);tau2=E(5)-2*r
    q[1][0]-=target;q[1][2]-=target*tau2
    return h,residuals,[trim(v) for v in q]


support_keys=set();open_support_keys=set();zero_edge_keys=set();zero_edge_occurrences=0
degree_counts=Counter();station_degree=Counter();endpoint_degree=Counter();membership_degree=Counter()
formula_count=0;open_formula_count=0
for key in sorted(set(open_inc)|set(closed_inc)):
    for ids in bases:
        h,res,q=derive(key,ids);formula_count+=1
        support_keys.add(tuple(v.key() for coord in h for v in coord))
        if key not in open_inc:continue
        open_formula_count+=1
        support_key=tuple(v.key() for coord in h for v in coord)
        open_support_keys.add(support_key)
        if any(coeff==[z]*4 for coeff in res[5:]):
            zero_edge_keys.add(support_key);zero_edge_occurrences+=1
        for p in q:degree_counts[degree(p)]+=1
        stationary=add(scale(mul(q[0],q[2]),E(4)),scale(mul(q[1],q[1]),-one))
        station_degree[degree(stationary)]+=1
        for coeff in res:
            c,a=trim([coeff[0],coeff[2]]),trim([coeff[1],coeff[3]])
            if degree(a)<0:continue
            endpoint=add(add(mul(q[0],mul(a,a)),scale(mul(q[1],mul(c,a)),-one)),mul(q[2],mul(c,c)))
            membership=add(scale(mul(q[1],a),-one),scale(mul(q[2],c),E(2)))
            endpoint_degree[degree(endpoint)]+=1;membership_degree[degree(membership)]+=1
assert len(open_inc)==21 and sum(map(len,open_inc.values()))==268
assert sum(map(len,closed_inc.values()))==258

# Consequential producer-coefficient comparison, separately derived above.
produced=json.loads((root/'formulas.json').read_text())
assert produced['input_sha256']==hashlib.sha256(source.read_bytes()).hexdigest()
vector_keys=[]
for ds in produced['vectors']:
    key=tuple(raw[l].index(list(map(m.decd,coeff))) for l,coeff in enumerate(ds))
    assert key in open_inc;vector_keys.append(key)
assert set(vector_keys)==set(open_inc) and len(vector_keys)==21
decode_poly=lambda p:trim(list(map(m.decd,p)))
def flatten(coordinate):
    c,a=map(decode_poly,coordinate)
    return [c[0],a[0],c[1] if len(c)>1 else z,a[1] if len(a)>1 else z]
def check_ref(ref,p):
    if p==[z]:
        assert ref=={'zero':True}
    else:
        original=scale(decode_poly(produced['polynomials'][ref['polynomial']]['coefficients']),m.decd(ref['multiplier']))
        assert p==original
assert len(produced['formulas'])==210
for f in produced['formulas']:
    key=vector_keys[f['vector']];ids=tuple(f['fit_triple'])
    h,res,q=derive(key,ids)
    assert h==[flatten(coord) for coord in f['supports']]
    assert res==[flatten(coord) for coord in f['residuals']]
    assert q==list(map(decode_poly,f['gap']))
    area=q[:];area[1]=add(area[1],[one/(E(3)+r),z,(E(5)-2*r)/(E(3)+r)])
    assert area==list(map(decode_poly,f['area']))
    expected_bounds=[]
    for ri,coeff in enumerate(res):
        c,a=trim([coeff[0],coeff[2]]),trim([coeff[1],coeff[3]])
        if a==[z]:continue
        expected_bounds.append(ri)
        record=next(v for v in f['bounds'] if v['residual']==ri)
        assert decode_poly(record['numerator'])==scale(c,-one) and decode_poly(record['denominator'])==a
        endpoint=add(add(mul(q[0],mul(a,a)),scale(mul(q[1],mul(c,a)),-one)),mul(q[2],mul(c,c)))
        membership=add(scale(mul(q[1],a),-one),scale(mul(q[2],c),E(2)))
        check_ref(record['endpoint_gap'],endpoint);check_ref(record['stationary_membership'],membership)
    assert sorted(expected_bounds)==sorted(v['residual'] for v in f['bounds'])
    check_ref(f['stationary_gap'],add(scale(mul(q[0],q[2]),E(4)),scale(mul(q[1],q[1]),-one)))
    for i in range(3):check_ref(f['q'+str(i)],q[i])
assert len(produced['open_domains'])==268 and len(produced['closed_domains'])==258
for rec in produced['open_domains']:
    si,ti,ci=rec['incidence'];cell=packet['joint']['topology_strips'][si]['substrips'][ti]['cells'][ci]
    assert vector_keys[rec['vector']]==tuple(v[0] for v in cell['formula_candidates'])

# Load only definitions from the independently reviewed quadratic-fiber adapter.
names={'rational_sqrt','is_field_square','Root','root_decode','V','value_decode','compare'}
tree=ast.parse((root.parent/'joint-denominators'/'review_joint.py').read_text())
selected=[node for node in tree.body if isinstance(node,(ast.FunctionDef,ast.ClassDef)) and node.name in names]
ns={'E':E,'z':z,'dec':m.decd,'Fraction':Fraction,'isqrt':isqrt,'registry':{}}
exec(compile(ast.Module(body=selected,type_ignores=[]),'independent_review_field_definitions','exec'),ns)
V,decode_value=ns['V'],ns['value_decode']
closed_matches={}
for rec in produced['closed_domains']:
    fi,ci=rec['incidence'];fiber=packet['joint']['closed_fibers'][fi];cell=fiber['cells'][ci]
    u=decode_value(fiber['u']);actual=[raw[l][v['candidate_ids'][0]] for l,v in enumerate(cell['vector'])]
    assert rec['vectors'];closed_matches[fi,ci]=rec['vectors']
    for vi in rec['vectors']:
        ds=[raw[l][i] for l,i in enumerate(vector_keys[vi])]
        for expected,observed in zip(ds,actual):
            assert V(expected[0])+u*expected[2]==V(observed[0])+u*observed[2]
            assert V(expected[1])+u*expected[3]==V(observed[1])+u*observed[3]
point_incidence_checks=0
for fi,fiber in enumerate(packet['joint']['closed_fibers']):
    u=decode_value(fiber['u'])
    for pi,point in enumerate(fiber['point_vectors']):
        x=decode_value(point['x'])
        for ci in ([pi-1,pi] if pi else [0]):
            for vi in closed_matches[fi,ci]:
                ds=[raw[l][i] for l,i in enumerate(vector_keys[vi])]
                for coeff,observed in zip(ds,point['vector']):
                    assert (V(coeff[1])+u*coeff[3])*x+V(coeff[0])+u*coeff[2]==decode_value(observed['value'])
                point_incidence_checks+=1
print(json.dumps({'status':'PASS independent formula/residual derivation',
    'open_D_vectors':len(open_inc),'closed_first_label_D_vectors':len(closed_inc),
    'union_first_label_D_vectors':len(set(open_inc)|set(closed_inc)),
    'open_support_formulas':open_formula_count,'all_first_label_support_formulas':formula_count,
    'distinct_support_formulas':len(support_keys),'open_D_cell_incidences':268,'closed_D_cell_incidences':258,
    'distinct_open_support_formulas':len(open_support_keys),
    'zero_edge_open_formula_classes':len(zero_edge_keys),'zero_edge_open_formula_occurrences':zero_edge_occurrences,
    'producer_formulas_residuals_signed_polynomial_references':'PASS',
    'closed_embedding_slope_intercept_equivalences':'PASS','closed_point_incidence_checks':point_incidence_checks,
    'open_fit_basis_incidences':2680,'closed_fit_basis_incidences':2580,
    'x_gap_coefficient_u_degree_histogram':dict(degree_counts),
    'stationary_numerator_degree_histogram':dict(station_degree),
    'raw_residual_endpoint_numerator_degrees':dict(endpoint_degree),
    'raw_stationary_membership_numerator_degrees':dict(membership_degree),
    'writes':False}))
