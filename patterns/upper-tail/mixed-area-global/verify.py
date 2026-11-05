#!/usr/bin/env python3
"""Exact Q(sqrt5) controls plus independent floating support/mixed-area checks.

No dataset, fit, optimizer or original producer is used. Capacity control uses
complete 5x5 closure supports and the imported two-shadow theorem. The proof of
profile equality for every angle is in REPORT.md, not inferred from the grid.
"""
import argparse, hashlib, itertools, json, math
from fractions import Fraction as F
from functools import cmp_to_key, total_ordering
from pathlib import Path

@total_ordering
class K:
    """Ordered Q(sqrt5), following the retained conditional-coupling field."""
    def __init__(self,a=0,b=0):
        if isinstance(a,K): self.a,self.b=a.a,a.b
        else: self.a,self.b=F(a),F(b)
    def __add__(self,o):
        o=K(o);return K(self.a+o.a,self.b+o.b)
    __radd__=__add__
    def __neg__(self):return K(-self.a,-self.b)
    def __sub__(self,o):return self+-K(o)
    def __rsub__(self,o):return K(o)+-self
    def __mul__(self,o):
        o=K(o);return K(self.a*o.a+5*self.b*o.b,self.a*o.b+self.b*o.a)
    __rmul__=__mul__
    def __truediv__(self,o):
        o=K(o);d=o.a*o.a-5*o.b*o.b
        return self*K(o.a/d,-o.b/d)
    def __rtruediv__(self,o):return K(o)/self
    def __bool__(self):return bool(self.a or self.b)
    def sign(self):
        a,b=self.a,self.b
        if not a:return (b>0)-(b<0)
        if not b:return (a>0)-(a<0)
        if (a>0)==(b>0):return 1 if a>0 else -1
        d=a*a-5*b*b
        return ((d>0)-(d<0))*(1 if a>0 else -1)
    def __eq__(self,o):
        try:o=K(o)
        except (TypeError,ValueError):return False
        return self.a==o.a and self.b==o.b
    def __lt__(self,o):return (self-K(o)).sign()<0
    def __abs__(self):return self if self>=0 else -self
    def __float__(self):return float(self.a)+math.sqrt(5)*float(self.b)
    def serial(self):return {'rational':str(self.a),'sqrt5':str(self.b)}

def dot(a,b):return sum(x*y for x,y in zip(a,b))
def cross(a,b):return a[0]*b[1]-a[1]*b[0]
def J(a):return [-a[1],a[0]]
def area(v):return sum(cross(a,b) for a,b in zip(v,v[1:]+v[:1]))/2

def polygon(normals,offsets,tol=0):
    out=[]
    for i,u in enumerate(normals):
        j=(i+1)%len(normals);v=normals[j];c=cross(u,v)
        p=[(offsets[i]*v[1]-u[1]*offsets[j])/c,
           (u[0]*offsets[j]-offsets[i]*v[0])/c]
        assert all(dot(w,p)<=h+tol for w,h in zip(normals,offsets))
        out.append(p)
    assert area(out)>0
    return out

def closures(normals):
    out=[]
    for ids in itertools.combinations(range(len(normals)),3):
        x,y,z=[normals[i] for i in ids];v=[cross(y,z),cross(z,x),cross(x,y)];s=sum(v)
        if not s:continue
        w=[q/s for q in v]
        if min(w)<=0:continue
        a=[K(0)]*len(normals)
        for i,q in zip(ids,w):a[i]=q
        assert sum(a)==1 and all(sum(a[i]*normals[i][k] for i in range(len(a)))==0 for k in range(2))
        out.append(a)
    return out

def word(a,b,sgn):
    v=a+[[sgn*q for q in J(u)] for u in b]
    def half(u):return 0 if u[1]>0 or (u[1]==0 and u[0]>=0) else 1
    def cmp(i,j):
        d=half(v[i])-half(v[j])
        if d:return d
        c=cross(v[i],v[j])
        if c:return -c.sign()
        return (i>j)-(i<j)
    w=sorted(range(len(v)),key=cmp_to_key(cmp))
    return w if sgn>0 else w[::-1]

def capacity(a,b):
    ca,cb=closures(a),closures(b);assert len(ca)==len(cb)==5
    best=K(0);winner=None
    for sgn in [1,-1]:
        w=word(a,b,sgn);pos={j:i for i,j in enumerate(w)}
        for ai,x in enumerate(ca):
            for bi,y in enumerate(cb):
                q=sum(x[i]*y[j]*dot(u,v)*(1 if pos[i]<pos[len(a)+j] else -1)
                      for i,u in enumerate(a) for j,v in enumerate(b))/4
                if q>best:best=q;winner={'shadow':sgn,'A_support':ai,'B_support':bi,'Q':q}
    return 1/(2*best),winner

def fpolygon(sign,e,phase=0):
    n=[(math.cos(2*math.pi*i/5),math.sin(2*math.pi*i/5)) for i in range(5)]
    h=[1+sign*e*math.cos(4*math.pi*i/5+phase) for i in range(5)]
    return polygon(n,h,1e-13)
def rotate(v,t):
    c,s=math.cos(t),math.sin(t)
    return [[c*x-s*y,s*x+c*y] for x,y in v]
def mixed(p,q):
    # Outer normal times edge length is (dy,-dx); support is homogeneous.
    return sum(max(dot((b[1]-a[1],a[0]-b[0]),u) for u in q)
               for a,b in zip(p,p[1:]+p[:1]))/2

