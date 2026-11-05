# /// script
# requires-python = ">=3.11"
# dependencies = ["numpy>=2.0", "scipy>=1.14", "sympy>=1.13"]
# ///
"""Finite envelope LP for translating finite planar templates along one direction.

The floating solver proposes primal/dual active sets. Exact field reconstruction
and full residual/sign checks, when reported PASS, certify the slice optimum.
No unrestricted two-dimensional translation optimum is asserted.
"""
from __future__ import annotations
import argparse, importlib.util, json, time
from pathlib import Path
from functools import cmp_to_key
import numpy as np
from scipy.optimize import linprog
from sympy.polys.matrices import DomainMatrix as DM


def load_fixture(root):
    path = root / 'topics/products/pentagon-arbitrary-partner/local-cover/pentagon_fixture.py'
    spec = importlib.util.spec_from_file_location('fixture', path)
    module = importlib.util.module_from_spec(spec); spec.loader.exec_module(module)
    return module


def build_lp(groups, field, sign):
    """Groups are already in (horizontal, vertical) area-preserving coordinates."""
    Z, O = field.zero, field.one
    heights = sorted(set(p[1] for g in groups for p in g), key=cmp_to_key(lambda a,b:sign(a-b)))
    if len(heights) < 2: raise ValueError('All points are collinear horizontally; optimum is zero.')
    m, k = len(heights), len(groups); n = 2*m+k-1
    index = {y:r for r,y in enumerate(heights)}
    A, b, names = [], [], []
    def add(row, rhs, name): A.append(row); b.append(rhs); names.append(name)
    for j,g in enumerate(groups):
        for i,(a,y) in enumerate(g):
            r=index[y]
            row=[Z]*n; row[r]=O
            if j: row[2*m+j-1]=-O
            add(row,a,('left',j,i,r))
            row=[Z]*n; row[m+r]=-O
            if j: row[2*m+j-1]=O
            add(row,-a,('right',j,i,r))
    gaps=[heights[r+1]-heights[r] for r in range(m-1)]
    for r in range(1,m-1):
        for start,sgn,label in [(0,O,'convex-left'),(m,-O,'concave-right')]:
            row=[Z]*n
            row[start+r-1]=-sgn*gaps[r]
            row[start+r]=sgn*(gaps[r-1]+gaps[r])
            row[start+r+1]=-sgn*gaps[r-1]
            add(row,Z,(label,r))
    weights=[Z]*m
    for r,gap in enumerate(gaps): weights[r]+=gap/field(2); weights[r+1]+=gap/field(2)
    c=[-w for w in weights]+weights+[Z]*(k-1)
    return {'A':A,'b':b,'c':c,'heights':heights,'names':names,'m':m,'k':k,'n':n}


def solve_and_certify(lp, fixture):
    F,Z=fixture.F,fixture.Z
    def num(x): return float(F.to_sympy(x).evalf())
    A,b,c=lp['A'],lp['b'],lp['c']; n=lp['n']
    An=np.array([[num(x) for x in row] for row in A]); bn=np.array([num(x) for x in b]); cn=np.array([num(x) for x in c])
    ans=linprog(cn,A_ub=An,b_ub=bn,bounds=[(None,None)]*n,method='highs')
    if not ans.success: raise RuntimeError(ans.message)
    # Primal: exact independent active equations suggested by the floating solve.
    candidates=[i for i,r in enumerate(ans.ineqlin.residual) if abs(r)<1e-8]
    matrix=DM([A[i] for i in candidates],(len(candidates),n),F)
    _,piv=matrix.transpose().rref()
    selected=[candidates[i] for i in piv]
    if len(selected)!=n: raise ArithmeticError(f'Primal active rank {len(selected)} < {n}')
    z=fixture.solve([A[i] for i in selected],[b[i] for i in selected])
    slacks=[rhs-sum((a*x for a,x in zip(row,z)),Z) for row,rhs in zip(A,b)]
    if not all(fixture.sign(s)>=0 for s in slacks): raise ArithmeticError('Exact primal infeasible')
    # Dual convention: y>=0, A^T y=-c gives lower bound -b.y.
    support=[i for i,y in enumerate(ans.ineqlin.marginals) if y < -1e-9]
    dualmat=DM([A[i] for i in support],(len(support),n),F)
    _,cols=dualmat.rref()
    if len(cols)!=len(support): raise ArithmeticError('Dual support dependent')
    square=[[A[i][j] for i in support] for j in cols]
    ys=fixture.solve(square,[-c[j] for j in cols])
    if not all(fixture.sign(y)>=0 for y in ys): raise ArithmeticError('Exact dual sign failure')
    if any(sum((A[i][j]*y for i,y in zip(support,ys)),Z)!=-c[j] for j in range(n)):
        raise ArithmeticError('Exact dual stationarity failure')
    primal=sum((a*x for a,x in zip(c,z)),Z)
    dual=-sum((b[i]*y for i,y in zip(support,ys)),Z)
    if primal!=dual: raise ArithmeticError('Exact primal-dual gap')
    return {'status':'PASS exact primal feasibility, dual feasibility and zero gap',
            'objective':fixture.serialize(primal),'objective_float':num(primal),
            'variables':n,'inequalities':len(A),'height_levels':lp['m'],
            'primal_active_basis':selected,'dual_support':support,
            'dual_weights':[fixture.serialize(y) for y in ys],
            'primal_variables':[fixture.serialize(x) for x in z],
            'point_contacts':[lp['names'][i] for i,s in enumerate(slacks) if s==Z and lp['names'][i][0] in ('left','right')],
            'minimum_exact_primal_slack_float':min(num(s) for s in slacks),
            'minimum_exact_dual_weight_float':min(num(y) for y in ys)}, z


