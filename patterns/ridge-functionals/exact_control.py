"""Exact obstruction to extending the affine-pentagon max-ridge law.
Standard library only. Capacity lower bound imports monotonicity, conformality,
and exact c(T x_L T)=9/2 from ../product-shape/exact_controls.py.
"""
from fractions import Fraction as F
import json
from itertools import product, combinations

T=[(-1,-1),(2,-1),(-1,2)]
P=[(-1,-1),(F(7,4),-1),(F(7,4),F(-3,4)),(F(-3,4),F(7,4)),(-1,F(7,4))]
def area(p):return abs(sum(p[i][0]*p[(i+1)%len(p)][1]-p[i][1]*p[(i+1)%len(p)][0] for i in range(len(p))))/2
def dot(a,b):return sum(x*y for x,y in zip(a,b))
def edges(p):return [tuple(p[(i+1)%len(p)][j]-p[i][j] for j in range(2)) for i in range(len(p))]
def verify():
    p=[tuple(map(F,v)) for v in P];t=[tuple(map(F,v)) for v in T];scale=F(7,8)
    # P = {x>=-1, y>=-1, x+y<=1, x<=7/4, y<=7/4}; all constraints valid.
    normals=[(-1,0),(0,-1),(1,1),(1,0),(0,1)];heights=[1,1,1,F(7,4),F(7,4)]
    assert all(dot(n,v)<=h for n,h in zip(normals,heights) for v in p)
    # Enumerate every feasible intersection of two boundary lines. The first
    # three halfplanes already form bounded T, so these are all H-vertices.
    corners=set()
    for i,j in combinations(range(5),2):
        a,b=map(F,normals[i]);c,d=map(F,normals[j]);det=a*d-b*c
        if det==0:continue
        u,v=heights[i],heights[j];point=((u*d-b*v)/det,(a*v-u*c)/det)
        if all(dot(n,point)<=h for n,h in zip(normals,heights)):corners.add(point)
    assert corners==set(p)
    assert all(any(x!=0 for x in edge) for edge in edges(p))
    assert all(dot(n,tuple(scale*x for x in v))<=h for n,h in zip(normals,heights) for v in t)
    a=area(p);assert a==F(71,16)
    e=edges(p);amax=max(abs(dot(u,v)) for u,v in product(e,e));assert amax==F(25,2)
    c_lower=scale**2*F(9,2);assert c_lower==F(441,128)
    sys_lower=c_lower**2/(2*a*a);rmax=amax/a
    q_lower=sys_lower*rmax**2;law=F(8,25)
    assert q_lower>7*law
    # Existing exact symmetrized control S x J S has sys=8/9. This gives an
    # exact reason the proposed 5x5 lower envelope must retain its side counts.
    S=[(F(-3,2),0),(F(-3,2),F(3,2)),(0,F(3,2)),(F(3,2),0),(F(3,2),F(-3,2)),(0,F(-3,2))]
    JS=[(-y,x) for x,y in S]
    six_max=max(abs(dot(u,v)) for u,v in product(edges(S),edges(JS)))
    six_q=F(8,9)*(six_max/area(S))**2
    assert area(S)==F(27,4) and six_max==F(9,4) and six_q==F(8,81)<law
    return dict(status='PASS',vertices=[[str(x) for x in v] for v in p],area=str(a),maximum_symplectic_ridge_area=str(amax),capacity_lower_bound=str(c_lower),sys_lower_bound=str(sys_lower),normalized_maximum_ridge_area=str(rmax),sys_times_rmax_squared_lower_bound=str(q_lower),lower_bound_decimal=float(q_lower),affine_pentagon_law_constant=str(law),exact_hexagon_comparison=dict(imported_sys='8/9',factor_area=str(area(S)),maximum_ridge_area=str(six_max),sys_times_rmax_squared=str(six_q)),violation_factor_lower_bound=float(q_lower/law),assumptions=['capacity monotonicity and degree-two conformality','exact c(T x_L T)=9/2 and symmetrized S x J S sys=8/9, independently verified in patterns/product-shape/exact_controls.py'])
if __name__=='__main__':print(json.dumps(verify(),indent=2))
