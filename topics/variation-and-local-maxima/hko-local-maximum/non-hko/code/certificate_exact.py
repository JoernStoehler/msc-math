"""Exact certificate checks. No floating arithmetic is used for acceptance."""
import itertools,json,time,hashlib,sys
import sympy as S
from sympy.polys.matrices import DomainMatrix as DM
from family_exact import family,O,word_data,gradient,sym_columns,continuation_spec

def matrix_strings(M):return [[str(x) for x in M[i,:]] for i in range(M.rows)]

def null_direction(G,A0,E0):
    prod=[4*i+j for i in range(6,10) for j in range(2)]
    P=G[:,prod];pc=list(P.to_DM().rref()[1]);assert len(pc)==4
    # Prefer simple rows for the four-row off-product minor.
    order=sorted(range(G.rows),key=lambda i:sum(len(str(z)) for z in P[i,:]))
    rr=[]
    for i in order:
        trial=rr+[i]
        if P[trial,pc].to_DM().rank()>len(rr):rr=trial
        if len(rr)==4:break
    free=[j for j in range(len(prod)) if j not in pc]
    T=sym_columns(A0);TE=T.row_join(E0)
    assert T.to_DM().rank()==15 and TE.to_DM().rank()==17
    for j in free:
        w=S.zeros(len(prod),1);w[j]=1;sol=-P[rr,pc].inv(method='DM')*P[rr,j]
        for k,i in enumerate(pc):w[i]=sol[k]
        W=S.zeros(40,1)
        for k,i in enumerate(prod):W[i]=w[k]
        assert G*W==S.zeros(G.rows,1)
        if TE.row_join(W).to_DM().rank()==18:
            return W,{'flat_coordinates':prod,'rows':rr,'columns':pc,'free_columns':free,'free_unit':j}
    raise AssertionError('no flat direction transverse to the equality family')

def vertices_and_pulling(A0,KX,QX):
    X=S.Matrix([list(KX[i,:])+list(QX[j,:]) for i in range(6) for j in range(4)])
    assert X.rows==24
    incidence=[]
    for x in X.tolist():
        vals=A0*S.Matrix(x);assert max(vals)==1 and max(vals)<=1
        ids=[i for i,z in enumerate(vals) if z==1];assert len(ids)==4;incidence.append(ids)
    # Independently enumerate all four-hyperplane intersections.
    found=set()
    for ids in itertools.combinations(range(10),4):
        B=A0[list(ids),:]
        if B.det()==0:continue
        x=B.inv(method='DM')*S.ones(4,1)
        if max(A0*x)<=1:found.add(tuple(x))
    assert found==set(tuple(row) for row in X.tolist())
    facets=[[i for i,ac in enumerate(incidence) if f in ac] for f in range(10)]
    tets=[]
    for f,ids in enumerate(facets):
        assert S.Matrix([list(X[i,:]-X[ids[0],:]) for i in ids[1:]]).to_DM().rank()==3
        apex=min(ids)
        for other in range(10):
            if other==f:continue
            face=[i for i in ids if other in incidence[i]]
            if len(face)<3 or apex in face:continue
            # In a simple four-polytope an edge of this 2-face shares a third facet.
            edges={i:[] for i in face}
            for i,j in itertools.combinations(face,2):
                if len(set(incidence[i])&set(incidence[j]))==3:edges[i].append(j);edges[j].append(i)
            assert all(len(a)==2 for a in edges.values())
            start=min(face);order=[start,min(edges[start])]
            while len(order)<len(face):
                nxt=[j for j in edges[order[-1]] if j!=order[-2]][0]
                assert nxt not in order;order.append(nxt)
            assert start in edges[order[-1]]
            for k in range(1,len(order)-1):
                tet=[apex,order[0],order[k],order[k+1]];det=X[tet,:].det();assert det!=0
                tets.append((tet,1 if det>0 else -1))
    V=sum(sign*X[tet,:].det()/24 for tet,sign in tets)
    assert V==S.Rational(1,2)
    return X,incidence,facets,tets

