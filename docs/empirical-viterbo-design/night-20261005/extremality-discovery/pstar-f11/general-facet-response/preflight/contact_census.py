# /// script
# dependencies = ["sympy"]
# ///
import sys,json,time,hashlib,itertools
from pathlib import Path
import sympy as S
from sympy.polys.matrices import DomainMatrix as DM
start=time.monotonic();repo=next(p for p in Path(__file__).resolve().parents if (p/'formal').is_dir() and (p/'experiments').is_dir());home=repo/'docs/empirical-viterbo-design/night-20261005/extremality-discovery/pstar-f11';out=home/'general-facet-response/preflight'
sys.path.insert(0,str(repo/'experiments/hko-local-maximum/non-hko/code'))
from family_exact import family,word_data,sym_columns
v,s,r=S.symbols('v s r');F=S.QQ.frac_field(v,s,r);zero=F.zero;base={v:S.Rational(1,2),s:S.Rational(1,12)}
A,vg,K,T=family(v,s);AA=[[F.from_sympy(z) for z in A[i,:]] for i in range(10)];VG=[F.from_sympy(z) for z in vg.reshape(40,1)]
sources={};
def read(p):
 z=home/p;data=z.read_bytes();sources[p]={'bytes':len(data),'sha256':hashlib.sha256(data).hexdigest()};return json.loads(data)
P=read('vertex-cones/family-witnesses.json');BP=read('vertex-cones/witnesses.json');ridge=read('cut-strata/family-ridge-margins.json');stationary=read('endpoint-moving-normal/certificate.json')
oldpath=repo/'experiments/hko-local-maximum/non-hko/certificate/certificate_base.json';olddata=oldpath.read_bytes();sources[str(oldpath.relative_to(repo))]={'bytes':len(olddata),'sha256':hashlib.sha256(olddata).hexdigest()};oldcert=json.loads(olddata);G=S.Matrix(oldcert['gradient_rows']);W=S.Matrix([S.sympify(z,locals={'v':v,'s':s,'r':r}) for z in stationary['W']]);ker=sym_columns(A).row_join(A.diff(v).reshape(40,1)).row_join(A.diff(s).reshape(40,1)).row_join(W);KK=[[F.from_sympy(ker[i,j]) for i in range(40)] for j in range(18)]

def ff(x):return F.from_sympy(S.sympify(x,locals={'v':v,'s':s,'r':r}))
def expr(x):return str(F.to_sympy(x))
def can(word,weights):
 k=min(range(len(word)),key=lambda i:word[i]);return word[k:]+word[:k],weights[k:]+weights[:k]
def reduce(word,weights):
 pairs=[(0 if i==10 else i,b) for i,b in zip(word,weights) if b!=zero]
 # Only identical consecutive labels can merge without changing Q.
 while len({i for i,b in pairs})<len(pairs):
  for k in range(len(pairs)):
   j=(k+1)%len(pairs)
   if pairs[k][0]==pairs[j][0]:
    if j==0:pairs=[(pairs[0][0],pairs[0][1]+pairs[-1][1])]+pairs[1:-1]
    else:pairs=pairs[:k]+[(pairs[k][0],pairs[k][1]+pairs[j][1])]+pairs[j+1:]
    break
  else:raise ValueError('nonadjacent repeated positive label')
 return can([i for i,b in pairs],[b for i,b in pairs])
candidates=[]
for row,brow in zip(P['witnesses'],BP['witnesses']):
 assert row['vertex']==brow['vertex']
 if 0 not in brow['facets']:continue
 j=brow['facets'].index(0);weights=[ff(z) for z in row['base_beta']];d=[zero]*len(weights);d[row['word'].index(10)]=F.one
 for pos,co in zip(row['old_positions'],row['d_coefficient_rows']):d[pos]=ff(co[j])
 assert ff(row['C_coefficients'][f'{j},{j}'])==zero
 word,bb=reduce(row['word'],[z+F.from_sympy(r)*dz for z,dz in zip(weights,d)])
 candidates.append({'source':'vertex-cones/family-witnesses.json','source_vertex':row['vertex'],'facet_limit':f't{j}=1; all other t=0; delta renamed r','word':word,'weights':bb,'domain':'v,s in a common equality-family patch; r>0; all listed reduced beta>0; degenerate endpoints retained separately'})
primitive={}
for row in P['witnesses']:
 pairs=[(i,ff(b)) for i,b in zip(row['word'],row['base_beta']) if ff(b)!=zero];word,bb=can([i for i,b in pairs],[b for i,b in pairs]);key=tuple(word);primitive.setdefault(key,{'word':word,'weights':bb,'vertices':[]});primitive[key]['vertices'].append(row['vertex'])
