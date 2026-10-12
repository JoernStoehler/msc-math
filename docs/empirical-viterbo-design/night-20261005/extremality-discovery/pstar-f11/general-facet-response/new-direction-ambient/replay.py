# /// script
# dependencies = ["sympy"]
# ///
import json,sys,time,hashlib
from pathlib import Path
from zipfile import ZipFile
from io import BytesIO
import sympy as S
from sympy.polys.matrices import DomainMatrix as DM
repo=next(p for p in Path(__file__).resolve().parents if (p/'formal').is_dir() and (p/'experiments').is_dir());home=repo/'docs/empirical-viterbo-design/night-20261005/extremality-discovery/pstar-f11';dest=home/'general-facet-response/new-direction-ambient';start=time.monotonic()
sys.path.insert(0,str(repo/'experiments/hko-local-maximum/non-hko/code'));from family_exact import family,word_data,sym_columns
oldpath=repo/'experiments/hko-local-maximum/non-hko/certificate/certificate_base.json';old=json.loads(oldpath.read_text());lockedpath=home/'general-facet-response/discriminator/certificate.json';locked=json.loads(lockedpath.read_text());curvpath=home/'general-facet-response/weighted-w-gate/curvature.json';curv=json.loads(curvpath.read_text());censuspath=home/'general-facet-response/preflight/contact-census.json';census=json.loads(censuspath.read_text());cby={r['id']:r for r in census['contacts']}
archive=repo/'docs/pro-handoffs/msc-math-consolidation-20260924/msc_math_consolidation_codex.zip';member='incoming/non_hko_f10_certified_result.zip';jetpath='evidence/exact/certificate_base_exact.json';familypath='evidence/exact/certificate_family_exact.json'
with ZipFile(archive) as o:raw=o.read(member)
with ZipFile(BytesIO(raw)) as i:jr=i.read(jetpath);fr=i.read(familypath);jets=json.loads(jr);source=json.loads(fr)
v,s,r,delta=S.symbols('v s r delta');F=S.QQ.frac_field(v,s);zero=F.zero;loc={'v':v,'s':s,'r':r,'delta':delta};base={v:S.Rational(1,2),s:S.Rational(1,12)};A,vg,_,_=family(v,s);A0=A.subs(base)
AA=[[F.from_sympy(z) for z in A[i,:]] for i in range(10)];VG=[F.from_sympy(z) for z in vg.reshape(40,1)];W=[F.from_sympy(S.sympify(z,locals=loc)) for z in source['flat_field_vector']];K=sym_columns(A).row_join(A.diff(v).reshape(40,1)).row_join(A.diff(s).reshape(40,1)).row_join(S.Matrix([F.to_sympy(z) for z in W]));KK=[[F.from_sympy(K[i,j]) for i in range(40)] for j in range(18)]
def parse(z):return F.from_sympy(S.sympify(z,locals=loc))
def expr(z):return str(F.to_sympy(z))
def value(z):
 zz=F.to_sympy(z);assert zz.as_numer_denom()[1].subs(base)!=0;return zz.subs(base)
def grow(word,bb,mu):
 row=[-2*z for z in VG]
 for i,lab in enumerate(word):
  vv=[sum((bb[j]*AA[0 if l==10 else l][k]*(1 if j>i else -1 if j<i else 0) for j,l in enumerate(word)),zero) for k in range(4)];ov=[vv[2],vv[3],-vv[0],-vv[1]]
  for k in range(4):row[4*(0 if lab==10 else lab)+k]-=4*bb[i]*(ov[k]-mu[k])
 return row
