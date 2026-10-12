#!/usr/bin/env python3
"""Replay the ridge concentration discovery and exact sharpness controls.

Reads existing historical summary rows; makes no geometry/capacity calls.
The inverse-Simpson count here is NOT the repository's Shannon effective count.
"""
import argparse
import hashlib
import itertools
import json
import math
from collections import defaultdict
from fractions import Fraction
from pathlib import Path


def omega(a, b):
    return a[0] * b[2] + a[1] * b[3] - a[2] * b[0] - a[3] * b[1]


def simplex_areas(epsilon):
    z = Fraction(0)
    o = Fraction(1)
    vertices = [(z,z,z,z),(o,z,z,z),(z,z,o,z),
                (z,epsilon,z,z),(z,z,z,epsilon)]
    areas = []
    for i,j,k in itertools.combinations(range(5), 3):
        a = [x-y for x,y in zip(vertices[j], vertices[i])]
        b = [x-y for x,y in zip(vertices[k], vertices[i])]
        areas.append(abs(omega(a,b))/2)
    return areas


def rectangle_product_areas(epsilon):
    edges = [(Fraction(1),0),(0,epsilon),(-Fraction(1),0),(0,-epsilon)]
    # Eight same-factor ridges carry zero symplectic area.
    return [abs(a[0]*b[0]+a[1]*b[1]) for a in edges for b in edges] + [Fraction(0)]*8


def exact_controls():
    result = []
    for n in (1,2,10,100):
        e = Fraction(1,n)
        for family, fn, g in [('simplex',simplex_areas,3),
                              ('rectangle_product',rectangle_product_areas,4)]:
            areas = fn(e)
            total = sum(areas)
            square_sum = sum(x*x for x in areas)
            n2 = total*total/square_sum
            predicted = g*(1+e*e)**2/(1+e**4)
            assert n2 == predicted
            assert n2 > g
            result.append({'family':family,'epsilon':str(e),
                           'areas':list(map(str,areas)),
                           'inverse_simpson_count_exact':str(n2),
                           'inverse_simpson_count':float(n2),
                           'max_share_exact':str(max(areas)/total)})
    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--historical',type=Path,required=True)
    parser.add_argument('--out',type=Path,required=True)
    args = parser.parse_args()
    data = args.historical.read_bytes()
    groups = defaultdict(list)
    for line in data.splitlines():
        row = json.loads(line)
        assert row['capacity_source'] in ('random_product_sample','random_sample')
        family = 'product' if row['capacity_source'] == 'random_product_sample' else 'generic'
        n = row['ridge_symp_area_ordered_face_count']
        assert n == row['ridge_count']
        assert row['ridge_symp_area_ordering_failure_count'] == 0
        mean = row['ridge_symp_area_mean_over_volume_sqrt']
        std = row['ridge_symp_area_std_over_volume_sqrt']
        total = row['ridge_symp_area_sum_over_volume_sqrt']
        square_sum = n*(mean*mean+std*std)
        assert total > 0 and square_sum > 0
        assert math.isclose(total,n*mean,rel_tol=1e-9)
        groups[family].append({'poly_id':row['poly_id'],
                              'inverse_simpson_count':total*total/square_sum,
                              'max_share':row['ridge_symp_area_max_share']})
    summaries = {}
    for family, rows in groups.items():
        g = 4 if family == 'product' else 3
        summaries[family] = {
            'rows':len(rows),'universal_infimum':g,
            'minimum_inverse_simpson_witness':min(rows,key=lambda x:x['inverse_simpson_count']),
            'maximum_share_witness':max(rows,key=lambda x:x['max_share']),
            'inverse_simpson_violations':sum(x['inverse_simpson_count'] < g-1e-7 for x in rows),
            'max_share_violations':sum(x['max_share'] > 1/g+1e-7 for x in rows)}
    output = {'schema':'ridge-circulation-check-v1',
              'historical_sha256':hashlib.sha256(data).hexdigest(),
              'historical_path':str(args.historical),
              'method':'N2 = sum(area)^2 / [ridge_count*(mean(area)^2+population_std(area)^2)]',
              'claim_status':'theorem argument separately independently audited; numerical data are checks',
              'source_target_values_used':False,'summaries':summaries,
              'exact_sharpness_controls':exact_controls()}
    args.out.parent.mkdir(parents=True,exist_ok=True)
    args.out.write_text(json.dumps(output,indent=2)+'\n')
    print(json.dumps(summaries,indent=2))


if __name__ == '__main__':
    main()
