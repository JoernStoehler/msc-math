#!/usr/bin/env python3
"""Read-only bounded exact bilinear candidate count; no angled cell enumeration."""
from denominators import K
from itertools import product, combinations
import json
import time

def main():
    started=time.perf_counter()
    z=K(0,1); t2=5-2*z; b=(1+z)/4; w=1+b; R=(5-z)/10
    normals=[(K(1),K(0)),((z-1)/4,(5-z)/4),(-(1+z)/4,(3*z-5)/4),
             (-(1+z)/4,-(3*z-5)/4),((z-1)/4,-(5-z)/4)]
    def rotate(v): return (b*v[0]-(3*z-5)/4*v[1],b*(v[0]+v[1]))
    base=[(-R,K(0)),(R,K(0)),(K(0),R)]; cur=base; triangles=[]
    for _ in range(10): triangles.append(cur); cur=[rotate(v) for v in cur]
    assert cur==base
    lam0=[K(1)/(2*w),K(1)/(2*w),K(0),b/w,K(0)]
    lams=[[lam0[(i-l)%5] for i in range(5)] for l in range(5)]
    counts=[]; pair_differences=[]; pair_direction_counts=[]; all_lines=set()
    # Difference curves can coincide under multiplication by any nonzero field scalar.
    all_directions=set()
    for lam in lams:
        active=[i for i,v in enumerate(lam) if v>0]; records=[]
        for tri in triangles:
            for choice in product(range(3),repeat=3):
                coeff=[K(0)]*4 # constant,x,y,xy
                for i,k in zip(active,choice):
                    nx,ny=normals[i]; vx,vy=tri[k]; weight=lam[i]
                    terms=(ny*vy,nx*vx,t2*nx*vy,-ny*vx)
                    coeff=[a+weight*c for a,c in zip(coeff,terms)]
                records.append(tuple(coeff))
        unique=set(records); counts.append(len(unique)); all_lines.update(unique)
        directions=set()
        for u,v in combinations(unique,2):
            diff=tuple(a-c for a,c in zip(u,v)); pivot=next(a for a in diff if a!=0)
            directions.add(tuple(a/pivot for a in diff))
        pair_differences.append(len(unique)*(len(unique)-1)//2)
        pair_direction_counts.append(len(directions)); all_directions.update(directions)
    print(json.dumps(dict(raw_per_circuit=270,raw_total=1350,
          distinct_bilinear_per_circuit=counts,distinct_total_with_circuit_labels=sum(counts),
          distinct_bilinear_union_across_circuits=len(all_lines),
          unordered_distinct_candidate_pairs_per_circuit=pair_differences,
          distinct_pair_equality_polynomials_up_to_field_scale_per_circuit=pair_direction_counts,
          equality_polynomial_union_across_circuits=len(all_directions),
          elapsed_seconds=round(time.perf_counter()-started,3)),indent=2))

if __name__=='__main__': main()
