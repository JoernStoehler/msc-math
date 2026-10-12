# /// script
# dependencies = ["sympy"]
# ///
import sys,json,time,hashlib,itertools
from pathlib import Path
from zipfile import ZipFile
from io import BytesIO
import sympy as S
repo=next(p for p in Path(__file__).resolve().parents if (p/'formal').is_dir() and (p/'experiments').is_dir());home=repo/'docs/empirical-viterbo-design/night-20261005/extremality-discovery/pstar-f11/general-facet-response';dest=home/'weighted-w-gate';start=time.monotonic();z,r,delta=S.symbols('z r delta');cert=json.loads((home/'discriminator/certificate.json').read_text());oldpath=repo/'experiments/hko-local-maximum/non-hko/certificate/certificate_base.json';old=json.loads(oldpath.read_text());archive=repo/'docs/pro-handoffs/msc-math-consolidation-20260924/msc_math_consolidation_codex.zip';member='incoming/non_hko_f10_certified_result.zip';innerpath='evidence/exact/certificate_base_exact.json'
with ZipFile(archive) as outer:raw=outer.read(member)
with ZipFile(BytesIO(raw)) as inner:jetraw=inner.read(innerpath);source=json.loads(jetraw)
sys.path.insert(0,str(repo/'experiments/hko-local-maximum/non-hko/code'));from family_exact import O
A0=S.Matrix(old['dual_vertices']);W=S.Matrix(source['flat_vector']).reshape(10,4);A=A0+z*W;V=S.Rational(1,2)+S.Rational(13,1728)*z*z
# Direct signed-determinant physical volume source, along the exact same W path.
X=S.Matrix([(A[ids,:].inv()*S.ones(4,1)).T.tolist()[0] for ids in source['incidence']]);Vdirect=S.cancel(sum(sign*X[tet,:].det()/24 for tet,sign in source['pulling_tetrahedra']));assert S.cancel(Vdirect-V)==0

def section(word,b0,I,spectator_slope=None,new=False):
 B=(A.col_join(A[0,:]) if new else A)[word,:];C=B.T.col_join(S.ones(1,len(word)));J=[i for i in range(len(word)) if i not in I];beta=b0.copy();d=S.zeros(len(word),1) if spectator_slope is None else spectator_slope
 for j in J:beta[j]=b0[j]+z*d[j]
 solve=C[:,I].inv()*(S.Matrix([0,0,0,0,1])-C[:,J]*beta[J,0])
 for k,i in enumerate(I):beta[i]=S.cancel(solve[k])
 assert beta.subs(z,0)==b0 and min(b0)>0
 assert all(S.cancel(x)==0 for x in C*beta-S.Matrix([0,0,0,0,1]))
 B0=B.subs(z,0);H0=S.zeros(len(word));
 for i,j in itertools.combinations(range(len(word)),2):H0[i,j]=H0[j,i]=(B0[i,:]*O*B0[j,:].T)[0]
 nr=list(B0.T.to_DM().rref()[1]);mu=B0[nr,:].inv()*(H0*b0-S.ones(len(word),1))[nr,0]
 assert H0*b0-S.ones(len(word),1)==B0*mu
 G=S.zeros(10,4)
 for i,lab in enumerate(word):
  vec=sum((b0[j]*B0[j,:].T*(1 if j>i else -1 if j<i else 0) for j in range(len(word))),S.zeros(4,1));G[0 if lab==10 else lab,:]+=(b0[i]*(O*vec-mu)).T
 G=(-4*G-2*S.Matrix(old['volume_gradient'])).reshape(1,40)
 Q=S.cancel(sum(beta[i]*beta[j]*(B[i,:]*O*B[j,:].T)[0] for i,j in itertools.combinations(range(len(word)),2)));U=S.cancel(1/(8*V*Q*Q));d1=S.diff(U,z).subs(z,0);d2=S.diff(U,z,2).subs(z,0);assert Q.subs(z,0)==S.Rational(1,2)
 return {'word':word,'pivot_positions':I,'pivot_labels':[word[i] for i in I],'spectator_positions':J,'spectator_slopes':[str(d[j]) for j in J],'pivot_minor_z0':str(C[:,I].subs(z,0).det()),'beta0':[str(x) for x in b0],'beta_of_z':[str(x) for x in beta],'beta_z':[str(S.diff(x,z).subs(z,0)) for x in beta],'beta_zz':[str(S.diff(x,z,2).subs(z,0)) for x in beta],'Q_of_z':str(Q),'U_of_z':str(U),'U_z':str(d1),'U_zz':str(d2),'base_merged_gradient':[str(x) for x in G]}
