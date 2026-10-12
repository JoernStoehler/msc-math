# /// script
# dependencies = ["sympy"]
# ///
"""Independent exact jet/volume/mixture audit; no producer imports or writes."""
import hashlib,io,itertools,json,sys,zipfile
from pathlib import Path
import sympy as S
here=Path(__file__).resolve().parent
repo=next(p for p in here.parents if (p/'formal').is_dir() and (p/'experiments').is_dir())
raw=(here/'curvature.json').read_bytes();packet=json.loads(raw)
for source in packet['sources'].values():
 assert hashlib.sha256((repo/source['path']).read_bytes()).hexdigest()==source['sha256']
archive=packet['sources']['old_jet_archive']
with zipfile.ZipFile(repo/archive['path']) as outer:inner=outer.read(archive['nested_member'])
with zipfile.ZipFile(io.BytesIO(inner)) as zipped:oldraw=zipped.read(archive['inner_member'])
assert hashlib.sha256(oldraw).hexdigest()==archive['member_sha256'];historical=json.loads(oldraw)
sys.path.insert(0,str(repo/'topics/variation-and-local-maxima/hko-local-maximum/non-hko/code'))
from family_exact import family
A,_,_,_=family(S.Rational(1,2),S.Rational(1,12));W=S.Matrix(packet['W']).reshape(10,4)
assert list(W.reshape(40,1))==list(S.Matrix(historical['flat_vector']))
expected=S.Matrix([[0,0,0,0]]*6+[[S.Rational(1,2),S.Rational(1,3),0,0],[S.Rational(3,2),-1,0,0],[1,0,0,0],[0,0,0,0]])
assert W==expected
old=json.loads((repo/packet['sources']['old_certificate']['path']).read_text())
locked=json.loads((repo/packet['sources']['locked_certificate']['path']).read_text())
def om(x,y):return x[0]*y[2]+x[1]*y[3]-x[2]*y[0]-x[3]*y[1]
# Independent physical-volume derivatives by multilinearity of tetra determinants.
X=S.Matrix(historical['primal_vertices']);DX=S.zeros(24,4);DDX=S.zeros(24,4)
for i,facets in enumerate(historical['incidence']):
 N=A[facets,:];NW=W[facets,:];inv=N.inv();x=X[i,:].T
 assert N*x==S.ones(4,1)
 DX[i,:]=(-inv*NW*x).T;DDX[i,:]=(2*inv*NW*inv*NW*x).T
volume=[S.Rational(0)]*3
for tet,orientation in historical['pulling_tetrahedra']:
 M=X[tet,:];volume[0]+=orientation*M.det()/24
 for i in range(4):
  dM=M.copy();dM[i,:]=DX[tet[i],:];volume[1]+=orientation*dM.det()/24
  ddM=M.copy();ddM[i,:]=DDX[tet[i],:];volume[2]+=orientation*ddM.det()/24
 for i,j in itertools.combinations(range(4),2):
  ddM=M.copy();ddM[i,:]=DX[tet[i],:];ddM[j,:]=DX[tet[j],:];volume[2]+=orientation*ddM.det()/12
