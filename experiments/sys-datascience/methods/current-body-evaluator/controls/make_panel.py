"""Target-blind six-body panel plus two explicit invalid inputs."""
import json
from pathlib import Path


def product(factor):
    return [[a,b,0,0] for a,b in factor]+[[0,0,a,b] for a,b in factor]


def rotate(vs,i,j):
    result=[]
    for v in vs:
        w=list(v); w[i]=.6*v[i]-.8*v[j]; w[j]=.8*v[i]+.6*v[j];result.append(w)
    return result

cube=product([(1,0),(-1,0),(0,1),(0,-1)])
tri=product([(1,0),(0,1),(-1,-1)])
# q=(x0,x1), p=(x2,x3). Rotating (q0,p0) preserves omega;
# rotating only (q0,p1) is SO(4), generally not symplectic.
rows=[('cube',cube),('cube_scaled2',[[x/2 for x in v] for v in cube]),
      ('tri',tri),('tri_u2',rotate(tri,0,2)),('tri_so4',rotate(tri,0,3)),
      ('cube_so4',rotate(cube,0,3)),('unbounded',[[1,0,0,0]]*5),('bad_schema',[[1,2,3]])]
Path(__file__).with_name('inputs.jsonl').write_text(''.join(json.dumps({'id':k,'dual_vertices':v})+'\n' for k,v in rows))
