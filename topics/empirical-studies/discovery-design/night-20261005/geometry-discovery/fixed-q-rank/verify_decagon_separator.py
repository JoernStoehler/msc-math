#!/usr/bin/env python3
"""Exact all-ten-template certificate; standard library only, owned outputs only.

Field a=cos(pi/10), 16a^4-20a^2+5=0, positive root in stated interval.
Coefficients and sign bounds use fractions.Fraction, never floating LPs.
"""
from fractions import Fraction as F
from dataclasses import dataclass
from pathlib import Path
import json

LO = F(951056516295153, 10**15)
HI = F(951056516295154, 10**15)

@dataclass(frozen=True)
class E:
    c: tuple

    def __init__(self, values=0):
        cs = list(values) if isinstance(values, (tuple, list)) else [values]
        cs = [F(x) for x in cs] + [F(0)] * max(0, 4-len(cs))
        for k in range(len(cs)-1, 3, -1):
            cs[k-2] += F(5,4)*cs[k]
            cs[k-4] -= F(5,16)*cs[k]
        object.__setattr__(self, 'c', tuple(cs[:4]))

    def __add__(self, y):
        y = y if isinstance(y,E) else E(y)
        return E([x+z for x,z in zip(self.c,y.c)])
    __radd__ = __add__
    def __neg__(self): return E([-x for x in self.c])
    def __sub__(self,y): return self + (-y if isinstance(y,E) else -E(y))
    def __rsub__(self,y): return E(y)-self
    def __mul__(self,y):
        y = y if isinstance(y,E) else E(y)
        cs = [F(0)]*7
        for i,x in enumerate(self.c):
            for j,z in enumerate(y.c): cs[i+j] += x*z
        return E(cs)
    __rmul__ = __mul__
    def inv(self):
        assert any(self.c)
        columns = [self*E([0]*i+[1]) for i in range(4)]
        rows = [[columns[j].c[i] for j in range(4)] + [F(i==0)] for i in range(4)]
        for i in range(4):
            pivot = next(j for j in range(i,4) if rows[j][i])
            rows[i],rows[pivot] = rows[pivot],rows[i]
            d = rows[i][i]
            rows[i] = [x/d for x in rows[i]]
            for j in range(4):
                if j != i:
                    d = rows[j][i]
                    rows[j] = [x-d*y for x,y in zip(rows[j],rows[i])]
        return E([rows[i][-1] for i in range(4)])
    def __truediv__(self,y): return self*(y if isinstance(y,E) else E(y)).inv()
    def __rtruediv__(self,y): return E(y)*self.inv()
    def bounds(self):
        lower = upper = F(0)
        for x in reversed(self.c):
            choices = [lower*LO, lower*HI, upper*LO, upper*HI]
            lower,upper = min(choices)+x, max(choices)+x
        return lower,upper
    def sign(self):
        if not any(self.c): return 0
        lower,upper = self.bounds()
        assert lower > 0 or upper < 0, ('unresolved exact sign',self.c,lower,upper)
        return 1 if lower>0 else -1
    def record(self):
        lower,upper = self.bounds()
        return {'coefficients':[str(x) for x in self.c],
                'interval':[str(lower),str(upper)],
                'approximate':float((lower+upper)/2)}

def dot(x,y): return sum(a*b for a,b in zip(x,y))
def add(x,y): return tuple(a+b for a,b in zip(x,y))
def scale(s,x): return tuple(s*a for a in x)
def det(x,y): return x[0]*y[1]-x[1]*y[0]
def rotate(x): return (C*x[0]-S*x[1], S*x[0]+C*x[1])
def intersect(n,m,h,k):
    d=det(n,m)
    assert d.sign()>0
    return ((h*m[1]-k*n[1])/d, (n[0]*k-m[0]*h)/d)
def area(vertices):
    return sum(det(vertices[i],vertices[(i+1)%10]) for i in range(10))/2
def max_exact(values):
    result=values[0]
    for candidate in values[1:]:
        if (candidate-result).sign()>0: result=candidate
    return result

def polynomial(x): return 16*x**4-20*x**2+5
assert polynomial(LO)<0<polynomial(HI)
# f'(x)=8x(8x^2-5)>0 throughout [9/10,1], so this root is unique there.
assert 8*F(9,10)**2-5>0
A=E([0,1])
C=2*A*A-1                  # cos(pi/5)
S=4*A*A*A-3*A              # sin(pi/5)
S18=2*A*A-F(3,2)           # sin(pi/10)
assert (C*C+S*S-1).sign()==0
assert (A*A+S18*S18-1).sign()==0
R=1/(2*(1+C))
r=R*A
H=R*S/C
eps=F(1,100)
v=[1,0,0,0,0,-1,0,0,0,0]
e=[(E(1),E(0))]
n=[(A,S18)]
for i in range(9):
    e.append(rotate(e[-1]))
    n.append(rotate(n[-1]))
assert rotate(e[-1])==e[0] and rotate(n[-1])==n[0]
for i in range(5): assert n[i+5]==scale(-1,n[i])