oldpolicies=[];oldG=[];failed=[]
for n,w in enumerate(old['sections']):
 sig=w['sigma'];sp=source['continuation_specs'][n];_,C,H,M,rhs=word_data(A,sig);MF=DM.from_Matrix(M).convert_to(F);RF=DM.from_Matrix(rhs).convert_to(F);pp=sp['pivot_columns'];rr=sp['pivot_rows'];free=sp['free_columns'];yy=[zero]*M.cols
 for k,z in zip(free,sp['free_values']):yy[k]=parse(z)
 sol=MF.extract(rr,pp).inv()*(RF.extract(rr,[0])-MF.extract(rr,free)*DM.from_list([[yy[k]] for k in free],F))
 for k,z in zip(pp,sol.to_list()):yy[k]=z[0]
 assert (MF*DM.from_list([[z] for z in yy],F)-RF).is_zero_matrix
 beta=yy[:len(sig)];mu=yy[len(sig):];assert [value(z) for z in beta]==[S.Rational(z) for z in w['beta']];assert min(value(z) for z in beta)>0
 g=grow(sig,beta,mu);tests=[sum((g[i]*col[i] for i in range(40)),zero) for col in KK];assert all(z==zero for z in tests)
 I=w['pivot'];det=DM.from_Matrix(C[:,I]).convert_to(F).det();assert value(det)!=0
 assert [str(value(z)) for z in g]==old['gradient_rows'][n]
 oldpolicies.append({'old_index':n,'word':sig,'beta':[expr(z) for z in beta],'mu':[expr(z) for z in mu],'source_continuation_spec':sp,'old_closure_pivots':I,'closure_minor':expr(det),'closure_minor_base':str(value(det)),'merged_gradient':[expr(z) for z in g],'kernel18':'PASS exact QQ(v,s)','accepted_old_spectator_jets':jets['first_weight_jets'][n]});oldG.append(g)
newpolicies=[]
for p in locked['primal']['participants']:
 if p['kind']=='old':continue
 sch=cby[p['schema']];sub={r:S.Rational(p['r'])} if p['r'] is not None else {};sig=sch['word'];bb=[F.from_sympy(S.sympify(z,locals=loc).subs(sub)) for z in sch['beta']];dd=F(S.Rational(p['delta']));bb[0]-=dd;slot=p['slot'];bb.insert(slot,dd);word=sig[:slot]+[10]+sig[slot:];norm=A.col_join(A[0,:]);B,C,H,M,rhs=word_data(norm,word);BF=DM.from_Matrix(B).convert_to(F);HF=DM.from_Matrix(H).convert_to(F);CF=DM.from_Matrix(C).convert_to(F);betaf=DM.from_list([[z] for z in bb],F);assert (CF*betaf-DM.from_Matrix(S.Matrix([0,0,0,0,1])).convert_to(F)).is_zero_matrix
 nr=list(B.subs(base).T.to_DM().rref()[1]);muF=BF.extract(nr,range(4)).inv()*(HF*betaf-DM.from_list([[F.one] for j in word],F)).extract(nr,[0]);mu=[z[0] for z in muF.to_list()];res=HF*betaf-DM.from_list([[F.one] for j in word],F)-BF*muF
 assert res.is_zero_matrix and (betaf.transpose()*HF*betaf).to_list()[0][0]==F.one
 assert min(value(z) for z in bb)>0
 g=grow(word,bb,mu);tests=[sum((g[i]*col[i] for i in range(40)),zero) for col in KK];kernel=all(z==zero for z in tests)
 I=[i+(1 if i>=slot else 0) for i in sch['closure_pivot_positions']];det=DM.from_Matrix(C[:,I]).convert_to(F).det();assert value(det)!=0
 policy={'schema':p['schema'],'slot':slot,'r':p['r'],'delta':p['delta'],'word':word,'beta':[expr(z) for z in bb],'mu':[expr(z) for z in mu],'old_closure_pivots':I,'old_closure_pivot_labels':[word[k] for k in I],'closure_minor':expr(det),'closure_minor_base':str(value(det)),'merged_gradient':[expr(z) for z in g],'kernel18_residuals':[expr(z) for z in tests],'kernel18':'PASS exact QQ(v,s)' if kernel else 'FAIL exact QQ(v,s)','base_min_beta':str(min(value(z) for z in bb)),'locked_lambda':p['lambda']}
 assert [str(value(z)) for z in g]==p['G']
 expected=next(row for row in curv['participants'] if row.get('schema')==p['schema'] and row.get('slot')==p['slot'] and row.get('r')==p['r'] and row.get('delta')==p['delta'])
 assert [str(value(z)) for z in bb]==expected['beta0'] and I==expected['pivot_positions']
 policy['base_fixed_germ_beta_z']=expected['beta_z'];policy['base_fixed_germ_beta_zz']=expected['beta_zz'];policy['base_fixed_germ_U_zz']=expected['U_zz']
 newpolicies.append(policy)
 if not kernel:failed.append({'schema':p['schema'],'slot':slot,'first_nonzero':next((j,expr(z)) for j,z in enumerate(tests) if z!=zero)});break
