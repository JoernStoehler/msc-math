# /// script
# dependencies = ["sympy"]
# ///
import sys,json,itertools,time,hashlib
from pathlib import Path
import sympy as S
repo=next(p for p in Path(__file__).resolve().parents if (p/'formal').is_dir() and (p/'experiments').is_dir());home=repo/'topics/empirical-studies/discovery-design/night-20261005/extremality-discovery/pstar-f11/general-facet-response';out=home/'discriminator';start=time.monotonic()
certificate=json.loads((out/'certificate.json').read_text())
for path,snapshot in certificate['sources'].items():
 assert hashlib.sha256((repo/path).read_bytes()).hexdigest()==snapshot['sha256']
accepted=certificate['accepted_candidate'];assert hashlib.sha256((repo/accepted['path']).read_bytes()).hexdigest()==accepted['sha256']
assert json.loads((repo/accepted['path']).read_text())==certificate['primal']
sys.path.insert(0,str(repo/'topics/variation-and-local-maxima/hko-local-maximum/non-hko/code'))
from family_exact import family,O,word_data,gradient
v,s,r,delta=S.symbols('v s r delta');a,b,c,d=S.symbols('a b c d');loc={'v':v,'s':s,'r':r};base={v:S.Rational(1,2),s:S.Rational(1,12)};A,vg,_,_=family(base[v],base[s]);chosen=S.Matrix([[1,-1,0,1]]);generic=S.Matrix([[a,b,c,d]])
census=json.loads((home/'preflight/contact-census.json').read_text());oldcert=json.loads((repo/'topics/variation-and-local-maxima/hko-local-maximum/non-hko/certificate/certificate_base.json').read_text());schemas=[];touch=[];nontouch=[]
for row in census['contacts']:
 if not(row['facet0_positive_on_domain'] and row['closure_rank']==5):continue
 word=row['word'];beta=S.Matrix([S.factor(S.sympify(z,locals=loc).subs(base)) for z in row['beta']]);param=any(r in z.free_symbols for z in beta);R=min(S.solve(z,r)[0] for z in beta if r in z.free_symbols and S.diff(z,r)<0) if param else None
 piv=row['closure_pivot_positions'];C=A[word,:].T.col_join(S.ones(1,len(word)));inv=C[:,piv].inv();schema={'id':row['id'],'word':word,'beta':[str(z) for z in beta],'r_upper':str(R) if R else None,'domain':f'0<r<{R}; 0<delta<r' if param else f'0<delta<{beta[0]}','closure_minor':str(C[:,piv].det()),'source_contact':row['id'],'closure_pivot_positions':piv,'closure_pivot_labels':row['closure_pivot_facet_labels'],'placements':[]}
 for slot in range(len(word)):
  ww=word[:slot]+[10]+word[slot:];norm=A.col_join(A[0,:]);bb=list(beta);bb[0]-=delta;bb.insert(slot,delta);bb=S.Matrix(bb);B,CC,H,MM,rhs=word_data(norm,ww)
  assert all(S.expand(z)==0 for z in CC*bb-S.Matrix([0,0,0,0,1]));Q=S.factor((bb.T*H*bb)[0]/2);deficit=S.factor(S.Rational(1,2)-Q)
  # Unique multiplier: normal rank4 persists despite duplicate.
  nr=list(B.T.to_DM().rref()[1]);mu=S.simplify(B[nr,:].inv()*(H*bb-S.ones(len(ww),1))[nr,0]);res=(H*bb-S.ones(len(ww),1)-B*mu).applyfunc(S.factor);stationary=all(z==0 for z in res);touching=deficit==0
  rec={'slot':slot,'word':ww,'Q_deficit':str(deficit),'touching_full_positive_domain':touching,'stationary_full_positive_domain':stationary,'stationarity_residual':[str(z) for z in res],'Q_degree_r_delta':S.Poly(deficit,r,delta).total_degree()}
  if touching:
   assert stationary
   # Full merged gradient, with the duplicate old0 variation shared.
   gg=S.zeros(10,4)
   for i,lab in enumerate(ww):
    vec=sum((bb[j]*B[j,:].T*(1 if j>i else -1 if j<i else 0) for j in range(len(ww))),S.zeros(4,1));gg[0 if lab==10 else lab,:]+=(bb[i]*(O*vec-mu)).T
   G=(-4*gg-2*vg).reshape(1,40).applyfunc(S.factor)
   ni=slot;vec=sum((bb[j]*B[j,:].T*(1 if j>ni else -1 if j<ni else 0) for j in range(len(ww))),S.zeros(4,1));k=O*vec-mu;ellgen=S.factor(2*delta*(k.T*generic.T)[0]);ell=S.factor(ellgen.subs({a:1,b:-1,c:0,d:1}));dg=inv*(-delta*generic.T.col_join(S.zeros(1,1)));jet=S.zeros(len(ww),1)
   for oldpos,z in zip(piv,dg):jet[oldpos+(1 if oldpos>=slot else 0)]=z
   # Direct pair differentiation includes the normal jet of new10.
   def om(x,y):return (x*O*y.T)[0]
   edges=[bb[i]*B[i,:] for i in range(len(ww))];jets=[jet[i]*B[i,:]+(delta*generic if i==slot else S.zeros(1,4)) for i in range(len(ww))];DQ=sum((om(jets[i],edges[j])+om(edges[i],jets[j]) for i,j in itertools.combinations(range(len(ww)),2)),S.Integer(0));assert S.expand(2*DQ-ellgen)==0
   rec.update({'mu':[str(z) for z in mu],'merged_gradient':[str(z) for z in G],'generic_ell':str(ellgen),'ell_D':str(ell),'ell_degree_r_delta':S.Poly(ell,r,delta).total_degree(),'gradient_degree_r_delta':max(S.Poly(z,r,delta).total_degree() for z in G)})
   touch.append((row['id'],slot,ell,G))
  else:nontouch.append((row['id'],slot,deficit))
  schema['placements'].append(rec)
 schemas.append(schema)

