# /// script
# requires-python = ">=3.11"
# dependencies = ["numpy>=2.0", "scipy>=1.14", "sympy>=1.13"]
# ///
"""Replay retained exact parallel-cover certificates without invoking an LP solver."""
import argparse, json
from pathlib import Path
from parallel_envelope import load_fixture, build_lp


def main():
    p=argparse.ArgumentParser();p.add_argument('--root',type=Path,default=Path.cwd());p.add_argument('certificate',type=Path);args=p.parse_args()
    fixture=load_fixture(args.root);F,Z=fixture.F,fixture.Z
    obj=json.loads(args.certificate.read_text());original=fixture.make_fixture()
    def parse(coeff):
        x=Z
        for q in coeff:x=x*fixture.tau+F(fixture.sp.Rational(q))
        return x
    T=[[[parse(c) for c in v] for v in shape['vertices']] for shape in original['normal_shapes']]
    H=parse(original['candidate_K_area'])
    if parse(obj['HKO_area'])!=H:raise ArithmeticError('HKO reference mismatch')
    for rec in obj['results']:
        base=[[parse(c) for c in t] for t in rec['base_translations']];slope=parse(rec['shear_slope'])
        groups=[[[a+t[0],b+t[1]-slope*(a+t[0])] for a,b in tri] for tri,t in zip(T,base)]
        lp=build_lp(groups,F,fixture.sign);A,b,c=lp['A'],lp['b'],lp['c']
        z=[parse(c) for c in rec['primal_variables']]
        support=rec['dual_support'];ys=[parse(c) for c in rec['dual_weights']]
        if len(z)!=lp['n'] or len(support)!=len(ys):raise ArithmeticError('Dimension mismatch')
        if len(set(support))!=len(support) or any(i<0 or i>=len(A) for i in support):raise ArithmeticError('Invalid dual support')
        if not all(fixture.sign(rhs-sum((a*x for a,x in zip(row,z)),Z))>=0 for row,rhs in zip(A,b)):raise ArithmeticError('Primal infeasibility')
        if not all(fixture.sign(y)>=0 for y in ys):raise ArithmeticError('Dual negative weight')
        for j,cj in enumerate(c):
            if sum((A[i][j]*y for i,y in zip(support,ys)),Z)!=-cj:raise ArithmeticError('Dual stationarity mismatch')
        primal=sum((ci*zi for ci,zi in zip(c,z)),Z);dual=-sum((b[i]*y for i,y in zip(support,ys)),Z)
        if primal!=dual or primal!=parse(rec['objective']):raise ArithmeticError('Objective mismatch')
        points=[]
        for j,tri in enumerate(groups):
            shift=Z if j==0 else z[2*lp['m']+j-1]
            points.extend([[a+shift,b] for a,b in tri])
        if fixture.area(fixture.hull(points))!=primal:raise ArithmeticError('Hull area mismatch')
        if rec['name'].startswith('HKO_') and primal!=H:raise ArithmeticError('HKO control mismatch')
        print(f"PASS {rec['name']}: exact primal, dual, zero gap, hull area; {lp['n']} variables, {len(A)} inequalities")
    print('PASS all retained slice certificates; no LP solver invoked; no full-space optimum inferred')

if __name__=='__main__':main()
