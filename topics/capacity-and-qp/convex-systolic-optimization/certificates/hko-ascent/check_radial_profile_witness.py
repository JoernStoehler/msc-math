#!/usr/bin/env python3
"""Exact concentrated-response witness for the fixed radial profile cone."""
from pathlib import Path
import argparse,importlib.util,itertools,json
from fractions import Fraction as F

def main():
 p=argparse.ArgumentParser(description=__doc__)
 p.add_argument('--core',type=Path,default=Path(__file__).resolve().parent/'complete-simple'/'exact_core.py')
 p.add_argument('--out',type=Path,required=True);args=p.parse_args()
 if args.out.exists():p.error('output already exists')
 spec=importlib.util.spec_from_file_location('radial_witness_exact_core',args.core)
 E=importlib.util.module_from_spec(spec);spec.loader.exec_module(E)
 u=E.TAU**2;sig=[0,6,5,2,1,8,4]
 b=[x/200 for x in [17+5*u,25+5*u,25+5*u,44-8*u,E.K(8),50-10*u,31+3*u]]
 expected_h=[x/40 for x in [-59+5*u,-9-5*u,-9-5*u,-9-5*u,E.K(8),-9-5*u,-45+3*u]]
 assert all(E.sign(x)>0 for x in b) and sum(b,E.ZERO)==E.ONE
 assert all(sum((bw*E.A[lab][k] for lab,bw in zip(sig,b)),E.ZERO)==E.ZERO for k in range(4))
 assert E.qeval(E.A,sig,b)==E.Q0
 g=[sum((b[j]*(E.omega(E.A[sig[j]],E.A[sig[i]]) if j<i else E.omega(E.A[sig[i]],E.A[sig[j]])) for j in range(len(sig)) if i!=j),E.ZERO) for i in range(len(sig))]
 C=[E.A[lab]+[E.ONE] for lab in sig]
 for I in itertools.combinations(range(len(sig)),5):
  try:ci=E.inv([C[i] for i in I]);break
  except ArithmeticError:pass
 lam=E.mv(ci,[g[i] for i in I]);assert all(E.dot(row,lam)==gi for row,gi in zip(C,g))
 T=1/(2*E.Q0);N=E.SQRT5-1;x=E.scale(-T,lam[:4]);terms=[];actual_h=[]
 J=lambda a:[-a[2],-a[3],a[0],a[1]]
 for lab,bw,hh in zip(sig,b,expected_h):
  a=E.A[lab];end=E.add(x,E.scale(2*T*bw,J(a)))
  assert all(E.sign(E.dot(n,x)-1)<=0 and E.sign(E.dot(n,end)-1)<=0 for n in E.A)
  assert E.dot(a,x)==E.ONE and E.dot(a,end)==E.ONE
  if lab<5:v=[-a[1]/N,a[0]/N];ell=E.dot(v,x[2:]);assert ell==E.dot(v,end[2:])
  else:v=[a[3]/N,-a[2]/N];ell=E.dot(v,x[:2]);assert ell==E.dot(v,end[:2])
  h=ell-E.K(F(3,5));assert h==hh;actual_h.append(h)
  terms.append(bw*h if E.sign(h)>0 else E.ZERO);x=end
 assert terms==[E.ZERO,E.ZERO,E.ZERO,E.ZERO,E.K(F(1,125)),E.ZERO,E.ZERO]
 # Check the factor-exchanging involution on all base normals and velocities.
 S=lambda a:[-a[3],a[2],a[1],-a[0]]
 D=[]
 for a in E.A:
  if a[2]==E.ZERO and a[3]==E.ZERO:D.append([-3*a[0]/5,-3*a[1]/5,-a[1]/N,a[0]/N])
  else:D.append([a[3]/N,-a[2]/N,-3*a[2]/5,-3*a[3]/5])
 for i in range(5):
  assert S(E.A[i])==E.A[5+i] and S(E.A[5+i])==E.A[i]
  assert S(D[i])==D[5+i] and S(D[5+i])==D[i]
 for i in range(4):
  for j in range(4):
   ei=[E.K(k==i) for k in range(4)];ej=[E.K(k==j) for k in range(4)]
   assert E.omega(S(ei),S(ej))==E.omega(ei,ej)
 kappa=8*(5+E.SQRT5)/46875
 assert E.sign(E.K(F(1,125))-6*kappa)>0 and E.sign(7*kappa-E.K(F(1,125)))>0
 slope=4*(7*kappa-E.K(F(1,125)));assert slope==4*(56*E.SQRT5-95)/46875
 out={'all_checks_passed':True,'word':sig,'weights_coefficients_ascending':[v.coefficients() for v in b],
  'hinge_arguments_coefficients_ascending':[v.coefficients() for v in actual_h],
  'response_terms_coefficients_ascending':[v.coefficients() for v in terms],
  'only_contributing_facet':1,'concentrated_response':'1/125','factor_exchange_symplectic_and_profile_preserving':True,
  'six_kappa_less_than_1_over_125_less_than_seven_kappa':True,'seventeen_facet_log_derivative_coefficients_ascending':slope.coefficients(),
  'scope':'Exact witness, factor-exchange symmetry and scalar comparisons. The weighted upper bound additionally uses the previously audited complete all-word response bound.'}
 args.out.parent.mkdir(parents=True,exist_ok=True);args.out.write_text(json.dumps(out,indent=2)+'\n')
 print(json.dumps(out,indent=2))

if __name__=='__main__':main()