def volume_jets_exact(A0,W,X,incidence,tets):
    dX=S.zeros(*X.shape);ddX=S.zeros(*X.shape)
    for i,ids in enumerate(incidence):
        B=A0[ids,:];H=W[ids,:];dx=-B.inv(method='DM')*H*X[i,:].T
        ddx=-2*B.inv(method='DM')*H*dx;dX[i,:]=dx.T;ddX[i,:]=ddx.T
    V=dV=ddV=S.Rational(0)
    for tet,sign in tets:
        M=X[tet,:];D=dX[tet,:];DD=ddX[tet,:];V+=sign*M.det()/24
        for i in range(4):
            N=M.copy();N[i,:]=D[i,:];dV+=sign*N.det()/24
            N=M.copy();N[i,:]=DD[i,:];ddV+=sign*N.det()/24
            for j in range(i+1,4):
                N=M.copy();N[i,:]=D[i,:];N[j,:]=D[j,:];ddV+=sign*N.det()/12
    return V,dV,ddV

def solve_consistent(M,rhs):
    pp=list(M.to_DM().rref()[1]);rr=list(M[:,pp].T.to_DM().rref()[1]);x=S.zeros(M.cols,1)
    sol=M[rr,pp].inv(method='DM')*rhs[rr,0]
    for k,i in enumerate(pp):x[i]=sol[k]
    assert M*x==rhs
    return x

def section_second(A,W,w,V,dV,ddV):
    sig=w['sigma'];b=S.Matrix(w['beta']);I=w['pivot'];m=len(sig)
    B,C,H,_,_=word_data(A,sig);hh=W[sig,:].T.col_join(S.zeros(1,m));mu5=C[:,I].T.inv(method='DM')*(H*b)[I,0]
    db=S.zeros(m,1);dbI=-C[:,I].inv(method='DM')*hh*b
    for k,i in enumerate(I):db[i]=dbI[k]
    Z=S.Matrix.hstack(*C.nullspace())
    Wp=W[sig,:]*O*B.T+B*O*W[sig,:].T;Hp=S.zeros(m)
    for i,j in itertools.combinations(range(m),2):Hp[i,j]=Hp[j,i]=Wp[i,j]
    R=Z.T*H*Z;L=Z.T*(H*db+Hp*b-hh.T*mu5)
    tt=solve_consistent(R,-L);db+=Z*tt
    assert C*db+hh*b==S.zeros(5,1)
    ddb=S.zeros(m,1);ddI=-2*C[:,I].inv(method='DM')*hh*db
    for k,i in enumerate(I):ddb[i]=ddI[k]
    edges=S.diag(*b)*B;de=S.diag(*db)*B+S.diag(*b)*W[sig,:]
    dde=S.diag(*ddb)*B+2*S.diag(*db)*W[sig,:]
    Q=dQ=ddQ=S.Rational(0)
    for i,j in itertools.combinations(range(m),2):
        Q+=(edges[i,:]*O*edges[j,:].T)[0]
        dQ+=(de[i,:]*O*edges[j,:].T+edges[i,:]*O*de[j,:].T)[0]
        ddQ+=(dde[i,:]*O*edges[j,:].T+2*de[i,:]*O*de[j,:].T+edges[i,:]*O*dde[j,:].T)[0]
    assert Q==S.Rational(1,2)
    assert -dV/V-2*dQ/Q==0
    Hsys=2*(dV/V)**2+4*dV/V*dQ/Q+6*(dQ/Q)**2-ddV/V-2*ddQ/Q
    return Hsys,db

