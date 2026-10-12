# /// script
# dependencies = ["sympy"]
# ///
"""Continue exact adjacent-word mixed-ridge sections over the P* equality family."""
import itertools,json,sys
from pathlib import Path
import sympy as S
root=Path(__file__).resolve().parent
repo=next(p for p in root.parents if(p/'formal').is_dir() and(p/'experiments').is_dir())
sys.path.insert(0,str(repo/'topics/variation-and-local-maxima/hko-local-maximum/non-hko/code'))
from family_exact import family
v,s=S.symbols('v s');A,_,K,T=family(v,s);F=S.QQ.frac_field(v,s);base={v:S.Rational(1,2),s:S.Rational(1,12)}
loc={'v':v,'s':s}
def parse(z):return F.from_sympy(S.sympify(z,locals=loc))
def expr(z):return F.to_sympy(z)
def value(z):
 e=expr(z);assert e.as_numer_denom()[1].subs(base)!=0
 return e.subs(base)
N=[[F.from_sympy(A[i,j]) for j in range(4)] for i in range(10)]
def om(a,b):return a[0]*b[2]+a[1]*b[3]-a[2]*b[0]-a[3]*b[1]
packet=json.loads((root.parent/'vertex-cones/family-witnesses.json').read_text());words={}
for row in packet['witnesses']:
 pairs=[(i,parse(b)) for i,b in zip(row['word'],row['base_beta']) if parse(b)!=F.zero]
 k=min(range(len(pairs)),key=lambda k:pairs[k][0]);pairs=pairs[k:]+pairs[:k]
 word=tuple(i for i,b in pairs);beta=dict(pairs)
 assert all(value(b)>0 for b in beta.values())
 assert sum(beta.values(),F.zero)==F.one
 assert all(sum((beta[i]*N[i][j] for i in word),F.zero)==F.zero for j in range(4))
 Q=sum((beta[i]*beta[j]*om(N[i],N[j]) for r,i in enumerate(word) for j in word[r+1:]),F.zero)
 assert Q==F(S.Rational(1,2));words[word]=beta
assert len(words)==12
R={}
for i in range(10):
 n=A[i,:2] if i<6 else A[i,2:];P=K if i<6 else T
 ends=[P[k,:] for k in range(P.rows) if S.cancel((n*P[k,:].T)[0]-1)==0]
 assert len(ends)==2;e=ends[1]-ends[0]
 if n[1].subs(base)!=0:z=F.from_sympy(e[0]/n[1])
 else:z=F.from_sympy(e[1]/n[0])
 assert value(z)!=0
 if value(z)<0:z=-z
 R[i]=z;assert value(z)>0
old=json.loads((root/'ridge-margins.json').read_text());rows=[]
for row in old['rows']:
 word=tuple(row['word']);a,b=word[0],word[2]
 primitive=tuple(i for i in word if i!=10)
 k=min(range(len(primitive)),key=lambda k:primitive[k]);can=primitive[k:]+primitive[:k];beta=words[can]
 omega=om(N[a],N[b]);assert value(omega)==S.Rational(row['omega']) and value(omega)>0
 qi,pi=row['q_facet'],row['p_facet'];product=R[qi]*R[pi]*omega
 assert value(product)==S.Rational(row['margin_product']) and 0<value(product)<=S.Rational(5,8)
 assert all(value(beta[i])>0 for i in primitive)
 # Adjacency surgery identities are universal algebraic identities in normals,
 # while these exact closure/Q predicates validate its parameter-dependent base.
 assert primitive[0]==a and primitive[1]==b
 rows.append({'q_facet':qi,'p_facet':pi,'word':word,'beta':[str(expr(beta[i])) if i!=10 else '0' for i in word],'omega':str(expr(omega)),'Rq':str(expr(R[qi])),'Rp':str(expr(R[pi])),'margin_product':str(expr(product)),'base_margin_product':str(value(product))})
out={'status':'PASS exact family mixed-ridge continuation; common parameter patch by strict finite margins','shape_parameters':'v,s near 1/2,1/12','rows':rows}
(root/'family-ridge-margins.json').write_text(json.dumps(out,indent=2)+'\n')
print('PASS: 12 primitive tight positive words persist identically over QQ(v,s).')
print('PASS: all 24 mixed ridge adjacencies, positive densities and margin functions regular at base.')
print('Consequence: one small equality-family parameter patch preserves every mixed-ridge margin < 1; per-direction cut depth only.')
