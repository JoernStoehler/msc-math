"""Independent checks of a hand-derived primitive; no symbolic integration."""
import sympy as S

a, r = S.symbols('a r')
b, c = 1+a, 1-a
D = 2*(b/c+c/b+1)
F = (D+b*(2*r-r*r)+c*(2/r-1/r**2))/16
sd = -(b*b*r+b*c+b*c/r**2+c*c/r**3)/8

# Rational part of a primitive independently obtained by multiplying the two
# Laurent polynomials above and integrating each monomial by hand.
G = -(-b**3*r**4/4+b**2*(2*b-c)*r**3/3
       +b**2*(D+2*c)*r**2/2+b*c*(b+D)*r
       -b*c*(c+D)/r-c**2*(D+2*b)/(2*r**2)
       -c**2*(2*c-b)/(3*r**3)+c**3/(4*r**4))/128
logcoef = -b*c/64
assert S.cancel(S.diff(G,r)+logcoef/r-F*sd)==0

def J(alpha, start, end):
    prim = G.subs(a,alpha)
    return prim.subs(r,end)-prim.subs(r,start)

def B(q):
    q=S.sympify(q)
    return 2/b*(q**3/3-q**4/4)
def E(q):
    q=S.sympify(q)
    return 2/b*(q**3/3+q**4/4)
def R(q):
    q=S.sympify(q)
    return 2/c*(q**3/3-q**4/4)
def Q(q): return a*q*q+B(q)

qp, qm, d = c/(3-a), b/(3+a), a*c/(3*a-1)
low = 2*(J(a,1,qp)+B(qp)+J(-a,1,qm)+E(a)+R(qm)-R(a))
mid = 2*(J(a,1,qp)+B(qp)+J(-a,1,(3*a-1)/c)+Q(1)-Q(d)+E(a))
high = 2*(Q(a)-Q(d)+J(a,c/(3*a-1),qp)+B(qp)+Q(1)-Q(a)+E(a))
low_target = (136+24*a*a-64*a**4)/(96*(1-a*a))
high_target = (100*a*a+120*a+100)/96
for name, expr, target in [('low',low,low_target),('middle',mid,high_target),('high',high,high_target)]:
    numerator, denominator = S.fraction(S.together(expr-target))
    polynomial = S.Poly(numerator,a)
    assert polynomial.is_zero
    print(name, ': rational-part difference numerator =',polynomial.as_expr())

# Logarithm terms are checked by their positive arguments, without log rewrite
# assumptions: LOW has qp*qm=(1-a²)/(9-a²), while MID and HIGH have ratio r0.
assert S.cancel(qp*qm-(1-a*a)/(9-a*a))==0
assert S.cancel(qp*(3*a-1)/c-(3*a-1)/(3-a))==0
assert S.cancel(qp/(c/(3*a-1))-(3*a-1)/(3-a))==0
print('primitive derivative and logarithm arguments also agree exactly')
