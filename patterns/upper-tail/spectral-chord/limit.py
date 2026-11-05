#!/usr/bin/env python3
"""Exact first-order principal eigenspace split, Q(sqrt5,sqrt(D))."""
import importlib.util,json,math,hashlib
from fractions import Fraction as F
from pathlib import Path
D=Path(__file__).parent;R=Path(__file__).resolve().parents[3] if 'patterns' in Path(__file__).parts else Path('/workspaces/msc-math-worktrees/patterns-2026-10-17');sp=importlib.util.spec_from_file_location('q5',R/'patterns/upper-tail/conditional-coupling/quadratic.py');q=importlib.util.module_from_spec(sp);sp.loader.exec_module(q);Q=q.Q5
class J:
 def __init__(self,x=0,d=0):
  if isinstance(x,J):self.x,self.d=x.x,x.d
  else:self.x,self.d=Q(x),Q(d)
 def __add__(self,o):o=J(o);return J(self.x+o.x,self.d+o.d)
 __radd__=__add__
 def __neg__(self):return J(-self.x,-self.d)
 def __sub__(self,o):return self+-J(o)
 def __rsub__(self,o):return J(o)+-self
 def __mul__(self,o):o=J(o);return J(self.x*o.x,self.d*o.x+self.x*o.d)
 __rmul__=__mul__
 def __truediv__(self,o):o=J(o);return J(self.x/o.x,(self.d*o.x-self.x*o.d)/(o.x*o.x))
 def __rtruediv__(self,o):return J(o)/self
 def __bool__(self):return bool(self.x or self.d)
 def __eq__(self,o):o=J(o);return self.x==o.x and self.d==o.d
 def __lt__(self,o):o=J(o);return self.x<o.x if self.x!=o.x else self.d<o.d
 def __le__(self,o):return self<o or self==o
 def __gt__(self,o):return J(o)<self
 def __ge__(self,o):return J(o)<=self
 def __abs__(self):return self if self>=0 else -self
 def __float__(self):return float(self.x)
 def serial(self):return dict(value=self.x.serial(),derivative=self.d.serial())
class E:
 # Same quadratic discriminant within one word.
 rad=Q(1)
 def __init__(self,a=0,b=0):
  if isinstance(a,E):self.a,self.b=a.a,a.b
  else:self.a,self.b=Q(a),Q(b)
 def __add__(self,o):o=E(o);return E(self.a+o.a,self.b+o.b)
 __radd__=__add__
 def __neg__(self):return E(-self.a,-self.b)
 def __sub__(self,o):return self+-E(o)
 def __rsub__(self,o):return E(o)+-self
 def __mul__(self,o):o=E(o);return E(self.a*o.a+E.rad*self.b*o.b,self.a*o.b+self.b*o.a)
 __rmul__=__mul__
 def __truediv__(self,o):o=E(o);dd=o.a*o.a-E.rad*o.b*o.b;return self*E(o.a/dd,-o.b/dd)
 def __rtruediv__(self,o):return E(o)/self
 def __bool__(self):return bool(self.a or self.b)
 def sign(self):
  a,b=self.a,self.b
  if not b:return a.sign()
  if not a:return b.sign()
  if a.sign()==b.sign():return a.sign()
  return (a*a-E.rad*b*b).sign()*a.sign()
 def __eq__(self,o):o=E(o);return (self-o).sign()==0
 def __lt__(self,o):return (self-E(o)).sign()<0
 def __le__(self,o):return (self-E(o)).sign()<=0
 def __gt__(self,o):return (self-E(o)).sign()>0
 def __ge__(self,o):return (self-E(o)).sign()>=0
 def __abs__(self):return self if self>=0 else -self
 def __float__(self):return float(self.a)+math.sqrt(float(E.rad))*float(self.b)
 def serial(self):return dict(a=self.a.serial(),sqrt_discriminant_coefficient=self.b.serial())
def dot(a,b):return sum(x*y for x,y in zip(a,b))
def cross(a,b):return a[0]*b[1]-a[1]*b[0]
def trans(a):return list(map(list,zip(*a)))
def times(a,b):return [[dot(u,v) for v in trans(b)] for u in a]
def apply(a,v):return [dot(u,v) for u in a]
def inverse(a):
 n=len(a);z=[u[:]+[J(i==j) if isinstance(u[0],J) else Q(i==j) for j in range(n)] for i,u in enumerate(a)]
 for j in range(n):
  k=next(i for i in range(j,n) if z[i][j]);z[j],z[k]=z[k],z[j];v=z[j][j];z[j]=[x/v for x in z[j]]
  for i in range(n):
   if i!=j:
    v=z[i][j];z[i]=[x-v*y for x,y in zip(z[i],z[j])]
 return [u[n:] for u in z]
def poly(a):
 vs=[]
 for i in range(5):
  u,v=a[i],a[(i+1)%5];det=cross(u,v);vs.append([(v[1]-u[1])/det,(u[0]-v[0])/det])
 twice=sum(cross(vs[i],vs[(i+1)%5]) for i in range(5));center=[sum((vs[i][k]+vs[(i+1)%5][k])*cross(vs[i],vs[(i+1)%5]) for i in range(5))/(3*twice) for k in [0,1]];centered=[[x/(1-dot(u,center)) for x in u] for u in a];h=[[J(1)]*5]+trans(centered);v=times(times(trans(h),inverse(times(h,trans(h)))),h);projector=[[J(i==j)-v[i][j] for j in range(5)] for i in range(5)];return centered,projector

