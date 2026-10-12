# /// script
# dependencies = ["sympy"]
# ///
"""Independent geometric capacity/area check of the selected sharp optimizer."""
import json
from pathlib import Path
import body_candidates as bc
F=bc.F;Z=bc.Z;O=bc.O;r5=bc.r5
root=Path(__file__).parent;packet=json.loads((root/'denominators.json').read_text())
dec=lambda p:F(bc.sp.Rational(p[0]))+F(bc.sp.Rational(p[1]))*r5
N=[[dec(v) for v in p] for p in packet['normals']]
T=[[[dec(v) for v in p] for p in tri] for tri in packet['triangles']]
x=r5/F(3);h=[(r5-O)/F(6),(r5+O)/F(6),O/F(3),Z,Z]
G=[[x*p[0],p[1]] for p in N]
def dot(a,b):return sum((u*v for u,v in zip(a,b)),Z)
def cross(a,b):return a[0]*b[1]-a[1]*b[0]
def maximum(vals):
 out=vals[0]
 for v in vals[1:]:
  if bc.sign(v-out)>0:out=v
 return out
def solve(a,b,u,v):
 det=cross(a,b);assert det!=Z
 return [(u*b[1]-v*a[1])/det,(a[0]*v-b[0]*u)/det]
vertices=[solve(G[i],G[(i+1)%5],h[i],h[(i+1)%5]) for i in range(5)]
for i,p in enumerate(vertices):
 assert all(bc.sign(h[k]-dot(G[k],p))>=0 for k in range(5))
 assert cross([vertices[i][k]-vertices[(i-1)%5][k] for k in range(2)], [vertices[(i+1)%5][k]-vertices[i][k] for k in range(2)])!=Z
area=sum((cross(vertices[i],vertices[(i+1)%5]) for i in range(5)),Z)/F(2)
P=sum((h[i]*h[(i+1)%5]-bc.a*h[i]*h[i] for i in range(5)),Z)
assert area==P/(x*(F(5)-r5)/F(4))
Ds=[]
for item in packet['denominators']:
 cell=next(c for c in item['cells'] if bc.sign(x-dec(c['left']))>=0 and (c['right'] is None or bc.sign(dec(c['right'])-x)>0))
 Ds.append(dec(cell['intercept'])+x*dec(cell['slope']))
assert all(sum((bc.lams[l][i]*h[i] for i in range(5)),Z)==Ds[l] for l in range(5))
placements=[]
for j,tri in enumerate(T):
 supports=[maximum([dot(normal,p) for p in tri]) for normal in G]
 rhs=[h[i]-supports[i] for i in range(5)];placement=None
 for i,k in bc.combinations(range(5),2):
  t=solve(G[i],G[k],rhs[i],rhs[k])
  if all(bc.sign(rhs[l]-dot(G[l],t))>=0 for l in range(5)):placement=t;break
 assert placement is not None
 placements.append({'template':j,'translation':[bc.ser(v) for v in placement],'slacks':[bc.ser(rhs[i]-dot(G[i],placement)) for i in range(5)]})
# A maximizing denominator on an active closure circuit gives an exact dual
# upper bound of one; all ten primal placements give the reverse capacity bound.
assert any(sum((bc.lams[0][i]*maximum([dot(G[i],p) for p in tri]) for i in range(5)),Z)==Ds[0] for tri in T)
rho=x/(F(5)*P);bound=F(3)*(O+r5)/F(10);assert rho==bound
edges=[sum((bc.edges[i][k]*h[k] for k in range(5)),Z) for i in range(5)]
assert all(bc.sign(e)>0 for e in edges)
assert Ds[0]!=Ds[1] # affine regular bodies in this fan have all circuit sums equal.
# Exact affine-regular test: convex affine maps preserve/reverse cyclic order,
# so only ten labelings are possible. Three point images fix the affine map.
Q=[solve(N[i],N[(i+1)%5],bc.b,bc.b) for i in range(5)]
sub=lambda p,q:[p[k]-q[k] for k in range(2)]
u=sub(Q[1],Q[0]);v=sub(Q[2],Q[0]);det=cross(u,v)
affine_failures=[]
for direction in (1,-1):
 for shift in range(5):
  image=[vertices[(shift+direction*k)%5] for k in range(5)]
  failed=[]
  for k in (3,4):
   delta=sub(Q[k],Q[0]);alpha=cross(delta,v)/det;beta=cross(u,delta)/det
   expected=[image[0][l]+alpha*(image[1][l]-image[0][l])+beta*(image[2][l]-image[0][l]) for l in range(2)]
   residual=sub(image[k],expected)
   if residual!=[Z,Z]:failed.append({'vertex':k,'residual':[bc.ser(z) for z in residual]})
  assert failed
  affine_failures.append({'direction':direction,'shift':shift,'failures':failed})
out={'status':'PASS exact optimizer capacity1/alltenfits/area/actual5sides','x':bc.ser(x),'s':bc.ser(x-O),'canonical_quotient_supports':[bc.ser(v) for v in h],'actual_sides':5,'computational_K_vertices':[[bc.ser(v) for v in p] for p in vertices],'computational_K_area':bc.ser(area),'edge_numerators':[bc.ser(v) for v in edges],'circuit_denominators':[bc.ser(v) for v in Ds],'capacity':'exactly1: ten primal fits + active maximal circuit dual','placements':placements,'sharp_ratio':bc.ser(rho),'sharp_ratio_numeric':bc.num(rho),'not_affine_regular_reason':'all ten possible cyclic/reversed affine vertex correspondences fail exactly','affine_regular_exclusion':affine_failures}
(root/'optimizer-certificate.json').write_text(json.dumps(out,indent=2)+'\n')
print(out['status'], 'ratio',F.to_sympy(rho), 'actualsides',5)
