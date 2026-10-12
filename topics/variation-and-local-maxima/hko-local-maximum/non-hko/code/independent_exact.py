"""Second implementation: direct rational functions, not differentiated KKT jets.

This checker differentiates the explicit determinant volume and explicit
five-pivot feasible sections.  Its only acceptance arithmetic is SymPy QQ(z).
"""
import json,time,itertools
import sympy as S
from family_exact import family,O

def verify_independent(cert,base,verbose=True):
    start=time.time();z=S.symbols('z');A0,vg,K,Q=family(S.Rational(1,2),S.Rational(1,12))
    W=S.Matrix(base['flat_vector']).reshape(10,4);A=A0+z*W
    incidence=base['incidence'];tets=base['pulling_tetrahedra']
    X0=S.Matrix(base['primal_vertices']);X=S.zeros(24,4)
    for k,ids in enumerate(incidence):
        X[k,:]=(A[ids,:].inv()*S.ones(4,1)).T
    assert X.subs(z,0)==X0
    V=S.cancel(sum(sign*X[tet,:].det()/24 for tet,sign in tets))
    expectedV=S.Rational(1,2)+S.Rational(13,1728)*z*z
    assert S.cancel(V-expectedV)==0
    # Independent derivative of the volume from determinant cofactors.
    dVX=S.zeros(24,4)
    for tet,sign in tets:
        M=X0[tet,:];cof=M.cofactor_matrix()
        for i,idx in enumerate(tet):
            for j in range(4):dVX[idx,j]+=sign*cof[i,j]/24
    g=S.zeros(10,4)
    for k,ids in enumerate(incidence):
        Bi=A0[ids,:].inv()
        for pos,label in enumerate(ids):
            factor=-(dVX[k,:]*Bi[:,pos])[0]
            for j in range(4):g[label,j]+=factor*X0[k,j]
    assert g==vg
    rr=[];direct_seconds=[];q_functions=[]
    for j,w in enumerate(cert['sections']):
        sig=w['sigma'];I=w['pivot'];J=[k for k in range(len(sig)) if k not in I]
        b0=S.Matrix(w['beta']);db=S.Matrix(base['first_weight_jets'][j]);b=b0.copy()
        for k in J:b[k]=b0[k]+z*db[k]
        B=A[sig,:];C=B.T.col_join(S.ones(1,len(sig)));e=S.Matrix([0,0,0,0,1])
        bi=C[:,I].inv()*(e-C[:,J]*b[J,0])
        for k,idx in enumerate(I):b[idx]=S.cancel(bi[k])
        assert all(S.cancel(x)==0 for x in C*b-e)
        assert b.subs(z,0)==b0
        q=S.cancel(sum(b[k]*b[l]*(B[k,:]*O*B[l,:].T)[0]
                       for k,l in itertools.combinations(range(len(sig)),2)))
        u=S.cancel(1/(8*V*q*q))
        assert q.subs(z,0)==S.Rational(1,2)
        d1=S.diff(u,z).subs(z,0);d2=S.diff(u,z,2).subs(z,0)
        assert d1==0
        assert d2==S.Rational(base['section_second_derivatives'][j])
        assert d2==S.Rational(-13,432)
        direct_seconds.append(str(d2));q_functions.append(str(q))
        rr.append({'section':j,'Q_of_z':str(q),'U_of_z':str(u),
            'beta_of_z':[str(x) for x in b],'first_derivative':str(d1),'second_derivative':str(d2)})
        if verbose:print('direct section',j+1,'/',len(cert['sections']),'PASS',round(time.time()-start,3),flush=True)
    weighted=sum(S.Rational(l)*S.Rational(h) for l,h in zip(cert['weights'],direct_seconds))
    assert weighted==S.Rational(-13,432)
    # Shoelace identities certify area/centroid input to the volume-gradient formula on E.
    v,s=S.symbols('v s');Af,vgf,Kf,Qf=family(v,s)
    areas=[];moments=[]
    for P,orientation in [(Kf,-1),(Qf,1)]:
        area=0;mx=my=0
        for i in range(P.rows):
            x=P[i,:];y=P[(i+1)%P.rows,:];cross=S.Matrix.vstack(x,y).det()
            area+=orientation*cross/2
            mx+=orientation*(x[0]+y[0])*cross/6
            my+=orientation*(x[1]+y[1])*cross/6
        areas.append(S.cancel(area));moments.append([S.cancel(mx),S.cancel(my)])
    assert areas==[1,S.Rational(1,2)]
    assert moments[0]==[0,0]
    assert moments[1]==[(2*v*v-2*v+1)/6,(2*v-1)/6]
    for off,P,coords in [(0,Kf,[0,1]),(6,Qf,[2,3])]:
        for i in range(P.rows):
            for j in [i,(i+1)%P.rows]:
                assert S.cancel((Af[i+off,coords]*P[j,:].T)[0]-1)==0
    return {'arithmetic':'exact rational functions; direct determinant and inverse differentiation',
        'volume_of_z':str(V),'all_Q_of_z_constant_half':all(q=='1/2' for q in q_functions),
        'weighted_second_derivative':str(weighted),'independent_full_volume_gradient_pass':True,
        'family_areas':[str(x) for x in areas],
        'family_moments':[[str(x) for x in mm] for mm in moments],
        'family_edge_support_identities_pass':True,'sections':rr,'seconds':time.time()-start}

if __name__=='__main__':
    c=json.load(open('work/certificate_base.json'));b=json.load(open('evidence/certificate_base_exact.json'))
    r=verify_independent(c,b)
    json.dump(r,open('evidence/independent_exact.json','w'),indent=2)
    print('ALL INDEPENDENT EXACT CHECKS PASS',flush=True)
