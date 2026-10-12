# /// script
# dependencies = ["sympy"]
# ///
"""Exact mixed-ridge sections: positive active-word adjacency surgery."""
import itertools
import json
from pathlib import Path
import sympy as S

root = Path(__file__).resolve().parent
packet = json.loads((root.parent/'vertex-cones/witnesses.json').read_text())
A=S.Matrix(packet['normal_rows']);K=S.Matrix(packet['q_vertices']);T=S.Matrix(packet['p_vertices'])
O=S.Matrix([[0,0,1,0],[0,0,0,1],[-1,0,0,0],[0,-1,0,0]])
t,delta=S.symbols('t delta')
words={}
for row in packet['witnesses']:
 pairs=[(i,S.Rational(b)) for i,b in zip(row['word'],row['base_beta']) if S.Rational(b)>0]
 k=min(range(len(pairs)),key=lambda k:pairs[k][0]);pairs=pairs[k:]+pairs[:k]
 word=tuple(i for i,b in pairs);beta=dict(pairs)
 assert sum(beta.values())==1
 assert sum((beta[i]*A[i,:] for i in word),S.zeros(1,4))==S.zeros(1,4)
 Q=sum(beta[i]*beta[j]*(A[i,:]*O*A[j,:].T)[0] for r,i in enumerate(word) for j in word[r+1:])
 assert Q==S.Rational(1,2)
 words[word]=beta
assert len(words)==12
Ls={}
for i in range(10):
 n=A[i,:2] if i<6 else A[i,2:];V=K if i<6 else T
 ends=[V[k,:] for k in range(V.rows) if (n*V[k,:].T)[0]==1]
 assert len(ends)==2
 e=ends[1]-ends[0]
 Ls[i]=abs(e[0]/n[1]) if n[1]!=0 else abs(e[1]/n[0])
adjacencies={}
for word,beta in words.items():
 for a,b in zip(word,word[1:]+word[:1]):
  if (a<6)==(b<6):continue
  omega=(A[a,:]*O*A[b,:].T)[0]
  if omega>0:adjacencies[(min(a,b),max(a,b))]=(word,beta,a,b,omega)
assert set(adjacencies)==set(itertools.product(range(6),range(6,10)))
rows=[]
for (qi,pi),(word,beta,a,b,omega) in sorted(adjacencies.items()):
 k=word.index(a);rot=word[k:]+word[:k];assert rot[1]==b
 new_word=rot[:1]+(10,)+rot[1:]
 w=t*A[a,:]+(1-t)*A[b,:];B=A.col_join(w)
 base=S.Matrix([0 if i==10 else beta[i] for i in new_word])
 d=S.Matrix([1 if i==10 else -t if i==a else -(1-t) if i==b else 0 for i in new_word])
 closure=B[list(new_word),:].T.col_join(S.ones(1,len(new_word)))
 assert closure*base==S.Matrix([0,0,0,0,1])
 assert all(S.expand(z)==0 for z in closure*d)
 H=S.zeros(len(new_word))
 for r,s in itertools.combinations(range(len(new_word)),2):H[r,s]=H[s,r]=(B[new_word[r],:]*O*B[new_word[s],:].T)[0]
 assert S.expand((base.T*H*base)[0])==1
 assert S.expand((base.T*H*d)[0])==0
 C=omega*t*(1-t)
 assert S.expand((d.T*H*d)[0]+2*C)==0
 product=Ls[qi]*Ls[pi]*omega
 assert 0<product<=S.Rational(5,8)
 assert base[new_word.index(a)]>0 and base[new_word.index(b)]>0
 rows.append({'q_facet':qi,'p_facet':pi,'word':new_word,'beta':[str(x) for x in base],'d':[str(x) for x in d],'omega':str(omega),'C':str(C),'Lq':str(Ls[qi]),'Lp':str(Ls[pi]),'margin_product':str(product)})
out={'status':'PASS exact 24 mixed-ridge adjacency sections','maximum_margin_product':str(max(S.Rational(r['margin_product']) for r in rows)),'required_strict_bound':'<1','rows':rows}
(root/'ridge-margins.json').write_text(json.dumps(out,indent=2)+'\n')
print('PASS: 12 feasible tight primitive words; positive adjacencies cover all 24 mixed ridges.')
print('PASS: all 24 exact new-label surgery branches; max Lq Lp omega = 5/8 < 1.')