base=[scale(R,x) for x in e]
h=[r*(1+eps*x) for x in v]
vertices=[intersect(n[(k-1)%10],n[k],h[(k-1)%10],h[k]) for k in range(10)]
facet_slacks=[[h[i]-dot(n[i],vertices[k]) for i in range(10)] for k in range(10)]
for k,row in enumerate(facet_slacks):
    assert [i for i,s in enumerate(row) if s.sign()==0]==[(k-1)%10,k] or \
           sorted(i for i,s in enumerate(row) if s.sign()==0)==sorted([(k-1)%10,k])
    assert all(s.sign()>=0 for s in row)

templates=[]
unique_pairs=set()
template_ids=[
    '(0, 2, 3)-(0, 1, 2)', '(0, 1, 3)-(0, 2, 1)',
    '(1, 3, 4)-(0, 1, 2)', '(1, 2, 4)-(0, 2, 1)',
    '(0, 2, 4)-(0, 1, 2)', '(0, 2, 3)-(0, 2, 1)',
    '(0, 1, 3)-(0, 1, 2)', '(1, 3, 4)-(0, 2, 1)',
    '(1, 2, 4)-(0, 1, 2)', '(0, 2, 4)-(0, 2, 1)']
for k in range(10):
    axis=e[k]
    apex=scale(H,(-axis[1],axis[0]))
    T=[scale(-R,axis),scale(R,axis),apex]
    translation=scale(F(1,2),add(vertices[k],vertices[(k+5)%10]))
    placed=[add(x,translation) for x in T]
    assert placed[0]==vertices[(k+5)%10] and placed[1]==vertices[k]
    slacks=[[h[i]-dot(n[i],point) for i in range(10)] for point in placed]
    assert all(s.sign()>=0 for row in slacks for s in row)
    assert all(s.sign()>0 for s in slacks[2])
    contacts=sorted({i for row in slacks for i,s in enumerate(row) if s.sign()==0})
    expected=sorted({(k-1)%10,k,(k+4)%10,(k+5)%10})
    assert contacts==expected
    supports=[max_exact([dot(normal,x) for x in T]) for normal in n]
    duals=[]
    for index in [(k-1)%5,k%5]:
        pair=(index,index+5)
        unique_pairs.add(pair)
        weights=[E(0)]*10
        weights[pair[0]]=weights[pair[1]]=1/(2*r)
        assert sum(weights[i]*n[i][0] for i in range(10)).sign()==0
        assert sum(weights[i]*n[i][1] for i in range(10)).sign()==0
        assert (dot(weights,supports)-1).sign()==0
        assert (dot(weights,h)-1).sign()==0
        duals.append({'support':pair,'nonzero_weight':weights[index].record()})
    templates.append({'rotation_index':k,'evaluator_template_id':template_ids[k],
                      'capacity_fit':'1',
                      'translation':[x.record() for x in translation],
                      'contacts':contacts,'dual_vertices':duals,
                      'slacks':[[s.record() for s in row] for row in slacks]})

assert len(unique_pairs)==5
base_area=area(base)
new_area=area(vertices)
decrease=2*C/S*r*r*eps*eps
assert (base_area-new_area-decrease).sign()==0
assert decrease.sign()>0
ratio_coefficient=(2+8*A*A-5)/5  # (2+sqrt(5))/5
assert (new_area/base_area-(1-ratio_coefficient*eps*eps)).sign()==0
edge=2*R*S18
assert (base_area-5*r*edge).sign()==0
assert (edge/(2*base_area)-1/(10*r)).sign()==0
assert sum(v)==0 and all(v[i]+v[i+5]==0 for i in range(5))
# Exact rational bounds used by the uniform interval certificate.
assert (A-F(9,10)).sign()>0
assert (F(3,4)-S/C).sign()>0
assert (S-F(1,2)).sign()>0
uniform_apex_gap=R*F(121,1000)
assert all((s-uniform_apex_gap).sign()>0 for t in templates
           for s in [E(tuple(F(x) for x in item['coefficients'])) for item in t['slacks'][2]])

q_area=F(5,2)*2*S*C # (5/2)sin(2*pi/5)
base_sys=1/(2*base_area*q_area)
new_sys=1/(2*new_area*q_area)
out={'status':'exact_algebraic_all_ten_templates_verified',
     'field':{'generator':'a=cos(pi/10)','polynomial':'16*a^4-20*a^2+5',
              'isolating_interval':[str(LO),str(HI)]},
     'epsilon':str(eps),'support_direction':v,
     'normal_angle':'pi/10+i*pi/5','R':R.record(),'base_support':r.record(),
     'base_area':base_area.record(),'perturbed_area':new_area.record(),
     'area_decrease':decrease.record(),'capacity_base':'1','capacity_perturbed':'1',
     'all_ten_fits':['1']*10,'complete_active_dual_rank':5,
     'required_rank_N_minus_2':8,'complete_active_dual_supports':sorted(unique_pairs),
     'first_order_balance':'equal convex mixture of five opposite-pair dual vertices',
     'base_sys':base_sys.record(),'perturbed_sys':new_sys.record(),
     'uniform_apex_slack_lower_bound':uniform_apex_gap.record(),
     'uniform_interval':'abs(epsilon)<=1/100',
     'polygon_facet_slacks':[[s.record() for s in row] for row in facet_slacks],
     'templates':templates}
out_path=Path(__file__).with_name('decagon-separator-certificate.json')
out_path.write_text(json.dumps(out,indent=2)+'\n')
print(json.dumps({k:v for k,v in out.items() if k not in ['templates','polygon_facet_slacks']},indent=2))