assert len(primitive)==12
ridgewords=set()
for row in ridge['rows']:
 pairs=[(lab,ff(beta)) for lab,beta in zip(row['word'],row['beta']) if lab!=10]
 ww,wb=can([lab for lab,z in pairs],[z for lab,z in pairs]);key=tuple(ww);assert key in primitive and wb==primitive[key]['weights'];ridgewords.add(key)
assert len(ridgewords)==12
for row in primitive.values():
 candidates.append({'source':'vertex-cones/family-witnesses.json (primitive reductions), independently retained by cut-strata/family-ridge-margins.json','source_vertices':row['vertices'],'facet_limit':'zero-weight/new-label deletion; no new direction','word':row['word'],'weights':row['weights'],'domain':'v,s in a common equality-family patch; all primitive beta>0'})
cc=stationary['contact'];word,bb=can(cc['word'],[ff(z) for z in cc['beta']]);candidates.append({'source':'endpoint-moving-normal/certificate.json','facet_limit':'retained stationary continuum (no facet limit needed)','word':word,'weights':bb,'domain':'v,s near base; 0<r<=1/1200; all beta>0; r=0 reduced limit retained'})
records=[];lookup={}
for c in candidates:
 key=(tuple(c['word']),tuple(c['weights']));alias={k:val for k,val in c.items() if k not in ['word','weights']}
 if key in lookup:records[lookup[key]]['aliases'].append(alias);continue
 lookup[key]=len(records);word=c['word'];bb=c['weights'];n=len(word);B,C,H,M,rhs=word_data(A,word);CF=DM.from_Matrix(C).convert_to(F);HF=DM.from_Matrix(H).convert_to(F);bf=DM.from_list([[z] for z in bb],F)
 assert (CF*bf-DM.from_Matrix(S.Matrix([0,0,0,0,1])).convert_to(F)).is_zero_matrix
 assert (bf.transpose()*HF*bf).to_list()[0][0]==F.one
 piv=list(C.subs(base).to_DM().rref()[1]);crow=list(C[:,piv].subs(base).T.to_DM().rref()[1]);minor=S.cancel(C[crow,piv].det());assert minor.subs(base)!=0
 # Stationarity requires Hb=1+Bmu; solve four independent normal rows.
 normalpiv=list(B.subs(base).T.to_DM().rref()[1]);normalcols=list(B[normalpiv,:].subs(base).to_DM().rref()[1]);BF=DM.from_Matrix(B).convert_to(F);target=HF*bf-DM.from_list([[F.one] for i in range(n)],F);smallmu=BF.extract(normalpiv,normalcols).inv()*target.extract(normalpiv,[0]);mvals=[[zero] for k in range(4)];
 for k,j in enumerate(normalcols):mvals[j]=smallmu.to_list()[k]
 mf=DM.from_list(mvals,F);res=BF*mf-target;ok=res.is_zero_matrix
 row={'id':f'contact-{len(records):02d}','word':word,'beta':[expr(z) for z in bb],'aliases':[alias],'facet0_positive_on_domain':0 in word,'parameter_dimensions':3 if any(r in F.to_sympy(z).free_symbols for z in bb) else 2,'domain':c['domain'],'closure_Q':'derived exact QQ(v,s,r): C beta=(0,0,0,0,1), beta^T H beta=1','closure_rank':len(piv),'closure_pivot_rows':crow,'closure_pivot_positions':piv,'closure_pivot_facet_labels':[word[i] for i in piv],'closure_minor':str(minor),'closure_minor_base':str(minor.subs(base)),'stationarity':'derived exact' if ok else 'FAILED exact residual; no stationary-gradient claim','stationarity_equations':'H beta=1+B mu','stationarity_residual':[expr(z[0]) for z in res.to_list()]}
 if ok:
  mm=[z[0] for z in mf.to_list()];gr=[-2*z for z in VG]
  for i,lab in enumerate(word):
   vv=[sum((bb[j]*AA[ll][k]*(1 if j>i else -1 if j<i else 0) for j,ll in enumerate(word)),zero) for k in range(4)];ov=[vv[2],vv[3],-vv[0],-vv[1]]
   for k in range(4):gr[4*lab+k]-=4*bb[i]*(ov[k]-mm[k])
  tests=[sum((gr[i]*col[i] for i in range(40)),zero) for col in KK]
  row['multiplier_gauge']='unique' if len(normalcols)==4 else 'free multiplier coordinates set to zero; representative only, no ambient section';row['mu']=[expr(z) for z in mm];row['merged_gradient']=[expr(z) for z in gr];row['kernel_annihilation']={'symmetry15':all(z==zero for z in tests[:15]),'equality_v_s':all(z==zero for z in tests[15:17]),'W':tests[17]==zero,'residuals':[expr(z) for z in tests]};row['old_rank22_span']='derived: all18 common annihilators vanish, inherited constant old rank22' if all(z==zero for z in tests) else 'not derived: exact nonzero common-kernel residual'
  row['ambient_feasibility']='regular full five-row closure pivot inherited by continuity' if len(piv)==5 else 'ABSENT: closure rank below5; no full old-normal continuation supplied';row['regularity_at_base_r0']=all(S.cancel(F.to_sympy(z)).as_numer_denom()[1].subs(base).subs(r,0)!=0 for z in mm+gr+bb)
 row['positivity']='explicit beta inequalities retained; base interval below is a sufficient inherited-neighborhood control, not maximal domain'
 # Whole small interval positivity for families via Bernstein, allowing r=0 limiting zero coordinates.
 rmax=S.Rational(1,1200);mins=[]
 for z in bb:
  val=F.to_sympy(z).subs(base);p=S.Poly(val,r);deg=p.degree();bc=[sum(p.nth(k)*rmax**k*S.binomial(j,k)/S.binomial(deg,k) for k in range(j+1)) for j in range(deg+1)];assert min(bc)>=0;assert val.subs(r,rmax)>0;mins.append(str(min(bc)))
 row['base_control_interval']='0<r<=1/1200 if r-dependent; v=1/2,s=1/12';row['base_bernstein_lower'] =mins
 limitpairs=[(lab,S.cancel(F.to_sympy(z).subs(r,0))) for lab,z in zip(word,bb) if S.cancel(F.to_sympy(z).subs(r,0))!=0]
 row['r0_limit']={'word':[lab for lab,z in limitpairs],'beta':[str(z) for lab,z in limitpairs],'domain':'degenerate r0 limit; positivity of original r-dependent word not asserted at r0'}
 for z in bb:
  zz=F.to_sympy(z); c0=S.cancel(zz.subs(r,0));
  if c0.subs(base)==0:
   assert c0==0 and S.diff(zz,r).subs(base).subs(r,0)>0
 row['uniform_positive_family_control']='derived via identically-zero r0 weights with positive r slope and strict remaining base interval signs; shrink common v,s patch'

 records.append(row)
