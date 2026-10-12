"""Exact rational family and algebra shared by certificate verification."""
import sympy as S
import itertools
O=S.Matrix([[0,0,1,0],[0,0,0,1],[-1,0,0,0],[0,-1,0,0]])
BASE={}

def family(v,s):
    half=S.Rational(1,2);u=1-v+v*v;a=half-s;b=half+s;t=v-half
    D1=b/2-a*t;D2=b/2+a*t
    K=S.Matrix([[0,2],[(half-t)/D1,2*s/D1],[(half+t)/D2,-2*s/D2],
      [0,-2],[-(half-t)/D1,-2*s/D1],[-(half+t)/D2,2*s/D2]])
    Q=S.Matrix([[1/u,1/v],[1/(u-1),1/v],[1/(u-1),1/(v-1)],[1/u,1/(v-1)]])
    A=K.row_join(S.zeros(6,2)).col_join(S.zeros(4,2).row_join(Q))
    KX=S.Matrix([[-a,half],[a,half],[b,t],[a,-half],[-a,-half],[-b,-t]])
    QX=S.Matrix([[u,0],[0,v],[u-1,0],[0,v-1]])
    vg=S.zeros(10,4);mom=S.Matrix([[(2*u-1)/6,(2*v-1)/6]])
    for i in range(6):
        x=KX[i,:];y=KX[(i+1)%6,:];D=-S.Matrix.vstack(x,y).det();ce=(x+y)/2
        vg[i,:]=-D*(ce/2).row_join(mom)
    for i in range(4):
        x=QX[i,:];y=QX[(i+1)%4,:];D=S.Matrix.vstack(x,y).det();ce=(x+y)/2
        vg[i+6,2:4]=-D*ce
    return A,vg,KX,QX

def sym_columns(A):
    cols=[(-S.diag(*A[:,j])*A).reshape(40,1) for j in range(4)]
    cols.append((-A).reshape(40,1))
    for i in range(4):
        for j in range(i,4):
            T=S.zeros(4);T[i,j]=T[j,i]=1
            cols.append((-A*(O*T)).reshape(40,1))
    return S.Matrix.hstack(*cols)

def word_data(A,sigma):
    B=A[sigma,:];m=len(sigma);C=B.T.col_join(S.ones(1,m));W=B*O*B.T;H=S.zeros(m)
    for i,j in itertools.combinations(range(m),2):H[i,j]=H[j,i]=W[i,j]
    M=H.row_join(-B).col_join(C.row_join(S.zeros(5,4)))
    rhs=S.ones(m,1).col_join(S.Matrix([0,0,0,0,1]))
    return B,C,H,M,rhs

def gradient(A,vg,sigma,b,mu):
    B=A[sigma,:];m=len(sigma);G=S.zeros(10,4)
    for i,label in enumerate(sigma):
        vec=S.zeros(4,1)
        for j in range(m):
            if j>i:vec+=b[j]*B[j,:].T
            elif j<i:vec-=b[j]*B[j,:].T
        G[label,:]=(b[i]*(O*vec-mu)).T
    return (-4*G-2*vg).reshape(1,40)


def continuation_spec(A0,w):
    sig=w['sigma'];b=S.Matrix(w['beta']);I=w['pivot'];B,C,H,M,rhs=word_data(A0,sig)
    mu=(C[:,I].T.inv(method='DM')*(H*b)[I,0])[:4,0]
    y=b.col_join(mu)
    assert M*y==rhs
    pp=list(M.to_DM().rref()[1]);rr=list(M[:,pp].T.to_DM().rref()[1])
    free=[i for i in range(M.cols) if i not in pp]
    return pp,rr,free,y

if __name__=='__main__':
    import numpy as np,json,time
    from localmax_tools import positive_relation_probe
    cert=json.load(open('work/certificate_base.json'));A0,vg0,_,_=family(S.Rational(1,2),S.Rational(1,12))
    assert A0==S.Matrix(cert['dual_vertices']);assert vg0==S.Matrix(cert['volume_gradient'])
    specs=[continuation_spec(A0,w) for w in cert['sections']]
    for vv,ss in [(S.Rational(1,2),S.Rational(1,12)),(S.Rational(51,100),S.Rational(2,25)),(S.Rational(49,100),S.Rational(9,100))]:
        A,vg,_,_=family(vv,ss);Gs=[];qerr=0.;bmin=1.
        for w,(pp,rr,free,y0) in zip(cert['sections'],specs):
            B,C,H,M,rhs=word_data(A,w['sigma']);y=y0.copy();sol=M[rr,pp].inv(method='DM')*(rhs[rr,0]-M[rr,free]*y[free,0])
            for j,i in enumerate(pp):y[i]=sol[j]
            if M*y!=rhs:raise RuntimeError('exact family continuation inconsistent')
            b=y[:len(w['sigma']),0];mu=y[len(w['sigma']):,0]
            bmin=min(bmin,float(min(b)));assert (b.T*H*b)[0]==1
            Gs.append(gradient(A,vg,w['sigma'],b,mu))
        G=S.Matrix.vstack(*Gs);rank=G.to_DM().rank();p=positive_relation_probe(np.array(G,dtype=float),np.array(A,dtype=float))
        print('parameters',vv,ss,'rank',rank,'beta_min',bmin,'positive_margin',p.get('common_weight_margin'),flush=True)