def word(a,b,sign):
 from functools import cmp_to_key
 vec=a+[[sign*-u[1],sign*u[0]] for u in b]
 def half(v):return 0 if v[1]>0 or v[1]==0 and v[0]>=0 else 1
 def cmp(i,j):
  h=half(vec[i])-half(vec[j])
  if h:return h
  c=cross(vec[i],vec[j]);return -1 if c>0 else 1 if c<0 else (i>j)-(i<j)
 w=sorted(range(10),key=cmp_to_key(cmp));return w if sign>0 else w[::-1]
def ends(c,u):
 lo=max(-x/y for x,y in zip(c,u) if y>0);hi=min(-x/y for x,y in zip(c,u) if y<0);return [[x+t*y for x,y in zip(c,u)] for t in [lo,hi]]
def serial(v):
 if hasattr(v,'serial'):return v.serial()
 if isinstance(v,F):return str(v)
 if isinstance(v,dict):return {k:serial(x) for k,x in v.items()}
 if isinstance(v,(tuple,list)):return [serial(x) for x in v]
 return v
s=Q(0,1);a=[[J(1),J(0,1)],[J((s-1)/4),J(1)],[J(-(s+1)/4),J((s-1)/2)],[J(-(s+1)/4),J(-(s-1)/2)],[J((s-1)/4),J(-1)]];b=[[J(0),J(1)],[J(-1),J((s-1)/4)],[J(-(s-1)/2),J(-(s+1)/4)],[J((s-1)/2),J(-(s+1)/4)],[J(1),J((s-1)/4)]];aa,pa=poly(a);bb,pb=poly(b);rows=[]
for sign in [1,-1]:
 w=word(aa,bb,sign);pos={i:k for k,i in enumerate(w)};ss=[[(1 if pos[i]<pos[5+j] else -1)*dot(u,v)/4 for j,v in enumerate(bb)] for i,u in enumerate(aa)];m=times(times(pa,ss),pb);h=times(m,trans(m));h0=[[v.x for v in row] for row in h];h1=[[v.d for v in row] for row in h];p0=[[v.x for v in row] for row in pa];eigenvalue0=sum(h0[i][i] for i in range(5))/2;assert eigenvalue0>0 and all(h0[i][j]==eigenvalue0*p0[i][j] for i in range(5) for j in range(5));basis=[[row[3],row[4]] for row in p0];gram=times(trans(basis),basis);restricted=times(trans(basis),times(h1,basis));k=times(inverse(gram),restricted);trace=k[0][0]+k[1][1];det=k[0][0]*k[1][1]-k[0][1]*k[1][0];disc=trace*trace-4*det;assert disc>0;E.rad=disc;lam=E(trace/2,Q(F(1,2)));ev=[E(k[0][1]),lam-k[0][0]];u=[E(row[0])*ev[0]+E(row[1])*ev[1] for row in basis];m0=[[v.x for v in row] for row in m];v=apply(trans([[E(x) for x in row] for row in m0]),u);ac=[E(F(1,5))]*5;bc=ac[:];s0=[[E(v.x) for v in row] for row in ss];candidates=[dot(alpha,apply(s0,gamma)) for alpha in ends(ac,u) for gamma in ends(bc,v)];best=max(candidates);capacity=1/(2*best);assert capacity>Q(F(5015,1000));reference_candidates=[]
 for role,proj in [('A',p0),('B',[[v.x for v in row] for row in pb])]:
  for coordinate in range(5):
   direction=[E(row[coordinate]) for row in proj]
   if role=='A':ax=direction;by=apply(trans([[E(x) for x in row] for row in m0]),direction)
   else:by=direction;ax=apply([[E(x) for x in row] for row in m0],direction)
   reference_candidates += [dot(alpha,apply(s0,gamma)) for alpha in ends(ac,ax) for gamma in ends(bc,by)]
 reference_best=max(reference_candidates);assert reference_best==Q(F(1,10));gap=capacity/5-1;print(sign,'disc',disc,'limit capacity',float(capacity),'gap',float(gap),flush=True);rows.append(serial(dict(sign=sign,word=w,discriminant=disc,H0=h0,H1=h1,M0=m0,M1=[[v.d for v in row] for row in m],P0=p0,basis=basis,reference_Q=reference_best,reference_candidate_count=len(reference_candidates),restricted_operator=k,eigenvalue_derivative=lam,left_line=u,right_line=v,Q_limit=best,capacity_limit=capacity,relative_gap=gap,candidate_Q=candidates)))
(D/'limit.json').write_text(json.dumps(dict(rows=rows,code_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),quadratic_field_sha256=hashlib.sha256((R/'patterns/upper-tail/conditional-coupling/quadratic.py').read_bytes()).hexdigest(),interpretation='Exact right-derivative eigenspace calculation; mathematical proof of limit implication needs separate review.'),indent=2)+'\n')
