"""Exact rational finite proof check for simultaneous central symmetrization.

No third-party dependencies and no capacity-evaluator call. For triangular
factors closure is unique. Product closure reduction puts half the mass in
each factor, so all six weights are 1/6 for these centered triangles. Every
cyclic full word is enumerated; shorter feasible words do not exist.
"""
from fractions import Fraction as F
from itertools import permutations
import json

P = [(-1, -1), (2, -1), (-1, 2)]
A = [(-1, 0), (0, -1), (1, 1)]  # irredundant polar vertices of P
DP_POLAR = [(F(-1, 3), 0), (0, F(-1, 3)), (F(-1, 3), F(-1, 3)),
            (F(1, 3), 0), (0, F(1, 3)), (F(1, 3), F(1, 3))]

def rotate(v):
    return [(-y, x) for x, y in v]

def dot(a, b):
    return sum(x*y for x, y in zip(a, b))

def omega(a, b):
    return a[0]*b[2]+a[1]*b[3]-a[2]*b[0]-a[3]*b[1]

def hull(points):
    pts=sorted(set(tuple(map(F,p)) for p in points))
    def cross(o,a,b):
        return (a[0]-o[0])*(b[1]-o[1])-(a[1]-o[1])*(b[0]-o[0])
    lower=[];upper=[]
    for point in pts:
        while len(lower)>=2 and cross(lower[-2],lower[-1],point)<=0: lower.pop()
        lower.append(point)
    for point in reversed(pts):
        while len(upper)>=2 and cross(upper[-2],upper[-1],point)<=0: upper.pop()
        upper.append(point)
    return lower[:-1]+upper[:-1]

def area(vertices):
    return abs(sum(vertices[i][0]*vertices[(i+1)%len(vertices)][1]-
                   vertices[i][1]*vertices[(i+1)%len(vertices)][0]
                   for i in range(len(vertices))))/2

def difference(vertices):
    return hull([(x-u,y-v) for x,y in vertices for u,v in vertices])

def exact_polar(vertices):
    answer=[]
    for i,(x,y) in enumerate(vertices):
        u,v=vertices[(i+1)%len(vertices)]; det=x*v-y*u
        answer.append(((v-y)/det,(x-u)/det))
    assert all(dot(a,v)<=1 for a in answer for v in vertices)
    return answer

def compute(rotated):
    assert set(exact_polar(hull(P)))==set(A)
    assert set(exact_polar(difference(P)))==set(DP_POLAR)
    b=rotate(A) if rotated else A
    normals=[(x,y,0,0) for x,y in A]+[(0,0,x,y) for x,y in b]
    assert all(sum(n[j] for n in normals)==0 for j in range(4))
    # Six-facet reduction: factor closure vertices use all three normals.
    # Rotational invariance permits fixing label 0 first: 5! words suffice.
    values=[]
    for tail in permutations(range(1,6)):
        word=(0,)+tail
        q=sum(F(omega(normals[i],normals[j]),36)
              for pos,i in enumerate(word) for j in word[pos+1:])
        values.append(q)
    qmax=max(values)
    capacity=1/(2*qmax)
    # A2=max{r: r DQ^polar subset DP}.
    # Both polars have the displayed six exact vertices, with J on Q.
    dqp=rotate(DP_POLAR) if rotated else DP_POLAR
    c2=1/max(dot(u,v) for u in DP_POLAR for v in dqp)
    area_p=area(hull(P)); area_s=area(difference(P))/4
    assert area_p==F(9,2) and area_s==F(27,4)
    sys=capacity**2/(2*area_p**2)
    # For TWO centrally symmetric factors the known symmetric-product
    # theorem identifies capacity with A2; it is not claimed for one factor.
    sym_sys=c2**2/(2*area_s**2)
    return dict(name='P x JP' if rotated else 'P x P',
                cyclic_words=len(values),max_Q=str(qmax),capacity=str(capacity),
                two_bounce_capacity=str(c2),factor_area=str(area_p),
                sym_factor_area=str(area_s),sys=str(sys),sym_sys=str(sym_sys),
                sym_over_original=str(sym_sys/sys),
                maximizing_words=sum(q==qmax for q in values))

def verify():
    rows=[compute(False),compute(True)]
    assert rows[0]['sys']==rows[1]['sys']=='1/2'
    assert [r['sym_sys'] for r in rows]==['2/9','8/9']
    assert [r['sym_over_original'] for r in rows]==['4/9','16/9']
    return dict(status='PASS', controls=rows,
                assumptions=['Product six-facet/closure-vertex reduction',
                             'Known capacity=inradius theorem for two centrally symmetric factors'])

if __name__=='__main__':
    print(json.dumps(verify(),indent=2))
