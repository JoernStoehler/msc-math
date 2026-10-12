#!/usr/bin/env python3
"""Independent checks of the returned non-HKO certificate.

Does not import any of the submitted verifier's modules. Uses the rational
witness as data, and the recorded weight jets as additional proposed witnesses.
Recomputes geometry by a triangle-product staircase triangulation (not the
submitted facet pulling triangulation), all 40 derivatives of every upper
section by differentiated constraint solves, and the W-line volume by Fubini.

Usage: python independent_audit.py --source source --rerun rerun --output independent.json
"""
from __future__ import annotations
import argparse, hashlib, itertools, json, platform, time
from pathlib import Path
import sympy as sp

R = sp.Rational
O = sp.Matrix([[0,0,1,0],[0,0,0,1],[-1,0,0,0],[0,-1,0,0]])

def polygon_data(v, s):
    u = 1-v+v*v
    a,b,t = R(1,2)-s,R(1,2)+s,v-R(1,2)
    H = sp.Matrix([[-a,R(1,2)],[a,R(1,2)],[b,t],
                   [a,-R(1,2)],[-a,-R(1,2)],[-b,-t]])
    T = sp.Matrix([[u,0],[0,v],[u-1,0],[0,v-1]])
    # Obtain all normals by independent edge intersection solves.
    normals=[]
    for polygon, off in [(H,0),(T,2)]:
        for i in range(polygon.rows):
            n=polygon[[i,(i+1)%polygon.rows],:].inv(method='DM')*sp.ones(2,1)
            row=[sp.Integer(0)]*4
            row[off:off+2]=list(n)
            normals.append(row)
    return sp.Matrix(normals),H,T

def geometry(A,H,T):
    X=sp.Matrix([list(H[i,:])+list(T[j,:]) for i in range(6) for j in range(4)])
    incidence=[]
    for x in X.tolist():
        values=A*sp.Matrix(x)
        assert max(values)<=1
        ids=[j for j,value in enumerate(values) if value==1]
        assert len(ids)==4 and A[ids,:].det()!=0
        incidence.append(ids)
    found=set()
    for ids in itertools.combinations(range(10),4):
        N=A[list(ids),:]
        if N.det()==0: continue
        x=N.inv(method='DM')*sp.ones(4,1)
        if max(A*x)<=1: found.add(tuple(x))
    assert found==set(map(tuple,X.tolist()))
    # H and T are independently triangulated by fans from vertex 0.
    # Each product of two triangles has the canonical 6 staircase 4-simplices.
    simplices=[]
    for i in range(1,5):
        hi=[0,i,i+1]
        for j in range(1,3):
            tj=[0,j,j+1]
            for horizontal in itertools.combinations(range(4),2):
                a=b=0; indices=[4*hi[a]+tj[b]]
                for k in range(4):
                    if k in horizontal:a+=1
                    else:b+=1
                    indices.append(4*hi[a]+tj[b])
                simplices.append(indices)
    V=sp.Integer(0); vertex_gradient=sp.zeros(24,4)
    for ids in simplices:
        D=sp.Matrix.vstack(*[X[i,:]-X[ids[0],:] for i in ids[1:]])
        det=D.det(); assert det!=0
        sign=sp.sign(det); V+=sign*det/24
        cof=D.cofactor_matrix()
        for k,i in enumerate(ids[1:]):
            term=sign*cof[k,:]/24
            vertex_gradient[i,:]+=term
            vertex_gradient[ids[0],:]-=term
    vg=sp.zeros(10,4)
    for j,ids in enumerate(incidence):
        inv=A[ids,:].inv(method='DM')
        for pos,label in enumerate(ids):
            vg[label,:]-=(vertex_gradient[j,:]*inv[:,pos])[0]*X[j,:]
    return X,incidence,V,vg,len(simplices)

def direct_gradient(A,vg,word,beta,pivots):
    m=len(word); B=A[word,:]
    C=B.T.col_join(sp.ones(1,m))
    assert C*beta==sp.Matrix([0,0,0,0,1]) and min(beta)>0
    H=sp.zeros(m); q=sp.Integer(0); geom=sp.zeros(1,40)
    for i,j in itertools.combinations(range(m),2):
        w=(B[i,:]*O*B[j,:].T)[0]
        H[i,j]=H[j,i]=w; q+=beta[i]*beta[j]*w
        for k in range(4):
            geom[4*word[i]+k]+=beta[i]*beta[j]*(O*B[j,:].T)[k]
            geom[4*word[j]+k]+=beta[i]*beta[j]*(B[i,:]*O)[k]
    assert q==R(1,2)
    # All forty partial derivatives from the literal feasible section,
    # holding its free weights fixed (no KKT gradient shortcut).
    rhs=sp.zeros(5,40)
    for i,label in enumerate(word):
        for k in range(4):rhs[k,4*label+k]=-beta[i]
    db=sp.zeros(m,40); dbp=C[:,pivots].inv(method='DM')*rhs
    for k,i in enumerate(pivots):db[i,:]=dbp[k,:]
    qgrad=geom+beta.T*H*db
    return -4*qgrad-2*vg.reshape(1,40)

