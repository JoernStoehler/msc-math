"""Independent exact Sturm/gcd arithmetic; no producer imports or writes.

Replay basic adversarial tests and all retained relevant-polynomial sequences.
Evaluation uses the independently reviewed ordered quadratic boundary adapter.
"""
import ast, importlib.util, json, time
from pathlib import Path
from fractions import Fraction
from math import isqrt
root=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('independent_arithmetic',root.parent.parent/'hko-angle-nonconformal'/'review_candidates.py')
m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
E,z,one=m.E,m.z,m.one
names={'rational_sqrt','is_field_square','Root','root_decode','V','value_decode','compare'}
tree=ast.parse((root.parent/'joint-denominators'/'review_joint.py').read_text())
ns={'E':E,'z':z,'dec':m.decd,'Fraction':Fraction,'isqrt':isqrt,'registry':{}}
exec(compile(ast.Module(body=[n for n in tree.body if isinstance(n,(ast.FunctionDef,ast.ClassDef)) and n.name in names],type_ignores=[]),'independent_boundary_adapter','exec'),ns)
V,decv,compare=ns['V'],ns['value_decode'],ns['compare']
def trim(p):
 p=list(p)
 while len(p)>1 and p[-1]==z:p.pop()
 return p
def degree(p):return -1 if trim(p)==[z] else len(trim(p))-1
def add(p,q):return trim([(p[i] if i<len(p) else z)+(q[i] if i<len(q) else z) for i in range(max(len(p),len(q)))])
def mul(p,q):
 r=[z]*(len(p)+len(q)-1)
 for i,a in enumerate(p):
  for j,b in enumerate(q):r[i+j]+=a*b
 return trim(r)
def scale(p,c):return trim([v*c for v in p])
def derivative(p):return trim([E(i)*p[i] for i in range(1,len(p))] or [z])
def evaluate(p,x):
 v=V(z)
 for a in reversed(p):v=v*x+V(a)
 return v
def divrem(p,q):
 p=trim(p);q=trim(q);assert q!=[z]
 quotient=[z]*max(1,len(p)-len(q)+1)
 while p!=[z] and len(p)>=len(q):
  k=len(p)-len(q);a=p[-1]/q[-1];quotient[k]+=a
  p=add(p,[z]*k+scale(q,-a))
 return trim(quotient),p
def monic(p):return scale(p,one/p[-1]) if p!=[z] else p
def gcd(p,q):
 while q!=[z]:p,q=q,divrem(p,q)[1]
 return monic(p)
def sturm(p):
 p=trim(p);assert p!=[z]
 if len(p)==1:return [p]
 seq=[p,derivative(p)]
 while True:
  r=divrem(seq[-2],seq[-1])[1]
  if r==[z]:break
  # Positive rescaling controls growth without changing Sturm signs.
  r=scale(r,-one);lead=r[-1];r=scale(r,one/(lead if lead.sign()>0 else -lead))
  seq.append(r)
 return seq
def side_sign(p,x,side):
 """Polynomial sign immediately to one side, handling endpoint roots."""
 order=0
 while p!=[z]:
  s=evaluate(p,x).sign()
  if s:return s*((-1)**order if side<0 else 1)
  p=derivative(p);order+=1
 return 0
def variations(signs):
 signs=[s for s in signs if s]
 return sum(a!=b for a,b in zip(signs,signs[1:]))
def count_open(p,left,right):
 assert compare(left,right)<0
 seq=sturm(p)
 return variations([side_sign(q,left,1) for q in seq])-variations([side_sign(q,right,-1) for q in seq])
def multiplicity(p,x):
 n=0
 while p!=[z] and evaluate(p,x).sign()==0:p=derivative(p);n+=1
 return n

def tests():
 X=[z,one];a=V(E(-2));b=V(E(2))
 assert count_open(X,a,b)==1
 assert count_open(mul(X,X),a,b)==1
 assert count_open(mul(X,X),V(z),b)==0
 assert count_open(mul(X,X),a,V(z))==0
 p=mul(mul(X,X),add(X,[E(-1)]))
 assert count_open(p,a,b)==2 and multiplicity(p,V(z))==2
 assert side_sign(p,V(z),1)==side_sign(p,V(z),-1)==-1
 assert gcd(p,derivative(p))==X
 p=mul([E(-1),z,one],[E(-1),z,one])
 assert count_open(p,V(E(-1)),V(E(1)))==0
 assert multiplicity(p,V(E(1)))==2
 # Positive sqrt(5) embedding: x^2-sqrt(5) has one positive root in (1,2).
 p=[E(0,-1),z,one]
 assert count_open(p,V(E(1)),V(E(2)))==1
 # Algebraic endpoint beta=sqrt(2), actual ordered quadratic extension.
 beta=decv({'kind':'quadratic_event_root','monic_coefficients':[['-2','0'],['0','0'],['1','0']],'left':['1','0'],'right':['3/2','0'],'branch':1})
 p=[E(-2),z,one]
 assert evaluate(p,beta).sign()==0 and count_open(p,V(z),beta)==0
 assert count_open(mul(p,p),beta,V(E(2)))==0
 cubic=[E(-2),z,z,one]
 assert count_open(cubic,V(E(1)),beta)==1
 quartic=mul(cubic,[E(1),one]);assert count_open(quartic,V(E(-2)),beta)==2
 return 15

def main():
 start=time.perf_counter();n=tests();packet=json.loads((root/'clipped.json').read_text())
 seqs=[]
 for row in packet['polynomials']:
  p=list(map(m.decd,row['coefficients']));seq=sturm(p);seqs.append(seq)
  for i in range(2,len(seq)):
   remainder=divrem(seq[i-2],seq[i-1])[1]
   assert remainder!=[z]
   ratio=seq[i][-1]/(-remainder[-1]);assert ratio.sign()>0
   assert seq[i]==scale(remainder,-ratio)
 print(json.dumps({'status':'PASS independent Sturm/gcd/basic ordered-boundary tests','basic_tests':n,'relevant_polynomials':len(seqs),'degree_counts':packet['degree_counts'],'elapsed_seconds':round(time.perf_counter()-start,3),'writes':False}))
if __name__=='__main__':main()
