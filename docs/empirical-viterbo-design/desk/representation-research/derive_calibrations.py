# /// script
# requires-python = ">=3.11"
# dependencies = ["sympy>=1.13"]
# ///
"""Exact identities independent of the f64 collection's tensor integrator."""
from fractions import Fraction
import itertools
import json
import math
from pathlib import Path
import sympy as s

HERE=Path(__file__).resolve().parent
u=s.symbols('a b c',real=True)
J=s.Matrix([[0,0,1,0],[0,0,0,1],[-1,0,0,0],[0,-1,0,0]])
B=s.Matrix([[0,1,0,0],[-1,0,0,0],[0,0,0,-1],[0,0,1,0]])
omega=u[0]*J+u[1]*B+u[2]*(J*B)
radius=sum(z*z for z in u)
assert omega.T==-omega
assert s.simplify(omega*omega+radius*s.eye(4))==s.zeros(4)

# Regular simplex, isotropic uniform volume. Raw vertices e_i,-1 have
# covariance (I+11^T)/30; whitening is exact in Q(sqrt(6),sqrt(30)).
raw=s.eye(4).col_join(-s.ones(1,4))
projection=s.ones(4,4)/4
whitening=s.sqrt(30)*(s.eye(4)-projection)+s.sqrt(6)*projection
vertices=s.simplify(raw*whitening)
assert s.simplify(vertices.T*vertices)==30*s.eye(4)
pairings=s.simplify(vertices*omega*vertices.T)
fourth_sum=s.Poly(s.expand(sum(entry**4 for entry in pairings)),*u)
expected=1166400*radius**2
assert s.simplify(fourth_sum.as_expr()-expected)==0
# Dirichlet(1,1,1,1,1): M4=(15/28)D+(1/280) sum v_i^tensor4.
alpha=s.Rational(15,28);beta=s.Rational(1,280)
simplex_fourth=s.simplify(72*alpha**2+2*8640*alpha*beta+1166400*beta**2)
assert simplex_fourth/16==s.Rational(6723,1568)

# 24-cell = [-1,1]^4 intersect {sum |x_i|<=2}. In one orthant, subtract
# four shifted unit simplices from the radius2 simplex; double overlaps
# have zero volume. Integrate even monomials, then multiply by16.
def simplex_integral(exponents,radius):
    degree=sum(exponents)
    return Fraction(radius**(4+degree)*math.prod(math.factorial(p) for p in exponents), math.factorial(4+degree))

def cell24_integral(exponents):
    value=simplex_integral(exponents,2)
    for axis in range(4):
        for k in range(exponents[axis]+1):
            shifted=list(exponents);shifted[axis]=k
            value-=math.comb(exponents[axis],k)*simplex_integral(shifted,1)
    return 16*value

cellvolume=cell24_integral([0]*4)
cellcov=cell24_integral([2,0,0,0])/cellvolume
cellfourthdiag=cell24_integral([4,0,0,0])/cellvolume
cellfourthmixed=cell24_integral([2,2,0,0])/cellvolume
assert cellvolume==8 and cellcov==Fraction(13,60)
assert cellfourthdiag==Fraction(3,28) and cellfourthmixed==Fraction(1,28)
cellalpha=cellfourthmixed/cellcov**2
cellkurtosis=Fraction(72,16)*cellalpha**2

# Independent centered uniform interval coordinates with variance1 have
# fourth moment9/5. Expanding the bilinear fourth moment gives the formula.
mu=s.Rational(9,5)
cube_fourth=s.expand(24*mu*radius**2+(mu-3)**2*sum(entry**4 for entry in omega))
cube_kurtosis=s.simplify(cube_fourth/16)
assert s.expand(cube_kurtosis-s.Rational(27,10)*radius**2-s.Rational(9,25)*sum(z**4 for z in u))==0

result=dict(status='exact symbolic/rational identities; not capacity statements',
    simplex_pairing_fourth=str(simplex_fourth),simplex_pairing_kurtosis=str(simplex_fourth/16),
    simplex_vertex_pairing_fourth_sum=str(expected),
    cell24_volume=str(cellvolume),cell24_covariance_diagonal=str(cellcov),
    cell24_fourth_diagonal=str(cellfourthdiag),cell24_fourth_mixed=str(cellfourthmixed),
    cell24_isotropic_fourth_tensor_coefficient=str(cellalpha),cell24_pairing_kurtosis=str(cellkurtosis),
    cube_pairing_kurtosis_on_unit_sphere='27/10 + (9/25)*(a^4+b^4+c^4)',
    cube_pairing_kurtosis_range=['141/50','153/50'],
    cube_equal_kurtosis_contrast=dict(orientations=['(1/sqrt(2),1/sqrt(2),0)','(sqrt(2/3),1/sqrt(6),1/sqrt(6))'],
        common_kurtosis='72/25',normalized_dual_reciprocals=['sqrt(2)/4','sqrt(3/2)/4']),
    cube_normalized_dual_reciprocal='1/(4 max(|a|,|b|,|c|))',
    cube_normalized_ridge_sum='8*(|a|+|b|+|c|)')
(HERE/'artifacts'/'exact-calibrations.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
print(json.dumps(result,indent=2,sort_keys=True))
