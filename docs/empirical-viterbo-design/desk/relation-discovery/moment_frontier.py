#!/usr/bin/env python3
"""A finite-data moment inequality and an explicit adverse convex family.

No capacity calculations. Imports only the partner owner's pure moment routine.
Run with PYTHONDONTWRITEBYTECODE=1 to avoid writing into the other owner's tree.
"""
import argparse
import hashlib
import importlib.util
import json
import math
import sys
from fractions import Fraction as F
from pathlib import Path

import numpy as np


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def radial_moments(a):
    """Exact polar integrals for r(theta)=1+a*cos(3 theta); π cancels."""
    area_factor = 1+a*a/2
    radial2 = F(1,2)*(1+3*a*a+F(3,8)*a**4)/area_factor
    radial4 = F(1,3)*(1+F(15,2)*a*a+F(45,8)*a**4+F(5,16)*a**6)/area_factor
    harmonic3 = a*(1+F(3,2)*a*a+a**4/8)/area_factor
    beta4 = 4*radial4/radial2**2
    beta3 = 2*harmonic3**2/radial2**3
    slack = beta4-F(16,3)-F(2,3)*beta3
    return {'a':str(a),'beta4_exact':str(beta4),'beta3_norm2_exact':str(beta3),
            'slack_exact':str(slack),'beta4':float(beta4),'beta3_norm2':float(beta3),
            'slack':float(slack),
            'curvature_numerator_lower_bound_exact':str((1-a)*(1-10*a))}


def main():
    p = argparse.ArgumentParser()
    p.add_argument('--collection',type=Path,required=True)
    p.add_argument('--moment-code',type=Path,required=True)
    p.add_argument('--out',type=Path,required=True)
    args = p.parse_args()
    rows = [json.loads(x) for x in args.collection.read_text().splitlines()]
    evidence=[]
    for row in rows:
        v=row['scalars']
        evidence.append({'body_id':row['body_id'],'name':row['name'],
                         'parent_id':row['parent_id'],
                         'slack':v['affine_radial_fourth_moment']-16/3-
                                 2/3*v['affine_skew_tensor_norm2']})
    slope_frontier=[]
    for row in rows:
        v=row['scalars']
        if v['affine_skew_tensor_norm2'] > 1e-12:
            slope_frontier.append({'body_id':row['body_id'],'name':row['name'],
                'slope':(v['affine_radial_fourth_moment']-16/3)/v['affine_skew_tensor_norm2']})
    exact=[radial_moments(F(1,n)) for n in (100,40,20,11)]
    assert all(F(x['slack_exact']) < 0 for x in exact)
    assert all(F(x['curvature_numerator_lower_bound_exact']) > 0 for x in exact)
    assert radial_moments(F(0))['beta4_exact'] == '16/3'

    sys.path.insert(0,str(args.moment_code.parent.resolve()))
    spec=importlib.util.spec_from_file_location('partner_moment_source',args.moment_code)
    module=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    polygons=[]
    polygon_geometry=[]
    a=F(1,20)
    target=radial_moments(a)
    for n in (96,384,1536):
        theta=np.arange(n)*2*np.pi/n
        radius=1+float(a)*np.cos(3*theta)
        vertices=np.column_stack((radius*np.cos(theta),radius*np.sin(theta)))
        m=module.moments(vertices)
        b3=m['affine_skew_tensor_norm2']; b4=m['affine_radial_fourth_moment']
        polygons.append({'body_id':hashlib.sha256(vertices.tobytes()).hexdigest(),
                         'construction':'r=1+(1/20)cos(3theta), theta=2pi*j/n',
                         'n':n,'beta4':b4,'beta3_norm2':b3,
                         'slack':b4-16/3-2/3*b3,
                         'beta4_error_vs_smooth':abs(b4-target['beta4']),
                         'beta3_error_vs_smooth':abs(b3-target['beta3_norm2'])})
        if n == 96:
            polygon_geometry.append({'body_id':polygons[-1]['body_id'],
                'measurement_version':'pure-third-harmonic-polygon-v1',
                'construction':polygons[-1]['construction'],'a_exact':'1/20',
                'vertices':vertices.tolist(),
                'numerical_status':'rounded trigonometric vertices; f64 integrated moments; no capacity'})
    assert polygons[-1]['slack'] < -4e-5
    assert polygons[-1]['beta4_error_vs_smooth'] < polygons[0]['beta4_error_vs_smooth']
    assert polygons[-1]['beta3_error_vs_smooth'] < polygons[0]['beta3_error_vs_smooth']

    zero_skew_asymmetric=[]
    for row in rows:
        v=row['scalars']
        if v['affine_skew_tensor_norm2']<1e-20 and v['difference_area_excess']>1e-3:
            zero_skew_asymmetric.append({'body_id':row['body_id'],'name':row['name'],
                'skew_norm2':v['affine_skew_tensor_norm2'],
                'difference_area_excess':v['difference_area_excess']})
    output={'schema':'moment-frontier-adverse-example-v1',
        'candidate':'beta4 >= 16/3 + (2/3)*beta3_norm2',
        'candidate_status':'FALSE, despite surviving the supplied finite collection',
        'candidate_origin':'exploratory cross-view inequality; coefficient suggested by third-harmonic disk perturbation, not predeclared',
        'collection_path':str(args.collection),'collection_sha256':digest(args.collection),
        'moment_code_sha256':digest(args.moment_code),'moment_code_path':str(args.moment_code),
        'collection_count':len(rows),'base_count':sum(r['parent_id'] is None for r in rows),
        'collection_minimum_slack':min(x['slack'] for x in evidence),
        'collection_violations':sum(x['slack'] < -1e-10 for x in evidence),
        'finite_maximum_slope_witness':min(slope_frontier,key=lambda x:x['slope']),
        'slope_frontier_note':'finite upperlimit for alpha in beta4>=16/3+alpha*beta3; rows with beta3<=1e-12 excluded from division',
        'collection_checks':evidence,'exact_smooth_counterexamples':exact,
        'polygon_approximation_checks':polygons,
        'zero_third_moment_noncentral_examples':zero_skew_asymmetric,
        'capacity_calls':0}
    args.out.write_text(json.dumps(output,indent=2)+'\n')
    geometry_path=args.out.with_name('moment-adverse-polygons.jsonl')
    geometry_path.write_text(''.join(json.dumps(x)+'\n' for x in polygon_geometry))
    print(json.dumps({'collection_minimum_slack':output['collection_minimum_slack'],
                      'exact_counterexample':target,'polygon_checks':polygons},indent=2))


if __name__=='__main__':
    main()
