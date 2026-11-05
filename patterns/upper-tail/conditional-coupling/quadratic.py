"""Exact ordered field Q(sqrt(5)); standard library only."""
from fractions import Fraction as F
from functools import total_ordering
@total_ordering
class Q5:
 def __init__(self,a=0,b=0):
  if isinstance(a,Q5):self.a,self.b=a.a,a.b
  else:self.a,self.b=F(a),F(b)
 def __add__(self,o):
  o=Q5(o);return Q5(self.a+o.a,self.b+o.b)
 __radd__=__add__
 def __neg__(self):return Q5(-self.a,-self.b)
 def __sub__(self,o):return self+-Q5(o)
 def __rsub__(self,o):return Q5(o)+-self
 def __mul__(self,o):
  o=Q5(o);return Q5(self.a*o.a+5*self.b*o.b,self.a*o.b+self.b*o.a)
 __rmul__=__mul__
 def __truediv__(self,o):
  o=Q5(o);d=o.a*o.a-5*o.b*o.b
  return self*Q5(o.a/d,-o.b/d)
 def __rtruediv__(self,o):return Q5(o)/self
 def __bool__(self):return bool(self.a or self.b)
 def sign(self):
  a,b=self.a,self.b
  if not a:return (b>0)-(b<0)
  if not b:return (a>0)-(a<0)
  if (a>0)==(b>0):return 1 if a>0 else -1
  d=a*a-5*b*b
  return ((d>0)-(d<0))*(1 if a>0 else -1)
 def __eq__(self,o):
  try:o=Q5(o)
  except (TypeError,ValueError):return False
  return self.a==o.a and self.b==o.b
 def __lt__(self,o):return (self-Q5(o)).sign()<0
 def __abs__(self):return self if self>=0 else -self
 def __float__(self):return float(self.a)+float(self.b)*5**.5
 def __repr__(self):return f'Q5({self.a},{self.b})'
 def serial(self):return {'rational':str(self.a),'sqrt5':str(self.b)}
 @staticmethod
 def parse(x):return Q5(x['rational'],x['sqrt5']) if isinstance(x,dict) else Q5(x)