# Generic facet-cap signs, retained formula only; no sign-cell enumeration.
a,b,c,d=S.symbols('a b c d');predicates=[str(S.expand(a*K[qi,0]+b/S.Integer(2)+c*T[pi,0]+d*T[pi,1])) for qi in [0,1] for pi in range(4)]
result={'status':'bounded supplied-input preflight; no response cover or completeness theorem','sources':sources,'counts':{'vertex_input_families':24,'facet0_vertex_limits':8,'primitive_words':12,'primitive_words_with_positive_facet0':sum(0 in z['word'] for z in primitive.values()),'ridge_rows':len(ridge['rows']),'ridge_distinct_primitive_words':len(ridgewords),'facet0_ridge_rows':sum(z['q_facet']==0 for z in ridge['rows']),'stationary_continua':1,'pre_dedup_candidates':len(candidates),'deduplicated_contacts':len(records),'facet0_positive_contacts':sum(z['facet0_positive_on_domain'] for z in records),'stationary_contacts':sum(z['stationarity']=='derived exact' for z in records),'fullrank5_contacts':sum(z['closure_rank']==5 for z in records),'fullrank5_facet0_contacts':sum(z['closure_rank']==5 and z['facet0_positive_on_domain'] for z in records),'rank4_contacts':sum(z['closure_rank']==4 for z in records),'rank4_facet0_contacts':sum(z['closure_rank']==4 and z['facet0_positive_on_domain'] for z in records),'full_kernel_contacts':sum(z.get('old_rank22_span','').startswith('derived') for z in records)},'contacts':records,'facet0_cap_vertex_sign_predicates':predicates,'generic_duplication_response':'requires chosen split placement and positive split weight delta; generic intra-block term includes delta*(beta_f-delta)*omega(a_f,D), omega(a0,D)=2*d; other old-weight closure terms must be retained','first_missing_input':'The seven regular facet0 contacts have no supplied general-D balanced response certificate. Rank-deficient primitive contacts separately need a specified full ambient feasible continuation/multiplier gauge before their forty-component gradients can be used in the balance. No supplied definition/certificate of direction-dependent balanced response weights satisfying sum lambda G=0 and sum lambda ell(D)>L_f(D) over all facet0 split directions. Inventory does not establish complete touching-contact coverage or a uniform cover.','absent':['complete contact coverage theorem','general D balanced response certificate','maximal positive parameter domains and overlap/degenerate transitions','full facet-cap functional sign-cell formulas'],'inherited':['F10 constant rank22/full chart/old positive null balance','stationary-continuum W curvature and endpoint balance only'],'measured_seconds':time.monotonic()-start}
retained=json.loads((out/'contact-census.json').read_text());oldseconds=retained.pop('measured_seconds');newseconds=result.pop('measured_seconds');assert result==retained;print('PASS supplied-contact census:',result['counts']);print('Read-only reproduction seconds:',newseconds,'retained first measurement:',oldseconds)
