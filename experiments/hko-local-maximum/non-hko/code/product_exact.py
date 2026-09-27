"""Complete exact product capacity via bilinear reduction to closure vertices."""
import itertools,json,time
import sympy as S
from sympy.polys.matrices import DomainMatrix as DM
from family_exact import family,O

def closure_vertices(C0):
    out=[];status=[]
    for m in [2,3]:
        for ids in itertools.combinations(range(C0.cols),m):
            C=C0[:,ids];rr=list(C.T.to_DM().rref()[1]);r=len(rr)
            if r!=m:raise RuntimeError('unexpected closure rank')
            e=S.Matrix([0,0,1]);b=C[rr,:].inv(method='DM')*e[rr,0]
            feasible=C*b==e and min(b)>0
            status.append((ids,rr,b,feasible,C*b==e))
            if feasible:out.append((ids,b))
    return out,status

def verify_product(symbolic=True):
    start=time.time();v,s=S.symbols('v s');field=S.QQ.frac_field(v,s)
    A0,_,_,_=family(S.Rational(1,2),S.Rational(1,12));A,_,_,_=family(v,s)
    factors=[];sups=[];sign_checks=0
    for off,n,coord in [(0,6,0),(6,4,2)]:
        C0=A0[off:off+n,coord:coord+2].T.col_join(S.ones(1,n));C=A[off:off+n,coord:coord+2].T.col_join(S.ones(1,n))
        vertices,status=closure_vertices(C0);vv=[]
        for ids,rr,b0,ok,consistent in status:
            if symbolic:
                FF=DM.from_Matrix(C[:,ids]).convert_to(field)
                ee=DM.from_Matrix(S.Matrix([0,0,1])).convert_to(field)
                bf=(FF.extract(rr,list(range(len(ids)))).inv()*ee.extract(rr,[0]))
                residual=FF*bf-ee
                if consistent:assert residual.is_zero_matrix
                if not consistent:continue
                for k in range(len(ids)):
                    if b0[k]==0:
                        assert bf.to_list()[k][0]==field.zero
                        sign_checks+=1
                bb=bf.to_list()
            else:bb=[[field.from_sympy(z)] for z in b0]
            if ok:
                labels=[off+i for i in ids]
                vv.append((labels,b0,bb));sups.append(labels)
        factors.append(vv)
    qmax=S.Rational(0);total=0;tight=0;strict_gap=None;tight_failures=[];base_witness=None
    for labels1,b1,bf1 in factors[0]:
      for labels2,b2,bf2 in factors[1]:
        labels=labels1+labels2;betas=list(b1/2)+list(b2/2);bets={i:b for i,b in zip(labels,betas)}
        fbet={i:bb[0]/field(2) for i,bb in zip(labels,bf1+bf2)}
        cross0={};crossf={}
        for i,j in itertools.combinations(labels,2):
            cross0[i,j]=bets[i]*bets[j]*(A0[i,:]*O*A0[j,:].T)[0]
            cross0[j,i]=-cross0[i,j]
            if symbolic:
                wij=field.from_sympy((A[i,:]*O*A[j,:].T)[0]);q=fbet[i]*fbet[j]*wij
                crossf[i,j]=q;crossf[j,i]=-q
        first=min(labels)
        for tail in itertools.permutations([i for i in labels if i!=first]):
            sig=[first]+list(tail);val=sum(cross0[i,j] for i,j in itertools.combinations(sig,2));total+=1
            if val>qmax:qmax=val;base_witness={'sigma':sig,'beta':[str(bets[i]) for i in sig]}
            assert val<=S.Rational(1,2)
            if val==S.Rational(1,2):
                tight+=1
                if symbolic:
                    fun=sum((crossf[i,j] for i,j in itertools.combinations(sig,2)),field.zero)
                    if fun!=field(S.Rational(1,2)):tight_failures.append({'sigma':sig,'difference':str(fun-field(S.Rational(1,2)))})
            else:
                gap=S.Rational(1,2)-val;strict_gap=gap if strict_gap is None else min(strict_gap,gap)
    assert qmax==S.Rational(1,2)
    assert not tight_failures,tight_failures
    return {'arithmetic':'exact QQ and QQ(v,s)','closure_vertex_supports':sups,'ordered_candidates':total,
      'tight_ordered_candidates':tight,'strict_Q_gap_at_base':str(strict_gap),'max_Q':'1/2','capacity':'1',
      'zero_weight_identity_checks':sign_checks,'tight_identities_pass':True,'witness':base_witness,'seconds':time.time()-start}
if __name__=='__main__':
    r=verify_product();print(json.dumps(r,indent=2));json.dump(r,open('evidence/product_exact.json','w'),indent=2)
