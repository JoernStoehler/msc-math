"""Independent all-choice source/cone inventory; no producer imports or writes."""
from pathlib import Path
import importlib.util,json,time,hashlib
from fractions import Fraction
from itertools import product
root=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('independent_pair_arithmetic',root.parent.parent/'hko-angle-nonconformal'/'review_candidates.py')
m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
E,z,one,r=m.E,m.z,m.one,m.r

N=[(one,z),((r-one)/4,(E(5)-r)/4),(-(one+r)/4,(3*r-E(5))/4),(-(one+r)/4,-(3*r-E(5))/4),((r-one)/4,-(E(5)-r)/4)]
b=(one+r)/4;R=(E(5)-r)/10;tau2=E(5)-2*r
triangles=[];cur=[(-R,z),(R,z),(z,R)]
for j in range(10):
 triangles.append(cur);cur=[(b*x-(3*r-E(5))/4*y,b*(x+y)) for x,y in cur]
assert cur==triangles[0]
lams=m.lams

def dot(a,b):return sum((x*y for x,y in zip(a,b)),z)
def cross(a,b):return a[0]*b[1]-a[1]*b[0]
def solve(a,b,c,d):
 det=cross(a,b);assert det!=z
 return ((c*b[1]-d*a[1])/det,(a[0]*d-b[0]*c)/det)
def positive_normalize(row):
 first=next(v for v in row if v!=z);scale=first if first.sign()>0 else -first
 assert scale.sign()>0
 return [v/scale for v in row]
def key(row):return tuple(v.key() for v in row)
def rank(rows):
 rows=[r[:] for r in rows];at=0
 for col in range(len(rows[0])):
  pivot=next((i for i in range(at,len(rows)) if rows[i][col]!=z),None)
  if pivot is None:continue
  rows[at],rows[pivot]=rows[pivot],rows[at];factor=rows[at][col];rows[at]=[v/factor for v in rows[at]]
  for i in range(at+1,len(rows)):
   factor=rows[i][col]
   if factor!=z:rows[i]=[v-factor*w for v,w in zip(rows[i],rows[at])]
  at+=1
  if at==len(rows[0]):break
 return at

def areaP(h):return sum((h[i]*h[(i+1)%5]-m.a*h[i]*h[i] for i in range(5)),z)
def edge(h,i):return h[i-1]+h[(i+1)%5]-2*m.a*h[i]
def gauge(h):
 t=solve(N[3],N[4],-h[3],-h[4]);g=[h[i]+dot(N[i],t) for i in range(5)]
 assert g[3]==g[4]==z;return g

