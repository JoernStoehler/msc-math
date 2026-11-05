#!/usr/bin/env python3
"""Exact constant check in Q(sqrt(5)); standard library, no write effects."""
from fractions import Fraction as F

def add(x,y): return (x[0]+y[0],x[1]+y[1])
def mul(x,y): return (x[0]*y[0]+5*x[1]*y[1],x[0]*y[1]+x[1]*y[0])
def inv(x):
    d=x[0]**2-5*x[1]**2
    return (x[0]/d,-x[1]/d)
def div(x,y): return mul(x,inv(y))
A=(F(1,8),F(-1,40)); B=(F(1,8),F(1,40))
b=(F(0),F(1,10)); C=(F(1,2),F(1,5))
k1=(F(-1),F(1)); onek=(F(3),F(-1))
assert mul(div(b,A),k1)==(F(4),F(0))
assert mul(div(C,B),onek)==(F(4),F(0))
print('PASS: lower density/occupation integral=4; upper=4; total=8; R>=5nu/4.')