def capacity_product(A):
    factors=[]
    for offset,n,coord in [(0,6,0),(6,4,2)]:
        N=A[offset:offset+n,coord:coord+2].T.col_join(sp.ones(1,n))
        vertices=[]
        for m in [1,2,3]:
            for subset in itertools.combinations(range(n),m):
                C=N[:,subset]
                if C.to_DM().rank()!=m:continue
                try:
                    b,params=C.gauss_jordan_solve(sp.Matrix([0,0,1]))
                except ValueError:continue
                assert params.rows==0
                if min(b)>0:vertices.append(([offset+k for k in subset],b/2))
        factors.append(vertices)
    count=0; vals=[]
    for left,right in itertools.product(*factors):
        labels=left[0]+right[0]; weights=dict(zip(labels,list(left[1])+list(right[1])))
        first=min(labels)
        for rest in itertools.permutations([i for i in labels if i!=first]):
            word=(first,)+rest
            q=sum(weights[i]*weights[j]*(A[i,:]*O*A[j,:].T)[0]
                  for i,j in itertools.combinations(word,2))
            vals.append(q); count+=1
    mx=max(vals)
    return {'cases':count,'maximum_Q':str(mx),'capacity':str(1/(2*mx)),
            'tight_cases':vals.count(mx),'strict_Q_gap':str(min(mx-v for v in vals if v<mx))}

def fubini_W(A,H,W):
    z,x,y=sp.symbols('z x y'); q=sp.Matrix([x,y])
    pnormals=A[6:,2:]
    h=sp.ones(4,1)-z*W[6:,:2]*q
    vertices=[]
    # Vertex i is the intersection of edge i-1 and edge i.
    for i in range(4):
        ids=[(i-1)%4,i]
        vertices.append(pnormals[ids,:].inv(method='DM')*h[ids,0])
    area=sp.expand(sum(sp.det(sp.Matrix.hstack(vertices[i],vertices[(i+1)%4]))
                       for i in range(4))/2)
    # Independently integrate the polynomial by affine triangle pullbacks.
    u,v=sp.symbols('u v'); value=sp.Integer(0)
    for i in range(1,5):
        a,b,c=[H[j,:].T for j in [0,i,i+1]]
        M=sp.Matrix.hstack(b-a,c-a); p=a+M*sp.Matrix([u,v])
        integrand=sp.Poly(sp.expand(area.subs({x:p[0],y:p[1]},simultaneous=True)),u,v)
        integral=sum(coef*sp.factorial(powers[0])*sp.factorial(powers[1])/
                      sp.factorial(powers[0]+powers[1]+2)
                      for powers,coef in integrand.terms())
        value+=abs(M.det())*integral
    value=sp.factor(value)
    assert sp.expand(value-R(1,2)-R(13,1728)*z*z)==0
    return z,area,value

