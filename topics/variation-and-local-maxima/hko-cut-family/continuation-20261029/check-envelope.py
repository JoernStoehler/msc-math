from sage.all import *
from pathlib import Path
import json
here=Path(__file__).resolve().parent
data=json.loads((here/'chamber-results.json').read_text())
P=PolynomialRing(QQ,'z');z=P.gen();p=z**4-10*z**2+5
r=next(v for v,m in p.roots(AA) if 0<v<1)
K=NumberField(p,'tau');tau=K.gen();emb=K.hom([r],AA)
T=PolynomialRing(K,'t');F=T.fraction_field();t=F.gen()
AP=PolynomialRing(AA,'x');lo=QQ(3)/10;hi=QQ(2)/5
def decode(s):return F(sage_eval(s,locals={'tau':tau,'t':t}))
def roots_closed(poly):return [(v,m) for v,m in AP([emb(c) for c in poly.list()]).roots(AA) if lo<=v<=hi]
V=decode(data['volume_t']);q7=decode(data['Q7_t']);q8=decode(data['Q8_t'])
literal=((-QQ(30379)/43200*tau**2+QQ(26027)/2880)*t**3+(QQ(6521)/1080*tau**2-QQ(6265)/72)*t**2+(-QQ(25655)/1728*tau**2+QQ(379705)/1728)*t+QQ(3125)/288*tau**2-QQ(15625)/96)/(t**3-QQ(40)/3*t**2+QQ(325)/9*t-QQ(250)/9)
assert V==literal
for v in [V,q7,q8]:
 assert not roots_closed(v.numerator()) and not roots_closed(v.denominator())
 assert emb(v(lo))>0
R7=1/(8*V*q7**2);R8=1/(8*V*q8**2)
assert R7==decode(data['rho7_feasible_upper_t']) and R8==decode(data['rho8_feasible_upper_t'])
for d in [R7.derivative(),R8.derivative()]:
 assert not roots_closed(d.numerator()) and not roots_closed(d.denominator())
assert emb(R7.derivative()(lo))>0
assert emb(R8.derivative()(hi))<0
cross=q7-q8;rr=roots_closed(cross.numerator());assert len(rr)==1 and rr[0][1]==1
assert emb(cross(lo))>0 and emb(cross(hi))<0
crossing=rr[0][0]
def evaluate_aa(v,x):
 return AP([emb(c) for c in v.numerator().list()])(x)/AP([emb(c) for c in v.denominator().list()])(x)
height=evaluate_aa(R7,crossing)
assert height==evaluate_aa(R8,crossing)
assert QQ(3403557218556924)/10**16<crossing<QQ(3403557218556926)/10**16
assert QQ(10524316650246791)/10**16<height<QQ(10524316650246794)/10**16
assert height>emb(R7(lo))
out={'pass':True,'literal_README_volume_matches':True,'V_Q7_Q8_positive_finite_on_closed_interval':True,'R7_derivative_strict_positive':True,'R8_derivative_strict_negative':True,'unique_simple_crossover':str(crossing),'cusp_upper_bound':str(height),'cusp_decimal_bracket':['1.0524316650246791','1.0524316650246794'],'cusp_exceeds_certified_endpoint':True}
assert height < QQ(65777)/62500
out['rational_envelope_upper_bound']='65777/62500' # 1.052432
print(json.dumps(out,indent=2))