oldaccepted=[];oldfixed=[]
for index,w in enumerate(old['sections']):
 b0=S.Matrix(w['beta']);I=w['pivot'];fixed=section(w['sigma'],b0,I);accepted=section(w['sigma'],b0,I,S.Matrix(source['first_weight_jets'][index]));assert S.Rational(accepted['U_zz'])==-S.Rational(13,432) and accepted['Q_of_z']=='1/2';oldaccepted.append(accepted);oldfixed.append(fixed)

lookup={s['id']:s for s in cert['formulas']['schemas']};participants=[];K=0
for p in cert['primal']['participants']:
 if p['kind']=='old':data=oldfixed[p['index']].copy()
 else:
  schema=lookup[p['schema']];slot=p['slot'];sub={delta:S.Rational(p['delta'])}
  if p['r'] is not None:sub[r]=S.Rational(p['r'])
  b0=S.Matrix([S.sympify(x,locals={'r':r}).subs(sub) for x in schema['beta']]);b0[0]-=sub[delta];bb=list(b0);bb.insert(slot,sub[delta]);word=schema['placements'][slot]['word'];I=[i+(1 if i>=slot else 0) for i in schema['closure_pivot_positions']];data=section(word,S.Matrix(bb),I,new=True)
 assert data['base_merged_gradient']==p['G'];data.update({key:value for key,value in p.items() if key not in ['G','ell']});data['ell']=p['ell'];participants.append(data);K+=S.Rational(p['lambda'])*S.Rational(data['U_zz'])
balance=sum((S.Rational(p['lambda'])*S.Matrix(p['base_merged_gradient']) for p in participants),S.zeros(40,1));assert balance==S.zeros(40,1)
oldbalance=sum((S.Rational(weight)*S.Matrix(data['base_merged_gradient']) for weight,data in zip(old['weights'],oldaccepted)),S.zeros(40,1));assert oldbalance==S.zeros(40,1) and sum(S.Rational(x) for x in old['weights'])==1
assert min(S.Rational(x) for x in old['weights'])>0
assert S.Rational(3,4)*balance+S.Rational(1,4)*oldbalance==S.zeros(40,1)
R=S.Rational(cert['primal']['response']);L=S.Rational(cert['L0']);mix_response=S.Rational(3,4)*R;mix_margin=mix_response-L;mix_curvature=S.Rational(3,4)*K-S.Rational(13,1728)
result={'status':'exact fixed-spectator locked21 curvature with separately bound inherited old24 affine-spectator germ; independent review pending','sources':{'locked_certificate':{'path':str((home/'discriminator/certificate.json').relative_to(repo)),'sha256':hashlib.sha256((home/'discriminator/certificate.json').read_bytes()).hexdigest()},'old_certificate':{'path':str(oldpath.relative_to(repo)),'sha256':hashlib.sha256(oldpath.read_bytes()).hexdigest()},'old_jet_archive':{'path':str(archive.relative_to(repo)),'sha256':hashlib.sha256(archive.read_bytes()).hexdigest(),'nested_member':member,'inner_member':innerpath,'member_sha256':hashlib.sha256(jetraw).hexdigest()}},'W':[str(x) for x in W.reshape(40,1)],'physical_volume':str(Vdirect),'score':'U=1/(8 V Q²), mass1, c_upper=1/(2Q)','derivative_convention':'U_zz is second derivative, not Taylor z² coefficient','old24_accepted_germs':oldaccepted,'old24_fixed_spectator_germs':oldfixed,'participants':participants,'K':str(K),'threshold':'13/1296','threshold_pass':bool(K<S.Rational(13,1296)),'mixed_response':str(mix_response),'mixed_margin':str(mix_margin),'mixed_curvature':str(mix_curvature),'mixed_curvature_negative':bool(mix_curvature<0),'seconds':time.monotonic()-start}
retained=json.loads((dest/'curvature.json').read_text());retained.pop('seconds');result.pop('seconds');assert result==retained
assert mix_margin>0 and K<S.Rational(13,1296) and mix_curvature<0
print('PASS exact locked21fixed-spectator K=',K)
print('PASS inheritedold24 affine-spectator germs -13/432; old7fixed exception67/216 explicitly retained.')
print('PASS3/4 mixture full40 balance, positive response margin',float(mix_margin),'negative second derivative',float(mix_curvature))
print('Read-only replay seconds:',time.monotonic()-start)
