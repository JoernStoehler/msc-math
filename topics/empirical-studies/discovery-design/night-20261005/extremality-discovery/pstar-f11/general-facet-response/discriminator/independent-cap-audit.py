# /// script
# dependencies = ["sympy"]
# ///
"""Independent exact facet cap for selected D=(1,-1,0,1)."""
import sympy as S
q,t=S.symbols('q t');a=S.Rational(5,12);lower=S.Rational(1,12)
P=[(S.Rational(3,4),0),(0,S.Rational(1,2)),(-S.Rational(1,4),0),(0,-S.Rational(1,2))]
heights=[S.Rational(x)+p2-S.Rational(1,2) for x in [-a,a] for p1,p2 in P]
assert sum(bool(z>0) for z in heights)==1 and all(z!=0 for z in heights)
width=1-2*t
positive_measure=S.integrate(width*(t-lower),(t,lower,S.Rational(1,2)))
total_measure=2*a*S.Rational(1,2)
fraction=S.cancel(positive_measure/total_measure)
moment=S.integrate(width*S.integrate(q+t-S.Rational(1,2),(q,S.Rational(1,2)-t,a)),(t,lower,S.Rational(1,2)))
cap=moment/2
assert fraction==S.Rational(25,432)
assert cap==S.Rational(625,497664)
print('PASS selected D has exactly one positive vertex; heights:',heights)
print('positive facet fraction:',fraction,'; exact cap derivative:',cap)
r,delta=S.symbols('r delta')
before=2*delta*(-S.Rational(1,4)-2*(r-delta))
after=2*delta*(-S.Rational(1,4)+2*(r-delta))
assert S.expand(before)==-delta/2-4*r*delta+4*delta**2
assert S.expand(after)==-delta/2+4*r*delta-4*delta**2
print('Original continuum adjacent ell<=-delta/18<0 on0<delta<r<1/9; nonadjacent placements audited separately.')