def serial(x):
    if isinstance(x,K):return x.serial()
    if isinstance(x,F):return str(x)
    if isinstance(x,dict):return {k:serial(v) for k,v in x.items()}
    if isinstance(x,(list,tuple)):return [serial(v) for v in x]
    return x

def main():
    ap=argparse.ArgumentParser();ap.add_argument('--out',type=Path,required=True);args=ap.parse_args()
    s=K(0,1);b=(1+s)/4;r=(s-1)/4;gold=(1+s)/2;rho=(3+s)/5
    n=[[K(1),K(0)],[r,K(1)],[-b,2*r],[-b,-2*r],[r,K(-1)]]
    d=[K(1),-b,r,r,-b];one=[K(1)]*5
    p0=polygon(n,one);a0=area(p0);assert a0==5*(5-s)/4
    cb=closures(n);loads=[dot(c,d) for c in cb]
    assert min(loads)==-K(F(1,2)) and max(loads)==1/gold
    # Exact chart: q=T*x, p=T^-T*y, T=diag(1,sin72).
    # Original Q=sin72*J*P0 maps to J*(T*P0), so c0=5.
    bn=[J(v) for v in n];q0=[J(v) for v in p0]
    assert area(q0)==a0 and 25/(2*a0*a0)==rho
    controls=[]
    for den in [25,32,100]:
        e=F(1,den);rows=[]
        for sign in [1,-1]:
            h=[1+sign*e*x for x in d];p=polygon(n,h);ar=area(p)
            assert ar==a0*(1-b*e*e)
            core=min(dot(c,h) for c in cb)
            expected=1-e/2 if sign>0 else 1-e/gold
            assert core==expected
            dual=[[x/hh for x in v] for v,hh in zip(n,h)]
            cap,witness=capacity(dual,bn);assert cap==5*core
            sys=cap*cap/(2*ar*a0)
            assert sys==rho*core*core/(1-b*e*e)
            rows.append({'sign':sign,'offsets':h,'vertices_in_symplectic_chart':p,
                         'area_in_chart':ar,'core':core,'capacity':cap,'ratio':sys,
                         'ratio_decimal':float(sys),'above_one':sys>1,'winner':witness})
        assert rows[0]['area_in_chart']==rows[1]['area_in_chart']
        assert rows[0]['ratio']>rows[1]['ratio']
        if den==25:assert rows[0]['above_one'] and not rows[1]['above_one']
        else:assert all(t['above_one'] for t in rows)
        controls.append({'epsilon':e,'rows':rows,'ratio_gap':rows[0]['ratio']-rows[1]['ratio']})
    # These checks use ordinary Euclidean physical coordinates independently.
    max_cross=max_self=max_sum=max_area=max_phase=0.0;numeric=[]
    for e in [1/25,1/32,1/100]:
        p,m=fpolygon(1,e),fpolygon(-1,e);base=fpolygon(1,0)
        q=rotate(base,math.pi/2);q=[[math.sin(2*math.pi/5)*x,math.sin(2*math.pi/5)*y] for x,y in q]
        max_area=max(max_area,abs(area(p)-area(m)))
        for j in range(721):
            t=2*math.pi*j/720
            dc=abs(mixed(p,rotate(q,t))-mixed(m,rotate(q,t)))
            ds=abs(mixed(p,rotate(p,t))-mixed(m,rotate(m,t)))
            max_cross=max(max_cross,dc);max_self=max(max_self,ds)
        # Pointwise support orbit sum, independent random-looking directions.
        for j in range(113):
            t=j*math.sqrt(2);u=(math.cos(t),math.sin(t))
            for shape in [p,m]:
                a=sum(max(dot(u,v) for v in rotate(shape,2*math.pi*k/5)) for k in range(5))
                z=5*max(dot(u,v) for v in base);max_sum=max(max_sum,abs(a-z))
        numeric.append({'epsilon':e,'physical_P_plus':p,'physical_P_minus':m,'physical_Q':q,
                        'common_area':area(p),'common_Q_area':area(q)})
    for j in range(113):
        phase=j*math.sqrt(2)
        direct=min(sum(float(c[i])*math.cos(4*math.pi*i/5+phase) for i in range(5)) for c in cb)
        delta=math.acos(max(-1,min(1,-math.cos(5*phase))))/5
        closed=-math.cos(delta)/float(gold)
        max_phase=max(max_phase,abs(direct-closed))
    assert max(max_cross,max_self,max_sum,max_area,max_phase)<1e-12
    out={'status':'PASS','check_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
         'scope':'Six exact nonregular products; 300 complete support-pair/shadow capacity evaluations; Q(sqrt5) areas, closures, core values and threshold tests. Physical floating profile checks corroborate conventions only; REPORT proof covers every angle.',
         'exact_baseline_area_in_chart':a0,'exact_baseline_ratio':rho,'closure_loads':loads,
         'exact_controls':controls,'floating_checks':{'angles_per_control':721,'pointwise_directions_per_control':113,
             'max_rotated_cross_mixed_area_discrepancy':max_cross,'max_rotated_self_mixed_area_discrepancy':max_self,
             'max_orbit_support_identity_error':max_sum,'max_area_discrepancy':max_area,'max_general_phase_core_formula_error':max_phase},
         'physical_geometry_controls':numeric}
    args.out.write_text(json.dumps(serial(out),indent=2)+'\n')
    print(json.dumps({'status':'PASS','controls':[{ 'epsilon':str(c['epsilon']),'ratios':[float(r['ratio']) for r in c['rows']]} for c in controls], 'floating_checks':out['floating_checks']},indent=2))
if __name__=='__main__':main()
