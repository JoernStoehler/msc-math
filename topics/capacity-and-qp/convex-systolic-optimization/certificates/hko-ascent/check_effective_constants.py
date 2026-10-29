#!/usr/bin/env python3
"""Exact primitive action gap and rational constants for the effective step.

This checks finite arithmetic predicates, not the written rounding/remainder
argument. It accepts a path to the unmodified exact_core.py dependency.
"""
from pathlib import Path
from fractions import Fraction as F
import argparse,functools,importlib.util,itertools,json,time

def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--core',type=Path,default=Path(__file__).resolve().parents[1]/'complete-simple'/'exact_core.py')
    p.add_argument('--out',type=Path,required=True)
    args=p.parse_args()
    if args.out.exists():p.error('output already exists')
    spec=importlib.util.spec_from_file_location('effective_exact_core',args.core)
    E=importlib.util.module_from_spec(spec);spec.loader.exec_module(E)
    tic=time.time()
    def closure_vertices(offset):
        out=[]
        for ids in itertools.combinations(range(offset,offset+5),3):
            axes=range(2) if offset==0 else range(2,4)
            M=[[E.A[i][r] for i in ids] for r in axes]+[[E.ONE]*3]
            try:w=E.mv(E.inv(M),[E.ZERO,E.ZERO,E.ONE])
            except ArithmeticError:continue
            if all(E.sign(v)>0 for v in w):out.append((ids,[v/2 for v in w]))
        assert len(out)==5
        return out
    qs,ps=closure_vertices(0),closure_vertices(5)
    gap=None;values=set();words=winners=0
    for qi,qw in qs:
      for pi,pw in ps:
        labels=qi+pi;weights=dict(zip(labels,qw+pw));first=min(labels)
        for tail in itertools.permutations([x for x in labels if x!=first]):
            word=(first,)+tail;words+=1
            value=E.qeval(E.A,word,[weights[x] for x in word]);values.add(value)
            loss=E.Q0-value;assert E.sign(loss)>=0
            if not loss:winners+=1
            elif gap is None or E.sign(loss-gap)<0:gap=loss
    assert words==3000 and winners==50 and len(values)==13
    assert gap==(-E.TAU+3*E.TAU**3)/20
    assert E.sign(gap-E.K(F(1,50)))>0
    assert E.sign(E.Q0-E.K(F(1,10)))>0
    t=F(1,10**30);M=F(200);c=F(1,10000)
    R=10**5*M*M;distance=F(2000);A=88*M*distance;C=F(10**15)
    interval=F(1,10**8)
    checks={
      'base_pivot_stays_above_c': F(1,4500)-2*M*t>c,
      'neumann_inverse_factor_below_two':10*M*interval<F(1,2),
      'chart_l1_norm_below_two':1+20*M*interval<2,
      'second_derivative_below_twice_R':19200*M*M+1280*M+16<2*R,
      'distance_coefficient_2000_suffices':4+160*10<distance,
      'quadratic_remainder_constant_suffices':A*A/4+R<C,
    }
    dmax=44*M*t+R*t*t
    checks['deficit_below_1e_minus_20']=dmax<F(1,10**20)
    checks['retained_vertex_mass_nonempty']=(80**2)*100*dmax<1
    checks['selected_pivot_preserved_by_rounding']=distance**2*dmax<c*c/4
    a=F(2,125);b=F(10**16);cq=F(3*10**16);cv=F(8,5)
    checks['normalized_action_square_coefficient']=a*a+2*b+2*a*b*t+b*b*t*t<cq
    checks['action_volume_product_coefficient']=cq+cv+2*a*cv*t+cq*cv*t*t<4*10**16
    eta=(20+64*E.SQRT5)/9375
    checks['linear_gain_exceeds_one_percent']=E.sign(eta-E.K(F(1,100)))>0
    checks['quadratic_cost_below_half_gain']=4*10**16*t<F(1,200)
    assert all(checks.values()),checks
    out={'all_finite_checks_passed':True,
      'scope':'Finite exact action-gap and rational inequalities; analytic coverage, pivot, rounding and Taylor proofs are separate reviewed arguments.',
      'primitive_words':words,'primitive_winners':winners,
      'primitive_values_coefficients_ascending':[v.coefficients() for v in sorted(values,key=functools.cmp_to_key(lambda x,y:E.sign(x-y)))],
      'minimum_positive_action_gap_coefficients_ascending':gap.coefficients(),
      'gap_exceeds_1_over_50':True,'checks':checks,
      'parameter':str(t),'relative_gain_lower_bound':str(t/200),
      'M':str(M),'c':str(c),'R':str(R),'distance_constant':str(distance),
      'A':str(A),'A_squared_over_four_plus_R':str(A*A/4+R),'action_remainder_bound':str(C),
      'elapsed_seconds':time.time()-tic}
    args.out.parent.mkdir(parents=True,exist_ok=True)
    args.out.write_text(json.dumps(out,indent=2)+'\n')
    print(json.dumps({k:v for k,v in out.items() if k!='primitive_values_coefficients_ascending'},indent=2))

if __name__=='__main__':main()