assert volume==[S.Rational(1,2),0,S.Rational(13,864)]
AN=A.col_join(A[0,:]);WN=W.col_join(W[0,:])
def check_germ(record,slopes=None):
 word=record['word'];b=S.Matrix(record['beta0']);piv=record['pivot_positions'];spectators=[i for i in range(len(word)) if i not in piv]
 B=AN[word,:];BW=WN[word,:];C=B.T.col_join(S.ones(1,len(word)));DC=BW.T.col_join(S.zeros(1,len(word)));inv=C[:,piv].inv()
 assert min(b)>0 and C*b==S.Matrix([0,0,0,0,1])
 assert [word[i] for i in piv]==record['pivot_labels'] and C[:,piv].det()==S.Rational(record['pivot_minor_z0'])!=0
 db=S.zeros(len(word),1);ddb=db.copy()
 if slopes is not None:
  for i in spectators:db[i]=S.Rational(slopes[i])
 rhs=-DC*b-C[:,spectators]*db[spectators,0]
 for i,z in zip(piv,inv*rhs):db[i]=z
 for i,z in zip(piv,-2*inv*DC*db):ddb[i]=z
 assert C*db+DC*b==S.zeros(5,1) and C*ddb+2*DC*db==S.zeros(5,1)
 assert list(db)==list(S.Matrix(record['beta_z'])) and list(ddb)==list(S.Matrix(record['beta_zz']))
 E=[b[i]*B[i,:] for i in range(len(word))];DE=[db[i]*B[i,:]+b[i]*BW[i,:] for i in range(len(word))];DDE=[ddb[i]*B[i,:]+2*db[i]*BW[i,:] for i in range(len(word))]
 q0=sum(om(E[i],E[j]) for i,j in itertools.combinations(range(len(word)),2))
 q1=sum(om(DE[i],E[j])+om(E[i],DE[j]) for i,j in itertools.combinations(range(len(word)),2))
 q2=sum(om(DDE[i],E[j])+2*om(DE[i],DE[j])+om(E[i],DDE[j]) for i,j in itertools.combinations(range(len(word)),2))
 assert q0==S.Rational(1,2) and q1==0
 # U=(8VQ²)^(-1), U(0)=1, V'=Q'=0: U''=-V''/V−2Q''/Q.
 curvature=-volume[2]/volume[0]-2*q2/q0
 assert curvature==S.Rational(record['U_zz'])
 return curvature
accepted=[];fixed=[]
for i,record in enumerate(packet['old24_accepted_germs']):
 assert record['word']==old['sections'][i]['sigma'] and record['pivot_positions']==old['sections'][i]['pivot']
 assert S.Matrix(record['beta0'])==S.Matrix(old['sections'][i]['beta'])
 accepted.append(check_germ(record,historical['first_weight_jets'][i]))
for record in packet['old24_fixed_spectator_germs']:fixed.append(check_germ(record))
assert accepted==[-S.Rational(13,432)]*24
assert fixed[7]==S.Rational(67,216) and all(fixed[i]==-S.Rational(13,432) for i in range(24) if i!=7)
K=0;R=0;G=S.zeros(40,1);mass=0
for point,record in zip(locked['primal']['participants'],packet['participants']):
 coefficient=S.Rational(point['lambda']);assert coefficient==S.Rational(record['lambda'])>0
 curvature=check_germ(record);K+=coefficient*curvature;mass+=coefficient;R+=coefficient*S.Rational(point['ell']);G+=coefficient*S.Matrix(point['G'])
 assert point['kind']==record['kind']
 if point['kind']=='old':
  source=old['sections'][point['index']]
  assert record['word']==source['sigma'] and record['pivot_positions']==source['pivot'] and S.Matrix(record['beta0'])==S.Matrix(source['beta'])
 else:
  schema=next(s for s in locked['formulas']['schemas'] if s['id']==point['schema']);r=S.symbols('r');slot=point['slot'];delta=S.Rational(point['delta'])
  subs={r:S.Rational(point['r'])} if point['r'] is not None else {}
  beta=[S.sympify(z,locals={'r':r}).subs(subs) for z in schema['beta']];beta[0]-=delta;beta.insert(slot,delta)
  assert record['word']==schema['placements'][slot]['word'] and S.Matrix(record['beta0'])==S.Matrix(beta)
  assert record['pivot_positions']==[i+(1 if i>=slot else 0) for i in schema['closure_pivot_positions']]
assert mass==1 and G==S.zeros(40,1)
assert K==S.Rational(packet['K']) and K<S.Rational(13,1296)
oldlambda=S.Matrix(old['weights']);assert min(oldlambda)>0 and sum(oldlambda)==1
assert S.Matrix(old['gradient_rows']).T*oldlambda==S.zeros(40,1)
mixedK=S.Rational(3,4)*K-S.Rational(13,1728);margin=S.Rational(3,4)*R-S.Rational(625,497664)
assert mixedK==S.Rational(packet['mixed_curvature'])<0 and margin==S.Rational(packet['mixed_margin'])>0
print('PASS source-bound W, independent physical V0/Vz/Vzz, all24 accepted germs and distinct fixed-spectator section7.')
print('PASS all21 actual fixed-spectator βz/βzz, positive regular sections and SECOND derivatives.')
print('K:',K,'; mixed Fzz:',mixedK,'; positive mixed response margin:',margin)
print('curvature packet SHA256:',hashlib.sha256(raw).hexdigest())
