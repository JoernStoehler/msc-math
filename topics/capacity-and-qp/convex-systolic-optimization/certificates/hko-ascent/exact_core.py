#!/usr/bin/env python3
"""Independent finite-certificate audit of the HKO ridge/facet-sector return.

Standard library only. Does not import submitted modules. Input JSON is
untrusted witness data. Gradients are differentiated constraint solves;
the facet integral is computed by Fubini and clipped-pentagon moments,
not the submitted multiplier-gradient and tetrahedron formulas.
The analytic neighborhood argument is reviewed separately.
"""
from __future__ import annotations
from fractions import Fraction as F
from functools import lru_cache
from itertools import combinations
from pathlib import Path
import argparse, json, time, platform, sys, hashlib


def require(ok, message):
    if not ok: raise ValueError(message)

class K:
    """Q[t]/(t^4-10t^2+5), ascending coefficients; t in (0,1)."""
    __slots__=('a',)
    def __init__(self, a=0):
        if isinstance(a,K): self.a=a.a
        elif isinstance(a,(list,tuple)):
            require(len(a)<=4,'field coefficient length')
            self.a=tuple(map(F,a))+(F(0),)*(4-len(a))
        else: self.a=(F(a),F(0),F(0),F(0))
    def __hash__(self): return hash(self.a)
    def __eq__(self,b): return self.a==K(b).a
    def __bool__(self): return any(self.a)
    def __neg__(self): return K(tuple(-x for x in self.a))
    def __add__(self,b):
        b=K(b); return K(tuple(x+y for x,y in zip(self.a,b.a)))
    __radd__=__add__
    def __sub__(self,b): return self+-K(b)
    def __rsub__(self,b): return K(b)+-self
    def __mul__(self,b):
        b=K(b)
        if not self or not b: return ZERO
        if not any(b.a[1:]): return K(tuple(x*b.a[0] for x in self.a))
        if not any(self.a[1:]): return K(tuple(x*self.a[0] for x in b.a))
        c=[F(0)]*7
        for i,x in enumerate(self.a):
            if x:
                for j,y in enumerate(b.a):
                    if y:c[i+j]+=x*y
        for k in range(6,3,-1):c[k-2]+=10*c[k];c[k-4]-=5*c[k]
        return K(c[:4])
    __rmul__=__mul__
    def __truediv__(self,b): return self*inverse(K(b))
    def __rtruediv__(self,b): return K(b)*inverse(self)
    def __pow__(self,n):
        require(isinstance(n,int) and n>=0,'power')
        out=ONE;x=self
        while n:
            if n&1:out=out*x
            x=x*x;n//=2
        return out
    def coefficients(self): return list(map(str,self.a))
    def __repr__(self): return str(self.coefficients())

ZERO=K(0);ONE=K(1);TAU=K([0,1])

@lru_cache(None)
def inverse(x):
    require(bool(x),'division by zero')
    if not any(x.a[1:]):return K(1/x.a[0])
    cols=[(x*K([0]*j+[1])).a for j in range(4)]
    m=[[cols[j][i] for j in range(4)]+[F(i==0)] for i in range(4)]
    for c in range(4):
        p=next(i for i in range(c,4) if m[i][c]);m[c],m[p]=m[p],m[c]
        d=m[c][c];m[c]=[v/d for v in m[c]]
        for i in range(4):
            if i!=c and m[i][c]:
                d=m[i][c];m[i]=[u-d*v for u,v in zip(m[i],m[c])]
    out=K([r[-1] for r in m]);require(x*out==ONE,'inverse check');return out

SQRT5=(5-TAU*TAU)/2

# Independent rational isolation by bisection, using strict monotonicity of
# t^4-10t^2+5 on (0,1). The certificate needs no float sign decisions.
lo,hi=F(0),F(1)
for _ in range(210):
    mid=(lo+hi)/2;val=mid**4-10*mid**2+5
    if val>0:lo=mid
    else:hi=mid

@lru_cache(None)
def sign(x):
    if not x:return 0
    l=h=F(0)
    for c in reversed(x.a):
        pp=(l*lo,l*hi,h*lo,h*hi);l,h=min(pp)+c,max(pp)+c
    if l>0:return 1
    if h<0:return -1
    raise ValueError('sign not isolated: '+repr(x))

