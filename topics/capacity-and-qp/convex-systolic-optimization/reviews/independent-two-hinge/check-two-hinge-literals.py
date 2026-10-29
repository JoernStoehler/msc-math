"""Independent exact printed-family and scalar-identity check, no submitted imports."""
from pathlib import Path
from itertools import combinations
exec((Path(__file__).parent/'check-weighted-witness.py').read_text().split('word=')[0])
def sign(v):
 v=E(v)
 if not v:return 0
 lo,hi=F(0),F(1)
 for _ in range(1000):
  mid=(lo+hi)/2
  if mid**4-10*mid**2+5>0:lo=mid
  else:hi=mid
  low=high=v.c[3]
  for c in reversed(v.c[:3]):
   vals=[low*lo,low*hi,high*lo,high*hi];low=min(vals)+c;high=max(vals)+c
  if low>0:return 1
  if high<0:return -1
 raise ValueError(v)
def power(v,n):
 out=E(1)
 for _ in range(n):out=out*v
 return out
alpha=(5-r)/20;b=r/10;k=r-2;A=alpha/2;B=(5+r)/40;C=(5+2*r)/10;mu=-1+2*r/5
families=[
 ([0,1,7,6,3,2,9],1,'L',[alpha,alpha,alpha,alpha,b,E(),b],[b,E(),alpha,alpha,alpha,alpha,b]),
 ([0,6,3,2,9,8,4],-1,'L',[alpha,b,E(),b,alpha,alpha,alpha],[b,b,alpha,alpha,alpha,alpha,E()]),
 ([0,7,3,9,4,5,1],1,'U',[alpha,b,b,alpha,E(),alpha,alpha],[E(),b,alpha,alpha,alpha,alpha,b]),
 ([0,4,6,5,2,1,8],-1,'U',[alpha,alpha,alpha,alpha,b,E(),b],[E(),b,alpha,alpha,alpha,alpha,b]),
]
T=5/(2*t);N=r-1;e=mul(1/N,J2(n[0]));smax=t/N;m=(1-1/N)/2;h=(1+1/N)/2
for word,edge,kind,b0,b1 in families:
 M=[a[i]+[E(1)] for i in word]
 for z in [E(),E(F(1,2)),E(1)]:
  weights=[(1-z)*x+z*y for x,y in zip(b0,b1)]
  assert sum(weights)==1 and all(sign(x)>=0 for x in weights)
  assert all(dot(weights,[a[j][k] for j in word])==0 for k in range(4))
  Q=sum(weights[i]*weights[j]*om(a[word[i]],a[word[j]]) for i in range(7) for j in range(i+1,7))
  assert Q==t/5
  g=[sum(weights[j]*(om(a[word[j]],a[word[i]]) if j<i else om(a[word[i]],a[word[j]])) for j in range(7) if j!=i) for i in range(7)]
  for ids in combinations(range(7),5):
   try:lam=solve([M[i] for i in ids],[g[i] for i in ids]);break
   except StopIteration:pass
  else:raise AssertionError('no pivot')
  assert all(dot(row,lam)==gi for row,gi in zip(M,g))
  x0=mul(-T,lam[:4]);x=x0
  expected_y=-1+(k+1)*z if kind=='L' else k+(1-k)*z
  for label,w in zip(word,weights):
   end=add(x,mul(2*T*w,Jc(a[label])))
   assert all(sign(dot(normal,x)-1)<=0 and sign(dot(normal,end)-1)<=0 for normal in a)
   assert dot(a[label],x)==1 and dot(a[label],end)==1
   if label==0:
    assert dot(e,x[:2])/smax==edge
    assert (dot(e,x[2:])-m)/h==expected_y
    assert dot(e,x[2:])==dot(e,end[2:])
    assert w==(A*(expected_y+3) if kind=='L' else B*(1-expected_y))
   x=end
  assert x==x0
print('PASS: all four literal printed words, oriented endpoint vectors and midpoint, normalization, nonnegativity, closure, action tau/5, full KKT equations, all segment facet containments, edge coordinates, affine y and occupation-weight laws.')
def lower_integral(z):return mu-z+b*((z+1)*(z+1)+power(z+1,3)/6)
def down_difference(z):return 5*A*(z+3)*(z+3)/4-(z-mu)
zstar=1+4*r/5
for z in [E(),E(1),E(2)]:assert down_difference(z)==5*A*(z-zstar)*(z-zstar)/4
assert 20*A-(1-mu)==(5-r)/10
assert 5*b*(k+1)-(mu+1)==(25-9*r)/10
assert 5*b*k-lower_integral(E())==(210-91*r)/60
assert 5*B/4-lower_integral(E())==(555-233*r)/480
assert 5*B*(1-k)*(1-k)/4-lower_integral(k)==7*B*(1-k)*(1-k)/12
assert 5*B/2-3*b==(25-19*r)/80
assert C*(1-k)/B==4
for v in [(5-r)/10,(25-9*r)/10,(210-91*r)/60,(555-233*r)/480,(5*b-1),k,1-k]:assert sign(v)>0
assert sign((25-19*r)/80)<0
mass=b*((k*k-1)/2+3*(k+1))+C*((1-k)-(1-k*k)/2)
mean=b*((power(k,3)+1)/3+3*(k*k-1)/2)+C*((1-k*k)/2-(1-power(k,3))/3)
assert mass==1 and mean==mu
print('PASS: scalar perfect-square identity, endpoint comparison identities, concavity sign, upper-density ratio, density normalization and mean. The interval partition, Jensen and two-cut arguments are reviewed analytically in REPORT-r6-delta.md.')
