"""Exact single-contact realization and one-block escape check; no global claim.
Run from repository root: uv run --with sympy python PATH/verify_stratum.py
"""
from pathlib import Path
import importlib.util,json
ROOT=Path(__file__).resolve().parents[4]
fixture_path=ROOT/'docs/empirical-viterbo-design/desk/pentagon-arbitrary-partner/local-cover/pentagon_fixture.py'
spec=importlib.util.spec_from_file_location('fixture',fixture_path);f=importlib.util.module_from_spec(spec);spec.loader.exec_module(f)
obj=f.make_fixture()
def dec(cs):
 p=f.Z
 for c in cs:p=p*f.tau+f.F(f.sp.Rational(c))
 return p
sh=[[[dec(c) for c in p] for p in s['vertices']] for s in obj['normal_shapes']]
raw=[(0,0),(10,1),(-3,10),(0,0),(6,1),(-5,0),(1,4),(4,-10),(1,4),(-10,1)]
t=[[f.F(f.sp.Rational(x,50)),f.F(f.sp.Rational(y,50))] for x,y in raw]
A=(0,0);B=(5,0);C=(9,0)
b=f.add(sh[B[0]][B[1]],t[B[0]]);c=f.add(sh[C[0]][C[1]],t[C[0]])
t[0]=f.sub(f.scale(f.O/f.F(2),f.add(b,c)),sh[A[0]][A[1]])
anchor=t[5];t=[f.sub(z,anchor) for z in t]
cycle=[(5,0),(9,0),(7,0),(7,2),(4,1),(1,1),(2,1),(8,1),(0,2),(9,1)]
d=f.sub(f.add(sh[C[0]][C[1]],t[C[0]]),f.add(sh[B[0]][B[1]],t[B[0]]))
def points(s):return {(j,k):f.add(p,f.add(t[j],f.scale(s,d) if j==0 else [f.Z,f.Z])) for j,shape in enumerate(sh) for k,p in enumerate(shape)}
def determinants(s):
 P=points(s);out={}
 for i,l in enumerate(cycle):
  r=cycle[(i+1)%len(cycle)]
  for v in P:out[(l,r,v)]=f.cross(f.sub(P[r],P[l]),f.sub(P[v],P[l]))
 return out
D0=determinants(f.Z);D1=determinants(f.O);D2=determinants(f.F(2))
zeros=[];strict=[];lower=[(-f.O/f.F(2),'contact reaches B')];upper=[(f.O/f.F(2),'contact reaches C')]
for key,z in D0.items():
 l,r,v=key;coef=D1[key]-z
 assert D2[key]-D1[key]==coef
 if v in (l,r) or (l==B and r==C and v==A):
  assert z==f.Z and coef==f.Z;zeros.append(key);continue
 assert f.sign(z)>0;strict.append(key)
 if f.sign(coef)>0:lower.append((-z/coef,str(key)))
 if f.sign(coef)<0:upper.append((-z/coef,str(key)))
lo=lower[0];hi=upper[0]
for x in lower[1:]:
 if f.sign(x[0]-lo[0])>0:lo=x
for x in upper[1:]:
 if f.sign(x[0]-hi[0])<0:hi=x
assert f.sign(lo[0])<0 and f.sign(hi[0])>0
P=points(f.Z);H=f.hull(list(P.values()));assert len(H)==len(cycle)
assert f.area([P[l] for l in cycle])==f.area(H)
area0=f.area([P[l] for l in cycle]);area1=f.area([points(f.O)[l] for l in cycle])-area0
assert f.area([points(f.F(2))[l] for l in cycle])==area0+f.F(2)*area1
m=dec(obj['candidate_K_area']);assert f.sign(area0-m)>0
for s in (lo[0],hi[0]):
 assert all(f.sign(z)>=0 for z in determinants(s).values())
 assert f.area(f.hull(list(points(s).values())))==area0+s*area1
