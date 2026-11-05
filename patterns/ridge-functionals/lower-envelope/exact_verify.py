"""Standard-library exact feasible HK certificate; no capacity evaluator."""
from fractions import Fraction as F
import json
from pathlib import Path
P=[(1,-2),(1,3),(-1,4),(-1,-1),(0,-4)]
Q=[(0,-2),(2,-1),(1,1),(-1,2),(-2,0)]
WORD=[(0,0),(1,1),(0,2),(1,3),(1,4)]
WEIGHTS=[F(1,4),F(3,14),F(1,4),F(2,21),F(4,21)]
def cross(a,b):return a[0]*b[1]-a[1]*b[0]
def dot(a,b):return sum(x*y for x,y in zip(a,b))
def sub(a,b):return tuple(x-y for x,y in zip(a,b))
def edges(p):return [sub(p[(i+1)%len(p)],p[i]) for i in range(len(p))]
def area(p):return sum(cross(p[i],p[(i+1)%len(p)]) for i in range(len(p)))/2
def polar(p):
 e=edges(p);n=[(v[1],-v[0]) for v in e];h=[dot(a,v) for a,v in zip(n,p)]
 assert min(h)>0 # origin strictly interior
 answer=[tuple(x/y for x in a) for a,y in zip(n,h)]
 # CCW strict convexity and genuine edges: every nonincident vertex is
 # strictly within each edge's supporting halfplane.
 for i,a in enumerate(answer):
  for j,v in enumerate(p):
   assert dot(a,v)==1 if j in (i,(i+1)%len(p)) else dot(a,v)<1
 assert all(cross(e[i],e[(i+1)%len(e)])>0 for i in range(len(e)))
 return answer

def verify():
 p=[tuple(map(F,v)) for v in P];q=[tuple(map(F,v)) for v in Q]
 normals=[polar(p),polar(q)]
 n=[]
 for factor,edge in WORD:
  a=normals[factor][edge];n.append(a+(F(0),F(0)) if factor==0 else (F(0),F(0))+a)
 assert len(set(WORD))==len(WORD)
 assert min(WEIGHTS)>0 and sum(WEIGHTS)==1
 assert all(sum(w*a[i] for w,a in zip(WEIGHTS,n))==0 for i in range(4))
 assert all(sum(w for w,a in zip(WEIGHTS,WORD) if a[0]==factor)==F(1,2) for factor in (0,1))
 def omega(a,b):return a[0]*b[2]+a[1]*b[3]-a[2]*b[0]-a[3]*b[1]
 value=sum(WEIGHTS[i]*WEIGHTS[j]*omega(n[i],n[j]) for i in range(len(n)) for j in range(i+1,len(n)))
 assert value==F(1,14)
 c=1/(2*value);ap=area(p);aq=area(q);amax=max(abs(dot(a,b)) for a in edges(p) for b in edges(q))
 normalized=c*amax/(ap*aq);target=F(4,5);law_upper=normalized**2/2
 assert (ap,aq,amax,c)==(F(25,2),F(9),F(10),F(7))
 assert normalized==F(28,45)<target and law_upper==F(392,2025)<F(8,25)
 return {'status':'PASS','polygon_P':[[str(x) for x in v] for v in p], 'polygon_Q':[[str(x) for x in v] for v in q], 'word':[list(x) for x in WORD],'weights':[str(x) for x in WEIGHTS],'word_normals':[[str(x) for x in v] for v in n],'Q':str(value),'capacity_bound':str(c),'area_P':str(ap),'area_Q':str(aq),'maximum_edge_pairing':str(amax),'c_upper_times_Amax_over_area_product':str(normalized),'target':'4/5','law_upper_bound':str(law_upper),'law_target':'8/25','relative_gap_below_target':str(1-normalized/target),'assumption':'HK feasible-word capacity upper bound c<=1/(2Q), coordinates (q1,q2,p1,p2).'}
if __name__=='__main__':
 result=verify();print(json.dumps(result,indent=2))
