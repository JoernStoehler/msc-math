#!/usr/bin/env python3
"""Independent exact quarter-turn check, all 6! restricted word orders.

Steps are s*e,s*a,s*b in each factor, with a=(-1/2,t/2),
b=(-1/2,-t/2), and the second factor rotated by J. Each actual
product step carries mass 1/2. Q/(rho*t) is rational, rho=s*s/2.
No numerical geometry, capacity producer or population is used.
"""
from fractions import Fraction as F
from itertools import permutations
import json
steps=[(F(1),F(0)),(F(-1,2),F(1,2)),(F(-1,2),F(-1,2))]
# Pairings divided by t; second coordinate of raw steps is t*y.
def pairing(i,j):
    if (i<3)==(j<3):return F(0)
    if i>=3:return -pairing(j,i)
    x,y=steps[i];u,v=steps[j-3]
    return -x*v+y*u
values=[]
for order in permutations(range(6)):
    # beta contributes s^2/4; rho=s^2/2.
    values.append(sum((pairing(order[i],order[j]) for i in range(6) for j in range(i+1,6)),F(0))/2)
assert len(values)==720 and max(values)==1 and min(values)==-1
# Identity (tan36/cos18)^2 = (3-sqrt(5))^2 is checked
# without transcendental evaluation, using tan^2=5-2sqrt5,
# cos^2=(5+sqrt5)/8. Both sides are positive.
# Q(sqrt5) represented by pairs (a,b), a+b*sqrt5.
def mul(x,y):return (x[0]*y[0]+5*x[1]*y[1],x[0]*y[1]+x[1]*y[0])
assert mul(mul((F(3),F(-1)),(F(3),F(-1))),(F(5,8),F(1,8)))==(F(5),F(-2))
print(json.dumps({'restricted_orders':len(values),'max_Q_over_rho_t':str(max(values)),'min_Q_over_rho_t':str(min(values)),'positive_maximizers':values.count(F(1)),'exact_quadratic_field_identity':True,'scope':'Quarter-turn restricted-triple objective and constant identity only; averaging proof requires separate mathematical review.'},indent=2))
