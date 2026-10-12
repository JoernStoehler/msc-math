# /// script
# dependencies = ["sympy"]
# ///
import json,sys,itertools,hashlib
from pathlib import Path
import sympy as S
here=Path(__file__).resolve().parent
repo=next(p for p in here.parents if (p/'formal').is_dir() and (p/'experiments').is_dir())
root=here.parent.parent
sys.path.insert(0,str(repo/'experiments/hko-local-maximum/non-hko/code'))
from family_exact import family
v,s,t=S.symbols('v s t');base={v:S.Rational(1,2),s:S.Rational(1,12)}
A,vg,_,_=family(S.Rational(1,2),S.Rational(1,12))
rows=json.loads((root/'vertex-cones/family-witnesses.json').read_text())['witnesses']
old=json.loads((root/'vertex-cones/witnesses.json').read_text())['witnesses']
def om(x,y):return x[0]*y[2]+x[1]*y[3]-x[2]*y[0]-x[3]*y[1]
W=S.Matrix([[0,0,0,0]]*6+[[S.Rational(1,2),S.Rational(1,3),0,0],[S.Rational(3,2),-1,0,0],[1,0,0,0],[0,0,0,0]]).reshape(40,1)
out=[]
for fr,br in zip(rows,old):
 assert fr['vertex']==br['vertex'] and fr['word']==br['word']
 if 0 not in br['facets']:continue
 slot=br['facets'].index(0);word=fr['word'];bet=[S.sympify(z,locals={'v':v,'s':s}).subs(base) for z in fr['base_beta']]
 for i,coeff in zip(fr['old_positions'],fr['d_coefficient_rows']):bet[i]+=t*S.sympify(coeff[slot],locals={'v':v,'s':s}).subs(base)
 bet[word.index(10)]+=t
 pairs=[(0 if lab==10 else lab,S.expand(b)) for lab,b in zip(word,bet) if b!=0]
 # Merge only consecutive equal labels, including wrap-around.
 while any(pairs[i][0]==pairs[(i+1)%len(pairs)][0] for i in range(len(pairs))):
  i=next(i for i in range(len(pairs)) if pairs[i][0]==pairs[(i+1)%len(pairs)][0]);j=(i+1)%len(pairs)
  if j==0:pairs[0]=(pairs[0][0],S.expand(pairs[0][1]+pairs[i][1]));pairs.pop(i)
  else:pairs[i]=(pairs[i][0],S.expand(pairs[i][1]+pairs[j][1]));pairs.pop(j)
 assert len({a for a,b in pairs})==len(pairs)
 k=min(range(len(pairs)),key=lambda k:pairs[k][0]);pairs=pairs[k:]+pairs[:k]
 sig=[a for a,b in pairs];b=S.Matrix([b for a,b in pairs]);C=A[sig,:].T.col_join(S.ones(1,len(sig)))
 assert C*b==S.Matrix([0,0,0,0,1])
 Q=S.expand(sum(b[i]*b[j]*om(A[sig[i],:],A[sig[j],:]) for i,j in itertools.combinations(range(len(sig)),2)))
 assert Q==S.Rational(1,2)
 lower=S.Rational(0);upper=S.oo
 for z in b:
  z0=z.subs(t,0);slope=z.diff(t)
  if slope<0:upper=min(upper,-z0/slope)
  elif slope>0:lower=max(lower,-z0/slope)
  else:assert z0>0
 piv=list(C.to_DM().rref()[1]);
 if len(piv)!=5:
  out.append({'vertex':br['vertex'],'word':sig,'beta':list(map(str,b)),'nonconstant':any(t in z.free_symbols for z in b),'positive_interval':(str(lower),str(upper)),'closure_rank':len(piv),'minor':None,'corrected_DQ_W':'not a regular full-chart section'})
  continue
 H=S.zeros(len(sig))
 for i,j in itertools.combinations(range(len(sig)),2):H[i,j]=H[j,i]=om(A[sig[i],:],A[sig[j],:])
 mu=C[:,piv].T.inv()*(H*b)[piv,0]
 assert H*b-C.T*mu==S.zeros(len(sig),1)
 dqW=0
 for i,j in itertools.combinations(range(len(sig)),2):
  dqW+=b[i]*b[j]*(om(W[4*sig[i]:4*sig[i]+4,0],A[sig[j],:])+om(A[sig[i],:],W[4*sig[j]:4*sig[j]+4,0]))
 # Stationary weight correction has no score contribution along closure-preserving motion except multiplier closure.
 closure_W=sum((b[i]*W[4*lab:4*lab+4,0] for i,lab in enumerate(sig)),S.zeros(4,1))
 corrected=S.expand(dqW-(mu[:4,0].T*closure_W)[0])
 out.append({'vertex':br['vertex'],'word':sig,'beta':list(map(str,b)),'nonconstant':any(t in z.free_symbols for z in b),'positive_interval':(str(lower),str(upper)),'closure_rank':len(piv),'minor':str(C[:,piv].det()),'corrected_DQ_W':str(corrected)})