num=lambda a:float(f.F.to_sympy(a))
ser=lambda a:f.serialize(a)
out={'status':'PASS exact realized codimension-one contact; non-HKO; no global bound',
 'arithmetic':obj['arithmetic'],'field_polynomial':obj['field_polynomial'],'root_interval':obj['root_interval'],
 'coefficient_format':obj['coefficient_format'],'templates_source':str(fixture_path.relative_to(ROOT)),
 'anchor_template':5,'translations':[[ser(z) for z in p] for p in t],
 'contact':{'A':A,'B':B,'C':C,'initial_relative_parameter':'1/2','slide_direction':[ser(z) for z in d]},
 'hull_cycle':cycle,'actual_side_count':len(cycle),'strict_determinant_count':len(strict),'prescribed_or_endpoint_zero_count':len(zeros),
 'area_polynomial':{'constant':ser(area0),'slide_linear_coefficient':ser(area1),'constant_numeric':num(area0),'linear_numeric':num(area1)},
 'm':ser(m),'m_numeric':num(m),'area_minus_m_sign':f.sign(area0-m),
 'feasible_slide_interval':{'lo':ser(lo[0]),'hi':ser(hi[0]),'lo_numeric':num(lo[0]),'hi_numeric':num(hi[0]),'lo_new_contact':lo[1],'hi_new_contact':hi[1]},
 'endpoint_areas':[{'s':ser(s),'area':ser(area0+s*area1),'area_numeric':num(area0+s*area1),'area_minus_m_sign':f.sign(area0+s*area1-m)} for s in (lo[0],hi[0])],
 'NOT_verified':['global fixed-Q area inequality','stationarity or local minimality of example','capacity exactly one of example','new equality family']}
# Sparse exact polynomials on all 18 free translation coordinates (t5=0).
def padd(a,b):
 out=dict(a)
 for mon,c in b.items():out[mon]=out.get(mon,f.Z)+c
 return {mon:c for mon,c in out.items() if c!=f.Z}
def pscale(a,c):return {mon:c*z for mon,z in a.items() if c*z!=f.Z}
def pmul(a,b):
 out={}
 for u,c in a.items():
  for v,z in b.items():
   mon=tuple(sorted(u+v));out[mon]=out.get(mon,f.Z)+c*z
 return {mon:c for mon,c in out.items() if c!=f.Z}
def psub(a,b):return padd(a,pscale(b,-f.O))
def pcross(a,b):return psub(pmul(a[0],b[1]),pmul(a[1],b[0]))
def pv(label):
 j,k=label;out=[]
 for coord,name in enumerate(('x','y')):
  poly={():sh[j][k][coord]}
  if j!=5:poly[(f'{name}_{j}',)]=f.O
  out.append(poly)
 return out
def pvsub(a,b):return [psub(a[k],b[k]) for k in range(2)]
areap={}
for i,l in enumerate(cycle):areap=padd(areap,pscale(pcross(pv(l),pv(cycle[(i+1)%len(cycle)])),f.O/f.F(2)))
contactp=pcross(pvsub(pv(C),pv(B)),pvsub(pv(A),pv(B)))
def peval(poly,translations):
 values={f'{name}_{j}':translations[j][coord] for j in range(10) if j!=5 for coord,name in enumerate(('x','y'))}
 result=f.Z
 for mon,coefficient in poly.items():
  term=coefficient
  for variable in mon:term*=values[variable]
  result+=term
 return result
assert peval(areap,t)==area0 and peval(contactp,t)==f.Z
assert d!=[f.Z,f.Z]
polyser=lambda p:[{'monomial':list(mon),'coefficient':ser(c)} for mon,c in sorted(p.items())]
out['full_translation_variables']=[f'{name}_{j}' for j in range(10) if j!=5 for name in ('x','y')]
out['full_area_polynomial']=polyser(areap)
out['contact_polynomial']=polyser(contactp)
out['strict_hull_inequality_labels']=[list(key) for key in strict]
Path(__file__).with_name('stratum-certificate.json').write_text(json.dumps(out,indent=2)+'\n')
print(json.dumps({k:out[k] for k in ['status','actual_side_count','strict_determinant_count','area_polynomial','m_numeric','feasible_slide_interval','endpoint_areas']},indent=2))
