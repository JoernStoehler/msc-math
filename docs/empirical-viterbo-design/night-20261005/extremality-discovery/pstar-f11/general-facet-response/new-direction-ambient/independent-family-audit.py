# /// script
# dependencies = ["sympy"]
# ///
"""Independent fixed-policy moving-family check; reads source definitions only."""
import itertools,json,sys,time
from pathlib import Path
import sympy as S
from sympy.polys.matrices import DomainMatrix as DM
here=Path(__file__).resolve().parent
repo=next(p for p in here.parents if (p/'formal').is_dir())
sys.path.insert(0,str(repo/'experiments/hko-local-maximum/non-hko/code'))
from family_exact import family,sym_columns
v,s,r=S.symbols('v s r');F=S.QQ.frac_field(v,s);zero=F.zero
A,vg,_,_=family(v,s);AN=A.col_join(A[0,:]);base={v:S.Rational(1,2),s:S.Rational(1,12)}
root=here.parent.parent
packet=json.loads((here/'family-policies.json').read_text())
bank=json.loads((here.parent/'preflight/contact-census.json').read_text())['contacts']
locked=json.loads((here.parent/'discriminator/certificate.json').read_text())
endpoint=json.loads((root/'endpoint-moving-normal/certificate.json').read_text())
W=S.Matrix([S.sympify(z,locals={'v':v,'s':s}) for z in endpoint['W']])
K=sym_columns(A).row_join(A.diff(v).reshape(40,1)).row_join(A.diff(s).reshape(40,1)).row_join(W)
def matrix(M):return DM.from_Matrix(M).convert_to(F)
def om(x,y):return x[0]*y[2]+x[1]*y[3]-x[2]*y[0]-x[3]*y[1]
for point in locked['primal']['participants']:
 if point['kind']=='old':continue
 t=time.time();schema=next(z for z in bank if z['id']==point['schema']);slot=point['slot'];delta=S.Rational(point['delta']);rr=S.Rational(point['r']) if point['r'] is not None else 0
 b=[S.sympify(z,locals={'v':v,'s':s,'r':r}).subs(r,rr) for z in schema['beta']];b[0]-=delta;b.insert(slot,delta);b=S.Matrix(b)
 word=schema['word'][:slot]+[10]+schema['word'][slot:];B=AN[word,:];C=B.T.col_join(S.ones(1,len(word)));piv=[i+(i>=slot) for i in schema['closure_pivot_positions']]
 BB=[[F.from_sympy(z) for z in B[i,:]] for i in range(len(word))];bf=[F.from_sympy(z) for z in b]
 assert matrix(C)*matrix(b)==matrix(S.Matrix([0,0,0,0,1]))
 H=S.zeros(len(word))
 for i,j in itertools.combinations(range(len(word)),2):H[i,j]=H[j,i]=F.to_sympy(om(BB[i],BB[j]))
 hf=matrix(H)*matrix(b);mu=(matrix(C[:,piv].T).inv()*hf.extract(piv,[0])).to_Matrix()
 residual=matrix(H)*matrix(b)-matrix(C.T)*matrix(mu)
 assert residual.is_zero_matrix, ('STATIONARITY FAIL',schema['id'],slot)
 q=sum((bf[i]*bf[j]*om(BB[i],BB[j]) for i,j in itertools.combinations(range(len(word)),2)),zero)
 assert q==F.from_sympy(S.Rational(1,2))
 assert mu[4]==1
 gf=[F.from_sympy(-2*z) for z in vg.reshape(40,1)];mf=[F.from_sympy(z) for z in mu[:4,0]]
 for i,lab in enumerate(word):
  vec=[sum((bf[j]*BB[j][k]*(1 if j>i else -1) for j in range(len(word)) if i!=j),zero) for k in range(4)]
  rot=[vec[2],vec[3],-vec[0],-vec[1]];lab=0 if lab==10 else lab
  for k in range(4):gf[4*lab+k]-=4*bf[i]*(rot[k]-mf[k])
 for col in range(18):
  contraction=sum((gf[i]*F.from_sympy(K[i,col]) for i in range(40)),zero)
  if contraction!=zero:
   print('FAIL',schema['id'],'slot',slot,'kernelcolumn',col,'contraction',F.to_sympy(contraction),flush=True);raise SystemExit(2)
 record=next(z for z in packet['new6_fixed_family_policies'] if z['schema']==schema['id'] and z['slot']==slot and z['delta']==point['delta'])
 assert record['word']==word and record['old_closure_pivots']==piv
 assert all(F.from_sympy(S.sympify(a,locals={'v':v,'s':s}))==F.from_sympy(z) for a,z in zip(record['beta'],b))
 assert all(F.from_sympy(S.sympify(a,locals={'v':v,'s':s}))==z for a,z in zip(record['merged_gradient'],gf))
 assert min(b.subs(base))>0
 det=matrix(C[:,piv]).det();assert F.to_sympy(det).subs(base)!=0
 print('PASS',schema['id'],'slot',slot,'r',rr,'delta',delta,'full family closure/Q/stationarity/kernel18','pivot',F.to_sympy(det).subs(base),'seconds',round(time.time()-t,2),flush=True)
