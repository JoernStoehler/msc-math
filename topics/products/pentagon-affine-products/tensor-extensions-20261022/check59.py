#!/usr/bin/env python3
"""Rigorous fixed-point interval checker for 5x7 full-tensor containment.

Mathematical input: Machin's identity pi/4 = 4 atan(1/5) - atan(1/239).
All later interval endpoints and operations use exact Python integers/Fractions.
"""
import hashlib
import itertools
import json
import math
from fractions import Fraction as F
from pathlib import Path

if not __debug__:
    raise RuntimeError('Run without -O; assertions verify the certificate.')

HERE=Path(__file__).resolve().parent
CERT=HERE/'weights59.json'
S=10**30


def ceildiv(a,b):
    return -((-a)//b)

class I:
    __slots__=('lo','hi')
    def __init__(self,lo,hi):
        assert isinstance(lo,int) and isinstance(hi,int) and lo<=hi
        self.lo=lo; self.hi=hi
    @staticmethod
    def c(v):
        q=F(v)*S
        return I(q.numerator//q.denominator,ceildiv(q.numerator,q.denominator))
    def __add__(a,b):
        b=b if isinstance(b,I) else I.c(b)
        return I(a.lo+b.lo,a.hi+b.hi)
    __radd__=__add__
    def __neg__(a): return I(-a.hi,-a.lo)
    def __sub__(a,b): return a+(-b if isinstance(b,I) else -I.c(b))
    def __rsub__(a,b): return I.c(b)-a
    def __mul__(a,b):
        b=b if isinstance(b,I) else I.c(b)
        ps=[x*y for x in (a.lo,a.hi) for y in (b.lo,b.hi)]
        return I(min(ps)//S,ceildiv(max(ps),S))
    __rmul__=__mul__
    def __truediv__(a,b):
        b=b if isinstance(b,I) else I.c(b)
        assert b.lo>0 or b.hi<0, 'denominator interval contains zero'
        qs=[F(S*x,y) for x in (a.lo,a.hi) for y in (b.lo,b.hi)]
        l=min(qs); h=max(qs)
        return I(l.numerator//l.denominator,ceildiv(h.numerator,h.denominator))
    def positive(self): return self.lo>0
    def negative(self): return self.hi<0
    def __repr__(self): return f'I({self.lo},{self.hi})'


def atan_bounds(z,N=40):
    z=F(z)
    partial=sum(((-1)**k*z**(2*k+1)/F(2*k+1) for k in range(N)),F(0))
    nextterm=(-1)**N*z**(2*N+1)/F(2*N+1)
    return min(partial,partial+nextterm),max(partial,partial+nextterm)


def pi_bounds():
    a0,a1=atan_bounds(F(1,5)); b0,b1=atan_bounds(F(1,239))
    return 16*a0-4*b1,16*a1-4*b0

PI0,PI1=pi_bounds()
assert F(3)<PI0<PI1<F(4)

def trig(num,den,kind):
    scale=F(num,den)
    left=scale*PI0; right=scale*PI1
    mid=(left+right)/2
    assert abs(mid)<82
    err=(right-left)/2
    if kind=='cos':
        poly=sum(((-1)**k*mid**(2*k)/math.factorial(2*k) for k in range(41)),F(0))
    else:
        poly=sum(((-1)**k*mid**(2*k+1)/math.factorial(2*k+1) for k in range(41)),F(0))
    # Taylor's theorem (all sine/cosine derivatives have magnitude <=1),
    # and both sine/cosine are 1-Lipschitz on the real line.
    rem=abs(mid)**81/math.factorial(81)+err
    low=poly-rem; high=poly+rem
    q0=low*S; q1=high*S
    return I(q0.numerator//q0.denominator,ceildiv(q1.numerator,q1.denominator))


def cross(a,b): return a[0]*b[1]-a[1]*b[0]
def outer(a,b): return [a[0]*b[0],a[0]*b[1],a[1]*b[0],a[1]*b[1]]

def factor(n):
    h=trig(1,n,'cos')
    normal=[[trig(2*i,n,'cos'),trig(2*i,n,'sin')] for i in range(n)]
    dual=[[v/h for v in p] for p in normal]
    triples=[]; rejected=[]
    for ids in itertools.combinations(range(n),3):
        a,b,c=[dual[i] for i in ids]
        raw=[cross(b,c),cross(c,a),cross(a,b)]
        total=sum(raw,I.c(0))
        weights=[v/total for v in raw]
        if all(w.positive() for w in weights):
            steps=[[weights[k]*v for v in dual[ids[k]]] for k in range(3)]
            assert all((sum((steps[k][coord] for k in range(3)),I.c(0)).lo<=0<=sum((steps[k][coord] for k in range(3)),I.c(0)).hi) for coord in range(2))
            triples.append({'indices':list(ids),'weights':weights,'steps':steps})
        elif any(w.negative() for w in weights):
            rejected.append(list(ids))
        else:
            raise AssertionError(('unclassified triple',n,ids,weights))
    assert len(triples)+len(rejected)==math.comb(n,3)
    return {'n':n,'h':h,'normal':normal,'triples':triples,'rejected':rejected}


def inverse(matrix):
    dim=len(matrix)
    a=[[matrix[i][j] for j in range(dim)]+[I.c(int(i==j)) for j in range(dim)] for i in range(dim)]
    for i in range(dim):
        if not (a[i][i].positive() or a[i][i].negative()):
            j=next(j for j in range(i+1,dim) if a[j][i].positive() or a[j][i].negative())
            a[i],a[j]=a[j],a[i]
        pivot=a[i][i]
        a[i]=[v/pivot for v in a[i]]
        for j in range(dim):
            if i!=j:
                factor=a[j][i]
                a[j]=[a[j][k]-factor*a[i][k] for k in range(2*dim)]
    return [row[dim:] for row in a]


def main():
    cert=json.loads(CERT.read_text())
    assert cert['pair']==[5,9] and cert['denominator']==10**12
    A,B=factor(5),factor(9)
    assert len(A['triples'])==5 and len(B['triples'])==30
    G=[]
    for p,q in itertools.product(A['normal'],B['normal']):
        tensor=[v/(I.c(2)*(I.c(1)+A['h'])*(I.c(1)+B['h'])) for v in outer(p,q)]
        G.append(tensor); G.append([-v for v in tensor])
    assert len(G)==90
    basis=cert['basis_indices']; assert len(basis)==5 and len(set(basis))==5
    M=[[G[j][k] for j in basis] for k in range(4)]+[[I.c(1) for j in basis]]
    inv=inverse(M)
    min_positive=None; min_rational_numerator=None; max_delta=0; count=0
    rows=iter(cert['rows'])
    for ai,at in enumerate(A['triples']):
        for bi,bt in enumerate(B['triples']):
            for tail in itertools.permutations((1,2)):
                ap=(0,)+tail
                for bp in itertools.permutations((0,1,2)):
                    row=next(rows)
                    assert row['a_triple']==ai and row['b_triple']==bi and row['a_order']==list(ap) and row['b_order']==list(bp)
                    nums=row['weights_numerators']; assert len(nums)==90 and all(isinstance(z,int) and z>=10**7 for z in nums)
                    this_min=min(nums)
                    min_rational_numerator=this_min if min_rational_numerator is None else min(min_rational_numerator,this_min)
                    x=at['steps']; y=bt['steps']
                    lhs=outer(x[ap[0]],y[bp[0]])
                    rhs=outer(x[ap[2]],y[bp[1]])
                    T=[(lhs[k]-rhs[k])/I.c(2) for k in range(4)]
                    residual=[T[k]-sum((I.c(F(nums[j],cert['denominator']))*G[j][k] for j in range(len(G))),I.c(0)) for k in range(4)]
                    residual.append(I.c(F(cert['denominator']-sum(nums),cert['denominator'])))
                    delta=[sum((inv[j][k]*residual[k] for k in range(5)),I.c(0)) for j in range(5)]
                    for j,d in zip(basis,delta):
                        corrected=I.c(F(nums[j],cert['denominator']))+d
                        assert corrected.positive(),('nonpositive corrected weight',count,j,corrected)
                        min_positive=corrected.lo if min_positive is None else min(min_positive,corrected.lo)
                        max_delta=max(max_delta,abs(d.lo),abs(d.hi))
                    count+=1
    assert next(rows,None) is None and count==1800
    assert S%cert['denominator']==0
    global_lower=min(min_positive,min_rational_numerator*(S//cert['denominator']))
    result={'status':'PASS','meaning':'Exact 5x9 T6 membership in T4 via rational weights plus exact five-coordinate correction; intervals certify all corrected weights positive','pi_interval_width_less_than':str(PI1-PI0),'scale':S,'triples_5':len(A['triples']),'triples_9':len(B['triples']),'rejected_5':len(A['rejected']),'rejected_9':len(B['rejected']),'orders':count,'positive_triple_indices_5':[x['indices'] for x in A['triples']],'positive_triple_indices_9':[x['indices'] for x in B['triples']],'min_corrected_weight_lower_bound':str(F(min_positive,S)),'min_rational_weight_exact':str(F(min_rational_numerator,cert['denominator'])),'global_min_weight_lower_bound':str(F(global_lower,S)),'max_abs_correction_upper_bound':str(F(max_delta,S)),'certificate_sha256':hashlib.sha256(CERT.read_bytes()).hexdigest(),'checker_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
    target=HERE/'check-result59.json'
    if target.exists(): raise FileExistsError(target)
    target.write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps(result,indent=2))

if __name__=='__main__': main()
