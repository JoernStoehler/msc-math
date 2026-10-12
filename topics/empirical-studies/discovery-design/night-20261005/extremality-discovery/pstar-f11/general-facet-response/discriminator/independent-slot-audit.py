# /// script
# dependencies = ["sympy"]
# ///
"""Independent all-cyclic-slot classification at the fixed equality base."""
import json,sys,itertools
from pathlib import Path
import sympy as S
here=Path(__file__).resolve().parent
repo=next(p for p in here.parents if (p/'formal').is_dir() and (p/'experiments').is_dir())
root=here.parent.parent
sys.path.insert(0,str(repo/'topics/variation-and-local-maxima/hko-local-maximum/non-hko/code'))
from family_exact import family
A,vg,_,_=family(S.Rational(1,2),S.Rational(1,12));D=S.Matrix([[1,-1,0,1]])
v,s,r,d=S.symbols('v s r delta');base={v:S.Rational(1,2),s:S.Rational(1,12)}
bank=json.loads((root/'general-facet-response/preflight/contact-census.json').read_text())['contacts']
def om(x,y):return x[0]*y[2]+x[1]*y[3]-x[2]*y[0]-x[3]*y[1]
rows=[]
for schema in bank:
 if schema['closure_rank']!=5 or not schema['facet0_positive_on_domain']:continue
 oldword=schema['word'];bb=[S.sympify(z,locals={'v':v,'s':s,'r':r}).subs(base) for z in schema['beta']];oldpos=oldword.index(0)
 C0=A[oldword,:].T.col_join(S.ones(1,len(oldword)));piv=schema['closure_pivot_positions'];inverse=C0[:,piv].inv()
 for slot in range(len(oldword)):
  word=oldword[:slot]+[10]+oldword[slot:];b=S.Matrix(bb[:slot]+[d]+bb[slot:]);b[word.index(0)]-=d
  normals=A.col_join(A[0,:]);B=normals[word,:];C=B.T.col_join(S.ones(1,len(word)));H=S.zeros(len(word))
  for i,j in itertools.combinations(range(len(word)),2):H[i,j]=H[j,i]=om(B[i,:],B[j,:])
  Q=S.expand((b.T*H*b)[0]/2);T=S.cancel((Q-S.Rational(1,2))/d)
  assert S.Poly(T,r).degree()<=1 and d not in T.free_symbols
  record={'schema':schema['id'],'slot':slot,'word':word,'touching':T==0,'touching_defect_coefficient':str(T)}
  if T==0:
   pivsplit=list(C.to_DM().rref()[1]);mu=C[:,pivsplit].T.inv()*(H*b)[pivsplit,0]
   assert all(S.expand(z)==0 for z in H*b-C.T*mu) and mu[4]==1
   g=-2*vg.copy()
   for i,lab in enumerate(word):
    vec=sum((b[j]*B[j,:].T*(1 if j>i else -1) for j in range(len(word)) if i!=j),S.zeros(4,1))
    rotated=S.Matrix([vec[2],vec[3],-vec[0],-vec[1]])
    g[0 if lab==10 else lab,:]-=(4*b[i]*(rotated-mu[:4,0])).T
   g=[S.expand(z) for z in g.reshape(40,1)]
   corrections=S.zeros(len(word),1);solution=inverse*(-d*D.T.col_join(S.zeros(1,1)))
   for oldp,z in zip(piv,solution):corrections[oldp+(1 if oldp>=slot else 0)]=z
   edges=[b[i]*B[i,:] for i in range(len(word))]
   jets=[corrections[i]*B[i,:]+(d*D if lab==10 else S.zeros(1,4)) for i,lab in enumerate(word)]
   ell=S.expand(2*sum(om(jets[i],edges[j])+om(edges[i],jets[j]) for i,j in itertools.combinations(range(len(word)),2)))
   assert S.Poly(ell,r,d).total_degree()<=2 and all(S.Poly(z,r,d).total_degree()<=2 for z in g)
   record.update({'mu':list(map(str,mu[:4,0])),'gradient':list(map(str,g)),'ell':str(ell)})
  rows.append(record)
out={'schemas':7,'slots':len(rows),'touching_slots':sum(z['touching'] for z in rows),'rows':rows}
print('PASS independent all-slot classification:',out['slots'],'slots;',out['touching_slots'],'touching.')
for schema in sorted(set(z['schema'] for z in rows)):
 print(schema,[(z['slot'],z.get('ell',z['touching_defect_coefficient'])) for z in rows if z['schema']==schema])
from family_exact import sym_columns
AF,_,_,_=family(v,s)
endpoint=json.loads((root/'endpoint-moving-normal/certificate.json').read_text())
W=S.Matrix([S.sympify(z,locals={'v':v,'s':s,'r':r}).subs(base) for z in endpoint['W']])
kernel=sym_columns(AF).row_join(AF.diff(v).reshape(40,1)).row_join(AF.diff(s).reshape(40,1)).row_join(W).subs(base)
assert kernel.to_DM().rank()==18
for row in rows:
 if not row['touching']:continue
 G=S.Matrix([S.sympify(z,locals={'r':r,'delta':d}) for z in row['gradient']])
 assert all(S.expand(z)==0 for z in kernel.T*G)
print('PASS all18 inherited symmetry/equality/W contractions for all18 touching slot gradients.')
formulas=json.loads((here/'formulas.json').read_text())
for schema in formulas['schemas']:
 for placement in schema['placements']:
  independent=next(z for z in rows if z['schema']==schema['id'] and z['slot']==placement['slot'])
  assert independent['word']==placement['word']
  assert independent['touching']==placement['touching_full_positive_domain']
  if not independent['touching']:continue
  assert all(S.expand(S.sympify(a,locals={'r':r,'delta':d})-S.sympify(b,locals={'r':r,'delta':d}))==0 for a,b in zip(independent['gradient'],placement['merged_gradient']))
  assert S.expand(S.sympify(independent['ell'],locals={'r':r,'delta':d})-S.sympify(placement['ell_D'],locals={'r':r,'delta':d}))==0
print('PASS full gradient/response comparison against retained formulas for every touching slot.')
