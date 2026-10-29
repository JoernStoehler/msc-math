"""Independent exact six-dimensional certificate audit, standard library only."""
from fractions import Fraction as F
from itertools import combinations, permutations
from math import factorial

class Q:
    def __init__(self,a=0,b=0):
        if isinstance(a,Q):self.a,self.b=a.a,a.b
        else:self.a,self.b=F(a),F(b)
    def __add__(self,o):o=Q(o);return Q(self.a+o.a,self.b+o.b)
    __radd__=__add__
    def __neg__(self):return Q(-self.a,-self.b)
    def __sub__(self,o):return self+-Q(o)
    def __rsub__(self,o):return Q(o)+-self
    def __mul__(self,o):o=Q(o);return Q(self.a*o.a+5*self.b*o.b,self.a*o.b+self.b*o.a)
    __rmul__=__mul__
    def __truediv__(self,o):
        o=Q(o);den=o.a*o.a-5*o.b*o.b
        return self*Q(o.a/den,-o.b/den)
    def __eq__(self,o):o=Q(o);return self.a==o.a and self.b==o.b
    def __bool__(self):return bool(self.a or self.b)
    def sign(self):
        if not self.b:return (self.a>0)-(self.a<0)
        if not self.a:return (self.b>0)-(self.b<0)
        if self.a*self.b>0:return (self.a>0)-(self.a<0)
        d=self.a*self.a-5*self.b*self.b
        return ((self.a>0)-(self.a<0))*((d>0)-(d<0))
    def __repr__(self):return f'({self.a})+({self.b})sqrt5'

E=list(combinations(range(6),2)); ix={e:k for k,e in enumerate(E)}
def form(terms):
    v=[0]*15
    for i,j,c in terms:v[ix[i-1,j-1]]=c
    return v
V=[form([(1,3,1),(1,5,-1),(3,5,1)]),form([(2,4,-1),(2,6,1),(4,6,-1)]),form([(1,2,1)]),form([(3,4,1)]),form([(5,6,1)])]
orders=list(permutations(range(6)))
P=[]
for order in orders:
    pos={x:i for i,x in enumerate(order)}
    P.append([1 if pos[i]<pos[j] else -1 for i,j in E])
def dot(a,b):return sum(x*y for x,y in zip(a,b))
D=[[dot(p,v) for v in V] for p in P]
assert all(x in(-1,1) for row in D for x in row)
active=[i for i,row in enumerate(D) if row==[1]*5]
assert len(active)==21

def rank(rows):
    A=[list(map(F,row)) for row in rows];r=0
    for c in range(len(A[0])):
        p=next((k for k in range(r,len(A)) if A[k][c]),None)
        if p is None:continue
        A[r],A[p]=A[p],A[r];div=A[r][c];A[r]=[x/div for x in A[r]]
        for k in range(r+1,len(A)):
            if A[k][c]:
                div=A[k][c];A[k]=[x-div*y for x,y in zip(A[k],A[r])]
        r+=1
        if r==len(A):break
    return r
assert rank([P[i] for i in active])==11
assert rank(V)==5
isolators=['125364','123456','213564','142356','123645']
for j,p in enumerate(isolators):
    idx=orders.index(tuple(int(x)-1 for x in p));expected=[1]*5;expected[j]=-1
    assert D[idx]==expected

# A different Pfaffian calculation: signed perfect matchings, sign by crossings.
def matchings(vertices):
    if not vertices:yield [];return
    i=vertices[0]
    for j in vertices[1:]:
        for rest in matchings([v for v in vertices if v not in(i,j)]):yield [(i,j)]+rest
terms=[]
for match in matchings(list(range(6))):
    crosses=sum(i<k<j<l or k<i<l<j for (i,j),(k,l) in combinations(match,2))
    terms.append(((-1)**crosses,[ix[e] for e in match]))
assert len(terms)==15

def pf(x):
    return sum(sign*x[e[0]]*x[e[1]]*x[e[2]] for sign,e in terms)
def grad(x):
    out=[Q() for _ in E]
    for sign,edges in terms:
        for k,e in enumerate(edges):out[e]+=sign*x[edges[(k+1)%3]]*x[edges[(k+2)%3]]
    return out