def field_family_check(cert,Wspec,verbose=True):
    start=time.time();v,s=S.symbols('v s');F=S.QQ.frac_field(v,s)
    A,vg,_,_=family(v,s);A0,_,_,_=family(S.Rational(1,2),S.Rational(1,12))
    Al=[[F.from_sympy(z) for z in A[i,:]] for i in range(10)]
    Vgl=[[F.from_sympy(z) for z in vg[i,:]] for i in range(10)]
    Gs=[];continuations=[];cache={}
    for wi,w in enumerate(cert['sections']):
        sig=w['sigma'];m=len(sig);pp,rr,free,y0=continuation_spec(A0,w)
        key=tuple(sig)
        if key not in cache:
            _,_,_,M,rhs=word_data(A,sig)
            Mf=DM.from_Matrix(M).convert_to(F);rf=DM.from_Matrix(rhs).convert_to(F)
            inv=Mf.extract(rr,pp).inv();cache[key]=(Mf,rf,inv)
        Mf,rf,inv=cache[key]
        yy=[[F.from_sympy(z)] for z in y0]
        freey=DM.from_list([yy[i] for i in free],F)
        sol=inv*(rf.extract(rr,[0])-Mf.extract(rr,free)*freey)
        for k,i in enumerate(pp):yy[i]=sol.to_list()[k]
        yf=DM.from_list(yy,F)
        assert (Mf*yf-rf).is_zero_matrix
        for ii,value in enumerate(yy):
            assert F.to_sympy(value[0]).subs({v:S.Rational(1,2),s:S.Rational(1,12)})==y0[ii]
        beta=[yy[i][0] for i in range(m)];mu=[yy[i+m][0] for i in range(4)]
        # By the KKT system with the final multiplier fixed to one, Q=1/2.
        row=[-2*z for vv in Vgl for z in vv]
        for i,label in enumerate(sig):
            vec=[F.zero for k in range(4)]
            for j,lab in enumerate(sig):
                sign=1 if j>i else -1 if j<i else 0
                for k in range(4):vec[k]+=sign*beta[j]*Al[lab][k]
            ov=[vec[2],vec[3],-vec[0],-vec[1]]
            for k in range(4):row[4*label+k]-=4*beta[i]*(ov[k]-mu[k])
        Gs.append(row)
        continuations.append({'pivot_columns':pp,'pivot_rows':rr,'free_columns':free,
          'free_values':[str(y0[i]) for i in free]})
        if verbose:print('field section',wi+1,'/',len(cert['sections']),'seconds',round(time.time()-start,3),flush=True)
    GG=DM.from_list(Gs,F)
    prod=Wspec['flat_coordinates'];rr=Wspec['rows'];pc=Wspec['columns'];free=Wspec['free_columns'];unit=Wspec['free_unit']
    P=GG.extract(list(range(len(Gs))),prod);N=P.extract(rr,pc)
    print('field flat-kernel solve',flush=True)
    sol= -(N.inv()*P.extract(rr,[unit]));ww=[F.zero for i in range(len(prod))];ww[unit]=F.one
    for k,i in enumerate(pc):ww[i]=sol.to_list()[k][0]
    assert (P*DM.from_list([[z] for z in ww],F)).is_zero_matrix
    print('field flat-kernel identity PASS',round(time.time()-start,3),flush=True)
    # Base substitution verifies that the rational function W extends the selected W0.
    W40=[F.zero for i in range(40)]
    for k,i in enumerate(prod):W40[i]=ww[k]
    W0=S.Matrix([F.to_sympy(z).subs({v:S.Rational(1,2),s:S.Rational(1,12)}) for z in W40])
    return {'field':'QQ(v,s)','section_count':len(Gs),'stationarity_identities':True,'base_continuation_values_checked':True,'flat_kernel_identity':True,
       'flat_field_vector':[str(F.to_sympy(z)) for z in W40],'flat_base_vector':[str(z) for z in W0],
       'continuation_specs':continuations,'seconds':time.time()-start}