def nominal(x):
    t=float((lo+hi)/2)
    return sum(float(c)*t**i for i,c in enumerate(x.a))

def dot(a,b):return sum((x*y for x,y in zip(a,b)),ZERO)
def add(a,b):return [x+y for x,y in zip(a,b)]
def scale(t,a):return [t*x for x in a]
def omega(a,b):return a[0]*b[2]+a[1]*b[3]-a[2]*b[0]-a[3]*b[1]
def det2(a,b):return a[0]*b[1]-a[1]*b[0]
def area(p):return sum((det2(p[i],p[(i+1)%len(p)]) for i in range(len(p))),ZERO)/2

@lru_cache(None)
def matrix_inverse(rows):
    n=len(rows); require(all(len(row)==n for row in rows),'matrix dimensions')
    m=[list(row)+[K(i==j) for j in range(n)] for i,row in enumerate(rows)]
    for c in range(n):
        possible=[i for i in range(c,n) if m[i][c]]
        if not possible: raise ArithmeticError('singular matrix')
        p=possible[0];m[c],m[p]=m[p],m[c]
        div=m[c][c];m[c]=[x/div for x in m[c]]
        for i in range(n):
            if i!=c and m[i][c]:
                a=m[i][c];m[i]=[x-a*y for x,y in zip(m[i],m[c])]
    return tuple(tuple(row[n:]) for row in m)

def inv(rows):return matrix_inverse(tuple(tuple(r) for r in rows))
def mv(m,v):return [dot(r,v) for r in m]
def decode(desc):return K(list(reversed([F(x) for x in desc])))

# Reconstruct vertices by rotation, and normals by solving edge equations.
C=(SQRT5-1)/4;SN=TAU*(3+SQRT5)/4
U=[[ONE,ZERO]]
for _ in range(4):u=U[-1];U.append([C*u[0]-SN*u[1],SN*u[0]+C*u[1]])
W=[[u[1],-u[0]] for u in U]
A=[]
for factor in [U,W]:
    for i in range(5):
        n=mv(inv([factor[i],factor[(i+1)%5]]),[ONE,ONE])
        A.append(n+[ZERO,ZERO] if factor is U else [ZERO,ZERO]+n)
V=[u+w for u in U for w in W]
V0=area(U)*area(W);Q0=TAU/5
require(V0==25*(5+SQRT5)/32,'V0')
require(A[0]==[ONE,TAU,ZERO,ZERO],'normal labels')
volume_factor=area(U)*U[1][1]/dot(A[0],A[0])
require(volume_factor==K(F(25,32))+5*SQRT5/16,'facet-derived volume differential')
VOL_GRAD=[-volume_factor*x for a in A for x in a]
BASIS=[[K(i==j) for i in range(4)] for j in range(4)]
VALUES={'small':(5-SQRT5)/20,'large':SQRT5/10,
        'middle':(5+SQRT5)/40,'halfsmall':(5-SQRT5)/40}

def qeval(normals,sig,be):
    prefix=[ZERO]*4;q=ZERO
    for label,w in zip(sig,be):
        v=scale(w,normals[label]);q+=omega(prefix,v);prefix=add(prefix,v)
    return q

