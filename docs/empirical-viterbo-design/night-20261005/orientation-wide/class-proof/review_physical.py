"""Fresh independent physical equality reconstruction; reads certificates only."""
from review_signs import *
from itertools import combinations
import hashlib

def main():
 start=time.perf_counter();eq=json.loads((root/'equality.json').read_text());assert len(eq['equality_bodies'])==1
 rec=eq['equality_bodies'][0];h=[m.decd(v) for v in rec['supports']];u=m.decd(rec['u']);x=m.decd(rec['x']);assert x==one and u==E(0,'1/5')
 physical=json.loads((root/'physical-control.json').read_text())
 assert physical['equality_sha256']==hashlib.sha256((root/'equality.json').read_bytes()).hexdigest()
 r=E(0,1);tau2=E(5)-2*r;b=(one+r)/4;R=(E(5)-r)/10
 N=[(one,z),((r-one)/4,(E(5)-r)/4),(-(one+r)/4,(3*r-E(5))/4),(-(one+r)/4,-(3*r-E(5))/4),((r-one)/4,-(E(5)-r)/4)]
 G=[(x*(a-u*c),tau2*u*a+c) for a,c in N]
 triangles=[];cur=[(-R,z),(R,z),(z,R)]
 for j in range(10):
  triangles.append(cur);cur=[(b*a-(3*r-E(5))/4*c,b*(a+c)) for a,c in cur]
 assert cur==triangles[0]
 dot=lambda a,b:a[0]*b[0]+a[1]*b[1]
 cross=lambda a,b:a[0]*b[1]-a[1]*b[0]
 def solve(a,b,c,d):
  det=cross(a,b);assert det!=z;return ((c*b[1]-d*a[1])/det,(a[0]*d-b[0]*c)/det)
 vertices=[solve(G[i],G[(i+1)%5],h[i],h[(i+1)%5]) for i in range(5)]
 for v in vertices:assert all((h[i]-dot(G[i],v)).sign()>=0 for i in range(5))
 sub=lambda a,b:(a[0]-b[0],a[1]-b[1])
 assert all(cross(sub(vertices[i],vertices[i-1]),sub(vertices[(i+1)%5],vertices[i])).sign()>0 for i in range(5))
 area=sum((cross(vertices[i],vertices[(i+1)%5]) for i in range(5)),z)/2
 P=sum((h[i]*h[(i+1)%5]-m.a*h[i]*h[i] for i in range(5)),z);d=one+tau2*u*u
 assert area==P/(x*d*(E(5)-r)/4) and P==x*d/(E(3)+r)
 placements=[];duals=[]
 def maximum(values):
  value=values[0]
  for other in values[1:]:
   if (other-value).sign()>0:value=other
  return value
 packet=json.loads((root.parent/'joint-denominators'/'joint-denominators.json').read_text())
 maxima=[None]*5
 for j,tri in enumerate(triangles):
  supports=[maximum([dot(g,v) for v in tri]) for g in G]
  # Exact independent support maximization.
  for i,g in enumerate(G):assert all((supports[i]-dot(g,v)).sign()>=0 for v in tri)
  rhs=[h[i]-supports[i] for i in range(5)];placement=None
  for i,k in combinations(range(5),2):
   at=solve(G[i],G[k],rhs[i],rhs[k])
   if all((rhs[l]-dot(G[l],at)).sign()>=0 for l in range(5)):placement=at;break
  assert placement is not None;placements.append(placement)
  reported=physical['placements'][j];assert reported['template']==j
  assert supports==list(map(m.decd,reported['support_coefficients']))
  translation=tuple(map(m.decd,reported['translation']))
  assert all((rhs[i]-dot(G[i],translation)).sign()>=0 for i in range(5))
  assert [rhs[i]-dot(G[i],translation) for i in range(5)]==list(map(m.decd,reported['slacks']))
  for l,lam in enumerate(m.lams):
   value=sum((lam[i]*supports[i] for i in range(5)),z)
   assert value==m.decd(physical['all50_denominators'][l][j])
   if maxima[l] is None or (value-maxima[l]).sign()>0:maxima[l]=value
   if value==sum((lam[i]*h[i] for i in range(5)),z):duals.append((l,j))
 assert duals and len(placements)==10
 for l,circuit in enumerate(packet['circuits']):
  raw=[]
  for row in circuit['candidates']:
   c,a,du,bu=map(m.decd,row['coefficients']);raw.append(c+a*x+du*u+bu*x*u)
  assert all((maxima[l]-v).sign()>=0 for v in raw) and maxima[l] in raw
  assert sum((m.lams[l][i]*h[i] for i in range(5)),z)==maxima[l]
 D=(r-one)/5;t=solve(N[3],N[4],-D,-D)
 assert all(h[i]==D+dot(N[i],t) for i in range(5))
 norm=lambda v:v[0]*v[0]+tau2*v[1]*v[1]
 sides=[norm(sub(vertices[(i+1)%5],vertices[i])) for i in range(5)]
 center=tuple(sum((v[k] for v in vertices),z)/5 for k in range(2));radii=[norm(sub(v,center)) for v in vertices]
 assert all(v==sides[0] and v.sign()>0 for v in sides) and all(v==radii[0] and v.sign()>0 for v in radii)
 qverts=[solve(N[i],N[(i+1)%5],b,b) for i in range(5)]
 qarea=sum((cross(qverts[i],qverts[(i+1)%5]) for i in range(5)),z)/2
 ratio=one/(2*tau2*area*qarea);assert ratio==(E(3)+r)/5
 assert N==[tuple(map(m.decd,n)) for n in physical['source_regular_covectors']]
 assert triangles==[[tuple(map(m.decd,v)) for v in tri] for tri in physical['source_template_vertices']]
 assert G==[tuple(map(m.decd,n)) for n in physical['scaled_normals']]
 assert vertices==[tuple(map(m.decd,v)) for v in physical['K_computational_vertices']]
 assert qverts==[tuple(map(m.decd,v)) for v in physical['Q_computational_vertices']]
 assert area==m.decd(physical['K_computational_area']) and qarea==m.decd(physical['Q_computational_area'])
 assert ratio==m.decd(physical['direct_product_ratio'])
 assert sides[0]==m.decd(physical['physical_side_squared']) and radii[0]==m.decd(physical['physical_circumradius_squared'])
 assert t==tuple(map(m.decd,physical['canonical_Y_translation']))
 print(json.dumps({'status':'PASS fresh independent physical equality control','actual_sides':5,'primal_fits':len(placements),'active_dual_pairs':len(duals),'complete_capacity':'exactly one','ratio':[str(ratio.a),str(ratio.b)],'physical_body':'regular pentagon at covector rotation +pi/10','direct_shoelace':True,'catalogue_capacity_agreement':True,'elapsed_seconds':round(time.perf_counter()-start,3),'writes':False}))
if __name__=='__main__':main()
