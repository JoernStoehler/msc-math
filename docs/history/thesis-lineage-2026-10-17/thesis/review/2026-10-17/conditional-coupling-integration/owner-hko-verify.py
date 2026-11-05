#!/usr/bin/env python3
"""Independent Q(sqrt5) arithmetic, HKO LP certificates and explicit coupling."""
from fractions import Fraction as F
from pathlib import Path
from functools import total_ordering
import argparse,datetime,hashlib,importlib.util,json

@total_ordering
class Q5:
    def __init__(self,a=0,b=0):
        if isinstance(a,Q5):self.a,self.b=a.a,a.b
        else:self.a,self.b=F(a),F(b)
    def __add__(self,v):
        v=Q5(v);return Q5(self.a+v.a,self.b+v.b)
    __radd__=__add__
    def __neg__(self):return Q5(-self.a,-self.b)
    def __sub__(self,v):return self+-Q5(v)
    def __rsub__(self,v):return Q5(v)+-self
    def __mul__(self,v):
        v=Q5(v);return Q5(self.a*v.a+5*self.b*v.b,self.a*v.b+self.b*v.a)
    __rmul__=__mul__
    def __truediv__(self,v):
        v=Q5(v);return self*Q5(v.a,-v.b)*(F(1)/(v.a*v.a-5*v.b*v.b))
    def __rtruediv__(self,v):return Q5(v)/self
    def __eq__(self,v):
        v=Q5(v);return self.a==v.a and self.b==v.b
    def sign(self):
        a,b=self.a,self.b
        if not b:return (a>0)-(a<0)
        if not a:return (b>0)-(b<0)
        if a>0 and b>0:return 1
        if a<0 and b<0:return -1
        delta=a*a-5*b*b
        return ((delta>0)-(delta<0))*((a>0)-(a<0))
    def __lt__(self,v):return (self-Q5(v)).sign()<0
    def __float__(self):return float(self.a)+float(self.b)*5**.5
    def __repr__(self):return f'{self.a}+({self.b})sqrt5'
def read(v):return Q5(v['rational'],v['sqrt5']) if isinstance(v,dict) else Q5(v)
def load_helper(path):
    spec=importlib.util.spec_from_file_location('independent_rational',path);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m

def main():
    p=argparse.ArgumentParser();p.add_argument('--certificate',type=Path,required=True);p.add_argument('--out',type=Path,required=True);args=p.parse_args()
    helper=load_helper(Path(__file__).with_name('owner-verify.py'));x=json.loads(args.certificate.read_text());a=[[read(t) for t in v] for v in x['A']];b=[[read(t) for t in v] for v in x['B']]
    r=Q5(0,1);c=(r-1)/4;d=-(r+1)/4;k=(r-1)/2
    assert a==[[Q5(1),Q5(0)],[c,Q5(1)],[d,k],[d,-k],[c,Q5(-1)]]
    assert b==[[-v[1],v[0]] for v in a]
    eq,rhs=helper.equations(a,b);values=[]
    for i,v in enumerate(x['variants']):
        sign=1 if i==0 else -1;word=helper.angular(a,b,sign);assert word==v['word']
        co=helper.objective(a,b,word);assert co==[read(t) for t in v['objective']]
        assert eq==[[read(t) for t in row] for row in v['A_eq']]
        assert rhs==[read(t) for t in v['b_eq']]
        w=[read(t) for t in v['W']];dual=[read(t) for t in v['dual']];assert min(w)>=0
        assert [helper.dot(row,w) for row in eq]==rhs
        sl=[sum(eq[z][j]*dual[z] for z in range(len(eq)))-co[j] for j in range(25)]
        assert min(sl)>=0 and sl==[read(t) for t in v['dual_slack']]
        q=read(v['Q']);assert helper.dot(co,w)==helper.dot(rhs,dual)==q;values.append(q)
    assert values==[(7*r-5)/100,(5+3*r)/100]
    # Independent circulant witness, distinct from active-basis discovery.
    u=1/r;v=(1-u)/2;weights=[Q5(0),u,Q5(0),v,v]
    wc=[weights[(j-i)%5]/5 for i in range(5) for j in range(5)]
    assert min(wc)>=0 and [helper.dot(row,wc) for row in eq]==rhs
    minus=helper.objective(a,b,helper.angular(a,b,-1));qcir=helper.dot(minus,wc)
    assert qcir==values[1]
    # Average the negative-word coefficients by cyclic offset.
    averaged=[sum(minus[i*5+(i+j)%5] for i in range(5))/5 for j in range(5)]
    assert averaged==[Q5(0),Q5(F(3,20)),k/20,k/20,Q5(F(3,20))]
    true=Q5(F(1,10));assert read(x['Q_true'])==true
    factor=max(values)/true;cap_ratio=true/max(values)
    assert factor==(5+3*r)/10 and cap_ratio==(3*r-5)/2 and factor>Q5(F(117,100))
    out={'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'status':'PASS','field':'Q(sqrt5),rational-square sign comparison','LPs':2,'primal_dual_objective_equalities':2,'independent_circulant_feasible':True,'LP_over_Qtrue':float(factor),'capacity_lower_over_true':float(cap_ratio),'certificate_sha256':hashlib.sha256(args.certificate.read_bytes()).hexdigest(),'checker_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'helper_sha256':hashlib.sha256(Path(__file__).with_name('owner-verify.py').read_bytes()).hexdigest(),'scope':'LP matrices/words independently reconstructed; exact primal/dual, cyclic-average and explicit coupling; Qtrue imported from established regular-family formula via determinant rescaling.'}
    args.out.write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out))
if __name__=='__main__':main()
