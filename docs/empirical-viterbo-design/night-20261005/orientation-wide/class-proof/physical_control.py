#!/usr/bin/env python3
"""Fresh exact equality-body geometry and all-ten primal/dual controls.

Writes only physical-control.json. Does not run an earlier producer.
"""
from derive import *

def main():
    start=time.perf_counter();eq=json.loads((ROOT/'equality.json').read_text());assert len(eq['equality_bodies'])==1
    body=eq['equality_bodies'][0];u=K(*body['u']);x=K(*body['x']);h=[K(*v) for v in body['supports']]
    z,a,lams,edges,matrices=geometry();tau2=5-2*z;d=1+tau2*u*u;b=(1+z)/4;R=(5-z)/10
    source=[(K(1),K(0)),((z-1)/4,(5-z)/4),(-(1+z)/4,(3*z-5)/4),(-(1+z)/4,-(3*z-5)/4),((z-1)/4,-(5-z)/4)]
    normals=[(x*(nx-u*ny),tau2*u*nx+ny) for nx,ny in source]
    def dot(p,q):return sum(v*w for v,w in zip(p,q))
    def cross(p,q):return p[0]*q[1]-p[1]*q[0]
    def solve(n,m,r,s):
        determinant=cross(n,m);assert determinant!=0
        return ((r*m[1]-s*n[1])/determinant,(n[0]*s-m[0]*r)/determinant)
    def rotate(v):return (b*v[0]-(3*z-5)/4*v[1],b*(v[0]+v[1]))
    vertices=[solve(normals[i],normals[(i+1)%5],h[i],h[(i+1)%5]) for i in range(5)]
    for v in vertices:assert all(h[i]-dot(normals[i],v)>=0 for i in range(5))
    area=sum(cross(vertices[i],vertices[(i+1)%5]) for i in range(5))/2
    assert area>0 and len(set(vertices))==5
    edgevalues=[sum(row[i]*h[i] for i in range(5)) for row in edges]
    assert all(v>0 for v in edgevalues)
    P=sum(h[i]*h[(i+1)%5]-a*h[i]*h[i] for i in range(5))
    assert area==P/(x*d*(5-z)/4)
    tri=[(-R,K(0)),(R,K(0)),(K(0),R)];templates=[]
    for j in range(10):templates.append(tri);tri=[rotate(v) for v in tri]
    assert tri==templates[0]
    placements=[];denominators=[[K(0)]*10 for _ in range(5)]
    for j,tri in enumerate(templates):
        supports=[max(dot(n,v) for v in tri) for n in normals]
        rhs=[h[i]-supports[i] for i in range(5)];placement=None
        for i,k in combinations(range(5),2):
            translation=solve(normals[i],normals[k],rhs[i],rhs[k])
            slack=[rhs[l]-dot(normals[l],translation) for l in range(5)]
            if all(v>=0 for v in slack):placement=translation;break
        assert placement is not None
        placements.append(dict(template=j,vertices=[[v.data() for v in p] for p in tri],
            support_coefficients=[v.data() for v in supports],translation=[v.data() for v in placement],
            slacks=[v.data() for v in slack]))
        for l,lam in enumerate(lams):denominators[l][j]=sum(lam[i]*supports[i] for i in range(5));assert denominators[l][j]>0
    Ds=[max(row) for row in denominators];numerators=[sum(lam[i]*h[i] for i in range(5)) for lam in lams]
    catalogue_path=ROOT.parent/'preflight'/'active-events.json';catalogue=json.loads(catalogue_path.read_text())
    for l,circuit in enumerate(catalogue['circuits']):
        for j in range(10):
            values=[]
            for candidate in circuit['candidates']:
                if any(src[0]==j for src in candidate['sources']):
                    c0,cx,cu,cxu=[K(*v) for v in candidate['coefficients']]
                    values.append(c0+cx*x+cu*u+cxu*x*u)
            assert max(values)==denominators[l][j]
    assert all(v==Ds[i] for i,v in enumerate(numerators))
    active_duals=[dict(circuit=l,template=j,numerator=numerators[l].data(),denominator=value.data(),
        weights=[v.data() for v in lams[l]]) for l,row in enumerate(denominators) for j,value in enumerate(row) if value==Ds[l]]
    assert active_duals
    Qvertices=[solve(source[i],source[(i+1)%5],b,b) for i in range(5)]
    Qarea=sum(cross(Qvertices[i],Qvertices[(i+1)%5]) for i in range(5))/2
    ratio=K(1)/(2*tau2*area*Qarea);assert ratio==(3+z)/5
    assert ratio==x*d/(5*P)
    metric=lambda p:p[0]*p[0]+tau2*p[1]*p[1]
    subtract=lambda p,q:tuple(v-w for v,w in zip(p,q))
    sides=[metric(subtract(vertices[(i+1)%5],vertices[i])) for i in range(5)]
    center=tuple(sum(v[k] for v in vertices)/5 for k in range(2));radii=[metric(subtract(v,center)) for v in vertices]
    assert len(set(sides))==len(set(radii))==1 and sides[0]>0 and radii[0]>0
    D=(z-1)/5;translationY=solve(source[3],source[4],-D,-D)
    assert all(h[i]==D+dot(source[i],translationY) for i in range(5))
    assert x==1 and u==z/5
    out=dict(status='PASS exact five-edge equality body: allten primal fits + active dual + direct product area',
        equality_sha256=hashlib.sha256((ROOT/'equality.json').read_bytes()).hexdigest(),
        accepted_catalogue_sha256=hashlib.sha256(catalogue_path.read_bytes()).hexdigest(),
        template_source='Accepted fixed-Q regular core: ten rotations by pi/5 of conv{(-R,0),(R,0),(0,R)}, R=(5-sqrt5)/10',
        source_regular_covectors=[[v.data() for v in n] for n in source],tau_squared=tau2.data(),
        source_template_vertices=[[[v.data() for v in p] for p in tri] for tri in templates],
        complete_catalogue_binding='All50direct geometric template-circuit sums equal expanded original270-choice maxima at this x,u',
        u=u.data(),x=x.data(),supports=[v.data() for v in h],scaled_normals=[[v.data() for v in n] for n in normals],
        determinant=d.data(),actual_sides=5,edge_numerators=[v.data() for v in edgevalues],
        K_computational_vertices=[[v.data() for v in p] for p in vertices],K_computational_area=area.data(),
        Q_computational_vertices=[[v.data() for v in p] for p in Qvertices],Q_computational_area=Qarea.data(),
        actual_physical_area_note='Physical area is tau times computational area; direct product uses tau²',
        capacity='Exactly1: allten primal fitting translations plus listed active closure duals',
        placements=placements,all50_denominators=[[v.data() for v in row] for row in denominators],
        maximal_denominators=[v.data() for v in Ds],active_duals=active_duals,
        direct_product_ratio=ratio.data(),canonical_Y_translation=[v.data() for v in translationY],
        physical_side_squared=sides[0].data(),physical_circumradius_squared=radii[0].data(),
        equality_class='K is regular; x=1 and theta=pi/10 in selected symmetry chamber. Restore translations/dilations and theta=pi/10 modulo pi/5.',
        elapsed_seconds=round(time.perf_counter()-start,3))
    (ROOT/'physical-control.json').write_text(json.dumps(out,indent=2)+'\n')
    print(out['status']);print('ratio',ratio,'seconds',out['elapsed_seconds'])

if __name__=='__main__':main()
