"""Exact dyadic interval certificate of heptagon paired-triangle tensor containment.

No floating trigonometry or convex-hull output is trusted by verify(). Discovery
uses SciPy only to propose four signed rank-one generators per representative.
"""
import argparse, itertools, json, math
from pathlib import Path
from fractions import Fraction
BITS=160
S=1<<BITS
class I:
    def __init__(self,lo,hi=None):self.lo=int(lo);self.hi=int(lo if hi is None else hi)
    @staticmethod
    def q(x):
        x=Fraction(x);return I(x.numerator*S//x.denominator,-((-x.numerator*S)//x.denominator))
    def __add__(a,b):
        if not isinstance(b,I): b=I.q(b)
        return I(a.lo+b.lo,a.hi+b.hi)
    __radd__=__add__
    def __neg__(a):return I(-a.hi,-a.lo)
    def __sub__(a,b):return a+-b if isinstance(b,I) else a+I.q(-b)
    def __rsub__(a,b):return -a+b
    def __mul__(a,b):
        if not isinstance(b,I):b=I.q(b)
        p=[a.lo*b.lo,a.lo*b.hi,a.hi*b.lo,a.hi*b.hi]
        return I(min(p)//S,-((-max(p))//S))
    __rmul__=__mul__
    def __truediv__(a,b):
        if not isinstance(b,I):b=I.q(b)
        assert b.lo*b.hi>0,'division by interval containing zero'
        p=[Fraction(a.lo*S,b.lo),Fraction(a.lo*S,b.hi),Fraction(a.hi*S,b.lo),Fraction(a.hi*S,b.hi)]
        low=min(p);high=max(p)
        return I(low.numerator//low.denominator,-((-high.numerator)//high.denominator))
    def absmax(a):return max(abs(a.lo),abs(a.hi))
    def __repr__(a):return f'[{a.lo}/{S}, {a.hi}/{S}]'

def atan_recip(q,terms=100):
    # Alternating series; true arctan lies between consecutive partial sums.
    a=sum((Fraction((-1)**k,(2*k+1)*q**(2*k+1)) for k in range(terms)),Fraction())
    b=a+Fraction((-1)**terms,(2*terms+1)*q**(2*terms+1))
    return I(I.q(min(a,b)).lo,I.q(max(a,b)).hi)

def pi_interval():
    # Machin's identity pi=16 atan(1/5)-4 atan(1/239).
    return 16*atan_recip(5)-4*atan_recip(239)

def trig(x,sine):
    # Degree 100 Taylor polynomial; Lagrange remainder <= 7^101/101!.
    # Every requested |x| < 2*pi < 7; adding this rational bound is rigorous.
    assert x.absmax()<7*S
    term=x if sine else I.q(1)
    out=term
    xx=x*x
    for j in range(1,50 if sine else 51):
        den=(2*j)*(2*j+1) if sine else (2*j-1)*(2*j)
        term=-(term*xx)/den
        out=out+term
    err=I.q(Fraction(7**101,math.factorial(101))).hi
    return I(out.lo-err,out.hi+err)

def solve(a,b):
    a=[list(row)+[rhs] for row,rhs in zip(a,b)]
    for k in range(len(b)):
        piv=max(range(k,len(b)),key=lambda j: min(abs(a[j][k].lo),abs(a[j][k].hi)))
        a[k],a[piv]=a[piv],a[k]
        scale=a[k][k];assert scale.lo*scale.hi>0
        a[k]=[z/scale for z in a[k]]
        for j in range(len(b)):
            if j==k:continue
            fact=a[j][k];a[j]=[z-fact*w for z,w in zip(a[j],a[k])]
    return [row[-1] for row in a]

def exact_inputs():
    pi=pi_interval(); ns=[(trig(pi*Fraction(2*j,7),False),trig(pi*Fraction(2*j,7),True)) for j in range(7)]
    h=trig(pi/7,False);w=1+h
    types=[(0,3,4),(0,2,4)]
    xs=[]
    for ids in types:
        z=[ns[j] for j in ids]
        det=lambda a,b:a[0]*b[1]-a[1]*b[0]
        ds=[det(z[1],z[2]),det(z[2],z[0]),det(z[0],z[1])]
        den=sum(ds,I.q(0)); gammas=[v/den for v in ds]
        assert all(g.lo>0 for g in gammas)
        xs.append([(g*v[0]*w/h,g*v[1]*w/h) for g,v in zip(gammas,z)])
    return ns,xs

def discover(path):
    import numpy as np
    from scipy.optimize import linprog
    ns=np.array([[math.cos(2*math.pi*j/7),math.sin(2*math.pi*j/7)] for j in range(7)])
    atoms=[(sgn,i,j) for sgn in [-1,1] for i in range(7) for j in range(7)]
    vertices=np.array([sgn*np.outer(ns[i],ns[j]).ravel() for sgn,i,j in atoms]).T
    types=[(0,3,4),(0,2,4)];xs=[];h=math.cos(math.pi/7)
    for ids in types:
        z=ns[list(ids)];g=np.linalg.solve(np.vstack([z.T,np.ones(3)]),[0,0,1])
        xs.append(z*(g*(1+h)/h)[:,None])
    cases=[]
    for a,b in itertools.product(range(2),repeat=2):
        for ox in [(0,1,2),(0,2,1)]:
            for oy in itertools.permutations(range(3)):
                x,y=xs[a],xs[b]; t=(np.outer(x[ox[0]],y[oy[0]])-np.outer(x[ox[2]],y[oy[1]])).ravel()
                lp=linprog(np.ones(len(atoms)),A_eq=vertices,b_eq=t,bounds=(0,None),method='highs')
                assert lp.success
                support=list(np.flatnonzero(lp.x>1e-8))
                # If an LP solution uses fewer than 4 atoms, complete to a basis.
                for j in range(len(atoms)):
                    if len(support)==4:break
                    if np.linalg.matrix_rank(vertices[:,support+[j]],tol=1e-9)>len(support):support.append(j)
                assert len(support)==4
                cases.append(dict(types=[a,b],x_order=ox,y_order=oy,atoms=[atoms[j] for j in support]))
    path.write_text(json.dumps(dict(format=1,cases=cases),indent=2)+'\n')

def verify(path):
    data=json.loads(path.read_text());ns,xs=exact_inputs();cases=data['cases']
    expected=set((a,b,ox,oy) for a,b in itertools.product(range(2),repeat=2) for ox in [(0,1,2),(0,2,1)] for oy in itertools.permutations(range(3)))
    actual=set();maximum=0
    for c in cases:
        a,b=c['types'];ox=tuple(c['x_order']);oy=tuple(c['y_order']);actual.add((a,b,ox,oy))
        x,y=xs[a],xs[b]
        target=[x[ox[0]][i]*y[oy[0]][j]-x[ox[2]][i]*y[oy[1]][j] for i in range(2) for j in range(2)]
        cols=[]
        for sign,i,j in c['atoms']:
            assert sign in [-1,1] and 0<=i<7 and 0<=j<7
            cols.append([sign*ns[i][u]*ns[j][v] for u in range(2) for v in range(2)])
        coeff=solve([[col[j] for col in cols] for j in range(4)],target)
        bound=sum(v.absmax() for v in coeff)
        assert bound*25<24*S, 'failed exact 24/25 containment bound'
        maximum=max(maximum,bound)
    assert actual==expected and len(cases)==48
    pi=pi_interval(); h=trig(pi/7,False); w=1+h
    area=trig(pi*Fraction(2,7),True)*Fraction(7,2)
    cosine=trig(pi/14,False)
    ratio=(w*w*w*w)/(2*area*area*cosine*cosine)
    assert 0<ratio.lo<=ratio.hi<S
    print(json.dumps(dict(status='PASS',cases=48,dyadic_bits=BITS,bound='24/25',maximum_mass_upper_numerator=maximum,denominator=S,maximum_mass_upper_decimal=maximum/S,sharp_sys_interval=[str(Fraction(ratio.lo,S)),str(Fraction(ratio.hi,S))],sharp_sys_decimal=ratio.hi/S),indent=2))

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('mode',choices=['discover','verify']);p.add_argument('certificate',type=Path);a=p.parse_args()
    discover(a.certificate) if a.mode=='discover' else verify(a.certificate)
