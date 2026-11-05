"""Independent exact sector symmetry audit, using only this reviewer's field code."""
from pathlib import Path
exec(Path(__file__).with_name('independent_check.py').read_text().split('c=json.loads(SRC.read_text())')[0])
c=json.loads(SRC.read_text());rows=[[dec(v) for v in row['coefficients']] for row in c['rows']]
Q=[[dec(v) for v in row] for row in c['positive_det_quadratic_matrix']]
r=E(0,1);b=(1+r)/4;tau2=5-2*r;l=(3*r-5)/4
assert l==b*tau2
T=[[E(int(i==j)) for j in range(7)] for i in range(7)]
T[3]=[E(),E(),E(),b,E(),-b,E()]
T[4]=[E(),E(),E(),E(),b,E(),-b]
T[5]=[E(),E(),E(),l,E(),b,E()]
T[6]=[E(),E(),E(),E(),l,E(),b]
def norm(row):
    scale=next(v for v in row if v!=0)
    if scale.sign()<0:scale=-scale
    return tuple(tuple((v/scale).enc()) for v in row)
keys={norm(row) for row in rows}
pullbacks={norm([dot(row,col) for col in zip(*T)]) for row in rows}
assert keys==pullbacks
assert matmul(list(map(list,zip(*T))),matmul(Q,T))==Q
I=[[E(int(i==j)) for j in range(7)] for i in range(7)];T10=I
for _ in range(10):T10=matmul(T,T10)
assert T10==I
trace=[E(),E(),E(),E(1),E(),E(),E(1)]
u=[E(),E(),E(),E(),-tau2,E(1),E()]
trace_pull=[dot(trace,col) for col in zip(*T)];u_pull=[dot(u,col) for col in zip(*T)]
assert trace_pull==[b*(x-y) for x,y in zip(trace,u)]
assert u_pull==[b*(tau2*x+y) for x,y in zip(trace,u)]
points=json.loads((OUT/'independent-receipt.json').read_text())['points']
inside=[]
for k,x in enumerate(points):
    x=list(map(dec,x));tt,uu=dot(trace,x),dot(u,x)
    if uu.sign()>=0 and (tau2*tt-uu).sign()>=0:inside.append(k)
assert inside==[0]
# Exact determinant identity on the four independent Bvariables, tested as
# homogeneous quadratic polynomial by diagonal and mixed polarization.
def identity(z):
    B00,B01,B10,B11=z;t=B00+B11;uu=B10-tau2*B01
    return (t*t+uu*uu/tau2-(B00-B11)*(B00-B11)-(B10+tau2*B01)*(B10+tau2*B01)/tau2)/4-(B00*B11-B01*B10)
basis=[[E(int(i==j)) for i in range(4)] for j in range(4)]
assert all(identity(x)==0 for x in basis)
assert all(identity([a+b for a,b in zip(x,y)])==0 for x in basis for y in basis)
receipt={'status':'PASS','all610rows_permuted_with_positive_scaling':True,'target_invariant':True,'rotation_order':10,
 'equality_ray_indices_in_closed_sector':inside,
 'determinant_identity_exact':True,
 'proof':'For detB>0, (t,u/tau) is nonzero by the determinant identity. The exact action tprime=b(t-u), uprime=b(tau2*t+u) rotates this vector by36degrees. Ten rotations cover the closed36degree wedge u>=0,tau2*t-u>=0. Row permutation and target invariance carry positivity questions into this sector.',
 'limitation':'Only the ten-ray certificate obstruction disappears after sector restriction; no sector positivity proof or feasible SDP certificate is established.'}
print(json.dumps(receipt,indent=2))