assert K.subs(base).to_DM().rank()==18
G0=S.Matrix(old['gradient_rows']);Y=[0,1,2,3,4,5,6,7,8,9,10,11,12,14,15,16,18,24,25,26,28,29];H0=G0[:,Y];rr=list(H0.T.to_DM().rref()[1]);det22=H0[rr,:].det(method='domain-ge');assert det22!=0;E0=H0.T.col_join(S.ones(1,24));cc=list(E0.to_DM().rref()[1]);assert len(cc)==23;det23=E0[:,cc].det(method='domain-ge');assert det23!=0
oldlambda=S.Matrix(old['weights']);assert min(oldlambda)>0 and sum(oldlambda)==1 and G0.T*oldlambda==S.zeros(40,1)
fixedold=[S.Rational(1,4)*z for z in oldlambda]
for p in locked['primal']['participants']:
 if p['kind']=='old':fixedold[p['index']]+=S.Rational(3,4)*S.Rational(p['lambda'])
alphaold=[S.Integer(0)]*24
for p in locked['primal']['participants']:
 if p['kind']=='old':alphaold[p['index']]=S.Rational(3,4)*S.Rational(p['lambda'])
betasource=[fixedold[j]-alphaold[j] for j in range(24)];assert betasource==[S.Rational(1,4)*z for z in oldlambda] and min(betasource)>0
newweights=[S.Rational(3,4)*S.Rational(p['locked_lambda']) for p in newpolicies];assert min(fixedold)>0
assert S.Rational(3,4)*S.Rational(locked['primal']['response'])-S.Rational(locked['L0'])>0
result={'status':'first exact failed family premise' if failed else 'PASS targeted fixed-policy family readiness; independent review pending','failures':failed,'sources':{str(path.relative_to(repo)):{'sha256':hashlib.sha256(path.read_bytes()).hexdigest()} for path in [oldpath,lockedpath,curvpath,censuspath,archive]},'historical_family_member':{'outer_member':member,'inner_member':familypath,'sha256':hashlib.sha256(fr).hexdigest()},'historical_jet_member':{'outer_member':member,'inner_member':jetpath,'sha256':hashlib.sha256(jr).hexdigest()},'W':[expr(z) for z in W],'old24_family_aliases':oldpolicies,'new6_fixed_family_policies':newpolicies,'rank_balance':{'Y_coordinates':Y,'rank22_rows':rr,'rank22_minor_base':str(det22),'augmented23_old_columns':cc,'augmented23_minor_base':str(det23),'held_old_column':[k for k in range(24) if k not in cc][0],'base_old_mixture_weights':[str(z) for z in fixedold],'new_fixed_mixture_weights':[str(z) for z in newweights],'old_positive_null_weights':old['weights'],'fixed_locked_old21_weights':[str(z) for z in alphaold],'actual_old24_source_weights_at_base':[str(z) for z in betasource],'source_germ_subtraction_policy':'solve aggregate b; actual SOURCE-germ coefficient beta_j=b_j-alpha_old_j. Locked21 alpha=.75lambda remains fixed. Base beta=.25mu_old>0; positivity uses beta, not aggregates.','normalization':'sum actual SOURCE weights=.25; sum locked21 alpha=.75' },'specific_old15_aliases':[p['index'] for p in locked['primal']['participants'] if p['kind']=='old'],'cap_policy':{'incident_labels':[0,1,6,7],'base_gamma':['-2/3','7/12','3/8','1/8'],'base_vertex':['5/12','1/2','0','1/2'],'base_D_vertex':'5/12','base_detN':'-256/7','L_base':'625/497664','definition':'v=N(A)^(-1)1; gamma=D N(A)^(-1); removedV=tau*(D.v)^4/[24 abs(detN) gamma1 gamma2 gamma3 (1+tau gamma0)] on the stable one-vertex cap chamber'},'seconds':time.monotonic()-start}
stored=json.loads((dest/'family-policies.json').read_text());stored.pop('seconds');result.pop('seconds');assert result==stored
assert not failed
print('PASS all24 source familyaliases, actual15old sourcebinding, all6 fixed nonadjacent policies and moving18 kernel.')
print('PASS exact rank22 and augmented23 minors; source-coefficient subtraction beta=b-alpha; all24 base beta>0.')
print('Read-only focused family replay seconds:',time.monotonic()-start)
