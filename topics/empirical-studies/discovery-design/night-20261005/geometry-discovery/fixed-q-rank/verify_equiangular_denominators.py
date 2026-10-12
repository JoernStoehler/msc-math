# /// script
# dependencies = ["sympy"]
# ///
"""Bounded exact coefficient check; theorem proof is equiangular-capacity-core.md."""
import sympy as s
z=s.sqrt(5)
b=(1+z)/4
w=1+b
R=(5-z)/10
rot=s.Matrix([[b,-(3*z-5)/4],[b,b]])
n=[s.Matrix(v) for v in [(1,0),((z-1)/4,(5-z)/4),(-(1+z)/4,(3*z-5)/4),(-(1+z)/4,-(3*z-5)/4),((z-1)/4,-(5-z)/4)]]
base=[s.Matrix([-R,0]),s.Matrix([R,0]),s.Matrix([0,R])]
tris=[]
cur=base
for j in range(10):
    tris.append(cur)
    cur=[s.simplify(rot*v) for v in cur]
assert all(s.simplify(cur[k]-base[k])==s.zeros(2,1) for k in range(3))
lam0=[1/(2*w),1/(2*w),0,b/w,0]
lams=[[lam0[(i-l)%5] for i in range(5)] for l in range(5)]
assert all(s.simplify(sum((lam[i]*n[i] for i in range(5)),s.zeros(2,1)))==s.zeros(2,1) for lam in lams)
# Compare exactly in Q(sqrt5); no numerical ordering is used.
def cmp_sign(v):
    v=s.simplify(s.expand(v))
    a=s.simplify(v.coeff(z,0)); c=s.simplify(v.coeff(z,1))
    assert s.simplify(v-a-c*z)==0 and a.is_Rational and c.is_Rational
    if not c: return s.sign(a)
    if not a: return s.sign(c)
    if a>0 and c>0:return 1
    if a<0 and c<0:return -1
    q=s.sign(a*a-5*c*c)
    return q if a>0 else -q

def exactmax(vals):
    out=vals[0]
    for v in vals[1:]:
        if cmp_sign(v-out)>0:out=v
    return s.simplify(out)
supports=[[exactmax([(n[i].T*v)[0] for v in tri]) for i in range(5)] for tri in tris]
expected=(z-1)/5
for l,lam in enumerate(lams):
    ds=[s.simplify(sum(lam[i]*supp[i] for i in range(5))) for supp in supports]
    maximum=exactmax(ds)
    assert s.simplify(maximum-expected)==0
    assert all(cmp_sign(d)>0 for d in ds)
    print('circuit',l,'maximum=',maximum,'argmax=',[j for j,d in enumerate(ds) if s.simplify(d-maximum)==0])
print('D0=b/w² identity:',s.simplify(b/w**2-expected))
print('All 50 positive denominators checked, with exact comparisons in Q(sqrt5).')