def main():
    parser=argparse.ArgumentParser(); parser.add_argument('--root',type=Path,default=Path.cwd()); parser.add_argument('--out',type=Path,required=True)
    args=parser.parse_args()
    if args.out.exists(): raise FileExistsError(args.out)
    fixture=load_fixture(args.root); f=fixture.make_fixture(); F,Z,O=fixture.F,fixture.Z,fixture.O
    def parse(coeff):
        x=Z
        for q in coeff: x=x*fixture.tau+F(fixture.sp.Rational(q))
        return x
    T=[[[parse(c) for c in p] for p in s['vertices']] for s in f['normal_shapes']]
    known=[[parse(c) for c in p] for p in f['candidate_translations']]
    H=parse(f['candidate_K_area'])
    # Rational shear x'=x, y'=y-q*x has determinant one; horizontal shifts
    # in these coordinates correspond to direction (1,q) in the original plane.
    examples=[('HKO_horizontal',known,F(0)),('HKO_slope_one_seventh',known,F(1)/F(7)),
              ('common_center_horizontal',[[Z,Z] for _ in T],F(0)),
              ('remote_rational_vertical',[[Z,F((j*j+3*j)%7-3)/F(20)] for j in range(10)],F(0))]
    results=[]; started=time.monotonic()
    for name,base,slope in examples:
        groups=[[[p[0]+t[0],p[1]+t[1]-slope*(p[0]+t[0])] for p in g] for g,t in zip(T,base)]
        lp=build_lp(groups,F,fixture.sign)
        result,z=solve_and_certify(lp,fixture)
        points=[]
        for j,g in enumerate(groups):
            shift=Z if j==0 else z[2*lp['m']+j-1]
            points.extend([[p[0]+shift,p[1]] for p in g])
        area=fixture.area(fixture.hull(points))
        if fixture.serialize(area)!=result['objective']: raise ArithmeticError('Independent exact hull area differs')
        result.update(name=name,area_over_HKO=float(F.to_sympy(area/H).evalf()),
                      hull_area_check='PASS exact independently reconstructed convex hull',
                      shear_slope=fixture.serialize(slope),
                      base_translations=[[fixture.serialize(c) for c in t] for t in base])
        if name.startswith('HKO_') and area!=H: raise ArithmeticError('HKO control differs')
        results.append(result)
        print(json.dumps({'name':name,'status':result['status'],'area_over_HKO':result['area_over_HKO'],'vars':lp['n'],'rows':len(lp['A'])}),flush=True)
    output={'scope':'Exact common-direction slice optima only; not full translation-cover optimum',
            'field':f['arithmetic'],'field_polynomial':f['field_polynomial'],'root_interval':f['root_interval'],
            'coefficient_format':f['coefficient_format'],'HKO_area':f['candidate_K_area'],
            'results':results,'elapsed_seconds':time.monotonic()-started}
    args.out.parent.mkdir(parents=True,exist_ok=True); args.out.write_text(json.dumps(output,indent=2)+'\n')
    print(json.dumps({'output':str(args.out),'elapsed_seconds':output['elapsed_seconds']}),flush=True)

if __name__=='__main__': main()
