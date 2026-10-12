import sys,json,itertools
from pathlib import Path
repo_root=Path(__file__).resolve().parents[4]
sys.path.insert(0,str(repo_root/'docs/empirical-viterbo-design/desk/pentagon-arbitrary-partner/local-cover'))
from pentagon_fixture import F,tau,s5,Z,O,sign,dot,sub,add,scale,cross,solve,hull,area,serialize
h=(O+s5)/F(4);c72=(s5-O)/F(4);s36=h*tau;s72=F(2)*h*h*tau
normals=[[O,Z],[c72,s72],[-h,s36],[-h,-s36],[c72,-s72]]
Q=[[h,s36],[-c72,s72],[-O,Z],[-c72,-s72],[h,-s36]]
K=hull([[F(-3)/F(5),F(-1)/F(2)],[F(3)/F(5),F(-4)/F(5)],[Z,O],[F(-3)/F(5),F(-3)/F(10)]])
D=hull([sub(a,b) for a in K for b in K]);AK=area(K);AD=area(D)
assert AD/(F(4)*AK)==F(101)/F(70)
facets=[]
for a,b in zip(K,K[1:]+K[:1]):
 e=sub(b,a);n=[e[1],-e[0]];facets.append((n,dot(n,a)))
r=F(9)/F(5);cert=[]
for ids in itertools.combinations(range(5),3):
 try:w=solve([[normals[i][j] for i in ids] for j in range(2)]+[[O]*3],[Z,Z,O])
 except Exception:continue
 if not all(sign(x)>0 for x in w):continue
 for order in [(0,1,2),(0,2,1)]:
  edges=[scale(w[k]/h,normals[ids[k]]) for k in order]
  T=[[Z,Z],edges[0],add(edges[0],edges[1])]
  rhs=[]
  for n,b in facets:
   vals=[dot(n,p) for p in T];mx=vals[0]
   for x in vals[1:]:
    if sign(x-mx)>0:mx=x
   rhs.append(b-r*mx)
  found=None
  for i,j in itertools.combinations(range(len(facets)),2):
   if cross(facets[i][0],facets[j][0])==Z:continue
   t=solve([facets[i][0],facets[j][0]],[rhs[i],rhs[j]])
   if all(sign(rhs[k]-dot(facets[k][0],t))>=0 for k in range(len(facets))):found=t;break
  assert found is not None,(ids,order)
  cert.append({'ids':ids,'order':order,'translation':[serialize(x) for x in found]})
assert len(cert)==10
v=[F(-9)/F(10),F(-3)/F(5)]
# v lies on edge (-6/5,3/10) -- (-3/5,-3/2) of D.
a=[F(-6)/F(5),F(3)/F(10)];b=[F(-3)/F(5),F(-3)/F(2)]
assert tuple(a) in D and tuple(b) in D and v==scale(O/F(2),add(a,b))
e=sub(b,a);outer=[e[1],-e[0]]
assert all(sign(dot(outer,sub(p,a)))<=0 for p in D)
assert cross(e,sub(v,a))==Z
DQ=[sub(a,b) for a in Q for b in Q]
upper=F(203)/F(100)
assert all(sign(upper-dot(v,p))>=0 for p in DQ)
lower=(F(101)/F(70))*(r/upper)**2
assert sign(lower-s5/F(2))>0
out={'status':'PASS exact ordered algebraic certificate','K':[[serialize(x) for x in p] for p in K],'areaK':serialize(AK),'areaD':serialize(AD),'asymmetry_factor':'101/70','capacity_lower':'9/5','A2_upper':'203/100','retention_asymmetry_lower':serialize(lower),'target':'sqrt(5)/2','fits':cert,'A2_boundary_witness':[serialize(x) for x in v]}
path=Path(__file__).with_name('asymmetry-counterexample.json');path.write_text(json.dumps(out,indent=2)+'\n')
print(json.dumps({k:v for k,v in out.items() if k not in ['fits','K']},indent=2))