def hess(x):
    H=[[Q() for _ in E] for _ in E]
    for sign,edges in terms:
        for i,j in permutations(range(3),2):H[edges[i]][edges[j]]+=sign*x[edges[3-i-j]]
    return H

# Recover the full cubic on five variables by sparse polynomial expansion.
poly={}
for sign,edges in terms:
    for i,j,k in __import__('itertools').product(range(5),repeat=3):
        val=sign*V[i][edges[0]]*V[j][edges[1]]*V[k][edges[2]]
        if val:
            key=tuple(sorted((i,j,k)));poly[key]=poly.get(key,0)+val
poly={k:v for k,v in poly.items() if v}
assert poly=={(0,1,2):1,(0,1,3):1,(0,1,4):1,(2,3,4):1}

a=Q(F(13,62),F(3,62));b=Q(F(6,31),F(-1,31))
assert 2*a+3*b==1 and a.sign()>0 and b.sign()>0 and (a-b).sign()>0
assert a*a+b*b==3*a*b
x=[sum(c*V[j][i] for j,c in enumerate([a,a,b,b,b])) for i in range(15)]
assert pf(x)==Q(F(63,1922),F(5,1922))
assert pf(x)==a*b
# Exact all720 ordering feasibility and active-set identification.
values=[dot(p,x) for p in P]
assert all((1-v).sign()>=0 for v in values)
assert [i for i,v in enumerate(values) if v==1]==active
alpha=b*b/6;beta=b*(a-b)
assert alpha.sign()>0 and beta.sign()>0
cyclic={0:2,2:4,4:0,1:3,3:5,5:1}
p=tuple(int(t)-1 for t in '135264');special=[]
for _ in range(3):special.append(orders.index(p));p=tuple(cyclic[t] for t in p)
certificate=[sum((beta if k in special else alpha)*P[k][i] for k in active) for i in range(15)]
assert certificate==grad(x)

# Restricted negative definiteness via exact Sylvester minors in a separate basis.
T=[[V[i][j]-V[4][j] for j in range(15)] for i in range(4)]
H=hess(x)
HF=[[sum(T[i][k]*H[k][l]*T[j][l] for k in range(15) for l in range(15)) for j in range(4)] for i in range(4)]
def det(A):
    n=len(A)
    return sum((-1)**sum(p[i]>p[j] for i in range(n) for j in range(i+1,n))*__import__('functools').reduce(lambda a,b:a*b,(A[i][p[i]] for i in range(n)),Q(1)) for p in permutations(range(n)))
minors=[]
for k in range(1,5):
    d=det([row[:k] for row in HF[:k]])
    assert ((-1)**k*d).sign()>0;minors.append(d)

# Seven-facet relabeling stabilizer, using irrational independence of a,b.
Aform=[V[0][k]+V[1][k] for k in range(15)]
Bform=[V[2][k]+V[3][k]+V[4][k] for k in range(15)]
def extended(v):
    M=[[0]*7 for _ in range(7)]
    for (i,j),c in zip(E,v):M[i][j]=c;M[j][i]=-c
    for i in range(6):M[i][6]=-sum(M[i][:6]);M[6][i]=-M[i][6]
    return M
MA,MB=extended(Aform),extended(Bform)
stabilizer=[];antistabilizer=[]
for p in permutations(range(7)):
    coordsA=[MA[p[i]][p[j]] for i,j in E];coordsB=[MB[p[i]][p[j]] for i,j in E]
    if coordsA==Aform and coordsB==Bform:stabilizer.append(p)
    if coordsA==[-v for v in Aform] and coordsB==[-v for v in Bform]:antistabilizer.append(p)
assert len(stabilizer)==3 and len(antistabilizer)==3

print('All720 exact order scores checked;21 active, normal rank11, face dimension4.')
print('All five isolators checked; exact F32 cubic independently recovered by perfect matchings.')
print('Pfaffian:',pf(x),'ratio:',15*pf(x))
print('Positive gradient coefficients:',alpha,beta)
print('Negative-definite restricted Hessian: leading minors',minors)
print('Seven-facet symplectic stabilizer size:',len(stabilizer),'maps to minusform:',len(antistabilizer))
print('stabilizer:',[tuple(i+1 for i in p) for p in stabilizer])
print('PASS')