res={'status':'exact source-reconstructed seven-schema/all-slot formulas','D':['1','-1','0','1'],'L0':'625/497664','schemas':schemas,'old_zero_loss_gradients':oldcert['gradient_rows'],'counts':{'schemas':len(schemas),'all_cyclic_slots':len(touch)+len(nontouch),'touching_slots':len(touch),'nontouching_slots':len(nontouch)},'elapsed_seconds':time.monotonic()-start}
expected=certificate['formulas'].copy();expected.pop('elapsed_seconds');res.pop('elapsed_seconds');assert res==expected
# Strict nontouching classification on every positive domain; closures may
# have zero gap, but are not claimed to be regular positive contacts.
for schema in schemas:
 R=S.Rational(schema['r_upper']) if schema['r_upper'] else None
 for row in schema['placements']:
  if row['touching_full_positive_domain']:continue
  gap=S.sympify(row['Q_deficit'],locals={'r':r,'delta':delta})
  slope=S.cancel(gap/delta);assert delta not in slope.free_symbols
  if R is not None:
   assert S.Poly(slope,r).degree()<=1
   endpoints=[slope.subs(r,0),slope.subs(r,R)]
   assert min(endpoints)>=0 and max(endpoints)>0
  else:assert slope>0
# Reconstruct the exact specific old24 static sections, including their
# source beta/pivot choices, stationary multipliers, full gradients and
# positive null cancellation. No old policies are silently added.
oldG=[]
for section in oldcert['sections']:
 word=section['sigma'];beta=S.Matrix(section['beta']);I=section['pivot']
 B,C,H,M,rhs=word_data(A,word)
 assert min(beta)>0 and C*beta==S.Matrix([0,0,0,0,1])
 mu5=C[:,I].T.inv()*(H*beta)[I,0]
 assert mu5[4]==1 and M*beta.col_join(mu5[:4,0])==rhs
 oldG.append(gradient(A,vg,word,beta,mu5[:4,0]))