def main():
 started=time.perf_counter();raw=[];rows={};origins={};groups=[];rawmap={}
 source=root.parent/'joint-denominators'/'joint-denominators.json';packet=json.loads(source.read_text());sourcechoices={}
 for l,circuit in enumerate(packet['circuits']):
  for candidate in circuit['candidates']:
   for choice in candidate['sources']:
    sk=(l,choice[0],tuple(choice[1:]));assert sk not in sourcechoices;sourcechoices[sk]=list(map(m.decd,candidate['coefficients']))
 assert len(sourcechoices)==1350
 for nx,ny in N:assert nx*nx+ny*ny/tau2==one
 for l,lam in enumerate(lams):
  assert all(dot(lam,[N[i][k] for i in range(5)])==z for k in range(2))
  active=[i for i,v in enumerate(lam) if v.sign()>0];assert len(active)==3
  for j,tri in enumerate(triangles):
   for choice in product(range(3),repeat=len(active)):
    coeff=[z]*4
    for i,k in zip(active,choice):
     nx,ny=N[i];vx,vy=tri[k]
     terms=[nx*vx,ny*vx,nx*vy,ny*vy]
     coeff=[c+lam[i]*v for c,v in zip(coeff,terms)]
    # Pull back general L onto old L=diag(x,1)C_u exactly.
    assert [coeff[3],coeff[0],tau2*coeff[2],-coeff[1]]==sourcechoices[l,j,choice]
    row=list(lam[:3])+[-v for v in coeff];normal=positive_normalize(row);kk=key(normal)
    rows.setdefault(kk,normal);origins.setdefault(kk,[]).append((l,j,choice));raw.append(row);rawmap[('fit',l,j,choice)]=row
 for i in range(5):
  row=[z]*7
  for j in range(3):row[j]=(one if j==(i-1)%5 else z)+(one if j==(i+1)%5 else z)-(2*m.a if j==i else z)
  normal=positive_normalize(row);kk=key(normal);rows.setdefault(kk,normal);origins.setdefault(kk,[]).append(('edge',i));raw.append(row);rawmap[('edge',i)]=row
 # Symbolic translation invariance reduces to the exact regular recurrence.
 for i in range(5):
  assert all(N[(i-1)%5][k]+N[(i+1)%5][k]==2*m.a*N[i][k] for k in range(2))
  assert (N[i][0],-N[i][1])==N[-i%5]
 for l,lam in enumerate(lams):assert [lam[-i%5] for i in range(5)]==lams[4-l]
 for h in ([one]*5,[E(i+1,i-2) for i in range(5)]):
  g=gauge(h);assert areaP(h)==areaP(g)
  assert [edge(h,i) for i in range(5)]==[edge(g,i) for i in range(5)]
  assert all(dot(lam,h)==dot(lam,g) for lam in lams)
  assert areaP(h)==areaP([h[-i%5] for i in range(5)])
 # Exact reflected/re-gauged action on all seven variables preserves the cone.
 columns=[]
 for j in range(7):
  if j<3:
   hh=[one if i==j else z for i in range(5)];gg=gauge([hh[-i%5] for i in range(5)]);columns.append(gg[:3]+[z]*4)
  else:columns.append([z]*3+[E((-1 if k in (1,3) else 1) if k==j-3 else 0) for k in range(4)])
 reflected={key(positive_normalize([dot(row,col) for col in columns])) for row in rows.values()}
 assert reflected==set(rows)
 # Area conversion constant is independently bound to the physical fixed Q.
 qvertices=[solve(N[i],N[(i+1)%5],b,b) for i in range(5)]
 qarea=sum((cross(qvertices[i],qvertices[(i+1)%5]) for i in range(5)),z)/2
 kappa=(E(5)-r)/4;assert 2*tau2*qarea/kappa==E(5)
 # Strict witness: identity matrix and regular support H=1, then exact gauge.
 h=gauge([one]*5);witness=h[:3]+[one,z,z,one]
 slacks=[dot(row,witness) for row in rows.values()];assert all(v.sign()>0 for v in slacks)
 # Singular matrices can lie strictly inside the coefficient cone.
 singularwitness=h[:3]+[z]*4;assert all(dot(row,singularwitness).sign()>0 for row in rows.values())
 # Accepted HKO equality is an exact source/control evaluation, no old rerun.
 eq=json.loads((root.parent/'class-proof'/'equality.json').read_text())['equality_bodies'][0]
 H=list(map(m.decd,eq['supports']));u=m.decd(eq['u']);x=m.decd(eq['x'])
 L=[x,-x*u,tau2*u,one];control=H[:3]+L
 assert all(dot(row,control).sign()>=0 for row in rows.values())
 determinant=L[0]*L[3]-L[1]*L[2];assert areaP(H)==determinant/(E(3)+r)
 rnk=rank(list(rows.values()));assert rnk==7
 # Constant target inertia is an ambient arithmetic control, not cone positivity.
 H3=[[-m.a,one/2,z],[one/2,-m.a,one/2],[z,one/2,-m.a]]
 principal=[H3[0][0],H3[0][0]*H3[1][1]-H3[0][1]*H3[1][0],m.det(H3)]
 assert [v.sign() for v in principal]==[-1,-1,1]
 assert [principal[0].sign(),(principal[1]/principal[0]).sign(),(principal[2]/principal[1]).sign()]==[-1,1,-1]
 target=[[z]*7 for _ in range(7)]
 for i in range(3):
  for j in range(3):target[i][j]=H3[i][j]
 kk=one/(E(3)+r);target[3][6]=target[6][3]=-kk/2;target[4][5]=target[5][4]=kk/2
 # Two independent determinant variable pairs each give one positive/negative square.
 assert kk.sign()>0;target_inertia=[3,4,0]
 bits=max(max(v.a.numerator.bit_length(),v.a.denominator.bit_length(),v.b.numerator.bit_length(),v.b.denominator.bit_length()) for row in rows.values() for v in row)
 producer_checked=False
 if (root/'cone.json').exists():
  cone=json.loads((root/'cone.json').read_text());decrow=lambda rr:list(map(m.decd,rr))
  assert [tuple(decrow(n)) for n in cone['normals']]==N
  assert [[tuple(decrow(v)) for v in tri] for tri in cone['templates']]==triangles
  assert list(map(decrow,cone['closure_rays']))==lams
  assert m.decd(cone['tau_squared'])==tau2
  got=list(map(lambda rec:decrow(rec['coefficients']),cone['rows']))
  assert len(got)==len(rows) and {key(rr) for rr in got}==set(rows)
  seen=set()
  for rr,record in zip(got,cone['rows']):
   for occurrence in record['sources']:
    origin=occurrence['source'];divisor=m.decd(occurrence['positive_normalization_divisor']);assert divisor.sign()>0
    if origin['kind']=='fit':
     l,j=origin['circuit'],origin['template'];choice=tuple(origin['vertices'])
     assert origin['active_normals']==[i for i,v in enumerate(lams[l]) if v.sign()>0];rk=('fit',l,j,choice)
    else:rk=('edge',origin['edge'])
    assert rk not in seen;seen.add(rk);assert [v*divisor for v in rr]==rawmap[rk]
  assert seen==set(rawmap)
  assert cone['rank']==rnk and cone['lineality_dimension']==0 and cone['lineality_basis']==[]
  assert rank([got[i] for i in cone['independent_row_indices']])==7
  assert cone['raw_fit_count']==1350 and cone['raw_edge_count']==5 and cone['raw_total']==1355
  assert cone['normalized_distinct_fit_rows']==605 and cone['normalized_distinct_edge_rows']==5 and cone['normalized_distinct_total']==610
  assert cone['maximum_rational_coefficient_bit_size']==bits
  assert decrow(cone['strict_invertible_witness'])==witness and decrow(cone['strict_singular_witness'])==singularwitness
  minimum=lambda vals:next(v for v in vals if all((v-w).sign()<=0 for w in vals))
  assert m.decd(cone['strict_invertible_minimum_slack'])==minimum(slacks)
  assert m.decd(cone['strict_singular_minimum_slack'])==minimum([dot(rr,singularwitness) for rr in got])
  transform=list(map(decrow,cone['reflection_map']));assert transform==[[columns[j][i] for j in range(7)] for i in range(7)]
  for i,j in enumerate(cone['reflection_row_permutation']):assert positive_normalize([dot(got[i],col) for col in columns])==got[j]
  assert decrow(cone['hko_control']['point'])==control and m.decd(cone['hko_control']['target_gap'])==z
  assert cone['scalar_chart_source_sha256']==hashlib.sha256((root.parent/'preflight'/'active-events.json').read_bytes()).hexdigest()
  Erows=[];Amat=[]
  for i in range(5):
   er=[(one if j==(i-1)%5 else z)+(one if j==(i+1)%5 else z)-(2*m.a if j==i else z) for j in range(5)]
   Erows.append(er);Amat.append([v/2 for v in er])
  if 'positive_det_quadratic_matrix' in cone:
   assert list(map(decrow,cone['positive_det_quadratic_matrix']))==target
   inertia=cone['ambient_inertia'];assert [inertia['positive'],inertia['negative'],inertia['zero']]==target_inertia
   assert decrow(inertia['support_principal_minors'])==principal
  assert list(map(decrow,cone['canonical_edge_rows']))==Erows and list(map(decrow,cone['area_quadratic_matrix']))==Amat
  producer_checked=True
 print(json.dumps({'status':'PASS independent source/gauge/reflection/inventory/strict-witness audit','variables':['h0','h1','h2','L00','L01','L10','L11'],'raw_fit_rows':1350,'raw_edge_rows':5,'unique_positive_scale_rows':len(rows),'rank':rnk,'lineality_dimension':7-rnk,'strict_full_dimensional':True,'max_coefficient_bit_size':bits,'strict_witness':[[str(v.a),str(v.b)] for v in witness],'elapsed_seconds':round(time.perf_counter()-started,3),'producer_packet_independently_accepted':producer_checked,'target_inertia_positive_negative_zero':target_inertia,'writes':False}))
 return rows,raw,origins,witness
if __name__=='__main__':main()
