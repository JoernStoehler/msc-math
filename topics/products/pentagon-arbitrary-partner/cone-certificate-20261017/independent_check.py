"""Independent exact arithmetic, rotation construction and certificate obstruction audit."""
from fractions import Fraction as F
from pathlib import Path
import json, hashlib, sys

OUT = Path(__file__).resolve().parent
SRC = Path(sys.argv[1])

class E:
    def __init__(self,a=0,b=0): self.a,self.b=F(a),F(b)
    def __add__(self,x):
        x=co(x); return E(self.a+x.a,self.b+x.b)
    __radd__=__add__
    def __neg__(self): return E(-self.a,-self.b)
    def __sub__(self,x): return self+-co(x)
    def __rsub__(self,x): return co(x)+-self
    def __mul__(self,x):
        x=co(x);return E(self.a*x.a+5*self.b*x.b,self.a*x.b+self.b*x.a)
    __rmul__=__mul__
    def __truediv__(self,x):
        x=co(x);d=x.a*x.a-5*x.b*x.b;assert d
        return self*E(x.a/d,-x.b/d)
    def __eq__(self,x):
        x=co(x);return self.a==x.a and self.b==x.b
    def sign(self):
        if self.b==0:return (self.a>0)-(self.a<0)
        if self.a==0:return (self.b>0)-(self.b<0)
        if self.a>0 and self.b>0:return 1
        if self.a<0 and self.b<0:return -1
        gap=self.a*self.a-5*self.b*self.b
        assert gap!=0
        return ((gap>0)-(gap<0))*((self.a>0)-(self.a<0))
    def enc(self):return [str(self.a),str(self.b)]
    def __repr__(self):return str(self.enc())
def co(x):return x if isinstance(x,E) else E(x)
def dec(x):return E(*x)
def dot(x,y):return sum((a*b for a,b in zip(x,y)),E())
def matmul(A,B):return [[sum((a*b for a,b in zip(row,col)),E()) for col in zip(*B)] for row in A]
def mv(A,x):return [dot(row,x) for row in A]
def det2(B):return B[0][0]*B[1][1]-B[0][1]*B[1][0]

c=json.loads(SRC.read_text());rows=[[dec(v) for v in row['coefficients']] for row in c['rows']]
Q=[[dec(v) for v in row] for row in c['positive_det_quadratic_matrix']]
assert len(rows)==610 and all(len(row)==7 for row in rows)
r=E(0,1);b=(1+r)/4;a=(r-1)/4
# Construct directly in the physical B=SAS^-1 coordinate convention.
# C conjugates a physical36degree rotation; unlike point coordinates its
# upper off-diagonal is -sin36/tau=-cos36, lower is tau*sin36.
C=[[b,-b],[(3*r-5)/4,b]]
assert det2(C)==1
assert matmul(C,[[b,b],[-(3*r-5)/4,b]])==[[E(1),E()],[E(),E(1)]]
B=[[E(1),-r/5],[r-2,E(1)]]
# Reconstruct gauge support coordinates from an ungauged constant support.
N=[[dec(v) for v in row] for row in c['normals']]
assert N[3][0]!=N[4][0]
# Solve N3.z=N4.z=-s, then h_i=s+Ni.z; source HKO support fixes s.
H0=[-1+3*r/5,1-r/5,-1+3*r/5]
gauge=[[dec(v) for v in row] for row in c['support_gauge_projection']]
gconst=mv(gauge,[E(1)]*5)
s=H0[0]/gconst[0]
assert mv(gauge,[s]*5)==H0
assert H0+[E(1),-r/5,r-2,E(1)]==[dec(v) for v in c['hko_control']['point']]
I=[[E(1),E()],[E(),E(1)]]
power=I;points=[];slacks=[];target_gaps=[]
for k in range(10):
    Bt=matmul(power,B);x=H0+Bt[0]+Bt[1]
    assert det2(Bt).sign()>0
    values=[dot(row,x) for row in rows]
    assert all(v.sign()>=0 for v in values)
    p=sum((H0[i]*H0[i+1] for i in range(2)),E())-a*dot(H0,H0)
    gap=p-det2(Bt)/(3+r)
    assert gap==0 and dot(x,mv(Q,x))==0
    points.append(x);slacks.append(values);target_gaps.append(gap)
    power=matmul(power,C)
assert power==I
assert len({tuple(tuple(v.enc()) for v in x) for x in points})==10
zeros=[sum(v==0 for v in vals) for vals in slacks]
row_zeros=[sum(vals[i]==0 for vals in slacks) for i in range(610)]
assert zeros==[20]*10
assert max(row_zeros)<=2
# Each pair is strictly positive wherever neither row is zero.
minimum_common_positive=10
pair_hist={}
for i in range(610):
    for j in range(i,610):
        n=sum(slacks[k][i].sign()>0 and slacks[k][j].sign()>0 for k in range(10))
        minimum_common_positive=min(minimum_common_positive,n)
        pair_hist[n]=pair_hist.get(n,0)+1
assert minimum_common_positive>=6
# Negative ambient control suffices to exclude PSD, without trusting inertia.
negative=[E(1)]+[E()]*6
qnegative=dot(negative,mv(Q,negative))
assert qnegative==(1-r)/4 and qnegative.sign()<0
receipt={'status':'PASS','source_sha256':hashlib.sha256(SRC.read_bytes()).hexdigest(),
 'construction':'ten physical36degree rotations of exact HKO B; gauged regular supports fixed',
 'points':[[v.enc() for v in x] for x in points],
 'active_rows_per_point':zeros,'row_zero_incidence_histogram':{str(n):row_zeros.count(n) for n in sorted(set(row_zeros))},
 'minimum_common_positive_rays_per_pair':minimum_common_positive,'pair_positive_incidence_histogram':pair_hist,
 'ambient_negative_control_value':qnegative.enc(),
 'proof':'If Q=x^TSx+sum_ij c_ij ell_i ell_j globally, S PSD and c_ij nonnegative, evaluating at any of ten feasible Qzeros makes each summand zero. Every ell_i ell_j is positive at some zero, forcing every c_ij=0. Then Q=S is PSD, contradicted by Q(e0)<0.',
 'scope':'Excludes global quadratic identity with ambient PSD remainder plus nonnegative products of the610retained linear rows. Does not settle Qpositivity on C or exclude sector constraints, subdivisions, higher degree, or different certificates.'}
print(json.dumps(receipt,indent=2))