def main():
    ap=argparse.ArgumentParser();ap.add_argument('--source',type=Path,required=True)
    ap.add_argument('--rerun',type=Path,required=True);ap.add_argument('--output',type=Path,required=True)
    args=ap.parse_args();start=time.perf_counter()
    cert=json.loads((args.source/'certificate/certificate_base.json').read_text())
    base=json.loads((args.rerun/'certificate_base_exact.json').read_text())
    A,H,T=polygon_data(R(1,2),R(1,12));assert A==sp.Matrix(cert['dual_vertices'])
    X,inc,V,vg,ns=geometry(A,H,T);assert V==R(1,2)
    assert vg==sp.Matrix(cert['volume_gradient'])
    print('Independent 48-simplex volume and all 40 volume derivatives PASS',flush=True)
    G=sp.Matrix.vstack(*[direct_gradient(A,vg,w['sigma'],sp.Matrix(w['beta']),w['pivot'])
                        for w in cert['sections']])
    assert G==sp.Matrix(cert['gradient_rows']);assert G.to_DM().rank()==22
    lam=sp.Matrix(cert['weights']);assert min(lam)>0 and sum(lam)==1 and G.T*lam==sp.zeros(40,1)
    print('All 960 upper-section gradient entries and positive relation PASS',flush=True)
    # Explicit sp(4) block generators, independent of the submitted O*R implementation.
    columns=[(-sp.diag(*A[:,k])*A).reshape(40,1) for k in range(4)]+[(-A).reshape(40,1)]
    for i,j in itertools.product(range(2),repeat=2):
        B=sp.zeros(2);B[i,j]=1;Z=sp.zeros(2)
        S=B.row_join(Z).col_join(Z.row_join(-B.T));assert S.T*O+O*S==sp.zeros(4)
        columns.append((-A*S).reshape(40,1))
    for side in [0,1]:
        for i,j in [(0,0),(0,1),(1,1)]:
            B=sp.zeros(2);B[i,j]=B[j,i]=1;Z=sp.zeros(2)
            S=(Z.row_join(B).col_join(Z.row_join(Z)) if side==0
               else Z.row_join(Z).col_join(B.row_join(Z)))
            assert S.T*O+O*S==sp.zeros(4);columns.append((-A*S).reshape(40,1))
    symmetry=sp.Matrix.hstack(*columns); assert symmetry.to_DM().rank()==15 and G*symmetry==sp.zeros(24,15)
    v,s=sp.symbols('v s');Af,_,_=polygon_data(v,s);subs={v:R(1,2),s:R(1,12)}
    E=Af.diff(v).subs(subs).reshape(40,1).row_join(Af.diff(s).subs(subs).reshape(40,1))
    assert G*E==sp.zeros(24,2) and symmetry.row_join(E).to_DM().rank()==17
    W=sp.zeros(10,4);W[6,0]=R(1,2);W[6,1]=R(1,3);W[7,0]=R(3,2);W[7,1]=-1;W[8,0]=1
    assert G*W.reshape(40,1)==sp.zeros(24,1)
    Y=sp.eye(40)[:,list(G.to_DM().rref()[1])]
    assert symmetry.row_join(E).row_join(W.reshape(40,1)).row_join(Y).to_DM().rank()==40
    print('Independent symmetry basis and full chart rank PASS',flush=True)
    cap=capacity_product(A);assert cap['maximum_Q']=='1/2'
    print('Independent complete product enumeration PASS',flush=True)
    z,slice_area,Vz=fubini_W(A,H,W)
    Bz=A+z*W;qs=[]
    for j,w in enumerate(cert['sections']):
        word=w['sigma'];I=w['pivot'];J=[i for i in range(len(word)) if i not in I]
        beta=sp.Matrix(w['beta']);d=sp.Matrix(base['first_weight_jets'][j]);b=beta.copy()
        for k in J:b[k]+=z*d[k]
        C=Bz[word,:].T.col_join(sp.ones(1,len(word)))
        bp=C[:,I].inv(method='DM')*(sp.Matrix([0,0,0,0,1])-C[:,J]*b[J,0])
        for k,i in enumerate(I):b[i]=sp.cancel(bp[k])
        assert all(sp.cancel(t)==0 for t in C*b-sp.Matrix([0,0,0,0,1]))
        assert b.subs(z,0)==beta
        q=sp.cancel(sum(b[k]*b[l]*(Bz[word[k],:]*O*Bz[word[l],:].T)[0]
                         for k,l in itertools.combinations(range(len(word)),2)))
        assert q==R(1,2);qs.append(str(q))
    U=sp.factor(1/(8*Vz*R(1,2)**2));hess=sp.diff(U,z,2).subs(z,0)
    assert hess==R(-13,432)
    print('Fubini volume and all exact W-line sections PASS',flush=True)
    result={'status':'PASS','independent_of_submitted_modules':True,
       'certificate_sha256':hashlib.sha256((args.source/'certificate/certificate_base.json').read_bytes()).hexdigest(),
       'geometry':{'vertices':X.rows,'facets':10,'simple':True,'volume':str(V),'independent_triangulation_simplices':ns},
       'gradient_entries_recomputed':G.rows*G.cols,'gradient_rank':G.to_DM().rank(),'min_lambda':str(min(lam)),
       'symmetry_rank':15,'equality_rank_mod_symmetry':2,'full_chart_rank':40,
       'product_capacity':cap,'p_slice_area':str(slice_area),'volume_W_line':str(Vz),
       'all_24_Q_W_line':qs,'common_U_W_line':str(U),'common_U_second_derivative':str(hess),
       'qualifier':'This checks algebra. Neighborhood implication is reviewed separately; no claim to compute the actual full capacity away from the product family.',
       'python':platform.python_version(),'sympy':sp.__version__,'elapsed_seconds':time.perf_counter()-start}
    args.output.write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps(result,indent=2),flush=True)

if __name__=='__main__':main()
