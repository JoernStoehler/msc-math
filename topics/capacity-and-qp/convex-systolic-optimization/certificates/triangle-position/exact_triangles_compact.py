"""Rational certificate for the triangular product positioning theorem.
Sage QQ/PPL. Enumerates 120 words, four coefficient maxima, and a rank-one hull.
"""
from sage.all import QQ, matrix, vector, Polyhedron
from itertools import permutations, combinations
from pathlib import Path
import json
out = Path(__file__).resolve().parent
N = matrix(QQ, [[1,0,0,0],[0,1,0,0],[-1,-1,0,0],
                [0,0,1,0],[0,0,0,1],[0,0,-1,-1]])
x = vector(QQ, [1,1,-2,1,1,-1])
pairs = list(combinations(range(4),2))
Omega = matrix(QQ,4,4)
for (i,j),v in zip(pairs,x):
    Omega[i,j],Omega[j,i]=v,-v
rows=set(); groups={}
for tail in permutations(range(1,6)):
    word=(0,)+tail
    a=b=c=QQ(0)
    row=vector(QQ,4)
    for i in range(6):
        for j in range(i+1,6):
            u,v=word[i],word[j]
            pairing=N.row(u)*Omega*N.row(v)
            if u<3 and v<3: a+=pairing
            elif u>=3 and v>=3: b+=pairing
            else: c+=pairing
            for k,(r,s) in enumerate([pairs[z] for z in [1,2,3,4]]):
                row[k]+=(N[u,r]*N[v,s]-N[u,s]*N[v,r])/3
    rows.add(tuple(row))
    groups.setdefault((a,b),[]).append((c,word))
expected={(1,1):4,(1,-1):6,(-1,1):6,(-1,-1):4}
assert {k:max(v[0] for v in vs) for k,vs in groups.items()}==expected
assert all(len(vs)==30 for vs in groups.values())
# Complete cross action at mass 1/2, multiplied by12, gives the rows above.
C=Polyhedron(ieqs=[[1]+list(-vector(QQ,b)) for b in sorted(rows)],base_ring=QQ,backend='ppl')
assert C.is_compact() and C.dim()==4
U=[vector(QQ,[1,0]),vector(QQ,[0,1]),vector(QQ,[1,-1])]
expected_vertices={tuple(s*QQ(3)/2*a[i]*b[j] for i in range(2) for j in range(2))
                   for a in U for b in U for s in [-1,1]}
assert {tuple(v) for v in C.vertices_list()}==expected_vertices
assert len(expected_vertices)==18 and C.n_inequalities()==30
assert max(abs(matrix(QQ,[u,v]).det()) for u in U for v in U)==1
sharp=vector(QQ,[1,-QQ(1)/2,-QQ(1)/2,1])
assert C.contains(sharp) and matrix(QQ,2,2,sharp).det()==QQ(3)/4
A=matrix(QQ,[[1,0,0,0],[1,-1,1,0],[0,1,0,0],[-QQ(1)/4,-QQ(1)/2,0,-QQ(1)/4]])
J=matrix(QQ,[[0,0,1,0],[0,0,0,1],[-1,0,0,0],[0,-1,0,0]])
assert A.inverse()*J*A.inverse().transpose()==Omega and A.det()==QQ(1)/4
# The four univariate maxima are obtained at1/2,2/3,1/3,1/2.
maxima={}
for (a,b),c in expected.items():
    t=QQ(1)/2 if a==b else (QQ(2)/3 if a==1 else QQ(1)/3)
    value=(a*t*t+b*(1-t)**2+c*t*(1-t))/9
    maxima[f'{a},{b}']=str(value)
assert max(QQ(v) for v in maxima.values())==QQ(5)/27
summary={'verified':True,'full_h':'5/27','rho':'18/25','lagrangian_max':'2/3',
         'coefficient_groups':{f'{a},{b}':{'max_cross':c,'count':30,'max_action':maxima[f'{a},{b}']}
          for (a,b),c in expected.items()},'rank_one_vertices':18,'cross_facets':30,
         'note':'Global determinant bound is the analytic Cauchy-Binet argument in the LaTeX proof.'}
(out/'exact-triangles-compact-summary.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps(summary,indent=2))