class Section:
    def __init__(self,normals,sig,be,pivot):
        self.normals=normals;self.sig=sig;self.be=be;self.pivot=pivot
        require(len(sig)==len(set(sig))==len(be),'word labels')
        require(all(0<=i<len(normals) for i in sig),'word label bounds')
        require(len(pivot)==len(set(pivot))==5,'pivot count')
        require(all(0<=i<len(sig) for i in pivot),'pivot bounds')
        require(all(sign(x)>0 for x in be),'positive weights')
        self.c=[[normals[i][r] for i in sig] for r in range(4)]+[[ONE]*len(sig)]
        require(mv(self.c,be)==[ZERO]*4+[ONE],'closure normalization')
        self.ci=inv([[row[j] for j in pivot] for row in self.c])
        require(qeval(normals,sig,be)==Q0,'touching objective')
        # Derivatives with respect to weights from the literal ordered sum.
        self.gb=[]
        for i in range(len(sig)):
            s=ZERO
            for j in range(len(sig)):
                if j<i:s+=be[j]*omega(normals[sig[j]],normals[sig[i]])
                elif j>i:s+=be[j]*omega(normals[sig[i]],normals[sig[j]])
            self.gb.append(s)
        # Check stationarity on a basis of ker C, without multiplier formula.
        for j in range(len(sig)):
            if j in pivot:continue
            change=[ZERO]*len(sig);change[j]=ONE
            pi=mv(self.ci,[-r[j] for r in self.c])
            for i,v in zip(pivot,pi):change[i]=v
            require(mv(self.c,change)==[ZERO]*5,'stationarity tangent')
            require(dot(self.gb,change)==ZERO,'stationarity')
    def gradient(self):
        # Direct differentiation of the fixed-free-weight constraint solve.
        ans=[[ZERO]*4 for _ in self.normals]
        for pos,label in enumerate(self.sig):
            for coord in range(4):
                rhs=[ZERO]*5;rhs[coord]=-self.be[pos]
                dpi=mv(self.ci,rhs)
                corr=sum((self.gb[i]*v for i,v in zip(self.pivot,dpi)),ZERO)
                explicit=ZERO
                for j in range(len(self.sig)):
                    if j<pos:explicit+=self.be[pos]*self.be[j]*omega(self.normals[self.sig[j]],BASIS[coord])
                    elif j>pos:explicit+=self.be[pos]*self.be[j]*omega(BASIS[coord],self.normals[self.sig[j]])
                ans[label][coord]+=explicit+corr
        return ans


def insertion(seed,br,b):
    sig,be=seed.sig,seed.be;m=len(sig);I=br['pivot'];pos=br['pos']
    require(len(I)==len(set(I))==5 and all(0<=i<m for i in I),'insertion pivot')
    require(0<=pos<=m,'insertion slot')
    ci=inv([[row[j] for j in I] for row in seed.c]);h=[ZERO]*m
    for i,v in zip(I,mv(ci,[-x for x in b]+[-ONE])):h[i]=v
    require(mv(seed.c,h)==[-x for x in b]+[-ONE],'insertion constraint')
    # Expand ordered symplectic area of the weighted velocities by prefixes.
    p0=[ZERO]*4;p1=[ZERO]*4;l=r=q=ZERO
    v0=[scale(w,A[label]) for label,w in zip(sig,be)]
    v1=[scale(w,A[label]) for label,w in zip(sig,h)]
    v0.insert(pos,[ZERO]*4);v1.insert(pos,b)
    for x,y in zip(v0,v1):
        q+=omega(p0,x);l+=omega(p0,y)+omega(p1,x);r+=omega(p1,y)
        p0=add(p0,x);p1=add(p1,y)
    require(q==Q0 and p0==[ZERO]*4 and p1==[ZERO]*4,'insertion polynomial')
    return l,r


def clipped_square_integral(poly,linear):
    """Exact integral of (linear)_+^2 over a convex CCW polygon."""
    output=[]
    for i,a in enumerate(poly):
        b=poly[(i+1)%len(poly)];fa,fb=linear(a),linear(b)
        inside_a=sign(fa)>=0;inside_b=sign(fb)>=0
        if inside_a:output.append(a)
        if inside_a!=inside_b:
            t=fa/(fa-fb);x=[a[k]+t*(b[k]-a[k]) for k in range(2)]
            require(linear(x)==ZERO,'clipped vertex');output.append(x)
    clean=[]
    for x in output:
        if not clean or x!=clean[-1]:clean.append(x)
    if len(clean)>1 and clean[0]==clean[-1]:clean.pop()
    if len(clean)<3:return ZERO
    total=ZERO
    for i in range(1,len(clean)-1):
        tri=[clean[0],clean[i],clean[i+1]]
        aa=det2([tri[1][k]-tri[0][k] for k in range(2)],
                [tri[2][k]-tri[0][k] for k in range(2)])/2
        require(sign(aa)>=0,'orientation clipped triangle')
        f=[linear(v) for v in tri]
        require(all(sign(v)>=0 for v in f),'clipped values')
        mean=(sum((v*v for v in f),ZERO)+sum((f[i]*f[j] for i,j in combinations(range(3),2)),ZERO))/6
        total+=aa*mean
    return total