print(json.dumps(out,indent=2));print('Count:',len(out),'nonconstant:',sum(z['nonconstant'] for z in out))
census=json.loads((here/'contact-census.json').read_text())
for path,record in census['sources'].items():
 source_path=(repo/path) if path.startswith('experiments/') else (root/path)
 raw=source_path.read_bytes();assert len(raw)==record['bytes'];assert hashlib.sha256(raw).hexdigest()==record['sha256']
assert len(census['contacts'])==17
assert sum(z['facet0_positive_on_domain'] for z in census['contacts'])==11
assert sum(z['closure_rank']==5 for z in census['contacts'])==9
assert sum(z['closure_rank']==5 and z['facet0_positive_on_domain'] for z in census['contacts'])==7
for contact in census['contacts']:
 sig=contact['word'];rr=S.symbols('r');bet=S.Matrix([S.sympify(z,locals={'v':v,'s':s,'r':rr}).subs(base) for z in contact['beta']])
 C=A[sig,:].T.col_join(S.ones(1,len(sig)))
 assert C.to_DM().rank()==contact['closure_rank']
 assert all(S.expand(z)==0 for z in C*bet-S.Matrix([0,0,0,0,1]))
 assert S.expand(sum(bet[i]*bet[j]*om(A[sig[i],:],A[sig[j],:]) for i,j in itertools.combinations(range(len(sig)),2)))==S.Rational(1,2)
 if contact['closure_rank']==5:
  assert C[:,contact['closure_pivot_positions']].det()==S.Rational(contact['closure_minor_base'])!=0
 for alias in contact['aliases']:
  if alias.get('source_vertex') is not None:
   row=next(z for z in out if z['vertex']==alias['source_vertex'])
   assert row['word']==sig
   assert all(S.expand(S.sympify(z,locals={'t':t})-b.subs(rr,t))==0 for z,b in zip(row['beta'],bet))
print('PASS source hashes, all17 base closure/action/ranks/minors, and all8 independent facet-limit aliases.')
from family_exact import sym_columns
rr=S.symbols('r');F=S.QQ.frac_field(v,s,rr);Af,Vf,_,_=family(v,s)
endpoint=json.loads((root/'endpoint-moving-normal/certificate.json').read_text())
Wf=S.Matrix([S.sympify(z,locals={'v':v,'s':s,'r':rr}) for z in endpoint['W']])
kernel=sym_columns(Af).row_join(Af.diff(v).reshape(40,1)).row_join(Af.diff(s).reshape(40,1)).row_join(Wf)
KF=[[F.from_sympy(kernel[i,j]) for i in range(40)] for j in range(18)]
NN=[[F.from_sympy(Af[i,j]) for j in range(4)] for i in range(10)]
for contact in census['contacts']:
 if contact['closure_rank']!=5:continue
 sig=contact['word'];bb=[F.from_sympy(S.sympify(z,locals={'v':v,'s':s,'r':rr})) for z in contact['beta']]
 mu=[F.from_sympy(S.sympify(z,locals={'v':v,'s':s,'r':rr})) for z in contact['mu']]
 assert sum(bb,F.zero)==F.one
 assert all(sum((bb[i]*NN[lab][k] for i,lab in enumerate(sig)),F.zero)==F.zero for k in range(4))
 assert sum((bb[i]*bb[j]*om(NN[sig[i]],NN[sig[j]]) for i,j in itertools.combinations(range(len(sig)),2)),F.zero)==F.from_sympy(S.Rational(1,2))
 for i,lab in enumerate(sig):
  hbeta=sum((bb[j]*om(NN[lab],NN[other])*(1 if j>i else -1) for j,other in enumerate(sig) if i!=j),F.zero)
  assert hbeta==F.one+sum((NN[lab][k]*mu[k] for k in range(4)),F.zero)
 gg=[-2*F.from_sympy(Vf[i,j]) for i in range(10) for j in range(4)]
 for i,lab in enumerate(sig):
  summed=[sum((bb[j]*NN[other][k]*(1 if j>i else -1 if j<i else 0) for j,other in enumerate(sig)),F.zero) for k in range(4)]
  rotated=[summed[2],summed[3],-summed[0],-summed[1]]
  for k in range(4):gg[4*lab+k]-=4*bb[i]*(rotated[k]-mu[k])
 assert gg==[F.from_sympy(S.sympify(z,locals={'v':v,'s':s,'r':rr})) for z in contact['merged_gradient']]
 assert all(sum((a*b for a,b in zip(gg,col)),F.zero)==F.zero for col in KF)
print('PASS independent QQ(v,s,r) closure/action/stationarity, full-gradient reconstruction and18 annihilators for all9 rank-five contacts.')