def verify_base(cert):
    start=time.time();A0,vg0,KX,QX=family(S.Rational(1,2),S.Rational(1,12))
    assert A0==S.Matrix(cert['dual_vertices']);assert vg0==S.Matrix(cert['volume_gradient'])
    boundedness_beta=S.Matrix([S.Rational(1,12)]*6+[S.Rational(3,16),S.Rational(1,16),S.Rational(1,16),S.Rational(3,16)])
    assert min(boundedness_beta)>0 and sum(boundedness_beta)==1
    assert A0.T*boundedness_beta==S.zeros(4,1) and A0.to_DM().rank()==4
    G=[];pivdets=[];bmin=None
    for w in cert['sections']:
        sig=w['sigma'];b=S.Matrix(w['beta']);I=w['pivot'];B,C,H,M,rhs=word_data(A0,sig)
        assert len(set(sig))==len(sig) and min(b)>0;det=C[:,I].det();assert det!=0;pivdets.append(det)
        mu5=C[:,I].T.inv(method='DM')*(H*b)[I,0]
        assert C*b==S.Matrix([0,0,0,0,1]);assert H*b==C.T*mu5;assert mu5[4]==1
        gg=gradient(A0,vg0,sig,b,mu5[:4,0]);G.append(gg);bmin=min(b) if bmin is None else min(bmin,min(b))
    G=S.Matrix.vstack(*G);assert G==S.Matrix(cert['gradient_rows']);assert G.to_DM().rank()==22
    lam=S.Matrix(cert['weights']);assert min(lam)>0 and sum(lam)==1 and G.T*lam==S.zeros(40,1)
    v,s=S.symbols('v s');A,_,_,_=family(v,s);subs={v:S.Rational(1,2),s:S.Rational(1,12)}
    E0=A.diff(v).subs(subs).reshape(40,1).row_join(A.diff(s).subs(subs).reshape(40,1))
    W,Wspec=null_direction(G,A0,E0);T=sym_columns(A0)
    yy=list(G.to_DM().rref()[1]);Y=S.eye(40)[:,yy]
    assert G*T==S.zeros(24,15) and G*E0==S.zeros(24,2)
    full=T.row_join(E0).row_join(W).row_join(Y);assert full.to_DM().rank()==40
    print('base rank / relation / full chart PASS',round(time.time()-start,3),flush=True)
    X,inc,facets,tets=vertices_and_pulling(A0,KX,QX)
    V,dV,ddV=volume_jets_exact(A0,W.reshape(10,4),X,inc,tets)
    assert dV==(vg0.reshape(1,40)*W)[0]
    print('exact geometry and volume jets PASS',round(time.time()-start,3),flush=True)
    hess=[];jets=[]
    for w in cert['sections']:
        h,db=section_second(A0,W.reshape(10,4),w,V,dV,ddV);hess.append(h);jets.append([str(z) for z in db])
    weighted=(S.Matrix(hess).T*lam)[0];assert weighted<0
    print('weighted flat Hessian',weighted,'PASS',round(time.time()-start,3),flush=True)
    return {'arithmetic':'exact rational','rank':22,'section_count':len(G.tolist()),'min_beta':str(bmin),
      'min_weight':str(min(lam)),'symmetry_rank':15,'equality_parameter_rank_mod_symmetries':2,
      'full_chart_rank':40,'flat_vector':[str(z) for z in W],'flat_spec':Wspec,'sharp_coordinate_indices':yy,
      'boundedness_positive_closure':[str(z) for z in boundedness_beta],
      'volume':'1/2','vertex_count':24,'genuine_facet_count':10,'simple':True,
      'pulling_tetrahedra':tets,'incidence':inc,'primal_vertices':matrix_strings(X),
      'volume_jets':[str(z) for z in [V,dV,ddV]],'section_second_derivatives':[str(z) for z in hess],
      'first_weight_jets':jets,'weighted_flat_hessian':str(weighted),'pivot_determinants':[str(z) for z in pivdets],
      'seconds':time.time()-start}

if __name__=='__main__':
    cert=json.load(open('work/certificate_base.json'));r=verify_base(cert)
    json.dump(r,open('evidence/certificate_base_exact.json','w'),indent=2)
    ff=field_family_check(cert,r['flat_spec']);assert ff['flat_base_vector']==r['flat_vector']
    json.dump(ff,open('evidence/certificate_family_exact.json','w'),indent=2)
    print('ALL EXACT CERTIFICATE CHECKS PASS',flush=True)
