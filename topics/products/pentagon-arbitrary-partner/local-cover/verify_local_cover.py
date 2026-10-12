#!/usr/bin/env python3
"""Exact local-rigidity certificate for ten translated pentagon-normal triangles.

python verify_local_cover.py --fixture-code /path/pentagon_fixture.py --out fresh.json
No global minimum is asserted. See LOCAL_COVER_RIGIDITY.md for the continuum proof.
"""
from __future__ import annotations
import argparse, importlib.util, json, time
from pathlib import Path
from functools import cmp_to_key
from itertools import combinations
import sympy as sp
from sympy.polys.matrices import DomainMatrix


def main():
    if not __debug__: raise RuntimeError('Run without -O; assertions verify the certificate.')
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--fixture-code',type=Path,required=True)
    p.add_argument('--out',type=Path,required=True)
    args=p.parse_args()
    if args.out.exists(): raise FileExistsError(args.out)
    start=time.perf_counter()
    spec=importlib.util.spec_from_file_location('source_fixture',args.fixture_code)
    source=importlib.util.module_from_spec(spec); spec.loader.exec_module(source)
    data=source.make_fixture()
    F=sp.QQ.algebraic_field(sp.sqrt(5)); Z=F.zero; O=F.one; sqrt5=F.unit
    def sign(x):
        if not x: return 0
        cs=x.to_list(); a=cs[-1]; b=cs[-2] if len(cs)==2 else sp.QQ.zero
        if not b:return 1 if a>0 else -1
        if not a:return 1 if b>0 else -1
        if (a>0)==(b>0):return 1 if a>0 else -1
        disc=a*a-5*b*b; assert disc
        return (1 if disc>0 else -1)*(1 if a>0 else -1)
    def cv(raw,is_y=False):
        x=source.F.new([sp.QQ(c) for c in raw])
        if is_y:x/=source.tau
        out=Z; cs=x.to_list()
        for i,c in enumerate(cs):
            deg=len(cs)-1-i
            if deg%2:assert c==0
            else:out+=F(c)*(F(5)-F(2)*sqrt5)**(deg//2)
        return out
    def point(p):return (cv(p[0]),cv(p[1],True))
    def sub(a,b):return tuple(x-y for x,y in zip(a,b))
    def add(a,b):return tuple(x+y for x,y in zip(a,b))
    def cross(a,b):return a[0]*b[1]-a[1]*b[0]
    def hull(points):
        def cmp(a,b):return sign(a[0]-b[0]) or sign(a[1]-b[1])
        pts=sorted(set(points),key=cmp_to_key(cmp)); lo=[];hi=[]
        for dest,seq in [(lo,pts),(hi,reversed(pts))]:
            for x in seq:
                while len(dest)>=2 and sign(cross(sub(dest[-1],dest[-2]),sub(x,dest[-2])))<=0:dest.pop()
                dest.append(x)
        return lo[:-1]+hi[:-1]
    def mat(rows):return DomainMatrix(rows,(len(rows),len(rows[0])),F)
    def rank(rows):return len(mat(rows).rref()[1])
    def serialize(x):return [str(a) for a in x.to_list()]
    H=[point(x) for x in data['known_optimal_candidate_K']]
    triangles=[[point(x) for x in sh['vertices']] for sh in data['normal_shapes']]
    base=[point(x) for x in data['candidate_translations']]
    placed=[[add(x,b) for x in tri] for tri,b in zip(triangles,base)]
    m=sum((cross(H[i],H[(i+1)%5]) for i in range(5)),Z)/F(2)
    assert sign(m)>0
    assert hull([x for tri in placed for x in tri])==hull(H)
    D=hull([sub(x,y) for x in H for y in H]);assert len(D)==10
    unique_pairs=[]
    for tri in placed:
        candidates=[(j,k) for j,k in combinations(range(3),2) if sub(tri[j],tri[k]) in D or sub(tri[k],tri[j]) in D]
        assert candidates
        j,k=candidates[0];assert tri[j] in H and tri[k] in H
        unique_pairs.append([j,k])
    groups=[]; incidence=[]
    for i,v in enumerate(H):
        labels=[j for j,tri in enumerate(placed) if v in tri]
        assert len(labels)==5
        incidence.append(labels)
        for neighbor,direction in [(H[(i-1)%5],1),(H[(i+1)%5],-1)]:
            e=sub(neighbor,v);c=(-F(direction)*e[1]/F(2),F(direction)*e[0]/F(2))
            rows=[]
            for j in labels:
                row=[Z]*18
                if j:row[2*(j-1):2*j]=c
                rows.append(row)
            groups.append(rows)
    E=[[groups[g][j][k] for g in range(10) for j in range(5)] for k in range(18)]
    E+=[[O if col//5==g else Z for col in range(50)] for g in range(10)]
    E=mat(E);assert len(E.rref()[1])==28
    rhs=DomainMatrix([[Z]]*18+[[O]]*10,(28,1),F)
    p0=DomainMatrix([[O/F(5)]]*50,(50,1),F)
    weights=p0+E.transpose()*(E*E.transpose()).inv()*(rhs-E*p0)
    assert E*weights==rhs
    ws=[r[0] for r in weights.to_list()]
    assert all(sign(x)>0 for x in ws)
    diffs=[[a-b for a,b in zip(row,g[0])] for g in groups for row in g[1:]]
    assert rank(diffs)==18
    minimum=min(ws,key=cmp_to_key(lambda a,b:sign(a-b)))
    out={
      'status':'PASS',
      'arithmetic':'ordered Q(sqrt(5)); coordinates (x,y/tau)',
      'fixture_reconstructed_and_verified':True,
      'area_unit':serialize(m),'triangles':10,'translation_quotient_dimension':18,
      'groups':10,'choices_per_group':5,'implicit_full_gradient_bank_size':5**10,
      'constraint_rank':28,'gradient_difference_rank':18,
      'minimum_marginal':serialize(minimum),'minimum_marginal_approx':float(F.to_sympy(minimum)),
      'positive_marginals':[[serialize(x) for x in ws[5*g:5*g+5]] for g in range(10)],
      'vertex_triangle_incidence':incidence,'unique_placement_difference_pairs':unique_pairs,
      'interpretation':'Strict local minimum of translated-triangle hull area modulo common translation, via the stated local signed-polygon estimate. This gives local equality rigidity for arbitrary convex partners of fixed Q. No unrestricted global bound is proved.',
      'elapsed_seconds':time.perf_counter()-start}
    args.out.parent.mkdir(parents=True,exist_ok=True)
    args.out.write_text(json.dumps(out,indent=2)+'\n')
    print('PASS: exact 50-marginal certificate, rank 18, unique base placements.')
    print('Minimum positive marginal:',F.to_sympy(minimum))
    print('Seconds:',out['elapsed_seconds'])
if __name__=='__main__':main()