print('PASS all six actual fixed policies throughout generic equality family.')
# Bind every static source row to its inherited stationary-family section.
from family_exact import continuation_spec,word_data
old=json.loads((repo/'experiments/hko-local-maximum/non-hko/certificate/certificate_base.json').read_text())
A0,_,_,_=family(S.Rational(1,2),S.Rational(1,12))
rows=[];aliases=[]
AA=[[F.from_sympy(z) for z in A[i,:]] for i in range(10)]
for index,w in enumerate(old['sections']):
 t=time.time();pp,rr,free,y0=continuation_spec(A0,w);B,C,H,M,rhs=word_data(A,w['sigma']);MF=matrix(M);RF=matrix(rhs)
 yf=[F.from_sympy(z) for z in y0]
 solution=MF.extract(rr,pp).inv()*(RF.extract(rr,[0])-MF.extract(rr,free)*DM.from_list([[yf[i]] for i in free],F))
 for i,z in zip(pp,solution.to_list()):yf[i]=z[0]
 YF=DM.from_list([[z] for z in yf],F);assert (MF*YF-RF).is_zero_matrix
 n=len(w['sigma']);bb=yf[:n];mm=yf[n:];assert (DM.from_list([bb],F)*matrix(H)*DM.from_list([[z] for z in bb],F)).to_list()[0][0]==F.one
 gf=[F.from_sympy(-2*z) for z in vg.reshape(40,1)]
 for i,lab in enumerate(w['sigma']):
  vec=[sum((bb[j]*AA[ll][k]*(1 if j>i else -1) for j,ll in enumerate(w['sigma']) if i!=j),zero) for k in range(4)];rot=[vec[2],vec[3],-vec[0],-vec[1]]
  for k in range(4):gf[4*lab+k]-=4*bb[i]*(rot[k]-mm[k])
 for col in range(18):assert sum((gf[i]*F.from_sympy(K[i,col]) for i in range(40)),zero)==zero, ('old kernel',index,col)
 bbase=[F.to_sympy(z).subs(base) for z in bb];assert bbase==list(S.Matrix(w['beta'])) and min(bbase)>0
 pivotdet=matrix(C[:,w['pivot']]).det();assert F.to_sympy(pivotdet).subs(base)!=0
 for z in yf:assert F.to_sympy(z).as_numer_denom()[1].subs(base)!=0
 record=packet['old24_family_aliases'][index]
 assert record['old_index']==index and record['word']==w['sigma'] and record['old_closure_pivots']==w['pivot']
 assert record['source_continuation_spec']['pivot_columns']==pp and record['source_continuation_spec']['pivot_rows']==rr and record['source_continuation_spec']['free_columns']==free
 assert all(F.from_sympy(S.sympify(a,locals={'v':v,'s':s}))==z for a,z in zip(record['beta']+record['mu'],yf))
 assert all(F.from_sympy(S.sympify(a,locals={'v':v,'s':s}))==z for a,z in zip(record['merged_gradient'],gf))
 rows.append(gf)
 if index in [p['index'] for p in locked['primal']['participants'] if p['kind']=='old']:aliases.append({'index':index,'stationary_pivots':pp,'stationary_equations':rr,'frozen_positions':free,'closure_pivots':w['pivot']})
 print('PASS actual old family alias',index,'stationary/free',pp,free,'movingkernel18','seconds',round(time.time()-t,2),flush=True)
