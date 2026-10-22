"""Exact interval verification of the numerically found four-contact candidate."""
import importlib.util
from pathlib import Path
p=Path(__file__).resolve().parent.parent/'tensor-certificate-20261022/check_certificate.py'
s=importlib.util.spec_from_file_location('cert',p); c=importlib.util.module_from_spec(s); s.loader.exec_module(c)
I=c.I; tr=c.trig; S=c.S
norm5=[[tr(2*i,5,'cos'),tr(2*i,5,'sin')] for i in range(5)]
norm7=[[tr(2*j,7,'cos'),tr(2*j,7,'sin')] for j in range(7)]
active=[(0,3,1),(3,0,1),(1,1,-1),(4,5,-1)]
def row(i,j,e):
 x,y=norm5[i]; a,b=norm7[j]
 return [e*x*a,e*x*b,e*y*a,e*y*b]
A=[row(i,j,e) for i,j,e in active]
# interval Gauss-Jordan solve Ax=1
m=[r+[I.c(1) for k in range(4)] for i,r in enumerate(A)]
for i in range(4):
 if not(m[i][i].positive() or m[i][i].negative()):
  j=next(j for j in range(i+1,4) if m[j][i].positive() or m[j][i].negative())
  m[i],m[j]=m[j],m[i]
 pvt=m[i][i]; m[i]=[v/pvt for v in m[i]]
 for j in range(4):
  if i!=j:
   q=m[j][i]; m[j]=[m[j][k]-q*m[i][k] for k in range(8)]
x=[m[i][4] for i in range(4)]
assert all(xi.lo<xi.hi for xi in x)
# Check every signed inequality except the four defining contacts.
min_slack=None; checked=0
for i in range(5):
 for j in range(7):
  for e in (-1,1):
   if (i,j,e) in active: continue
   v=sum((a*b for a,b in zip(row(i,j,e),x)),I.c(0))
   assert v.hi < S, (i,j,e,v.lo/S,v.hi/S)
   slack=S-v.hi; min_slack=slack if min_slack is None else min(min_slack,slack); checked+=1
# det = x0*x3-x1*x2
D=x[0]*x[3]-x[1]*x[2]
assert D.lo > S
print('other signed inequalities checked:',checked)
print('candidate coordinate intervals:')
for v in x: print(v.lo/S,v.hi/S)
print('minimum certified slack lower bound:',min_slack/S)
print('determinant endpoints:',D.lo,D.hi,'over',S)
assert D.lo*10**7 > 10063055*S
print('proved lower bound: det > 1.0063055')
rot=I.c(1)/(tr(1,70,'cos')*tr(1,70,'cos'))
gain=D-rot
assert gain.lo*10000 > 42*S
print('proved improvement over best rotation: >0.0042 in normalized determinant')