def facet_integral(d):
    c0=dot(d[:2],U[0]);c1=dot(d[:2],U[1]);slope=c1-c0
    require(bool(slope),'nonzero interval slope for independent Fubini rule')
    i0=clipped_square_integral(W,lambda p: c0+dot(d[2:],p))
    i1=clipped_square_integral(W,lambda p: c1+dot(d[2:],p))
    return K(F(2,5))*(i1-i0)/(2*slope*area(W))


def run(data):
    begin=time.perf_counter();out={}
    # Full vertex intersection check of the displayed ten inequalities.
    found=set()
    for ids in combinations(range(10),4):
        try:p=tuple(mv(inv([A[i] for i in ids]),[ONE]*4))
        except ArithmeticError:continue
        if all(sign(ONE-dot(a,p))>=0 for a in A):found.add(p)
    require(found==set(map(tuple,V)),'all primal vertices')
    faces=[]
    for v in V:
        labels={i for i,a in enumerate(A) if dot(a,v)==ONE}
        require(len(labels)==4,'simple vertex');faces.append(labels)
    edges={tuple(p) for f in faces for p in combinations(sorted(f),2)}
    def orbit(e):
        oo=set()
        for k in range(5):
            ee=[5*(i//5)+(i+k)%5 for i in e]
            oo.add(tuple(sorted(ee)));oo.add(tuple(sorted((i+5)%10 for i in ee)))
        return oo
    orbs=[orbit(e) for e in [(0,6),(0,7),(0,1),(0,5)]]
    require(list(map(len,orbs))==[10,10,10,5],'orbit cardinalities')
    require(set.union(*orbs)==edges and len(edges)==35,'edge orbit coverage')
    def rot(v):return [C*v[0]-SN*v[1],SN*v[0]+C*v[1],C*v[2]-SN*v[3],SN*v[2]+C*v[3]]
    def swap(v):return [-v[3],v[2],v[1],-v[0]]
    for op in [rot,swap]:
        require(all(omega(op(x),op(y))==omega(x,y) for x in BASIS for y in BASIS),'symplectic symmetry')
        require(set(map(tuple,map(op,A)))==set(map(tuple,A)),'normal symmetry')
    require(all(not omega(A[i],A[j]) for i,j in orbs[2]|orbs[3]),'zero pairings')
    require(all(omega(A[i],A[j]) for i,j in orbs[0]|orbs[1]),'nonzero pairings')
    edge_sq=dot([U[1][k]-U[0][k] for k in range(2)],[U[1][k]-U[0][k] for k in range(2)])
    require(edge_sq/(2*dot(A[0],A[0])*V0)==K(F(2,25)),'quadratic cap scale')
    out['geometry']={'vertices':25,'facets':10,'polar_edges':35,'volume_coefficients':V0.coefficients()}
    print('PASS independent geometry and all 35 edge orbits',flush=True)
    raw=json.loads((data/'seeds.json').read_text());require(len(raw)==65,'65 seeds')
    seeds=[Section(A,r['sigma'],[VALUES[w] for w in r['weights']],r['pivot']) for r in raw]
    gradq=[s.gradient() for s in seeds]
    gs=[[-2*x/Q0-VOL_GRAD[k]/V0 for k,x in enumerate(sum(g,[]))] for g in gradq]
    out['differentiated_seed_gradient_entries']=65*40
    print('PASS 65 seed sections and 2600 direct derivative entries',flush=True)
    M=[[ZERO]*4 for _ in range(4)]
    for tri in [(0,1,3),(0,2,3)]:
        for i in tri:
            for j in tri:M[i][j]+=K(F(2 if i==j else 1,24))
    require(sum(sum(M,[]),ZERO)==ONE,'quadrature mass')
    certs=json.loads((data/'ridge_certificates.json').read_text());outputs=[]
    require({(tuple(c['edge']),c['half']) for c in certs}=={(e,h) for e in [(0,6),(0,7)] for h in [0,1]},'all ridge records')
    contact_count=0
    def checkcontact(br,v):
        nonlocal contact_count
        for b in [[ZERO]*4]+BASIS:
            l,r=insertion(seeds[br['seed']],br,b)
            require(l==2*Q0*(dot(V[v],b)-1),'affine contact identity')
        contact_count+=1
    for cert in certs:
        i,j=cert['edge'];vs=cert['vertices'];half=cert['half']
        require(vs==[k for k,v in enumerate(V) if dot(A[i],v)==dot(A[j],v)==ONE],'vertex-resolved incidence')
        u=[K(F(x)) for x in cert['u']];require(all(sign(x)>0 for x in u),'quadrature u')
        d=[dot(M[k],u)/u[k] for k in range(4)]
        sos=[[ZERO]*4 for _ in range(4)]
        for k,l in combinations(range(4),2):
            vv=[(1/u[k] if r==k else -1/u[l] if r==l else ZERO) for r in range(4)]
            for r in range(4):
                for c in range(4):sos[r][c]+=M[k][l]*u[k]*u[l]*vv[r]*vv[c]
        require(sos==[[(d[r] if r==c else ZERO)-M[r][c] for c in range(4)] for r in range(4)],'quadrature sum of squares')
        total=ZERO;gradient=[ZERO]*40;masses=[ZERO]*4
        for term in cert['terms']:
            w=decode(term['weight']);require(sign(w)>0,'positive ridge coefficient');total+=w
            gradient=add(gradient,scale(w,gs[term['seed']]))
            br=term['branch']
            if br is None:require(term['vertex']==-1,'pure seed');continue
            require(br['seed']==term['seed'] and term['vertex'] in vs,'branch label')
            checkcontact(br,term['vertex'])
            rr=[]
            for s in [ZERO,K(F(1,2)),ONE]:
                b=add(scale(1-s,A[i]),scale(s,A[j]));rr.append(insertion(seeds[br['seed']],br,b)[1])
            require(not rr[2*half] and sign(rr[2*(1-half)])<=0 and sign(rr[1])<0,'half-edge signs')
            # Independent fourth-point check on the interpolated polynomial.
            s=K(F(1,3));b=add(scale(1-s,A[i]),scale(s,A[j]))
            pol=2*(s-K(F(1,2)))*(s-1)*rr[0]+4*s*(1-s)*rr[1]+2*s*(s-K(F(1,2)))*rr[2]
            require(insertion(seeds[br['seed']],br,b)[1]==pol,'R polynomial degree')
            rate=-Q0/(2*rr[1]);masses[vs.index(term['vertex'])]+=w*rate
        theta=decode(cert['theta']);threshold=decode(cert['threshold'])
        require(theta==(K(F(24,175)) if (i,j)==(0,6) else (1308-36*SQRT5)/14795),'stated theta')
        require(total==ONE and gradient==[ZERO]*40,'all 40 derivative cancellation')
        require(sign(theta-threshold)>0 and sign(threshold-K(F(2,25)))>0,'strict gap')
        require(all(sign(masses[k]-theta*d[k])>=0 for k in range(4)),'vertex capacities dominate quadrature')
        outputs.append({'edge':[i,j],'half':half,'terms':len(cert['terms']),'theta_coefficients':theta.coefficients(),'theta_nominal':nominal(theta),'volume_coefficient':'2/25'})
        print('PASS vertex-resolved ridge',i,j,half,flush=True)
    zero=json.loads((data/'zero_ridges.json').read_text())
    for edge in [[0,1],[0,5]]:
        i,j=edge;rows=[r for r in zero if r['edge']==edge]
        expected={k for k,v in enumerate(V) if dot(A[i],v)==dot(A[j],v)==ONE}
        require({r['vertex'] for r in rows}==expected and len(rows)==len(expected),'all zero-face contacts')
        for row in rows:
            checkcontact(row['branch'],row['vertex'])
            for s in [ZERO,K(F(1,2)),ONE,K(F(1,3))]:
                b=add(scale(1-s,A[i]),scale(s,A[j]));require(insertion(seeds[row['branch']['seed']],row['branch'],b)[1]==ZERO,'identically flat cost')
    out['ridges']=outputs;out['zero_pairing_contacts']=len(zero);out['affine_contact_checks']=contact_count
    print('PASS nine flat contacts and independent half-edge expansions',flush=True)
    fac=json.loads((data/'facet_cone_certificate.json').read_text());normal11=A+[A[0]]
    require(fac['direction']==[8,6,13,-11] and len(fac['terms'])==26,'facet packet shape')
    direction=list(map(K,fac['direction']));total=ZERO;G=[ZERO]*40;response=[ZERO]*4
    for term in fac['terms']:
        s=Section(normal11,term['sigma'],list(map(decode,term['beta'])),term['pivot'])
        gq=s.gradient();weight=decode(term['lambda']);require(sign(weight)>0,'facet lambda');total+=weight
        cc=scale(2/Q0,gq[10]);agg=[v[:] for v in gq[:10]];agg[0]=add(agg[0],gq[10])
        gg=[-2*v/Q0-VOL_GRAD[k]/V0 for k,v in enumerate(sum(agg,[]))]
        G=add(G,scale(weight,gg));response=add(response,scale(weight,cc))
    require(total==ONE and G==[ZERO]*40,'facet aggregate gradient cancellation')
    require(response==list(map(decode,fac['response'])),'facet mean covector')
    require(all(sign(1-x)>0 and sign(1+x)>0 for x in response),'covector norm')
    vals=[dot(direction,U[i]+p) for i in [0,1] for p in W]
    require(sum(sign(v)>0 for v in vals)==6 and sum(sign(v)<0 for v in vals)==4,'genuine split')
    require(all(sign(v-K(F(1,100)))>0 or sign(-v-K(F(1,100)))>0 for v in vals),'vertex buffers')
    v=facet_integral(direction);margin=dot(response,direction)-v
    require(sign(margin-K(F(1,10)))>0,'independent facet integral margin')
    # Compare to the reported tetrahedral expression, using a new integration.
    reference=K(F(204897,3338530))*SQRT5+K(F(4737679,8346325))+K(F(8306239,10015590))*SQRT5*TAU+K(F(1215471,642025))*TAU
    require(v==reference,'Fubini equals submitted tetrahedral integral')
    require(fac['radius_l1']=='1/100','claimed radius')
    sector=K(F(1,10))-K(F(7,5))*K(F(1,100))
    require(sign(sector-K(F(1,20)))>0,'uniform direction margin')
    out['facet']={'sections':26,'direct_derivative_entries':26*44,'response_coefficients':[v.coefficients() for v in response],'volume_slope_coefficients':v.coefficients(),'volume_slope_nominal':nominal(v),'margin_coefficients':margin.coefficients(),'margin_nominal':nominal(margin),'radius_l1':'1/100','safe_sector_margin':'43/500 > 1/20','integration_method':'Fubini in the q-edge coordinate, then exact clipped-pentagon quadratic moments'}
    print('PASS 26 facet sections, aggregate coupling, independent Fubini integral and uniform margin',flush=True)
    out.update(all_checks_passed=True,python=sys.version,platform=platform.platform(),seconds=time.perf_counter()-begin,
               scope='Independent finite certificate audit; mathematical neighborhood proof must also be read. No new F<=11 theorem and no completeness claim for the facet bank.',
               dependencies='Python standard library only; reads submitted witness JSON but imports none of its code.')
    return out

def main():
    ap=argparse.ArgumentParser(description=__doc__);ap.add_argument('--data',type=Path,required=True);ap.add_argument('--out',type=Path,required=True)
    a=ap.parse_args();require(not a.out.exists(),'output already exists')
    out=run(a.data)
    out['input_sha256']={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(a.data.glob('*.json'))}
    a.out.parent.mkdir(parents=True,exist_ok=True);a.out.write_text(json.dumps(out,indent=2)+'\n')
    print('ALL CHECKS PASSED in %.3f s'%out['seconds'],flush=True)
if __name__=='__main__':main()
