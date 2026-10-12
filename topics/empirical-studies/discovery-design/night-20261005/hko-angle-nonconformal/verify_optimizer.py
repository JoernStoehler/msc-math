# /// script
# dependencies = ["sympy"]
# ///
"""Exact physical geometry/capacity/area check of the HKO-angle optimizer."""
import json
from pathlib import Path
import body_candidates as bc
F=bc.F;Z=bc.Z;O=bc.O;r5=bc.r5
root=Path(__file__).parent;packet=json.loads((root/'denominators.json').read_text())
dec=lambda p:F(bc.sp.Rational(p[0]))+F(bc.sp.Rational(p[1]))*r5
N=[[dec(v) for v in p] for p in packet['normals']]
T=[[[dec(v) for v in p] for p in tri] for tri in packet['triangles']]
x=O;h=[-O+F(3)*r5/F(5),O-r5/F(5),-O+F(3)*r5/F(5),Z,Z]
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
assert area==P/(x*bc.detC*(F(5)-r5)/F(4))
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
rho=x*bc.detC/(F(5)*P);bound=(F(3)+r5)/F(5);assert rho==bound
edges=[sum((bc.edges[i][k]*h[k] for k in range(5)),Z) for i in range(5)]
assert all(bc.sign(e)>0 for e in edges)
# Canonical Y is exactly a translate of a regular pentagon: prove all
# support differences from the common D lie in the translation kernel.
source_N=[[dec(v) for v in p] for p in packet['source_regular_normals']]
D=(r5-O)/F(5);assert all(value==D for value in Ds)
zY=solve(source_N[3],source_N[4],-D,-D)
assert all(h[i]==D+dot(source_N[i],zY) for i in range(5))
# Direct physical Euclidean geometry check uses diag(1,tau²) on computational
# points. This is independent of the area transformation formula above.
tau2=F(5)-F(2)*r5
physical_norm2=lambda p:p[0]*p[0]+tau2*p[1]*p[1]
sub=lambda p,q:[p[k]-q[k] for k in range(2)]
side_squared=[physical_norm2(sub(vertices[(i+1)%5],vertices[i])) for i in range(5)]
center=[sum((p[k] for p in vertices),Z)/F(5) for k in range(2)]
radius_squared=[physical_norm2(sub(p,center)) for p in vertices]
assert len(set(side_squared))==1 and len(set(radius_squared))==1
assert bc.sign(side_squared[0])>0 and bc.sign(radius_squared[0])>0
Q_vertices=[solve(source_N[i],source_N[(i+1)%5],bc.b,bc.b) for i in range(5)]
Q_computational_area=sum((cross(Q_vertices[i],Q_vertices[(i+1)%5]) for i in range(5)),Z)/F(2)
rho_direct=O/(F(2)*tau2*area*Q_computational_area)
assert rho_direct==bound
# In physical coordinates normal zero is (1,tau*u), hence angle +pi/10;
# Y=M'^T K at x=1 implies K=cos(pi/10) R_(+pi/10) Y.
out={'status':'PASS exact optimizer capacity1/alltenfits/area/actual5sides','x':bc.ser(x),'angle':'pi/10, positive covector rotation', 'normal_convention':'G_x C_u N with cos angle absorbed into supports','detC':bc.ser(bc.detC),'canonical_quotient_supports':[bc.ser(v) for v in h],'actual_sides':5,'computational_K_vertices':[[bc.ser(v) for v in p] for p in vertices],'computational_K_area':bc.ser(area),'edge_numerators':[bc.ser(v) for v in edges],'circuit_denominators':[bc.ser(v) for v in Ds],'capacity':'exactly1: ten primal fits + active maximal circuit dual','placements':placements,'sharp_ratio':bc.ser(rho),'sharp_ratio_numeric':bc.num(rho),'regular_Y_translation':[bc.ser(v) for v in zY],'physical_side_squared':bc.ser(side_squared[0]),'physical_circumradius_squared':bc.ser(radius_squared[0]),'physical_regular_geometry':'equal five Euclidean sides and circumradii; canonical Y support equals translated constant D','direct_Q_computational_area':bc.ser(Q_computational_area),'direct_product_ratio':bc.ser(rho_direct)}
(root/'optimizer-certificate.json').write_text(json.dumps(out,indent=2)+'\n')
print(out['status'], 'ratio',F.to_sympy(rho), 'actualsides',5)