G_old=S.Matrix.vstack(*oldG)
assert G_old==S.Matrix(oldcert['gradient_rows'])==S.Matrix(res['old_zero_loss_gradients'])
oldlambda=S.Matrix(oldcert['weights']);assert min(oldlambda)>0 and sum(oldlambda)==1
assert G_old.T*oldlambda==S.zeros(40,1) and G_old.to_DM().rank()==22
# Full kernel check for all18 touching placements, including nonadjacent ones.
from family_exact import sym_columns
vv,ss=S.symbols('v s');AF,_,_,_=family(vv,ss)
endpoint=json.loads((home.parent/'endpoint-moving-normal/certificate.json').read_text())
WF=S.Matrix([S.sympify(z,locals={'v':vv,'s':ss}) for z in endpoint['W']])
K=sym_columns(AF).row_join(AF.diff(vv).reshape(40,1)).row_join(AF.diff(ss).reshape(40,1)).row_join(WF).subs({vv:S.Rational(1,2),ss:S.Rational(1,12)})
assert K.to_DM().rank()==18
for schema in schemas:
 for row in schema['placements']:
  if row['touching_full_positive_domain']:
   G=S.Matrix([[S.sympify(z,locals={'r':r,'delta':delta}) for z in row['merged_gradient']]])
   assert all(S.expand(z)==0 for z in G*K)
# Finite strictly positive certificate: derive participant G/ell from the
# reconstructed formulas, independently of participant G/ell declarations.
lookup={schema['id']:schema for schema in schemas};balance=S.zeros(40,1);mass=0;response=0
for participant in certificate['primal']['participants']:
 lam=S.Rational(participant['lambda']);assert lam>0;mass+=lam
 if participant['kind']=='old':
  GG=list(G_old[participant['index'],:]);ell=S.Integer(0)
 else:
  schema=lookup[participant['schema']];row=schema['placements'][participant['slot']]
  assert row['touching_full_positive_domain'];dd=S.Rational(participant['delta']);sub={delta:dd}
  if schema['r_upper']:
   rr=S.Rational(participant['r']);assert 0<rr<S.Rational(schema['r_upper']);sub[r]=rr
  beta=S.Matrix([S.sympify(z,locals={'r':r}).subs(sub) for z in schema['beta']])
  assert min(beta)>0 and 0<dd<beta[0]
  beta[0]-=dd;split=list(beta);split.insert(row['slot'],dd);assert min(split)>0
  GG=[S.sympify(z,locals={'r':r,'delta':delta}).subs(sub) for z in row['merged_gradient']]
  ell=S.sympify(row['ell_D'],locals={'r':r,'delta':delta}).subs(sub)
 assert GG==[S.Rational(z) for z in participant['G']] and ell==S.Rational(participant['ell'])
 balance+=lam*S.Matrix(GG);response+=lam*ell
assert mass==1 and balance==S.zeros(40,1)
assert response==S.Rational(certificate['primal']['response'])
# Physical cap integral for the selected D, without sign-cell enumeration.
q,t=S.symbols('q t');aa=S.Rational(5,12)
L=S.integrate(S.integrate((q-t)*2*t,(t,0,q)),(q,0,aa))/2
fraction=S.integrate(q*q,(q,0,aa))/aa
assert L==S.Rational(625,497664)==S.Rational(certificate['L0'])
assert fraction==S.Rational(25,432)==S.Rational(certificate['facet0_positive_fraction'])
assert response-L==S.Rational(certificate['primal']['margin'])>0
print('PASS: all7 complete domains/all48 slots;18 touching,30 strict nontouching; full18 kernel and old rank22.')
print('PASS: source-bound static old24 recomputed;21 strictly positive participant weights, full40 gradient balance, normalization1.')
print('PASS: physical cap L=625/497664; exact weighted response',float(response),'and strict margin',float(response-L))
print('Read-only replay seconds:',time.monotonic()-start)
