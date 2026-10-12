#!/usr/bin/env python3
"""Exact degree<=4 Sturm queries over F at F/quadratic-F endpoints.

No file writes when imported. Running writes only sibling sturm-tests.json.
"""
from derive import *

def derivative(p):return trim([K(i)*v for i,v in enumerate(p) if i] or [K(0)])
def remainder(p,q):
    p=trim(p);q=trim(q);assert any(v!=0 for v in q)
    while len(p)>=len(q) and any(v!=0 for v in p):
        degree=len(p)-len(q);scalar=p[-1]/q[-1]
        p=sub(p,[K(0)]*degree+scale(q,scalar))
    return p
def gcd(p,q):
    while any(v!=0 for v in q):p,q=q,remainder(p,q)
    return scale(p,K(1)/p[-1]) if any(v!=0 for v in p) else [K(0)]
def divide(p,q):
    p=trim(p);out=[K(0)]*max(1,len(p)-len(q)+1)
    while len(p)>=len(q) and any(v!=0 for v in p):
        degree=len(p)-len(q);scalar=p[-1]/q[-1];out[degree]+=scalar
        p=sub(p,[K(0)]*degree+scale(q,scalar))
    assert not any(v!=0 for v in p)
    return trim(out)
def sequence(p):
    p=trim(p)
    if len(p)<=1:return [p]
    chain=[p,derivative(p)]
    while any(v!=0 for v in chain[-1]):
        r=neg(remainder(chain[-2],chain[-1]))
        if not any(v!=0 for v in r):break
        leading=r[-1];positive=leading if leading>0 else -leading
        chain.append(scale(r,K(1)/positive))
    return chain
def variation(chain,x):
    signs=[evaluate(p,x).sign() for p in chain];nonzero=[s for s in signs if s]
    return sum(a!=b for a,b in zip(nonzero,nonzero[1:])),signs
def count_open(p,left,right):
    """Distinct roots in (left,right), excluding both endpoints explicitly."""
    p=trim(p)
    if len(p)<=1:return dict(count=0,left_value=evaluate(p,left).sign(),right_value=evaluate(p,right).sign(),chain=[encode(p)])
    squarefree=divide(p,gcd(p,derivative(p)));chain=sequence(squarefree)
    vl,sl=variation(chain,left);vr,sr=variation(chain,right)
    count=vl-vr-int(evaluate(squarefree,right)==0)
    assert count>=0
    return dict(count=count,left_value=evaluate(p,left).sign(),right_value=evaluate(p,right).sign(),
        left_variation=vl,right_variation=vr,left_chain_signs=sl,right_chain_signs=sr,
        chain=[encode(v) for v in chain])

def odd_part(p):
    """Square-free factor product with odd multiplicities, without factorization."""
    p=trim(p)
    if len(p)<=1:return [K(1)]
    p=scale(p,K(1)/p[-1]);c=gcd(p,derivative(p));w=divide(p,c);result=[K(1)];i=1
    while len(w)>1:
        y=gcd(w,c);factor=divide(w,y)
        if i%2:result=mul(result,factor)
        w=y;c=divide(c,y);i+=1
    return result

def sign_certificate(p,left,right,sample,target=1):
    """Prove target*p>=0 on closed interval from odd roots/probe/endpoints.

Returning failed is a localized unproved predicate, not a negative-body claim.
"""
    p=trim(p)
    if not any(v!=0 for v in p):return dict(passed=True,identically_zero=True)
    odd=odd_part(p);query=count_open(odd,left,right)
    probe=evaluate(p,sample).sign()*target
    # A probe at a repeated root needs a nearby exact nonzero F probe.
    if probe==0:
        trial=sample
        lo=left.right if isinstance(left,engine.EventRoot) else left
        for _ in range(8):
            trial=(lo+trial)/2;probe=evaluate(p,trial).sign()*target
            if probe:sample=trial;break
    endpoints=[evaluate(p,left).sign()*target,evaluate(p,right).sign()*target]
    passed=query['count']==0 and probe>0 and all(s>=0 for s in endpoints)
    return dict(passed=passed,odd_part=encode(odd),odd_root_query=query,
        probe=sample.data(),probe_sign=probe,endpoint_signs=endpoints,target=target)

def tests():
    zero=K(0);one=K(1)
    x=[zero,one]
    assert count_open(x,K(-1),K(1))['count']==1
    assert count_open(x,zero,one)['count']==0
    assert count_open(x,K(-1),zero)['count']==0
    double=mul(x,x)
    assert count_open(double,K(-1),one)['count']==1
    assert odd_part(double)==[one]
    assert sign_certificate(double,K(-1),one,K('1/2'))['passed']
    cubic=mul(sub(x,[K('1/4')]),mul(sub(x,[K('1/2')]),sub(x,[K('3/4')])))
    assert count_open(cubic,zero,one)['count']==3
    quartic=mul(mul(sub(x,[K('1/4')]),sub(x,[K('1/4')])),mul(sub(x,[K('3/4')]),sub(x,[K('3/4')])))
    assert count_open(quartic,zero,one)['count']==2
    assert sign_certificate(quartic,zero,one,K('1/2'))['passed']
    mixed=mul(cubic,double)
    assert count_open(mixed,K(-1),one)['count']==4
    alpha=engine.roots_in_strip([K('-1/10'),zero,one],K(0,'1/5'))[0]
    assert count_open(sub(x,[K('1/4')]),zero,alpha)['count']==1
    assert count_open([K('-1/10'),zero,one],zero,alpha)['count']==0
    assert count_open([K('-1/10'),zero,one],alpha,one)['count']==0
    shared=gcd(mul(cubic,x),mul(quartic,x));assert len(shared)==4
    return dict(status='passed',tests=['simple interior root','both excluded endpoints',
        'repeated-root distinct count','squarefree odd multiplicity','three cubic roots',
        'two double quartic roots','quadratic-extension endpoint exclusion',
        'gcd/shared roots'],quadratic_endpoint=alpha.data())

if __name__=='__main__':
    result=tests();result['source_sha256']=hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
    (ROOT/'sturm-tests.json').write_text(json.dumps(result,indent=2)+'\n');print(result['status'])