print('PASS fifteen locked old participant aliases',aliases,flush=True)
# The old moving mean is an exact positive normalized cancellation mean.
weights=S.Matrix([S.Rational(z) for z in old['weights']]);assert min(weights)>0 and sum(weights)==1
print('PASS inherited strict positive base old24 masses; smooth generic cancellation uses the augmented minor below, not frozen coefficients.',flush=True)
baseG=S.Matrix([[F.to_sympy(z).subs(base) for z in row] for row in rows]);assert baseG==S.Matrix(old['gradient_rows'])
endpointbalance=endpoint['balance'];aug=baseG.T.col_join(S.ones(1,24));prows=endpointbalance['pivot_rows'];pcols=endpointbalance['pivot_columns'];det=aug[prows,pcols].det(method='domain-ge')
assert det==S.Rational(endpointbalance['pivot_determinant'])!=0
assert len(prows)==len(pcols)==23 and aug.to_DM().rank()==23
assert K.subs(base).to_DM().rank()==18
print('PASS old augmented23 rows',prows,'columns',pcols,'det',det,flush=True)
print('PASS source bound augmented solve: rank22 plus mass and common kernel18 imply full40 cancellation; positivity by the strict base coefficient margin.',flush=True)

rb=packet['rank_balance'];Y=rb['Y_coordinates'];columns=rb['augmented23_old_columns'];ap=baseG[:,Y].T.col_join(S.ones(1,24));pd=ap[:,columns].det(method='domain-ge')
assert pd==S.Rational(rb['augmented23_minor_base'])!=0
alpha=S.zeros(24,1)
for point in locked['primal']['participants']:
 if point['kind']=='old':alpha[point['index']]+=S.Rational(3,4)*S.Rational(point['lambda'])
baggregated=S.Matrix([S.Rational(z) for z in rb['base_old_mixture_weights']]);beta=baggregated-alpha
assert beta==weights/4 and min(beta)>0
assert S.Matrix(rb['new_fixed_mixture_weights']).applyfunc(S.Rational)==S.Matrix([S.Rational(3,4)*S.Rational(z['lambda']) for z in locked['primal']['participants'] if z['kind']!='old'])
print('PASS producer selected augmented23 determinant and45 score policy split: α21=.75λ fixed germs, β24=.25μ inherited germs at base; β stays positive locally after subtraction.',flush=True)
# Read-only archival source bindings and exact spectator-jet policy.
import hashlib,io,zipfile
for path,metadata in packet['sources'].items():assert hashlib.sha256((repo/path).read_bytes()).hexdigest()==metadata['sha256']
archivepath=next(path for path in packet['sources'] if path.endswith('msc_math_consolidation_codex.zip'))
with zipfile.ZipFile(repo/archivepath) as outer:inner=outer.read(packet['historical_family_member']['outer_member'])
with zipfile.ZipFile(io.BytesIO(inner)) as archive:
 fr=archive.read(packet['historical_family_member']['inner_member']);jr=archive.read(packet['historical_jet_member']['inner_member'])
assert hashlib.sha256(fr).hexdigest()==packet['historical_family_member']['sha256'] and hashlib.sha256(jr).hexdigest()==packet['historical_jet_member']['sha256']
history=json.loads(fr);jets=json.loads(jr)
assert all(F.from_sympy(S.sympify(z,locals={'v':v,'s':s}))==F.from_sympy(w) for z,w in zip(history['flat_field_vector'],W))
for index,record in enumerate(packet['old24_family_aliases']):
 assert record['source_continuation_spec']==history['continuation_specs'][index]
 assert record['accepted_old_spectator_jets']==jets['first_weight_jets'][index]
curvature=json.loads((here.parent/'weighted-w-gate/curvature.json').read_text())
for policy in packet['new6_fixed_family_policies']:
 source=next(z for z in curvature['participants'] if z.get('schema')==policy['schema'] and z.get('slot')==policy['slot'] and z.get('r')==policy['r'] and z.get('delta')==policy['delta'])
 assert policy['base_fixed_germ_beta_z']==source['beta_z'] and policy['base_fixed_germ_beta_zz']==source['beta_zz'] and policy['base_fixed_germ_U_zz']==source['U_zz']
print('PASS source/member hashes, moving W/archive stationary aliases, exact archived source-germ spectator slopes, six locked base W jets.',flush=True)
